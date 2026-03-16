use std::sync::Mutex;
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::core::config::PlayFabConfig;

/// Refresh buffer — tokens are refreshed 5 minutes before actual expiry.
const REFRESH_BUFFER: Duration = Duration::from_secs(300);

/// Default TTL when the server does not provide an expiration timestamp.
const DEFAULT_TTL: Duration = Duration::from_secs(24 * 60 * 60); // 24 hours

// ---------------------------------------------------------------------------
// Response types
// ---------------------------------------------------------------------------

/// Wrapper for the PlayFab Authentication/GetEntityToken response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetEntityTokenResponse {
    pub code: Option<u16>,
    pub status: Option<String>,
    pub data: Option<EntityTokenData>,
}

/// The `data` portion of the GetEntityToken response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityTokenData {
    #[serde(rename = "EntityToken")]
    pub entity_token: String,
    #[serde(rename = "TokenExpiration")]
    pub token_expiration: Option<String>,
}

// ---------------------------------------------------------------------------
// Cached token
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
struct CachedToken {
    token: String,
    acquired_at: Instant,
    ttl: Duration,
}

// ---------------------------------------------------------------------------
// AuthManager
// ---------------------------------------------------------------------------

/// Manages PlayFab Entity Token acquisition, caching, and automatic refresh.
#[derive(Debug)]
pub struct AuthManager {
    config: PlayFabConfig,
    cached_token: Mutex<Option<CachedToken>>,
}

impl AuthManager {
    /// Create a new `AuthManager` backed by the given PlayFab configuration.
    pub fn new(config: PlayFabConfig) -> Self {
        Self {
            config,
            cached_token: Mutex::new(None),
        }
    }

    /// Return a valid entity token, refreshing transparently when the cached
    /// token is missing or within [`REFRESH_BUFFER`] of expiry.
    pub fn get_entity_token(&self) -> Result<String> {
        let guard = self
            .cached_token
            .lock()
            .unwrap_or_else(|poison| poison.into_inner());

        if let Some(ref cached) = *guard {
            if Self::is_token_valid(cached) {
                return Ok(cached.token.clone());
            }
        }
        // Drop the lock before making the network call.
        drop(guard);

        self.refresh_token()
    }

    /// Force-refresh the entity token by calling the PlayFab Authentication API.
    ///
    /// The new token is stored in the internal cache and also returned.
    pub fn refresh_token(&self) -> Result<String> {
        let url = self.config.api_url("Authentication", "GetEntityToken");

        let response: GetEntityTokenResponse = ureq::post(&url)
            .set("X-SecretKey", &self.config.dev_secret_key)
            .set("Content-Type", "application/json")
            .send_json(serde_json::json!({}))
            .context("Failed to call Authentication/GetEntityToken")?
            .into_json()
            .context("Failed to parse GetEntityToken response")?;

        let data = response
            .data
            .context("GetEntityToken response is missing 'data' field")?;

        let ttl = parse_ttl(data.token_expiration.as_deref());

        let cached = CachedToken {
            token: data.entity_token.clone(),
            acquired_at: Instant::now(),
            ttl,
        };

        let mut guard = self
            .cached_token
            .lock()
            .unwrap_or_else(|poison| poison.into_inner());
        *guard = Some(cached);

        Ok(data.entity_token)
    }

