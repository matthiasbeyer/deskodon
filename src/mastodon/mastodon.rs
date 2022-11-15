use super::AccessToken;

const USER_AGENT: &str = "deskodon";

pub struct Mastodon {
    mastodon: megalodon::mastodon::Mastodon,
}

impl Mastodon {
    pub fn new(instance: url::Url, token: AccessToken) -> Self {
        let mastodon = megalodon::mastodon::Mastodon::new(
            instance.as_ref().to_string(),
            Some(token.as_ref().to_string()),
            Some(USER_AGENT.to_string()),
        );

        Self { mastodon }
    }
}

impl std::fmt::Debug for Mastodon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Mastodon").field("mastodon", &self.mastodon).finish()
    }
}
