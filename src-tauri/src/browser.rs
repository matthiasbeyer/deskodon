#[tauri::command]
pub async fn open_browser(url: String) -> Result<(), String> {
    let url = url::Url::parse(&url).map_err(|e| e.to_string())?;
    open::that(url.as_ref()).map_err(|e| e.to_string())
}