    /// A token is considered valid if it has not yet entered the refresh buffer
    /// window (i.e., it still has more than [`REFRESH_BUFFER`] of remaining lifetime).
    fn is_token_valid(cached: &CachedToken) -> bool {
        cached.acquired_at.elapsed() < cached.ttl.saturating_sub(REFRESH_BUFFER)
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Parse the `TokenExpiration` ISO-8601 timestamp to compute a TTL from *now*.
/// Falls back to [`DEFAULT_TTL`] when the expiration is absent or unparseable.
fn parse_ttl(expiration: Option<&str>) -> Duration {
    let Some(expiration) = expiration else {
        return DEFAULT_TTL;
    };

    // PlayFab returns timestamps like "2025-01-15T12:00:00.000Z".
    // We do a lightweight parse without pulling in chrono — good enough for a TTL.
    parse_iso8601_to_duration_from_now(expiration).unwrap_or(DEFAULT_TTL)
}

/// Very simple ISO-8601 parser that computes seconds-from-now.
/// Returns `None` when the string cannot be parsed or the expiry is in the past.
fn parse_iso8601_to_duration_from_now(iso: &str) -> Option<Duration> {
    // Expected format: "2025-01-15T12:34:56Z" or "2025-01-15T12:34:56.123Z"
    // We rely on the system clock being approximately correct.
    use std::time::SystemTime;

    // Strip fractional seconds and trailing 'Z' for simplicity.
    let cleaned = iso.trim_end_matches('Z');
    let main_part = cleaned.split('.').next()?;

    let parts: Vec<&str> = main_part.split('T').collect();
    if parts.len() != 2 {
        return None;
    }

    let date_parts: Vec<u64> = parts[0].split('-').filter_map(|s| s.parse().ok()).collect();
    let time_parts: Vec<u64> = parts[1].split(':').filter_map(|s| s.parse().ok()).collect();

    if date_parts.len() != 3 || time_parts.len() != 3 {
        return None;
    }

    let (year, month, day) = (date_parts[0], date_parts[1], date_parts[2]);
    let (hour, minute, second) = (time_parts[0], time_parts[1], time_parts[2]);

    // Convert to a rough Unix timestamp using the same algorithm as mktime.
    let epoch_secs = simple_utc_to_epoch(year, month, day, hour, minute, second)?;

    let now_epoch = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .ok()?
        .as_secs();

    if epoch_secs <= now_epoch {
        return None; // already expired
    }

    Some(Duration::from_secs(epoch_secs - now_epoch))
}

/// Convert a UTC date-time to an approximate Unix epoch (seconds).
fn simple_utc_to_epoch(year: u64, month: u64, day: u64, h: u64, m: u64, s: u64) -> Option<u64> {
    if year < 1970 || !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }

    // Days from 1970-01-01 to the start of the given year.
    let mut days: u64 = 0;
    for y in 1970..year {
        days += if is_leap(y) { 366 } else { 365 };
    }

    let month_days: [u64; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    for (mi, &md) in month_days.iter().enumerate().take((month - 1) as usize) {
        days += md;
        if mi == 1 && is_leap(year) {
            days += 1;
        }
    }

    days += day - 1;

    Some(days * 86400 + h * 3600 + m * 60 + s)
}

fn is_leap(y: u64) -> bool {
    y.is_multiple_of(4) && (!y.is_multiple_of(100) || y.is_multiple_of(400))
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_token_valid_returns_true_for_fresh_token() {
        let cached = CachedToken {
            token: "tok".to_string(),
            acquired_at: Instant::now(),
            ttl: Duration::from_secs(3600),
        };
        assert!(AuthManager::is_token_valid(&cached));
    }

    #[test]
    fn is_token_valid_returns_false_when_near_expiry() {
        let cached = CachedToken {
            token: "tok".to_string(),
            // Simulate a token acquired 55 minutes ago with a 1-hour TTL.
            acquired_at: Instant::now() - Duration::from_secs(55 * 60),
            ttl: Duration::from_secs(3600),
        };
        assert!(!AuthManager::is_token_valid(&cached));
    }

    #[test]
    fn is_token_valid_returns_false_when_expired() {
        let cached = CachedToken {
            token: "tok".to_string(),
            acquired_at: Instant::now() - Duration::from_secs(7200),
            ttl: Duration::from_secs(3600),
        };
        assert!(!AuthManager::is_token_valid(&cached));
    }

    #[test]
    fn parse_ttl_returns_default_for_none() {
        assert_eq!(parse_ttl(None), DEFAULT_TTL);
    }

    #[test]
    fn parse_ttl_returns_default_for_garbage() {
        assert_eq!(parse_ttl(Some("not-a-date")), DEFAULT_TTL);
    }

    #[test]
    fn simple_utc_to_epoch_known_value() {
        // 2020-01-01T00:00:00Z == 1577836800
        let secs = simple_utc_to_epoch(2020, 1, 1, 0, 0, 0).unwrap();
        assert_eq!(secs, 1_577_836_800);
    }

    #[test]
    fn auth_manager_new_has_no_cached_token() {
        let config = PlayFabConfig {
            title_id: "TEST".to_string(),
            dev_secret_key: "secret".to_string(),
            api_endpoint: "https://TEST.playfabapi.com".to_string(),
        };
        let manager = AuthManager::new(config);
        let guard = manager.cached_token.lock().unwrap();
        assert!(guard.is_none());
    }
}
