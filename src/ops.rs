//! Dimensional operators between built-in units.

macro_rules! for_each_distance_unit {
    ($callback:ident $(, $extra:ty)*) => {
        $callback!(crate::distance::Picometers $(, $extra)*);
        $callback!(crate::distance::Angstroms $(, $extra)*);
        $callback!(crate::distance::Nanometers $(, $extra)*);
        $callback!(crate::distance::Micrometers $(, $extra)*);
        $callback!(crate::distance::Millimeters $(, $extra)*);
        $callback!(crate::distance::Centimeters $(, $extra)*);
        $callback!(crate::distance::Decimeters $(, $extra)*);
        $callback!(crate::distance::Meters $(, $extra)*);
        $callback!(crate::distance::Kilometers $(, $extra)*);
        $callback!(crate::distance::Inches $(, $extra)*);
        $callback!(crate::distance::Feet $(, $extra)*);
        $callback!(crate::distance::Yards $(, $extra)*);
        $callback!(crate::distance::Miles $(, $extra)*);
        $callback!(crate::distance::NauticalMiles $(, $extra)*);
        $callback!(crate::distance::AstronomicalUnits $(, $extra)*);
        $callback!(crate::distance::LightYears $(, $extra)*);
        $callback!(crate::distance::Parsecs $(, $extra)*);
    };
}

macro_rules! for_each_area_unit {
    ($callback:ident $(, $extra:ty)*) => {
        $callback!(crate::area::SquarePicometers $(, $extra)*);
        $callback!(crate::area::SquareAngstroms $(, $extra)*);
        $callback!(crate::area::SquareNanometers $(, $extra)*);
        $callback!(crate::area::SquareMicrometers $(, $extra)*);
        $callback!(crate::area::SquareMillimeters $(, $extra)*);
        $callback!(crate::area::SquareCentimeters $(, $extra)*);
        $callback!(crate::area::SquareDecimeters $(, $extra)*);
        $callback!(crate::area::SquareMeters $(, $extra)*);
        $callback!(crate::area::SquareKilometers $(, $extra)*);
        $callback!(crate::area::SquareInches $(, $extra)*);
        $callback!(crate::area::SquareFeet $(, $extra)*);
        $callback!(crate::area::SquareYards $(, $extra)*);
        $callback!(crate::area::SquareMiles $(, $extra)*);
        $callback!(crate::area::SquareNauticalMiles $(, $extra)*);
        $callback!(crate::area::SquareAstronomicalUnits $(, $extra)*);
        $callback!(crate::area::SquareLightYears $(, $extra)*);
        $callback!(crate::area::SquareParsecs $(, $extra)*);
        $callback!(crate::area::Ares $(, $extra)*);
        $callback!(crate::area::Hectares $(, $extra)*);
        $callback!(crate::area::Acres $(, $extra)*);
        $callback!(crate::area::Barns $(, $extra)*);
    };
}

macro_rules! for_each_time_unit {
    ($callback:ident $(, $extra:ty)*) => {
        $callback!(crate::time::Femtoseconds $(, $extra)*);
        $callback!(crate::time::Picoseconds $(, $extra)*);
        $callback!(crate::time::Nanoseconds $(, $extra)*);
        $callback!(crate::time::Microseconds $(, $extra)*);
        $callback!(crate::time::Milliseconds $(, $extra)*);
        $callback!(crate::time::Seconds $(, $extra)*);
        $callback!(crate::time::Minutes $(, $extra)*);
        $callback!(crate::time::Hours $(, $extra)*);
        $callback!(crate::time::Days $(, $extra)*);
        $callback!(crate::time::Weeks $(, $extra)*);
        $callback!(crate::time::MeanGregorianMonths $(, $extra)*);
        $callback!(crate::time::MeanGregorianYears $(, $extra)*);
        $callback!(crate::time::JulianYears $(, $extra)*);
    };
}

macro_rules! for_each_speed_unit {
    ($callback:ident $(, $extra:ty)*) => {
        $callback!(crate::speed::CentimetersPerSecond $(, $extra)*);
        $callback!(crate::speed::MetersPerSecond $(, $extra)*);
        $callback!(crate::speed::KilometersPerSecond $(, $extra)*);
        $callback!(crate::speed::KilometersPerHour $(, $extra)*);
        $callback!(crate::speed::MilesPerHour $(, $extra)*);
        $callback!(crate::speed::FeetPerSecond $(, $extra)*);
        $callback!(crate::speed::FeetPerMinute $(, $extra)*);
        $callback!(crate::speed::Knots $(, $extra)*);
        $callback!(crate::speed::Mach $(, $extra)*);
    };
}

