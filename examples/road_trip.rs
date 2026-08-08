use units_of_measure::{distance::Miles, speed::Speed, time::Hours};

fn main() {
    let average_speed = Speed::new(Miles::new(275.0), Hours::new(4.5));
    let mph = average_speed.value();

    println!("average speed: {mph:.1} mph");
}
