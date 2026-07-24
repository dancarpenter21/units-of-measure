use units_of_measure::{
    audio,
    frequency::{Frequency, Kilohertz, Megahertz},
    time::{Milliseconds, Time},
};

fn main() {
    let a4 = audio::midi_note_frequency(69);
    let clock = Megahertz(16.0);
    let samples = Kilohertz(48.0) * Milliseconds(10.0);

    println!("A4: {:.1} Hz", a4.0);
    println!("16 MHz period: {:.1} ns", clock.period().to_nanoseconds().0);
    println!("48 kHz samples in 10 ms: {samples:.0}");
}
