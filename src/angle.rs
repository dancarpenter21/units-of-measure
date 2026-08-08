//! Plane angle units and conversions.

const RADIANS_PER_DEGREE: f64 = std::f64::consts::PI / 180.0;

/// A plane angle that can be converted into every supported angle unit.
///
/// External types only need to implement [`Angle::to_radians`].
pub trait Angle {
    /// Converts this angle to degrees.
    fn to_degrees(&self) -> Degrees {
        Degrees(self.to_radians().0 / RADIANS_PER_DEGREE)
    }

    /// Converts this angle to radians.
    fn to_radians(&self) -> Radians;

    /// Returns this angle divided by another angle.
    fn ratio(&self, rhs: &dyn Angle) -> f64 {
        self.to_radians().0 / rhs.to_radians().0
    }

    /// Returns whether the canonical value is finite.
    fn is_finite(&self) -> bool {
        self.to_radians().0.is_finite()
    }
}

macro_rules! define_angle_unit {
    ($name:ident, $method:ident, $symbol:literal, $factor:expr, $doc:literal) => {
        #[doc = $doc]
        #[derive(Clone, Copy, Debug, Default)]
        pub struct $name(pub f64);

        impl Angle for $name {
            fn to_radians(&self) -> Radians {
                Radians(self.0 * $factor)
            }
        }

        impl_unit_common!($name, Angle, $method, $symbol);
    };
}

define_angle_unit!(
    Degrees,
    to_degrees,
    "°",
    RADIANS_PER_DEGREE,
    "A plane angle measured in degrees."
);
define_angle_unit!(
    Radians,
    to_radians,
    "rad",
    1.0,
    "A plane angle measured in radians."
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn degrees_and_radians_convert() {
        assert!((Degrees::new(180.0).to_radians().value() - std::f64::consts::PI).abs() < 1e-12);
        assert!((Radians::new(std::f64::consts::PI).to_degrees().value() - 180.0).abs() < 1e-12);
    }
}
