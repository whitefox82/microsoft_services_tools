use anyhow::{Context, Result};
use clap::{Arg, Command};
use dotenv::dotenv;
use env_logger::Builder;
use log::{debug, info, error, LevelFilter};
use reqwest::Client;
use serde_json::Value;
use std::env;
use std::error::Error;
use std::path::PathBuf;

mod auth;

fn setup_logger() {
    let mut builder = Builder::from_default_env();
    builder.filter_level(if cfg!(debug_assertions) {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    });
    builder.init();
}

fn get_env_file_path() -> Result<PathBuf> {
    let mut exe_path = std::env::current_exe().context("Failed to get current executable path")?;
    exe_path.pop();
    debug!("Executable path: {:?}", exe_path);

    let mut env_path = exe_path;
    env_path.push(".env");
    info!("Determined .env file path: {:?}", env_path);
    Ok(env_path)
}

async fn get_access_token_from_env() -> Result<String> {
    let env_path = get_env_file_path()?;
    dotenv::from_path(&env_path).context("Failed to load .env file")?;
    debug!(".env file loaded from {:?}", env_path);

    let tenant_id = env::var("TENANT_ID").context("TENANT_ID environment variable not found")?;
    let client_id = env::var("CLIENT_ID").context("CLIENT_ID environment variable not found")?;
    let client_secret =
        env::var("CLIENT_SECRET").context("CLIENT_SECRET environment variable not found")?;

    info!("Environment variables TENANT_ID, CLIENT_ID, and CLIENT_SECRET loaded successfully.");

    auth::get_access_token(&tenant_id, &client_id, &client_secret).await
}

async fn get_authentication_methods(
    access_token: &str,
    user_id: &str,
) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let url = format!(
        "https://graph.microsoft.com/v1.0/users/{}/authentication/methods",
        user_id
    );
    info!("Retrieving authentication methods for user: {}", user_id);

    let client = Client::new();
    let response = client.get(&url).bearer_auth(access_token).send().await?;

    if response.status().is_success() {
        debug!("Received successful response for authentication methods.");
        let methods: Value = response.json().await?;
        let method_ids: Vec<(String, String)> = methods["value"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|method| {
                let method_type = method["@odata.type"].as_str()?;
                let method_id = method["id"].as_str()?;
                debug!("Found method type: {}, method ID: {}", method_type, method_id);
                Some((method_type.to_string(), method_id.to_string()))
            })
            .collect();
        Ok(method_ids)
    } else {
        let status = response.status();
        let text = response.text().await?;
        error!(
            "Failed to retrieve authentication methods for user {}: {} - {}",
            user_id, status, text
        );
        Err(format!(
            "Failed to retrieve authentication methods for user {}: {} - {}",
            user_id, status, text
        )
        .into())
    }
}

async fn delete_authentication_method(
    access_token: &str,
    user_id: &str,
    method_type: &str,
    method_id: &str,
) -> Result<(), Box<dyn Error>> {
    let url = match method_type {
        "#microsoft.graph.softwareOathAuthenticationMethod" => format!(
            "https://graph.microsoft.com/v1.0/users/{}/authentication/softwareOathMethods/{}",
            user_id, method_id
        ),
        _ => {
            debug!("Ignoring unsupported method type: {}", method_type);
            return Ok(()); // Ignore other method types
        }
    };

    info!(
        "Attempting to delete authentication method {} for user {}",
        method_id, user_id
    );

    let client = Client::new();
    let response = client.delete(&url).bearer_auth(access_token).send().await?;

    if response.status().is_success() {
        info!(
            "Successfully deleted authentication method {} for user {}",
            method_id, user_id
        );
        Ok(())
    } else {
        let status = response.status();
        let text = response.text().await?;
        error!(
            "Failed to delete authentication method {} for user {}: {} - {}",
            method_id, user_id, status, text
        );
        Err(format!(
            "Failed to delete authentication method {} for user {}: {} - {}",
            method_id, user_id, status, text
        )
        .into())
    }
}

async fn require_mfa_reregistration(access_token: &str, upn: &str) -> Result<(), Box<dyn Error>> {
    info!("Requiring MFA re-registration for user {}", upn);
    let method_ids = get_authentication_methods(access_token, upn).await?;
    for (method_type, method_id) in method_ids {
        if method_type == "#microsoft.graph.softwareOathAuthenticationMethod" {
            delete_authentication_method(access_token, upn, &method_type, &method_id).await?;
        }
    }
    info!(
        "MFA re-registration required successfully for user {}.",
        upn
    );
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    setup_logger();

    let matches = Command::new("revoke_mfaregistrations")
        .version("1.0")
        .author("Bryan Abbott <bryan.abbott01@pm.me>")
        .about("Requires MFA re-registration using the Microsoft API")
        .arg(
            Arg::new("upn")
                .short('u')
                .long("upn")
                .value_name("UPN")
                .help("User Principal Name")
                .required(true),
        )
        .get_matches();

    let upn = matches.get_one::<String>("upn").expect("UPN is required");

    info!("Starting MFA re-registration process for user: {}", upn);

    match get_access_token_from_env().await {
        Ok(access_token) => match require_mfa_reregistration(&access_token, upn).await {
            Ok(_) => info!("Operation completed successfully."),
            Err(e) => error!("Error during MFA re-registration: {}", e),
        },
        Err(e) => error!("Failed to obtain access token: {}", e),
    }

    Ok(())
}
