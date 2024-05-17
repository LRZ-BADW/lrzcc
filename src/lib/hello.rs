use crate::{ApiError, ErrorResponse};
use anyhow::Context;
use reqwest::blocking::Client;
use reqwest::StatusCode;
use serde::Deserialize;
use std::fmt::Display;
use std::rc::Rc;

#[derive(Deserialize)]
pub struct Hello {
    pub message: String,
}

impl Display for Hello {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.message.as_str())
    }
}

pub struct HelloApi {
    pub url: String,
    pub client: Rc<Client>,
}

impl HelloApi {
    pub fn new(base_url: &str, client: &Rc<Client>) -> HelloApi {
        HelloApi {
            url: format!("{}/hello", base_url),
            client: Rc::clone(client),
        }
    }

    pub fn admin(&self) -> Result<Hello, ApiError> {
        let url = format!("{}/admin", self.url);
        let response = self
            .client
            .get(url)
            .send()
            .context("Could not send request.")?;
        let status = response.status();
        if status != StatusCode::OK {
            let text = response
                .text()
                .context(
                    format!("Could not retrieve response text on unexpected status code {}.",
                        status)
                    )?;
            let err_resp: ErrorResponse = serde_json::from_str(text.as_str())
                .context(format!(
                "Unexpected status code {} without error message.",
                status,
            ))?;
            return Err(ApiError::ResponseError(err_resp.detail));
        }
        let text = response
            .text()
            .context("Could not retrieve response text.")?;
        let hello: Hello = serde_json::from_str(text.as_str())
            .context(format!("Could not parse response text: {}", text))?;
        Ok(hello)
    }

    pub fn user(&self) -> Result<Hello, ApiError> {
        let response = self
            .client
            .get(self.url.clone())
            .send()
            .context("Could not send request.")?;
        let status = response.status();
        if status != StatusCode::OK {
            let text = response
                .text()
                .context(
                    format!("Could not retrieve response text on unexpected status code {}.",
                        status)
                    )?;
            let err_resp: ErrorResponse = serde_json::from_str(text.as_str())
                .context(format!(
                "Unexpected status code {} without error message.",
                status
            ))?;
            return Err(ApiError::ResponseError(err_resp.detail));
        }
        let text = response
            .text()
            .context("Could not retrieve response text.")?;
        let hello: Hello = serde_json::from_str(text.as_str())
            .context(format!("Could not parse response text: {}", text))?;
        Ok(hello)
    }
}
