// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, you can obtain one at https://mozilla.org/MPL/2.0/.

use std::{thread::sleep, time::Duration};

use rocket::http::Status;
use serde_json::{self, Value as Json};

use super::*;
use app_errors::{AppErrorKind, AppResult};
use auth_db::{Db, DbClient};
use db::test::TestFixture;
use queues::notification::{BounceSubtype, BounceType, ComplaintFeedbackType};
use settings::{Host, Settings};

const SECOND: u64 = 1000;
const MINUTE: u64 = SECOND * 60;
const HOUR: u64 = MINUTE * 60;
const DAY: u64 = HOUR * 24;
const WEEK: u64 = DAY * 7;
const MONTH: u64 = DAY * 30;

#[test]
fn check_no_bounces() {
    let bounce_settings: Json = serde_json::from_str(
        r#"{
        "enabled": true,
        "soft": [
        { "period": "day", "limit": 0 }
        ],
        "hard": [
        { "period": "week", "limit": 0 }
        ],
        "complaint": [
        { "period": "month", "limit": 0 }
        ]
        }"#,
    ).expect("Unexpected json parsing error");
    let settings = create_settings(bounce_settings);
    let db = DbMockNoBounce;
    let problems = DeliveryProblems::new(&settings, db);
    if let Err(error) = problems.check(&"foo@example.com".parse().unwrap()) {
        assert!(false, format!("{}", error));
    }
}

fn create_settings(bounce_limits: Json) -> Settings {
    let mut settings = Settings::default();
    settings.bouncelimits = serde_json::from_value(bounce_limits).expect("JSON error");
    settings.redis.host = Host(String::from("127.0.0.1"));
    settings.redis.port = 6379;
    settings
}

#[derive(Debug)]
pub struct DbMockNoBounce;

impl Db for DbMockNoBounce {
    fn get_bounces(&self, _address: &EmailAddress) -> AppResult<Vec<DeliveryProblem>> {
        let now = now_as_milliseconds();
        Ok(vec![
            DeliveryProblem {
                address: "foo@example.com".parse().unwrap(),
                problem_type: ProblemType::SoftBounce,
                problem_subtype: ProblemSubtype::Undetermined,
                created_at: now - DAY - 1000,
            },
            DeliveryProblem {
                address: "foo@example.com".parse().unwrap(),
                problem_type: ProblemType::HardBounce,
                problem_subtype: ProblemSubtype::Undetermined,
                created_at: now - WEEK - 1000,
            },
            DeliveryProblem {
                address: "foo@example.com".parse().unwrap(),
                problem_type: ProblemType::Complaint,
                problem_subtype: ProblemSubtype::Undetermined,
                created_at: now - MONTH - 1000,
            },
        ])
    }
}

fn now_as_milliseconds() -> u64 {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("system time error");
    now.as_secs() * 1000 + u64::from(now.subsec_millis())
}

#[test]
fn check_soft_bounce() {
    let bounce_settings: Json = serde_json::from_str(
        r#"{
        "enabled": true,
        "soft": [
        { "period": "day", "limit": 0 }
        ],
        "hard": [],
        "complaint": []
        }"#,
    ).expect("Unexpected json parsing error");
    let settings = create_settings(bounce_settings);
    let db = DbMockBounceSoft;
    let problems = DeliveryProblems::new(&settings, db);
    match problems.check(&"foo@example.com".parse().unwrap()) {
        Ok(_) => assert!(false, "DeliveryProblems::check should have failed"),
        Err(error) => {
            assert_eq!(format!("{}", error), "Email account soft bounced");
            let err_data = error.kind().additional_fields();
            let address = err_data.get("address");
            if let Some(ref address) = address {
                assert_eq!("foo@example.com", address.as_str().unwrap());
            } else {
                assert!(false, "Error::address should be set");
            }
            let problem = err_data.get("problem");
            assert!(problem.is_some());
            let record: Json = serde_json::from_str(problem.unwrap().as_str().unwrap()).unwrap();
            assert_eq!(record["bounceType"], 2);
            assert_eq!(&record["createdAt"], err_data.get("time").unwrap());
            assert_eq!(error.kind().http_status(), Status::TooManyRequests);
        }
    }
}

#[derive(Debug)]
pub struct DbMockBounceSoft;

