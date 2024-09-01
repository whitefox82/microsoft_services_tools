use clap::{Arg, ArgAction, Command};
use colored::*;
use env_logger::Builder;
use log::{debug, error, info, warn, LevelFilter};
use reqwest::Client;
use serde_json::Value;
use std::error::Error;
use std::path::PathBuf;
use std::process::Command as ProcessCommand;
use std::sync::atomic::{AtomicBool, Ordering};

static VERBOSE: AtomicBool = AtomicBool::new(false);

#[derive(Debug)]
struct AppConfig {
    company: String,
    upn: String,
    subject: String,
    verbose: bool,
    spoofed: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = parse_args();
    VERBOSE.store(config.verbose, Ordering::Relaxed);
    setup_logger(config.verbose);

    info!("Starting email search");
    debug!("Configuration: {:?}", config);

    let access_token = get_access_token(&config.company).await?;
    let client = Client::new();
    let emails =
        search_email_messages(&client, &access_token, &config.upn, &config.subject).await?;

    if config.spoofed {
        process_spoofed_emails(&emails);
    } else {
        println!("{}", serde_json::to_string_pretty(&emails)?);
    }

    info!("Operation completed successfully.");
    Ok(())
}

async fn get_access_token(company: &str) -> Result<String, Box<dyn Error>> {
    debug!("Getting access token for company: {}", company);
    let auth_service_path = get_auth_service_path()?;

    let output = ProcessCommand::new(&auth_service_path)
        .arg("-c")
        .arg(company)
        .output()?;

    if output.status.success() {
        let token = String::from_utf8(output.stdout)?.trim().to_string();
        debug!("Access token received successfully");
        Ok(token)
    } else {
        let error_msg = format!(
            "Auth service failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        error!("{}", error_msg);
        Err(error_msg.into())
    }
}

fn get_auth_service_path() -> Result<PathBuf, Box<dyn Error>> {
    let current_exe = std::env::current_exe()?;
    let mut auth_service_path = current_exe
        .parent()
        .ok_or("Unable to get parent directory")?
        .to_path_buf();
    auth_service_path.push("ms_auth_service");
    debug!("Auth service path: {:?}", auth_service_path);
    Ok(auth_service_path)
}

async fn search_email_messages(
    client: &Client,
    access_token: &str,
    upn: &str,
    subject: &str,
) -> Result<Value, Box<dyn Error>> {
    let url = format!(
        "https://graph.microsoft.com/v1.0/users/{}/messages?$search=\"subject:{}\"",
        upn, subject
    );
    debug!("Searching email messages with URL: {}", url);

    let response = client
        .get(&url)
        .bearer_auth(access_token)
        .header("ConsistencyLevel", "eventual")
        .send()
        .await?
        .error_for_status()?;

    debug!("Received response from Microsoft Graph API");
    let json: Value = response.json().await?;
    debug!("Successfully parsed JSON response");
    Ok(json)
}

fn parse_args() -> AppConfig {
    debug!("Parsing command line arguments");
    let matches = Command::new("get_email")
        .version("1.0")
        .author("Bryan Abbott <bryan.abbott01@pm.me>")
        .about("Obtains Basic Email Information")
        .arg(
            Arg::new("company")
                .short('c')
                .long("company")
                .value_name("COMPANY")
                .help("Company name to get access token for")
                .required(true),
        )
        .arg(
            Arg::new("upn")
                .short('u')
                .long("upn")
                .value_name("UPN")
                .help("User Principal Name")
                .required(true),
        )
        .arg(
            Arg::new("subject")
                .short('s')
                .long("subject")
                .value_name("SUBJECT")
                .help("Email subject to search for")
                .required(true),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose logging")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("spoofed")
                .short('p')
                .long("spoofed")
                .help("Output spoofed email addresses")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    AppConfig {
        company: matches
            .get_one::<String>("company")
            .expect("Company name is required")
            .clone(),
        upn: matches
            .get_one::<String>("upn")
            .expect("UPN is required")
            .clone(),
        subject: matches
            .get_one::<String>("subject")
            .expect("Subject is required")
            .clone(),
        verbose: *matches.get_one::<bool>("verbose").unwrap(),
        spoofed: *matches.get_one::<bool>("spoofed").unwrap(),
    }
}

fn setup_logger(verbose: bool) {
    let mut builder = Builder::from_default_env();
    if verbose {
        builder.filter(None, LevelFilter::Debug);
        debug!("Verbose logging enabled");
    } else {
        builder.filter(None, LevelFilter::Info);
        info!("Normal logging mode");
    }
    builder.init();
}

fn process_spoofed_emails(emails: &Value) {
    debug!("Processing emails for spoofed output");
    if let Some(email_array) = emails["value"].as_array() {
        for (index, email) in email_array.iter().enumerate() {
            debug!("Processing email {}", index + 1);
            println!(
                "Subject: {}",
                email["subject"].as_str().unwrap_or("Unknown")
            );

            process_reply_to(email);
            process_sender_and_from(email);

            println!();
        }
    }
}

fn process_reply_to(email: &Value) {
    if let Some(reply_to) = email["replyTo"].as_array() {
        if !reply_to.is_empty() {
            for reply in reply_to {
                let reply_address = reply["emailAddress"]["address"]
                    .as_str()
                    .unwrap_or("Unknown");
                let from_address = email["from"]["emailAddress"]["address"]
                    .as_str()
                    .unwrap_or("Unknown");
                if reply_address == from_address {
                    println!("ReplyTo: {}", reply_address.green());
                    debug!("ReplyTo matches From address: {}", reply_address);
                } else {
                    println!("ReplyTo: {}", reply_address.red());
                    debug!(
                        "ReplyTo does not match From address. ReplyTo: {}",
                        reply_address
                    );
                }
            }
        } else {
            println!("ReplyTo: None");
            debug!("No ReplyTo address found");
        }
    }
}

fn process_sender_and_from(email: &Value) {
    let sender_address = email["sender"]["emailAddress"]["address"]
        .as_str()
        .unwrap_or("Unknown");
    let from_address = email["from"]["emailAddress"]["address"]
        .as_str()
        .unwrap_or("Unknown");

    if sender_address == from_address {
        println!("Sender: {}", sender_address.green());
        println!("From: {}", from_address.green());
        debug!("Sender and From addresses match: {}", sender_address);
    } else {
        println!("Sender: {}", sender_address.red());
        println!("From: {}", from_address.red());
        warn!(
            "Sender and From addresses do not match. Sender: {}, From: {}",
            sender_address, from_address
        );
    }
}
