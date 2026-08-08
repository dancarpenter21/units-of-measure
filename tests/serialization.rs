#![cfg(feature = "serde")]

use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;
use units_of_measure::{
    acceleration::Acceleration, angle::Degrees, angular_acceleration::AngularAcceleration,
    angular_velocity::AngularVelocity, area::Area, distance::Meters, frequency::Frequency,
    mass::Kilograms, speed::Speed, time::Seconds, torque::Torque, weight::Weight,
};

fn assert_round_trip<T>(value: T)
where
    T: Serialize + DeserializeOwned + PartialEq + Debug,
{
    let json = serde_json::to_string(&value).expect("value should serialize to JSON");
    let from_json: T = serde_json::from_str(&json).expect("value should deserialize from JSON");
    assert_eq!(from_json, value);

    let yaml = serde_yaml::to_string(&value).expect("value should serialize to YAML");
    let from_yaml: T = serde_yaml::from_str(&yaml).expect("value should deserialize from YAML");
    assert_eq!(from_yaml, value);
}

#[test]
fn primitive_units_remain_numeric() {
    assert_eq!(serde_json::to_string(&Meters::new(12.5)).unwrap(), "12.5");
    assert_round_trip(Meters::new(12.5));
    assert_round_trip(Degrees::new(90.0));
}

#[test]
fn composed_values_use_named_component_objects() {
    let speed = Speed::new(Meters::new(10.0), Seconds::new(1.0));
    assert_eq!(
        serde_json::to_string(&speed).unwrap(),
        r#"{"distance":10.0,"time":1.0}"#
    );
    assert_round_trip(speed);

    assert_round_trip(Area::new(Meters::new(2.0), Meters::new(3.0)));
    assert_round_trip(Acceleration::new(
        Meters::new(9.80665),
        Seconds::new(1.0),
        Seconds::new(1.0),
    ));
    assert_round_trip(Weight::new(
        Kilograms::new(80.0),
        Meters::new(9.80665),
        Seconds::new(1.0),
        Seconds::new(1.0),
    ));
    assert_round_trip(Frequency::new(440.0, Seconds::new(1.0)));
    assert_round_trip(AngularVelocity::new(Degrees::new(90.0), Seconds::new(1.0)));
    assert_round_trip(AngularAcceleration::new(
        Degrees::new(90.0),
        Seconds::new(1.0),
        Seconds::new(1.0),
    ));
    assert_round_trip(Torque::new(
        Kilograms::new(2.0),
        Meters::new(3.0),
        Meters::new(4.0),
        Seconds::new(1.0),
        Seconds::new(1.0),
    ));
}
