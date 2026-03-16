use anyhow::{anyhow, Context, Result};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use std::time::Duration;
use ureq::Agent;

use crate::http::retry::{is_retryable_status, sleep_before_retry, RetryTier};

const DEFAULT_TIMEOUT_SECS: u64 = 30;
const USER_AGENT_VALUE: &str = "playfab-cli";

pub struct PlayFabClient {
    agent: Agent,
}

impl Default for PlayFabClient {
    fn default() -> Self {
        Self::new()
    }
}

impl PlayFabClient {
    pub fn new() -> Self {
        let agent = ureq::AgentBuilder::new()
            .timeout(Duration::from_secs(DEFAULT_TIMEOUT_SECS))
            .build();
        Self { agent }
    }

    /// Execute a PlayFab API call with retry logic
    pub fn call<T: Serialize, R: DeserializeOwned>(
        &self,
        url: &str,
        body: &T,
        auth_header: (&str, &str), // ("X-SecretKey", key) or ("X-EntityToken", token)
        tier: RetryTier,
    ) -> Result<R> {
        let json_body = serde_json::to_string(body)?;
        let mut last_error = None;

        for attempt in 1..=tier.max_attempts() {
            let result = self
                .agent
                .post(url)
                .set("Content-Type", "application/json")
                .set("User-Agent", USER_AGENT_VALUE)
                .set(auth_header.0, auth_header.1)
                .send_string(&json_body);

            match result {
                Ok(response) => {
                    return response
                        .into_json::<R>()
                        .context("Failed to parse PlayFab API response");
                }
                Err(ureq::Error::Status(status, response)) => {
                    let body = response.into_string().unwrap_or_default();
                    let error = anyhow!(
                        "PlayFab API error: HTTP {} for {} - {}",
                        status,
                        url,
                        body
                    );
                    if attempt < tier.max_attempts() && is_retryable_status(status) {
                        tracing::warn!(
                            "Retryable error (attempt {}/{}): {}",
                            attempt,
                            tier.max_attempts(),
                            error
                        );
                        last_error = Some(error);
                        sleep_before_retry(tier, attempt);
                        continue;
                    }
                    return Err(error);
                }
                Err(ureq::Error::Transport(e)) => {
                    let error = anyhow!("PlayFab API transport error for {}: {}", url, e);
                    if attempt < tier.max_attempts() {
                        tracing::warn!(
                            "Transport error (attempt {}/{}): {}",
                            attempt,
                            tier.max_attempts(),
                            error
                        );
                        last_error = Some(error);
                        sleep_before_retry(tier, attempt);
                        continue;
                    }
                    return Err(error);
                }
            }
        }

        Err(last_error.unwrap_or_else(|| anyhow!("Request failed after all retries: {}", url)))
    }

    /// Convenience: call with raw JSON Value
    pub fn call_raw(
        &self,
        url: &str,
        body: &Value,
        auth_header: (&str, &str),
        tier: RetryTier,
    ) -> Result<Value> {
        self.call(url, body, auth_header, tier)
    }
}
