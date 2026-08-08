//! Angular acceleration as angle divided by two time components.

use crate::angle::{Angle, AngleUnit};
use crate::angular_velocity::AngularVelocity;
use crate::time::{Time, TimeUnit};

/// An angular acceleration represented by angle divided by two time components.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct AngularAcceleration<A: Angle, T1: Time, T2: Time> {
    angle: A,
    first_time: T1,
    second_time: T2,
}

impl<A: Angle, T1: Time, T2: Time> AngularAcceleration<A, T1, T2> {
    /// Creates angular acceleration from angle divided by two time components.
    pub const fn new(angle: A, first_time: T1, second_time: T2) -> Self {
        Self {
            angle,
            first_time,
            second_time,
        }
    }

    /// Creates angular acceleration from angular velocity and change time.
    pub fn from_angular_velocity_and_time(
        angular_velocity: AngularVelocity<A, T1>,
        second_time: T2,
    ) -> Self {
        let (angle, first_time) = angular_velocity.into_parts();
        Self::new(angle, first_time, second_time)
    }

    /// Returns the angle component in its original unit.
    pub const fn angle(&self) -> &A {
        &self.angle
    }

    /// Returns the first time component in its original unit.
    pub const fn first_time(&self) -> &T1 {
        &self.first_time
    }

    /// Returns the second time component in its original unit.
    pub const fn second_time(&self) -> &T2 {
        &self.second_time
    }

    /// Converts the angle and time components to requested units.
    pub fn to_units<A2: AngleUnit, U1: TimeUnit, U2: TimeUnit>(
        &self,
    ) -> AngularAcceleration<A2, U1, U2> {
        AngularAcceleration::new(
            A2::from_radians(self.angle.to_radians()),
            U1::from_seconds(self.first_time.to_seconds()),
            U2::from_seconds(self.second_time.to_seconds()),
        )
    }

    /// Returns the angular acceleration value in the currently stored component units.
    pub fn value(&self) -> f64 {
        self.angle.value() / (self.first_time.value() * self.second_time.value())
    }

    /// Returns the angular acceleration value after converting components to requested units.
    pub fn value_in<A2: AngleUnit, U1: TimeUnit, U2: TimeUnit>(&self) -> f64 {
        self.to_units::<A2, U1, U2>().value()
    }

    /// Consumes this angular acceleration and returns its base components.
    pub fn into_parts(self) -> (A, T1, T2) {
        (self.angle, self.first_time, self.second_time)
    }

    /// Returns whether all components are finite.
    pub fn is_finite(&self) -> bool {
        self.angle.is_finite() && self.first_time.is_finite() && self.second_time.is_finite()
    }
}
