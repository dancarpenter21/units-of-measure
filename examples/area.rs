use units_of_measure::{
    area::Area,
    distance::{Distance, Feet},
};

fn main() {
    let room = Area::new(Feet::new(12.0), Feet::new(15.0));
    let square_feet = room.width().to_feet().value() * room.height().to_feet().value();

    println!("room: {square_feet:.1} ft²");
}
