use reqwest::blocking::{Client, ClientBuilder};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};

pub struct Api {
    url: String,
    // token: String,
    client: Client,
}

impl Api {
    pub fn new(url: String, token: String) -> Self {
        let mut headers = HeaderMap::new();
        headers
            .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        let value = match HeaderValue::from_str(token.trim()) {
            Ok(value) => value,
            Err(e) => {
                println!("Error: {}", e);
                HeaderValue::from_static("")
            }
        };
        headers.insert("X-Auth-Token", value);
        let client = ClientBuilder::new()
            .default_headers(headers)
            .build()
            .unwrap();
        Api { url, client }
    }

    pub fn hello_admin(self) {
        let url = format!("{}/hello/admin", self.url);
        let response = self.client.get(url).send().unwrap();
        println!("response: {:?}", response.text());
    }

    pub fn hello_user(self) {
        println!("Hello, user!");
    }
}
