//! Frequency as a dimensionless cycle count over a duration.

use crate::time::{Time, TimeUnit};

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

    /// Converts the duration component to a requested unit.
    pub fn to_units<T2: TimeUnit>(&self) -> Frequency<T2> {
        Frequency::new(self.cycles, T2::from_seconds(self.duration.to_seconds()))
    }

    /// Returns the frequency value in the currently stored duration unit.
    pub fn value(&self) -> f64 {
        self.cycles / self.duration.value()
    }

    /// Returns the frequency value after converting duration to a requested unit.
    pub fn value_in<T2: TimeUnit>(&self) -> f64 {
        self.to_units::<T2>().value()
    }

    /// Returns whether both components are finite.
    pub fn is_finite(&self) -> bool {
        self.cycles.is_finite() && self.duration.is_finite()
    }
}
