// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, you can obtain one at https://mozilla.org/MPL/2.0/.

//! Maps duration strings to millisecond values.

use std::convert::{From, TryFrom};

use regex::Regex;
use serde::{
    de::{Deserialize, Deserializer, Error as SerdeError, Unexpected},
    ser::{Serialize, Serializer},
};

use types::error::{AppError, AppErrorKind, AppResult};

#[cfg(test)]
mod test;

// Durations are measured in milliseconds, to play nicely with
// the rest of the FxA ecosystem
const SECOND: u64 = 1000;
const MINUTE: u64 = SECOND * 60;
const HOUR: u64 = MINUTE * 60;
const DAY: u64 = HOUR * 24;
const WEEK: u64 = DAY * 7;
const MONTH: u64 = DAY * 30;
const YEAR: u64 = DAY * 365;

lazy_static! {
    static ref DURATION_FORMAT: Regex =
        Regex::new("^(?:([0-9]+) )?(second|minute|hour|day|week|month|year)s?$").unwrap();
}

/// A duration type
/// represented in milliseconds,
/// for compatibility with
/// the rest of the FxA ecosystem.
///
/// Can be serialized to
/// and deserialized from
/// duration strings
/// of the format `"{number} {period}"`,
/// e.g. `"1 hour"` or `"10 minutes"`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Duration(pub u64);

impl<'d> Deserialize<'d> for Duration {
    /// Validate and deserialize a
    /// duration from a string
    /// of the format `"{number} {period}"`,
    /// e.g. `"1 hour"` or `"10 minutes"`.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'d>,
    {
        let value: String = Deserialize::deserialize(deserializer)?;
        Duration::try_from(value.as_str())
            .map(From::from)
            .map_err(|_| D::Error::invalid_value(Unexpected::Str(&value), &"duration"))
    }
}

impl Serialize for Duration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut duration = if self.0 % YEAR == 0 {
            format!("{} year", self.0 / YEAR)
        } else if self.0 % MONTH == 0 {
            format!("{} month", self.0 / MONTH)
        } else if self.0 % WEEK == 0 {
            format!("{} week", self.0 / WEEK)
        } else if self.0 % DAY == 0 {
            format!("{} day", self.0 / DAY)
        } else if self.0 % HOUR == 0 {
            format!("{} hour", self.0 / HOUR)
        } else if self.0 % MINUTE == 0 {
            format!("{} minute", self.0 / MINUTE)
        } else {
            format!("{} second", self.0 / SECOND)
        };

        if self.0 > 1 {
            duration = format!("{}s", duration);
        }

        serializer.serialize_str(&duration)
    }
}

impl From<Duration> for u64 {
    fn from(value: Duration) -> u64 {
        value.0
    }
}

impl<'v> TryFrom<&'v str> for Duration {
    type Error = AppError;

    fn try_from(value: &str) -> AppResult<Duration> {
        fn fail(value: &str) -> AppResult<Duration> {
            Err(AppErrorKind::InvalidDuration(value.to_string()))?
        }

        if let Some(matches) = DURATION_FORMAT.captures(value) {
            if let Ok(multiplier) = matches.get(1).map_or(Ok(1), |m| m.as_str().parse::<u64>()) {
                return match matches.get(2).map_or("", |m| m.as_str()) {
                    "second" => Ok(Duration(multiplier * SECOND)),
                    "minute" => Ok(Duration(multiplier * MINUTE)),
                    "hour" => Ok(Duration(multiplier * HOUR)),
                    "day" => Ok(Duration(multiplier * DAY)),
                    "week" => Ok(Duration(multiplier * WEEK)),
                    "month" => Ok(Duration(multiplier * MONTH)),
                    "year" => Ok(Duration(multiplier * YEAR)),
                    _ => fail(value),
                };
            }
        }

        fail(value)
    }
}
