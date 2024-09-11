use actix_web::HttpResponse;

#[tracing::instrument(name = "hello")]
pub async fn hello() -> Result<HttpResponse, actix_web::Error> {
    // TODO implement
    Ok(HttpResponse::Ok().finish())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn hello_works() {
        let response = hello().await.unwrap();
        assert!(response.status().is_success())
    }
}
