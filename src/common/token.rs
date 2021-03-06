use crate::common::current_time_s;

pub struct Token {
    pub id: String,
    pub expire_at: i64,
    pub sid: String,
}

impl Token {
    pub fn new(id: &str) -> Token {
        Token {
            id: id.to_string(),
            expire_at: current_time_s() + 14 * 24 * 60 * 60,
            sid: super::new_id(20),
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}.{}.{}", self.sid, self.expire_at, self.id)
    }

    pub fn parse(token: &str) -> Option<Token> {
        let parts: Vec<_> = token.split('.').collect();
        if parts.len() != 3 {
            return None;
        }
        match parts[1].parse::<i64>() {
            Err(_) => None,
            Ok(expire_at) => Some(Token {
                id: parts[2].to_string(),
                expire_at: expire_at,
                sid: parts[0].to_string(),
            }),
        }
    }

    pub fn is_valid(token: &str) -> bool {
        Token::parse(token).is_some()
    }
}
