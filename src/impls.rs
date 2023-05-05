use std::{
    convert::Infallible,
    num::TryFromIntError,
    ops::{FromResidual, Try},
};

use crate::errors::{
    ChannelError, DatabaseError, Error, ErrorTy, Result, SrcDatabase, SrcRust, SrcSerde, SrcUnkown,
};
use redis::{ErrorKind, RedisError};
use serde_json::json;

pub enum ErrorPurposeRust {
    FailedToParseInt,
}
pub enum ErrorPurpose<'a> {
    Redis(ErrorKind),
    Serde(serde_json::error::Category),
    Rust(ErrorPurposeRust),
    Sqlx(&'a (dyn sqlx::error::DatabaseError + 'static)),
    CannotFind,
}
pub trait GeneralizedError {
    fn get_purpose(&self) -> ErrorPurpose;
}

impl GeneralizedError for RedisError {
    fn get_purpose(&self) -> ErrorPurpose {
        ErrorPurpose::Redis(self.kind())
    }
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

impl GeneralizedError for TryFromIntError {
    fn get_purpose(&self) -> ErrorPurpose {
        ErrorPurpose::Rust(ErrorPurposeRust::FailedToParseInt)
    }
}

impl<O, E: ChannelError> FromResidual<E> for Result<O, Error> {
    fn from_residual(residual: E) -> Self {
        Self::Err(Error {
            source: &SrcUnkown,
            ty: residual.get_type().clone(),
            msg: residual.get_msg(),
        })
    }
}

impl<O> Try for Result<O, Error> {
    type Output = O;
    type Residual = Error;

    fn from_output(output: Self::Output) -> Self {
        Self::Ok(output)
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            Result::Ok(ok) => std::ops::ControlFlow::Continue(ok),
            Result::Err(err) => std::ops::ControlFlow::Break(err),
        }
    }
}

impl<O, E> FromResidual<std::result::Result<Infallible, E>> for Result<O, Error>
where
    E: GeneralizedError,
{
    fn from_residual(residual: std::result::Result<Infallible, E>) -> Self {
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
                ErrorPurpose::Redis(_) => Self::Err(Error {
                    source: &SrcUnkown,
                    ty: ErrorTy::Unkown,
                    msg: Some(json!({"msg": "An unkown error occured (no really)."}).to_string()),
                }),
                ErrorPurpose::Rust(_) => Self::Err(Error {
                    source: &SrcRust,
                    ty: ErrorTy::Unkown,
                    msg: Some(json!({"msg": "An unkown error occured (no really)."}).to_string()),
                }),
            },
        }
    }
}
