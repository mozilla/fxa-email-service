// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, you can obtain one at https://mozilla.org/MPL/2.0/.

use super::*;
use auth_db::{Db, DbError};
use settings::Settings;

pub struct DbMockNoBounce;

impl Db for DbMockNoBounce
{
  fn get_email_bounces(&self, _address: &str) -> Result<Vec<BounceRecord>, DbError>
  {
    let now = now_as_milliseconds();
    Ok(vec![
      BounceRecord {
        bounce_type: BounceType::Hard,
        created_at: now - 86400001,
      },
      BounceRecord {
        bounce_type: BounceType::Soft,
        created_at: now - 300001,
      },
      BounceRecord {
        bounce_type: BounceType::Complaint,
        created_at: now - 86400001,
      },
    ])
  }
}

fn now_as_milliseconds() -> u64
{
  let now = SystemTime::now()
    .duration_since(SystemTime::UNIX_EPOCH)
    .expect("system time error");
  now.as_secs() * 1000
}

#[test]
fn check_no_bounces()
{
  let settings = Settings::new().expect("config error");
  let db = DbMockNoBounce;
  let bounces = Bounces::new(&settings, Box::new(&db));
  if let Err(error) = bounces.check("foo@example.com") {
    assert!(false, error.description().to_string());
  }
}

pub struct DbMockBounceHard;

impl Db for DbMockBounceHard
{
  fn get_email_bounces(&self, _address: &str) -> Result<Vec<BounceRecord>, DbError>
  {
    let now = now_as_milliseconds();
    Ok(vec![BounceRecord {
      bounce_type: BounceType::Hard,
      created_at: now - 86398000,
    }])
  }
}

#[test]
fn check_hard_bounce()
{
  let settings = Settings::new().expect("config error");
  let db = DbMockBounceHard;
  let bounces = Bounces::new(&settings, Box::new(&db));
  match bounces.check("foo@example.com") {
    Ok(_) => assert!(false, "Bounces::check should have failed"),
    Err(error) => {
      assert_eq!(
        error.description(),
        "email address violated hard bounce limit"
      );
      assert_eq!(error.address, "foo@example.com");
      if let Some(bounce) = error.bounce {
        assert_eq!(bounce.bounce_type, BounceType::Hard);
      } else {
        assert!(false, "Error::bounce should be set");
      }
    }
  }
}

pub struct DbMockBounceSoft;

impl Db for DbMockBounceSoft
{
  fn get_email_bounces(&self, _address: &str) -> Result<Vec<BounceRecord>, DbError>
  {
    let now = now_as_milliseconds();
    Ok(vec![BounceRecord {
      bounce_type: BounceType::Soft,
      created_at: now - 298000,
    }])
  }
}

#[test]
fn check_soft_bounce()
{
  let settings = Settings::new().expect("config error");
  let db = DbMockBounceSoft;
  let bounces = Bounces::new(&settings, Box::new(&db));
  match bounces.check("bar@example.com") {
    Ok(_) => assert!(false, "Bounces::check should have failed"),
    Err(error) => {
      assert_eq!(
        error.description(),
        "email address violated soft bounce limit"
      );
      assert_eq!(error.address, "bar@example.com");
      if let Some(bounce) = error.bounce {
        assert_eq!(bounce.bounce_type, BounceType::Soft);
      } else {
        assert!(false, "Error::bounce should be set");
      }
    }
  }
}

pub struct DbMockComplaint;

impl Db for DbMockComplaint
{
  fn get_email_bounces(&self, _address: &str) -> Result<Vec<BounceRecord>, DbError>
  {
    let now = now_as_milliseconds();
    Ok(vec![BounceRecord {
      bounce_type: BounceType::Complaint,
      created_at: now - 86398000,
    }])
  }
}

#[test]
fn check_complaint()
{
  let settings = Settings::new().expect("config error");
  let db = DbMockComplaint;
  let bounces = Bounces::new(&settings, Box::new(&db));
  match bounces.check("baz@example.com") {
    Ok(_) => assert!(false, "Bounces::check should have failed"),
    Err(error) => {
      assert_eq!(
        error.description(),
        "email address violated complaint limit"
      );
      assert_eq!(error.address, "baz@example.com");
      if let Some(bounce) = error.bounce {
        assert_eq!(bounce.bounce_type, BounceType::Complaint);
      } else {
        assert!(false, "Error::bounce should be set");
      }
    }
  }
}

pub struct DbMockError;

impl Db for DbMockError
{
  fn get_email_bounces(&self, _address: &str) -> Result<Vec<BounceRecord>, DbError>
  {
    Err(DbError::new(String::from("wibble blee")))
  }
}

#[test]
fn check_db_error()
{
  let settings = Settings::new().expect("config error");
  let db = DbMockError;
  let bounces = Bounces::new(&settings, Box::new(&db));
  match bounces.check("foo@example.com") {
    Ok(_) => assert!(false, "Bounces::check should have failed"),
    Err(error) => {
      assert_eq!(error.description(), "database error: wibble blee");
      assert_eq!(error.address, "");
      if let Some(_) = error.bounce {
        assert!(false, "Error::bounce should not be set");
      }
    }
  }
}