impl Db for DbMockBounceSoft {
    fn get_bounces(&self, _address: &EmailAddress) -> AppResult<Vec<DeliveryProblem>> {
        let now = now_as_milliseconds();
        Ok(vec![DeliveryProblem {
            address: "foo@example.com".parse().unwrap(),
            problem_type: ProblemType::SoftBounce,
            problem_subtype: ProblemSubtype::Undetermined,
            created_at: now - DAY + SECOND * 2,
        }])
    }
}

#[test]
fn check_hard_bounce() {
    let bounce_settings: Json = serde_json::from_str(
        r#"{
        "enabled": true,
        "soft": [],
        "hard": [
        { "period": "week", "limit": 0 }
        ],
        "complaint": []
        }"#,
    ).expect("Unexpected json parsing error");
    let settings = create_settings(bounce_settings);
    let db = DbMockBounceHard;
    let problems = DeliveryProblems::new(&settings, db);
    match problems.check(&"bar@example.com".parse().unwrap()) {
        Ok(_) => assert!(false, "DeliveryProblems::check should have failed"),
        Err(error) => {
            assert_eq!(format!("{}", error), "Email account hard bounced");
            let err_data = error.kind().additional_fields();
            let address = err_data.get("address");
            if let Some(ref address) = address {
                assert_eq!("bar@example.com", address.as_str().unwrap());
            } else {
                assert!(false, "Error::address should be set");
            }
            let problem = err_data.get("problem");
            assert!(problem.is_some());
            let record: Json = serde_json::from_str(problem.unwrap().as_str().unwrap()).unwrap();
            assert_eq!(record["bounceType"], 1);
            assert_eq!(&record["createdAt"], err_data.get("time").unwrap());
            assert_eq!(error.kind().http_status(), Status::TooManyRequests);
        }
    }
}

#[derive(Debug)]
pub struct DbMockBounceHard;

impl Db for DbMockBounceHard {
    fn get_bounces(&self, _address: &EmailAddress) -> AppResult<Vec<DeliveryProblem>> {
        let now = now_as_milliseconds();
        Ok(vec![DeliveryProblem {
            address: "bar@example.com".parse().unwrap(),
            problem_type: ProblemType::HardBounce,
            problem_subtype: ProblemSubtype::Undetermined,
            created_at: now - WEEK + SECOND * 2,
        }])
    }
}

#[test]
fn check_complaint() {
    let bounce_settings: Json = serde_json::from_str(
        r#"{
        "enabled": true,
        "soft": [],
        "hard": [],
        "complaint": [
        { "period": "month", "limit": 0 }
        ]
        }"#,
    ).expect("Unexpected json parsing error");
    let settings = create_settings(bounce_settings);
    let db = DbMockComplaint;
    let problems = DeliveryProblems::new(&settings, db);
    match problems.check(&"baz@example.com".parse().unwrap()) {
        Ok(_) => assert!(false, "DeliveryProblems::check should have failed"),
        Err(error) => {
            assert_eq!(format!("{}", error), "Email account sent complaint");
            let err_data = error.kind().additional_fields();
            let address = err_data.get("address");
            if let Some(ref address) = address {
                assert_eq!("baz@example.com", address.as_str().unwrap());
            } else {
                assert!(false, "Error::address should be set");
            }
            let problem = err_data.get("problem");
            assert!(problem.is_some());
            let record: Json = serde_json::from_str(problem.unwrap().as_str().unwrap()).unwrap();
            assert_eq!(record["bounceType"], 3);
            assert_eq!(&record["createdAt"], err_data.get("time").unwrap());
            assert_eq!(error.kind().http_status(), Status::TooManyRequests);
        }
    }
}

#[derive(Debug)]
pub struct DbMockComplaint;

impl Db for DbMockComplaint {
    fn get_bounces(&self, _address: &EmailAddress) -> AppResult<Vec<DeliveryProblem>> {
        let now = now_as_milliseconds();
        Ok(vec![DeliveryProblem {
            address: "baz@example.com".parse().unwrap(),
            problem_type: ProblemType::Complaint,
            problem_subtype: ProblemSubtype::Undetermined,
            created_at: now - MONTH + SECOND * 2,
        }])
    }
}

