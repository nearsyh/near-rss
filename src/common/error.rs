use std::fmt::{Debug, Display};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Errors {
    #[error("User[email={email:?}] doesn't exist.")]
    NonExistUser { email: String },
    #[error("Invalid token {token:?}")]
    InvalidToken { token: String },
    #[error("Wrong password.")]
    WrongPassword,
    #[error("Subscription is not found")]
    SubscriptionNotFound,
}

unsafe impl Send for Errors {}

unsafe impl Sync for Errors {}

pub fn to_internal_error<T>(e: T) -> actix_web::Error
where
    T: Debug + Display + 'static,
{
    actix_web::error::ErrorInternalServerError(e)
}

pub fn to_forbidden_error(msg: String) -> impl FnOnce() -> actix_web::Error {
    || actix_web::error::ErrorForbidden(msg)
}
