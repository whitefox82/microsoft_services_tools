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

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let config = AppConfig::parse();
    setup_logger(config.verbose);

    info!("Starting api_template");
    debug!("Configuration: {:?}", config);

    let tenant_id = env::var("TENANT_ID").context("TENANT_ID not set in .env file")?;
    let client_id = env::var("CLIENT_ID").context("CLIENT_ID not set in .env file")?;
    let client_secret = env::var("CLIENT_SECRET").context("CLIENT_SECRET not set in .env file")?;

    let access_token = get_access_token(&tenant_id, &client_id, &client_secret, config.verbose)
        .await
        .context("Failed to obtain access token")?;

    println!("Access Token: {}", access_token);

    Ok(())
}
