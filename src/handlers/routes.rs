use super::{
    authorization::sign_in,
    users::{create_user, delete_user, get_user, update_user},
};
use actix_web::web::ServiceConfig;

pub fn app_routes(config: &mut ServiceConfig) {
    config
        .service(get_user)
        .service(create_user)
        .service(update_user)
        .service(delete_user)
        .service(sign_in);
}