#[test]
fn check_db_error() {
    let bounce_settings: Json = serde_json::from_str(
        r#"{
        "enabled": true,
        "soft": [
        { "period": "day", "limit": 0 }
        ],
        "hard": [
        { "period": "week", "limit": 0 }
        ],
        "complaint": [
        { "period": "month", "limit": 0 }
        ]
        }"#,
    ).expect("Unexpected json parsing error");
    let settings = create_settings(bounce_settings);
    let db = DbMockError;
    let problems = DeliveryProblems::new(&settings, db);
    match problems.check(&"foo@example.com".parse().unwrap()) {
        Ok(_) => assert!(false, "DeliveryProblems::check should have failed"),
        Err(error) => {
            assert_eq!(format!("{}", error), "wibble blee");
            assert_eq!(error.kind().http_status(), Status::InternalServerError);
        }
    }
}

#[derive(Debug)]
pub struct DbMockError;

impl Db for DbMockError {
    fn get_bounces(&self, _address: &EmailAddress) -> AppResult<Vec<DeliveryProblem>> {
        Err(AppErrorKind::AuthDbError(String::from("wibble blee")).into())
    }
}

#[test]
fn check_no_bounces_with_nonzero_limits() {
    let bounce_settings: Json = serde_json::from_str(
        r#"{
        "enabled": true,
        "soft": [
        { "period": "day", "limit": 2 }
        ],
        "hard": [
        { "period": "week", "limit": 2 }
        ],
        "complaint": [
        { "period": "month", "limit": 2 }
        ]
        }"#,
    ).expect("Unexpected json parsing error");
    let settings = create_settings(bounce_settings);
    let db = DbMockNoBounceWithNonZeroLimits;
    let problems = DeliveryProblems::new(&settings, db);
    if let Err(error) = problems.check(&"foo@example.com".parse().unwrap()) {
        assert!(false, format!("{}", error));
    }
}

#[derive(Debug)]
pub struct DbMockNoBounceWithNonZeroLimits;

impl Db for DbMockNoBounceWithNonZeroLimits {
    fn get_bounces(&self, _address: &EmailAddress) -> AppResult<Vec<DeliveryProblem>> {
        let now = now_as_milliseconds();
        Ok(vec![
            DeliveryProblem {
                address: "foo@example.com".parse().unwrap(),
                problem_type: ProblemType::SoftBounce,
                problem_subtype: ProblemSubtype::Undetermined,
                created_at: now - DAY + MINUTE,
            },
            DeliveryProblem {
                address: "foo@example.com".parse().unwrap(),
                problem_type: ProblemType::HardBounce,
                problem_subtype: ProblemSubtype::Undetermined,
                created_at: now - WEEK + MINUTE,
            },
            DeliveryProblem {
                address: "foo@example.com".parse().unwrap(),
                problem_type: ProblemType::Complaint,
                problem_subtype: ProblemSubtype::Undetermined,
                created_at: now - MONTH + MINUTE,
            },
            DeliveryProblem {
                address: "foo@example.com".parse().unwrap(),
                problem_type: ProblemType::SoftBounce,
                problem_subtype: ProblemSubtype::Undetermined,
                created_at: now - DAY + SECOND * 2,
            },
            DeliveryProblem {
                address: "foo@example.com".parse().unwrap(),
                problem_type: ProblemType::HardBounce,
                problem_subtype: ProblemSubtype::Undetermined,
                created_at: now - WEEK + SECOND * 2,
            },
            DeliveryProblem {
                address: "foo@example.com".parse().unwrap(),
                problem_type: ProblemType::Complaint,
                problem_subtype: ProblemSubtype::Undetermined,
                created_at: now - MONTH + SECOND * 2,
            },
            DeliveryProblem {
                address: "foo@example.com".parse().unwrap(),
                problem_type: ProblemType::SoftBounce,
                problem_subtype: ProblemSubtype::Undetermined,
                created_at: now - DAY - 1000,
            },
            DeliveryProblem {
                address: "foo@example.com".parse().unwrap(),
                problem_type: ProblemType::HardBounce,
                problem_subtype: ProblemSubtype::Undetermined,
                created_at: now - WEEK - 1000,
            },
            DeliveryProblem {
                address: "foo@example.com".parse().unwrap(),
                problem_type: ProblemType::Complaint,
                problem_subtype: ProblemSubtype::Undetermined,
                created_at: now - MONTH - 1000,
            },
        ])
    }
}

