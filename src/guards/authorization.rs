use crate::{
    constants::{
        authorization::{AuthorizationMessages, AuthorizationValidation},
        error::ErrorMessages,
    },
    database::{adapter::lock_db_connection, tokens::SELECT_TOKEN},
    internal_server_error,
    models::state::State,
    unauthorized,
};
use actix_web::{web::Data, HttpRequest, HttpResponse};
use regex::Regex;
use rusqlite::{params, OptionalExtension};

fn extract_credentials(request: &HttpRequest) -> Option<(u32, &str)> {
    return request
        .headers()
        .get("authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|token| {
            let pattern = Regex::new(AuthorizationValidation::TOKEN_PATTERN).expect(ErrorMessages::CREATE_REGEXP);
            if pattern.is_match(token) {
                let parts: Vec<&str> = token.split(':').collect();
                if let Ok(unsigned) = parts[0].parse::<u32>() {
                    let parts = (unsigned, parts[1]);
                    return Some(parts);
                }
            }
            return None;
        });
}

pub fn authorization_verify(request: &HttpRequest, data: &Data<State>) -> Result<u32, HttpResponse> {
    let (user_id, hash) = extract_credentials(request).ok_or_else(|| unauthorized!(AuthorizationMessages::UNATHORIZED))?;
    return lock_db_connection(&data)?
        .prepare(SELECT_TOKEN)
        .map_err(|_| internal_server_error!(ErrorMessages::QUERY_EXECUTION))?
        .query_row(params![user_id, hash], |row| row.get::<_, u32>(0))
        .optional()
        .map_err(|_| internal_server_error!(ErrorMessages::QUERY_EXECUTION))?
        .map(|_| user_id)
        .ok_or_else(|| unauthorized!(AuthorizationMessages::UNATHORIZED));
}
