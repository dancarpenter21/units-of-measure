//! Acceleration as distance divided by two time components.

use crate::distance::{Distance, Meters};
use crate::speed::Speed;
use crate::time::{Seconds, Time};

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

    /// Consumes this acceleration and returns its base components.
    pub fn into_parts(self) -> (D, T1, T2) {
        (self.distance, self.first_time, self.second_time)
    }

    /// Returns whether all components are finite.
    pub fn is_finite(&self) -> bool {
        self.distance.is_finite() && self.first_time.is_finite() && self.second_time.is_finite()
    }
}
