use rusqlite::Connection;
use std::sync::{Arc, Mutex};

pub struct State {
    pub connection: Arc<Mutex<Connection>>,
}
