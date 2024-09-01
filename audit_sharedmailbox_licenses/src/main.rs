use anyhow::{Context, Result};
use clap::Parser;
use dotenv::dotenv;
use env_logger::Builder;
use log::{debug, info, LevelFilter};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;
use futures::future::join_all;

mod auth;
use auth::get_access_token;

#[derive(Parser, Debug)]
#[command(name = "audit_sharedmailbox_licenses")]
#[command(author = "Bryan Abbott <bryan.abbott01@pm.me>")]
#[command(version = "1.0")]
#[command(about = "Audit shared mailbox licenses")]
struct AppConfig;

fn setup_logger() {
    let mut builder = Builder::from_default_env();

    builder.filter(None, if cfg!(debug_assertions) {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    });

    builder.init();
}

#[derive(Deserialize, Debug)]
struct UsersResponse {
    value: Vec<User>,
    #[serde(rename = "@odata.nextLink")]
    next_link: Option<String>,
}

#[derive(Deserialize, Debug)]
struct User {
    #[serde(rename = "userPrincipalName")]
    user_principal_name: String,
    #[serde(rename = "assignedLicenses")]
    assigned_licenses: Vec<License>,
}

#[derive(Deserialize, Debug)]
struct License {
    #[serde(rename = "skuId")]
    sku_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct MailboxSettings {
    #[serde(rename = "userPurpose")]
    user_purpose: Option<String>,
}

async fn get_users(access_token: &str) -> Result<Vec<User>> {
    let client = Client::new();
    let mut url = "https://graph.microsoft.com/v1.0/users?$select=userPrincipalName,assignedLicenses".to_string();
    let mut users: Vec<User> = Vec::new();
    let mut page_count = 0;

    loop {
        page_count += 1;
        debug!("Fetching users from URL: {} (Page {})", url, page_count);
        let response = client
            .get(&url)
            .bearer_auth(access_token)
            .send()
            .await
            .context("Failed to send request to fetch users")?;

        debug!("Response status: {}", response.status());

        if response.status().is_success() {
            let users_response: UsersResponse = response
                .json()
                .await
                .context("Failed to parse users response")?;
            debug!("Number of users fetched on this page: {}", users_response.value.len());
            users.extend(users_response.value);

            if let Some(next_link) = users_response.next_link {
                url = next_link;
                debug!("Next page URL: {}", url);
            } else {
                debug!("No more pages. Finished fetching users.");
                break;
            }
        } else {
            let error_text = response
                .text()
                .await
                .context("Failed to read error response text")?;
            return Err(anyhow::anyhow!("HTTP error: {}", error_text));
        }
    }

    debug!("Total number of users fetched: {}", users.len());
    Ok(users)
}

async fn get_mailbox_settings(client: &Client, access_token: &str, user_principal_name: &str) -> Result<MailboxSettings> {
    let url = format!(
        "https://graph.microsoft.com/v1.0/users/{}/mailboxSettings",
        user_principal_name
    );

    debug!("Fetching mailbox settings for user: {}", user_principal_name);

    let response = client
        .get(&url)
        .bearer_auth(access_token)
        .send()
        .await
        .context("Failed to send request to fetch mailbox settings")?;

    debug!("Response status: {}", response.status());

    if response.status().is_success() {
        let mailbox_settings: MailboxSettings = response
            .json()
            .await
            .context("Failed to parse mailbox settings response")?;
        debug!("Mailbox settings fetched for user {}: {:?}", user_principal_name, mailbox_settings);
        Ok(mailbox_settings)
    } else {
        let error_text = response
            .text()
            .await
            .context("Failed to read error response text")?;
        Err(anyhow::anyhow!("HTTP error: {}", error_text))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    setup_logger();

    info!("Starting api_template");

    let tenant_id = env::var("TENANT_ID").context("TENANT_ID not set in .env file")?;
    let client_id = env::var("CLIENT_ID").context("CLIENT_ID not set in .env file")?;
    let client_secret = env::var("CLIENT_SECRET").context("CLIENT_SECRET not set in .env file")?;

    let access_token = get_access_token(&tenant_id, &client_id, &client_secret)
        .await
        .context("Failed to obtain access token")?;

    // Step 1: Pull all users into memory
    let users = get_users(&access_token).await?;
    debug!("Completed fetching all users.");

    // Step 2: Once all users are pulled, fetch mailbox settings concurrently for those with licenses
    let client = Arc::new(Client::new());
    let user_purpose_map = Arc::new(Mutex::new(HashMap::new()));
    let mut shared_users_with_licenses_count = 0;

    let tasks: Vec<_> = users.iter()
        .filter(|user| !user.assigned_licenses.is_empty())
        .map(|user| {
            let client = Arc::clone(&client);
            let access_token = access_token.to_string();
            let user_purpose_map = Arc::clone(&user_purpose_map);
            let user_principal_name = user.user_principal_name.clone();

            tokio::spawn(async move {
                let mailbox_settings = match get_mailbox_settings(&client, &access_token, &user_principal_name).await {
                    Ok(settings) => settings,
                    Err(e) => {
                        debug!("Error fetching mailbox settings for {}: {}", user_principal_name, e);
                        return;
                    }
                };

                let mut map = user_purpose_map.lock().await;
                map.insert(user_principal_name.clone(), mailbox_settings.user_purpose);
            })
        })
        .collect();

    join_all(tasks).await;

    debug!("Finished processing all users. Outputting results...");

    // Output the results to the terminal
    let user_purpose_map = user_purpose_map.lock().await;
    for user in users {
        let user_purpose = user_purpose_map.get(&user.user_principal_name).unwrap_or(&None).clone();
        if let Some(ref up) = user_purpose {
            if up == "shared" {
                info!(
                    "User with shared purpose and licenses: {}",
                    user.user_principal_name
                );
                shared_users_with_licenses_count += 1;
            }
        }
    }

    info!(
        "Total number of users with shared purpose and licenses: {}",
        shared_users_with_licenses_count
    );

    debug!("Program finished successfully.");
    Ok(())
}
