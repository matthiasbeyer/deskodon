const invoke = window.__TAURI__.invoke

export async function invokeLogin(name) {
    return await invoke("login", {name: name});
}
