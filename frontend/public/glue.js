const invoke = window.__TAURI_INVOKE__

export async function invoke_configuration_file_path() {
    return await invoke("configuration_file_path");
}

export async function invoke_load_mastodon(config_file) {
    return await invoke("load_mastodon", {config_file: config_file});
}

export async function invoke_register(instance_url) {
    return await invoke("register", {instance_url: instance_url});
}

export async function invoke_finalize_registration(code) {
    return await invoke("finalize_registration", {code: code});
}
