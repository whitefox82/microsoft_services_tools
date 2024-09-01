use anyhow::{Context, Result};
use clap::Parser;
use dotenv::dotenv;
use env_logger::Builder;
use log::{debug, error, info, LevelFilter};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

mod auth;
use auth::get_access_token;

#[derive(Parser, Debug)]
#[command(name = "api_template")]
#[command(author = "Bryan Abbott <bryan.abbott01@pm.me>")]
#[command(version = "1.0")]
#[command(about = "Outputs the directory roles and mailbox settings using the Microsoft API")]
struct AppConfig {
    #[arg(long, help = "Enable info level logging")]
    info: bool,

    #[arg(long, help = "Enable debug level logging")]
    debug: bool,
}

fn setup_logger(info: bool, debug: bool) {
    let mut builder = Builder::from_default_env();
    builder.filter(
        None,
        if debug {
            LevelFilter::Debug
        } else if info {
            LevelFilter::Info
        } else {
            LevelFilter::Warn
        },
    );
    builder.init();
}

#[derive(Deserialize, Debug)]
struct DirectoryRole {
    id: String,
    displayName: String,
}

#[derive(Deserialize, Debug)]
struct DirectoryRoleResponse {
    value: Vec<DirectoryRole>,
}

#[derive(Deserialize, Debug)]
struct DirectoryRoleMembersResponse {
    value: Vec<RoleMember>,
}

#[derive(Deserialize, Debug)]
struct RoleMember {
    id: String,
    displayName: String,
    userPrincipalName: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct MailboxSettings {
    #[serde(rename = "userPurpose")]
    user_purpose: Option<String>,
}

async fn fetch_directory_roles(access_token: &str) -> Result<Vec<DirectoryRole>> {
    let client = Client::new();
    let url = "https://graph.microsoft.com/v1.0/directoryRoles".to_string();

    debug!("Requesting directory roles from URL: {}", url);
    let response = client
        .get(&url)
        .bearer_auth(access_token)
        .send()
        .await
        .context("Failed to send request to Graph API")?;

    debug!("Processing response");

    if response.status().is_success() {
        let role_response: DirectoryRoleResponse = response
            .json()
            .await
            .context("Failed to parse response from Graph API")?;
        Ok(role_response.value)
    } else {
        Err(anyhow::anyhow!(
            "Graph API request failed with status: {}",
            response.status()
        ))
    }
}

async fn fetch_directory_role_members(
    client: &Client,
    token: &str,
    role_id: &str,
) -> Vec<RoleMember> {
    let url = format!(
        "https://graph.microsoft.com/v1.0/directoryRoles/{}/members",
        role_id
    );

    let response = client.get(&url).bearer_auth(token).send().await;

    match response {
        Ok(res) => match res.json::<DirectoryRoleMembersResponse>().await {
            Ok(members_response) => {
                info!("Successfully fetched members for role: {}", role_id);
                members_response.value
            }
            Err(e) => {
                error!("Failed to parse response body: {:?}", e);
                Vec::new()
            }
        },
        Err(e) => {
            error!("Failed to send request: {:?}", e);
            Vec::new()
        }
    }
}

async fn get_mailbox_settings(
    client: &Client,
    access_token: &str,
    user_principal_name: &str,
) -> Result<MailboxSettings> {
    let url = format!(
        "https://graph.microsoft.com/v1.0/users/{}/mailboxSettings",
        user_principal_name
    );

    debug!(
        "Fetching mailbox settings for user: {}",
        user_principal_name
    );

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
        debug!(
            "Mailbox settings fetched for user {}: {:?}",
            user_principal_name, mailbox_settings
        );
        Ok(mailbox_settings)
    } else {
        let error_text = response
            .text()
            .await
            .context("Failed to read error response text")?;
        Err(anyhow::anyhow!("HTTP error: {}", error_text))
    }
}
async fn process_directory_roles(client: &Client, access_token: &str) -> Result<Vec<String>> {
    let mut shared_mailboxes = Vec::new();
    let roles = fetch_directory_roles(access_token).await?;

    info!("Fetched {} directory roles", roles.len());
    for role in roles {
        println!("Role ID: {}, DisplayName: {}", role.id, role.displayName);
        let members = fetch_directory_role_members(client, access_token, &role.id).await;
        for member in members {
            println!(
                " - Member ID: {}, DisplayName: {}, UserPrincipalName: {}",
                member.id, member.displayName, member.userPrincipalName
            );

            match get_mailbox_settings(client, access_token, &member.userPrincipalName).await {
                Ok(mailbox_settings) => {
                    debug!(
                        "Mailbox settings for {}: {:?}",
                        member.userPrincipalName, mailbox_settings
                    );
                    if let Some(purpose) = mailbox_settings.user_purpose {
                        if purpose.to_lowercase() == "shared" {
                            shared_mailboxes.push(member.userPrincipalName);
                        }
                    }
                }
                Err(e) => {
                    error!(
                        "Failed to fetch mailbox settings for {}: {:?}",
                        member.userPrincipalName, e
                    );
                }
            }
        }
    }

    Ok(shared_mailboxes)
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let config = AppConfig::parse();

    setup_logger(config.info, config.debug);

    info!("Starting api_template");
    debug!("Configuration: {:?}", config);

    debug!("Attempting to load TENANT_ID from environment");
    let tenant_id = env::var("TENANT_ID").context("TENANT_ID not set in .env file")?;
    debug!("TENANT_ID loaded successfully: {}", tenant_id);

    debug!("Attempting to load CLIENT_ID from environment");
    let client_id = env::var("CLIENT_ID").context("CLIENT_ID not set in .env file")?;
    debug!("CLIENT_ID loaded successfully: {}", client_id);

    debug!("Attempting to load CLIENT_SECRET from environment");
    let client_secret = env::var("CLIENT_SECRET").context("CLIENT_SECRET not set in .env file")?;
    debug!("CLIENT_SECRET loaded successfully");

    info!("Requesting access token");
    let access_token = get_access_token(&tenant_id, &client_id, &client_secret)
        .await
        .context("Failed to obtain access token")?;

    let client = Client::new();
    let shared_mailboxes = process_directory_roles(&client, &access_token).await?;

    println!("\nShared Mailboxes:");
    for mailbox in shared_mailboxes {
        println!("{} is a sharedmailbox with an admin role.", mailbox);
    }

    Ok(())
}