macro_rules! for_each_acceleration_unit {
    ($callback:ident $(, $extra:ty)*) => {
        $callback!(crate::acceleration::MetersPerSecondSquared $(, $extra)*);
        $callback!(crate::acceleration::CentimetersPerSecondSquared $(, $extra)*);
        $callback!(crate::acceleration::FeetPerSecondSquared $(, $extra)*);
        $callback!(crate::acceleration::KilometersPerHourPerSecond $(, $extra)*);
        $callback!(crate::acceleration::MilesPerHourPerSecond $(, $extra)*);
        $callback!(crate::acceleration::Gals $(, $extra)*);
        $callback!(crate::acceleration::Milligals $(, $extra)*);
        $callback!(crate::acceleration::StandardGravities $(, $extra)*);
    };
}

macro_rules! for_each_mass_unit {
    ($callback:ident $(, $extra:ty)*) => {
        $callback!(crate::mass::Nanograms $(, $extra)*);
        $callback!(crate::mass::Micrograms $(, $extra)*);
        $callback!(crate::mass::Milligrams $(, $extra)*);
        $callback!(crate::mass::Grams $(, $extra)*);
        $callback!(crate::mass::Kilograms $(, $extra)*);
        $callback!(crate::mass::MetricTonnes $(, $extra)*);
        $callback!(crate::mass::Carats $(, $extra)*);
        $callback!(crate::mass::Grains $(, $extra)*);
        $callback!(crate::mass::Ounces $(, $extra)*);
        $callback!(crate::mass::Pounds $(, $extra)*);
        $callback!(crate::mass::Stones $(, $extra)*);
        $callback!(crate::mass::ShortTons $(, $extra)*);
        $callback!(crate::mass::LongTons $(, $extra)*);
        $callback!(crate::mass::TroyOunces $(, $extra)*);
        $callback!(crate::mass::Slugs $(, $extra)*);
        $callback!(crate::mass::Daltons $(, $extra)*);
        $callback!(crate::mass::UnifiedAtomicMassUnits $(, $extra)*);
    };
}

macro_rules! for_each_weight_unit {
    ($callback:ident $(, $extra:ty)*) => {
        $callback!(crate::weight::Dynes $(, $extra)*);
        $callback!(crate::weight::Newtons $(, $extra)*);
        $callback!(crate::weight::Kilonewtons $(, $extra)*);
        $callback!(crate::weight::Ounces $(, $extra)*);
        $callback!(crate::weight::Pounds $(, $extra)*);
        $callback!(crate::weight::Stones $(, $extra)*);
        $callback!(crate::weight::Kilograms $(, $extra)*);
        $callback!(crate::weight::Kips $(, $extra)*);
    };
}

macro_rules! for_each_frequency_unit {
    ($callback:ident $(, $extra:ty)*) => {
        $callback!(crate::frequency::Microhertz $(, $extra)*);
        $callback!(crate::frequency::Millihertz $(, $extra)*);
        $callback!(crate::frequency::Hertz $(, $extra)*);
        $callback!(crate::frequency::Kilohertz $(, $extra)*);
        $callback!(crate::frequency::Megahertz $(, $extra)*);
        $callback!(crate::frequency::Gigahertz $(, $extra)*);
        $callback!(crate::frequency::Terahertz $(, $extra)*);
        $callback!(crate::frequency::Petahertz $(, $extra)*);
        $callback!(crate::frequency::RevolutionsPerSecond $(, $extra)*);
        $callback!(crate::frequency::RevolutionsPerMinute $(, $extra)*);
        $callback!(crate::frequency::BeatsPerMinute $(, $extra)*);
        $callback!(crate::frequency::RadiansPerSecond $(, $extra)*);
    };
}

macro_rules! impl_distance_mul_distance_pair {
    ($rhs:ty, $lhs:ty) => {
        impl std::ops::Mul<$rhs> for $lhs {
            type Output = crate::area::SquareMeters;

            fn mul(self, rhs: $rhs) -> Self::Output {
                use crate::distance::Distance;
                crate::area::SquareMeters(self.to_meters().0 * rhs.to_meters().0)
            }
        }
    };
}

