pub struct UserCreds {
  pub sid: String,
  pub lsid: String,
  pub cltoken: String,
}

pub trait Users {
  fn login(&self, email: &str, password: &str) -> Result<UserCreds, String>;

  fn is_token_valid(&self, token: &str) -> bool;
}

struct FakeUsers {}

impl Users for FakeUsers {
  fn login(&self, email: &str, password: &str) -> Result<UserCreds, String> {
    if email == "1" {
      Ok(UserCreds {
        sid: String::from("sid"),
        lsid: String::from("lsid"),
        cltoken: String::from("cltoken"),
      })
    } else {
      Err(String::from("Bad"))
    }
  }

  fn is_token_valid(&self, token: &str) -> bool {
    return true;
  }
}

pub fn new_user_service() -> Box<dyn Users> {
  Box::new(FakeUsers {})
}
