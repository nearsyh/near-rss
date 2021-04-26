use crate::common::token::Token;
use crate::database::users::User;
use crate::services::users::new_user_service;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

#[derive(Debug)]
pub enum AuthError {
    MissingToken,
    InvalidToken,
    NonExistUser,
    Internal,
}

pub struct AuthUser {
    pub user: User,
}

fn extract_token(authorization_value: &str) -> Option<&str> {
    authorization_value.strip_prefix("GoogleLogin auth=")
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthUser {
    type Error = AuthError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("Authorization") {
            None => Outcome::Failure((Status::Forbidden, AuthError::MissingToken)),
            Some(authorization) => match extract_token(authorization) {
                Some(token) => {
                    if !Token::is_valid(&token) {
                        return Outcome::Failure((Status::Forbidden, AuthError::InvalidToken));
                    }
                    match new_user_service().await.get_user(token).await {
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
