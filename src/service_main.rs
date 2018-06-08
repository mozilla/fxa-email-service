// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, you can obtain one at https://mozilla.org/MPL/2.0/.

#![feature(assoc_unix_epoch)]
#![feature(plugin)]
#![feature(try_from)]
#![feature(type_ascription)]
#![plugin(rocket_codegen)]

extern crate config;
extern crate hex;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate reqwest;
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate rusoto_core;
extern crate rusoto_credential;
extern crate rusoto_ses;
extern crate sendgrid;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate failure;
#[macro_use(
   slog_o, slog_info, slog_kv, slog_log, slog_record, slog_b, slog_record_static, slog_error
)]
extern crate slog;
extern crate slog_async;
extern crate slog_mozlog_json;
extern crate slog_term;
extern crate mozsvc_common;

mod app_errors;
mod auth_db;
mod bounces;
mod deserialize;
mod duration;
mod logging;
mod providers;
mod send;
mod settings;
mod validate;

fn main() {
    let logger = logging::init_logging(&settings::Settings::new().unwrap()).unwrap(); 
    rocket::ignite()
        .manage(logger)
        .mount("/", routes![send::handler])
        .catch(errors![
            app_errors::bad_request,
            app_errors::not_found,
            app_errors::method_not_allowed,
            app_errors::unprocessable_entity,
            app_errors::too_many_requests,
            app_errors::internal_server_error
        ])
        .attach(rocket::fairing::AdHoc::on_request(|request, _| {
            let log = logging::MozlogLogger::with_request(request).unwrap();
            slog_info!(log, "{}", "Request started.");
        }))
        .attach(rocket::fairing::AdHoc::on_response(|request, response| {
            let log = logging::MozlogLogger::with_request(request).unwrap();
            if response.status().code == 200 {
                slog_info!(log, "{}", "Request finished succesfully."; 
                    "status_code" => response.status().code, "status_msg" => response.status().reason);
            } else {
                slog_error!(log, "{}", "Request errored."; 
                    "status_code" => response.status().code, "status_msg" => response.status().reason);
            }
        }))
        .launch();
}
