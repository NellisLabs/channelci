use std::{convert::Infallible, ops::FromResidual};

use axum::response::IntoResponse;
use serde_json::json;

use crate::errors::{
    DatabaseError, Error, ErrorTy, ErrorWrapper, SrcDatabase, SrcSerde, SrcUnkown,
};

pub enum ErrorPurpose<'a> {
    Serde(serde_json::error::Category),
    Sqlx(&'a (dyn sqlx::error::DatabaseError + 'static)),
    CannotFind,
}
pub trait GeneralizedError {
    fn get_purpose(&self) -> ErrorPurpose;
}

impl GeneralizedError for sqlx::Error {
    fn get_purpose(&self) -> ErrorPurpose {
        let Some(db_err) = self.as_database_error() else {
            return ErrorPurpose::CannotFind
        };
        ErrorPurpose::Sqlx(db_err)
    }
}

impl GeneralizedError for serde_json::error::Error {
    fn get_purpose(&self) -> ErrorPurpose {
        ErrorPurpose::Serde(self.classify())
    }
}

impl<O, E> FromResidual<Result<Infallible, E>> for ErrorWrapper<O, Error>
where
    O: IntoResponse,
    E: GeneralizedError,
{
    fn from_residual(residual: Result<Infallible, E>) -> Self {
        match residual {
            Ok(_) => unreachable!(),
            Err(_err) => match _err.get_purpose() {
                ErrorPurpose::Serde(error) => Self::Err(Error {
                    source: &SrcSerde,
                    ty: ErrorTy::Serde,
                    msg: Some(
                        json!({
                            "msg": format!("There was an {error:?} error while using Serde.")
                        })
                        .to_string(),
                    ),
                }),
                /*
                    /// The (SQLSTATE) code for the error.
                    fn code(&self) -> Option<Cow<'_, str>> {
                        None
                    }

                    look up error codes and write the proper DatabaseError enum fields for them.
                */
                ErrorPurpose::Sqlx(error) => Self::Err(Error {
                    source: &SrcDatabase,
                    ty: ErrorTy::Database(DatabaseError::SyntaxError),
                    msg: Some(
                        json!({
                            "msg": error.message()
                        })
                        .to_string(),
                    ),
                }),
                ErrorPurpose::CannotFind => Self::Err(Error {
                    source: &SrcUnkown,
                    ty: ErrorTy::Unkown,
                    msg: Some(json!({"msg": "An unkown error occured (no really)."}).to_string()),
                }),
            },
        }
    }
}
