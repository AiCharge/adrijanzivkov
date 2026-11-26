//! Limits how often something can happen, like Laravel's `throttle:4,1` middleware and login lockout.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Remembers recent attempts per key (an IP address, or an email and IP for logins).
///
/// Requests run at the same time on several threads, so the map is shared through
/// `Arc<Mutex<…>>`: `Arc` ("atomically reference counted") lets many owners hold the same value, and
/// `Mutex` lets only one thread change it at a time. The compiler refuses to share a plain `HashMap`
/// across threads, which is how Rust rules out data races.
///
/// The counts live in memory, so they reset when the server restarts, and each server instance
/// counts on its own. That's enough here; a site with many instances would keep them in a database.
#[derive(Clone, Default)]
pub struct RateLimiter {
    attempts: Arc<Mutex<HashMap<String, Vec<Instant>>>>,
}

impl RateLimiter {
    /// Records an attempt. Returns `Err(wait)` with how long until the next try is allowed once
    /// `max` attempts happened within `window`.
    pub fn hit(&self, key: &str, max: usize, window: Duration) -> Result<(), Duration> {
        let now = Instant::now();
        // `lock()` waits for other threads to finish with the map. It fails only if a thread panicked
        // while holding it; `unwrap_or_else(…into_inner())` carries on with the data anyway.
        let mut attempts = self
            .attempts
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let times = attempts.entry(key.to_string()).or_default();

        times.retain(|time| now.duration_since(*time) < window);

        if times.len() >= max {
            let oldest = times[0];
            return Err(window.saturating_sub(now.duration_since(oldest)));
        }

        times.push(now);
        Ok(())
    }

    /// How long a key must wait, without recording an attempt. `None` when it may try now.
    pub fn too_many(&self, key: &str, max: usize, window: Duration) -> Option<Duration> {
        let now = Instant::now();
        let attempts = self
            .attempts
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let times = attempts.get(key)?;
        let recent: Vec<&Instant> = times
            .iter()
            .filter(|time| now.duration_since(**time) < window)
            .collect();

        (recent.len() >= max).then(|| window.saturating_sub(now.duration_since(*recent[0])))
    }

    /// Forgets a key's attempts, e.g. after a successful login.
    pub fn clear(&self, key: &str) {
        self.attempts
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .remove(key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allows_up_to_the_limit_then_refuses() {
        let limiter = RateLimiter::default();
        let minute = Duration::from_secs(60);

        for _ in 0..4 {
            assert!(limiter.hit("1.2.3.4", 4, minute).is_ok());
        }
        assert!(limiter.hit("1.2.3.4", 4, minute).is_err());
        assert!(
            limiter.hit("5.6.7.8", 4, minute).is_ok(),
            "other keys are counted separately"
        );
    }

    #[test]
    fn forgets_attempts_when_cleared() {
        let limiter = RateLimiter::default();
        let minute = Duration::from_secs(60);

        for _ in 0..5 {
            let _ = limiter.hit("login", 5, minute);
        }
        assert!(limiter.too_many("login", 5, minute).is_some());

        limiter.clear("login");
        assert!(limiter.too_many("login", 5, minute).is_none());
    }
}
