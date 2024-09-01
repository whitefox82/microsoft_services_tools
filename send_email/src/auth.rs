use anyhow::{Context, Result};
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
    verbose: bool,
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

    if verbose {
        println!("Requesting access token from: {}", url);
    }

    let response = client
        .post(&url)
        .form(&params)
        .send()
        .await
        .context("Failed to send request to obtain access token")?;

    if response.status().is_success() {
        let token_response = response
            .json::<AccessTokenResponse>()
            .await
            .context("Failed to parse access token response")?;
        Ok(token_response.access_token)
    } else {
        let error_text = response
            .text()
            .await
            .context("Failed to read error response text")?;
        Err(anyhow::anyhow!("HTTP error: {}", error_text))
    }
}
