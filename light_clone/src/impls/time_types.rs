//! LightClone implementations for the `time` crate's date/time types.
//!
//! These implementations are behind the `time` feature flag.
//!
//! All time crate date/time types are `Copy`, so cloning is always O(1).

use crate::LightClone;

/// Macro to implement LightClone for Copy types.
macro_rules! impl_light_clone_for_copy {
    ($($t:ty),* $(,)?) => {
        $(
            impl LightClone for $t {
                #[inline]
                fn light_clone(&self) -> Self {
                    *self
                }
            }
        )*
    };
}

impl_light_clone_for_copy!(
    time::Date,
    time::Time,
    time::PrimitiveDateTime,
    time::OffsetDateTime,
    time::UtcOffset,
    time::Duration,
    time::Month,
    time::Weekday,
);

#[cfg(test)]
mod tests {
    use super::*;
    use time::{
        Date, Duration, Month, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset, Weekday,
    };

    #[test]
    fn date_implements_light_clone() {
        let date = Date::from_calendar_date(2024, Month::January, 15).unwrap();
        let cloned = date.light_clone();
        assert_eq!(date, cloned);
    }

    #[test]
    fn time_implements_light_clone() {
        let time = Time::from_hms(12, 30, 45).unwrap();
        let cloned = time.light_clone();
        assert_eq!(time, cloned);
    }

    #[test]
    fn primitive_datetime_implements_light_clone() {
        let date = Date::from_calendar_date(2024, Month::January, 15).unwrap();
        let time = Time::from_hms(12, 30, 45).unwrap();
        let dt = PrimitiveDateTime::new(date, time);
        let cloned = dt.light_clone();
        assert_eq!(dt, cloned);
    }

    #[test]
    fn offset_datetime_implements_light_clone() {
        let dt = OffsetDateTime::now_utc();
        let cloned = dt.light_clone();
        assert_eq!(dt, cloned);
    }

    #[test]
    fn utc_offset_implements_light_clone() {
        let offset = UtcOffset::from_hms(5, 30, 0).unwrap();
        let cloned = offset.light_clone();
        assert_eq!(offset, cloned);
    }

    #[test]
    fn duration_implements_light_clone() {
        let duration = Duration::seconds(3600);
        let cloned = duration.light_clone();
        assert_eq!(duration, cloned);
    }

    #[test]
    fn month_implements_light_clone() {
        let month = Month::January;
        let cloned = month.light_clone();
        assert_eq!(month, cloned);
    }

    #[test]
    fn weekday_implements_light_clone() {
        let weekday = Weekday::Monday;
        let cloned = weekday.light_clone();
        assert_eq!(weekday, cloned);
    }
}
