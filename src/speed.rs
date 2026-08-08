//! Speed as distance traveled over a time interval.

use crate::distance::{Distance, DistanceUnit};
use crate::time::{Time, TimeUnit};

/// A speed represented by its distance and time components.
///
/// The supplied units are retained exactly, so callers can convert either
/// component to the unit needed for a calculation.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Speed<D: Distance, T: Time> {
    distance: D,
    time: T,
}

impl<D: Distance, T: Time> Speed<D, T> {
    /// Creates a speed from a distance traveled over a time interval.
    pub const fn new(distance: D, time: T) -> Self {
        Self { distance, time }
    }

    /// Returns the distance component in its original unit.
    pub const fn distance(&self) -> &D {
        &self.distance
    }

    /// Returns the time component in its original unit.
    pub const fn time(&self) -> &T {
        &self.time
    }

    /// Converts the distance and time components to requested units.
    pub fn to_units<D2: DistanceUnit, T2: TimeUnit>(&self) -> Speed<D2, T2> {
        Speed::new(
            D2::from_meters(self.distance.to_meters()),
            T2::from_seconds(self.time.to_seconds()),
        )
    }

    /// Returns the speed value in the currently stored component units.
    pub fn value(&self) -> f64 {
        self.distance.value() / self.time.value()
    }

    /// Returns the speed value after converting components to requested units.
    pub fn value_in<D2: DistanceUnit, T2: TimeUnit>(&self) -> f64 {
        self.to_units::<D2, T2>().value()
    }

    /// Consumes this speed and returns its distance and time components.
    pub fn into_parts(self) -> (D, T) {
        (self.distance, self.time)
    }

    /// Returns whether both components are finite.
    pub fn is_finite(&self) -> bool {
        self.distance.is_finite() && self.time.is_finite()
    }
}
