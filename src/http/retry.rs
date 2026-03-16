use std::thread;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetryTier {
    /// Critical operations: 1 retry, 500ms base delay
    Strict,
    /// Normal operations: 3 retries, 250ms base delay
    Standard,
    /// Bulk/batch operations: 5 retries, 1s base delay
    Bulk,
}

impl RetryTier {
    pub fn max_attempts(self) -> usize {
        match self {
            Self::Strict => 2,   // 1 + 1 retry
            Self::Standard => 4, // 1 + 3 retries
            Self::Bulk => 6,     // 1 + 5 retries
        }
    }

    pub fn base_delay_ms(self) -> u64 {
        match self {
            Self::Strict => 500,
            Self::Standard => 250,
            Self::Bulk => 1000,
        }
    }
}

pub fn is_retryable_status(status: u16) -> bool {
    status == 408 || status == 429 || (500..=599).contains(&status)
}

pub fn sleep_before_retry(tier: RetryTier, attempt: usize) {
    let backoff = 1u64 << attempt.saturating_sub(1);
    thread::sleep(Duration::from_millis(tier.base_delay_ms() * backoff));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strict_max_attempts() {
        assert_eq!(RetryTier::Strict.max_attempts(), 2);
    }

    #[test]
    fn standard_max_attempts() {
        assert_eq!(RetryTier::Standard.max_attempts(), 4);
    }

    #[test]
    fn bulk_max_attempts() {
        assert_eq!(RetryTier::Bulk.max_attempts(), 6);
    }

    #[test]
    fn strict_base_delay() {
        assert_eq!(RetryTier::Strict.base_delay_ms(), 500);
    }

    #[test]
    fn standard_base_delay() {
        assert_eq!(RetryTier::Standard.base_delay_ms(), 250);
    }

    #[test]
    fn bulk_base_delay() {
        assert_eq!(RetryTier::Bulk.base_delay_ms(), 1000);
    }

    #[test]
    fn retryable_status_408() {
        assert!(is_retryable_status(408));
    }

    #[test]
    fn retryable_status_429() {
        assert!(is_retryable_status(429));
    }

    #[test]
    fn retryable_status_500() {
        assert!(is_retryable_status(500));
    }

    #[test]
    fn retryable_status_502() {
        assert!(is_retryable_status(502));
    }

    #[test]
    fn retryable_status_503() {
        assert!(is_retryable_status(503));
    }

    #[test]
    fn retryable_status_599() {
        assert!(is_retryable_status(599));
    }

    #[test]
    fn non_retryable_status_200() {
        assert!(!is_retryable_status(200));
    }

    #[test]
    fn non_retryable_status_400() {
        assert!(!is_retryable_status(400));
    }

    #[test]
    fn non_retryable_status_401() {
        assert!(!is_retryable_status(401));
    }

    #[test]
    fn non_retryable_status_403() {
        assert!(!is_retryable_status(403));
    }

    #[test]
    fn non_retryable_status_404() {
        assert!(!is_retryable_status(404));
    }

    #[test]
    fn tier_equality() {
        assert_eq!(RetryTier::Strict, RetryTier::Strict);
        assert_ne!(RetryTier::Strict, RetryTier::Standard);
        assert_ne!(RetryTier::Standard, RetryTier::Bulk);
    }
}
