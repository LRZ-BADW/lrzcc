use std::time::Instant;

use anyhow::Context;
use jzon::object;
use reqwest::{
    header::{HeaderMap, HeaderValue, CONTENT_TYPE},
    ClientBuilder,
};
use tokio::sync::RwLock;

use crate::configuration::OpenStackSettings;

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

struct TokenHandler {
    token: RwLock<Token>,
}

impl TokenHandler {
    async fn new(settings: &OpenStackSettings) -> Result<Self, anyhow::Error> {
        Ok(TokenHandler {
            token: RwLock::new(Token::new(settings).await?),
        })
    }

    async fn get(&self) -> String {
        if self.token.read().await.is_expired() {
            self.token.write().await.renew().await.unwrap();
        }
        self.token.read().await.get()
    }
}

// TODO: maybe we could also use rust-openstack at some point.
pub struct OpenStack {
    settings: OpenStackSettings,
    token: TokenHandler,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct ProjectMinimal {
    pub id: String,
    pub name: String,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Link {
    pub href: String,
    pub rel: String,
}

#[derive(Clone, Debug, serde::Deserialize)]
#[allow(unused)]
pub struct FlavorDetailed {
    #[serde(rename = "OS-FLV-DISABLED:disabled")]
    pub disabled: bool,
    pub disk: u32,
    // TODO: this does not work, why?
    // #[serde(rename = "OS-FLV-EXT-DATA:ephemeral")]
    // pub ephemeral: bool,
    #[serde(rename = "os-flavor-access:is_public")]
    pub is_public: bool,
    pub id: String,
    pub links: Vec<Link>,
    pub name: String,
    pub ram: u32,
    // TODO: this does not work, why?
    // pub swap: u32,
    pub vcpus: u32,
    pub rxtx_factor: f32,
    pub description: Option<String>,
    // TODO: this is a more complicated field.
    // "extra_specs": {}
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct FlavorDetailedList {
    flavors: Vec<FlavorDetailed>,
}

impl OpenStack {
    pub async fn new(
        settings: OpenStackSettings,
    ) -> Result<Self, anyhow::Error> {
        Ok(OpenStack {
            token: TokenHandler::new(&settings).await?,
            settings,
        })
    }

    async fn client(&self) -> Result<reqwest::Client, anyhow::Error> {
        let mut headers = HeaderMap::new();
        headers
            .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            "X-Auth-Token",
            HeaderValue::from_str(self.token.get().await.as_str())
                .context("Could not create token header")?,
        );
        ClientBuilder::new()
            .default_headers(headers)
            .build()
            .context("Could not create client")
    }

    pub async fn validate_user_token(
        &self,
        token: &str,
    ) -> Result<ProjectMinimal, anyhow::Error> {
        #[derive(Debug, serde::Deserialize)]
        struct ValidateResponseToken {
            project: ProjectMinimal,
        }
        #[derive(Debug, serde::Deserialize)]
        struct ValidateResponse {
            token: ValidateResponseToken,
        }

        let client = self.client().await?;
        let url = format!("{}/auth/tokens/", self.settings.keystone_endpoint);
        let response = client
            .get(url.as_str())
            .header("X-Subject-Token", token)
            .send()
            .await
            .context("Could not validate user token")?;
        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Failed to validate user token, returned code {}",
                response.status().as_u16()
            ));
        }
        let project: ValidateResponse = serde_json::from_str(
            response
                .text()
                .await
                .context("Could not read response text")?
                .as_str(),
        )
        .context("Could not parse response")?;
        Ok(project.token.project)
    }

    pub async fn get_flavors(
        &self,
    ) -> Result<Vec<FlavorDetailed>, anyhow::Error> {
        let client = self.client().await?;
        let url = format!(
            "{}/v2.1/flavors/detail?is_public=False",
            self.settings.nova_endpoint
        );
        let response = client
            .get(url.as_str())
            .send()
            .await
            .context("Could not retrieve flavor list")?;
        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Failed to validate user token, returned code {}",
                response.status().as_u16()
            ));
        }
        let flavors: FlavorDetailedList = serde_json::from_str(
            response
                .text()
                .await
                .context("Could not read response text")?
                .as_str(),
        )
        .context("Could not parse response")?;
        Ok(flavors.flavors)
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
    let url = format!("{}/auth/tokens/", settings.keystone_endpoint);
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
