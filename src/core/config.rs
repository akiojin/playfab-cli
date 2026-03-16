use std::env;

use anyhow::{bail, Result};

const DEFAULT_API_ENDPOINT: &str = "https://{title_id}.playfabapi.com";

/// PlayFab service configuration loaded from environment variables.
#[derive(Debug, Clone)]
pub struct PlayFabConfig {
    pub title_id: String,
    pub dev_secret_key: String,
    pub api_endpoint: String,
}

impl PlayFabConfig {
    /// Build configuration from environment variables.
    ///
    /// Required:
    /// - `PLAYFAB_TITLE_ID`
    /// - `PLAYFAB_DEV_SECRET_KEY`
    ///
    /// Optional:
    /// - `PLAYFAB_API_ENDPOINT` — overrides the default `https://{title_id}.playfabapi.com`
    pub fn from_env() -> Result<Self> {
        let title_id = read_required_env("PLAYFAB_TITLE_ID")?;
        let dev_secret_key = read_required_env("PLAYFAB_DEV_SECRET_KEY")?;
        let api_endpoint = read_env(&["PLAYFAB_API_ENDPOINT"])
            .unwrap_or_else(|| DEFAULT_API_ENDPOINT.replace("{title_id}", &title_id));

        Ok(Self {
            title_id,
            dev_secret_key,
            api_endpoint,
        })
    }

    /// Build a full URL for a PlayFab REST API call.
    ///
    /// Example: `config.api_url("Authentication", "GetEntityToken")`
    /// yields `https://ABCDE.playfabapi.com/Authentication/GetEntityToken`
    pub fn api_url(&self, api_group: &str, method: &str) -> String {
        format!("{}/{}/{}", self.api_endpoint, api_group, method)
    }
}

// ---------------------------------------------------------------------------
// Environment helpers (following unity-cli patterns)
// ---------------------------------------------------------------------------

/// Read the first non-empty value from the given environment variable keys.
pub fn read_env(keys: &[&str]) -> Option<String> {
    for key in keys {
        if let Ok(value) = env::var(key) {
            let trimmed = value.trim().to_string();
            if !trimmed.is_empty() {
                return Some(trimmed);
            }
        }
    }
    None
}

/// Read a required environment variable, returning an error when missing or empty.
pub fn read_required_env(key: &str) -> Result<String> {
    match read_env(&[key]) {
        Some(value) => Ok(value),
        None => bail!(
            "Required environment variable '{}' is not set or empty",
            key
        ),
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper that sets env vars for the duration of a closure, then cleans up.
    fn with_env_vars<F, R>(vars: &[(&str, &str)], f: F) -> R
    where
        F: FnOnce() -> R,
    {
        let _lock = crate::test_env::env_lock()
            .lock()
            .unwrap_or_else(|poison| poison.into_inner());
        for (key, value) in vars {
            env::set_var(key, value);
        }
        let result = f();
        for (key, _) in vars {
            env::remove_var(key);
        }
        result
    }

    /// Helper that ensures specific env vars are removed for the duration of a closure.
    fn without_env_vars<F, R>(keys: &[&str], f: F) -> R
    where
        F: FnOnce() -> R,
    {
        let _lock = crate::test_env::env_lock()
            .lock()
            .unwrap_or_else(|poison| poison.into_inner());
        for key in keys {
            env::remove_var(key);
        }
        f()
    }

    #[test]
    fn from_env_succeeds_with_both_vars_set() {
        with_env_vars(
            &[
                ("PLAYFAB_TITLE_ID", "ABCDE"),
                ("PLAYFAB_DEV_SECRET_KEY", "secret123"),
            ],
            || {
                let config = PlayFabConfig::from_env().expect("should succeed");
                assert_eq!(config.title_id, "ABCDE");
                assert_eq!(config.dev_secret_key, "secret123");
                assert_eq!(config.api_endpoint, "https://ABCDE.playfabapi.com");
            },
        );
    }

    #[test]
    fn from_env_fails_when_title_id_missing() {
        without_env_vars(
            &[
                "PLAYFAB_TITLE_ID",
                "PLAYFAB_DEV_SECRET_KEY",
                "PLAYFAB_API_ENDPOINT",
            ],
            || {
                env::set_var("PLAYFAB_DEV_SECRET_KEY", "secret123");
                let err = PlayFabConfig::from_env().expect_err("should fail");
                assert!(err.to_string().contains("PLAYFAB_TITLE_ID"));
                env::remove_var("PLAYFAB_DEV_SECRET_KEY");
            },
        );
    }

    #[test]
    fn from_env_fails_when_secret_key_missing() {
        without_env_vars(
            &[
                "PLAYFAB_TITLE_ID",
                "PLAYFAB_DEV_SECRET_KEY",
                "PLAYFAB_API_ENDPOINT",
            ],
            || {
                env::set_var("PLAYFAB_TITLE_ID", "ABCDE");
                let err = PlayFabConfig::from_env().expect_err("should fail");
                assert!(err.to_string().contains("PLAYFAB_DEV_SECRET_KEY"));
                env::remove_var("PLAYFAB_TITLE_ID");
            },
        );
    }

    #[test]
    fn api_url_formats_correctly() {
        let config = PlayFabConfig {
            title_id: "ABCDE".to_string(),
            dev_secret_key: "secret".to_string(),
            api_endpoint: "https://ABCDE.playfabapi.com".to_string(),
        };
        assert_eq!(
            config.api_url("Authentication", "GetEntityToken"),
            "https://ABCDE.playfabapi.com/Authentication/GetEntityToken"
        );
    }

    #[test]
    fn custom_api_endpoint_override_works() {
        with_env_vars(
            &[
                ("PLAYFAB_TITLE_ID", "ABCDE"),
                ("PLAYFAB_DEV_SECRET_KEY", "secret123"),
                ("PLAYFAB_API_ENDPOINT", "https://custom.example.com"),
            ],
            || {
                let config = PlayFabConfig::from_env().expect("should succeed");
                assert_eq!(config.api_endpoint, "https://custom.example.com");
                assert_eq!(
                    config.api_url("Economy", "SearchItems"),
                    "https://custom.example.com/Economy/SearchItems"
                );
            },
        );
    }

    #[test]
    fn read_env_returns_none_when_not_set() {
        without_env_vars(&["PLAYFAB_NONEXISTENT"], || {
            assert!(read_env(&["PLAYFAB_NONEXISTENT"]).is_none());
        });
    }

    #[test]
    fn read_env_ignores_empty_values() {
        with_env_vars(&[("PLAYFAB_EMPTY_TEST", "  ")], || {
            assert!(read_env(&["PLAYFAB_EMPTY_TEST"]).is_none());
        });
    }

    #[test]
    fn read_required_env_fails_when_missing() {
        without_env_vars(&["PLAYFAB_MISSING_KEY"], || {
            let err = read_required_env("PLAYFAB_MISSING_KEY").expect_err("should fail");
            assert!(err.to_string().contains("PLAYFAB_MISSING_KEY"));
        });
    }
}
