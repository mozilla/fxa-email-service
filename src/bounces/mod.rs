// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, you can obtain one at https://mozilla.org/MPL/2.0/.

use std::{
    collections::HashMap,
    error::Error,
    fmt::{self, Display, Formatter},
    time::SystemTime,
};

use rocket::{http::Status, response::Failure};

use auth_db::{BounceRecord, BounceType, Db, DbError};
use settings::{BounceLimit, BounceLimits, Settings};

#[cfg(test)]
mod test;

#[derive(Debug)]
pub struct BounceError {
    pub address: String,
    pub bounce: Option<BounceRecord>,
    description: String,
}

impl BounceError {
    pub fn new(address: &str, bounce: &BounceRecord) -> BounceError {
        let description = format!(
            "email address violated {} limit",
            match bounce.bounce_type {
                BounceType::Hard => "hard bounce",
                BounceType::Soft => "soft bounce",
                BounceType::Complaint => "complaint",
            }
        );

        BounceError {
            address: address.to_string(),
            bounce: Some(bounce.clone()),
            description,
        }
    }
}

impl Error for BounceError {
    fn description(&self) -> &str {
        &self.description
    }
}

impl Display for BounceError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "{}", self.description)
    }
}

impl From<DbError> for BounceError {
    fn from(error: DbError) -> BounceError {
        BounceError {
            address: String::from(""),
            bounce: None,
            description: format!("database error: {}", error.description()),
        }
    }
}

impl From<BounceError> for Failure {
    fn from(error: BounceError) -> Failure {
        // Eventually we should be able to do something richer than this,
        // as per https://github.com/SergioBenitez/Rocket/issues/586.
        Failure(
            error
                .bounce
                .map_or(Status::InternalServerError, |_| Status::TooManyRequests),
        )
    }
}

pub struct Bounces<D: Db> {
    db: D,
    limits: BounceLimits,
}

impl<D> Bounces<D>
where
    D: Db,
{
    pub fn new(settings: &Settings, db: D) -> Bounces<D> {
        Bounces {
            db,
            limits: settings.bouncelimits.clone(),
        }
    }

    pub fn check(&self, address: &str) -> Result<(), BounceError> {
        let bounces = self.db.get_bounces(address)?;
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("system time error");
        let now = now.as_secs() * 1000;
        bounces
            .iter()
            .try_fold(HashMap::new(), |mut counts, bounce| {
                {
                    let count = counts.entry(&bounce.bounce_type).or_insert(0);
                    *count += 1;
                    let limits = match bounce.bounce_type {
                        BounceType::Hard => &self.limits.hard,
                        BounceType::Soft => &self.limits.soft,
                        BounceType::Complaint => &self.limits.complaint,
                    };
                    if is_bounce_violation(*count, bounce.created_at, now, limits) {
                        return Err(BounceError::new(address, bounce));
                    }
                }

                Ok(counts)
            })
            .map(|_| ())
    }
}

unsafe impl<D> Sync for Bounces<D> where D: Db {}

fn is_bounce_violation(count: u8, created_at: u64, now: u64, limits: &Vec<BounceLimit>) -> bool {
    for limit in limits.iter() {
        if count > limit.limit && created_at >= now - limit.period {
            return true;
        }
    }

    false
}
