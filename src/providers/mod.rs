// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, you can obtain one at https://mozilla.org/MPL/2.0/.

use std::{
    boxed::Box, collections::HashMap, error::Error, fmt::{self, Display, Formatter},
};

use self::{
    mock::MockProvider as Mock, sendgrid::SendgridProvider as Sendgrid, ses::SesProvider as Ses,
};
use settings::Settings;

mod mock;
mod sendgrid;
mod ses;

trait Provider {
    fn send(
        &self,
        to: &str,
        cc: &[&str],
        subject: &str,
        body_text: &str,
        body_html: Option<&str>,
    ) -> Result<String, ProviderError>;
}

#[derive(Debug)]
pub struct ProviderError {
    description: String,
}

impl ProviderError {
    pub fn new(description: String) -> ProviderError {
        ProviderError { description }
    }
}

impl Error for ProviderError {
    fn description(&self) -> &str {
        &self.description
    }
}

impl Display for ProviderError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "{}", self.description)
    }
}

pub struct Providers<'s> {
    default_provider: &'s str,
    providers: HashMap<String, Box<Provider + 's>>,
}

impl<'s> Providers<'s> {
    pub fn new(settings: &'s Settings) -> Providers {
        let mut providers: HashMap<String, Box<Provider + 's>> = HashMap::new();

        providers.insert(String::from("mock"), Box::new(Mock));
        providers.insert(String::from("ses"), Box::new(Ses::new(settings)));

        if let Some(ref sendgrid) = settings.sendgrid {
            providers.insert(
                String::from("sendgrid"),
                Box::new(Sendgrid::new(sendgrid, settings)),
            );
        }

        Providers {
            default_provider: &settings.provider,
            providers,
        }
    }

    pub fn send(
        &self,
        to: &str,
        cc: &[&str],
        subject: &str,
        body_text: &str,
        body_html: Option<&str>,
        provider_id: Option<&str>,
    ) -> Result<String, ProviderError> {
        let resolved_provider_id = provider_id.unwrap_or(self.default_provider);

        self.providers
            .get(resolved_provider_id)
            .ok_or(ProviderError::new(format!(
                "Invalid provider `{}`",
                resolved_provider_id
            )))
            .and_then(|provider| provider.send(to, cc, subject, body_text, body_html))
            .map(|message_id| format!("{}:{}", resolved_provider_id, message_id))
    }
}

unsafe impl<'s> Sync for Providers<'s> {}
