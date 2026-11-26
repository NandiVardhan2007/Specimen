use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// Per-host rate limiter ensuring requests to the same domain adhere to `rate_limit_rps`.
#[derive(Debug)]
pub struct RateLimiter {
    min_interval: Duration,
    last_requests: Mutex<HashMap<String, Instant>>,
}

impl RateLimiter {
    pub fn new(requests_per_second: f64) -> Self {
        let rps = if requests_per_second <= 0.0 {
            1.0
        } else {
            requests_per_second
        };
        let interval_secs = 1.0 / rps;
        RateLimiter {
            min_interval: Duration::from_secs_f64(interval_secs),
            last_requests: Mutex::new(HashMap::new()),
        }
    }

    /// Calculates how long the caller must wait before sending a request to `host`.
    /// Updates the internal timestamp for that host.
    pub fn acquire(&self, host: &str) -> Duration {
        let mut map = self.last_requests.lock().unwrap();
        let now = Instant::now();

        if let Some(&last) = map.get(host) {
            let elapsed = now.duration_since(last);
            if elapsed < self.min_interval {
                let wait = self.min_interval - elapsed;
                map.insert(host.to_string(), now + wait);
                return wait;
            }
        }

        map.insert(host.to_string(), now);
        Duration::ZERO
    }

    /// Asynchronously waits until a request is permitted for `host`.
    pub async fn wait_turn(&self, host: &str) {
        let wait = self.acquire(host);
        if !wait.is_zero() {
            tokio::time::sleep(wait).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limiter_interval() {
        let limiter = RateLimiter::new(2.0); // 500ms interval
        let wait1 = limiter.acquire("example.com");
        assert_eq!(wait1, Duration::ZERO);

        let wait2 = limiter.acquire("example.com");
        assert!(wait2 > Duration::from_millis(400));
        assert!(wait2 <= Duration::from_millis(500));

        // Different host should not be throttled
        let wait_other = limiter.acquire("other.org");
        assert_eq!(wait_other, Duration::ZERO);
    }
}
