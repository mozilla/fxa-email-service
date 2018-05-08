// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, you can obtain one at https://mozilla.org/MPL/2.0/.

use super::{Provider, ProviderError};

pub struct MockProvider;

impl Provider for MockProvider
{
  fn send(
    &self,
    _to: &str,
    _cc: &[&str],
    _subject: &str,
    _body_text: &str,
    _body_html: Option<&str>,
  ) -> Result<String, ProviderError>
  {
    Ok(String::from("deadbeef"))
  }
}
