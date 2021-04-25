use crate::services::users::new_user_service;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

pub struct AuthToken<'r>(&'r str);

#[derive(Debug)]
pub enum AuthTokenError {
  Missing,
  Invalid,
}

impl<'r> AuthToken<'r> {
  fn extract_token(authorization_value: &'r str) -> Option<&'r str> {
    authorization_value.strip_prefix("GoogleLogin auth=")
  }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthToken<'r> {
  type Error = AuthTokenError;

  async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
    if !req.uri().path().starts_with("/reader") {
      return Outcome::Success(AuthToken(""));
    }
    match req.headers().get_one("Authorization") {
      None => Outcome::Failure((Status::Forbidden, AuthTokenError::Missing)),
      Some(authorization) => match AuthToken::extract_token(authorization) {
        Some(token) => {
          if new_user_service().is_token_valid(token) {
            Outcome::Success(AuthToken(token))
          } else {
            Outcome::Failure((Status::Forbidden, AuthTokenError::Invalid))
          }
        }
        None => Outcome::Failure((Status::Forbidden, AuthTokenError::Invalid)),
      },
    }
  }
}
