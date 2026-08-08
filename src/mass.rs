//! Mass units and conversions.

const KILOGRAMS_PER_GRAIN: f64 = 0.000_064_798_91;
const KILOGRAMS_PER_AVOIRDUPOIS_OUNCE: f64 = 0.028_349_523_125;
const KILOGRAMS_PER_AVOIRDUPOIS_POUND: f64 = 0.453_592_37;
const KILOGRAMS_PER_TROY_OUNCE: f64 = 0.031_103_476_8;
const KILOGRAMS_PER_SLUG: f64 = 14.593_902_937_206_364;
const KILOGRAMS_PER_DALTON: f64 = 1.660_539_068_92e-27;

/// A mass that can be converted into every supported mass unit.
///
/// External types only need to implement [`Mass::to_kilograms`]. Mass does not
/// does not depend on gravity.
pub trait Mass {
    /// Converts this mass to nanograms.
    fn to_nanograms(&self) -> Nanograms {
        Nanograms(self.to_kilograms().0 / 1e-12)
    }

    /// Converts this mass to micrograms.
    fn to_micrograms(&self) -> Micrograms {
        Micrograms(self.to_kilograms().0 / 1e-9)
    }

    /// Converts this mass to milligrams.
    fn to_milligrams(&self) -> Milligrams {
        Milligrams(self.to_kilograms().0 / 1e-6)
    }

    /// Converts this mass to grams.
    fn to_grams(&self) -> Grams {
        Grams(self.to_kilograms().0 / 1e-3)
    }

    /// Converts this mass to kilograms.
    fn to_kilograms(&self) -> Kilograms;

    /// Converts this mass to metric tonnes.
    fn to_metric_tonnes(&self) -> MetricTonnes {
        MetricTonnes(self.to_kilograms().0 / 1_000.0)
    }

    /// Converts this mass to carats.
    fn to_carats(&self) -> Carats {
        Carats(self.to_kilograms().0 / 0.0002)
    }

    /// Converts this mass to avoirdupois grains.
    fn to_grains(&self) -> Grains {
        Grains(self.to_kilograms().0 / KILOGRAMS_PER_GRAIN)
    }

    /// Converts this mass to avoirdupois ounces-mass.
    fn to_ounces(&self) -> Ounces {
        Ounces(self.to_kilograms().0 / KILOGRAMS_PER_AVOIRDUPOIS_OUNCE)
    }

    /// Converts this mass to avoirdupois pounds-mass.
    fn to_pounds(&self) -> Pounds {
        Pounds(self.to_kilograms().0 / KILOGRAMS_PER_AVOIRDUPOIS_POUND)
    }

    /// Converts this mass to stones of 14 pounds-mass.
    fn to_stones(&self) -> Stones {
        Stones(self.to_kilograms().0 / (14.0 * KILOGRAMS_PER_AVOIRDUPOIS_POUND))
    }

    /// Converts this mass to US short tons.
    fn to_short_tons(&self) -> ShortTons {
        ShortTons(self.to_kilograms().0 / (2_000.0 * KILOGRAMS_PER_AVOIRDUPOIS_POUND))
    }

    /// Converts this mass to imperial long tons.
    fn to_long_tons(&self) -> LongTons {
        LongTons(self.to_kilograms().0 / (2_240.0 * KILOGRAMS_PER_AVOIRDUPOIS_POUND))
    }

    /// Converts this mass to troy ounces.
    fn to_troy_ounces(&self) -> TroyOunces {
        TroyOunces(self.to_kilograms().0 / KILOGRAMS_PER_TROY_OUNCE)
    }

    /// Converts this mass to slugs.
    fn to_slugs(&self) -> Slugs {
        Slugs(self.to_kilograms().0 / KILOGRAMS_PER_SLUG)
    }

    /// Converts this mass to daltons.
    fn to_daltons(&self) -> Daltons {
        Daltons(self.to_kilograms().0 / KILOGRAMS_PER_DALTON)
    }

    /// Converts this mass to unified atomic mass units.
    fn to_unified_atomic_mass_units(&self) -> UnifiedAtomicMassUnits {
        UnifiedAtomicMassUnits(self.to_kilograms().0 / KILOGRAMS_PER_DALTON)
    }

    /// Returns this mass divided by another mass.
    fn ratio(&self, rhs: &dyn Mass) -> f64 {
        self.to_kilograms().0 / rhs.to_kilograms().0
    }

    /// Returns whether the canonical value is finite.
    fn is_finite(&self) -> bool {
        self.to_kilograms().0.is_finite()
    }
}

