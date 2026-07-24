use units_of_measure::{
    acceleration::{
        Acceleration, CentimetersPerSecondSquared, FeetPerSecondSquared, Gals,
        KilometersPerHourPerSecond, MetersPerSecondSquared, MilesPerHourPerSecond, Milligals,
        StandardGravities,
    },
    area::{
        Acres, Area, Ares, Barns, Hectares, SquareAngstroms, SquareAstronomicalUnits,
        SquareCentimeters, SquareDecimeters, SquareFeet, SquareInches, SquareKilometers,
        SquareLightYears, SquareMeters, SquareMicrometers, SquareMiles, SquareMillimeters,
        SquareNanometers, SquareNauticalMiles, SquareParsecs, SquarePicometers, SquareYards,
    },
    distance::{
        Angstroms, AstronomicalUnits, Centimeters, Decimeters, Distance, Feet, Inches, Kilometers,
        LightYears, Meters, Micrometers, Miles, Millimeters, Nanometers, NauticalMiles, Parsecs,
        Picometers, Yards,
    },
    frequency::{
        BeatsPerMinute, Frequency, Gigahertz, Hertz, Kilohertz, Megahertz, Microhertz, Millihertz,
        Petahertz, RadiansPerSecond, RevolutionsPerMinute, RevolutionsPerSecond, Terahertz,
    },
    mass::{
        Carats, Daltons, Grains, Grams, Kilograms as MassKilograms, LongTons, Mass, MetricTonnes,
        Micrograms, Milligrams, Nanograms, Ounces as MassOunces, Pounds as MassPounds, ShortTons,
        Slugs, Stones as MassStones, TroyOunces, UnifiedAtomicMassUnits,
    },
    speed::{
        CentimetersPerSecond, FeetPerMinute, FeetPerSecond, KilometersPerHour, KilometersPerSecond,
        Knots, Mach, MetersPerSecond, MilesPerHour, Speed,
    },
    time::{
        Days, Femtoseconds, Hours, JulianYears, MeanGregorianMonths, MeanGregorianYears,
        Microseconds, Milliseconds, Minutes, Nanoseconds, Picoseconds, Seconds, Time, Weeks,
    },
    weight::{
        Dynes, Kilograms as WeightKilograms, Kilonewtons, Kips, Newtons, Ounces as WeightOunces,
        Pounds as WeightPounds, Stones as WeightStones, Weight,
    },
};

fn assert_close(actual: f64, expected: f64) {
    let scale = actual.abs().max(expected.abs()).max(1.0);
    assert!(
        (actual - expected).abs() <= f64::EPSILON * 32.0 * scale,
        "expected {expected:?}, got {actual:?}"
    );
}

#[test]
fn every_distance_unit_converts_to_meters() {
    assert_close(Picometers(1.0).to_meters().0, 1e-12);
    assert_close(Angstroms(1.0).to_meters().0, 1e-10);
    assert_close(Nanometers(1.0).to_meters().0, 1e-9);
    assert_close(Micrometers(1.0).to_meters().0, 1e-6);
    assert_close(Millimeters(1.0).to_meters().0, 1e-3);
    assert_close(Centimeters(1.0).to_meters().0, 1e-2);
    assert_close(Decimeters(1.0).to_meters().0, 1e-1);
    assert_close(Meters(1.0).to_meters().0, 1.0);
    assert_close(Kilometers(1.0).to_meters().0, 1e3);
    assert_close(Inches(1.0).to_meters().0, 0.0254);
    assert_close(Feet(1.0).to_meters().0, 0.3048);
    assert_close(Yards(1.0).to_meters().0, 0.9144);
    assert_close(Miles(1.0).to_meters().0, 1_609.344);
    assert_close(NauticalMiles(1.0).to_meters().0, 1_852.0);
    assert_close(AstronomicalUnits(1.0).to_meters().0, 149_597_870_700.0);
    assert_close(LightYears(1.0).to_meters().0, 9_460_730_472_580_800.0);
    assert_close(
        Parsecs(1.0).to_meters().0,
        149_597_870_700.0 * 648_000.0 / std::f64::consts::PI,
    );
}

