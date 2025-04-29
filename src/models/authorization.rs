use rand::{rng, Rng};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SignIn {
    pub login: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct Token {
    pub success: bool,
    pub user_id: Option<u32>,
    pub hash: Option<String>,
}

impl Token {
    pub fn create(user_id: u32, hash: String) -> Self {
        return Token {
            success: true,
            user_id: Some(user_id),
            hash: Some(hash),
        };
    }

    pub fn empty() -> Self {
        return Token {
            success: false,
            user_id: None,
            hash: None,
        };
    }

    pub fn generate_hash() -> String {
        const CHARS: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let length = 100;
        let mut hash = String::with_capacity(length);
        let mut thread = rng();
        for _ in 0..length {
            let index = thread.random_range(0..CHARS.len());
            hash.push(CHARS[index] as char);
        }
        return hash;
    }
}