#[test]
fn check_bounce_with_multiple_limits() {
    let bounce_settings: Json = serde_json::from_str(
        r#"{
        "enabled": true,
        "soft": [
        { "period": "2 seconds", "limit": 0 },
        { "period": "2 minutes", "limit": 1 },
        { "period": "2 hours", "limit": 2 }
        ],
        "hard": [],
        "complaint": []
        }"#,
    ).expect("Unexpected json parsing error");
    let settings = create_settings(bounce_settings);
    let db = DbMockBounceWithMultipleLimits;
    let problems = DeliveryProblems::new(&settings, db);
    match problems.check(&"foo@example.com".parse().unwrap()) {
        Ok(_) => assert!(false, "DeliveryProblems::check should have failed"),
        Err(error) => {
            assert_eq!(format!("{}", error), "Email account soft bounced");
            let err_data = error.kind().additional_fields();
            let address = err_data.get("address");
            if let Some(ref address) = address {
                assert_eq!("foo@example.com", address.as_str().unwrap());
            } else {
                assert!(false, "Error::address should be set");
            }
            let problem = err_data.get("problem");
            assert!(problem.is_some());
            let record: Json = serde_json::from_str(problem.unwrap().as_str().unwrap()).unwrap();
            assert_eq!(record["bounceType"], 2);
            assert_eq!(&record["createdAt"], err_data.get("time").unwrap());
        }
    }
}

#[derive(Debug)]
pub struct DbMockBounceWithMultipleLimits;

impl Db for DbMockBounceWithMultipleLimits {
    fn get_bounces(&self, _address: &EmailAddress) -> AppResult<Vec<DeliveryProblem>> {
        let now = now_as_milliseconds();
        Ok(vec![
            DeliveryProblem {
                address: "foo@example.com".parse().unwrap(),
                problem_type: ProblemType::SoftBounce,
                problem_subtype: ProblemSubtype::Undetermined,
                created_at: now - SECOND * 4,
            },
            DeliveryProblem {
                address: "foo@example.com".parse().unwrap(),
                problem_type: ProblemType::SoftBounce,
                problem_subtype: ProblemSubtype::Undetermined,
                created_at: now - MINUTE * 2 + SECOND * 4,
            },
            DeliveryProblem {
                address: "foo@example.com".parse().unwrap(),
                problem_type: ProblemType::SoftBounce,
                problem_subtype: ProblemSubtype::Undetermined,
                created_at: now - MINUTE * 2 + SECOND * 3,
            },
            DeliveryProblem {
                address: "foo@example.com".parse().unwrap(),
                problem_type: ProblemType::SoftBounce,
                problem_subtype: ProblemSubtype::Undetermined,
                created_at: now - MINUTE * 2 + SECOND * 2,
            },
        ])
    }
}

#[test]
fn record_bounce() {
    let settings = Settings::new().unwrap();
    let db = DbClient::new(&settings);
    let problems = DeliveryProblems::new(&settings, db);
    let address = create_address("record_bounce");
    let test = TestFixture::setup(&settings, address.as_ref(), DataType::DeliveryProblem);

    problems
        .record_bounce(
            &address,
            BounceType::Transient,
            BounceSubtype::AttachmentRejected,
        ).unwrap();

    test.assert_set();

    // Ensure there is an observable difference between timestamps
    sleep(Duration::from_millis(2));

    problems
        .record_bounce(&address, BounceType::Permanent, BounceSubtype::General)
        .unwrap();

    let db = DbClient::new(&settings);
    let bounce_records = db.get_bounces(&address).unwrap();
    let now = now_as_milliseconds();
    assert_eq!(bounce_records.len(), 2);
    assert_eq!(bounce_records[0].address, address);
    assert_eq!(bounce_records[0].problem_type, ProblemType::HardBounce);
    assert_eq!(bounce_records[0].problem_subtype, ProblemSubtype::General);
    assert!(bounce_records[0].created_at < now);
    assert!(bounce_records[0].created_at > now - 1000);

    assert_eq!(bounce_records[1].address, address);
    assert_eq!(bounce_records[1].problem_type, ProblemType::SoftBounce);
    assert_eq!(
        bounce_records[1].problem_subtype,
        ProblemSubtype::AttachmentRejected
    );
    assert!(bounce_records[1].created_at < now);
    assert!(bounce_records[1].created_at < bounce_records[0].created_at);

    test.assert_data(
        // created_at is probably a millisecond or two different between MySQL and Redis
        bounce_records
            .into_iter()
            .map(From::from)
            .collect::<Vec<AssertFriendlyDeliveryProblem>>(),
    );
}

fn create_address(test: &str) -> EmailAddress {
    format!(
        "fxa-email-service.bounces.test.{}.{}@example.com",
        test,
        now_as_milliseconds()
    ).parse()
    .unwrap()
}

