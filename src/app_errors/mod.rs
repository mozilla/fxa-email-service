// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, you can obtain one at https://mozilla.org/MPL/2.0/.


use error::{HandlerErrorKind, HandlerResult};

// #[cfg(test)]
// mod test;

#[error(400)]
pub fn bad_request() -> HandlerResult<()> {
    Err(HandlerErrorKind::BadRequest)?
}

#[error(404)]
pub fn not_found() -> HandlerResult<()> {
    Err(HandlerErrorKind::NotFound)?
}

#[error(405)]
pub fn method_not_allowed() -> HandlerResult<()> {
    Err(HandlerErrorKind::MethodNotAllowed)?
}

#[error(422)]
pub fn unprocessable_entity() -> HandlerResult<()> {
    Err(HandlerErrorKind::UnprocessableEntity)?
}

#[error(429)]
pub fn too_many_requests() -> HandlerResult<()> {
    Err(HandlerErrorKind::TooManyRequests)?
}

#[error(500)]
pub fn internal_server_error() -> HandlerResult<()> {
    Err(HandlerErrorKind::InternalServerError)?
}
