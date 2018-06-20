// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, you can obtain one at https://mozilla.org/MPL/2.0/.

use std::fmt;
use std::result;

use failure::{Backtrace, Context, Fail};
use rocket::http::{Status};
use rocket::response::{Responder, Response};
use rocket::{self, response, Request};
use rocket_contrib::Json;

use logging::MozlogLogger;

pub type HandlerResult<T> = result::Result<T, HandlerError>;

#[derive(Debug)]
pub struct HandlerError {
    inner: Context<HandlerErrorKind>,
}

#[derive(Clone, Eq, PartialEq, Debug, Fail)]
pub enum HandlerErrorKind {
    /// 400 Bad Request
    #[fail(display = "Bad Request")]
    BadRequest,

    /// 404 Not Found
    #[fail(display = "Not Found")]
    NotFound,

    /// 405 Method Not Allowed
    #[fail(display = "Method Not Allowed")]
    MethodNotAllowed,

    /// 422 Unprocessable Entity
    #[fail(display = "Unprocessable Entity")]
    UnprocessableEntity,

    /// 429 Too Many Requests
    #[fail(display = "Too Many Requests")]
    TooManyRequests,

    /// 500 Internal Server Error
    #[fail(display = "Internal Server Error")]
    InternalServerError,

    #[fail(display = "Invalid Email Address")]
    InvalidEmailAddress,

    #[fail(display = "Unexpected fxa-email-service error")]
    InternalError,

    #[fail(display = "Unexpected rocket error: {:?}", _0)]
    RocketError(rocket::Error), // rocket::Error isn't a std Error (so no #[cause])
}

impl HandlerErrorKind {
    /// Return a rocket response Status to be rendered for an error
    pub fn http_status(&self) -> Status {
        match *self {
            HandlerErrorKind::BadRequest => Status::BadRequest,
            HandlerErrorKind::NotFound => Status::NotFound,
            HandlerErrorKind::MethodNotAllowed => Status::MethodNotAllowed,
            HandlerErrorKind::UnprocessableEntity => Status::UnprocessableEntity,
            HandlerErrorKind::TooManyRequests => Status::TooManyRequests,
            HandlerErrorKind::InternalServerError => Status::InternalServerError,
            HandlerErrorKind::InvalidEmailAddress => Status::BadRequest,
            _ => Status::BadRequest,
        }
    }
}

impl HandlerError {
    pub fn kind(&self) -> &HandlerErrorKind {
        self.inner.get_context()
    }
}

impl Fail for HandlerError {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl fmt::Display for HandlerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.inner, f)
    }
}

impl From<HandlerErrorKind> for HandlerError {
    fn from(kind: HandlerErrorKind) -> HandlerError {
        Context::new(kind).into()
    }
}

impl From<Context<HandlerErrorKind>> for HandlerError {
    fn from(inner: Context<HandlerErrorKind>) -> HandlerError {
        HandlerError { inner: inner }
    }
}

/// Generate HTTP error responses for HandlerErrors
impl<'r> Responder<'r> for HandlerError {
    fn respond_to(self, request: &Request) -> response::Result<'r> {
        let status = self.kind().http_status();
        let log = MozlogLogger::with_request(request).map_err(|_| Status::InternalServerError)?;
        let kind = self.kind();
        slog_error!(log, "{}", &self; "status_code" => status.code,  "status_msg" => status.reason);

        let json = Json(json!({
            "status": status.code,
            "error": format!("{}", self)
        }));

        let mut builder = Response::build_from(json.respond_to(request)?);
        builder.status(status).ok()
    }
}