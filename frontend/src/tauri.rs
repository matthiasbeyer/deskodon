use deskodon_types::login::LoginHandle;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use deskodon_types::access_token::AccessToken;
use deskodon_types::auth::Auth;
use url::Url;

#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = invokeLogin, catch)]
    async fn login(name: String) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = invokeGenerateAuth, catch)]
    async fn generate_auth(instance: String) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = invokeFetchAccessToken, catch)]
    async fn fetch_access_token(
        instance: String,
        clientId: String,
        clientSecret: String,
        authToken: String,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = invokeOpenBrowser, catch)]
    async fn open_browser(
        url: String,
    ) -> Result<JsValue, JsValue>;
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Tauri error: {:?}", .0)]
    Tauri(String),

    #[error(transparent)]
    Deser(#[from] serde_wasm_bindgen::Error),
}

pub async fn call_login(name: String) -> Result<LoginHandle, Error> {
    login(name)
        .await
        .map_err(|jsval| Error::Tauri(format!("{:?}", jsval)))
        .and_then(|val| serde_wasm_bindgen::from_value::<LoginHandle>(val).map_err(Error::from))
}

pub async fn call_generate_auth(instance: url::Url) -> Result<Auth, Error> {
    match generate_auth(instance.to_string()).await {
        Err(e) => {
            log::error!("Error calling 'generate_auth()': {:?}", e);
            Err(Error::Tauri(format!("{:?}", e)))
        }
        Ok(val) => {
            serde_wasm_bindgen::from_value::<Auth>(val).map_err(Error::from)
        }
    }
}

pub async fn call_fetch_access_token(auth: Auth, auth_token: String) -> Result<AccessToken, Error> {
    fetch_access_token(
        auth.url.to_string(),
        auth.client_id,
        auth.client_secret,
        auth_token,
    )
    .await
    .map_err(|jsval| Error::Tauri(format!("{:?}", jsval)))
    .and_then(|val| serde_wasm_bindgen::from_value::<AccessToken>(val).map_err(Error::from))
}

pub async fn call_open_browser(url: url::Url) -> Result<(), Error> {
    open_browser(url.to_string())
        .await
    .map_err(|jsval| Error::Tauri(format!("{:?}", jsval)))
    .and_then(|val| serde_wasm_bindgen::from_value::<AccessToken>(val).map_err(Error::from))
}
