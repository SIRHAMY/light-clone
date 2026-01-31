//! LightClone implementations for the `chrono` crate's date/time types.
//!
//! These implementations are behind the `chrono` feature flag.
//!
//! All chrono date/time types are `Copy`, so cloning is always O(1).

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
    chrono::NaiveDate,
    chrono::NaiveTime,
    chrono::NaiveDateTime,
    chrono::Month,
    chrono::Weekday,
    chrono::TimeDelta,
    chrono::Utc,
    chrono::FixedOffset,
);

// DateTime<Tz> is Copy when Tz is Copy
impl<Tz: chrono::TimeZone + Copy> LightClone for chrono::DateTime<Tz>
where
    Tz::Offset: Copy,
{
    #[inline]
    fn light_clone(&self) -> Self {
        *self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};

    #[test]
    fn naive_date_implements_light_clone() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();
        let cloned = date.light_clone();
        assert_eq!(date, cloned);
    }

    #[test]
    fn naive_time_implements_light_clone() {
        let time = NaiveTime::from_hms_opt(12, 30, 45).unwrap();
        let cloned = time.light_clone();
        assert_eq!(time, cloned);
    }

    #[test]
    fn naive_datetime_implements_light_clone() {
        let dt = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            NaiveTime::from_hms_opt(12, 30, 45).unwrap(),
        );
        let cloned = dt.light_clone();
        assert_eq!(dt, cloned);
    }

    #[test]
    fn datetime_utc_implements_light_clone() {
        let dt: DateTime<Utc> = Utc::now();
        let cloned = dt.light_clone();
        assert_eq!(dt, cloned);
    }

    #[test]
    fn datetime_fixed_offset_implements_light_clone() {
        let offset = FixedOffset::east_opt(5 * 3600).unwrap();
        let dt: DateTime<FixedOffset> = offset.with_ymd_and_hms(2024, 1, 15, 12, 30, 45).unwrap();
        let cloned = dt.light_clone();
        assert_eq!(dt, cloned);
    }

    #[test]
    fn month_implements_light_clone() {
        let month = chrono::Month::January;
        let cloned = month.light_clone();
        assert_eq!(month, cloned);
    }

    #[test]
    fn weekday_implements_light_clone() {
        let weekday = chrono::Weekday::Mon;
        let cloned = weekday.light_clone();
        assert_eq!(weekday, cloned);
    }
}
