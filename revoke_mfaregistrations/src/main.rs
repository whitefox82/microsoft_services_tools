use anyhow::{Context, Result};
use clap::{Arg, Command};
use log::debug;
use reqwest::Client;
use serde_json::Value;
use std::env;
use std::error::Error;
use std::path::PathBuf;

mod auth;

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

async fn get_authentication_methods(
    access_token: &str,
    user_id: &str,
) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let url = format!(
        "https://graph.microsoft.com/v1.0/users/{}/authentication/methods",
        user_id
    );

    let client = Client::new();
    let response = client.get(&url).bearer_auth(access_token).send().await?;

    if response.status().is_success() {
        let methods: Value = response.json().await?;
        let method_ids: Vec<(String, String)> = methods["value"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|method| {
                let method_type = method["@odata.type"].as_str()?;
                let method_id = method["id"].as_str()?;
                Some((method_type.to_string(), method_id.to_string()))
            })
            .collect();
        Ok(method_ids)
    } else {
        let status = response.status();
        let text = response.text().await?;
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
        _ => return Ok(()), // Ignore other method types
    };

    let client = Client::new();
    let response = client.delete(&url).bearer_auth(access_token).send().await?;

    if response.status().is_success() {
        Ok(())
    } else {
        let status = response.status();
        let text = response.text().await?;
        Err(format!(
            "Failed to delete authentication method {} for user {}: {} - {}",
            method_id, user_id, status, text
        )
        .into())
    }
}

async fn require_mfa_reregistration(access_token: &str, upn: &str) -> Result<(), Box<dyn Error>> {
    let method_ids = get_authentication_methods(access_token, upn).await?;
    for (method_type, method_id) in method_ids {
        if method_type == "#microsoft.graph.softwareOathAuthenticationMethod" {
            delete_authentication_method(access_token, upn, &method_type, &method_id).await?;
        }
    }
    println!(
        "MFA re-registration required successfully for user {}.",
        upn
    );
    Ok(())
}

#[tokio::main]
async fn main() {
    let matches = Command::new("revoke_mfaregistrations")
        .version("1.0")
        .author("Bryan Abbott <bryan.abbott@fusionnetworks.co.nz>")
        .about("Requires MFA re-registration using the Microsoft API")
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
                .help("Enable verbose output"),
        )
        .get_matches();

    let upn = matches.get_one::<String>("upn").expect("UPN is required");
    let verbose = matches.contains_id("verbose");

    match get_access_token_from_env(verbose).await {
        Ok(access_token) => match require_mfa_reregistration(&access_token, upn).await {
            Ok(_) => println!("Operation completed successfully."),
            Err(e) => eprintln!("Error: {}", e),
        },
        Err(e) => eprintln!("Failed to obtain access token: {}", e),
    }
}
