use rocket::http::Status;
use rocket::request::{self, FromRequest, Outcome, Request};

struct AuthToken<'r>(&'r str);

#[derive(Debug)]
enum AuthTokenError {
  Missing,
  Invalid,
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
      Some(token) => Outcome::Success(AuthToken(""))
    }
  }
}