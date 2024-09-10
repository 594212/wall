use std::borrow::BorrowMut;

use actix_web::body::MessageBody;
use actix_web::error::ResponseError;
use actix_web::http::StatusCode;
use actix_web::HttpMessage;
use actix_web::HttpResponse;
use actix_web::HttpResponseBuilder;
use serde_json::Map as JsonMap;
use serde_json::Value as JsonValue;
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
