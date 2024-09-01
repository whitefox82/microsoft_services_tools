use anyhow::{Context, Result};
use clap::{Arg, ArgAction, Command};
use env_logger::Builder;
use log::{debug, info, LevelFilter};
use reqwest::Client;
use std::env;
use std::error::Error;
use std::path::PathBuf;

mod auth;

#[derive(Debug)]
struct AppConfig {
    upn: String,
    verbose: bool,
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

fn get_env_file_path() -> Result<PathBuf> {
    let mut exe_path = std::env::current_exe().context("Failed to get current executable path")?;
    exe_path.pop();
    debug!("Executable path: {:?}", exe_path);

    let mut env_path = exe_path;
    env_path.push(".env");
    Ok(env_path)
}

async fn revoke_sign_in_sessions(access_token: &str, upn: &str) -> Result<(), Box<dyn Error>> {
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
        .map_err(|e| format!("Failed to send request: {}", e))?;

    let status = response.status();
    let response_text = response
        .text()
        .await
        .map_err(|e| format!("Failed to get response text: {}", e))?;

    debug!("Response status: {:?}", status);
    debug!("Response text: {:?}", response_text);

    if status.is_success() {
        info!("Sign-in sessions revoked successfully for user {}.", upn);
        Ok(())
    } else {
        Err(format!(
            "Failed to revoke sign-in sessions for user {}: {} - {}",
            upn, status, response_text
        )
        .into())
    }
}

fn parse_args() -> AppConfig {
    let matches = Command::new("RevokeSessionService")
        .version("1.0")
        .author("Bryan Abbott <bryan.abbott01@pm.me>")
        .about("Revokes sign-in sessions using the Microsoft API")
        .arg(
            Arg::new("upn")
                .short('u')
                .long("upn")
                .value_name("UPN")
                .help("User Principal Name")
                .required(true),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose logging")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    AppConfig {
        upn: matches
            .get_one::<String>("upn")
            .expect("UPN is required")
            .clone(),
        verbose: *matches.get_one::<bool>("verbose").unwrap(),
    }
}

fn setup_logger(verbose: bool) {
    let mut builder = Builder::from_default_env();
    if verbose {
        builder.filter(None, LevelFilter::Debug);
    } else {
        builder.filter(None, LevelFilter::Info);
    }
    builder.init();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = parse_args();
    setup_logger(config.verbose);

    info!("Starting RevokeSessionService");
    debug!("Configuration: {:?}", config);

    let access_token = get_access_token_from_env(config.verbose).await?;
    debug!("Access token obtained successfully");

    revoke_sign_in_sessions(&access_token, &config.upn).await?;

    info!("Operation completed successfully.");
    Ok(())
}
