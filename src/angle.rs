//! Plane angle units and conversions.

const RADIANS_PER_DEGREE: f64 = std::f64::consts::PI / 180.0;

/// A plane angle that can be converted into every supported angle unit.
///
/// External types only need to implement [`Angle::to_radians`].
pub trait Angle {
    /// Returns the numeric value expressed in this angle's own unit.
    ///
    /// Custom angle types that retain a non-canonical numeric value should
    /// override this method.
    fn value(&self) -> f64 {
        self.to_radians().value()
    }

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

/// An angle unit that can be used as the destination of a conversion.
pub trait AngleUnit: Angle {
    /// Creates this unit from a value expressed in radians.
    fn from_radians(radians: Radians) -> Self;
}

macro_rules! define_angle_unit {
    ($name:ident, $method:ident, $symbol:literal, $factor:expr, $doc:literal) => {
        #[doc = $doc]
        #[derive(Clone, Copy, Debug, Default)]
        pub struct $name(pub f64);

        impl Angle for $name {
            fn value(&self) -> f64 {
                self.0
            }

            fn to_radians(&self) -> Radians {
                Radians(self.0 * $factor)
            }
        }

        impl AngleUnit for $name {
            fn from_radians(radians: Radians) -> Self {
                Self(radians.0 / $factor)
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
