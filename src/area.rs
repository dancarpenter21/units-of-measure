//! Area units and conversions.

use crate::distance::{Distance, Meters};

const SQUARE_METERS_PER_SQUARE_ANGSTROM: f64 = 1e-20;
const SQUARE_METERS_PER_SQUARE_INCH: f64 = 0.000_645_16;
const SQUARE_METERS_PER_SQUARE_FOOT: f64 = 0.092_903_04;
const SQUARE_METERS_PER_SQUARE_YARD: f64 = 0.836_127_36;
const SQUARE_METERS_PER_SQUARE_MILE: f64 = 2_589_988.110_336;
const SQUARE_METERS_PER_SQUARE_NAUTICAL_MILE: f64 = 3_429_904.0;
const SQUARE_METERS_PER_SQUARE_ASTRONOMICAL_UNIT: f64 = 149_597_870_700.0 * 149_597_870_700.0;
const SQUARE_METERS_PER_SQUARE_LIGHT_YEAR: f64 = 9_460_730_472_580_800.0 * 9_460_730_472_580_800.0;
const METERS_PER_PARSEC: f64 = 149_597_870_700.0 * 648_000.0 / std::f64::consts::PI;
const SQUARE_METERS_PER_SQUARE_PARSEC: f64 = METERS_PER_PARSEC * METERS_PER_PARSEC;
const SQUARE_METERS_PER_ACRE: f64 = 4_046.856_422_4;

/// An area that can be converted into every supported area unit.
///
/// External types only need to implement [`Area::to_square_meters`].
pub trait Area {
    /// Converts this area to square picometers.
    fn to_square_picometers(&self) -> SquarePicometers {
        SquarePicometers(self.to_square_meters().0 / 1e-24)
    }

    /// Converts this area to square ångströms.
    fn to_square_angstroms(&self) -> SquareAngstroms {
        SquareAngstroms(self.to_square_meters().0 / SQUARE_METERS_PER_SQUARE_ANGSTROM)
    }

    /// Converts this area to square nanometers.
    fn to_square_nanometers(&self) -> SquareNanometers {
        SquareNanometers(self.to_square_meters().0 / 1e-18)
    }

    /// Converts this area to square micrometers.
    fn to_square_micrometers(&self) -> SquareMicrometers {
        SquareMicrometers(self.to_square_meters().0 / 1e-12)
    }

    /// Converts this area to square millimeters.
    fn to_square_millimeters(&self) -> SquareMillimeters {
        SquareMillimeters(self.to_square_meters().0 / 1e-6)
    }

    /// Converts this area to square centimeters.
    fn to_square_centimeters(&self) -> SquareCentimeters {
        SquareCentimeters(self.to_square_meters().0 / 1e-4)
    }

    /// Converts this area to square decimeters.
    fn to_square_decimeters(&self) -> SquareDecimeters {
        SquareDecimeters(self.to_square_meters().0 / 1e-2)
    }

    /// Converts this area to square meters.
    fn to_square_meters(&self) -> SquareMeters;

    /// Converts this area to square kilometers.
    fn to_square_kilometers(&self) -> SquareKilometers {
        SquareKilometers(self.to_square_meters().0 / 1e6)
    }

    /// Converts this area to square inches.
    fn to_square_inches(&self) -> SquareInches {
        SquareInches(self.to_square_meters().0 / SQUARE_METERS_PER_SQUARE_INCH)
    }

    /// Converts this area to square feet.
    fn to_square_feet(&self) -> SquareFeet {
        SquareFeet(self.to_square_meters().0 / SQUARE_METERS_PER_SQUARE_FOOT)
    }

    /// Converts this area to square yards.
    fn to_square_yards(&self) -> SquareYards {
        SquareYards(self.to_square_meters().0 / SQUARE_METERS_PER_SQUARE_YARD)
    }

    /// Converts this area to square statute miles.
    fn to_square_miles(&self) -> SquareMiles {
        SquareMiles(self.to_square_meters().0 / SQUARE_METERS_PER_SQUARE_MILE)
    }

    /// Converts this area to square nautical miles.
    fn to_square_nautical_miles(&self) -> SquareNauticalMiles {
        SquareNauticalMiles(self.to_square_meters().0 / SQUARE_METERS_PER_SQUARE_NAUTICAL_MILE)
    }

    /// Converts this area to square astronomical units.
    fn to_square_astronomical_units(&self) -> SquareAstronomicalUnits {
        SquareAstronomicalUnits(
            self.to_square_meters().0 / SQUARE_METERS_PER_SQUARE_ASTRONOMICAL_UNIT,
        )
    }

    /// Converts this area to square light-years.
    fn to_square_light_years(&self) -> SquareLightYears {
        SquareLightYears(self.to_square_meters().0 / SQUARE_METERS_PER_SQUARE_LIGHT_YEAR)
    }

    /// Converts this area to square parsecs.
    fn to_square_parsecs(&self) -> SquareParsecs {
        SquareParsecs(self.to_square_meters().0 / SQUARE_METERS_PER_SQUARE_PARSEC)
    }

    /// Converts this area to ares.
    fn to_ares(&self) -> Ares {
        Ares(self.to_square_meters().0 / 100.0)
    }

    /// Converts this area to hectares.
    fn to_hectares(&self) -> Hectares {
        Hectares(self.to_square_meters().0 / 10_000.0)
    }

    /// Converts this area to acres.
    fn to_acres(&self) -> Acres {
        Acres(self.to_square_meters().0 / SQUARE_METERS_PER_ACRE)
    }

    /// Converts this area to barns.
    fn to_barns(&self) -> Barns {
        Barns(self.to_square_meters().0 / 1e-28)
    }

