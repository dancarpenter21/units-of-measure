//! Time interval units and conversions.

use crate::acceleration::Acceleration;
use crate::distance::Meters;
use crate::frequency::Frequency;
use crate::speed::{MetersPerSecond, Speed};

const SECONDS_PER_MINUTE: f64 = 60.0;
const SECONDS_PER_HOUR: f64 = 3_600.0;
const SECONDS_PER_DAY: f64 = 86_400.0;
const SECONDS_PER_WEEK: f64 = 604_800.0;
const DAYS_PER_MEAN_GREGORIAN_YEAR: f64 = 365.2425;
const SECONDS_PER_MEAN_GREGORIAN_YEAR: f64 = DAYS_PER_MEAN_GREGORIAN_YEAR * SECONDS_PER_DAY;
const SECONDS_PER_MEAN_GREGORIAN_MONTH: f64 = SECONDS_PER_MEAN_GREGORIAN_YEAR / 12.0;
const SECONDS_PER_JULIAN_YEAR: f64 = 365.25 * SECONDS_PER_DAY;

/// A time interval that can be converted into every supported time unit.
///
/// External types only need to implement [`Time::to_seconds`].
pub trait Time {
    /// Converts this interval to femtoseconds.
    fn to_femtoseconds(&self) -> Femtoseconds {
        Femtoseconds(self.to_seconds().0 / 1e-15)
    }

    /// Converts this interval to picoseconds.
    fn to_picoseconds(&self) -> Picoseconds {
        Picoseconds(self.to_seconds().0 / 1e-12)
    }

    /// Converts this interval to nanoseconds.
    fn to_nanoseconds(&self) -> Nanoseconds {
        Nanoseconds(self.to_seconds().0 / 1e-9)
    }

    /// Converts this interval to microseconds.
    fn to_microseconds(&self) -> Microseconds {
        Microseconds(self.to_seconds().0 / 1e-6)
    }

    /// Converts this interval to milliseconds.
    fn to_milliseconds(&self) -> Milliseconds {
        Milliseconds(self.to_seconds().0 / 1e-3)
    }

    /// Converts this interval to seconds.
    fn to_seconds(&self) -> Seconds;

    /// Converts this interval to minutes.
    fn to_minutes(&self) -> Minutes {
        Minutes(self.to_seconds().0 / SECONDS_PER_MINUTE)
    }

    /// Converts this interval to hours.
    fn to_hours(&self) -> Hours {
        Hours(self.to_seconds().0 / SECONDS_PER_HOUR)
    }

    /// Converts this interval to 24-hour days.
    fn to_days(&self) -> Days {
        Days(self.to_seconds().0 / SECONDS_PER_DAY)
    }

    /// Converts this interval to seven-day weeks.
    fn to_weeks(&self) -> Weeks {
        Weeks(self.to_seconds().0 / SECONDS_PER_WEEK)
    }

    /// Converts this interval to mean Gregorian months.
    ///
    /// A mean Gregorian month is one twelfth of 365.2425 days.
    fn to_mean_gregorian_months(&self) -> MeanGregorianMonths {
        MeanGregorianMonths(self.to_seconds().0 / SECONDS_PER_MEAN_GREGORIAN_MONTH)
    }

    /// Converts this interval to mean Gregorian years of 365.2425 days.
    fn to_mean_gregorian_years(&self) -> MeanGregorianYears {
        MeanGregorianYears(self.to_seconds().0 / SECONDS_PER_MEAN_GREGORIAN_YEAR)
    }

    /// Converts this interval to Julian years of exactly 365.25 days.
    fn to_julian_years(&self) -> JulianYears {
        JulianYears(self.to_seconds().0 / SECONDS_PER_JULIAN_YEAR)
    }

    /// Returns this interval divided by another interval.
    fn ratio(&self, rhs: &dyn Time) -> f64 {
        self.to_seconds().0 / rhs.to_seconds().0
    }

    /// Computes distance traveled at a constant speed.
    fn distance_at(&self, speed: &dyn Speed) -> Meters {
        Meters(self.to_seconds().0 * speed.to_meters_per_second().0)
    }

