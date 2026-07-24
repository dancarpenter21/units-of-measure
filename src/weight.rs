//! Weight and force units.

use crate::acceleration::{Acceleration, MetersPerSecondSquared, STANDARD_GRAVITY};
use crate::mass::{Kilograms as MassKilograms, Mass};

const NEWTONS_PER_POUND_FORCE: f64 = 4.448_221_615_260_5;
const NEWTONS_PER_KILOGRAM_FORCE: f64 = 9.806_65;

/// A weight or force that can be converted into every supported force unit.
///
/// External types only need to implement [`Weight::to_newtons`]. The
/// `Kilograms`, `Pounds`, and `Ounces` in this module are kilogram-force,
/// pound-force, and ounce-force respectively.
pub trait Weight {
    /// Converts this force to dynes.
    fn to_dynes(&self) -> Dynes {
        Dynes(self.to_newtons().0 / 1e-5)
    }

    /// Converts this force to newtons.
    fn to_newtons(&self) -> Newtons;

    /// Converts this force to kilonewtons.
    fn to_kilonewtons(&self) -> Kilonewtons {
        Kilonewtons(self.to_newtons().0 / 1_000.0)
    }

    /// Converts this force to ounces-force.
    fn to_ounces(&self) -> Ounces {
        Ounces(self.to_newtons().0 / (NEWTONS_PER_POUND_FORCE / 16.0))
    }

    /// Converts this force to pounds-force.
    fn to_pounds(&self) -> Pounds {
        Pounds(self.to_newtons().0 / NEWTONS_PER_POUND_FORCE)
    }

    /// Converts this force to stone-force.
    fn to_stones(&self) -> Stones {
        Stones(self.to_newtons().0 / (14.0 * NEWTONS_PER_POUND_FORCE))
    }

    /// Converts this force to kilograms-force.
    fn to_kilograms(&self) -> Kilograms {
        Kilograms(self.to_newtons().0 / NEWTONS_PER_KILOGRAM_FORCE)
    }

    /// Converts this force to kips of 1,000 pounds-force.
    fn to_kips(&self) -> Kips {
        Kips(self.to_newtons().0 / (1_000.0 * NEWTONS_PER_POUND_FORCE))
    }

    /// Returns this force divided by another force.
    fn ratio(&self, rhs: &dyn Weight) -> f64 {
        self.to_newtons().0 / rhs.to_newtons().0
    }

    /// Computes acceleration applied to a mass.
    fn acceleration_of(&self, mass: &dyn Mass) -> MetersPerSecondSquared {
        MetersPerSecondSquared(self.to_newtons().0 / mass.to_kilograms().0)
    }

    /// Computes the mass producing this weight under an acceleration.
    fn mass_at(&self, acceleration: &dyn Acceleration) -> MassKilograms {
        MassKilograms(self.to_newtons().0 / acceleration.to_meters_per_second_squared().0)
    }

    /// Computes the mass producing this weight under standard gravity.
    fn mass_at_standard_gravity(&self) -> MassKilograms {
        self.mass_at(&STANDARD_GRAVITY)
    }

    /// Returns whether the canonical value is finite.
    fn is_finite(&self) -> bool {
        self.to_newtons().0.is_finite()
    }
}

macro_rules! define_weight_unit {
    ($name:ident, $method:ident, $symbol:literal, $factor:expr, $doc:literal) => {
        #[doc = $doc]
        #[derive(Clone, Copy, Debug, Default)]
        pub struct $name(pub f64);

        impl Weight for $name {
            fn to_newtons(&self) -> Newtons {
                Newtons(self.0 * $factor)
            }
        }

        impl_unit_common!($name, Weight, $method, $symbol);
    };
}

define_weight_unit!(Dynes, to_dynes, "dyn", 1e-5, "A force measured in dynes.");
define_weight_unit!(
    Newtons,
    to_newtons,
    "N",
    1.0,
    "A force measured in newtons."
);
define_weight_unit!(
    Kilonewtons,
    to_kilonewtons,
    "kN",
    1_000.0,
    "A force measured in kilonewtons."
);
define_weight_unit!(
    Ounces,
    to_ounces,
    "ozf",
    NEWTONS_PER_POUND_FORCE / 16.0,
    "A force measured in ounces-force."
);
define_weight_unit!(
    Pounds,
    to_pounds,
    "lbf",
    NEWTONS_PER_POUND_FORCE,
    "A force measured in pounds-force."
);
define_weight_unit!(
    Stones,
    to_stones,
    "stf",
    14.0 * NEWTONS_PER_POUND_FORCE,
    "A force measured in stone-force."
);
define_weight_unit!(
    Kilograms,
    to_kilograms,
    "kgf",
    NEWTONS_PER_KILOGRAM_FORCE,
    "A colloquial kilogram weight measured in kilograms-force."
);
define_weight_unit!(
    Kips,
    to_kips,
    "kip",
    1_000.0 * NEWTONS_PER_POUND_FORCE,
    "A force measured in kips of 1,000 pounds-force."
);

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(actual: f64, expected: f64) {
        let scale = actual.abs().max(expected.abs()).max(1.0);
        assert!((actual - expected).abs() <= f64::EPSILON * 16.0 * scale);
    }

    #[test]
    fn force_units_convert() {
        assert_close(Dynes(100_000.0).to_newtons().0, 1.0);
        assert_close(Pounds(1.0).to_newtons().0, NEWTONS_PER_POUND_FORCE);
        assert_close(Ounces(16.0).to_pounds().0, 1.0);
        assert_close(Stones(1.0).to_pounds().0, 14.0);
        assert_close(Kilograms(1.0).to_newtons().0, 9.80665);
        assert_close(Kips(1.0).to_pounds().0, 1_000.0);
    }
}
