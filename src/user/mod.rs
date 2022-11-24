use crate::common::error::Errors;
use crate::common::new_id;
use crate::common::token::Token;
use anyhow::{Error, Result};
use sqlx::SqlitePool;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

#[derive(sqlx::FromRow, PartialEq, Eq, Debug, Clone)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password_hash: String,
    pub token: String,
}

impl User {
    fn hash_password(password: &str) -> String {
        let mut hasher = DefaultHasher::new();
        hasher.write(password.as_bytes());
        hasher.finish().to_string()
    }

    pub fn new(id: &str, email: &str, password: &str) -> User {
        User {
            id: id.to_string(),
            email: email.to_string(),
            password_hash: User::hash_password(password),
            token: Token::new(id).to_string(),
        }
    }

    pub fn token(&self) -> Token {
        Token::parse(&self.token).unwrap()
    }

    pub fn match_password(&self, password: &str) -> bool {
        self.password_hash.eq(&User::hash_password(password))
    }
}

pub struct UserCreds {
    pub sid: String,
    pub lsid: String,
    pub cltoken: String,
}

pub struct UserService {
    pool: SqlitePool,
}

impl UserService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn login(&self, email: &str, password: &str) -> Result<UserCreds> {
        match self.get_user_by_email(email).await? {
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

    pub async fn register(&self, email: &str, password: &str) -> Result<()> {
        self.create_user(email, password).await?.unwrap();
        Ok(())
    }

    pub async fn get_user(&self, token: &str) -> Result<User> {
        match self.get_user_by_token(token).await? {
            None => Err(Error::new(Errors::InvalidToken {
                token: token.to_string(),
            })),
            Some(user) => Ok(user),
        }
    }

    async fn create_user(&self, email: &str, password: &str) -> Result<Option<User>> {
        if let Some(user) = self.get_user_by_email(email).await? {
            return Ok(Some(user));
        }
        let new_user = User::new(&new_id(10), email, password);
        sqlx::query(
            "INSERT INTO Users (id, email, password_hash, token) 
       VALUES(?, ?, ?, ?)",
        )
        .bind(&new_user.id)
        .bind(&new_user.email)
        .bind(&new_user.password_hash)
        .bind(&new_user.token)
        .execute(&self.pool)
        .await?;
        Ok(Some(new_user))
    }

    async fn get_user_by_email(&self, email: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM Users WHERE email = ?")
            .bind(email)
            .fetch_optional(&self.pool)
            .await?;
        Ok(user)
    }

    async fn get_user_by_token(&self, token: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM Users WHERE token = ?")
            .bind(token)
            .fetch_optional(&self.pool)
            .await?;
        Ok(user)
    }
}
