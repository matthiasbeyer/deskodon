use std::sync::Arc;

use futures::FutureExt;
use megalodon::{entities::Status, Megalodon};

use super::AccessToken;

const USER_AGENT: &str = "deskodon";

#[derive(Clone)]
pub struct Mastodon {
    mastodon: Arc<megalodon::mastodon::Mastodon>,
}

impl Mastodon {
    pub fn new(instance: url::Url, token: AccessToken) -> Self {
        let mastodon = megalodon::mastodon::Mastodon::new(
            instance.as_ref().to_string(),
            Some(token.as_ref().to_string()),
            Some(USER_AGENT.to_string()),
        );

        Self { mastodon: Arc::new(mastodon) }
    }

    pub async fn get_home_timeline(&self) -> Result<Vec<Status>, String /* TODO */> {
        let options = megalodon::megalodon::GetHomeTimelineInputOptions {
            only_media: Some(false),
            limit: Some(10),
            max_id: None,
            since_id: None,
            min_id: None,
            local: Some(true),
        };

        self.mastodon.get_home_timeline(Some(&options))
            .map(|res| match res {
                Ok(response) => Ok(response.json),
                Err(e) => Err(e.to_string()),
            })
            .await
    }
}

impl std::fmt::Debug for Mastodon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Mastodon").field("mastodon", &self.mastodon).finish()
    }
}
