use crate::configuration::OpenStackSettings;
use anyhow::Context;
use jzon::object;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use reqwest::ClientBuilder;
use std::time::Instant;
use tokio::sync::RwLock;

struct Token {
    settings: OpenStackSettings,
    token: String,
    renewed_at: Instant,
}

impl Token {
    async fn new(settings: &OpenStackSettings) -> Result<Self, anyhow::Error> {
        Ok(Self {
            settings: settings.clone(),
            token: issue_token(settings).await?,
            renewed_at: Instant::now(),
        })
    }

    async fn renew(&mut self) -> Result<(), anyhow::Error> {
        self.token = issue_token(&self.settings).await?;
        self.renewed_at = Instant::now();
        Ok(())
    }

    fn is_expired(&self) -> bool {
        self.renewed_at.elapsed().as_secs() > 3600
    }

    fn get(&self) -> String {
        self.token.clone()
    }
}

#[tracing::instrument(name = "Issue an OpenStack token", skip(settings))]
pub async fn issue_token(
    settings: &OpenStackSettings,
) -> Result<String, anyhow::Error> {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    let client = ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap();
    let url = format!("{}/v3/auth/tokens/", settings.keystone_endpoint);
    let data = object! {
        "auth": {
            "identity": {
                "methods": ["password"],
                "password": {
                    "user": {
                        "name": settings.username.clone(),
                        "domain": {"name": settings.domain.clone()},
                        "password": settings.password.clone(),
                    }
                }
            },
            "scope": {
                "project": {
                    "name": settings.project.clone(),
                    "domain": {"id": settings.domain_id.clone()}
                }
            }
        }
    };
    let response = match client
        .post(url.as_str())
        .body(data.to_string())
        .send()
        .await
        .context("")
    {
        Ok(response) => response,
        Err(error) => {
            return Err(anyhow::anyhow!(
                "Could not complete authentication request: {}",
                error.root_cause()
            ));
        }
    };
    if !response.status().is_success() {
        return Err(anyhow::anyhow!(
            "Failed to authenticate, returned code {}",
            response.status().as_u16()
        ));
    }
    let token = match response.headers().get("X-Subject-Token") {
        Some(token) => token.to_str().unwrap().to_string(),
        None => {
            return Err(anyhow::anyhow!(
                "No token in authentication response header"
            ))
        }
    }
    .trim()
    .to_string();
    Ok(token)
}
