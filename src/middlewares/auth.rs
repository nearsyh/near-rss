use crate::common::error::to_internal_error;
use crate::common::token::Token;
use crate::user::UserService;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::error::ErrorForbidden;
use actix_web::http::header::AUTHORIZATION;
use actix_web::{web, HttpMessage};
use actix_web_lab::middleware::Next;

#[derive(Clone)]
pub struct AuthUser {
    pub id: String,
    pub email: String,
    pub token: String,
}

pub async fn reject_anonymous_user(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let header_value = req
        .headers()
        .get(AUTHORIZATION)
        .ok_or(ErrorForbidden("Missing authorization in header"))?;
    let header_value_str = header_value.to_str().map_err(to_internal_error)?;
    let token = header_value_str
        .strip_prefix("GoogleLogin auth=")
        .ok_or(ErrorForbidden("Missing token in header"))?;

    if Token::is_valid(token) {
        let user = req
            .app_data::<web::Data<UserService>>()
            .expect("Failed to get state")
            .get_user(token)
            .await
            .map_err(to_internal_error)?;
        req.extensions_mut().insert(AuthUser {
            id: user.id,
            email: user.email,
            token: user.token,
        });
        next.call(req).await
    } else {
        Err(ErrorForbidden("Unauthorized"))
    }
}
