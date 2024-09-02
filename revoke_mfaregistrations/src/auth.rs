use anyhow::{Context, Result};
use log::{debug, error, info};
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize)]
struct AccessTokenResponse {
    access_token: String,
}

pub async fn get_access_token(
    tenant_id: &str,
    client_id: &str,
    client_secret: &str,
) -> Result<String> {
    let client = Client::new();
    let url = format!(
        "https://login.microsoftonline.com/{}/oauth2/v2.0/token",
        tenant_id
    );

    let params = [
        ("client_id", client_id),
        ("client_secret", client_secret),
        ("scope", "https://graph.microsoft.com/.default"),
        ("grant_type", "client_credentials"),
    ];

    info!("Requesting access token for tenant ID: {}", tenant_id);
    debug!("Request URL: {}", url);
    debug!("Request parameters: {:?}", params);

    let response = client
        .post(&url)
        .form(&params)
        .send()
        .await
        .context("Failed to send request to obtain access token")?;

    if response.status().is_success() {
        info!("Access token request successful.");
        let token_response = response
            .json::<AccessTokenResponse>()
            .await
            .context("Failed to parse access token response")?;
        debug!("Received access token: {}", token_response.access_token);
        Ok(token_response.access_token)
    } else {
        let error_text = response
            .text()
            .await
            .context("Failed to read error response text")?;
        error!("HTTP error during access token request: {}", error_text);
        Err(anyhow::anyhow!("HTTP error: {}", error_text))
    }
}