    /// Returns this area divided by another area.
    fn ratio(&self, rhs: &dyn Area) -> f64 {
        self.to_square_meters().0 / rhs.to_square_meters().0
    }

    /// Divides this area by a width and returns the remaining length.
    fn length_for_width(&self, width: &dyn Distance) -> Meters {
        Meters(self.to_square_meters().0 / width.to_meters().0)
    }

    /// Returns whether the canonical value is finite.
    fn is_finite(&self) -> bool {
        self.to_square_meters().0.is_finite()
    }
}

macro_rules! define_area_unit {
    ($name:ident, $method:ident, $symbol:literal, $factor:expr, $doc:literal) => {
        #[doc = $doc]
        #[derive(Clone, Copy, Debug, Default)]
        pub struct $name(pub f64);

        impl Area for $name {
            fn to_square_meters(&self) -> SquareMeters {
                SquareMeters(self.0 * $factor)
            }
        }

        impl_unit_common!($name, Area, $method, $symbol);
    };
}

define_area_unit!(
    SquarePicometers,
    to_square_picometers,
    "pm²",
    1e-24,
    "An area measured in square picometers."
);
define_area_unit!(
    SquareAngstroms,
    to_square_angstroms,
    "Å²",
    SQUARE_METERS_PER_SQUARE_ANGSTROM,
    "An area measured in square ångströms."
);
define_area_unit!(
    SquareNanometers,
    to_square_nanometers,
    "nm²",
    1e-18,
    "An area measured in square nanometers."
);
define_area_unit!(
    SquareMicrometers,
    to_square_micrometers,
    "µm²",
    1e-12,
    "An area measured in square micrometers."
);
define_area_unit!(
    SquareMillimeters,
    to_square_millimeters,
    "mm²",
    1e-6,
    "An area measured in square millimeters."
);
define_area_unit!(
    SquareCentimeters,
    to_square_centimeters,
    "cm²",
    1e-4,
    "An area measured in square centimeters."
);
define_area_unit!(
    SquareDecimeters,
    to_square_decimeters,
    "dm²",
    1e-2,
    "An area measured in square decimeters."
);
define_area_unit!(
    SquareMeters,
    to_square_meters,
    "m²",
    1.0,
    "An area measured in square meters."
);
define_area_unit!(
    SquareKilometers,
    to_square_kilometers,
    "km²",
    1e6,
    "An area measured in square kilometers."
);
define_area_unit!(
    SquareInches,
    to_square_inches,
    "in²",
    SQUARE_METERS_PER_SQUARE_INCH,
    "An area measured in square inches."
);
define_area_unit!(
    SquareFeet,
    to_square_feet,
    "ft²",
    SQUARE_METERS_PER_SQUARE_FOOT,
    "An area measured in square feet."
);
define_area_unit!(
    SquareYards,
    to_square_yards,
    "yd²",
    SQUARE_METERS_PER_SQUARE_YARD,
    "An area measured in square yards."
);
define_area_unit!(
    SquareMiles,
    to_square_miles,
    "mi²",
    SQUARE_METERS_PER_SQUARE_MILE,
    "An area measured in square statute miles."
);
define_area_unit!(
    SquareNauticalMiles,
    to_square_nautical_miles,
    "nmi²",
    SQUARE_METERS_PER_SQUARE_NAUTICAL_MILE,
    "An area measured in square nautical miles."
);
define_area_unit!(
    SquareAstronomicalUnits,
    to_square_astronomical_units,
    "au²",
    SQUARE_METERS_PER_SQUARE_ASTRONOMICAL_UNIT,
    "An area measured in square astronomical units."
);
define_area_unit!(
    SquareLightYears,
    to_square_light_years,
    "ly²",
    SQUARE_METERS_PER_SQUARE_LIGHT_YEAR,
    "An area measured in square light-years."
);
define_area_unit!(
    SquareParsecs,
    to_square_parsecs,
    "pc²",
    SQUARE_METERS_PER_SQUARE_PARSEC,
    "An area measured in square parsecs."
);
define_area_unit!(Ares, to_ares, "a", 100.0, "An area measured in ares.");
define_area_unit!(
    Hectares,
    to_hectares,
    "ha",
    10_000.0,
    "An area measured in hectares."
);
define_area_unit!(
    Acres,
    to_acres,
    "ac",
    SQUARE_METERS_PER_ACRE,
    "An area measured in acres."
);
define_area_unit!(Barns, to_barns, "b", 1e-28, "An area measured in barns.");

#[cfg(test)]
mod tests {
    use super::*;
    use crate::distance::Feet;

    fn assert_close(actual: f64, expected: f64) {
        let scale = actual.abs().max(expected.abs()).max(1.0);
        assert!((actual - expected).abs() <= f64::EPSILON * 16.0 * scale);
    }

    #[test]
    fn land_and_scientific_units_convert() {
        assert_close(Acres(1.0).to_square_feet().0, 43_560.0);
        assert_close(Hectares(1.0).to_square_meters().0, 10_000.0);
        assert_close(Ares(1.0).to_square_meters().0, 100.0);
        assert_close(Barns(1.0).to_square_meters().0, 1e-28);
    }

    #[test]
    fn area_divided_by_width_is_length() {
        assert_close(
            SquareFeet(120.0).length_for_width(&Feet(10.0)).to_feet().0,
            12.0,
        );
    }

    #[test]
    fn mixed_arithmetic_preserves_left_unit() {
        assert_close((Acres(1.0) + Hectares(1.0)).0, 3.471_053_814_671_653);
        assert_eq!(SquareMeters(1.0), SquareFeet(10.763_910_416_709_722));
    }
}
