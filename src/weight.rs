//! Weight as mass times distance divided by two time components.

use crate::acceleration::Acceleration;
use crate::distance::{Distance, DistanceUnit};
use crate::mass::{Mass, MassUnit};
use crate::time::{Time, TimeUnit};

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

    /// Converts all base components to requested units.
    pub fn to_units<M2: MassUnit, D2: DistanceUnit, U1: TimeUnit, U2: TimeUnit>(
        &self,
    ) -> Weight<M2, D2, U1, U2> {
        Weight::new(
            M2::from_kilograms(self.mass.to_kilograms()),
            D2::from_meters(self.distance.to_meters()),
            U1::from_seconds(self.first_time.to_seconds()),
            U2::from_seconds(self.second_time.to_seconds()),
        )
    }

    /// Returns the weight value in the currently stored component units.
    pub fn value(&self) -> f64 {
        self.mass.value() * self.distance.value()
            / (self.first_time.value() * self.second_time.value())
    }

    /// Returns the weight value after converting all components to requested units.
    pub fn value_in<M2: MassUnit, D2: DistanceUnit, U1: TimeUnit, U2: TimeUnit>(&self) -> f64 {
        self.to_units::<M2, D2, U1, U2>().value()
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