macro_rules! impl_distance_div_time_pair {
    ($rhs:ty, $lhs:ty) => {
        impl std::ops::Div<$rhs> for $lhs {
            type Output = crate::speed::MetersPerSecond;

            fn div(self, rhs: $rhs) -> Self::Output {
                use crate::{distance::Distance, time::Time};
                crate::speed::MetersPerSecond(self.to_meters().0 / rhs.to_seconds().0)
            }
        }
    };
}

macro_rules! impl_distance_div_speed_pair {
    ($rhs:ty, $lhs:ty) => {
        impl std::ops::Div<$rhs> for $lhs {
            type Output = crate::time::Seconds;

            fn div(self, rhs: $rhs) -> Self::Output {
                use crate::{distance::Distance, speed::Speed};
                crate::time::Seconds(self.to_meters().0 / rhs.to_meters_per_second().0)
            }
        }
    };
}

macro_rules! impl_distance_ops_for {
    ($lhs:ty) => {
        for_each_distance_unit!(impl_distance_mul_distance_pair, $lhs);
        for_each_time_unit!(impl_distance_div_time_pair, $lhs);
        for_each_speed_unit!(impl_distance_div_speed_pair, $lhs);
    };
}

for_each_distance_unit!(impl_distance_ops_for);

macro_rules! impl_area_div_distance_pair {
    ($rhs:ty, $lhs:ty) => {
        impl std::ops::Div<$rhs> for $lhs {
            type Output = crate::distance::Meters;

            fn div(self, rhs: $rhs) -> Self::Output {
                use crate::{area::Area, distance::Distance};
                crate::distance::Meters(self.to_square_meters().0 / rhs.to_meters().0)
            }
        }
    };
}

macro_rules! impl_area_ops_for {
    ($lhs:ty) => {
        for_each_distance_unit!(impl_area_div_distance_pair, $lhs);
    };
}

for_each_area_unit!(impl_area_ops_for);

macro_rules! impl_speed_mul_time_pair {
    ($rhs:ty, $lhs:ty) => {
        impl std::ops::Mul<$rhs> for $lhs {
            type Output = crate::distance::Meters;

            fn mul(self, rhs: $rhs) -> Self::Output {
                use crate::{speed::Speed, time::Time};
                crate::distance::Meters(self.to_meters_per_second().0 * rhs.to_seconds().0)
            }
        }
    };
}

macro_rules! impl_speed_div_time_pair {
    ($rhs:ty, $lhs:ty) => {
        impl std::ops::Div<$rhs> for $lhs {
            type Output = crate::acceleration::MetersPerSecondSquared;

            fn div(self, rhs: $rhs) -> Self::Output {
                use crate::{speed::Speed, time::Time};
                crate::acceleration::MetersPerSecondSquared(
                    self.to_meters_per_second().0 / rhs.to_seconds().0,
                )
            }
        }
    };
}

macro_rules! impl_speed_div_acceleration_pair {
    ($rhs:ty, $lhs:ty) => {
        impl std::ops::Div<$rhs> for $lhs {
            type Output = crate::time::Seconds;

            fn div(self, rhs: $rhs) -> Self::Output {
                use crate::{acceleration::Acceleration, speed::Speed};
                crate::time::Seconds(
                    self.to_meters_per_second().0 / rhs.to_meters_per_second_squared().0,
                )
            }
        }
    };
}

macro_rules! impl_speed_ops_for {
    ($lhs:ty) => {
        for_each_time_unit!(impl_speed_mul_time_pair, $lhs);
        for_each_time_unit!(impl_speed_div_time_pair, $lhs);
        for_each_acceleration_unit!(impl_speed_div_acceleration_pair, $lhs);
    };
}

for_each_speed_unit!(impl_speed_ops_for);

macro_rules! impl_time_mul_speed_pair {
    ($rhs:ty, $lhs:ty) => {
        impl std::ops::Mul<$rhs> for $lhs {
            type Output = crate::distance::Meters;

            fn mul(self, rhs: $rhs) -> Self::Output {
                use crate::{speed::Speed, time::Time};
                crate::distance::Meters(self.to_seconds().0 * rhs.to_meters_per_second().0)
            }
        }
    };
}

