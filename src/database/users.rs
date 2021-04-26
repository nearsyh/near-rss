use crate::common::new_id;
use crate::common::token::Token;
use anyhow::Result;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

pub struct User {
  pub id: String,
  pub email: String,
  pub password_hash: String,
  pub token: Token,
}

impl User {
  pub fn new(id: &str, email: &str, password: &str) -> User {
    let mut hasher = DefaultHasher::new();
    hasher.write(password.as_bytes());
    User {
      id: id.to_string(),
      email: email.to_string(),
      password_hash: hasher.finish().to_string(),
      token: Token::new(id),
    }
  }
}

#[rocket::async_trait]
pub trait UserRepository {
  async fn create_user(&mut self, email: &str, password: &str) -> Result<Option<User>>;
  async fn update_user(&mut self, user: &User) -> Result<Option<User>>;
  async fn get_user_by_id(&mut self, id: &str) -> Result<Option<User>>;
  async fn get_user_by_email(&mut self, email: &str) -> Result<Option<User>>;
  async fn get_user_by_token(&mut self, token: &str) -> Result<Option<User>>;
}

use sqlx::Connection;
use sqlx::SqliteConnection;

struct UserRepositorySqlite {
  connection: SqliteConnection,
}

unsafe impl Send for UserRepositorySqlite {}
unsafe impl Sync for UserRepositorySqlite {}

impl UserRepositorySqlite {
  pub async fn new(path: &str) -> Result<UserRepositorySqlite> {
    let mut connection = SqliteConnection::connect(path).await?;
    sqlx::query(
      "CREATE TABLE IF NOT EXISTS Users (
          id TEXT NOT NULL PRIMARY KEY,
          email TEXT NOT NULL,
          password_hash TEXT NOT NULL,
          token TEXT)",
    )
    .execute(&mut connection)
    .await?;
    Ok(UserRepositorySqlite {
      connection: connection,
    })
  }
}

#[rocket::async_trait]
impl UserRepository for UserRepositorySqlite {
  async fn create_user(&mut self, email: &str, password: &str) -> Result<Option<User>> {
    if self.get_user_by_email(email).await?.is_some() {
      return Ok(None);
    }
    let new_user = User::new(&new_id(10), email, password);
    sqlx::query(
      "INSERT INTO Users (id, email, password_hash, token) 
       VALUES(?, ?, ?, ?)",
    )
    .bind(&new_user.id)
    .bind(&new_user.email)
    .bind(&new_user.password_hash)
    .bind(&new_user.token.to_string())
    .execute(&mut self.connection)
    .await?;
    Ok(Some(new_user))
  }

  async fn update_user(&mut self, user: &User) -> Result<Option<User>> {
    Ok(None)
  }

  async fn get_user_by_email(&mut self, email: &str) -> Result<Option<User>> {
    
  }

  async fn get_user_by_id(&mut self, id: &str) -> Result<Option<User>> {
    Ok(None)
  }

  async fn get_user_by_token(&mut self, token: &str) -> Result<Option<User>> {
    Ok(None)
  }
}

pub async fn new_user_repository(path: &str) -> Result<Box<dyn UserRepository + Send + Sync>> {
  let repository = UserRepositorySqlite::new(path).await?;
  Ok(Box::new(repository))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[rocket::async_test]
  async fn create_user_should_work() {
    let mut repository = new_user_repository("sqlite::memory:").await.unwrap();
    let created_user = repository
      .create_user("email", "password")
      .await
      .unwrap()
      .unwrap();
    assert_eq!(created_user.email, "email");
    assert_ne!(created_user.password_hash, "password");
  }
}
