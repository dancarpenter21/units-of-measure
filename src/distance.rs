//! Linear distance units and conversions.

const METERS_PER_ANGSTROM: f64 = 1e-10;
const METERS_PER_INCH: f64 = 0.0254;
const METERS_PER_FOOT: f64 = 0.3048;
const METERS_PER_YARD: f64 = 0.9144;
const METERS_PER_MILE: f64 = 1_609.344;
const METERS_PER_NAUTICAL_MILE: f64 = 1_852.0;
const METERS_PER_ASTRONOMICAL_UNIT: f64 = 149_597_870_700.0;
const METERS_PER_LIGHT_YEAR: f64 = 9_460_730_472_580_800.0;
const METERS_PER_PARSEC: f64 = METERS_PER_ASTRONOMICAL_UNIT * 648_000.0 / std::f64::consts::PI;

/// A linear distance that can be converted into every supported distance unit.
///
/// External types only need to implement [`Distance::to_meters`]; all other
/// conversions have default implementations.
pub trait Distance {
    /// Converts this distance to picometers.
    fn to_picometers(&self) -> Picometers {
        Picometers(self.to_meters().0 / 1e-12)
    }

    /// Converts this distance to ångströms.
    fn to_angstroms(&self) -> Angstroms {
        Angstroms(self.to_meters().0 / METERS_PER_ANGSTROM)
    }

    /// Converts this distance to nanometers.
    fn to_nanometers(&self) -> Nanometers {
        Nanometers(self.to_meters().0 / 1e-9)
    }

    /// Converts this distance to micrometers.
    fn to_micrometers(&self) -> Micrometers {
        Micrometers(self.to_meters().0 / 1e-6)
    }

    /// Converts this distance to millimeters.
    fn to_millimeters(&self) -> Millimeters {
        Millimeters(self.to_meters().0 / 1e-3)
    }

    /// Converts this distance to centimeters.
    fn to_centimeters(&self) -> Centimeters {
        Centimeters(self.to_meters().0 / 1e-2)
    }

    /// Converts this distance to decimeters.
    fn to_decimeters(&self) -> Decimeters {
        Decimeters(self.to_meters().0 / 1e-1)
    }

    /// Converts this distance to meters.
    fn to_meters(&self) -> Meters;

    /// Converts this distance to kilometers.
    fn to_kilometers(&self) -> Kilometers {
        Kilometers(self.to_meters().0 / 1e3)
    }

    /// Converts this distance to inches.
    fn to_inches(&self) -> Inches {
        Inches(self.to_meters().0 / METERS_PER_INCH)
    }

    /// Converts this distance to feet.
    fn to_feet(&self) -> Feet {
        Feet(self.to_meters().0 / METERS_PER_FOOT)
    }

    /// Converts this distance to yards.
    fn to_yards(&self) -> Yards {
        Yards(self.to_meters().0 / METERS_PER_YARD)
    }

    /// Converts this distance to statute miles.
    fn to_miles(&self) -> Miles {
        Miles(self.to_meters().0 / METERS_PER_MILE)
    }

    /// Converts this distance to international nautical miles.
    fn to_nautical_miles(&self) -> NauticalMiles {
        NauticalMiles(self.to_meters().0 / METERS_PER_NAUTICAL_MILE)
    }

    /// Converts this distance to astronomical units.
    fn to_astronomical_units(&self) -> AstronomicalUnits {
        AstronomicalUnits(self.to_meters().0 / METERS_PER_ASTRONOMICAL_UNIT)
    }

    /// Converts this distance to light-years.
    fn to_light_years(&self) -> LightYears {
        LightYears(self.to_meters().0 / METERS_PER_LIGHT_YEAR)
    }

    /// Converts this distance to parsecs.
    fn to_parsecs(&self) -> Parsecs {
        Parsecs(self.to_meters().0 / METERS_PER_PARSEC)
    }

    /// Returns this distance divided by another distance.
    fn ratio(&self, rhs: &dyn Distance) -> f64 {
        self.to_meters().0 / rhs.to_meters().0
    }

    /// Returns whether the canonical value is finite.
    fn is_finite(&self) -> bool {
        self.to_meters().0.is_finite()
    }
}

