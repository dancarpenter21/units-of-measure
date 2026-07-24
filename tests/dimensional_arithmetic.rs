use units_of_measure::{
    acceleration::{Acceleration, MetersPerSecondSquared, StandardGravities},
    area::{Acres, Area, SquareFeet, SquareMeters},
    distance::{Distance, Feet, Kilometers, Meters, Miles, NauticalMiles},
    frequency::{Frequency, Hertz, Kilohertz},
    mass::{Kilograms, Mass, Pounds as MassPounds},
    speed::{Knots, MetersPerSecond, MilesPerHour, Speed},
    time::{Hours, Milliseconds, Seconds, Time},
    weight::{Newtons, Pounds as WeightPounds, Weight},
};

fn assert_close(actual: f64, expected: f64) {
    let scale = actual.abs().max(expected.abs()).max(1.0);
    assert!((actual - expected).abs() <= f64::EPSILON * 32.0 * scale);
}

#[test]
fn canonical_dimensional_results_preserve_physical_identities() {
    let floor: SquareMeters = Feet(10.0) * Feet(12.0);
    assert_close(floor.to_square_feet().0, 120.0);
    let width: Meters = floor / Feet(10.0);
    assert_close(width.to_feet().0, 12.0);

    let speed: MetersPerSecond = Miles(120.0) / Hours(2.0);
    assert_close(speed.to_miles_per_hour().0, 60.0);
    assert_close((speed * Hours(2.0)).to_miles().0, 120.0);
    assert_close((Miles(120.0) / speed).to_hours().0, 2.0);

    let acceleration: MetersPerSecondSquared = speed / Seconds(10.0);
    assert_close(
        (acceleration * Seconds(10.0)).to_meters_per_second().0,
        speed.to_meters_per_second().0,
    );
    assert_close((speed / acceleration).to_seconds().0, 10.0);
}

#[test]
fn mass_and_weight_remain_distinct() {
    let mass = MassPounds(180.0);
    let weight: Newtons = mass * StandardGravities(1.0);
    assert_close(weight.to_pounds().0, 180.0);
    assert_close((weight / StandardGravities(1.0)).to_pounds().0, 180.0);

    let colloquial = WeightPounds(180.0);
    assert_close(colloquial.mass_at_standard_gravity().to_pounds().0, 180.0);
}

#[test]
fn nautical_and_frequency_calculations_are_typed() {
    let range = Knots(450.0) * Hours(2.5);
    assert_close(range.to_nautical_miles().0, 1_125.0);
    assert_close((NauticalMiles(20.0) / Hours(0.5)).to_knots().0, 40.0);

    assert_close(Kilohertz(48.0) * Milliseconds(10.0), 480.0);
    assert_close(Milliseconds(10.0) * Kilohertz(48.0), 480.0);
}

#[test]
fn mixed_units_preserve_the_left_unit_for_same_quantity_math() {
    let imperial: Feet = Feet(3.0) + Meters(1.0);
    let metric: Meters = Meters(1.0) + Feet(3.0);
    assert_close(imperial.0, 6.280_839_895_013_123);
    assert_close(metric.0, 1.9144);

    let land: Acres = Acres(1.0) + SquareMeters(10_000.0);
    assert_close(land.0, 3.471_053_814_671_653);
}

#[test]
fn custom_types_inherit_typed_conversions_and_calculations() {
    struct Leagues(f64);
    impl Distance for Leagues {
        fn to_meters(&self) -> Meters {
            Meters(self.0 * 4_828.032)
        }
    }

    struct Lots(f64);
    impl Area for Lots {
        fn to_square_meters(&self) -> SquareMeters {
            SquareMeters(self.0 * 500.0)
        }
    }

    struct Shifts(f64);
    impl Time for Shifts {
        fn to_seconds(&self) -> Seconds {
            Seconds(self.0 * 8.0 * 3_600.0)
        }
    }

    struct Cruise(f64);
    impl Speed for Cruise {
        fn to_meters_per_second(&self) -> MetersPerSecond {
            MetersPerSecond(self.0)
        }
    }

    struct Pull(f64);
    impl Acceleration for Pull {
        fn to_meters_per_second_squared(&self) -> MetersPerSecondSquared {
            MetersPerSecondSquared(self.0)
        }
    }

    struct Payload(f64);
    impl Mass for Payload {
        fn to_kilograms(&self) -> Kilograms {
            Kilograms(self.0)
        }
    }

    struct Thrust(f64);
    impl Weight for Thrust {
        fn to_newtons(&self) -> Newtons {
            Newtons(self.0)
        }
    }

    struct Oscillator(f64);
    impl Frequency for Oscillator {
        fn to_hertz(&self) -> Hertz {
            Hertz(self.0)
        }
    }

    let distance: &dyn Distance = &Leagues(1.0);
    assert_close(distance.to_miles().0, 3.0);
    assert_close(distance.area_with(&Meters(2.0)).0, 9_656.064);
    assert_close(distance.speed_over(&Seconds(2.0)).0, 2_414.016);

    let area: &dyn Area = &Lots(1.0);
    assert_close(area.to_square_feet().0, 5_381.955_208_354_861);
    assert_close(area.length_for_width(&Meters(20.0)).0, 25.0);

    let time: &dyn Time = &Shifts(1.0);
    assert_close(time.to_hours().0, 8.0);

    let speed: &dyn Speed = &Cruise(10.0);
    assert_close(speed.distance_over(&Seconds(2.0)).0, 20.0);

    let acceleration: &dyn Acceleration = &Pull(2.0);
    assert_close(acceleration.speed_change_over(&Seconds(3.0)).0, 6.0);

    let mass: &dyn Mass = &Payload(5.0);
    assert_close(mass.weight_at(&Pull(2.0)).0, 10.0);

    let force: &dyn Weight = &Thrust(10.0);
    assert_close(force.acceleration_of(&Payload(5.0)).0, 2.0);

    let frequency: &dyn Frequency = &Oscillator(440.0);
    assert_close(frequency.cycles_in(&Seconds(0.5)), 220.0);
}

#[test]
fn zero_division_retains_ieee_754_behavior() {
    assert!((Kilometers(1.0) / Seconds::ZERO).0.is_infinite());
    assert!((SquareFeet(1.0) / Feet::ZERO).0.is_infinite());
    assert!((Newtons(1.0) / Kilograms::ZERO).0.is_infinite());
    assert!(Hertz::ZERO.period().0.is_infinite());
}

#[test]
fn public_result_types_are_canonical() {
    let _: SquareMeters = Miles(1.0) * Feet(1.0);
    let _: Meters = SquareFeet(1.0) / Feet(1.0);
    let _: MetersPerSecond = Miles(1.0) / Hours(1.0);
    let _: Seconds = Miles(1.0) / MilesPerHour(1.0);
    let _: MetersPerSecondSquared = MetersPerSecond(1.0) / Seconds(1.0);
    let _: Newtons = Kilograms(1.0) * MetersPerSecondSquared(1.0);
}
