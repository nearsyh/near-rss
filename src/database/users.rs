use crate::common::new_id;
use crate::common::token::Token;
use anyhow::Result;
use async_trait::async_trait;
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

#[async_trait]
pub trait UserRepository {
    async fn create_user(&self, email: &str, password: &str) -> Result<Option<User>>;
    async fn update_user(&self, user: User) -> Result<Option<User>>;
    async fn get_user_by_id(&self, id: &str) -> Result<Option<User>>;
    async fn get_user_by_email(&self, email: &str) -> Result<Option<User>>;
    async fn get_user_by_token(&self, token: &str) -> Result<Option<User>>;
}

use sqlx::SqlitePool;

struct UserRepositorySqlite {
    pool: SqlitePool,
}

unsafe impl Send for UserRepositorySqlite {}
unsafe impl Sync for UserRepositorySqlite {}

impl UserRepositorySqlite {
    pub async fn new(pool: SqlitePool) -> Result<UserRepositorySqlite> {
        Ok(UserRepositorySqlite { pool })
    }
}

#[async_trait]
impl UserRepository for UserRepositorySqlite {
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

    async fn update_user(&self, user: User) -> Result<Option<User>> {
        if self.get_user_by_id(&user.id).await?.is_none() {
            return Ok(None);
        }
        sqlx::query("UPDATE Users SET email = ?, password_hash = ?, token = ? WHERE id = ?")
            .bind(&user.email)
            .bind(&user.password_hash)
            .bind(&user.token)
            .bind(&user.id)
            .execute(&self.pool)
            .await?;
        Ok(Some(user))
    }

    async fn get_user_by_email(&self, email: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM Users WHERE email = ?")
            .bind(email)
            .fetch_optional(&self.pool)
            .await?;
        Ok(user)
    }

    async fn get_user_by_id(&self, id: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM Users WHERE id = ?")
            .bind(id)
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

pub async fn new_user_repository(
    pool: SqlitePool,
) -> Result<Box<dyn UserRepository + Send + Sync>> {
    let repository = UserRepositorySqlite::new(pool).await?;
    Ok(Box::new(repository))
}

#[cfg(test)]
mod tests {
    use super::super::in_memory_pool;
    use super::*;

    #[tokio::test]
    async fn create_user_should_work() {
        let repository = new_user_repository(in_memory_pool().await).await.unwrap();
        let created_user = repository
            .create_user("email", "password")
            .await
            .unwrap()
            .unwrap();
        assert_eq!(created_user.email, "email");
        assert_ne!(created_user.password_hash, "password");
    }

    #[tokio::test]
    async fn create_users_with_same_emails_should_not_change_password() {
        let repository = new_user_repository(in_memory_pool().await).await.unwrap();
        assert!(repository
            .create_user("email", "1")
            .await
            .unwrap()
            .is_some());
        assert!(repository
            .create_user("email", "2")
            .await
            .unwrap()
            .is_some());
        assert!(repository
            .get_user_by_email("email")
            .await
            .unwrap()
            .unwrap()
            .match_password("1"));
    }

    #[tokio::test]
    async fn get_existing_users_should_succeed() {
        let repository = new_user_repository(in_memory_pool().await).await.unwrap();
        let created_user = repository.create_user("email", "").await.unwrap().unwrap();
        assert_eq!(
            repository
                .get_user_by_id(&created_user.id)
                .await
                .unwrap()
                .unwrap(),
            created_user
        );
        assert_eq!(
            repository
                .get_user_by_email("email")
                .await
                .unwrap()
                .unwrap(),
            created_user
        );
        assert_eq!(
            repository
                .get_user_by_token(&created_user.token)
                .await
                .unwrap()
                .unwrap(),
            created_user
        );
    }

    #[tokio::test]
    async fn get_non_existing_users_should_fail() {
        let repository = new_user_repository(in_memory_pool().await).await.unwrap();
        assert!(repository.get_user_by_id("id").await.unwrap().is_none());
        assert!(repository
            .get_user_by_email("email")
            .await
            .unwrap()
            .is_none());
        assert!(repository
            .get_user_by_token("token")
            .await
            .unwrap()
            .is_none());
    }
}
