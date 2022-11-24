use crate::common::error::to_internal_error;
use crate::common::{token::Token, Services};
use crate::database::users::User;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::error::ErrorForbidden;
use actix_web::http::header::AUTHORIZATION;
use actix_web::{web, HttpMessage};
use actix_web_lab::middleware::Next;

#[derive(Clone)]
pub struct AuthUser {
    pub user: User,
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
            .app_data::<web::Data<Services>>()
            .expect("Failed to get state")
            .user_service
            .get_user(token)
            .await
            .map_err(to_internal_error)?;
        req.extensions_mut().insert(AuthUser { user });
        next.call(req).await
    } else {
        Err(ErrorForbidden("Unauthorized"))
    }
}
