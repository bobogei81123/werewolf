use actix_web::{
    ResponseError,
    http::StatusCode,
};
use std::fmt;

#[derive(Debug)]
pub struct HttpError {
    status: StatusCode, 
    error: anyhow::Error,
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}) {}", self.status, self.error)
    }
}

pub trait ErrorExt: Sized {
    fn error_status(self, status: StatusCode) -> HttpError;

    fn bad_request(self) -> HttpError {
        self.error_status(StatusCode::BAD_REQUEST)
    }

    fn forbidden(self) -> HttpError {
        self.error_status(StatusCode::FORBIDDEN)
    }

    fn internal_error(self) -> HttpError {
        self.error_status(StatusCode::INTERNAL_SERVER_ERROR)
    }

    fn not_found(self) -> HttpError {
        self.error_status(StatusCode::NOT_FOUND)
    }
}

impl<T> ErrorExt for T
where T: Into<anyhow::Error> {
    fn error_status(self, status: StatusCode) -> HttpError {
        HttpError {
            status,
            error: self.into(),
        }
    }
}

impl ResponseError for HttpError {
    fn status_code(&self) -> StatusCode {
        self.status
    }
}

impl From<anyhow::Error> for HttpError {
    fn from(error: anyhow::Error) -> Self {
        error.internal_error()
    }
}

pub type HttpResult<T> = Result<T, HttpError>;

pub trait ResultExt<T>: Sized {
    fn with_error_status(self, status: StatusCode) -> HttpResult<T>;

    fn or_bad_request(self) -> HttpResult<T> {
        self.with_error_status(StatusCode::BAD_REQUEST)
    }

    fn or_forbidden(self) -> HttpResult<T> {
        self.with_error_status(StatusCode::FORBIDDEN)
    }

    fn or_internal_error(self) -> HttpResult<T> {
        self.with_error_status(StatusCode::INTERNAL_SERVER_ERROR)
    }

    fn or_not_found(self) -> HttpResult<T> {
        self.with_error_status(StatusCode::NOT_FOUND)
    }
}

impl<T, E> ResultExt<T> for Result<T, E>
where E: ErrorExt {
    fn with_error_status(self, status: StatusCode) -> HttpResult<T> {
        self.map_err(|err| err.error_status(status))
    }
}
