use super::di::SERVICES;
use crate::common::{debug::get_user_token, debug::is_debug, token::Token};
use crate::database::users::User;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

#[derive(Debug)]
pub enum AuthError {
    MissingToken,
    InvalidToken,
    Internal,
}

async fn extract_authorization(req: &Request<'_>) -> Option<String> {
    if is_debug() {
        let token = get_user_token(SERVICES.get().await).await;
        return Some(format!("GoogleLogin auth={}", token));
    }
    match req.headers().get_one("Authorization") {
        Some(authorization) => Some(authorization.to_owned()),
        None => {
            if is_debug() {
                let token = get_user_token(SERVICES.get().await).await;
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
                    match SERVICES.get().await.user_service.get_user(token).await {
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
                match SERVICES.get().await.user_service.get_user(token).await {
                    Err(_) => Outcome::Forward(()),
                    Ok(user) => Outcome::Success(AuthUiUser { user: user }),
                }
            }
        }
    }
}
