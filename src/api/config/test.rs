// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, you can obtain one at https://mozilla.org/MPL/2.0/.

use rocket::{
    http::{ContentType, Status},
    local::Client,
};

use db::config::Config;
use logging::MozlogLogger;
use settings::Settings;
use types::error::{AppError, AppErrorKind};

fn setup() -> Client {
    let mut settings = Settings::new().unwrap();
    settings.provider.forcedefault = false;
    let config = Config::new(&settings);
    let logger = MozlogLogger::new(&settings);
    let server = rocket::ignite()
        .manage(settings)
        .manage(config)
        .manage(logger)
        .mount("/", routes![super::handler]);

    Client::new(server).unwrap()
}

#[test]
fn request_with_all_the_things() {
    let client = setup();

    let mut response = client
        .post("/config")
        .header(ContentType::JSON)
        .body(
            r#"{
              "clobber": true,
              "config": {
                "default_provider": "sendgrid",
                "limits": {
                  "enabled": true,
                  "complaint": [
                    { "period": "1 day", "limit": 0 },
                    { "period": "1 year", "limit": 1 }
                  ],
                  "hard": [
                    { "period": "2 days", "limit": 2 },
                    { "period": "2 years", "limit": 10 }
                  ],
                  "soft": [
                    { "period": "3 minutes", "limit": 1 }
                  ]
                },
                "queue": "https://sqs.us-east-1.amazonaws.com/1234567890/blee",
                "rules": [
                  { "percentage": 50, "precedence": -127, "provider": "sendgrid" },
                  { "percentage": 100, "precedence": 0, "provider": "socketlabs", "regex": "^socketlabs@mozilla\\.com$" },
                  { "percentage": 100, "precedence": 127, "provider": "ses" }
                ],
                "sender": {
                  "name": "Firefox Accounts",
                  "address": "accounts@firefox.com"
                }
              }
            }"#,
        )
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let body = response.body().unwrap().into_string().unwrap();
    assert_eq!(body, json!({}).to_string());
}

#[test]
fn request_without_optional_fields() {
    let client = setup();

    let mut response = client
        .post("/config")
        .header(ContentType::JSON)
        .body(
            r#"{
              "config": {}
            }"#,
        )
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let body = response.body().unwrap().into_string().unwrap();
    assert_eq!(body, json!({}).to_string());
}

#[test]
fn empty_request() {
    let client = setup();

    let mut response = client
        .post("/config")
        .header(ContentType::JSON)
        .body("{}")
        .dispatch();

    assert_eq!(response.status(), Status::BadRequest);

    let body = response.body().unwrap().into_string().unwrap();
    let error: AppError =
        AppErrorKind::InvalidPayload(String::from("missing field `config` at line 1 column 2"))
            .into();
    let expected = serde_json::to_string(&error).unwrap();
    assert_eq!(body, expected);
}
