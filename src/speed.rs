//! Speed as distance traveled over a time interval.

use crate::distance::Distance;
use crate::time::Time;

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

    /// Consumes this speed and returns its distance and time components.
    pub fn into_parts(self) -> (D, T) {
        (self.distance, self.time)
    }

    /// Returns whether both components are finite.
    pub fn is_finite(&self) -> bool {
        self.distance.is_finite() && self.time.is_finite()
    }
}
