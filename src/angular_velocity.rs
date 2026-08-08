//! Angular velocity as angle divided by time.

use crate::angle::Angle;
use crate::time::Time;

/// An angular velocity represented by its angle and time components.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct AngularVelocity<A: Angle, T: Time> {
    angle: A,
    time: T,
}

impl<A: Angle, T: Time> AngularVelocity<A, T> {
    /// Creates an angular velocity from an angle over a time interval.
    pub const fn new(angle: A, time: T) -> Self {
        Self { angle, time }
    }

    /// Returns the angle component in its original unit.
    pub const fn angle(&self) -> &A {
        &self.angle
    }

    /// Returns the time component in its original unit.
    pub const fn time(&self) -> &T {
        &self.time
    }

    /// Consumes this angular velocity and returns its angle and time components.
    pub fn into_parts(self) -> (A, T) {
        (self.angle, self.time)
    }

    /// Returns whether both components are finite.
    pub fn is_finite(&self) -> bool {
        self.angle.is_finite() && self.time.is_finite()
    }
}
