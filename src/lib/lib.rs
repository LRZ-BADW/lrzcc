pub struct Api {
    pub url: String,
    pub token: String,
}

impl Api {
    pub fn new(url: String, token: String) -> Self {
        Api { url, token }
    }

    pub fn hello_admin(self) {
        println!("Hello, admin!");
    }

    pub fn hello_user(self) {
        println!("Hello, user!");
    }
}
