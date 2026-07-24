#![cfg(feature = "serde")]

use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

fn assert_json_and_yaml_round_trip<T>(value: T)
where
    T: Serialize + DeserializeOwned + PartialEq + Debug,
{
    let json = serde_json::to_string(&value).expect("unit should serialize to JSON");
    assert_eq!(json, "12.5");
    let from_json: T = serde_json::from_str(&json).expect("unit should deserialize from JSON");
    assert_eq!(from_json, value);

    let yaml = serde_yaml::to_string(&value).expect("unit should serialize to YAML");
    let from_yaml: T = serde_yaml::from_str(&yaml).expect("unit should deserialize from YAML");
    assert_eq!(from_yaml, value);
}

macro_rules! assert_units_round_trip {
    ($($unit:path),+ $(,)?) => {
        $(assert_json_and_yaml_round_trip(<$unit>::new(12.5));)+
    };
}

#[test]
fn every_distance_and_area_unit_round_trips() {
    assert_units_round_trip!(
        units_of_measure::distance::Picometers,
        units_of_measure::distance::Angstroms,
        units_of_measure::distance::Nanometers,
        units_of_measure::distance::Micrometers,
        units_of_measure::distance::Millimeters,
        units_of_measure::distance::Centimeters,
        units_of_measure::distance::Decimeters,
        units_of_measure::distance::Meters,
        units_of_measure::distance::Kilometers,
        units_of_measure::distance::Inches,
        units_of_measure::distance::Feet,
        units_of_measure::distance::Yards,
        units_of_measure::distance::Miles,
        units_of_measure::distance::NauticalMiles,
        units_of_measure::distance::AstronomicalUnits,
        units_of_measure::distance::LightYears,
        units_of_measure::distance::Parsecs,
        units_of_measure::area::SquarePicometers,
        units_of_measure::area::SquareAngstroms,
        units_of_measure::area::SquareNanometers,
        units_of_measure::area::SquareMicrometers,
        units_of_measure::area::SquareMillimeters,
        units_of_measure::area::SquareCentimeters,
        units_of_measure::area::SquareDecimeters,
        units_of_measure::area::SquareMeters,
        units_of_measure::area::SquareKilometers,
        units_of_measure::area::SquareInches,
        units_of_measure::area::SquareFeet,
        units_of_measure::area::SquareYards,
        units_of_measure::area::SquareMiles,
        units_of_measure::area::SquareNauticalMiles,
        units_of_measure::area::SquareAstronomicalUnits,
        units_of_measure::area::SquareLightYears,
        units_of_measure::area::SquareParsecs,
        units_of_measure::area::Ares,
        units_of_measure::area::Hectares,
        units_of_measure::area::Acres,
        units_of_measure::area::Barns,
    );
}

#[test]
fn every_time_speed_and_acceleration_unit_round_trips() {
    assert_units_round_trip!(
        units_of_measure::time::Femtoseconds,
        units_of_measure::time::Picoseconds,
        units_of_measure::time::Nanoseconds,
        units_of_measure::time::Microseconds,
        units_of_measure::time::Milliseconds,
        units_of_measure::time::Seconds,
        units_of_measure::time::Minutes,
        units_of_measure::time::Hours,
        units_of_measure::time::Days,
        units_of_measure::time::Weeks,
        units_of_measure::time::MeanGregorianMonths,
        units_of_measure::time::MeanGregorianYears,
        units_of_measure::time::JulianYears,
        units_of_measure::speed::CentimetersPerSecond,
        units_of_measure::speed::MetersPerSecond,
        units_of_measure::speed::KilometersPerSecond,
        units_of_measure::speed::KilometersPerHour,
        units_of_measure::speed::MilesPerHour,
        units_of_measure::speed::FeetPerSecond,
        units_of_measure::speed::FeetPerMinute,
        units_of_measure::speed::Knots,
        units_of_measure::speed::Mach,
        units_of_measure::acceleration::MetersPerSecondSquared,
        units_of_measure::acceleration::CentimetersPerSecondSquared,
        units_of_measure::acceleration::FeetPerSecondSquared,
        units_of_measure::acceleration::KilometersPerHourPerSecond,
        units_of_measure::acceleration::MilesPerHourPerSecond,
        units_of_measure::acceleration::Gals,
        units_of_measure::acceleration::Milligals,
        units_of_measure::acceleration::StandardGravities,
    );
}

#[test]
fn every_mass_weight_and_frequency_unit_round_trips() {
    assert_units_round_trip!(
        units_of_measure::mass::Nanograms,
        units_of_measure::mass::Micrograms,
        units_of_measure::mass::Milligrams,
        units_of_measure::mass::Grams,
        units_of_measure::mass::Kilograms,
        units_of_measure::mass::MetricTonnes,
        units_of_measure::mass::Carats,
        units_of_measure::mass::Grains,
        units_of_measure::mass::Ounces,
        units_of_measure::mass::Pounds,
        units_of_measure::mass::Stones,
        units_of_measure::mass::ShortTons,
        units_of_measure::mass::LongTons,
        units_of_measure::mass::TroyOunces,
        units_of_measure::mass::Slugs,
        units_of_measure::mass::Daltons,
        units_of_measure::mass::UnifiedAtomicMassUnits,
        units_of_measure::weight::Dynes,
        units_of_measure::weight::Newtons,
        units_of_measure::weight::Kilonewtons,
        units_of_measure::weight::Ounces,
        units_of_measure::weight::Pounds,
        units_of_measure::weight::Stones,
        units_of_measure::weight::Kilograms,
        units_of_measure::weight::Kips,
        units_of_measure::frequency::Microhertz,
        units_of_measure::frequency::Millihertz,
        units_of_measure::frequency::Hertz,
        units_of_measure::frequency::Kilohertz,
        units_of_measure::frequency::Megahertz,
        units_of_measure::frequency::Gigahertz,
        units_of_measure::frequency::Terahertz,
        units_of_measure::frequency::Petahertz,
        units_of_measure::frequency::RevolutionsPerSecond,
        units_of_measure::frequency::RevolutionsPerMinute,
        units_of_measure::frequency::BeatsPerMinute,
        units_of_measure::frequency::RadiansPerSecond,
    );
}

#[test]
fn deserialization_target_determines_the_unit_type() {
    let meters: units_of_measure::distance::Meters =
        serde_json::from_str("3.5").expect("meters should deserialize");
    let seconds: units_of_measure::time::Seconds =
        serde_yaml::from_str("3.5").expect("seconds should deserialize");

    assert_eq!(meters.0, 3.5);
    assert_eq!(seconds.0, 3.5);
}
