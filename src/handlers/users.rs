use crate::{
    constants::{error::ErrorMessages, users::UserMessages},
    database::{
        adapter::lock_db_connection,
        users::{DELETE_USER, INSERT_USER, SELECT_USER, UPDATE_USER},
    },
    guards::authorization::authorization_verify,
    internal_server_error,
    models::{
        state::State,
        users::{CreateUser, UpdateUser, User},
    },
    not_found, success_body, success_json,
};
use actix_web::{
    delete, get, post, put,
    web::{Data, Json},
    HttpRequest, HttpResponse, Responder,
};
use rusqlite::{params, OptionalExtension};

#[post("/users")]
async fn create_user(data: Data<State>, user: Json<CreateUser>) -> impl Responder {
    return lock_db_connection(&data)
        .and_then(|guard| {
            return guard
                .execute(INSERT_USER, params![user.login, user.password])
                .map_err(|_| internal_server_error!(ErrorMessages::QUERY_EXECUTION));
        })
        .map_or_else(|error| error, |_| success_body!(UserMessages::CREATED));
}

#[get("/users/self")]
async fn get_user(request: HttpRequest, data: Data<State>) -> impl Responder {
    return authorization_verify(&request, &data).map_or_else(
        |error| error,
        |user_id| {
            return lock_db_connection(&data)
                .and_then(|guard| {
                    return guard
                        .prepare(SELECT_USER)
                        .map_err(|_| internal_server_error!(ErrorMessages::PREPARE_STATEMENT))
                        .and_then(|mut statement| {
                            return statement
                                .query_row(params![user_id], |row| User::map(row))
                                .optional()
                                .map_err(|_| internal_server_error!(ErrorMessages::QUERY_EXECUTION));
                        });
                })
                .map_or_else(
                    |error| error,
                    |user| user.map_or_else(|| not_found!(UserMessages::NOT_FOUND), |user| success_json!(user)),
                );
        },
    );
}

#[put("/users/self")]
async fn update_user(request: HttpRequest, data: Data<State>, user: Json<UpdateUser>) -> impl Responder {
    return authorization_verify(&request, &data).map_or_else(
        |error| error,
        |user_id| {
            return lock_db_connection(&data)
                .and_then(|guard| {
                    return guard
                        .execute(UPDATE_USER, params![user.login, user.password, user_id])
                        .map_err(|_| internal_server_error!(ErrorMessages::QUERY_EXECUTION));
                })
                .map_or_else(
                    |error| error,
                    |affected_rows| {
                        if affected_rows == 0 {
                            return not_found!(UserMessages::NOT_FOUND);
                        } else {
                            return success_body!(UserMessages::UPDATED);
                        }
                    },
                );
        },
    );
}

#[delete("/users/self")]
async fn delete_user(request: HttpRequest, data: Data<State>) -> impl Responder {
    return authorization_verify(&request, &data).map_or_else(
        |error| error,
        |user_id| {
            return lock_db_connection(&data)
                .and_then(|guard| {
                    return guard
                        .execute(DELETE_USER, params![user_id])
                        .map_err(|_| internal_server_error!(ErrorMessages::QUERY_EXECUTION));
                })
                .map_or_else(
                    |error| error,
                    |affected_rows| {
                        if affected_rows == 0 {
                            return not_found!(UserMessages::NOT_FOUND);
                        } else {
                            return success_body!(UserMessages::DELETED);
                        }
                    },
                );
        },
    );
}
