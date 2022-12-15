#[derive(Debug)]
pub enum Message {
    Authenticate,
    InstanceUrlInvalid(String, String),
    AuthSuccess(deskodon_types::auth::Auth),
    AuthErr(String),

    Login,
}

