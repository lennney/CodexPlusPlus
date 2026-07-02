use std::sync::OnceLock;

/// Get or create a globally cached `reqwest::Client`.
///
/// The client is lazily initialized on the first call and reused for all subsequent
/// requests. The `user_agent` parameter is only consulted on the first call; after
/// that the cached client is returned regardless.
pub fn proxied_client(user_agent: &str) -> anyhow::Result<reqwest::Client> {
    static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

    // Fast path: cache hit
    if let Some(client) = CLIENT.get() {
        return Ok(client.clone());
    }

    // Cache miss: build the client (may fail on TLS init)
    let ua = if user_agent.trim().is_empty() {
        format!("CodexPlusPlus/{}", env!("CARGO_PKG_VERSION"))
    } else {
        user_agent.trim().to_string()
    };
    let client = reqwest::Client::builder().user_agent(ua).build()?;

    // get_or_init handles the race: if another thread beat us, use theirs
    Ok(CLIENT.get_or_init(|| client).clone())
}
