use axum::{
    body::{self, Body},
    http::StatusCode,
    response::IntoResponse,
};

use std::fmt::{Debug, Display};

#[derive(Debug)]
pub enum DatabaseError {
    SyntaxError,
    FailedToGetRow,
}

#[derive(Debug)]
pub enum ErrorTy {
    Database(DatabaseError),
    Anyhow,
    Serde,
    Unkown,
}

impl std::error::Error for ErrorTy {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self)
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

impl Display for ErrorTy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Database(database) => match database {
                DatabaseError::SyntaxError => write!(f, "There was a syntax error"),
                DatabaseError::FailedToGetRow => write!(f, "Failed to get row"),
            },
            Self::Anyhow => write!(f, "An Anyhow error was raised"),
            Self::Serde => write!(f, "There was an error while working with Serde."),
            Self::Unkown => write!(f, "An unkown error occured (no really)."),
        }
    }
}

fn error_ty_to_status_code(_ty: &ErrorTy) -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

impl From<ErrorTy> for StatusCode {
    fn from(val: ErrorTy) -> Self {
        error_ty_to_status_code(&val)
    }
}

impl From<&ErrorTy> for StatusCode {
    fn from(val: &ErrorTy) -> Self {
        error_ty_to_status_code(val)
    }
}

#[derive(Debug)]
pub struct SrcDatabase;
impl std::fmt::Display for SrcDatabase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Database.")
    }
}
impl std::error::Error for SrcDatabase {}
#[derive(Debug)]
pub struct SrcUnkown;
impl std::fmt::Display for SrcUnkown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unkown.")
    }
}
impl std::error::Error for SrcUnkown {}
#[derive(Debug)]
pub struct SrcSerde;
impl std::fmt::Display for SrcSerde {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Serde.")
    }
}
impl std::error::Error for SrcSerde {}

pub trait ChannelError {
    fn get_type(&self) -> &ErrorTy;
    fn get_msg(&self) -> Option<String>;
}

#[derive(Debug)]
pub struct Error {
    pub source: &'static (dyn std::error::Error + 'static),
    pub ty: ErrorTy,
    pub msg: Option<String>,
}

impl ChannelError for Error {
    fn get_msg(&self) -> Option<String> {
        self.msg.clone()
    }
    fn get_type(&self) -> &ErrorTy {
        &self.ty
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.msg
                .as_ref()
                .unwrap_or(&"No message provided".to_string())
        )
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let mut res = ().into_response();
        *res.status_mut() = self.ty.into();
        if let Some(msg) = self.msg {
            let msg = Into::<String>::into(msg);
            *res.body_mut() = body::boxed(Body::from(msg));
        }
        res
    }
}

pub enum ErrorWrapper<O, E = Error>
where
    O: IntoResponse,
    E: ChannelError,
{
    Ok(O),
    Err(E),
}

impl<O, E> IntoResponse for ErrorWrapper<O, E>
where
    O: IntoResponse,
    E: ChannelError,
{
    fn into_response(self) -> axum::response::Response {
        match self {
            ErrorWrapper::Ok(res) => res.into_response(),
            ErrorWrapper::Err(err) => {
                let mut res = ().into_response();
                *res.status_mut() = err.get_type().into();
                if let Some(msg) = err.get_msg() {
                    let msg = Into::<String>::into(msg);
                    *res.body_mut() = body::boxed(Body::from(msg));
                }
                res
            }
        }
    }
}

impl<O> From<anyhow::Error> for ErrorWrapper<O, Error>
where
    O: IntoResponse,
{
    fn from(_: anyhow::Error) -> Self {
        Self::Err(Error {
            source: &SrcUnkown,
            ty: ErrorTy::Anyhow,
            msg: None,
        })
    }
}