macro_rules! define_mass_unit {
    ($name:ident, $method:ident, $symbol:literal, $factor:expr, $doc:literal) => {
        #[doc = $doc]
        #[derive(Clone, Copy, Debug, Default)]
        pub struct $name(pub f64);

        impl Mass for $name {
            fn to_kilograms(&self) -> Kilograms {
                Kilograms(self.0 * $factor)
            }
        }

        impl_unit_common!($name, Mass, $method, $symbol);
    };
}

define_mass_unit!(
    Nanograms,
    to_nanograms,
    "ng",
    1e-12,
    "A mass measured in nanograms."
);
define_mass_unit!(
    Micrograms,
    to_micrograms,
    "µg",
    1e-9,
    "A mass measured in micrograms."
);
define_mass_unit!(
    Milligrams,
    to_milligrams,
    "mg",
    1e-6,
    "A mass measured in milligrams."
);
define_mass_unit!(Grams, to_grams, "g", 1e-3, "A mass measured in grams.");
define_mass_unit!(
    Kilograms,
    to_kilograms,
    "kg",
    1.0,
    "A mass measured in kilograms."
);
define_mass_unit!(
    MetricTonnes,
    to_metric_tonnes,
    "t",
    1_000.0,
    "A mass measured in metric tonnes."
);
define_mass_unit!(
    Carats,
    to_carats,
    "ct",
    0.0002,
    "A mass measured in carats."
);
define_mass_unit!(
    Grains,
    to_grains,
    "gr",
    KILOGRAMS_PER_GRAIN,
    "A mass measured in avoirdupois grains."
);
define_mass_unit!(
    Ounces,
    to_ounces,
    "oz",
    KILOGRAMS_PER_AVOIRDUPOIS_OUNCE,
    "A mass measured in avoirdupois ounces-mass."
);
define_mass_unit!(
    Pounds,
    to_pounds,
    "lb",
    KILOGRAMS_PER_AVOIRDUPOIS_POUND,
    "A mass measured in avoirdupois pounds-mass."
);
define_mass_unit!(
    Stones,
    to_stones,
    "st",
    14.0 * KILOGRAMS_PER_AVOIRDUPOIS_POUND,
    "A mass measured in stones of 14 pounds-mass."
);
define_mass_unit!(
    ShortTons,
    to_short_tons,
    "short ton",
    2_000.0 * KILOGRAMS_PER_AVOIRDUPOIS_POUND,
    "A mass measured in US short tons."
);
define_mass_unit!(
    LongTons,
    to_long_tons,
    "long ton",
    2_240.0 * KILOGRAMS_PER_AVOIRDUPOIS_POUND,
    "A mass measured in imperial long tons."
);
define_mass_unit!(
    TroyOunces,
    to_troy_ounces,
    "oz t",
    KILOGRAMS_PER_TROY_OUNCE,
    "A mass measured in troy ounces."
);
define_mass_unit!(
    Slugs,
    to_slugs,
    "slug",
    KILOGRAMS_PER_SLUG,
    "A mass measured in slugs."
);
define_mass_unit!(
    Daltons,
    to_daltons,
    "Da",
    KILOGRAMS_PER_DALTON,
    "A mass measured in daltons using the 2022 CODATA central value."
);
define_mass_unit!(
    UnifiedAtomicMassUnits,
    to_unified_atomic_mass_units,
    "u",
    KILOGRAMS_PER_DALTON,
    "A mass measured in unified atomic mass units using the 2022 CODATA central value."
);

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(actual: f64, expected: f64) {
        let scale = actual.abs().max(expected.abs()).max(1.0);
        assert!((actual - expected).abs() <= f64::EPSILON * 16.0 * scale);
    }

    #[test]
    fn customary_and_scientific_units_convert() {
        assert_close(Pounds(1.0).to_kilograms().0, 0.453_592_37);
        assert_close(Ounces(16.0).to_pounds().0, 1.0);
        assert_close(Stones(1.0).to_pounds().0, 14.0);
        assert_close(ShortTons(1.0).to_pounds().0, 2_000.0);
        assert_close(LongTons(1.0).to_pounds().0, 2_240.0);
        assert_close(TroyOunces(1.0).to_grams().0, 31.103_476_8);
        assert_close(Daltons(1.0).to_kilograms().0, KILOGRAMS_PER_DALTON);
        assert_eq!(Daltons(1.0), UnifiedAtomicMassUnits(1.0));
    }
}
