// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, you can obtain one at https://mozilla.org/MPL/2.0/.

//! Storage for configuration data.

use super::core::{Client as DbClient, DataType, Merge};
use settings::{DeliveryProblemLimits, Sender, Settings, SqsUrl};
use types::{error::AppResult, provider::Provider, regex::SerializableRegex};

#[cfg(test)]
mod test;

/// Configuration data store.
///
/// Data is keyed by client id.
#[derive(Debug)]
pub struct Config {
    client: DbClient,
}

impl Config {
    /// Instantiate a storage client.
    pub fn new(settings: &Settings) -> Self {
        Self {
            client: DbClient::new(settings),
        }
    }

    /// Read configuration data.
    pub fn get(&self, client_id: &str) -> AppResult<Option<Data>> {
        self.client.get(client_id, DataType::Configuration)
    }

    /// Store configuration data.
    ///
    /// Any data previously stored for the client id
    /// will be replaced.
    pub fn set(&self, client_id: &str, config: &Data) -> AppResult<()> {
        self.client.set(client_id, config, DataType::Configuration)
    }

    /// Merge configuration data with existing.
    pub fn merge(&self, client_id: &str, config: &mut Data) -> AppResult<()> {
        self.client
            .merge(client_id, config, DataType::Configuration)
    }
}

/// Configuration data.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Data {
    /// The fallback provider if no rules match and it's not specified in the call to `/send`.
    default_provider: Option<Provider>,

    /// Thresholds for bounce and complaint reports.
    limits: Option<DeliveryProblemLimits>,

    /// The queue for outgoing bounce, complaint and delivery notifications.
    queue: Option<SqsUrl>,

    /// Rules for selectively diverting subsets of email traffic via different providers.
    rules: Option<Vec<ProviderRules>>,

    /// The name and email address to use in the `From` and `Sender` headers.
    sender: Option<Sender>,
}

impl Merge<Data> for Data {
    fn merge(&self, with: &Self) -> Self {
        Self {
            default_provider: self
                .default_provider
                .as_ref()
                .map(|default_provider| default_provider.clone())
                .or_else(|| with.default_provider.clone()),
            limits: self
                .limits
                .as_ref()
                .map(|limits| limits.clone())
                .or_else(|| with.limits.clone()),
            queue: self
                .queue
                .as_ref()
                .map(|queue| queue.clone())
                .or_else(|| with.queue.clone()),
            rules: self
                .rules
                .as_ref()
                .map(|rules| {
                    with.rules.as_ref().map_or_else(
                        || rules.clone(),
                        |with_rules| {
                            let mut with_rules = with_rules.clone();
                            with_rules.extend(rules.clone());
                            with_rules
                        },
                    )
                })
                .or_else(|| with.rules.clone()),
            sender: self
                .sender
                .as_ref()
                .map(|sender| sender.clone())
                .or_else(|| with.sender.clone()),
        }
    }
}

/// Rules for selectively diverting subsets of email traffic via different providers.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ProviderRules {
    /// The percentage of prospective addresses to divert via this provider.
    percentage: Option<u8>,

    /// Relative precedence of this ruleset against its peers.
    /// Default is `0` (lower number equals higher precedence).
    precedence: Option<i8>,

    /// The email provider for this ruleset.
    provider: Provider,

    /// Filter for matching email addresses.
    regex: Option<SerializableRegex>,
}
