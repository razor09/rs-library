use crate::{
    constants::error::ErrorMessages,
    database::{adapter::lock_db_connection, tokens::INSERT_TOKEN, users::SELECT_USER_ID},
    internal_server_error,
    models::{
        authorization::{SignIn, Token},
        state::State,
    },
    success_json,
};
use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use rusqlite::{params, OptionalExtension};

#[post("/sign-in")]
async fn sign_in(data: Data<State>, payload: Json<SignIn>) -> impl Responder {
    return lock_db_connection(&data)
        .and_then(|guard| {
            return guard
                .prepare(SELECT_USER_ID)
                .map_err(|_| internal_server_error!(ErrorMessages::PREPARE_STATEMENT))
                .and_then(|mut statement| {
                    return statement
                        .query_row(params![payload.login, payload.password], |row| row.get::<_, u32>(0))
                        .optional()
                        .map_err(|_| internal_server_error!(ErrorMessages::QUERY_EXECUTION));
                });
        })
        .map_or_else(
            |error| error,
            |user_id| {
                return user_id.map_or_else(
                    || {
                        let token = Token::empty();
                        return success_json!(token);
                    },
                    |user_id| {
                        let hash = Token::generate_hash();
                        return lock_db_connection(&data)
                            .and_then(|guard| {
                                return guard
                                    .execute(INSERT_TOKEN, params![user_id, hash])
                                    .map_err(|_| internal_server_error!(ErrorMessages::QUERY_EXECUTION));
                            })
                            .map_or_else(
                                |error| error,
                                |_| {
                                    let token = Token::create(user_id, hash);
                                    return success_json!(token);
                                },
                            );
                    },
                );
            },
        );
}
