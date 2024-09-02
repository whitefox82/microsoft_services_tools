use dotenv::dotenv;
use log::{debug, error, info, warn};
use reqwest::Client;
use serde::Deserialize;
use std::env;

pub struct AuthConfig {
    pub client_id: String,
    pub client_secret: String,
    pub tenant_id: String,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
}

impl AuthConfig {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let current_dir = match env::current_dir() {
            Ok(dir) => {
                debug!("Current directory: {}", dir.display());
                dir
            }
            Err(e) => {
                error!("Failed to get current directory: {}", e);
                return Err(Box::new(e));
            }
        };

        debug!(
            "Attempting to load .env file from {}",
            current_dir.display()
        );
        match dotenv::from_path(current_dir.join(".env")) {
            Ok(_) => debug!(".env file loaded successfully."),
            Err(e) => {
                warn!("Failed to load .env file: {}", e);
                return Err(Box::new(e));
            }
        }

        let client_id = match env::var("CLIENT_ID") {
            Ok(val) => {
                debug!("CLIENT_ID loaded successfully.");
                val
            }
            Err(e) => {
                error!("CLIENT_ID is missing: {}", e);
                return Err(Box::new(e));
            }
        };

        let client_secret = match env::var("CLIENT_SECRET") {
            Ok(val) => {
                debug!("CLIENT_SECRET loaded successfully.");
                val
            }
            Err(e) => {
                error!("CLIENT_SECRET is missing: {}", e);
                return Err(Box::new(e));
            }
        };

        let tenant_id = match env::var("TENANT_ID") {
            Ok(val) => {
                debug!("TENANT_ID loaded successfully.");
                val
            }
            Err(e) => {
                error!("TENANT_ID is missing: {}", e);
                return Err(Box::new(e));
            }
        };

        info!("AuthConfig loaded successfully.");
        Ok(AuthConfig {
            client_id,
            client_secret,
            tenant_id,
        })
    }

    pub async fn get_access_token(&self) -> Result<String, Box<dyn std::error::Error>> {
        let client = Client::new();
        let token_url = format!(
            "https://login.microsoftonline.com/{}/oauth2/v2.0/token",
            self.tenant_id
        );

        let params = [
            ("grant_type", "client_credentials"),
            ("client_id", &self.client_id),
            ("client_secret", &self.client_secret),
            ("scope", "https://graph.microsoft.com/.default"),
        ];

        debug!("Sending request to token endpoint: {}", token_url);

        let res = match client.post(&token_url).form(&params).send().await {
            Ok(res) => {
                debug!("Received response from token endpoint.");
                res
            }
            Err(e) => {
                error!("Failed to send request to token endpoint: {}", e);
                return Err(Box::new(e));
            }
        };

        if res.status().is_success() {
            let token_response: TokenResponse = match res.json().await {
                Ok(token_response) => {
                    debug!("Token response successfully parsed.");
                    token_response
                }
                Err(e) => {
                    error!("Failed to parse token response: {}", e);
                    return Err(Box::new(e));
                }
            };
            info!("Access token retrieved successfully.");
            Ok(token_response.access_token)
        } else {
            let status = res.status();
            error!("Failed to retrieve access token: HTTP {}", status);
            Err(Box::new(res.error_for_status().unwrap_err()))
        }
    }
}
