// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, you can obtain one at https://mozilla.org/MPL/2.0/.

use std::time::SystemTime;

use regex::Regex;

use super::*;
use db::core::test::TestFixture;
use settings::SenderName;

#[test]
fn set() {
    let settings = Settings::new().unwrap();
    let config = Config::new(&settings);
    let key = create_key("set");
    let test = TestFixture::setup(&settings, &key, DataType::Configuration);
    let expected = Data {
        default_provider: Some(Provider::SocketLabs),
        limits: Some(DeliveryProblemLimits {
            enabled: true,
            complaint: vec![],
            hard: vec![],
            soft: vec![],
        }),
        queue: Some(SqsUrl(
            "https://sqs.us-east-1.amazonaws.com/1234567890/wibble".to_owned(),
        )),
        rules: Some(vec![ProviderRules {
            percentage: Some(50),
            precedence: Some(-127),
            provider: Provider::Sendgrid,
            regex: Some(SerializableRegex(Regex::new("wibble").unwrap())),
        }]),
        sender: Some(Sender {
            name: SenderName("Firefox Accounts".to_owned()),
            address: "accounts@firefox.com".parse().unwrap(),
        }),
    };

    if let Err(error) = config.set(&key, &expected) {
        assert!(false, format!("{:?}", error));
    } else {
        test.assert_data(&expected);
    }

    let data = config.get(&key).unwrap();
    assert_eq!(data, Some(expected));
}

#[test]
fn merge() {
    // TODO
}

fn create_key(test: &str) -> String {
    format!("fxa-email-service.test.config.{}.{}", test, now())
}

fn now() -> u64 {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("system time error");
    now.as_secs() * 1000 + u64::from(now.subsec_millis())
}
