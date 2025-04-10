#[macro_export]
macro_rules! internal_server_error {
    ($message:expr) => {
        HttpResponse::InternalServerError().body($message)
    };
}

#[macro_export]
macro_rules! not_found {
    ($message:expr) => {
        HttpResponse::NotFound().body($message)
    };
}

#[macro_export]
macro_rules! unauthorized {
    ($message:expr) => {
        HttpResponse::Unauthorized().body($message)
    };
}

#[macro_export]
macro_rules! success_body {
    ($message:expr) => {
        HttpResponse::Ok().body($message)
    };
}

#[macro_export]
macro_rules! success_json {
    ($data:expr) => {
        HttpResponse::Ok().json($data)
    };
}
