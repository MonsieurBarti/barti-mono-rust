use time::{Duration, OffsetDateTime, UtcOffset};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Instant(OffsetDateTime);

impl Instant {
    pub fn from_unix_timestamp(seconds: i64) -> Option<Self> {
        OffsetDateTime::from_unix_timestamp(seconds)
            .ok()
            .map(Self::from_utc)
    }

    pub fn checked_add_seconds(self, seconds: i64) -> Option<Self> {
        self.0
            .checked_add(Duration::new(seconds, 0))
            .map(Self::from_utc)
    }

    pub fn unix_timestamp(self) -> i64 {
        self.0.unix_timestamp()
    }

    pub(crate) fn from_utc(dt: OffsetDateTime) -> Self {
        Self(dt.to_offset(UtcOffset::UTC))
    }
}

#[cfg(test)]
mod tests {
    use super::Instant;

    #[test]
    fn checked_add_seconds_shifts_forward_and_back() {
        let instant = Instant::from_unix_timestamp(1_700_000_000).unwrap();
        assert_eq!(
            instant.checked_add_seconds(60).unwrap(),
            Instant::from_unix_timestamp(1_700_000_060).unwrap()
        );
        assert_eq!(
            instant.checked_add_seconds(-1).unwrap(),
            Instant::from_unix_timestamp(1_699_999_999).unwrap()
        );
    }

    #[test]
    fn checked_add_seconds_rejects_overflow() {
        let instant = Instant::from_unix_timestamp(0).unwrap();
        assert!(instant.checked_add_seconds(i64::MAX).is_none());
    }

    #[test]
    fn unix_timestamp_round_trips_seconds() {
        let instant = Instant::from_unix_timestamp(1_700_000_000).unwrap();
        assert_eq!(instant.unix_timestamp(), 1_700_000_000);
    }
}
