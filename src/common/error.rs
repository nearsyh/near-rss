use thiserror::Error;

#[derive(Error, Debug)]
pub enum Errors {
  #[error("User[email={email:?}] doesn't exist.")]
  NonExistUser {
    email: String
  },
  #[error("Invalid token {token:?}")]
  InvalidToken {
    token: String
  },
  #[error("Wrong password.")]
  WrongPassword
}

unsafe impl Send for Errors {}
unsafe impl Sync for Errors {}