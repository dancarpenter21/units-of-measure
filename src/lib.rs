#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

macro_rules! impl_unit_common {
    ($unit:ident, $quantity:ident, $to_unit:ident, $symbol:literal) => {
        impl $unit {
            /// Zero of this unit.
            pub const ZERO: Self = Self(0.0);

            /// Creates a value in this unit.
            pub const fn new(value: f64) -> Self {
                Self(value)
            }

            /// Returns the numeric value expressed in this unit.
            pub const fn value(self) -> f64 {
                self.0
            }

            /// Returns the absolute magnitude in this unit.
            pub fn abs(self) -> Self {
                Self(self.0.abs())
            }

            /// Returns whether the numeric value is finite.
            pub fn is_finite(self) -> bool {
                self.0.is_finite()
            }
        }

        impl From<f64> for $unit {
            fn from(value: f64) -> Self {
                Self(value)
            }
        }

        impl From<$unit> for f64 {
            fn from(value: $unit) -> Self {
                value.0
            }
        }

        #[cfg(feature = "serde")]
        impl serde::Serialize for $unit {
            fn serialize<Serializer>(
                &self,
                serializer: Serializer,
            ) -> Result<Serializer::Ok, Serializer::Error>
            where
                Serializer: serde::Serializer,
            {
                serializer.serialize_f64(self.0)
            }
        }

        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for $unit {
            fn deserialize<Deserializer>(
                deserializer: Deserializer,
            ) -> Result<Self, Deserializer::Error>
            where
                Deserializer: serde::Deserializer<'de>,
            {
                <f64 as serde::Deserialize<'de>>::deserialize(deserializer).map(Self)
            }
        }

        impl<Rhs: $quantity> std::ops::Add<Rhs> for $unit {
            type Output = Self;

            fn add(self, rhs: Rhs) -> Self::Output {
                Self(self.0 + rhs.$to_unit().0)
            }
        }

        impl<Rhs: $quantity> std::ops::AddAssign<Rhs> for $unit {
            fn add_assign(&mut self, rhs: Rhs) {
                self.0 += rhs.$to_unit().0;
            }
        }

        impl<Rhs: $quantity> std::ops::Sub<Rhs> for $unit {
            type Output = Self;

            fn sub(self, rhs: Rhs) -> Self::Output {
                Self(self.0 - rhs.$to_unit().0)
            }
        }

        impl<Rhs: $quantity> std::ops::SubAssign<Rhs> for $unit {
            fn sub_assign(&mut self, rhs: Rhs) {
                self.0 -= rhs.$to_unit().0;
            }
        }

        impl std::ops::Neg for $unit {
            type Output = Self;

            fn neg(self) -> Self::Output {
                Self(-self.0)
            }
        }

        impl std::ops::Mul<f64> for $unit {
            type Output = Self;

            fn mul(self, rhs: f64) -> Self::Output {
                Self(self.0 * rhs)
            }
        }

        impl std::ops::Mul<$unit> for f64 {
            type Output = $unit;

            fn mul(self, rhs: $unit) -> Self::Output {
                rhs * self
            }
        }

        impl std::ops::MulAssign<f64> for $unit {
            fn mul_assign(&mut self, rhs: f64) {
                self.0 *= rhs;
            }
        }

        impl std::ops::Div<f64> for $unit {
            type Output = Self;

            fn div(self, rhs: f64) -> Self::Output {
                Self(self.0 / rhs)
            }
        }

        impl std::ops::DivAssign<f64> for $unit {
            fn div_assign(&mut self, rhs: f64) {
                self.0 /= rhs;
            }
        }

        impl<Rhs: $quantity> std::ops::Div<Rhs> for $unit {
            type Output = f64;

            fn div(self, rhs: Rhs) -> Self::Output {
                self.0 / rhs.$to_unit().0
            }
        }

        impl<Rhs: $quantity + ?Sized> PartialEq<Rhs> for $unit {
            fn eq(&self, rhs: &Rhs) -> bool {
                self.$to_unit().0 == rhs.$to_unit().0
            }
        }

        impl<Rhs: $quantity + ?Sized> PartialOrd<Rhs> for $unit {
            fn partial_cmp(&self, rhs: &Rhs) -> Option<std::cmp::Ordering> {
                self.$to_unit().0.partial_cmp(&rhs.$to_unit().0)
            }
        }

        impl std::iter::Sum for $unit {
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold(Self::ZERO, std::ops::Add::add)
            }
        }

        impl<'a> std::iter::Sum<&'a Self> for $unit {
            fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
                iter.copied().sum()
            }
        }

        impl std::fmt::Display for $unit {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "{} {}", self.0, $symbol)
            }
        }
    };
}

pub mod acceleration;
pub mod area;
pub mod audio;
pub mod distance;
pub mod frequency;
pub mod mass;
pub mod speed;
pub mod time;
pub mod weight;

mod ops;

pub use acceleration::Acceleration;
pub use area::Area;
pub use distance::Distance;
pub use frequency::Frequency;
pub use mass::Mass;
pub use speed::Speed;
pub use time::Time;
pub use weight::Weight;