macro_rules! define_distance_unit {
    ($name:ident, $method:ident, $symbol:literal, $factor:expr, $doc:literal) => {
        #[doc = $doc]
        #[derive(Clone, Copy, Debug, Default)]
        pub struct $name(pub f64);

        impl Distance for $name {
            fn to_meters(&self) -> Meters {
                Meters(self.0 * $factor)
            }
        }

        impl_unit_common!($name, Distance, $method, $symbol);
    };
}

define_distance_unit!(
    Picometers,
    to_picometers,
    "pm",
    1e-12,
    "A distance measured in picometers."
);
define_distance_unit!(
    Angstroms,
    to_angstroms,
    "Å",
    METERS_PER_ANGSTROM,
    "A distance measured in ångströms."
);
define_distance_unit!(
    Nanometers,
    to_nanometers,
    "nm",
    1e-9,
    "A distance measured in nanometers."
);
define_distance_unit!(
    Micrometers,
    to_micrometers,
    "µm",
    1e-6,
    "A distance measured in micrometers."
);
define_distance_unit!(
    Millimeters,
    to_millimeters,
    "mm",
    1e-3,
    "A distance measured in millimeters."
);
define_distance_unit!(
    Centimeters,
    to_centimeters,
    "cm",
    1e-2,
    "A distance measured in centimeters."
);
define_distance_unit!(
    Decimeters,
    to_decimeters,
    "dm",
    1e-1,
    "A distance measured in decimeters."
);
define_distance_unit!(
    Meters,
    to_meters,
    "m",
    1.0,
    "A distance measured in meters."
);
define_distance_unit!(
    Kilometers,
    to_kilometers,
    "km",
    1e3,
    "A distance measured in kilometers."
);
define_distance_unit!(
    Inches,
    to_inches,
    "in",
    METERS_PER_INCH,
    "A distance measured in inches."
);
define_distance_unit!(
    Feet,
    to_feet,
    "ft",
    METERS_PER_FOOT,
    "A distance measured in feet."
);
define_distance_unit!(
    Yards,
    to_yards,
    "yd",
    METERS_PER_YARD,
    "A distance measured in yards."
);
define_distance_unit!(
    Miles,
    to_miles,
    "mi",
    METERS_PER_MILE,
    "A distance measured in statute miles."
);
define_distance_unit!(
    NauticalMiles,
    to_nautical_miles,
    "nmi",
    METERS_PER_NAUTICAL_MILE,
    "A distance measured in international nautical miles."
);
define_distance_unit!(
    AstronomicalUnits,
    to_astronomical_units,
    "au",
    METERS_PER_ASTRONOMICAL_UNIT,
    "A distance measured in astronomical units."
);
define_distance_unit!(
    LightYears,
    to_light_years,
    "ly",
    METERS_PER_LIGHT_YEAR,
    "A distance measured in light-years."
);
define_distance_unit!(
    Parsecs,
    to_parsecs,
    "pc",
    METERS_PER_PARSEC,
    "A distance measured in parsecs."
);

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(actual: f64, expected: f64) {
        let scale = actual.abs().max(expected.abs()).max(1.0);
        assert!((actual - expected).abs() <= f64::EPSILON * 16.0 * scale);
    }

    #[test]
    fn reference_conversions_are_correct() {
        assert_close(Feet(1.0).to_meters().0, 0.3048);
        assert_close(Miles(1.0).to_feet().0, 5_280.0);
        assert_close(NauticalMiles(1.0).to_meters().0, 1_852.0);
        assert_close(AstronomicalUnits(1.0).to_meters().0, 149_597_870_700.0);
        assert_close(LightYears(1.0).to_meters().0, 9_460_730_472_580_800.0);
        assert_close(Parsecs(1.0).to_light_years().0, 3.261_563_777_167_434);
    }

    #[test]
    fn mixed_arithmetic_preserves_left_unit() {
        assert_close((Feet(3.0) + Meters(1.0)).0, 6.280_839_895_013_123);
        assert_close((Meters(1.0) + Feet(3.0)).0, 1.9144);
        assert!(Meters(1.0) > Feet(3.0));
        assert_eq!(Meters(0.3048), Feet(1.0));
    }

    #[test]
    fn custom_distance_inherits_conversions() {
        struct Smoots(f64);

        impl Distance for Smoots {
            fn to_meters(&self) -> Meters {
                Meters(self.0 * 1.7018)
            }
        }

        let value: &dyn Distance = &Smoots(1.0);
        assert_close(value.to_feet().0, 5.583_333_333_333_333);
    }
}
