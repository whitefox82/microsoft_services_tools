use anyhow::{Context, Result};
use clap::Parser;
use dotenv::dotenv;
use env_logger::Builder;
use log::{debug, error, info, LevelFilter};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

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

#[derive(Deserialize, Debug)]
struct DirectoryRole {
    id: String,
    #[serde(rename = "displayName")]
    display_name: String,
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
    #[serde(rename = "displayName")]
    display_name: String,
    #[serde(rename = "userPrincipalName")]
    user_principal_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct MailboxSettings {
    #[serde(rename = "userPurpose")]
    user_purpose: Option<String>,
}

struct GraphApiClient {
    client: Client,
    access_token: String,
}

impl GraphApiClient {
    fn new(access_token: String) -> Self {
        Self {
            client: Client::new(),
            access_token,
        }
    }

    async fn fetch_directory_roles(&self) -> Result<Vec<DirectoryRole>> {
        let url = "https://graph.microsoft.com/v1.0/directoryRoles";
        let response = self
            .client
            .get(url)
            .bearer_auth(&self.access_token)
            .send()
            .await
            .context("Failed to send request to Graph API")?;

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

    async fn fetch_directory_role_members(&self, role_id: &str) -> Result<Vec<RoleMember>> {
        let url = format!(
            "https://graph.microsoft.com/v1.0/directoryRoles/{}/members",
            role_id
        );

        let response = self
            .client
            .get(&url)
            .bearer_auth(&self.access_token)
            .send()
            .await
            .context("Failed to send request to Graph API")?;

        if response.status().is_success() {
            let members_response: DirectoryRoleMembersResponse = response
                .json()
                .await
                .context("Failed to parse response from Graph API")?;
            Ok(members_response.value)
        } else {
            Err(anyhow::anyhow!(
                "Graph API request failed with status: {}",
                response.status()
            ))
        }
    }

    async fn get_mailbox_settings(&self, user_principal_name: &str) -> Result<MailboxSettings> {
        let url = format!(
            "https://graph.microsoft.com/v1.0/users/{}/mailboxSettings",
            user_principal_name
        );

        let response = self
            .client
            .get(&url)
            .bearer_auth(&self.access_token)
            .send()
            .await
            .context("Failed to send request to fetch mailbox settings")?;

        if response.status().is_success() {
            let mailbox_settings: MailboxSettings = response
                .json()
                .await
                .context("Failed to parse mailbox settings response")?;
            Ok(mailbox_settings)
        } else {
            let error_text = response
                .text()
                .await
                .context("Failed to read error response text")?;
            Err(anyhow::anyhow!("HTTP error: {}", error_text))
        }
    }
}

fn setup_logger(config: &AppConfig) {
    let mut builder = Builder::from_default_env();
    builder.filter(
        None,
        if config.debug {
            LevelFilter::Debug
        } else if config.info {
            LevelFilter::Info
        } else {
            LevelFilter::Warn
        },
    );
    builder.init();
}

async fn get_access_token(tenant_id: &str, client_id: &str, client_secret: &str) -> Result<String> {
    let client = Client::new();
    let token_url = format!(
        "https://login.microsoftonline.com/{}/oauth2/v2.0/token",
        tenant_id
    );

    let params = [
        ("grant_type", "client_credentials"),
        ("client_id", client_id),
        ("client_secret", client_secret),
        ("scope", "https://graph.microsoft.com/.default"),
    ];

    let response = client
        .post(&token_url)
        .form(&params)
        .send()
        .await
        .context("Failed to send token request")?;

    if response.status().is_success() {
        let token_response: serde_json::Value = response
            .json()
            .await
            .context("Failed to parse token response")?;
        Ok(token_response["access_token"]
            .as_str()
            .context("Access token not found in response")?
            .to_string())
    } else {
        Err(anyhow::anyhow!(
            "Token request failed with status: {}",
            response.status()
        ))
    }
}

async fn process_directory_roles(api_client: &GraphApiClient) -> Result<Vec<String>> {
    let mut shared_mailboxes = Vec::new();
    let roles = api_client.fetch_directory_roles().await?;

    info!("Fetched {} directory roles", roles.len());
    for role in roles {
        println!("Role ID: {}, DisplayName: {}", role.id, role.display_name);
        let members = api_client.fetch_directory_role_members(&role.id).await?;
        for member in members {
            println!(
                " - Member ID: {}, DisplayName: {}, UserPrincipalName: {}",
                member.id, member.display_name, member.user_principal_name
            );

            match api_client
                .get_mailbox_settings(&member.user_principal_name)
                .await
            {
                Ok(mailbox_settings) => {
                    debug!(
                        "Mailbox settings for {}: {:?}",
                        member.user_principal_name, mailbox_settings
                    );
                    if let Some(purpose) = mailbox_settings.user_purpose {
                        if purpose.to_lowercase() == "shared" {
                            shared_mailboxes.push(member.user_principal_name);
                        }
                    }
                }
                Err(e) => {
                    error!(
                        "Failed to fetch mailbox settings for {}: {:?}",
                        member.user_principal_name, e
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
    setup_logger(&config);

    info!("Starting api_template");
    debug!("Configuration: {:?}", config);

    let tenant_id = env::var("TENANT_ID").context("TENANT_ID not set in .env file")?;
    let client_id = env::var("CLIENT_ID").context("CLIENT_ID not set in .env file")?;
    let client_secret = env::var("CLIENT_SECRET").context("CLIENT_SECRET not set in .env file")?;

    info!("Requesting access token");
    let access_token = get_access_token(&tenant_id, &client_id, &client_secret).await?;

    let api_client = GraphApiClient::new(access_token);

    let shared_mailboxes = process_directory_roles(&api_client).await?;

    println!("\nShared Mailboxes:");
    for mailbox in shared_mailboxes {
        println!("{} is a shared mailbox with an admin role.", mailbox);
    }

    Ok(())
}
