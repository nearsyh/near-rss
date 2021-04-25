pub struct UserCreds {
    pub sid: String,
    pub lsid: String,
    pub cltoken: String,
}

pub struct User {
    pub id: String,
    pub email: String,
    pub password_hash: String,
    pub token: String,
}

#[rocket::async_trait]
pub trait UserService {
    async fn login(&self, email: &str, password: &str) -> Result<UserCreds, String>;

    async fn is_token_valid(&self, token: &str) -> bool;

    async fn get_user(&self, token: &str) -> User;
}

struct FakeUsers {}

#[rocket::async_trait]
impl UserService for FakeUsers {
    async fn login(&self, email: &str, password: &str) -> Result<UserCreds, String> {
        if email == "nearsy.h@gmail.com" {
            Ok(UserCreds {
                sid: String::from("sid"),
                lsid: String::from("lsid"),
                cltoken: String::from("cltoken"),
            })
        } else {
            Err(String::from("Bad"))
        }
    }

    async fn is_token_valid(&self, token: &str) -> bool {
        return true;
    }

    async fn get_user(&self, token: &str) -> User {
        User {
            id: "id".to_string(),
            email: "nearsy.h@gmail.com".to_string(),
            password_hash: "password_hash".to_string(),
            token: token.to_string(),
        }
    }
}

pub fn new_user_service() -> Box<dyn UserService + Send + Sync> {
    Box::new(FakeUsers {})
}
