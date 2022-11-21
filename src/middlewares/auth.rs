use super::di::SERVICES;
use crate::common::error::to_internal_error;
use crate::common::{debug::get_user_token, debug::is_debug, token::Token, Services};
use crate::database::users::User;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::error::{ErrorForbidden, ErrorInternalServerError, InternalError};
use actix_web::http::header::AUTHORIZATION;
use actix_web::{web, HttpMessage};
use actix_web_lab::middleware::Next;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::State;

#[derive(Debug)]
pub enum AuthError {
    MissingToken,
    InvalidToken,
    Internal,
}

async fn extract_authorization(req: &Request<'_>) -> Option<String> {
    let services = req.guard::<&State<Services>>().await.unwrap();
    if is_debug() {
        let token = get_user_token(services).await;
        return Some(format!("GoogleLogin auth={}", token));
    }
    match req.headers().get_one("Authorization") {
        Some(authorization) => Some(authorization.to_owned()),
        None => {
            if is_debug() {
                let token = get_user_token(services).await;
                Some(format!("GoogleLogin auth={}", token))
            } else {
                None
            }
        }
    }
}

fn extract_token(authorization_value: &str) -> Option<&str> {
    authorization_value.strip_prefix("GoogleLogin auth=")
}

#[derive(Clone)]
pub struct AuthUser {
    pub user: User,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthUser {
    type Error = AuthError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match extract_authorization(req).await {
            None => Outcome::Failure((Status::Forbidden, AuthError::MissingToken)),
            Some(ref authorization) => match extract_token(authorization) {
                Some(token) => {
                    if !Token::is_valid(&token) {
                        return Outcome::Failure((Status::Forbidden, AuthError::InvalidToken));
                    }
                    let services = req.guard::<&State<Services>>().await.unwrap();
                    match services.user_service.get_user(token).await {
                        Err(_) => {
                            Outcome::Failure((Status::InternalServerError, AuthError::Internal))
                        }
                        Ok(user) => Outcome::Success(AuthUser { user: user }),
                    }
                }
                None => Outcome::Failure((Status::Forbidden, AuthError::InvalidToken)),
            },
        }
    }
}

pub struct AuthUiUser {
    pub user: User,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthUiUser {
    type Error = AuthError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.cookies().get_pending("cltoken") {
            None => Outcome::Forward(()),
            Some(ref cookie) => {
                let token = cookie.value();
                if !Token::is_valid(&token) {
                    return Outcome::Forward(());
                }
                let services = req.guard::<&State<Services>>().await.unwrap();
                match services.user_service.get_user(token).await {
                    Err(_) => Outcome::Forward(()),
                    Ok(user) => Outcome::Success(AuthUiUser { user: user }),
                }
            }
        }
    }
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
