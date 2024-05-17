use reqwest::blocking::Client;
use std::rc::Rc;

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

    pub fn admin(&self) {
        let url = format!("{}/admin", self.url);
        let response = self.client.get(url).send().unwrap();
        println!("response: {:?}", response.text());
    }

    pub fn user(&self) {
        let response = self.client.get(self.url.to_owned()).send().unwrap();
        println!("response: {:?}", response.text());
    }
}
