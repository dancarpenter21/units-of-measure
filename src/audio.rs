//! Audio-specific frequency helpers.
//!
//! The module uses twelve-tone equal temperament with A4 tuned to 440 Hz.

use crate::frequency::{Frequency, Hertz};

/// The conventional lower edge of nominal human hearing, 20 Hz.
pub const NOMINAL_HUMAN_HEARING_MIN: Hertz = Hertz(20.0);

/// The conventional upper edge of nominal human hearing, 20 kHz.
pub const NOMINAL_HUMAN_HEARING_MAX: Hertz = Hertz(20_000.0);

/// Concert-pitch A4, 440 Hz.
pub const CONCERT_A4: Hertz = Hertz(440.0);

/// Returns the equal-tempered frequency of a MIDI note number.
///
/// MIDI note 69 is A4 at 440 Hz. All `u8` values are accepted, including
/// values above the conventional 0–127 MIDI range.
pub fn midi_note_frequency(note: u8) -> Hertz {
    Hertz(440.0 * 2.0_f64.powf((f64::from(note) - 69.0) / 12.0))
}

/// Returns the fractional equal-tempered MIDI note number of a frequency.
pub fn midi_note_number(frequency: &dyn Frequency) -> f64 {
    69.0 + 12.0 * (frequency.to_hertz().0 / 440.0).log2()
}

/// Returns whether a frequency is within the nominal 20 Hz–20 kHz range.
pub fn is_nominally_audible(frequency: &dyn Frequency) -> bool {
    let hertz = frequency.to_hertz().0;
    (NOMINAL_HUMAN_HEARING_MIN.0..=NOMINAL_HUMAN_HEARING_MAX.0).contains(&hertz)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frequency::Kilohertz;

    fn assert_close(actual: f64, expected: f64) {
        let scale = actual.abs().max(expected.abs()).max(1.0);
        assert!((actual - expected).abs() <= f64::EPSILON * 16.0 * scale);
    }

    #[test]
    fn midi_notes_round_trip() {
        assert_eq!(midi_note_frequency(69), CONCERT_A4);
        assert_close(midi_note_frequency(60).0, 261.625_565_300_598_6);
        for note in 0..=127 {
            assert_close(
                midi_note_number(&midi_note_frequency(note)),
                f64::from(note),
            );
        }
    }

    #[test]
    fn audibility_accepts_any_frequency_unit() {
        assert!(is_nominally_audible(&CONCERT_A4));
        assert!(is_nominally_audible(&Kilohertz(20.0)));
        assert!(!is_nominally_audible(&Hertz(19.0)));
        assert!(!is_nominally_audible(&Hertz(f64::NAN)));
    }
}
