use rusqlite::{Error, Row};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct User {
    pub id: u32,
    pub login: String,
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub login: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UpdateUser {
    pub login: Option<String>,
    pub password: Option<String>,
}

impl User {
    pub fn map(row: &Row<'_>) -> Result<Self, Error> {
        let user = User {
            id: row.get("id").unwrap_or(0),
            login: row.get("login").unwrap_or(String::new()),
        };
        return Ok(user);
    }
}