#[test]
fn every_area_unit_converts_to_square_meters() {
    assert_close(SquarePicometers(1.0).to_square_meters().0, 1e-24);
    assert_close(SquareAngstroms(1.0).to_square_meters().0, 1e-20);
    assert_close(SquareNanometers(1.0).to_square_meters().0, 1e-18);
    assert_close(SquareMicrometers(1.0).to_square_meters().0, 1e-12);
    assert_close(SquareMillimeters(1.0).to_square_meters().0, 1e-6);
    assert_close(SquareCentimeters(1.0).to_square_meters().0, 1e-4);
    assert_close(SquareDecimeters(1.0).to_square_meters().0, 1e-2);
    assert_close(SquareMeters(1.0).to_square_meters().0, 1.0);
    assert_close(SquareKilometers(1.0).to_square_meters().0, 1e6);
    assert_close(SquareInches(1.0).to_square_meters().0, 0.000_645_16);
    assert_close(SquareFeet(1.0).to_square_meters().0, 0.092_903_04);
    assert_close(SquareYards(1.0).to_square_meters().0, 0.836_127_36);
    assert_close(SquareMiles(1.0).to_square_meters().0, 2_589_988.110_336);
    assert_close(SquareNauticalMiles(1.0).to_square_meters().0, 3_429_904.0);
    assert_close(
        SquareAstronomicalUnits(1.0).to_square_meters().0,
        149_597_870_700.0_f64.powi(2),
    );
    assert_close(
        SquareLightYears(1.0).to_square_meters().0,
        9_460_730_472_580_800.0_f64.powi(2),
    );
    let parsec = 149_597_870_700.0 * 648_000.0 / std::f64::consts::PI;
    assert_close(SquareParsecs(1.0).to_square_meters().0, parsec * parsec);
    assert_close(Ares(1.0).to_square_meters().0, 100.0);
    assert_close(Hectares(1.0).to_square_meters().0, 10_000.0);
    assert_close(Acres(1.0).to_square_meters().0, 4_046.856_422_4);
    assert_close(Barns(1.0).to_square_meters().0, 1e-28);
}

#[test]
fn every_time_and_speed_unit_converts_to_si() {
    assert_close(Femtoseconds(1.0).to_seconds().0, 1e-15);
    assert_close(Picoseconds(1.0).to_seconds().0, 1e-12);
    assert_close(Nanoseconds(1.0).to_seconds().0, 1e-9);
    assert_close(Microseconds(1.0).to_seconds().0, 1e-6);
    assert_close(Milliseconds(1.0).to_seconds().0, 1e-3);
    assert_close(Seconds(1.0).to_seconds().0, 1.0);
    assert_close(Minutes(1.0).to_seconds().0, 60.0);
    assert_close(Hours(1.0).to_seconds().0, 3_600.0);
    assert_close(Days(1.0).to_seconds().0, 86_400.0);
    assert_close(Weeks(1.0).to_seconds().0, 604_800.0);
    assert_close(MeanGregorianYears(1.0).to_days().0, 365.2425);
    assert_close(MeanGregorianMonths(12.0).to_mean_gregorian_years().0, 1.0);
    assert_close(JulianYears(1.0).to_days().0, 365.25);

    assert_close(CentimetersPerSecond(1.0).to_meters_per_second().0, 0.01);
    assert_close(MetersPerSecond(1.0).to_meters_per_second().0, 1.0);
    assert_close(KilometersPerSecond(1.0).to_meters_per_second().0, 1_000.0);
    assert_close(KilometersPerHour(3.6).to_meters_per_second().0, 1.0);
    assert_close(MilesPerHour(1.0).to_meters_per_second().0, 0.447_04);
    assert_close(FeetPerSecond(1.0).to_meters_per_second().0, 0.3048);
    assert_close(FeetPerMinute(60.0).to_feet_per_second().0, 1.0);
    assert_close(Knots(1.0).to_kilometers_per_hour().0, 1.852);
    assert_close(Mach(1.0).to_meters_per_second().0, 340.294);
}

