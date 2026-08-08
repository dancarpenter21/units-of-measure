use units_of_measure::{
    acceleration::{Acceleration, STANDARD_GRAVITY},
    angle::{Angle, Degrees, Radians},
    angular_acceleration::AngularAcceleration,
    angular_velocity::AngularVelocity,
    area::Area,
    audio,
    distance::{Distance, Feet, Meters},
    frequency::Frequency,
    mass::{Kilograms, Mass},
    speed::Speed,
    time::{Seconds, Time},
    torque::Torque,
    weight::Weight,
};

fn assert_close(actual: f64, expected: f64) {
    let scale = actual.abs().max(expected.abs()).max(1.0);
    assert!((actual - expected).abs() <= f64::EPSILON * 32.0 * scale);
}

fn miles_per_hour<D: Distance, T: Time>(speed: &Speed<D, T>) -> f64 {
    speed.distance().to_miles().value() / speed.time().to_hours().value()
}

#[test]
fn speed_retains_components_and_converts_at_the_call_site() {
    let speed = Speed::new(Meters::new(10.0), Seconds::new(1.0));
    let _: &Meters = speed.distance();
    let _: &Seconds = speed.time();

    assert_close(miles_per_hour(&speed), 22.369_362_920_544_024);
    assert!(speed.is_finite());
}

#[test]
fn area_retains_its_two_distance_components() {
    let area = Area::new(Feet::new(12.0), Meters::new(3.0));
    let square_feet = area.width().to_feet().value() * area.height().to_feet().value();

    assert_close(square_feet, 118.110_236_220_472_44);
    assert!(area.is_finite());
}

#[test]
fn acceleration_and_weight_retain_all_base_factors() {
    let acceleration =
        Acceleration::new(Meters::new(9.80665), Seconds::new(1.0), Seconds::new(1.0));
    let weight = Weight::new(
        Kilograms::new(80.0),
        Meters::new(9.80665),
        Seconds::new(1.0),
        Seconds::new(1.0),
    );

    assert_eq!(acceleration, STANDARD_GRAVITY);
    assert_close(weight.mass().to_kilograms().value(), 80.0);
    assert_close(weight.distance().to_meters().value(), 9.80665);
    assert!(acceleration.is_finite());
    assert!(weight.is_finite());
}

#[test]
fn hierarchical_constructors_expand_to_the_same_base_components() {
    let acceleration = Acceleration::from_speed_and_time(
        Speed::new(Meters::new(9.80665), Seconds::new(1.0)),
        Seconds::new(1.0),
    );
    assert_eq!(acceleration, STANDARD_GRAVITY);

    let weight = Weight::from_mass_and_acceleration(Kilograms::new(2.0), acceleration);
    assert_eq!(
        weight,
        Weight::new(
            Kilograms::new(2.0),
            Meters::new(9.80665),
            Seconds::new(1.0),
            Seconds::new(1.0),
        )
    );

    let angular_acceleration = AngularAcceleration::from_angular_velocity_and_time(
        AngularVelocity::new(Degrees::new(90.0), Seconds::new(1.0)),
        Seconds::new(2.0),
    );
    assert_eq!(
        angular_acceleration,
        AngularAcceleration::new(Degrees::new(90.0), Seconds::new(1.0), Seconds::new(2.0),)
    );

    let torque = Torque::from_weight_and_distance(weight, Meters::new(3.0));
    assert_eq!(
        torque,
        Torque::new(
            Kilograms::new(2.0),
            Meters::new(9.80665),
            Meters::new(3.0),
            Seconds::new(1.0),
            Seconds::new(1.0),
        )
    );
}

#[test]
fn frequency_retains_cycles_and_duration() {
    let frequency = Frequency::new(48_000.0, Seconds::new(1.0));
    assert_eq!(frequency.cycles(), 48_000.0);
    assert_eq!(frequency.duration().to_seconds().value(), 1.0);
    assert!(frequency.is_finite());
}

#[test]
fn angles_convert_and_rotational_quantities_retain_components() {
    assert_close(
        Degrees::new(180.0).to_radians().value(),
        std::f64::consts::PI,
    );
    assert_close(
        Radians::new(std::f64::consts::PI).to_degrees().value(),
        180.0,
    );

    let velocity = AngularVelocity::new(Degrees::new(180.0), Seconds::new(2.0));
    let acceleration = AngularAcceleration::new(
        Radians::new(std::f64::consts::PI),
        Seconds::new(1.0),
        Seconds::new(2.0),
    );
    let torque = Torque::new(
        Kilograms::new(2.0),
        Meters::new(3.0),
        Meters::new(4.0),
        Seconds::new(1.0),
        Seconds::new(1.0),
    );

    let _: &Degrees = velocity.angle();
    assert_close(velocity.time().to_seconds().value(), 2.0);
    assert_close(acceleration.angle().to_degrees().value(), 180.0);
    assert_close(torque.mass().to_kilograms().value(), 2.0);
    assert_close(torque.first_distance().to_meters().value(), 3.0);
    assert_close(torque.second_distance().to_meters().value(), 4.0);
    assert!(velocity.is_finite());
    assert!(acceleration.is_finite());
    assert!(torque.is_finite());
}

#[test]
fn composed_quantities_accept_custom_primitive_units() {
    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    struct Leagues(f64);

    impl Distance for Leagues {
        fn to_meters(&self) -> Meters {
            Meters::new(self.0 * 4_828.032)
        }
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    struct Shifts(f64);

    impl Time for Shifts {
        fn to_seconds(&self) -> Seconds {
            Seconds::new(self.0 * 28_800.0)
        }
    }

    let speed = Speed::new(Leagues(1.0), Shifts(1.0));
    assert_close(miles_per_hour(&speed), 0.375);
}

#[test]
fn audio_helpers_operate_on_component_frequencies() {
    let a4 = Frequency::new(440.0, Seconds::new(1.0));
    assert_eq!(audio::midi_note_frequency(69), audio::CONCERT_A4);
    assert!(audio::is_nominally_audible(&a4));
    assert_close(audio::midi_note_number(&a4), 69.0);
}

#[test]
fn composed_non_finite_values_remain_detectable() {
    assert!(!Speed::new(Meters::new(f64::NAN), Seconds::new(1.0)).is_finite());
    assert!(!Frequency::new(1.0, Seconds::new(f64::INFINITY)).is_finite());
    assert!(!AngularVelocity::new(Degrees::new(f64::NAN), Seconds::new(1.0)).is_finite());
}
