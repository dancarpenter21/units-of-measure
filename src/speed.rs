//! Speed units and conversions.

use crate::acceleration::{Acceleration, MetersPerSecondSquared};
use crate::distance::Meters;
use crate::time::{Seconds, Time};

const METERS_PER_SECOND_PER_MILE_PER_HOUR: f64 = 0.447_04;
const METERS_PER_SECOND_PER_KNOT: f64 = 1_852.0 / 3_600.0;
const STANDARD_SEA_LEVEL_SPEED_OF_SOUND: f64 = 340.294;

/// A speed that can be converted into every supported speed unit.
///
/// External types only need to implement [`Speed::to_meters_per_second`].
pub trait Speed {
    /// Converts this speed to centimeters per second.
    fn to_centimeters_per_second(&self) -> CentimetersPerSecond {
        CentimetersPerSecond(self.to_meters_per_second().0 / 0.01)
    }

    /// Converts this speed to meters per second.
    fn to_meters_per_second(&self) -> MetersPerSecond;

    /// Converts this speed to kilometers per second.
    fn to_kilometers_per_second(&self) -> KilometersPerSecond {
        KilometersPerSecond(self.to_meters_per_second().0 / 1_000.0)
    }

    /// Converts this speed to kilometers per hour.
    fn to_kilometers_per_hour(&self) -> KilometersPerHour {
        KilometersPerHour(self.to_meters_per_second().0 * 3.6)
    }

    /// Converts this speed to statute miles per hour.
    fn to_miles_per_hour(&self) -> MilesPerHour {
        MilesPerHour(self.to_meters_per_second().0 / METERS_PER_SECOND_PER_MILE_PER_HOUR)
    }

    /// Converts this speed to feet per second.
    fn to_feet_per_second(&self) -> FeetPerSecond {
        FeetPerSecond(self.to_meters_per_second().0 / 0.3048)
    }

    /// Converts this speed to feet per minute.
    fn to_feet_per_minute(&self) -> FeetPerMinute {
        FeetPerMinute(self.to_meters_per_second().0 / (0.3048 / 60.0))
    }

    /// Converts this speed to knots.
    fn to_knots(&self) -> Knots {
        Knots(self.to_meters_per_second().0 / METERS_PER_SECOND_PER_KNOT)
    }

    /// Converts this speed to Mach at standard sea-level conditions.
    ///
    /// This reference conversion uses a speed of sound of `340.294 m/s`.
    fn to_mach(&self) -> Mach {
        Mach(self.to_meters_per_second().0 / STANDARD_SEA_LEVEL_SPEED_OF_SOUND)
    }

    /// Returns this speed divided by another speed.
    fn ratio(&self, rhs: &dyn Speed) -> f64 {
        self.to_meters_per_second().0 / rhs.to_meters_per_second().0
    }

    /// Computes distance traveled over a time interval.
    fn distance_over(&self, time: &dyn Time) -> Meters {
        Meters(self.to_meters_per_second().0 * time.to_seconds().0)
    }

    /// Computes acceleration over a time interval.
    fn acceleration_over(&self, time: &dyn Time) -> MetersPerSecondSquared {
        MetersPerSecondSquared(self.to_meters_per_second().0 / time.to_seconds().0)
    }

    /// Computes the time needed for this speed change at an acceleration.
    fn time_at_acceleration(&self, acceleration: &dyn Acceleration) -> Seconds {
        Seconds(self.to_meters_per_second().0 / acceleration.to_meters_per_second_squared().0)
    }

    /// Returns whether the canonical value is finite.
    fn is_finite(&self) -> bool {
        self.to_meters_per_second().0.is_finite()
    }
}

macro_rules! define_speed_unit {
    ($name:ident, $method:ident, $symbol:literal, $factor:expr, $doc:literal) => {
        #[doc = $doc]
        #[derive(Clone, Copy, Debug, Default)]
        pub struct $name(pub f64);

        impl Speed for $name {
            fn to_meters_per_second(&self) -> MetersPerSecond {
                MetersPerSecond(self.0 * $factor)
            }
        }

        impl_unit_common!($name, Speed, $method, $symbol);
    };
}

define_speed_unit!(
    CentimetersPerSecond,
    to_centimeters_per_second,
    "cm/s",
    0.01,
    "A speed measured in centimeters per second."
);
define_speed_unit!(
    MetersPerSecond,
    to_meters_per_second,
    "m/s",
    1.0,
    "A speed measured in meters per second."
);
define_speed_unit!(
    KilometersPerSecond,
    to_kilometers_per_second,
    "km/s",
    1_000.0,
    "A speed measured in kilometers per second."
);
define_speed_unit!(
    KilometersPerHour,
    to_kilometers_per_hour,
    "km/h",
    1.0 / 3.6,
    "A speed measured in kilometers per hour."
);
define_speed_unit!(
    MilesPerHour,
    to_miles_per_hour,
    "mph",
    METERS_PER_SECOND_PER_MILE_PER_HOUR,
    "A speed measured in statute miles per hour."
);
define_speed_unit!(
    FeetPerSecond,
    to_feet_per_second,
    "ft/s",
    0.3048,
    "A speed measured in feet per second."
);
define_speed_unit!(
    FeetPerMinute,
    to_feet_per_minute,
    "ft/min",
    0.3048 / 60.0,
    "A speed measured in feet per minute."
);
define_speed_unit!(
    Knots,
    to_knots,
    "kn",
    METERS_PER_SECOND_PER_KNOT,
    "A speed measured in international nautical miles per hour."
);
define_speed_unit!(
    Mach,
    to_mach,
    "Mach",
    STANDARD_SEA_LEVEL_SPEED_OF_SOUND,
    "A Mach number using standard sea-level speed of sound."
);

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(actual: f64, expected: f64) {
        let scale = actual.abs().max(expected.abs()).max(1.0);
        assert!((actual - expected).abs() <= f64::EPSILON * 16.0 * scale);
    }

    #[test]
    fn reference_conversions_are_correct() {
        assert_close(KilometersPerHour(3.6).to_meters_per_second().0, 1.0);
        assert_close(MilesPerHour(60.0).to_feet_per_second().0, 88.0);
        assert_close(Knots(1.0).to_kilometers_per_hour().0, 1.852);
        assert_close(Mach(1.0).to_meters_per_second().0, 340.294);
        assert_close(FeetPerMinute(60.0).to_feet_per_second().0, 1.0);
    }

    #[test]
    fn mixed_arithmetic_preserves_left_unit() {
        assert_close(
            (MilesPerHour(60.0) + KilometersPerHour(10.0)).0,
            66.213_711_922_373_34,
        );
        assert!(KilometersPerHour(100.0) > MilesPerHour(60.0));
    }
}
