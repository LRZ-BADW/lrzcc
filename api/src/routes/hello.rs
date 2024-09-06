use actix_web::HttpResponse;

#[tracing::instrument(name = "hello")]
pub async fn hello() -> Result<HttpResponse, actix_web::Error> {
    // TODO implement
    Ok(HttpResponse::Ok().finish())
}

// TODO tests
