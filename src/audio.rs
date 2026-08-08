//! Audio-specific frequency helpers.
//!
//! The module uses twelve-tone equal temperament with A4 tuned to 440 Hz.

use crate::frequency::Frequency;
use crate::time::{Seconds, Time};

/// The conventional lower edge of nominal human hearing, 20 cycles per second.
pub const NOMINAL_HUMAN_HEARING_MIN: Frequency<Seconds> = Frequency::new(20.0, Seconds::new(1.0));

/// The conventional upper edge of nominal human hearing, 20,000 cycles per second.
pub const NOMINAL_HUMAN_HEARING_MAX: Frequency<Seconds> =
    Frequency::new(20_000.0, Seconds::new(1.0));

/// Concert-pitch A4, 440 cycles per second.
pub const CONCERT_A4: Frequency<Seconds> = Frequency::new(440.0, Seconds::new(1.0));

/// Returns the equal-tempered frequency of a MIDI note number.
///
/// MIDI note 69 is A4 at 440 cycles per second. All `u8` values are accepted,
/// including values above the conventional 0–127 MIDI range.
pub fn midi_note_frequency(note: u8) -> Frequency<Seconds> {
    Frequency::new(
        440.0 * 2.0_f64.powf((f64::from(note) - 69.0) / 12.0),
        Seconds::new(1.0),
    )
}

/// Returns the fractional equal-tempered MIDI note number of a frequency.
pub fn midi_note_number<T: Time>(frequency: &Frequency<T>) -> f64 {
    69.0 + 12.0 * (cycles_per_second(frequency) / cycles_per_second(&CONCERT_A4)).log2()
}

/// Returns whether a frequency is within the nominal 20 Hz–20 kHz range.
pub fn is_nominally_audible<T: Time>(frequency: &Frequency<T>) -> bool {
    let value = cycles_per_second(frequency);
    (cycles_per_second(&NOMINAL_HUMAN_HEARING_MIN)..=cycles_per_second(&NOMINAL_HUMAN_HEARING_MAX))
        .contains(&value)
}

fn cycles_per_second<T: Time>(frequency: &Frequency<T>) -> f64 {
    frequency.cycles() / frequency.duration().to_seconds().value()
}
