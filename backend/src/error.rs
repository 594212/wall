use actix_web::{HttpResponse, ResponseError};
use diesel::result::{
    DatabaseErrorKind,
    Error::{self as DieselError},
};
use diesel_async::pooled_connection::deadpool::PoolError;
use serde_json::{json, Value as JsonValue};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unauthorized: {0}")]
    Unaftorized(JsonValue),
    #[error("Forbidden: {0}")]
    Forbidden(JsonValue),
    #[error("Not Found: {0}")]
    NotFound(JsonValue),
    #[error("Unprocessoble Entity: {0}")]
    UnprocessobalEntity(JsonValue),
    #[error("Internal Server Error")]
    InternalServerError,
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match *self {
            Error::Unaftorized(ref message) => HttpResponse::Unauthorized().json(message),
            Error::Forbidden(ref message) => HttpResponse::Forbidden().json(message),
            Error::NotFound(ref message) => HttpResponse::NotFound().json(message),
            Error::UnprocessobalEntity(ref message) => {
                HttpResponse::UnprocessableEntity().json(message)
            }
            Error::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error")
            }
        }
    }
}

impl From<DieselError> for Error {
    fn from(error: DieselError) -> Self {
        match error {
            DieselError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_string();
                    return Error::UnprocessobalEntity(json!({"error" : message}));
                }
                Error::InternalServerError
            }
            DieselError::NotFound => {
                Error::NotFound(json!({"error" : "request record was not found"}))
            }
            _ => Error::InternalServerError,
        }
    }
}

impl From<PoolError> for Error {
    fn from(_: PoolError) -> Self {
        Error::InternalServerError
    }
}
