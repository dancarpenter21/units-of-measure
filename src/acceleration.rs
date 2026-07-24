//! Acceleration units and conversions.

use crate::mass::Mass;
use crate::speed::MetersPerSecond;
use crate::time::Time;
use crate::weight::Newtons;

/// Conventional standard gravity in meters per second squared.
pub const STANDARD_GRAVITY: MetersPerSecondSquared = MetersPerSecondSquared(9.806_65);

/// The numeric conventional standard gravity in meters per second squared.
pub const STANDARD_GRAVITY_METERS_PER_SECOND_SQUARED: f64 = 9.806_65;

/// An acceleration that can be converted into every supported acceleration unit.
///
/// External types only need to implement
/// [`Acceleration::to_meters_per_second_squared`].
pub trait Acceleration {
    /// Converts this acceleration to meters per second squared.
    fn to_meters_per_second_squared(&self) -> MetersPerSecondSquared;

    /// Converts this acceleration to centimeters per second squared.
    fn to_centimeters_per_second_squared(&self) -> CentimetersPerSecondSquared {
        CentimetersPerSecondSquared(self.to_meters_per_second_squared().0 / 0.01)
    }

    /// Converts this acceleration to feet per second squared.
    fn to_feet_per_second_squared(&self) -> FeetPerSecondSquared {
        FeetPerSecondSquared(self.to_meters_per_second_squared().0 / 0.3048)
    }

    /// Converts this acceleration to kilometers per hour gained per second.
    fn to_kilometers_per_hour_per_second(&self) -> KilometersPerHourPerSecond {
        KilometersPerHourPerSecond(self.to_meters_per_second_squared().0 * 3.6)
    }

    /// Converts this acceleration to miles per hour gained per second.
    fn to_miles_per_hour_per_second(&self) -> MilesPerHourPerSecond {
        MilesPerHourPerSecond(self.to_meters_per_second_squared().0 / 0.447_04)
    }

    /// Converts this acceleration to gals.
    fn to_gals(&self) -> Gals {
        Gals(self.to_meters_per_second_squared().0 / 0.01)
    }

    /// Converts this acceleration to milligals.
    fn to_milligals(&self) -> Milligals {
        Milligals(self.to_meters_per_second_squared().0 / 1e-5)
    }

    /// Converts this acceleration to multiples of standard gravity.
    fn to_standard_gravities(&self) -> StandardGravities {
        StandardGravities(
            self.to_meters_per_second_squared().0 / STANDARD_GRAVITY_METERS_PER_SECOND_SQUARED,
        )
    }

    /// Returns this acceleration divided by another acceleration.
    fn ratio(&self, rhs: &dyn Acceleration) -> f64 {
        self.to_meters_per_second_squared().0 / rhs.to_meters_per_second_squared().0
    }

    /// Computes speed change over a time interval.
    fn speed_change_over(&self, time: &dyn Time) -> MetersPerSecond {
        MetersPerSecond(self.to_meters_per_second_squared().0 * time.to_seconds().0)
    }

    /// Computes the force exerted on a mass.
    fn weight_of(&self, mass: &dyn Mass) -> Newtons {
        Newtons(self.to_meters_per_second_squared().0 * mass.to_kilograms().0)
    }

    /// Returns whether the canonical value is finite.
    fn is_finite(&self) -> bool {
        self.to_meters_per_second_squared().0.is_finite()
    }
}

macro_rules! define_acceleration_unit {
    ($name:ident, $method:ident, $symbol:literal, $factor:expr, $doc:literal) => {
        #[doc = $doc]
        #[derive(Clone, Copy, Debug, Default)]
        pub struct $name(pub f64);

        impl Acceleration for $name {
            fn to_meters_per_second_squared(&self) -> MetersPerSecondSquared {
                MetersPerSecondSquared(self.0 * $factor)
            }
        }

        impl_unit_common!($name, Acceleration, $method, $symbol);
    };
}

define_acceleration_unit!(
    MetersPerSecondSquared,
    to_meters_per_second_squared,
    "m/s²",
    1.0,
    "An acceleration measured in meters per second squared."
);
define_acceleration_unit!(
    CentimetersPerSecondSquared,
    to_centimeters_per_second_squared,
    "cm/s²",
    0.01,
    "An acceleration measured in centimeters per second squared."
);
define_acceleration_unit!(
    FeetPerSecondSquared,
    to_feet_per_second_squared,
    "ft/s²",
    0.3048,
    "An acceleration measured in feet per second squared."
);
define_acceleration_unit!(
    KilometersPerHourPerSecond,
    to_kilometers_per_hour_per_second,
    "km/h/s",
    1.0 / 3.6,
    "An acceleration measured as kilometers per hour gained per second."
);
define_acceleration_unit!(
    MilesPerHourPerSecond,
    to_miles_per_hour_per_second,
    "mph/s",
    0.447_04,
    "An acceleration measured as miles per hour gained per second."
);
define_acceleration_unit!(
    Gals,
    to_gals,
    "Gal",
    0.01,
    "An acceleration measured in gals."
);
define_acceleration_unit!(
    Milligals,
    to_milligals,
    "mGal",
    1e-5,
    "An acceleration measured in milligals."
);
define_acceleration_unit!(
    StandardGravities,
    to_standard_gravities,
    "g₀",
    STANDARD_GRAVITY_METERS_PER_SECOND_SQUARED,
    "An acceleration measured in multiples of conventional standard gravity."
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
        assert_close(Gals(1.0).to_meters_per_second_squared().0, 0.01);
        assert_close(Milligals(1.0).to_meters_per_second_squared().0, 1e-5);
        assert_close(
            StandardGravities(1.0).to_meters_per_second_squared().0,
            9.80665,
        );
        assert_close(
            FeetPerSecondSquared(1.0).to_meters_per_second_squared().0,
            0.3048,
        );
    }

    #[test]
    fn duplicate_named_units_compare_equal() {
        assert_eq!(Gals(1.0), CentimetersPerSecondSquared(1.0));
        assert_eq!(STANDARD_GRAVITY, StandardGravities(1.0));
    }
}
