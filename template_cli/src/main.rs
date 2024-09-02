mod auth;

use clap::{Arg, Command};
use env_logger::Env;
use log::{debug, error, info, warn, LevelFilter};

#[tokio::main]
async fn main() {
    info!("Starting template_cli...");
    let matches = Command::new("template_cli")
        .version("1.0")
        .about("A template CLI app demonstrating authentication with Microsoft API")
        .arg(
            Arg::new("debug")
                .long("debug")
                .help("Sets the level of logging to DEBUG")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("info")
                .long("info")
                .help("Sets the level of logging to INFO")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("warn")
                .long("warn")
                .help("Sets the level of logging to WARN")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("error")
                .long("error")
                .help("Sets the level of logging to ERROR")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("off")
                .long("off")
                .help("Turns off logging")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let log_level = match () {
        _ if matches.get_flag("debug") => LevelFilter::Debug,
        _ if matches.get_flag("info") => LevelFilter::Info,
        _ if matches.get_flag("warn") => LevelFilter::Warn,
        _ if matches.get_flag("error") => LevelFilter::Error,
        _ if matches.get_flag("off") => LevelFilter::Off,
        _ => LevelFilter::Info,
    };

    env_logger::Builder::from_env(Env::default().default_filter_or(log_level.to_string())).init();
    debug!("Logging level set to {:?}", log_level);

    info!("Loading authentication configuration...");
    let config = match auth::AuthConfig::load() {
        Ok(config) => {
            info!("Authentication configuration loaded successfully.");
            config
        }
        Err(e) => {
            error!("Failed to load authentication configuration: {}", e);
            return;
        }
    };

    info!("Requesting access token...");
    match config.get_access_token().await {
        Ok(token) => {
            info!("Access token received successfully.");
            println!("Access Token: {}", token);
        }
        Err(e) => {
            error!("Failed to get access token: {}", e);
        }
    }

    info!("template_cli has finished execution.");
}
