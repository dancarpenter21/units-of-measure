//! Area as two distance components.

use crate::distance::Distance;

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

    /// Returns whether both components are finite.
    pub fn is_finite(&self) -> bool {
        self.width.is_finite() && self.height.is_finite()
    }
}
