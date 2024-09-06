use crate::error::{ApiError, ErrorResponse};
use anyhow::Context;
use reqwest::blocking::{Client, Response};
use reqwest::{Method, StatusCode};
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use std::borrow::Borrow;
use std::fmt::{Debug, Display};

#[derive(serde::Serialize, Debug)]
pub(crate) struct SerializableFoo {}
macro_rules! SerializableNone {
    () => {
        None::<crate::common::SerializableFoo>
    };
}
pub(crate) use SerializableNone;

pub(crate) fn request_bare<T>(
    client: &Client,
    method: Method,
    url: &str,
    data: Option<T>,
    expected_status: StatusCode,
) -> Result<Response, ApiError>
where
    T: Serialize + Debug,
{
    let mut request = client.request(method, url);
    if let Some(data) = data {
        request = request.body(serde_json::to_string(&data).context(
            format!("Could not serialize json request body from {:?}", data),
        )?);
    }
    let response = match request.send().context("") {
        Ok(response) => response,
        Err(err) => {
            let detail =
                format!("Could not complete request: {}", err.root_cause());
            return Err(ApiError::ResponseError(detail));
        }
    };
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
    Ok(response)
}

pub(crate) fn request<T, U>(
    client: &Client,
    method: Method,
    url: &str,
    data: Option<T>,
    expected_status: StatusCode,
) -> Result<U, ApiError>
where
    T: Serialize + Debug,
    U: DeserializeOwned,
{
    let response = request_bare(client, method, url, data, expected_status)?;
    let text = response
        .text()
        .context("Could not retrieve response text.")?;
    let u: U = serde_json::from_str(text.as_str())
        .context(format!("Could not parse response text: {}", text))?;
    Ok(u)
}

#[allow(dead_code)]
pub(crate) fn display_option<T: Display>(option: &Option<T>) -> String {
    match option {
        Some(value) => value.to_string(),
        None => "".to_string(),
    }
}

pub(crate) fn is_true(b: impl Borrow<bool>) -> bool {
    *b.borrow()
}

pub(crate) fn is_false(b: impl Borrow<bool>) -> bool {
    !b.borrow()
}
