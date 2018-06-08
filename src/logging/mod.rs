/// Logging via slog
///
/// Provides a MozlogLogger with moz log fields per request
 
use std::io;
use std::ops::Deref;

use failure::{err_msg};
use rocket::{
    Request, State,
};
use slog::{self, Drain, Record, Serializer, KV, Value, Key};
use slog_async;
use slog_mozlog_json::MozLogJson;
use slog_term;

use app_errors::Result;
use settings::*;

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
                .or(request.remote().map(|addr| addr.ip().to_string())),
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
    fn serialize(&self, _: &Record, _: Key, serializer: &mut Serializer) -> slog::Result {
        serializer.emit_str("auth_db_uri", &self.authdb.baseuri)?;
        serializer.emit_str("provider", &self.provider)?;

        if let Some(_) = self.aws.keys {
            serializer.emit_str("aws_keys_access", "[hidden]")?;
        } else {
            serializer.emit_str("aws_keys_access", "[not set]")?;
        }

        serializer.emit_str("aws_region", &self.aws.region)?;

        if let Some(ref sqsurls) = self.aws.sqsurls {
            serializer.emit_str("aws_sqs_urls_bounce", &sqsurls.bounce.to_owned())?;
            serializer.emit_str("aws_sqs_urls_complaint", &sqsurls.complaint.to_owned())?;
            serializer.emit_str("aws_sqs_urls_delivery", &sqsurls.delivery.to_owned())?;
            serializer.emit_str("aws_sqs_urls_notification", &sqsurls.notification.to_owned())?;
        } else {
            serializer.emit_str("aws_sqs_urls", "[not set]")?;
        }

        if let Some(ref jsonlogging) = self.jsonlogging {
            serializer.emit_str("json_logging", &format!("{}", jsonlogging))?;
        }

        if let Some(_) = self.sendgrid {
            serializer.emit_str("sendgrid_key", "[hidden]")?;
        } else {
            serializer.emit_str("sendgrid_key", "[not set]")?;
        }

        serializer.emit_str("smtp_host", &self.smtp.host)?;
        serializer.emit_str("smtp_port", &format!("{}", self.smtp.port))?;
        
        if let Some(ref user) = self.smtp.user {
            serializer.emit_str("smtp_user", user)?;
        } else {
            serializer.emit_str("smtp_user", "[not set]")?;
        }

        if let Some(_) = self.smtp.password {
            serializer.emit_str("smtp_password", "[hidden]")?;
        } else {
            serializer.emit_str("smtp_password", "[not set]")?;
        }

        serializer.emit_str("bounce_limits_enabled",  &format!("{}", &self.bouncelimits.enabled))?;
        serializer.emit_str("bounce_limits_complaint",  &format!("{:?}", &self.bouncelimits.complaint))?;
        serializer.emit_str("bounce_limits_hard",  &format!("{:?}", &self.bouncelimits.hard))?;
        serializer.emit_str("bounce_limits_soft",  &format!("{:?}", &self.bouncelimits.soft))?;

        Ok(())
    }
}

pub struct MozlogLogger(slog::Logger);

impl MozlogLogger {
    pub fn with_request(request: &Request) -> Result<MozlogLogger> {
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

pub fn init_logging(settings: &Settings) -> Result<MozlogLogger> {
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
    Ok(MozlogLogger(logger))
}
