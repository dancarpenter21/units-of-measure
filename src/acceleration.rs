//! Acceleration as distance divided by two time components.

use crate::distance::{Distance, DistanceUnit, Meters};
use crate::speed::Speed;
use crate::time::{Seconds, Time, TimeUnit};

/// Conventional standard gravity represented as meters per second squared.
pub const STANDARD_GRAVITY: Acceleration<Meters, Seconds, Seconds> =
    Acceleration::new(Meters::new(9.806_65), Seconds::new(1.0), Seconds::new(1.0));

/// An acceleration represented by distance divided by two time components.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Acceleration<D: Distance, T1: Time, T2: Time> {
    distance: D,
    first_time: T1,
    second_time: T2,
}

impl<D: Distance, T1: Time, T2: Time> Acceleration<D, T1, T2> {
    /// Creates an acceleration from distance divided by two time components.
    pub const fn new(distance: D, first_time: T1, second_time: T2) -> Self {
        Self {
            distance,
            first_time,
            second_time,
        }
    }

    /// Creates acceleration from a speed and the time over which it changes.
    pub fn from_speed_and_time(speed: Speed<D, T1>, second_time: T2) -> Self {
        let (distance, first_time) = speed.into_parts();
        Self::new(distance, first_time, second_time)
    }

    /// Returns the distance component in its original unit.
    pub const fn distance(&self) -> &D {
        &self.distance
    }

    /// Returns the first time component in its original unit.
    pub const fn first_time(&self) -> &T1 {
        &self.first_time
    }

    /// Returns the second time component in its original unit.
    pub const fn second_time(&self) -> &T2 {
        &self.second_time
    }

    /// Converts the distance and time components to requested units.
    pub fn to_units<D2: DistanceUnit, U1: TimeUnit, U2: TimeUnit>(
        &self,
    ) -> Acceleration<D2, U1, U2> {
        Acceleration::new(
            D2::from_meters(self.distance.to_meters()),
            U1::from_seconds(self.first_time.to_seconds()),
            U2::from_seconds(self.second_time.to_seconds()),
        )
    }

    /// Returns the acceleration value in the currently stored component units.
    pub fn value(&self) -> f64 {
        self.distance.value() / (self.first_time.value() * self.second_time.value())
    }

    /// Returns the acceleration value after converting components to requested units.
    pub fn value_in<D2: DistanceUnit, U1: TimeUnit, U2: TimeUnit>(&self) -> f64 {
        self.to_units::<D2, U1, U2>().value()
    }

    /// Consumes this acceleration and returns its base components.
    pub fn into_parts(self) -> (D, T1, T2) {
        (self.distance, self.first_time, self.second_time)
    }

    /// Returns whether all components are finite.
    pub fn is_finite(&self) -> bool {
        self.distance.is_finite() && self.first_time.is_finite() && self.second_time.is_finite()
    }
}
