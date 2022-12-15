const invoke = window.__TAURI_INVOKE__

export async function invokeLogin(name) {
    return await invoke("login", {name: name});
}
