use std::path::PathBuf;

use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use deskodon_types::authorization_code::AuthorizationCode;

#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = invoke_configuration_file_path, catch)]
    async fn configuration_file_path() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = invoke_load_mastodon, catch)]
    async fn load_mastodon(configFile: String) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = invoke_register, catch)]
    async fn register(instanceUrl: String) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = invoke_finalize_registration, catch)]
    async fn finalize_registration(code: String) -> Result<JsValue, JsValue>;
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Tauri error: {:?}", .0)]
    Tauri(String),

    #[error(transparent)]
    Deser(#[from] serde_wasm_bindgen::Error),
}

pub async fn call_configuration_file_path() -> Result<PathBuf, Error> {
    log::debug!("calling: configuration_file_path()");
    configuration_file_path()
        .await
        .map_err(|jsval| Error::Tauri(format!("{:?}", jsval)))
        .and_then(|val| serde_wasm_bindgen::from_value(val).map_err(Error::from))
}

pub async fn call_load_mastodon(config_file: PathBuf) -> Result<(), Error> {
    log::debug!("calling: load_mastodon({})", config_file.display());
    load_mastodon(config_file.display().to_string())
        .await
        .map_err(|jsval| Error::Tauri(format!("{:?}", jsval)))
        .and_then(|val| serde_wasm_bindgen::from_value(val).map_err(Error::from))
}

pub async fn call_register(instance_url: url::Url) -> Result<String, Error> {
    log::debug!("calling: register({})", instance_url);
    register(instance_url.to_string())
        .await
        .map_err(|jsval| Error::Tauri(format!("{:?}", jsval)))
        .and_then(|val| serde_wasm_bindgen::from_value(val).map_err(Error::from))
}

pub async fn call_finalize_registration(code: AuthorizationCode) -> Result<(), Error> {
    log::debug!("calling: finalize_registration({})", code.as_ref());
    finalize_registration(code.as_ref().to_string())
        .await
        .map_err(|jsval| Error::Tauri(format!("{:?}", jsval)))
        .and_then(|val| serde_wasm_bindgen::from_value(val).map_err(Error::from))
}
