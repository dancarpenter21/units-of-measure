use units_of_measure::{area::Area, distance::Feet};

fn main() {
    let room = Area::new(Feet::new(12.0), Feet::new(15.0));
    let square_feet = room.value();

    println!("room: {square_feet:.1} ft²");
}
