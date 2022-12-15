const invoke = window.__TAURI_INVOKE__

export async function invokeLogin(name) {
    return await invoke("login", {name: name});
}

export async function invokeGenerateAuth(instanceUrl) {
    return await invoke("generate_auth", {instance: instanceUrl});
}

export async function invokeFetchAccessToken(instance, clientId, clientSecret, authToken) {
    return await invoke("fetch_access_token", {instance: instance, client_id: clientId, client_secret: clientSecret, auth_token: authToken});
}
