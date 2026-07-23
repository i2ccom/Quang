//! TURN — TURN server integration for WebRTC.
//!
//! Generates time-limited TURN credentials for Coturn or Cloudflare's
//! TURN service. Credentials are issued per-participant when they join
//! a room and are valid for the duration of the meeting.

use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};

type HmacSha256 = Hmac<Sha256>;

/// TURN credentials returned when joining a room.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TurnCredentials {
    pub urls: Vec<String>,
    pub username: String,
    pub credential: String,
}

/// Time-to-live for TURN credentials in seconds (default: 4 hours).
const TURN_TTL_SECS: u64 = 14400;

/// Generate TURN credentials for a participant joining a meeting.
///
/// Uses HMAC-SHA256 with a shared secret to create time-limited credentials
/// that the TURN server (Coturn) will accept.
///
/// ## Credential format (Coturn REST API)
///
/// ```text
/// username = <timestamp>:<participant_id>
/// password = base64(hmac-sha256(secret, username))
/// ```
pub async fn generate_turn_credentials() -> TurnCredentials {
    let participant_id = uuid::Uuid::new_v4().to_string();
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
        + TURN_TTL_SECS;

    let username = format!("{}:{}", timestamp, participant_id);

    // In production, read the TURN shared secret from env
    // let secret = env::var("TURN_SHARED_SECRET").unwrap_or_default();
    let secret = "quanghub-turn-secret-change-in-production";

    let credential = generate_hmac(secret, &username);

    TurnCredentials {
        urls: vec![
            "turn:turn.quanghub.app:3478?transport=udp".to_string(),
            "turn:turn.quanghub.app:3478?transport=tcp".to_string(),
            "stun:stun.quanghub.app:3478".to_string(),
        ],
        username,
        credential,
    }
}

/// Generate an HMAC-SHA256 hash, base64-encoded.
fn generate_hmac(secret: &str, data: &str) -> String {
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC key length is valid");
    mac.update(data.as_bytes());
    let result = mac.finalize();
    base64::encode(&result.into_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generate_turn_credentials() {
        let creds = generate_turn_credentials().await;
        assert!(!creds.urls.is_empty());
        assert!(!creds.username.is_empty());
        assert!(!creds.credential.is_empty());
        assert!(creds.urls[0].contains("turn:"));
    }

    #[test]
    fn test_generate_hmac() {
        let hash = generate_hmac("secret", "test-user");
        assert!(!hash.is_empty());
        // HMAC-SHA256 produces 32 bytes -> 44 base64 chars
        assert_eq!(hash.len(), 44);
    }
}
