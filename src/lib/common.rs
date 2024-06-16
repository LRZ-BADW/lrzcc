use crate::error::{ApiError, ErrorResponse};
use anyhow::Context;
use reqwest::blocking::Client;
use reqwest::{Method, StatusCode};
use serde::de::DeserializeOwned;
use std::fmt::Display;

pub(crate) fn request<T>(
    client: &Client,
    method: Method,
    url: &str,
    expected_status: StatusCode,
) -> Result<T, ApiError>
where
    T: DeserializeOwned,
{
    let response = client
        .request(method, url)
        .send()
        .context("Could not send request.")?;
    let status = response.status();
    if status != expected_status {
        let text = response.text().context(format!(
            "Could not retrieve response text on unexpected status code {}.",
            status
        ))?;
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
    let t: T = serde_json::from_str(text.as_str())
        .context(format!("Could not parse response text: {}", text))?;
    Ok(t)
}

#[allow(dead_code)]
pub(crate) fn display_option<T: Display>(option: &Option<T>) -> String {
    match option {
        Some(value) => value.to_string(),
        None => "".to_string(),
    }
}
