use actix_web::{main, web::Data, App, HttpServer};
use rs_library::{
    constants::error::ErrorMessages,
    database::adapter::set_connection,
    handlers::routes::app_routes,
    models::{config::Config, state::State},
};
use std::io::Result;

#[main]
async fn main() -> Result<()> {
    let config = Config::load();
    let connection = set_connection();
    let state = State { connection };
    let data = Data::new(state);
    let app = move || App::new().app_data(data.clone()).configure(app_routes);
    let address = (config.host, config.port);
    let server = HttpServer::new(app).bind(address).expect(ErrorMessages::BIND_SERVER).run();
    return server.await;
}
