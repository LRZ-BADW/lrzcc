use jzon::object;
use reqwest::blocking::{Client, ClientBuilder};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use std::convert::AsRef;
use std::str::FromStr;

#[derive(Clone)]
struct TokenInner {
    url: String,
    client: Client,
}

pub(crate) struct Token {
    token: String,
    inner: Option<TokenInner>,
}

impl Token {
    pub(crate) fn new(
        auth_url: &str,
        username: &str,
        password: &str,
        project_name: &str,
        user_domain_name: &str,
        project_domain_id: &str,
    ) -> Result<Self, anyhow::Error> {
        let mut headers = HeaderMap::new();
        headers
            .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        let client = ClientBuilder::new()
            .default_headers(headers)
            .build()
            .unwrap();
        let url = format!("{}/auth/tokens/", auth_url);
        let data = object! {
            "auth": {
                "identity": {
                    "methods": ["password"],
                    "password": {
                        "user": {
                            "name": username,
                            "domain": {"name": user_domain_name},
                            "password": password,
                        }
                    }
                },
                "scope": {
                    "project": {
                        "name": project_name,
                        "domain": {"id": project_domain_id}
                    }
                }
            }
        };
        // TODO better error handling
        let response = client
            .post(url.as_str())
            .body(data.to_string())
            .send()
            .unwrap();
        let token = response
            .headers()
            .get("X-Subject-Token")
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        Ok(Self {
            token,
            inner: Some(TokenInner { client, url }),
        })
    }
}

impl FromStr for Token {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            token: s.to_string(),
            inner: None,
        })
    }
}

impl AsRef<str> for Token {
    fn as_ref(&self) -> &str {
        self.token.as_str()
    }
}

impl Drop for Token {
    fn drop(&mut self) {
        if let Some(inner) = self.inner.clone() {
            let value = HeaderValue::from_str(self.token.as_str()).unwrap();
            inner
                .client
                .delete(inner.url)
                .header("X-Auth-Token", value.clone())
                .header("X-Subject-Token", value)
                .send()
                .unwrap();
        }
    }
}
