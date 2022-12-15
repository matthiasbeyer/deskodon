use std::sync::Arc;

use crate::error::Error;

use deskodon_types::access_token::AccessToken;
use deskodon_types::auth::Auth;
use deskodon_types::error::Error as DError;

use megalodon::mastodon::Mastodon;

const USER_AGENT: &str = "deskodon";

#[derive(Debug)]
pub struct State {
    mastodon: Option<Arc<Mastodon>>,
}

impl Default for State {
    fn default() -> Self {
        Self { mastodon: None }
    }
}

impl State {
    pub fn connect(&mut self, instance: url::Url, token: AccessToken) -> Result<(), Error> {
        let mastodon = megalodon::mastodon::Mastodon::new(
            instance.as_ref().to_string(),
            Some(token.as_ref().to_string()),
            Some(USER_AGENT.to_string()),
        );

        self.mastodon = Some(Arc::new(mastodon));
        Ok(())
    }

    pub fn connected(&self) -> bool {
        self.mastodon.is_some()
    }
}
