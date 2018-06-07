/// Logging via slog
///
/// Provides a RequestLogger with moz log fields per request
 
use std::io;
use std::ops::Deref;

use failure::{err_msg};
use rocket::{
    Request, State,
};
use slog::{self, Drain, Record, Serializer, KV};
use slog_async;
use slog_mozlog_json::MozLogJson;
use slog_term;

use app_errors::Result;
use settings::Settings;

lazy_static! {
    static ref LOGGER_NAME: String =
        format!("{}-{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    static ref MSG_TYPE: String = format!("{}:log", env!("CARGO_PKG_NAME"));
}

#[derive(Clone)]
struct MozLogFields {
    method: &'static str,
    path: String,
    remote: Option<String>,
    agent: Option<String>,
}

impl MozLogFields {
    pub fn from_request(request: &Request) -> MozLogFields {
        let headers = request.headers();
        MozLogFields {
            method: request.method().as_str(),
            path: request.uri().to_string(),
            agent: headers.get_one("User-Agent").map(&str::to_owned),
            remote: headers
                .get_one("X-Forwarded-For")
                .map(&str::to_owned)
                .or(request.remote().map(|addr| addr.ip().to_string())),
        }
    }
}

impl KV for MozLogFields {
    fn serialize(&self, _: &Record, serializer: &mut Serializer) -> slog::Result {
        if let Some(ref agent) = self.agent {
            serializer.emit_str("agent", agent)?;
        }
        if let Some(ref remote) = self.remote {
            serializer.emit_str("remoteAddressChain", remote)?;
        }
        serializer.emit_str("path", &self.path)?;
        serializer.emit_str("method", self.method)
    }
}

pub struct RequestLogger(slog::Logger);

impl RequestLogger {
    pub fn with_request(request: &Request) -> Result<RequestLogger> {
        let logger = request
            .guard::<State<RequestLogger>>()
            .success_or(err_msg("Internal error: No managed RequestLogger"))?;
        Ok(RequestLogger(
            logger.new(slog_o!(MozLogFields::from_request(request))),
        ))
    }
}

impl Deref for RequestLogger {
    type Target = slog::Logger;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn init_logging(settings: &Settings) -> Result<RequestLogger> {
    let json_logging = match settings.jsonlogging {
        Some(json_logging) => json_logging,
        None => Err(err_msg("Invalid FXA_EMAIL_JSONLOGGING"))?,
    };

    let logger = if json_logging {
        let drain = MozLogJson::new(io::stdout())
            .logger_name(LOGGER_NAME.to_owned())
            .msg_type(MSG_TYPE.to_owned())
            .build()
            .fuse();
        let drain = slog_async::Async::new(drain).build().fuse();
        slog::Logger::root(drain, slog_o!())
    } else {
        let decorator = slog_term::TermDecorator::new().build();
        let drain = slog_term::FullFormat::new(decorator).build().fuse();
        let drain = slog_async::Async::new(drain).build().fuse();
        slog::Logger::root(drain, slog_o!())
    };
    Ok(RequestLogger(logger))
}
