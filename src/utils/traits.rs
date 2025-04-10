use crate::internal_server_error;
use actix_web::HttpResponse;

pub trait MapToInternalError<T> {
    fn map_to_internal_server_error(self, message: &'static str) -> Result<T, HttpResponse>;
}

impl<T, E> MapToInternalError<T> for Result<T, E> {
    fn map_to_internal_server_error(self, message: &'static str) -> Result<T, HttpResponse> {
        self.map_err(|_| internal_server_error!(message))
    }
}
