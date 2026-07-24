use units_of_measure::{
    area::{Acres, Area},
    distance::{Feet, Meters},
};

fn main() {
    let room = Feet(12.0) * Feet(15.0);
    let lot = Acres(0.25);

    println!("room: {:.1} ft²", room.to_square_feet().0);
    println!("lot: {:.1} m²", lot.to_square_meters().0);

    let _: Meters = room / Feet(12.0);
}
