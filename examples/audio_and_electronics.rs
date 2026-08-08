use units_of_measure::{audio, frequency::Frequency, time::Seconds};

fn main() {
    let a4 = audio::midi_note_frequency(69);
    let clock = Frequency::new(16_000_000.0, Seconds::new(1.0));

    println!("A4: {:.1} cycles/s", a4.value());
    println!("clock: {:.1} MHz", clock.value() / 1_000_000.0);
}
