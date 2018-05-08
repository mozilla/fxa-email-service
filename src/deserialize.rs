// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, you can obtain one at https://mozilla.org/MPL/2.0/.

use regex::Regex;
use serde::de::{Deserialize, Deserializer, Error, Unexpected};

use validate;

// Time periods are measured in milliseconds, to play nicely with
// the rest of the FxA ecosystem
const SECOND: u64 = 1000;
const MINUTE: u64 = SECOND * 60;
const HOUR: u64 = MINUTE * 60;
const DAY: u64 = HOUR * 24;
const WEEK: u64 = DAY * 7;
const MONTH: u64 = DAY * 30;
const YEAR: u64 = DAY * 365;

lazy_static! {
  static ref PERIOD_FORMAT: Regex =
    Regex::new("^(?:([0-9]+) )?(second|minute|hour|day|week|month|year)s?$").unwrap();
}

pub fn aws_region<'d, D>(deserializer: D) -> Result<String, D::Error>
where
  D: Deserializer<'d>,
{
  deserialize(deserializer, validate::aws_region, "AWS region")
}

pub fn aws_access<'d, D>(deserializer: D) -> Result<String, D::Error>
where
  D: Deserializer<'d>,
{
  deserialize(deserializer, validate::aws_access, "AWS access key")
}

pub fn aws_secret<'d, D>(deserializer: D) -> Result<String, D::Error>
where
  D: Deserializer<'d>,
{
  deserialize(deserializer, validate::aws_secret, "AWS secret key")
}

pub fn base_uri<'d, D>(deserializer: D) -> Result<String, D::Error>
where
  D: Deserializer<'d>,
{
  deserialize(deserializer, validate::base_uri, "base URI")
}

pub fn host<'d, D>(deserializer: D) -> Result<String, D::Error>
where
  D: Deserializer<'d>,
{
  deserialize(deserializer, validate::host, "host name or IP address")
}

pub fn period<'d, D>(deserializer: D) -> Result<u64, D::Error>
where
  D: Deserializer<'d>,
{
  fn fail<'d, D>(value: &str) -> Result<u64, D::Error>
  where
    D: Deserializer<'d>,
  {
    Err(D::Error::invalid_value(
      Unexpected::Str(&value),
      &"time period",
    ))
  }

  let value: String = Deserialize::deserialize(deserializer)?;
  if let Some(matches) = PERIOD_FORMAT.captures(&value) {
    if let Ok(multiplier) = matches.get(1).map_or(Ok(1), |m| m.as_str().parse::<u64>()) {
      match matches.get(2).map_or("", |m| m.as_str()) {
        "second" => Ok(multiplier * SECOND),
        "minute" => Ok(multiplier * MINUTE),
        "hour" => Ok(multiplier * HOUR),
        "day" => Ok(multiplier * DAY),
        "week" => Ok(multiplier * WEEK),
        "month" => Ok(multiplier * MONTH),
        "year" => Ok(multiplier * YEAR),
        _ => fail::<D>(&value),
      }
    } else {
      fail::<D>(&value)
    }
  } else {
    fail::<D>(&value)
  }
}

pub fn provider<'d, D>(deserializer: D) -> Result<String, D::Error>
where
  D: Deserializer<'d>,
{
  deserialize(deserializer, validate::provider, "'ses' or 'smtp'")
}

pub fn sender<'d, D>(deserializer: D) -> Result<String, D::Error>
where
  D: Deserializer<'d>,
{
  deserialize(
    deserializer,
    validate::sender,
    "sender name and email address",
  )
}

fn deserialize<'d, D>(
  deserializer: D,
  validator: fn(&str) -> bool,
  expected: &str,
) -> Result<String, D::Error>
where
  D: Deserializer<'d>,
{
  let value: String = Deserialize::deserialize(deserializer)?;
  if validator(&value) {
    Ok(value)
  } else {
    Err(D::Error::invalid_value(Unexpected::Str(&value), &expected))
  }
}
