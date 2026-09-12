use crate::instant::Instant;
use std::sync::{Arc, Mutex, MutexGuard};
use time::OffsetDateTime;

pub trait Clock: Send + Sync {
    fn now(&self) -> Instant;
}

#[derive(Clone, Copy)]
pub struct SystemClock;

impl Clock for SystemClock {
    #[expect(clippy::disallowed_methods)]
    fn now(&self) -> Instant {
        Instant::from_utc(OffsetDateTime::now_utc())
    }
}

#[derive(Clone)]
pub struct FakeClock {
    now: Arc<Mutex<Instant>>,
}

impl FakeClock {
    pub fn new(now: Instant) -> Self {
        Self {
            now: Arc::new(Mutex::new(now)),
        }
    }

    pub fn set(&self, now: Instant) {
        *self.lock() = now;
    }

    pub fn now(&self) -> Instant {
        *self.lock()
    }

    fn lock(&self) -> MutexGuard<'_, Instant> {
        self.now
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }
}

impl Clock for FakeClock {
    fn now(&self) -> Instant {
        FakeClock::now(self)
    }
}

#[cfg(test)]
mod tests {
    use super::{Clock, FakeClock, SystemClock};
    use crate::instant::Instant;

    fn t0() -> Instant {
        Instant::from_unix_timestamp(1_700_000_000).unwrap()
    }

    #[test]
    fn fake_clock_starts_at_the_given_instant() {
        assert_eq!(FakeClock::new(t0()).now(), t0());
    }

    #[test]
    fn fake_clock_set_changes_now() {
        let clock = FakeClock::new(t0());
        let later = t0().checked_add_seconds(1).unwrap();
        clock.set(later);
        assert_eq!(clock.now(), later);
    }

    #[test]
    fn fake_clock_is_usable_as_clock() {
        let clock = FakeClock::new(t0());
        assert_eq!(Clock::now(&clock), t0());
    }

    #[test]
    fn fake_clock_clone_shares_now() {
        let clock = FakeClock::new(t0());
        let clone = clock.clone();
        let later = t0().checked_add_seconds(1).unwrap();
        clock.set(later);
        assert_eq!(clone.now(), later);
    }

    #[test]
    fn system_clock_is_a_clock() {
        fn assert_clock<C: Clock>() {}
        assert_clock::<SystemClock>();
    }
}
