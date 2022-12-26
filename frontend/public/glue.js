const invoke = window.__TAURI_INVOKE__

export async function invoke_configuration_file_path() {
    return await invoke("configuration_file_path");
}

export async function invoke_load_mastodon(configFile) {
    return await invoke("load_mastodon", {configFile: configFile});
}

export async function invoke_register(instanceUrl) {
    return await invoke("register", {instanceUrl: instanceUrl});
}

export async function invoke_finalize_registration(code) {
    return await invoke("finalize_registration", {code: code});
}
