// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, you can obtain one at https://mozilla.org/MPL/2.0/.

use period::*;

#[test]
fn without_multipliers()
{
  match Period::try_from("second") {
    Ok(period) => assert_eq!(period.into(): u64, 1000),
    Err(error) => assert!(false, error.description().to_string()),
  }

  match Period::try_from("seconds") {
    Ok(period) => assert_eq!(period.into(): u64, 1000),
    Err(error) => assert!(false, error.description().to_string()),
  }

  match Period::try_from("minute") {
    Ok(period) => assert_eq!(period.into(): u64, 60000),
    Err(error) => assert!(false, error.description().to_string()),
  }

  match Period::try_from("hour") {
    Ok(period) => assert_eq!(period.into(): u64, 3600000),
    Err(error) => assert!(false, error.description().to_string()),
  }

  match Period::try_from("day") {
    Ok(period) => assert_eq!(period.into(): u64, 86400000),
    Err(error) => assert!(false, error.description().to_string()),
  }

  match Period::try_from("week") {
    Ok(period) => assert_eq!(period.into(): u64, 604800000),
    Err(error) => assert!(false, error.description().to_string()),
  }

  match Period::try_from("month") {
    Ok(period) => assert_eq!(period.into(): u64, 2592000000),
    Err(error) => assert!(false, error.description().to_string()),
  }

  match Period::try_from("year") {
    Ok(period) => assert_eq!(period.into(): u64, 31536000000),
    Err(error) => assert!(false, error.description().to_string()),
  }
}

#[test]
fn with_multipliers()
{
  match Period::try_from("2 seconds") {
    Ok(period) => assert_eq!(period.into(): u64, 2000),
    Err(error) => assert!(false, error.description().to_string()),
  }

  match Period::try_from("2 second") {
    Ok(period) => assert_eq!(period.into(): u64, 2000),
    Err(error) => assert!(false, error.description().to_string()),
  }

  match Period::try_from("3 seconds") {
    Ok(period) => assert_eq!(period.into(): u64, 3000),
    Err(error) => assert!(false, error.description().to_string()),
  }

  match Period::try_from("0 seconds") {
    Ok(period) => assert_eq!(period.into(): u64, 0),
    Err(error) => assert!(false, error.description().to_string()),
  }

  match Period::try_from("2 minutes") {
    Ok(period) => assert_eq!(period.into(): u64, 120000),
    Err(error) => assert!(false, error.description().to_string()),
  }

  match Period::try_from("2 hours") {
    Ok(period) => assert_eq!(period.into(): u64, 7200000),
    Err(error) => assert!(false, error.description().to_string()),
  }

  match Period::try_from("2 days") {
    Ok(period) => assert_eq!(period.into(): u64, 172800000),
    Err(error) => assert!(false, error.description().to_string()),
  }

  match Period::try_from("2 weeks") {
    Ok(period) => assert_eq!(period.into(): u64, 1209600000),
    Err(error) => assert!(false, error.description().to_string()),
  }

  match Period::try_from("2 months") {
    Ok(period) => assert_eq!(period.into(): u64, 5184000000),
    Err(error) => assert!(false, error.description().to_string()),
  }

  match Period::try_from("2 years") {
    Ok(period) => assert_eq!(period.into(): u64, 63072000000),
    Err(error) => assert!(false, error.description().to_string()),
  }
}

#[test]
fn invalid_patterns()
{
  match Period::try_from("seconx") {
    Ok(_) => assert!(false, "Period::try_from should have failed"),
    Err(error) => {
      assert_eq!(error.description().to_string(), "invalid period of time");
      assert_eq!(error.value, "seconx");
    }
  }

  match Period::try_from("secondx") {
    Ok(_) => assert!(false, "Period::try_from should have failed"),
    Err(error) => {
      assert_eq!(error.description().to_string(), "invalid period of time");
      assert_eq!(error.value, "secondx");
    }
  }

  match Period::try_from("secondsx") {
    Ok(_) => assert!(false, "Period::try_from should have failed"),
    Err(error) => {
      assert_eq!(error.description().to_string(), "invalid period of time");
      assert_eq!(error.value, "secondsx");
    }
  }

  match Period::try_from(" second") {
    Ok(_) => assert!(false, "Period::try_from should have failed"),
    Err(error) => {
      assert_eq!(error.description().to_string(), "invalid period of time");
      assert_eq!(error.value, " second");
    }
  }

  match Period::try_from("second ") {
    Ok(_) => assert!(false, "Period::try_from should have failed"),
    Err(error) => {
      assert_eq!(error.description().to_string(), "invalid period of time");
      assert_eq!(error.value, "second ");
    }
  }

  match Period::try_from("2x seconds") {
    Ok(_) => assert!(false, "Period::try_from should have failed"),
    Err(error) => {
      assert_eq!(error.description().to_string(), "invalid period of time");
      assert_eq!(error.value, "2x seconds");
    }
  }
}
