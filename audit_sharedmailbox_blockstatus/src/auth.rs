use anyhow::{Context, Result};
use log::{debug, info};
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
    debug!("Creating HTTP client");
    let client = Client::new();

    let url = format!(
        "https://login.microsoftonline.com/{}/oauth2/v2.0/token",
        tenant_id
    );
    debug!("Formatted URL for token request: {}", url);

    let params = [
        ("client_id", client_id),
        ("client_secret", client_secret),
        ("scope", "https://graph.microsoft.com/.default"),
        ("grant_type", "client_credentials"),
    ];
    debug!("Request parameters: {:?}", params);

    debug!("Sending POST request to obtain access token");
    let response = client
        .post(&url)
        .form(&params)
        .send()
        .await
        .context("Failed to send request to obtain access token")?;

    debug!("Response status: {}", response.status());

    if response.status().is_success() {
        debug!("Parsing response as AccessTokenResponse");
        let token_response: AccessTokenResponse = response
            .json()
            .await
            .context("Failed to parse access token response")?;

        info!("Access token obtained successfully");
        debug!("Access token: {}", token_response.access_token);

        Ok(token_response.access_token)
    } else {
        debug!("Non-successful response status: {}", response.status());
        let error_text = response
            .text()
            .await
            .context("Failed to read error response text")?;

        debug!("Error response text: {}", error_text);

        Err(anyhow::anyhow!("HTTP error: {}", error_text))
    }
}