    /// Computes the speed change under constant acceleration.
    fn speed_change_at(&self, acceleration: &dyn Acceleration) -> MetersPerSecond {
        MetersPerSecond(self.to_seconds().0 * acceleration.to_meters_per_second_squared().0)
    }

    /// Computes the cycles completed at a frequency.
    fn cycles_at(&self, frequency: &dyn Frequency) -> f64 {
        self.to_seconds().0 * frequency.to_hertz().0
    }

    /// Returns whether the canonical value is finite.
    fn is_finite(&self) -> bool {
        self.to_seconds().0.is_finite()
    }
}

macro_rules! define_time_unit {
    ($name:ident, $method:ident, $symbol:literal, $factor:expr, $doc:literal) => {
        #[doc = $doc]
        #[derive(Clone, Copy, Debug, Default)]
        pub struct $name(pub f64);

        impl Time for $name {
            fn to_seconds(&self) -> Seconds {
                Seconds(self.0 * $factor)
            }
        }

        impl_unit_common!($name, Time, $method, $symbol);
    };
}

define_time_unit!(
    Femtoseconds,
    to_femtoseconds,
    "fs",
    1e-15,
    "A time interval measured in femtoseconds."
);
define_time_unit!(
    Picoseconds,
    to_picoseconds,
    "ps",
    1e-12,
    "A time interval measured in picoseconds."
);
define_time_unit!(
    Nanoseconds,
    to_nanoseconds,
    "ns",
    1e-9,
    "A time interval measured in nanoseconds."
);
define_time_unit!(
    Microseconds,
    to_microseconds,
    "µs",
    1e-6,
    "A time interval measured in microseconds."
);
define_time_unit!(
    Milliseconds,
    to_milliseconds,
    "ms",
    1e-3,
    "A time interval measured in milliseconds."
);
define_time_unit!(
    Seconds,
    to_seconds,
    "s",
    1.0,
    "A time interval measured in seconds."
);
define_time_unit!(
    Minutes,
    to_minutes,
    "min",
    SECONDS_PER_MINUTE,
    "A time interval measured in minutes."
);
define_time_unit!(
    Hours,
    to_hours,
    "h",
    SECONDS_PER_HOUR,
    "A time interval measured in hours."
);
define_time_unit!(
    Days,
    to_days,
    "d",
    SECONDS_PER_DAY,
    "A time interval measured in 24-hour days."
);
define_time_unit!(
    Weeks,
    to_weeks,
    "wk",
    SECONDS_PER_WEEK,
    "A time interval measured in seven-day weeks."
);
define_time_unit!(
    MeanGregorianMonths,
    to_mean_gregorian_months,
    "mean Gregorian mo",
    SECONDS_PER_MEAN_GREGORIAN_MONTH,
    "A time interval measured in mean Gregorian months."
);
define_time_unit!(
    MeanGregorianYears,
    to_mean_gregorian_years,
    "mean Gregorian yr",
    SECONDS_PER_MEAN_GREGORIAN_YEAR,
    "A time interval measured in mean Gregorian years."
);
define_time_unit!(
    JulianYears,
    to_julian_years,
    "a",
    SECONDS_PER_JULIAN_YEAR,
    "A time interval measured in Julian years of exactly 365.25 days."
);

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(actual: f64, expected: f64) {
        let scale = actual.abs().max(expected.abs()).max(1.0);
        assert!((actual - expected).abs() <= f64::EPSILON * 16.0 * scale);
    }

    #[test]
    fn subsecond_and_calendar_units_convert() {
        assert_close(Nanoseconds(1.0).to_seconds().0, 1e-9);
        assert_close(Hours(1.0).to_minutes().0, 60.0);
        assert_close(Weeks(1.0).to_days().0, 7.0);
        assert_close(MeanGregorianYears(1.0).to_days().0, 365.2425);
        assert_close(MeanGregorianMonths(12.0).to_mean_gregorian_years().0, 1.0);
        assert_close(JulianYears(1.0).to_days().0, 365.25);
    }

    #[test]
    fn mixed_arithmetic_preserves_left_unit() {
        assert_close((Minutes(1.0) + Seconds(30.0)).0, 1.5);
        assert_close((Seconds(30.0) + Minutes(1.0)).0, 90.0);
        assert!(Hours(1.0) > Minutes(59.0));
    }
}
