#[derive(Debug, Clone)]
pub enum Message {
    ConfigLoaded(crate::config::Config),
    ConfigLoadingFailed(String),

    InstanceInputChanged(String),
    UsernameInputChanged(String),

    GenerateAccessTokenButtonPressed,
    GeneratedAuth(crate::mastodon::Auth),
    GeneratedAuthFailed(String),

    AccessTokenInputChanged(String),

    LoginButtonPressed(crate::mastodon::Auth, String),
    SavingConfigSucceeded,
    SavingConfigFailed(String),

    UrlOpened,
    UrlOpenFailed(String),

    AccessTokenFetched(crate::mastodon::AccessToken),
    AccessTokenFetchFailed(String),

    LoggedIn,
    LoginFailed(String),

    None,
}
