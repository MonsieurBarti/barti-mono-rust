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

    pub fn to_rfc3339_millis(self) -> String {
        let utc = self.0;
        format!(
            "{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}.{milli:03}Z",
            year = utc.year(),
            month = u8::from(utc.month()),
            day = utc.day(),
            hour = utc.hour(),
            minute = utc.minute(),
            second = utc.second(),
            milli = utc.millisecond(),
        )
    }

    pub(crate) fn from_utc(dt: OffsetDateTime) -> Self {
        Self(dt.to_offset(UtcOffset::UTC))
    }
}

#[cfg(test)]
mod tests {
    use super::Instant;
    use time::{Duration, OffsetDateTime};

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

    #[test]
    fn to_rfc3339_millis_is_utc_z_with_milliseconds() {
        let instant = Instant::from_unix_timestamp(1_700_000_000).unwrap();
        assert_eq!(instant.to_rfc3339_millis(), "2023-11-14T22:13:20.000Z");
    }

    #[test]
    fn to_rfc3339_millis_formats_nonzero_milliseconds() {
        let utc = OffsetDateTime::from_unix_timestamp(1_700_000_000).unwrap()
            + Duration::milliseconds(400);
        let instant = Instant::from_utc(utc);
        assert_eq!(instant.to_rfc3339_millis(), "2023-11-14T22:13:20.400Z");
    }
}
