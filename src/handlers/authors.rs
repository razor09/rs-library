use crate::{
    constants::error::ErrorMessages,
    database::adapter::lock_db_connection,
    guards::authorization::authorization_verify,
    internal_server_error,
    models::{state::State, users::User},
    success_json,
};
use actix_web::{get, web::Data, HttpRequest, HttpResponse, Responder};

#[get("/authors")]
async fn get_authors(request: HttpRequest, data: Data<State>) -> impl Responder {
    return authorization_verify(&request, &data).map_or_else(
        |error| error,
        |_| {
            return lock_db_connection(&data)
                .and_then(|guard| {
                    return guard
                        .prepare("")
                        .map_err(|_| internal_server_error!(ErrorMessages::PREPARE_STATEMENT))
                        .and_then(|mut statement| {
                            return statement
                                .query_map([], |row| User::map(row))
                                .and_then(|rows| rows.collect::<Result<Vec<_>, _>>())
                                .map_err(|_| internal_server_error!(ErrorMessages::QUERY_EXECUTION));
                        });
                })
                .map_or_else(|error| error, |users| success_json!(users));
        },
    );
}
