//! Angular velocity as angle divided by time.

use crate::angle::{Angle, AngleUnit};
use crate::time::{Time, TimeUnit};

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

    /// Converts the angle and time components to requested units.
    pub fn to_units<A2: AngleUnit, T2: TimeUnit>(&self) -> AngularVelocity<A2, T2> {
        AngularVelocity::new(
            A2::from_radians(self.angle.to_radians()),
            T2::from_seconds(self.time.to_seconds()),
        )
    }

    /// Returns the angular velocity value in the currently stored component units.
    pub fn value(&self) -> f64 {
        self.angle.value() / self.time.value()
    }

    /// Returns the angular velocity value after converting components to requested units.
    pub fn value_in<A2: AngleUnit, T2: TimeUnit>(&self) -> f64 {
        self.to_units::<A2, T2>().value()
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
