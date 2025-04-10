use crate::{constants::error::ErrorMessages, internal_server_error, models::state::State};
use actix_web::{web::Data, HttpResponse};
use rusqlite::Connection;
use std::{
    fs::read_to_string,
    sync::{Arc, Mutex, MutexGuard},
};

pub fn lock_db_connection(data: &Data<State>) -> Result<MutexGuard<'_, Connection>, HttpResponse> {
    return data
        .connection
        .lock()
        .map_err(|_| internal_server_error!(ErrorMessages::LOCK_DB_CONNECTION));
}

pub fn set_connection() -> Arc<Mutex<Connection>> {
    let connection = Connection::open("assets/main.db").expect(ErrorMessages::OPEN_DB_CONNECTION);
    db_initialize(&connection);
    let mutex = Mutex::new(connection);
    return Arc::new(mutex);
}

fn db_initialize(connection: &Connection) {
    read_to_string("assets/schema.sql")
        .expect(ErrorMessages::READ_SQL)
        .split(';')
        .map(|query| query.trim())
        .filter(|query| !query.is_empty())
        .for_each(|query| {
            connection.execute(query, []).expect(ErrorMessages::INITIALIZE_DB);
        });
}
