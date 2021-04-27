use crate::common::error::Errors;
use crate::database::users::{new_user_repository, User, UserRepository};
use anyhow::{Error, Result};

pub struct UserCreds {
    pub sid: String,
    pub lsid: String,
    pub cltoken: String,
}

#[rocket::async_trait]
pub trait UserService {
    async fn login(&self, email: &str, password: &str) -> Result<UserCreds>;

    async fn register(&self, email: &str, password: &str) -> Result<()>;

    async fn get_user(&self, token: &str) -> Result<User>;
}

struct UserServiceImpl {
    user_repository: Box<dyn UserRepository + Send + Sync>,
}

#[rocket::async_trait]
impl UserService for UserServiceImpl {
    async fn login(&self, email: &str, password: &str) -> Result<UserCreds> {
        match self.user_repository.get_user_by_email(email).await? {
            None => Err(Error::new(Errors::NonExistUser {
                email: email.to_string(),
            })),
            Some(ref user) => {
                let token = user.token();
                if user.match_password(password) {
                    Ok(UserCreds {
                        sid: token.sid.clone(),
                        lsid: token.sid.clone(),
                        cltoken: token.to_string(),
                    })
                } else {
                    Err(Error::new(Errors::WrongPassword))
                }
            }
        }
    }

    async fn get_user(&self, token: &str) -> Result<User> {
        match self.user_repository.get_user_by_token(token).await? {
            None => Err(Error::new(Errors::InvalidToken {
                token: token.to_string(),
            })),
            Some(user) => Ok(user),
        }
    }

    async fn register(&self, email: &str, password: &str) -> Result<()> {
        self.user_repository.create_user(email, password).await?;
        Ok(())
    }
}

pub async fn new_user_service() -> Box<dyn UserService + Send + Sync> {
    let repository = new_user_repository("sqlite::memory:").await.unwrap();
    Box::new(UserServiceImpl {
        user_repository: repository,
    })
}
