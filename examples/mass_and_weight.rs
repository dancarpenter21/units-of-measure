use units_of_measure::{distance::Meters, mass::Kilograms, time::Seconds, weight::Weight};

fn main() {
    let earth_weight = Weight::new(
        Kilograms::new(500.0),
        Meters::new(9.80665),
        Seconds::new(1.0),
        Seconds::new(1.0),
    );

    println!(
        "mass: {:.1} kg; gravity: {:.5} m/s²",
        earth_weight.mass().value(),
        earth_weight.value() / earth_weight.mass().value()
    );
}
