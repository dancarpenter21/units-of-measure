use units_of_measure::{
    distance::{Distance, Miles},
    speed::Speed,
    time::{Hours, Time},
};

fn main() {
    let average_speed = Speed::new(Miles::new(275.0), Hours::new(4.5));
    let mph = average_speed.distance().to_miles().value() / average_speed.time().to_hours().value();

    println!("average speed: {mph:.1} mph");
}
