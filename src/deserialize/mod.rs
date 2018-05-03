use std::fmt::{self, Display, Formatter};

use regex::Regex;
use serde::de::{Deserialize, Deserializer, Error, Unexpected};

lazy_static! {
  static ref HOST_FORMAT: Regex = Regex::new("^[A-Za-z0-9-]+(?:\\.[A-Za-z0-9-]+)*$").unwrap();
  static ref SENDER_FORMAT: Regex = Regex::new(
    "^[A-Za-z0-9-]+(?: [A-Za-z0-9-]+)* <[a-z0-9-]+@[a-z0-9-]+(?:\\.[a-z0-9-]+)+>$"
  ).unwrap();
}

#[derive(Debug, PartialEq)]
pub struct Host(pub String);

impl Host
{
  pub fn as_str(&self) -> &str
  {
    self.0.as_str()
  }
}

impl<'d> Deserialize<'d> for Host
{
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'d>,
  {
    match deserialize_string(deserializer, &HOST_FORMAT, "host name or IP address") {
      Ok(host) => Ok(Host(host)),
      Err(error) => Err(error),
    }
  }
}

impl Display for Host
{
  fn fmt(&self, formatter: &mut Formatter) -> fmt::Result
  {
    write!(formatter, "{}", self.as_str())
  }
}

#[derive(Debug, PartialEq)]
pub struct Sender(pub String);

impl Sender
{
  pub fn as_str(&self) -> &str
  {
    self.0.as_str()
  }
}

impl<'d> Deserialize<'d> for Sender
{
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'d>,
  {
    match deserialize_string(
      deserializer,
      &SENDER_FORMAT,
      "sender name and email address",
    ) {
      Ok(sender) => Ok(Sender(sender)),
      Err(error) => Err(error),
    }
  }
}

impl Display for Sender
{
  fn fmt(&self, formatter: &mut Formatter) -> fmt::Result
  {
    write!(formatter, "{}", self.as_str())
  }
}

fn deserialize_string<'d, D>(
  deserializer: D,
  format: &Regex,
  expected: &str,
) -> Result<String, D::Error>
where
  D: Deserializer<'d>,
{
  let value: String = Deserialize::deserialize(deserializer)?;
  if format.is_match(&value) {
    Ok(value)
  } else {
    Err(D::Error::invalid_value(Unexpected::Str(&value), &expected))
  }
}
