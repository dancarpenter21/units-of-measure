//! Weight as mass times distance divided by two time components.

use crate::acceleration::Acceleration;
use crate::distance::Distance;
use crate::mass::Mass;
use crate::time::Time;

/// A weight or force represented by its base physical components.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Weight<M: Mass, D: Distance, T1: Time, T2: Time> {
    mass: M,
    distance: D,
    first_time: T1,
    second_time: T2,
}

impl<M: Mass, D: Distance, T1: Time, T2: Time> Weight<M, D, T1, T2> {
    /// Creates a weight from mass times distance divided by two time components.
    pub const fn new(mass: M, distance: D, first_time: T1, second_time: T2) -> Self {
        Self {
            mass,
            distance,
            first_time,
            second_time,
        }
    }

    /// Creates weight from a mass and acceleration.
    pub fn from_mass_and_acceleration(mass: M, acceleration: Acceleration<D, T1, T2>) -> Self {
        let (distance, first_time, second_time) = acceleration.into_parts();
        Self::new(mass, distance, first_time, second_time)
    }

    /// Returns the mass component in its original unit.
    pub const fn mass(&self) -> &M {
        &self.mass
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

    /// Consumes this weight and returns its base components.
    pub fn into_parts(self) -> (M, D, T1, T2) {
        (self.mass, self.distance, self.first_time, self.second_time)
    }

    /// Returns whether all components are finite.
    pub fn is_finite(&self) -> bool {
        self.mass.is_finite()
            && self.distance.is_finite()
            && self.first_time.is_finite()
            && self.second_time.is_finite()
    }
}