#[test]
fn record_complaint() {
    let settings = Settings::new().unwrap();
    let db = DbClient::new(&settings);
    let problems = DeliveryProblems::new(&settings, db);
    let address = create_address("record_complaint");
    let test = TestFixture::setup(&settings, address.as_ref(), DataType::DeliveryProblem);

    problems
        .record_complaint(&address, Some(ComplaintFeedbackType::Virus))
        .unwrap();

    test.assert_set();

    let db = DbClient::new(&settings);
    let bounce_records = db.get_bounces(&address).unwrap();
    let now = now_as_milliseconds();
    assert_eq!(bounce_records.len(), 1);
    assert_eq!(bounce_records[0].address, address);
    assert_eq!(bounce_records[0].problem_type, ProblemType::Complaint);
    assert_eq!(bounce_records[0].problem_subtype, ProblemSubtype::Virus);
    assert!(bounce_records[0].created_at < now);
    assert!(bounce_records[0].created_at > now - 1000);

    test.assert_data(
        // created_at is probably a millisecond or two different between MySQL and Redis
        bounce_records
            .into_iter()
            .map(From::from)
            .collect::<Vec<AssertFriendlyDeliveryProblem>>(),
    );
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
struct AssertFriendlyDeliveryProblem {
    #[serde(rename = "email")]
    pub address: EmailAddress,
    #[serde(rename = "bounceType")]
    pub problem_type: ProblemType,
    #[serde(rename = "bounceSubType")]
    pub problem_subtype: ProblemSubtype,
}

impl From<DeliveryProblem> for AssertFriendlyDeliveryProblem {
    fn from(source: DeliveryProblem) -> Self {
        Self {
            address: source.address,
            problem_type: source.problem_type,
            problem_subtype: source.problem_subtype,
        }
    }
}

#[test]
fn deserialize_problem_type() {
    let problem_type: ProblemType = serde_json::from_value(From::from(0)).expect("JSON error");
    assert_eq!(problem_type, ProblemType::SoftBounce);
    let problem_type: ProblemType = serde_json::from_value(From::from(1)).expect("JSON error");
    assert_eq!(problem_type, ProblemType::HardBounce);
    let problem_type: ProblemType = serde_json::from_value(From::from(2)).expect("JSON error");
    assert_eq!(problem_type, ProblemType::SoftBounce);
    let problem_type: ProblemType = serde_json::from_value(From::from(3)).expect("JSON error");
    assert_eq!(problem_type, ProblemType::Complaint);
}

#[test]
fn deserialize_invalid_problem_type() {
    match serde_json::from_value::<ProblemType>(From::from(4)) {
        Ok(_) => assert!(false, "serde_json::from_value should have failed"),
        Err(error) => assert_eq!(
            format!("{}", error),
            "invalid value: integer `4`, expected problem type"
        ),
    }
    match serde_json::from_value::<ProblemType>(From::from(-1)) {
        Ok(_) => assert!(false, "serde_json::from_value should have failed"),
        Err(error) => assert_eq!(
            format!("{}", error),
            "invalid value: integer `-1`, expected u8"
        ),
    }
}

#[test]
fn serialize_problem_type() {
    let json = serde_json::to_string(&ProblemType::HardBounce).expect("JSON error");
    assert_eq!(json, "1");
    let json = serde_json::to_string(&ProblemType::SoftBounce).expect("JSON error");
    assert_eq!(json, "2");
    let json = serde_json::to_string(&ProblemType::Complaint).expect("JSON error");
    assert_eq!(json, "3");
}

#[test]
fn deserialize_problem_subtype() {
    let problem_type: ProblemSubtype = serde_json::from_value(From::from(0)).expect("JSON error");
    assert_eq!(problem_type, ProblemSubtype::Unmapped);
    let problem_type: ProblemSubtype = serde_json::from_value(From::from(1)).expect("JSON error");
    assert_eq!(problem_type, ProblemSubtype::Undetermined);
    let problem_type: ProblemSubtype = serde_json::from_value(From::from(2)).expect("JSON error");
    assert_eq!(problem_type, ProblemSubtype::General);
    let problem_type: ProblemSubtype = serde_json::from_value(From::from(3)).expect("JSON error");
    assert_eq!(problem_type, ProblemSubtype::NoEmail);
    let problem_type: ProblemSubtype = serde_json::from_value(From::from(4)).expect("JSON error");
    assert_eq!(problem_type, ProblemSubtype::Suppressed);
    let problem_type: ProblemSubtype = serde_json::from_value(From::from(5)).expect("JSON error");
    assert_eq!(problem_type, ProblemSubtype::MailboxFull);
    let problem_type: ProblemSubtype = serde_json::from_value(From::from(6)).expect("JSON error");
    assert_eq!(problem_type, ProblemSubtype::MessageTooLarge);
    let problem_type: ProblemSubtype = serde_json::from_value(From::from(7)).expect("JSON error");
    assert_eq!(problem_type, ProblemSubtype::ContentRejected);
    let problem_type: ProblemSubtype = serde_json::from_value(From::from(8)).expect("JSON error");
    assert_eq!(problem_type, ProblemSubtype::AttachmentRejected);
    let problem_type: ProblemSubtype = serde_json::from_value(From::from(9)).expect("JSON error");
    assert_eq!(problem_type, ProblemSubtype::Abuse);
    let problem_type: ProblemSubtype = serde_json::from_value(From::from(10)).expect("JSON error");
    assert_eq!(problem_type, ProblemSubtype::AuthFailure);
    let problem_type: ProblemSubtype = serde_json::from_value(From::from(11)).expect("JSON error");
    assert_eq!(problem_type, ProblemSubtype::Fraud);
    let problem_type: ProblemSubtype = serde_json::from_value(From::from(12)).expect("JSON error");
    assert_eq!(problem_type, ProblemSubtype::NotSpam);
    let problem_type: ProblemSubtype = serde_json::from_value(From::from(13)).expect("JSON error");
    assert_eq!(problem_type, ProblemSubtype::Other);
    let problem_type: ProblemSubtype = serde_json::from_value(From::from(14)).expect("JSON error");
    assert_eq!(problem_type, ProblemSubtype::Virus);
}

#[test]
fn deserialize_invalid_problem_subtype() {
    match serde_json::from_value::<ProblemSubtype>(From::from(15)) {
        Ok(_) => assert!(false, "serde_json::from_value should have failed"),
        Err(error) => assert_eq!(
            format!("{}", error),
            "invalid value: integer `15`, expected problem subtype"
        ),
    }
    match serde_json::from_value::<ProblemSubtype>(From::from(-1)) {
        Ok(_) => assert!(false, "serde_json::from_value should have failed"),
        Err(error) => assert_eq!(
            format!("{}", error),
            "invalid value: integer `-1`, expected u8"
        ),
    }
}

#[test]
fn serialize_problem_subtype() {
    let json = serde_json::to_string(&ProblemSubtype::Unmapped).expect("JSON error");
    assert_eq!(json, "0");
    let json = serde_json::to_string(&ProblemSubtype::Undetermined).expect("JSON error");
    assert_eq!(json, "1");
    let json = serde_json::to_string(&ProblemSubtype::General).expect("JSON error");
    assert_eq!(json, "2");
    let json = serde_json::to_string(&ProblemSubtype::NoEmail).expect("JSON error");
    assert_eq!(json, "3");
    let json = serde_json::to_string(&ProblemSubtype::Suppressed).expect("JSON error");
    assert_eq!(json, "4");
    let json = serde_json::to_string(&ProblemSubtype::MailboxFull).expect("JSON error");
    assert_eq!(json, "5");
    let json = serde_json::to_string(&ProblemSubtype::MessageTooLarge).expect("JSON error");
    assert_eq!(json, "6");
    let json = serde_json::to_string(&ProblemSubtype::ContentRejected).expect("JSON error");
    assert_eq!(json, "7");
    let json = serde_json::to_string(&ProblemSubtype::AttachmentRejected).expect("JSON error");
    assert_eq!(json, "8");
    let json = serde_json::to_string(&ProblemSubtype::Abuse).expect("JSON error");
    assert_eq!(json, "9");
    let json = serde_json::to_string(&ProblemSubtype::AuthFailure).expect("JSON error");
    assert_eq!(json, "10");
    let json = serde_json::to_string(&ProblemSubtype::Fraud).expect("JSON error");
    assert_eq!(json, "11");
    let json = serde_json::to_string(&ProblemSubtype::NotSpam).expect("JSON error");
    assert_eq!(json, "12");
    let json = serde_json::to_string(&ProblemSubtype::Other).expect("JSON error");
    assert_eq!(json, "13");
    let json = serde_json::to_string(&ProblemSubtype::Virus).expect("JSON error");
    assert_eq!(json, "14");
}
