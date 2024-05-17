use reqwest::blocking::Client;
use std::rc::Rc;

pub struct HelloApi {
    pub url: Rc<str>,
    pub client: Rc<Client>,
}

impl HelloApi {
    pub fn new(url: &Rc<str>, client: &Rc<Client>) -> HelloApi {
        HelloApi {
            url: Rc::clone(url),
            client: Rc::clone(client),
        }
    }

    pub fn admin(&self) {
        let url = format!("{}/hello/admin", self.url);
        let response = self.client.get(url).send().unwrap();
        println!("response: {:?}", response.text());
    }

    pub fn user(&self) {
        println!("Hello, user!");
    }
}
