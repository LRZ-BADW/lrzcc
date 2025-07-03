use avina::{Api, Token};
use dioxus::prelude::*;
use std::str::FromStr;

fn main() {
    launch(app);
}

fn app() -> Element {
    let future = use_resource(move || async move {
        let mut eval = document::eval(
            r#"
            window.addEventListener("message", function(event) {
                let token = event.data;
                dioxus.send(token);
            });
            window.parent.postMessage("request-token", "*");
            "#,
        );
        let token_str: String = eval.recv().await.unwrap();
        let token = Token::from_str(&token_str).unwrap();
        let api = Api::new(
            "http://localhost:8000/api".to_string(),
            token,
            None,
            None,
        )
        .unwrap();
        api.user.me().await.unwrap()
    });
    match future.read_unchecked().as_ref() {
        Some(user) => {
            rsx! { p { "Hello {user.name} from Dioxus!" } }
        }
        _ => rsx! { p { "No token provided!" } },
    }
}
