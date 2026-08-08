//! Torque as mass times two distance components divided by two time components.

use crate::distance::Distance;
use crate::mass::Mass;
use crate::time::Time;
use crate::weight::Weight;

/// A torque represented by its base physical components.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Torque<M: Mass, D1: Distance, D2: Distance, T1: Time, T2: Time> {
    mass: M,
    first_distance: D1,
    second_distance: D2,
    first_time: T1,
    second_time: T2,
}

impl<M: Mass, D1: Distance, D2: Distance, T1: Time, T2: Time> Torque<M, D1, D2, T1, T2> {
    /// Creates torque from mass times two distances divided by two time components.
    pub const fn new(
        mass: M,
        first_distance: D1,
        second_distance: D2,
        first_time: T1,
        second_time: T2,
    ) -> Self {
        Self {
            mass,
            first_distance,
            second_distance,
            first_time,
            second_time,
        }
    }

    /// Creates torque from a weight and perpendicular lever-arm distance.
    pub fn from_weight_and_distance(weight: Weight<M, D1, T1, T2>, second_distance: D2) -> Self {
        let (mass, first_distance, first_time, second_time) = weight.into_parts();
        Self::new(
            mass,
            first_distance,
            second_distance,
            first_time,
            second_time,
        )
    }

    /// Returns the mass component in its original unit.
    pub const fn mass(&self) -> &M {
        &self.mass
    }

    /// Returns the first distance component in its original unit.
    pub const fn first_distance(&self) -> &D1 {
        &self.first_distance
    }

    /// Returns the second distance component in its original unit.
    pub const fn second_distance(&self) -> &D2 {
        &self.second_distance
    }

    /// Returns the first time component in its original unit.
    pub const fn first_time(&self) -> &T1 {
        &self.first_time
    }

    /// Returns the second time component in its original unit.
    pub const fn second_time(&self) -> &T2 {
        &self.second_time
    }

    /// Returns whether all components are finite.
    pub fn is_finite(&self) -> bool {
        self.mass.is_finite()
            && self.first_distance.is_finite()
            && self.second_distance.is_finite()
            && self.first_time.is_finite()
            && self.second_time.is_finite()
    }
}
