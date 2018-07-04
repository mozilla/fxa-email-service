// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, you can obtain one at https://mozilla.org/MPL/2.0/.

//! Mozlog-compatible logging.

use std::{io, ops::Deref};

use failure::{err_msg, Error};
use rocket::{Request, State};
use serde_json;
use slog::{self, Discard, Drain, Record, Serializer, Value, KV};
use slog_async;
use slog_mozlog_json::MozLogJson;
use slog_term;

use settings::Settings;

lazy_static! {
    static ref LOGGER_NAME: String =
        format!("{}-{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    static ref MSG_TYPE: String = format!("{}:log", env!("CARGO_PKG_NAME"));
}

#[derive(Clone)]
struct RequestMozlogFields {
    method: &'static str,
    path: String,
    remote: Option<String>,
    agent: Option<String>,
}

impl RequestMozlogFields {
    pub fn from_request(request: &Request) -> RequestMozlogFields {
        let headers = request.headers();
        RequestMozlogFields {
            method: request.method().as_str(),
            path: request.uri().to_string(),
            agent: headers.get_one("User-Agent").map(&str::to_owned),
            remote: headers
                .get_one("X-Forwarded-For")
                .map(&str::to_owned)
                .or_else(|| request.remote().map(|addr| addr.ip().to_string())),
        }
    }
}

impl KV for RequestMozlogFields {
    fn serialize(&self, _: &Record, serializer: &mut Serializer) -> slog::Result {
        if let Some(ref agent) = self.agent {
            serializer.emit_str("agent", agent)?;
        }
        if let Some(ref remote) = self.remote {
            serializer.emit_str("remoteAddressChain", remote)?;
        }
        serializer.emit_str("path", &self.path)?;
        serializer.emit_str("method", self.method)?;
        Ok(())
    }
}

impl Value for Settings {
    fn serialize(&self, _: &Record, _: &'static str, serializer: &mut Serializer) -> slog::Result {
        let settings_json = serde_json::to_string(&self).unwrap();
        serializer.emit_str("settings", &settings_json)?;
        Ok(())
    }
}

/// Mozlog-compatible logger type.
pub struct MozlogLogger(slog::Logger);

impl MozlogLogger {
    /// Construct a logger.
    pub fn new(settings: &Settings) -> Result<MozlogLogger, Error> {
        let logger = match &*settings.logging {
            "mozlog" => {
                let drain = MozLogJson::new(io::stdout())
                    .logger_name(LOGGER_NAME.to_owned())
                    .msg_type(MSG_TYPE.to_owned())
                    .build()
                    .fuse();
                let drain = slog_async::Async::new(drain).build().fuse();
                Ok(slog::Logger::root(drain, slog_o!()))
            }
            "pretty" => {
                let decorator = slog_term::TermDecorator::new().build();
                let drain = slog_term::FullFormat::new(decorator).build().fuse();
                let drain = slog_async::Async::new(drain).build().fuse();
                Ok(slog::Logger::root(drain, slog_o!()))
            }
            "null" => Ok(slog::Logger::root(Discard, slog_o!())),
            _0 => Err(err_msg(format!("Unknown logger format: {}", _0))),
        };

        Ok(MozlogLogger(logger?))
    }

    /// Log a rocket request.
    pub fn with_request(request: &Request) -> Result<MozlogLogger, Error> {
        let logger = request
            .guard::<State<MozlogLogger>>()
            .success_or(err_msg("Internal error: No managed MozlogLogger"))?;
        Ok(MozlogLogger(
            logger.new(slog_o!(RequestMozlogFields::from_request(request))),
        ))
    }
}

impl Deref for MozlogLogger {
    type Target = slog::Logger;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
