use anyhow::{Context, Result};
use clap::Parser;
use dotenv::from_path;
use log::{debug, info, LevelFilter};
use regex::Regex;
use reqwest::Client;
use serde_json::json;
use std::env;
use std::path::PathBuf;
use std::str::FromStr;

mod auth;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, help = "Recipient email address")]
    email: Email,
    #[arg(short, long, help = "Email subject")]
    subject: String,
    #[arg(short, long, help = "Email body")]
    body: String,
    #[arg(short = 'u', long, help = "Sender email address")]
    sender: String,
    #[arg(short, long, help = "Enable verbose logging")]
    verbose: bool,
}

#[derive(Debug, Clone)]
struct Email(String);

impl FromStr for Email {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let email_regex = Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$")?;
        if email_regex.is_match(s) {
            Ok(Email(s.to_string()))
        } else {
            Err(anyhow::anyhow!("Invalid email: {}", s))
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    setup_logger(cli.verbose);
    info!("Starting EmailSendService");
    debug!("Configuration: {:?}", cli);

    let access_token = get_access_token_from_env(cli.verbose).await?;
    debug!("Access token obtained successfully");

    send_email(&cli, &access_token).await?;
    info!("Operation completed successfully.");
    Ok(())
}

fn setup_logger(verbose: bool) {
    let level = if verbose {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };
    env_logger::Builder::from_default_env()
        .filter(None, level)
        .init();
    debug!("Logger initialized. Verbose mode: {}", verbose);
}

async fn get_access_token_from_env(verbose: bool) -> Result<String> {
    let env_path = get_env_file_path()?;
    from_path(&env_path).context("Failed to load .env file")?;
    debug!(".env file loaded from {:?}", env_path);

    let tenant_id = env::var("TENANT_ID").context("TENANT_ID environment variable not found")?;
    let client_id = env::var("CLIENT_ID").context("CLIENT_ID environment variable not found")?;
    let client_secret =
        env::var("CLIENT_SECRET").context("CLIENT_SECRET environment variable not found")?;

    auth::get_access_token(&tenant_id, &client_id, &client_secret, verbose).await
}

fn get_env_file_path() -> Result<PathBuf> {
    let mut exe_path = env::current_exe().context("Failed to get current executable path")?;
    exe_path.pop();
    debug!("Executable path: {:?}", exe_path);

    let mut env_path = exe_path;
    env_path.push(".env");
    Ok(env_path)
}

async fn send_email(cli: &Cli, access_token: &str) -> Result<()> {
    debug!("Preparing to send email...");
    let client = Client::new();
    let url = format!(
        "https://graph.microsoft.com/v1.0/users/{}/sendMail",
        cli.sender
    );
    debug!("Send email URL: {}", url);

    let email_data = create_email_data(cli);
    let response = send_email_request(&client, &url, access_token, &email_data).await?;

    handle_email_response(response, &cli.email.0).await
}

fn create_email_data(cli: &Cli) -> serde_json::Value {
    json!({
        "message": {
            "subject": cli.subject,
            "body": {
                "contentType": "Text",
                "content": cli.body
            },
            "toRecipients": [
                {
                    "emailAddress": {
                        "address": cli.email.0
                    }
                }
            ]
        },
        "saveToSentItems": "true"
    })
}

async fn send_email_request(
    client: &Client,
    url: &str,
    access_token: &str,
    email_data: &serde_json::Value,
) -> Result<reqwest::Response> {
    debug!("Sending email request...");
    client
        .post(url)
        .bearer_auth(access_token)
        .json(email_data)
        .send()
        .await
        .context("Failed to send email request")
}

async fn handle_email_response(response: reqwest::Response, recipient: &str) -> Result<()> {
    let status = response.status();
    debug!("Response status: {:?}", status);
    let response_text = response
        .text()
        .await
        .context("Failed to get response text")?;
    debug!("Response text: {:?}", response_text);

    if status.is_success() {
        info!("Email sent successfully to {}", recipient);
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "Failed to send email to {}: {} - {}",
            recipient,
            status,
            response_text
        ))
    }
}
