use diesel::{
    r2d2::PoolError,
    result::{
        DatabaseErrorKind,
        Error::{self as DieselError},
    },
};
#[derive(thiserror::Error, Debug)]
pub enum DbError {
    #[error("Record was not found")]
    NotFound,
    #[error("Unprocessable Entity: {0}")]
    UnprocessableEntity(String),
    #[error("Internal Error")]
    Unknown,
}

impl From<DieselError> for DbError {
    fn from(error: DieselError) -> Self {
        match error {
            DieselError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_string();
                    return DbError::UnprocessableEntity(message);
                }
                DbError::Unknown
            }
            DieselError::NotFound => DbError::NotFound,
            _ => DbError::Unknown,
        }
    }
}

impl From<PoolError> for DbError {
    fn from(_: PoolError) -> Self {
        DbError::Unknown
    }
}
