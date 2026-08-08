use units_of_measure::{
    distance::{Distance, Feet, Meters, Miles},
    mass::{Kilograms, Mass, Pounds},
    time::{Hours, Seconds, Time},
};

fn assert_close(actual: f64, expected: f64) {
    let scale = actual.abs().max(expected.abs()).max(1.0);
    assert!((actual - expected).abs() <= f64::EPSILON * 32.0 * scale);
}

#[test]
fn distance_units_convert_and_keep_same_quantity_arithmetic() {
    assert_close(Meters::new(1.0).to_feet().value(), 3.280_839_895_013_123);
    assert_close(Miles::new(1.0).to_meters().value(), 1_609.344);
    assert_close(
        (Feet::new(3.0) + Meters::new(1.0)).value(),
        6.280_839_895_013_123,
    );
}

#[test]
fn time_units_convert_and_keep_same_quantity_arithmetic() {
    assert_close(Hours::new(1.0).to_seconds().value(), 3_600.0);
    assert_close(
        (Hours::new(1.0) + Seconds::new(30.0)).value(),
        1.0 + 30.0 / 3_600.0,
    );
}

#[test]
fn mass_units_convert_and_keep_same_quantity_arithmetic() {
    assert_close(Pounds::new(1.0).to_kilograms().value(), 0.453_592_37);
    assert_close(
        (Kilograms::new(1.0) + Pounds::new(1.0)).value(),
        1.453_592_37,
    );
}

#[test]
fn primitive_non_finite_values_remain_detectable() {
    assert!(!Meters::new(f64::NAN).is_finite());
    assert!(!Seconds::new(f64::INFINITY).is_finite());
    assert!(!Kilograms::new(f64::NEG_INFINITY).is_finite());
}
