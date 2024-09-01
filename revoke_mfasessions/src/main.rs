use anyhow::{Context, Result};
use clap::Parser;
use env_logger::Builder;
use log::{debug, info, LevelFilter};
use reqwest::Client;
use std::env;
use std::path::PathBuf;

mod auth;

#[derive(Parser, Debug)]
struct AppConfig {
    #[arg(short, long, help = "User Principal Name")]
    upn: String,

    #[arg(short, long, help = "Enable verbose logging")]
    verbose: bool,
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

fn get_env_file_path() -> Result<PathBuf> {
    let mut exe_path = std::env::current_exe().context("Failed to get current executable path")?;
    exe_path.pop();
    debug!("Executable path: {:?}", exe_path);

    let mut env_path = exe_path;
    env_path.push(".env");
    Ok(env_path)
}

async fn get_access_token_from_env(verbose: bool) -> Result<String> {
    let env_path = get_env_file_path()?;
    dotenv::from_path(&env_path).context("Failed to load .env file")?;
    debug!(".env file loaded from {:?}", env_path);

    let tenant_id = env::var("TENANT_ID").context("TENANT_ID environment variable not found")?;
    let client_id = env::var("CLIENT_ID").context("CLIENT_ID environment variable not found")?;
    let client_secret =
        env::var("CLIENT_SECRET").context("CLIENT_SECRET environment variable not found")?;

    auth::get_access_token(&tenant_id, &client_id, &client_secret, verbose).await
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

#[tokio::main]
async fn main() -> Result<()> {
    let config = AppConfig::parse();
    setup_logger(config.verbose);

    info!("Starting RevokeSessionService");
    debug!("Configuration: {:?}", config);

    let access_token = get_access_token_from_env(config.verbose).await?;
    debug!("Access token obtained successfully");

    revoke_sign_in_sessions(&access_token, &config.upn).await?;

    info!("Operation completed successfully.");
    Ok(())
}