macro_rules! impl_time_mul_acceleration_pair {
    ($rhs:ty, $lhs:ty) => {
        impl std::ops::Mul<$rhs> for $lhs {
            type Output = crate::speed::MetersPerSecond;

            fn mul(self, rhs: $rhs) -> Self::Output {
                use crate::{acceleration::Acceleration, time::Time};
                crate::speed::MetersPerSecond(
                    self.to_seconds().0 * rhs.to_meters_per_second_squared().0,
                )
            }
        }
    };
}

macro_rules! impl_time_mul_frequency_pair {
    ($rhs:ty, $lhs:ty) => {
        impl std::ops::Mul<$rhs> for $lhs {
            type Output = f64;

            fn mul(self, rhs: $rhs) -> Self::Output {
                use crate::{frequency::Frequency, time::Time};
                self.to_seconds().0 * rhs.to_hertz().0
            }
        }
    };
}

macro_rules! impl_time_ops_for {
    ($lhs:ty) => {
        for_each_speed_unit!(impl_time_mul_speed_pair, $lhs);
        for_each_acceleration_unit!(impl_time_mul_acceleration_pair, $lhs);
        for_each_frequency_unit!(impl_time_mul_frequency_pair, $lhs);
    };
}

for_each_time_unit!(impl_time_ops_for);

macro_rules! impl_acceleration_mul_time_pair {
    ($rhs:ty, $lhs:ty) => {
        impl std::ops::Mul<$rhs> for $lhs {
            type Output = crate::speed::MetersPerSecond;

            fn mul(self, rhs: $rhs) -> Self::Output {
                use crate::{acceleration::Acceleration, time::Time};
                crate::speed::MetersPerSecond(
                    self.to_meters_per_second_squared().0 * rhs.to_seconds().0,
                )
            }
        }
    };
}

macro_rules! impl_acceleration_mul_mass_pair {
    ($rhs:ty, $lhs:ty) => {
        impl std::ops::Mul<$rhs> for $lhs {
            type Output = crate::weight::Newtons;

            fn mul(self, rhs: $rhs) -> Self::Output {
                use crate::{acceleration::Acceleration, mass::Mass};
                crate::weight::Newtons(self.to_meters_per_second_squared().0 * rhs.to_kilograms().0)
            }
        }
    };
}

macro_rules! impl_acceleration_ops_for {
    ($lhs:ty) => {
        for_each_time_unit!(impl_acceleration_mul_time_pair, $lhs);
        for_each_mass_unit!(impl_acceleration_mul_mass_pair, $lhs);
    };
}

for_each_acceleration_unit!(impl_acceleration_ops_for);

macro_rules! impl_mass_mul_acceleration_pair {
    ($rhs:ty, $lhs:ty) => {
        impl std::ops::Mul<$rhs> for $lhs {
            type Output = crate::weight::Newtons;

            fn mul(self, rhs: $rhs) -> Self::Output {
                use crate::{acceleration::Acceleration, mass::Mass};
                crate::weight::Newtons(self.to_kilograms().0 * rhs.to_meters_per_second_squared().0)
            }
        }
    };
}

macro_rules! impl_mass_ops_for {
    ($lhs:ty) => {
        for_each_acceleration_unit!(impl_mass_mul_acceleration_pair, $lhs);
    };
}

for_each_mass_unit!(impl_mass_ops_for);

macro_rules! impl_weight_div_mass_pair {
    ($rhs:ty, $lhs:ty) => {
        impl std::ops::Div<$rhs> for $lhs {
            type Output = crate::acceleration::MetersPerSecondSquared;

            fn div(self, rhs: $rhs) -> Self::Output {
                use crate::{mass::Mass, weight::Weight};
                crate::acceleration::MetersPerSecondSquared(
                    self.to_newtons().0 / rhs.to_kilograms().0,
                )
            }
        }
    };
}

macro_rules! impl_weight_div_acceleration_pair {
    ($rhs:ty, $lhs:ty) => {
        impl std::ops::Div<$rhs> for $lhs {
            type Output = crate::mass::Kilograms;

            fn div(self, rhs: $rhs) -> Self::Output {
                use crate::{acceleration::Acceleration, weight::Weight};
                crate::mass::Kilograms(self.to_newtons().0 / rhs.to_meters_per_second_squared().0)
            }
        }
    };
}

macro_rules! impl_weight_ops_for {
    ($lhs:ty) => {
        for_each_mass_unit!(impl_weight_div_mass_pair, $lhs);
        for_each_acceleration_unit!(impl_weight_div_acceleration_pair, $lhs);
    };
}

