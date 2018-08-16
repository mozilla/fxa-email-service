// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, you can obtain one at https://mozilla.org/MPL/2.0/.

//! The queue-processing loop for fxa_email_service.
//!
//! Configuration is via [`settings::Settings`][settings].
//!
//! [settings]: ../fxa_email_service/settings/struct.Settings.html

extern crate futures;
extern crate fxa_email_service;
#[macro_use]
extern crate lazy_static;
extern crate env_logger;

use futures::future::{self, Future, Loop};
#[macro_use(
    slog_b,
    slog_error,
    slog_info,
    slog_kv,
    slog_log,
    slog_record,
    slog_record_static
)]
extern crate slog;
#[macro_use]
extern crate slog_scope;

use fxa_email_service::{
    app_errors::AppError,
    logging::MozlogLogger,
    queues::{QueueIds, Queues, Sqs},
    settings::Settings,
};

lazy_static! {
    static ref SETTINGS: Settings = Settings::new().expect("config error");
    static ref QUEUES: Queues = {
        let sqs_urls = match SETTINGS.aws.sqsurls {
            Some(ref urls) => urls,
            None => panic!("Missing config: aws.sqsurls.*"),
        };
        Queues::new::<Sqs>(
            QueueIds {
                bounce: sqs_urls.bounce.0.clone(),
                complaint: sqs_urls.complaint.0.clone(),
                delivery: sqs_urls.delivery.0.clone(),
                notification: sqs_urls.notification.0.clone(),
            },
            &SETTINGS,
        )
    };
}

type LoopResult = Box<Future<Item = Loop<usize, usize>, Error = AppError>>;

fn main() {
    env_logger::init();
    let logger = MozlogLogger::new(&SETTINGS).expect("MozlogLogger::init error");
    let _guard = slog_scope::set_global_logger(logger.0);
    let process_queues: &Fn(usize) -> LoopResult = &|previous_count: usize| {
        let future = QUEUES
            .process()
            .or_else(move |error: AppError| {
                let logger = MozlogLogger(slog_scope::logger());
                let log = MozlogLogger::with_app_error(&logger, &error)
                    .expect("MozlogLogger::with_app_error error");
                slog_error!(log, "{}", "Error processing queue");
                future::ok(0)
            }).and_then(move |count: usize| {
                let total_count = count + previous_count;
                info!(
                    "Succesfully processed queue message";
                    "processed_messages" => count, "total_messages" => total_count
                );
                Ok(Loop::Continue(total_count))
            });
        Box::new(future)
    };
    future::loop_fn(0, process_queues).wait().unwrap();
}
