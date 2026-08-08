//! Area as two distance components.

use crate::distance::{Distance, DistanceUnit};

/// A rectangular area represented by its width and height.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Area<W: Distance, H: Distance> {
    width: W,
    height: H,
}

impl<W: Distance, H: Distance> Area<W, H> {
    /// Creates an area from its width and height.
    pub const fn new(width: W, height: H) -> Self {
        Self { width, height }
    }

    /// Returns the width component in its original unit.
    pub const fn width(&self) -> &W {
        &self.width
    }

    /// Returns the height component in its original unit.
    pub const fn height(&self) -> &H {
        &self.height
    }

    /// Converts both distance components to requested units.
    pub fn to_units<W2: DistanceUnit, H2: DistanceUnit>(&self) -> Area<W2, H2> {
        Area::new(
            W2::from_meters(self.width.to_meters()),
            H2::from_meters(self.height.to_meters()),
        )
    }

    /// Returns the area value in the currently stored component units.
    pub fn value(&self) -> f64 {
        self.width.value() * self.height.value()
    }

    /// Returns the area value after converting both components to requested units.
    pub fn value_in<W2: DistanceUnit, H2: DistanceUnit>(&self) -> f64 {
        self.to_units::<W2, H2>().value()
    }

    /// Returns whether both components are finite.
    pub fn is_finite(&self) -> bool {
        self.width.is_finite() && self.height.is_finite()
    }
}
