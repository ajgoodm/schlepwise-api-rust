use std::env;

use diesel::result::Error;
use rocket::http::Status;

pub fn host() -> String {
    env::var("SCHLEPWISE_API_HOST").expect("SCHLEPWISE_API_HOST must be set")
}

pub fn port() -> String {
    env::var("SCHLEPWISE_API_PORT").expect("SCHLEPWISE_API_PORT must be set")
}

pub fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError,
    }
}
