use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::error::InternalError;
use actix_web::middleware::Next;
use actix_web::HttpResponse;

pub async fn reject_anonymous_users(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    // TODO implement actual authentication logic
    let succeed = false;
    if succeed {
        Ok(next.call(req).await?)
    } else {
        let response = HttpResponse::Unauthorized().finish();
        let e = anyhow::anyhow!("The user has not logged in");
        Err(InternalError::from_response(e, response).into())
    }
}
