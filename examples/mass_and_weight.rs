use units_of_measure::{
    acceleration::MetersPerSecondSquared,
    mass::{Kilograms, Mass},
    weight::Weight,
};

fn main() {
    let cargo = Kilograms(500.0);
    let earth = cargo.weight_at_standard_gravity();
    let moon = cargo.weight_at(&MetersPerSecondSquared(1.62));

    println!("mass: {:.1} kg", cargo.to_kilograms().0);
    println!("Earth weight: {:.1} N", earth.to_newtons().0);
    println!("Moon weight: {:.1} N", moon.to_newtons().0);
}
