//! Frequency as a dimensionless cycle count over a duration.

use crate::time::Time;

/// A frequency represented by cycles completed during a duration.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Frequency<T: Time> {
    cycles: f64,
    duration: T,
}

impl<T: Time> Frequency<T> {
    /// Creates a frequency from a cycle count and duration.
    pub const fn new(cycles: f64, duration: T) -> Self {
        Self { cycles, duration }
    }

    /// Returns the dimensionless number of cycles.
    pub const fn cycles(&self) -> f64 {
        self.cycles
    }

    /// Returns the duration component in its original unit.
    pub const fn duration(&self) -> &T {
        &self.duration
    }

    /// Returns whether both components are finite.
    pub fn is_finite(&self) -> bool {
        self.cycles.is_finite() && self.duration.is_finite()
    }
}
