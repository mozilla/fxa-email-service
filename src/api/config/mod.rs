// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, you can obtain one at https://mozilla.org/MPL/2.0/.

//! Route handler
//! for the `POST /config` endpoint.

#[cfg(test)]
mod test;

use rocket::{
    data::{self, FromData},
    http::Status,
    Data, Request, State,
};
use rocket_contrib::Json;

use db::config::{Config as ConfigDb, Data as ConfigData};
use logging::MozlogLogger;
use types::error::{AppError, AppErrorKind, AppResult};

/// Payload for `POST /config`.
#[derive(Debug, Deserialize)]
struct Payload {
    /// Flag indicating whether this payload clobbers existing config or should be merged with it.
    clobber: Option<bool>,

    /// The configuration data.
    config: ConfigData,
}

impl FromData for Payload {
    type Error = AppError;

    fn from_data(request: &Request, data: Data) -> data::Outcome<Self, Self::Error> {
        Json::<Payload>::from_data(request, data)
            .map_failure(|(_status, error)| {
                (
                    Status::BadRequest,
                    AppErrorKind::InvalidPayload(error.to_string()).into(),
                )
            })
            .map(|json| json.into_inner())
    }
}

#[post("/config", format = "application/json", data = "<payload>")]
fn handler(
    payload: AppResult<Payload>,
    config_db: State<ConfigDb>,
    // TODO: do we need the logger?
    _logger: State<MozlogLogger>,
) -> AppResult<Json> {
    let mut payload = payload?;
    if payload.clobber == Some(true) {
        config_db.set("fxa", &payload.config)?;
    } else {
        config_db.merge("fxa", &mut payload.config)?;
    };
    Ok(Json(json!({})))
}
