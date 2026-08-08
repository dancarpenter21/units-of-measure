use units_of_measure::{
    acceleration::Acceleration,
    angle::{Degrees, Radians},
    angular_acceleration::AngularAcceleration,
    area::Area,
    distance::{Feet, Meters, Miles},
    frequency::Frequency,
    mass::Kilograms,
    speed::Speed,
    time::{Hours, Milliseconds, Minutes, Seconds},
    torque::Torque,
    weight::Weight,
};

fn assert_close(actual: f64, expected: f64) {
    let scale = actual.abs().max(expected.abs()).max(1.0);
    assert!((actual - expected).abs() <= f64::EPSILON * 64.0 * scale);
}

#[test]
fn distance_area_speed_and_acceleration_identities_hold() {
    let area: Area<Feet, Meters> = Feet::new(12.0) * Meters::new(3.0);
    assert_close(area.value_in::<Feet, Feet>(), 118.110_236_220_472_44);

    let speed: Speed<Miles, Hours> = Miles::new(60.0) / Hours::new(1.0);
    assert_close((speed * Minutes::new(30.0)).value(), 48_280.32);

    let speed: Speed<Miles, Hours> = Miles::new(60.0) / Hours::new(1.0);
    assert_close((Minutes::new(30.0) * speed).value(), 48_280.32);

    let speed: Speed<Miles, Hours> = Miles::new(60.0) / Hours::new(1.0);
    assert_close((Miles::new(60.0) / speed).value(), 3_600.0);

    let speed: Speed<Miles, Hours> = Miles::new(60.0) / Hours::new(1.0);
    let acceleration = speed / Minutes::new(1.0);
    assert_close(acceleration.value(), 0.44704);

    let acceleration =
        Acceleration::new(Meters::new(9.80665), Seconds::new(1.0), Seconds::new(1.0));
    assert_close((acceleration * Seconds::new(2.0)).value(), 19.6133);

    let acceleration =
        Acceleration::new(Meters::new(9.80665), Seconds::new(1.0), Seconds::new(1.0));
    assert_close((Seconds::new(2.0) * acceleration).value(), 19.6133);
}

#[test]
fn force_and_torque_identities_hold() {
    let acceleration =
        Acceleration::new(Meters::new(9.80665), Seconds::new(1.0), Seconds::new(1.0));
    let weight = Kilograms::new(2.0) * acceleration;
    assert_close(weight.value(), 19.6133);

    let acceleration =
        Acceleration::new(Meters::new(9.80665), Seconds::new(1.0), Seconds::new(1.0));
    let weight = acceleration * Kilograms::new(2.0);
    assert_close(weight.value(), 19.6133);

    let weight = Weight::new(
        Kilograms::new(2.0),
        Meters::new(9.80665),
        Seconds::new(1.0),
        Seconds::new(1.0),
    );
    let torque: Torque<Kilograms, Meters, Feet, Seconds, Seconds> = weight * Feet::new(3.0);
    assert_close(
        torque.value_in::<Kilograms, Meters, Meters, Seconds, Seconds>(),
        17.934_401_52,
    );

    let weight = Weight::new(
        Kilograms::new(2.0),
        Meters::new(9.80665),
        Seconds::new(1.0),
        Seconds::new(1.0),
    );
    let torque: Torque<Kilograms, Meters, Feet, Seconds, Seconds> = Feet::new(3.0) * weight;
    assert_close(
        torque.value_in::<Kilograms, Meters, Meters, Seconds, Seconds>(),
        17.934_401_52,
    );

    let torque = Torque::new(
        Kilograms::new(2.0),
        Meters::new(9.80665),
        Meters::new(3.0),
        Seconds::new(1.0),
        Seconds::new(1.0),
    );
    assert_close((torque / Meters::new(3.0)).value(), 19.6133);
}

#[test]
fn rotational_and_frequency_identities_hold() {
    let angular_velocity = Degrees::new(90.0) / Seconds::new(2.0);
    assert_close(
        (angular_velocity * Seconds::new(2.0)).value(),
        std::f64::consts::PI / 2.0,
    );

    let angular_velocity = Degrees::new(90.0) / Seconds::new(2.0);
    assert_close(
        (Seconds::new(2.0) * angular_velocity).value(),
        std::f64::consts::PI / 2.0,
    );

    let angular_velocity = Degrees::new(90.0) / Seconds::new(2.0);
    let angular_acceleration = angular_velocity / Seconds::new(2.0);
    assert_close(angular_acceleration.value(), std::f64::consts::PI / 8.0);

    let angular_acceleration = AngularAcceleration::new(
        Radians::new(std::f64::consts::PI),
        Seconds::new(1.0),
        Seconds::new(2.0),
    );
    assert_close(
        (angular_acceleration * Seconds::new(2.0)).value(),
        std::f64::consts::PI,
    );

    let angular_acceleration = AngularAcceleration::new(
        Radians::new(std::f64::consts::PI),
        Seconds::new(1.0),
        Seconds::new(2.0),
    );
    assert_close(
        (Seconds::new(2.0) * angular_acceleration).value(),
        std::f64::consts::PI,
    );

    let frequency = Frequency::new(440.0, Seconds::new(1.0));
    assert_close(frequency * Milliseconds::new(10.0), 4.4);

    let frequency = Frequency::new(440.0, Seconds::new(1.0));
    assert_close(Milliseconds::new(10.0) * frequency, 4.4);
}
