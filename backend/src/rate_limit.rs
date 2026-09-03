use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

const MAX_FAILURES: u32 = 5;
const LOCK_DURATION: Duration = Duration::from_secs(15 * 60);
const MAX_TRACKED_KEYS: usize = 1000;

#[derive(Clone, Default)]
pub struct LoginLimiter {
    inner: Arc<Mutex<HashMap<String, LoginState>>>,
}

struct LoginState {
    failures: u32,
    locked_until: Option<Instant>,
}

impl LoginLimiter {
    pub fn is_locked(&self, username: &str) -> bool {
        let mut map = self.inner.lock().unwrap();
        match map.get(username) {
            Some(state) => match state.locked_until {
                Some(until) if until > Instant::now() => true,
                _ => {
                    map.remove(username);
                    false
                }
            },
            None => false,
        }
    }

    pub fn record_failure(&self, username: &str) {
        let mut map = self.inner.lock().unwrap();
        self.prune_if_needed(&mut map);

        let entry = map.entry(username.to_string()).or_insert(LoginState {
            failures: 0,
            locked_until: None,
        });

        if let Some(until) = entry.locked_until {
            if until <= Instant::now() {
                entry.failures = 0;
                entry.locked_until = None;
            }
        }

        entry.failures += 1;
        if entry.failures >= MAX_FAILURES {
            entry.locked_until = Some(Instant::now() + LOCK_DURATION);
        }
    }

    pub fn clear(&self, username: &str) {
        self.inner.lock().unwrap().remove(username);
    }

    fn prune_if_needed(&self, map: &mut HashMap<String, LoginState>) {
        if map.len() < MAX_TRACKED_KEYS {
            return;
        }
        let now = Instant::now();
        map.retain(|_, s| match s.locked_until {
            Some(until) => until > now,
            None => false,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn locks_after_max_failures() {
        let limiter = LoginLimiter::default();
        for _ in 0..MAX_FAILURES {
            limiter.record_failure("admin");
        }
        assert!(limiter.is_locked("admin"));
        assert!(!limiter.is_locked("other"));
    }

    #[test]
    fn not_locked_below_threshold() {
        let limiter = LoginLimiter::default();
        for _ in 0..MAX_FAILURES - 1 {
            limiter.record_failure("admin");
        }
        assert!(!limiter.is_locked("admin"));
    }

    #[test]
    fn clear_resets() {
        let limiter = LoginLimiter::default();
        for _ in 0..MAX_FAILURES {
            limiter.record_failure("admin");
        }
        assert!(limiter.is_locked("admin"));
        limiter.clear("admin");
        assert!(!limiter.is_locked("admin"));
    }
}
