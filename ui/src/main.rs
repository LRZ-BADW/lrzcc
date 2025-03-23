use dioxus::prelude::*;

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
        let token: String = eval.recv().await.unwrap();
        token
    });
    match future.read_unchecked().as_ref() {
        Some(v) => rsx! { p { "Hello from Dioxus! Token: {v}" } },
        _ => rsx! { p { "No token provided!" } },
    }
}
