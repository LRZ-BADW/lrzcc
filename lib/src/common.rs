use std::fmt::Debug;

use anyhow::Context;
use avina_wire::error::ErrorResponse;
use reqwest::{
    Method, StatusCode,
    blocking::{Client, Response},
};
use serde::{de::DeserializeOwned, ser::Serialize};

use crate::error::ApiError;

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
            format!("Could not serialize json request body from {data:?}"),
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
            "Could not retrieve response text on unexpected status code {status}.",
        ))?;
        let err_resp: ErrorResponse = serde_json::from_str(text.as_str())
            .context(format!(
                "Unexpected status code {status} without error message.",
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
        .context(format!("Could not parse response text: {text}"))?;
    Ok(u)
}
