use anyhow::{Context, Result};
use clap::Parser;
use dotenv::dotenv;
use env_logger::Builder;
use log::{debug, info, LevelFilter};
use reqwest::Client;
use std::env;

mod auth;
use auth::get_access_token;

#[derive(Parser, Debug)]
#[command(name = "RevokeSessionService")]
#[command(author = "Bryan Abbott <bryan.abbott01@pm.me>")]
#[command(version = "1.0")]
#[command(about = "Revokes sign-in sessions using the Microsoft API")]
struct AppConfig {
    #[arg(short, long, help = "User Principal Name")]
    upn: String,

    #[arg(short, long, help = "Enable verbose logging")]
    verbose: bool,
}

async fn revoke_sign_in_sessions(access_token: &str, upn: &str) -> Result<()> {
    let url = format!(
        "https://graph.microsoft.com/v1.0/users/{}/revokeSignInSessions",
        upn
    );
    debug!("Revoking sign-in sessions at URL: {}", url);

    let client = Client::new();
    let response = client
        .post(&url)
        .bearer_auth(access_token)
        .json(&serde_json::json!({}))
        .send()
        .await
        .context("Failed to send request")?;

    let status = response.status();
    let response_text = response
        .text()
        .await
        .context("Failed to get response text")?;

    debug!("Response status: {:?}", status);
    debug!("Response text: {:?}", response_text);

    if status.is_success() {
        info!("Sign-in sessions revoked successfully for user {}.", upn);
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "Failed to revoke sign-in sessions for user {}: {} - {}",
            upn,
            status,
            response_text
        ))
    }
}

fn setup_logger(verbose: bool) {
    let mut builder = Builder::from_default_env();
    builder.filter(
        None,
        if verbose {
            LevelFilter::Debug
        } else {
            LevelFilter::Info
        },
    );
    builder.init();
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let config = AppConfig::parse();
    setup_logger(config.verbose);

    info!("Starting RevokeSessionService");
    debug!("Configuration: {:?}", config);

    let tenant_id = env::var("TENANT_ID").context("TENANT_ID not set in .env file")?;
    let client_id = env::var("CLIENT_ID").context("CLIENT_ID not set in .env file")?;
    let client_secret = env::var("CLIENT_SECRET").context("CLIENT_SECRET not set in .env file")?;

    let access_token = get_access_token(&tenant_id, &client_id, &client_secret, config.verbose)
        .await
        .context("Failed to obtain access token")?;

    debug!("Access token obtained successfully");

    revoke_sign_in_sessions(&access_token, &config.upn).await?;

    info!("Operation completed successfully.");
    Ok(())
}