#[test]
fn every_acceleration_mass_and_weight_unit_converts_to_si() {
    assert_close(
        MetersPerSecondSquared(1.0).to_meters_per_second_squared().0,
        1.0,
    );
    assert_close(
        CentimetersPerSecondSquared(1.0)
            .to_meters_per_second_squared()
            .0,
        0.01,
    );
    assert_close(
        FeetPerSecondSquared(1.0).to_meters_per_second_squared().0,
        0.3048,
    );
    assert_close(
        KilometersPerHourPerSecond(3.6)
            .to_meters_per_second_squared()
            .0,
        1.0,
    );
    assert_close(
        MilesPerHourPerSecond(1.0).to_meters_per_second_squared().0,
        0.447_04,
    );
    assert_close(Gals(1.0).to_meters_per_second_squared().0, 0.01);
    assert_close(Milligals(1.0).to_meters_per_second_squared().0, 1e-5);
    assert_close(
        StandardGravities(1.0).to_meters_per_second_squared().0,
        9.80665,
    );

    assert_close(Nanograms(1.0).to_kilograms().0, 1e-12);
    assert_close(Micrograms(1.0).to_kilograms().0, 1e-9);
    assert_close(Milligrams(1.0).to_kilograms().0, 1e-6);
    assert_close(Grams(1.0).to_kilograms().0, 1e-3);
    assert_close(MassKilograms(1.0).to_kilograms().0, 1.0);
    assert_close(MetricTonnes(1.0).to_kilograms().0, 1_000.0);
    assert_close(Carats(5.0).to_grams().0, 1.0);
    assert_close(Grains(1.0).to_milligrams().0, 64.798_91);
    assert_close(MassOunces(16.0).to_pounds().0, 1.0);
    assert_close(MassPounds(1.0).to_kilograms().0, 0.453_592_37);
    assert_close(MassStones(1.0).to_pounds().0, 14.0);
    assert_close(ShortTons(1.0).to_pounds().0, 2_000.0);
    assert_close(LongTons(1.0).to_pounds().0, 2_240.0);
    assert_close(TroyOunces(1.0).to_grams().0, 31.103_476_8);
    assert_close(Slugs(1.0).to_kilograms().0, 14.593_902_937_206_364);
    assert_close(Daltons(1.0).to_kilograms().0, 1.660_539_068_92e-27);
    assert_eq!(Daltons(1.0), UnifiedAtomicMassUnits(1.0));

    assert_close(Dynes(100_000.0).to_newtons().0, 1.0);
    assert_close(Newtons(1.0).to_newtons().0, 1.0);
    assert_close(Kilonewtons(1.0).to_newtons().0, 1_000.0);
    assert_close(WeightOunces(16.0).to_pounds().0, 1.0);
    assert_close(WeightPounds(1.0).to_newtons().0, 4.448_221_615_260_5);
    assert_close(WeightStones(1.0).to_pounds().0, 14.0);
    assert_close(WeightKilograms(1.0).to_newtons().0, 9.80665);
    assert_close(Kips(1.0).to_pounds().0, 1_000.0);
}

#[test]
fn every_frequency_unit_converts_to_hertz() {
    assert_close(Microhertz(1.0).to_hertz().0, 1e-6);
    assert_close(Millihertz(1.0).to_hertz().0, 1e-3);
    assert_close(Hertz(1.0).to_hertz().0, 1.0);
    assert_close(Kilohertz(1.0).to_hertz().0, 1e3);
    assert_close(Megahertz(1.0).to_hertz().0, 1e6);
    assert_close(Gigahertz(1.0).to_hertz().0, 1e9);
    assert_close(Terahertz(1.0).to_hertz().0, 1e12);
    assert_close(Petahertz(1.0).to_hertz().0, 1e15);
    assert_close(RevolutionsPerSecond(1.0).to_hertz().0, 1.0);
    assert_close(RevolutionsPerMinute(60.0).to_hertz().0, 1.0);
    assert_close(BeatsPerMinute(60.0).to_hertz().0, 1.0);
    assert_close(RadiansPerSecond(std::f64::consts::TAU).to_hertz().0, 1.0);
}

#[test]
fn non_finite_values_remain_detectable() {
    assert!(!Meters(f64::NAN).is_finite());
    assert!(!SquareMeters(f64::INFINITY).is_finite());
    assert!(!Seconds(f64::NEG_INFINITY).is_finite());
    assert!(!MetersPerSecond(f64::NAN).is_finite());
    assert!(!MetersPerSecondSquared(f64::INFINITY).is_finite());
    assert!(!MassKilograms(f64::NAN).is_finite());
    assert!(!Newtons(f64::INFINITY).is_finite());
    assert!(!Hertz(f64::NAN).is_finite());
}
