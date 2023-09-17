/// A command gets send from the frontend to the backend
#[derive(Debug)]
pub enum Command {
    State(AppState),

    QuitApp,
    PostText { text: String },
}

#[derive(Debug)]
pub enum AppState {
    CreatingDefaultConfig,
    LoadingConfig,
    LoadingConfigFailed,
    LoggedIn,
    LoginFailed,
}
