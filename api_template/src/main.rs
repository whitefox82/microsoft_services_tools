use anyhow::{Context, Result};
use clap::Parser;
use dotenv::dotenv;
use env_logger::Builder;
use log::{debug, info, LevelFilter};
use std::env;

mod auth;
use auth::get_access_token;

#[derive(Parser, Debug)]
#[command(name = "api_template")]
#[command(author = "Bryan Abbott <bryan.abbott01@pm.me>")]
#[command(version = "1.0")]
#[command(about = "Outputs the access token using the Microsoft API")]
struct AppConfig {
    #[arg(long, help = "Enable info level logging")]
    info: bool,

    #[arg(long, help = "Enable debug level logging")]
    debug: bool,
}

fn setup_logger(info: bool, debug: bool) {
    // Initialize the logger based on the provided flags
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

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file
    dotenv().ok();

    // Parse command-line arguments
    let config = AppConfig::parse();

    // Set up logging based on the command-line flags
    setup_logger(config.info, config.debug);

    // Log the start of the application
    info!("Starting api_template");
    debug!("Configuration: {:?}", config);

    // Load and log the TENANT_ID environment variable
    debug!("Attempting to load TENANT_ID from environment");
    let tenant_id = env::var("TENANT_ID").context("TENANT_ID not set in .env file")?;
    debug!("TENANT_ID loaded successfully: {}", tenant_id);

    // Load and log the CLIENT_ID environment variable
    debug!("Attempting to load CLIENT_ID from environment");
    let client_id = env::var("CLIENT_ID").context("CLIENT_ID not set in .env file")?;
    debug!("CLIENT_ID loaded successfully: {}", client_id);

    // Load and log the CLIENT_SECRET environment variable
    debug!("Attempting to load CLIENT_SECRET from environment");
    let client_secret = env::var("CLIENT_SECRET").context("CLIENT_SECRET not set in .env file")?;
    debug!("CLIENT_SECRET loaded successfully");

    // Log the action of requesting an access token
    info!("Requesting access token");
    let access_token = get_access_token(&tenant_id, &client_id, &client_secret)
        .await
        .context("Failed to obtain access token")?;

    // Output the access token to the terminal
    println!("Access Token: {}", access_token);
    
    // Log the completion of the token retrieval process
    info!("Access token retrieval completed successfully");

    Ok(())
}