for_each_weight_unit!(impl_weight_ops_for);

macro_rules! impl_frequency_mul_time_pair {
    ($rhs:ty, $lhs:ty) => {
        impl std::ops::Mul<$rhs> for $lhs {
            type Output = f64;

            fn mul(self, rhs: $rhs) -> Self::Output {
                use crate::{frequency::Frequency, time::Time};
                self.to_hertz().0 * rhs.to_seconds().0
            }
        }
    };
}

macro_rules! impl_frequency_ops_for {
    ($lhs:ty) => {
        for_each_time_unit!(impl_frequency_mul_time_pair, $lhs);
    };
}

for_each_frequency_unit!(impl_frequency_ops_for);

#[cfg(test)]
mod tests {
    use crate::{
        acceleration::{Acceleration, MetersPerSecondSquared, StandardGravities},
        area::{Area, SquareFeet},
        distance::{Distance, Feet, Kilometers, Meters, Miles},
        frequency::{Hertz, Kilohertz},
        mass::{Kilograms, Mass, Pounds as PoundsMass},
        speed::{Knots, MetersPerSecond, MilesPerHour, Speed},
        time::{Hours, Milliseconds, Seconds, Time},
        weight::{Newtons, Weight},
    };

    fn assert_close(actual: f64, expected: f64) {
        let scale = actual.abs().max(expected.abs()).max(1.0);
        assert!((actual - expected).abs() <= f64::EPSILON * 32.0 * scale);
    }

    #[test]
    fn distance_and_area_operators_return_si_units() {
        assert_close((Feet(10.0) * Feet(12.0)).to_square_feet().0, 120.0);
        assert_close((SquareFeet(120.0) / Feet(10.0)).to_feet().0, 12.0);
    }

    #[test]
    fn distance_speed_time_identities_hold() {
        let speed = Miles(120.0) / Hours(2.0);
        assert_close(speed.to_miles_per_hour().0, 60.0);
        assert_close((speed * Hours(2.0)).to_miles().0, 120.0);
        assert_close((Hours(2.0) * speed).to_miles().0, 120.0);
        assert_close(
            (Kilometers(10.0) / MetersPerSecond(10.0)).to_seconds().0,
            1_000.0,
        );
        assert_close((Knots(20.0) * Hours(2.0)).to_nautical_miles().0, 40.0);
    }

    #[test]
    fn acceleration_mass_weight_identities_hold() {
        let acceleration = MetersPerSecond(30.0) / Seconds(5.0);
        assert_close(acceleration.to_meters_per_second_squared().0, 6.0);
        assert_close((acceleration * Seconds(5.0)).to_meters_per_second().0, 30.0);
        assert_close((MetersPerSecond(30.0) / acceleration).to_seconds().0, 5.0);

        let weight = PoundsMass(180.0) * StandardGravities(1.0);
        assert_close(weight.to_pounds().0, 180.0);
        assert_close((weight / StandardGravities(1.0)).to_pounds().0, 180.0);
        assert_close(
            (Newtons(20.0) / Kilograms(10.0))
                .to_meters_per_second_squared()
                .0,
            2.0,
        );
    }

    #[test]
    fn frequency_and_time_return_cycles() {
        assert_close(Kilohertz(48.0) * Milliseconds(10.0), 480.0);
        assert_close(Seconds(0.5) * Hertz(440.0), 220.0);
    }

    #[test]
    fn zero_division_follows_ieee_754() {
        assert!((Meters(1.0) / Seconds::ZERO).0.is_infinite());
        assert!((MetersPerSecond(1.0) / Seconds::ZERO).0.is_infinite());
        assert!((Newtons(1.0) / Kilograms::ZERO).0.is_infinite());
    }

    #[test]
    fn canonical_result_types_are_stable() {
        let _: crate::area::SquareMeters = Miles(1.0) * Feet(1.0);
        let _: crate::speed::MetersPerSecond = Miles(1.0) / Hours(1.0);
        let _: crate::time::Seconds = Miles(1.0) / MilesPerHour(1.0);
        let _: crate::acceleration::MetersPerSecondSquared = MetersPerSecond(1.0) / Seconds(1.0);
        let _: crate::weight::Newtons = Kilograms(1.0) * MetersPerSecondSquared(1.0);
    }
}
