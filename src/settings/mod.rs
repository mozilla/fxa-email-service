use std::env;

use config::{Config, ConfigError, Environment, File};

use deserialize::{Host, Sender};

#[cfg(test)]
mod test;

#[derive(Debug, Deserialize)]
pub struct Settings
{
  pub sender: Sender,
  // HACK: Nested settings prevent the tests from creating fresh values
  pub smtp_host: Host,
  pub smtp_port: u16,
  pub smtp_user: Option<String>,
  pub smtp_password: Option<String>,
}

impl Settings
{
  /// Construct a `Settings` instance, populating it with data from the file
  /// system and local environment.
  ///
  /// Precedence (earlier items override later ones):
  ///
  ///   1. Environment variables: `$FXA_EMAIL_<UPPERCASE_KEY_NAME>`
  ///   2. File: `config/local.json`
  ///   3. File: `config/<$NODE_ENV>.json`
  ///   4. File: `config/default.json`
  ///
  /// `$NODE_ENV` is used so that this service automatically picks up the
  /// appropriate state from our existing node.js ecosystem, without needing
  /// to manage an extra environment variable.
  ///
  /// Immediately before returning, the parsed config object will be logged to
  /// the console.
  pub fn new() -> Result<Self, ConfigError>
  {
    let mut config = Config::new();

    config.merge(File::with_name("config/default"))?;

    if let Ok(node_env) = env::var("NODE_ENV") {
      config.merge(File::with_name(&format!("config/{}", node_env)).required(false))?;
    }

    config.merge(File::with_name("config/local").required(false))?;

    config.merge(Environment::with_prefix("fxa_email"))?;

    match config.try_into::<Settings>() {
      Ok(settings) => {
        // TODO: replace this with proper logging when we have it
        println!("config: {:?}", settings);
        Ok(settings)
      }
      Err(error) => Err(error),
    }
  }
}
