use units_of_measure::{
    distance::{Distance, Miles},
    speed::{MilesPerHour, Speed},
    time::{Hours, Time},
};

fn main() {
    let distance = Miles(275.0);
    let average_speed = MilesPerHour(62.0);
    let duration = distance / average_speed;

    println!(
        "{:.1} miles at {:.1} mph takes {:.2} hours",
        distance.to_miles().0,
        average_speed.to_miles_per_hour().0,
        duration.to_hours().0
    );

    let _: Hours = duration.to_hours();
}
