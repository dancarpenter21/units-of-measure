//! Physical-identity operators for built-in unit types.

use crate::acceleration::Acceleration;
use crate::angle::{Angle, Radians};
use crate::angular_acceleration::AngularAcceleration;
use crate::angular_velocity::AngularVelocity;
use crate::area::Area;
use crate::distance::{Distance, Meters};
use crate::frequency::Frequency;
use crate::mass::{Kilograms, Mass};
use crate::speed::Speed;
use crate::time::{Seconds, Time};
use crate::torque::Torque;
use crate::weight::Weight;

macro_rules! for_each_distance_unit {
    ($callback:ident $(, $extra:tt)*) => {
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

macro_rules! for_each_time_unit {
    ($callback:ident $(, $extra:tt)*) => {
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

macro_rules! for_each_mass_unit {
    ($callback:ident $(, $extra:tt)*) => {
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

macro_rules! for_each_angle_unit {
    ($callback:ident $(, $extra:tt)*) => {
        $callback!(crate::angle::Degrees $(, $extra)*);
        $callback!(crate::angle::Radians $(, $extra)*);
    };
}

macro_rules! impl_distance_mul_distance_pair {
    ($rhs:ty, $lhs:ty) => {
        impl std::ops::Mul<$rhs> for $lhs {
            type Output = Area<$lhs, $rhs>;

            fn mul(self, rhs: $rhs) -> Self::Output {
                Area::new(self, rhs)
            }
        }
    };
}

macro_rules! impl_distance_div_time_pair {
    ($rhs:ty, $lhs:ty) => {
        impl std::ops::Div<$rhs> for $lhs {
            type Output = Speed<$lhs, $rhs>;

            fn div(self, rhs: $rhs) -> Self::Output {
                Speed::new(self, rhs)
            }
        }
    };
}

macro_rules! impl_distance_ops_for {
    ($lhs:ty) => {
        for_each_distance_unit!(impl_distance_mul_distance_pair, $lhs);
        for_each_time_unit!(impl_distance_div_time_pair, $lhs);

        impl<D: Distance, T: Time> std::ops::Div<Speed<D, T>> for $lhs {
            type Output = Seconds;

            fn div(self, rhs: Speed<D, T>) -> Self::Output {
                Seconds::new(self.to_meters().value() / rhs.value_in::<Meters, Seconds>())
            }
        }

        impl<M: Mass, D: Distance, T1: Time, T2: Time> std::ops::Mul<Weight<M, D, T1, T2>>
            for $lhs
        {
            type Output = Torque<M, D, $lhs, T1, T2>;

            fn mul(self, rhs: Weight<M, D, T1, T2>) -> Self::Output {
                let (mass, first_distance, first_time, second_time) = rhs.into_parts();
                Torque::new(mass, first_distance, self, first_time, second_time)
            }
        }
    };
}

for_each_distance_unit!(impl_distance_ops_for);

impl<D: Distance, T: Time, U: Time> std::ops::Mul<U> for Speed<D, T> {
    type Output = Meters;

    fn mul(self, rhs: U) -> Self::Output {
        Meters::new(self.value_in::<Meters, Seconds>() * rhs.to_seconds().value())
    }
}

impl<D: Distance, T: Time, U: Time> std::ops::Div<U> for Speed<D, T> {
    type Output = Acceleration<Meters, Seconds, Seconds>;

    fn div(self, rhs: U) -> Self::Output {
        Acceleration::new(
            Meters::new(self.value_in::<Meters, Seconds>()),
            Seconds::new(1.0),
            Seconds::new(rhs.to_seconds().value()),
        )
    }
}

macro_rules! impl_time_composed_ops {
    ($lhs:ty) => {
        impl<D: Distance, T: Time> std::ops::Mul<Speed<D, T>> for $lhs {
            type Output = Meters;

            fn mul(self, rhs: Speed<D, T>) -> Self::Output {
                Meters::new(rhs.value_in::<Meters, Seconds>() * self.to_seconds().value())
            }
        }

        impl<D: Distance, T1: Time, T2: Time> std::ops::Mul<Acceleration<D, T1, T2>> for $lhs {
            type Output = Speed<Meters, Seconds>;

            fn mul(self, rhs: Acceleration<D, T1, T2>) -> Self::Output {
                Speed::new(
                    Meters::new(
                        rhs.value_in::<Meters, Seconds, Seconds>() * self.to_seconds().value(),
                    ),
                    Seconds::new(1.0),
                )
            }
        }

        impl<A: Angle, T: Time> std::ops::Mul<AngularVelocity<A, T>> for $lhs {
            type Output = Radians;

            fn mul(self, rhs: AngularVelocity<A, T>) -> Self::Output {
                Radians::new(rhs.value_in::<Radians, Seconds>() * self.to_seconds().value())
            }
        }

        impl<A: Angle, T1: Time, T2: Time> std::ops::Mul<AngularAcceleration<A, T1, T2>> for $lhs {
            type Output = AngularVelocity<Radians, Seconds>;

            fn mul(self, rhs: AngularAcceleration<A, T1, T2>) -> Self::Output {
                AngularVelocity::new(
                    Radians::new(
                        rhs.value_in::<Radians, Seconds, Seconds>() * self.to_seconds().value(),
                    ),
                    Seconds::new(1.0),
                )
            }
        }

        impl<T: Time> std::ops::Mul<Frequency<T>> for $lhs {
            type Output = f64;

            fn mul(self, rhs: Frequency<T>) -> Self::Output {
                rhs.value_in::<Seconds>() * self.to_seconds().value()
            }
        }
    };
}

for_each_time_unit!(impl_time_composed_ops);

macro_rules! impl_acceleration_mul_time_pair {
    ($rhs:ty) => {
        impl<D: Distance, T1: Time, T2: Time> std::ops::Mul<$rhs> for Acceleration<D, T1, T2> {
            type Output = Speed<Meters, Seconds>;

            fn mul(self, rhs: $rhs) -> Self::Output {
                Speed::new(
                    Meters::new(
                        self.value_in::<Meters, Seconds, Seconds>() * rhs.to_seconds().value(),
                    ),
                    Seconds::new(1.0),
                )
            }
        }
    };
}

for_each_time_unit!(impl_acceleration_mul_time_pair);

impl<D: Distance, T1: Time, T2: Time, M: Mass> std::ops::Mul<M> for Acceleration<D, T1, T2> {
    type Output = Weight<Kilograms, Meters, Seconds, Seconds>;

    fn mul(self, rhs: M) -> Self::Output {
        Weight::new(
            Kilograms::new(rhs.to_kilograms().value()),
            Meters::new(self.value_in::<Meters, Seconds, Seconds>()),
            Seconds::new(1.0),
            Seconds::new(1.0),
        )
    }
}

macro_rules! impl_mass_mul_acceleration {
    ($lhs:ty) => {
        impl<D: Distance, T1: Time, T2: Time> std::ops::Mul<Acceleration<D, T1, T2>> for $lhs {
            type Output = Weight<Kilograms, Meters, Seconds, Seconds>;

            fn mul(self, rhs: Acceleration<D, T1, T2>) -> Self::Output {
                Weight::new(
                    Kilograms::new(self.to_kilograms().value()),
                    Meters::new(rhs.value_in::<Meters, Seconds, Seconds>()),
                    Seconds::new(1.0),
                    Seconds::new(1.0),
                )
            }
        }
    };
}

for_each_mass_unit!(impl_mass_mul_acceleration);

impl<M: Mass, D: Distance, T1: Time, T2: Time, R: Distance> std::ops::Mul<R>
    for Weight<M, D, T1, T2>
{
    type Output = Torque<M, D, R, T1, T2>;

    fn mul(self, rhs: R) -> Self::Output {
        let (mass, first_distance, first_time, second_time) = self.into_parts();
        Torque::new(mass, first_distance, rhs, first_time, second_time)
    }
}

impl<M: Mass, D1: Distance, D2: Distance, T1: Time, T2: Time, R: Distance> std::ops::Div<R>
    for Torque<M, D1, D2, T1, T2>
{
    type Output = Weight<Kilograms, Meters, Seconds, Seconds>;

    fn div(self, rhs: R) -> Self::Output {
        Weight::new(
            Kilograms::new(1.0),
            Meters::new(
                self.value_in::<Kilograms, Meters, Meters, Seconds, Seconds>()
                    / rhs.to_meters().value(),
            ),
            Seconds::new(1.0),
            Seconds::new(1.0),
        )
    }
}

macro_rules! impl_angle_div_time_pair {
    ($rhs:ty, $lhs:ty) => {
        impl std::ops::Div<$rhs> for $lhs {
            type Output = AngularVelocity<$lhs, $rhs>;

            fn div(self, rhs: $rhs) -> Self::Output {
                AngularVelocity::new(self, rhs)
            }
        }
    };
}

macro_rules! impl_angle_ops_for {
    ($lhs:ty) => {
        for_each_time_unit!(impl_angle_div_time_pair, $lhs);
    };
}

for_each_angle_unit!(impl_angle_ops_for);

impl<A: Angle, T: Time, U: Time> std::ops::Mul<U> for AngularVelocity<A, T> {
    type Output = Radians;

    fn mul(self, rhs: U) -> Self::Output {
        Radians::new(self.value_in::<Radians, Seconds>() * rhs.to_seconds().value())
    }
}

impl<A: Angle, T: Time, U: Time> std::ops::Div<U> for AngularVelocity<A, T> {
    type Output = AngularAcceleration<Radians, Seconds, Seconds>;

    fn div(self, rhs: U) -> Self::Output {
        AngularAcceleration::new(
            Radians::new(self.value_in::<Radians, Seconds>()),
            Seconds::new(1.0),
            Seconds::new(rhs.to_seconds().value()),
        )
    }
}

macro_rules! impl_angular_acceleration_mul_time_pair {
    ($rhs:ty) => {
        impl<A: Angle, T1: Time, T2: Time> std::ops::Mul<$rhs> for AngularAcceleration<A, T1, T2> {
            type Output = AngularVelocity<Radians, Seconds>;

            fn mul(self, rhs: $rhs) -> Self::Output {
                AngularVelocity::new(
                    Radians::new(
                        self.value_in::<Radians, Seconds, Seconds>() * rhs.to_seconds().value(),
                    ),
                    Seconds::new(1.0),
                )
            }
        }
    };
}

for_each_time_unit!(impl_angular_acceleration_mul_time_pair);

impl<T: Time, U: Time> std::ops::Mul<U> for Frequency<T> {
    type Output = f64;

    fn mul(self, rhs: U) -> Self::Output {
        self.value_in::<Seconds>() * rhs.to_seconds().value()
    }
}
