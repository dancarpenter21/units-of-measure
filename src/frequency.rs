//! Frequency units for periodic, audio, mechanical, and electronic signals.

use crate::time::{Seconds, Time};

/// A frequency that can be converted into every supported frequency unit.
///
/// External types only need to implement [`Frequency::to_hertz`].
pub trait Frequency {
    /// Converts this frequency to microhertz.
    fn to_microhertz(&self) -> Microhertz {
        Microhertz(self.to_hertz().0 / 1e-6)
    }

    /// Converts this frequency to millihertz.
    fn to_millihertz(&self) -> Millihertz {
        Millihertz(self.to_hertz().0 / 1e-3)
    }

    /// Converts this frequency to hertz.
    fn to_hertz(&self) -> Hertz;

    /// Converts this frequency to kilohertz.
    fn to_kilohertz(&self) -> Kilohertz {
        Kilohertz(self.to_hertz().0 / 1e3)
    }

    /// Converts this frequency to megahertz.
    fn to_megahertz(&self) -> Megahertz {
        Megahertz(self.to_hertz().0 / 1e6)
    }

    /// Converts this frequency to gigahertz.
    fn to_gigahertz(&self) -> Gigahertz {
        Gigahertz(self.to_hertz().0 / 1e9)
    }

    /// Converts this frequency to terahertz.
    fn to_terahertz(&self) -> Terahertz {
        Terahertz(self.to_hertz().0 / 1e12)
    }

    /// Converts this frequency to petahertz.
    fn to_petahertz(&self) -> Petahertz {
        Petahertz(self.to_hertz().0 / 1e15)
    }

    /// Converts this frequency to revolutions per second.
    fn to_revolutions_per_second(&self) -> RevolutionsPerSecond {
        RevolutionsPerSecond(self.to_hertz().0)
    }

    /// Converts this frequency to revolutions per minute.
    fn to_revolutions_per_minute(&self) -> RevolutionsPerMinute {
        RevolutionsPerMinute(self.to_hertz().0 * 60.0)
    }

    /// Converts this frequency to beats per minute.
    fn to_beats_per_minute(&self) -> BeatsPerMinute {
        BeatsPerMinute(self.to_hertz().0 * 60.0)
    }

    /// Converts this frequency to angular frequency in radians per second.
    fn to_radians_per_second(&self) -> RadiansPerSecond {
        RadiansPerSecond(self.to_hertz().0 * std::f64::consts::TAU)
    }

    /// Returns this frequency divided by another frequency.
    fn ratio(&self, rhs: &dyn Frequency) -> f64 {
        self.to_hertz().0 / rhs.to_hertz().0
    }

    /// Returns the time occupied by one cycle.
    fn period(&self) -> Seconds {
        Seconds(1.0 / self.to_hertz().0)
    }

    /// Returns the cycles completed in a time interval.
    fn cycles_in(&self, time: &dyn Time) -> f64 {
        self.to_hertz().0 * time.to_seconds().0
    }

    /// Returns whether the canonical value is finite.
    fn is_finite(&self) -> bool {
        self.to_hertz().0.is_finite()
    }
}

macro_rules! define_frequency_unit {
    ($name:ident, $method:ident, $symbol:literal, $factor:expr, $doc:literal) => {
        #[doc = $doc]
        #[derive(Clone, Copy, Debug, Default)]
        pub struct $name(pub f64);

        impl Frequency for $name {
            fn to_hertz(&self) -> Hertz {
                Hertz(self.0 * $factor)
            }
        }

        impl_unit_common!($name, Frequency, $method, $symbol);
    };
}

define_frequency_unit!(
    Microhertz,
    to_microhertz,
    "µHz",
    1e-6,
    "A frequency measured in microhertz."
);
define_frequency_unit!(
    Millihertz,
    to_millihertz,
    "mHz",
    1e-3,
    "A frequency measured in millihertz."
);
define_frequency_unit!(Hertz, to_hertz, "Hz", 1.0, "A frequency measured in hertz.");
define_frequency_unit!(
    Kilohertz,
    to_kilohertz,
    "kHz",
    1e3,
    "A frequency measured in kilohertz."
);
define_frequency_unit!(
    Megahertz,
    to_megahertz,
    "MHz",
    1e6,
    "A frequency measured in megahertz."
);
define_frequency_unit!(
    Gigahertz,
    to_gigahertz,
    "GHz",
    1e9,
    "A frequency measured in gigahertz."
);
define_frequency_unit!(
    Terahertz,
    to_terahertz,
    "THz",
    1e12,
    "A frequency measured in terahertz."
);
define_frequency_unit!(
    Petahertz,
    to_petahertz,
    "PHz",
    1e15,
    "A frequency measured in petahertz."
);
define_frequency_unit!(
    RevolutionsPerSecond,
    to_revolutions_per_second,
    "r/s",
    1.0,
    "A frequency measured in revolutions per second."
);
define_frequency_unit!(
    RevolutionsPerMinute,
    to_revolutions_per_minute,
    "rpm",
    1.0 / 60.0,
    "A frequency measured in revolutions per minute."
);
define_frequency_unit!(
    BeatsPerMinute,
    to_beats_per_minute,
    "bpm",
    1.0 / 60.0,
    "A frequency measured in beats per minute."
);
define_frequency_unit!(
    RadiansPerSecond,
    to_radians_per_second,
    "rad/s",
    1.0 / std::f64::consts::TAU,
    "An angular frequency measured in radians per second."
);

/// Creates a frequency whose cycle has the supplied period.
///
/// A zero period follows IEEE-754 division and produces infinite hertz.
pub fn from_period(period: &dyn Time) -> Hertz {
    Hertz(1.0 / period.to_seconds().0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::time::Milliseconds;

    fn assert_close(actual: f64, expected: f64) {
        let scale = actual.abs().max(expected.abs()).max(1.0);
        assert!((actual - expected).abs() <= f64::EPSILON * 16.0 * scale);
    }

    #[test]
    fn electronic_and_periodic_units_convert() {
        assert_close(Megahertz(1.0).to_hertz().0, 1e6);
        assert_close(Petahertz(1.0).to_terahertz().0, 1_000.0);
        assert_close(RevolutionsPerMinute(3_600.0).to_hertz().0, 60.0);
        assert_close(BeatsPerMinute(120.0).to_hertz().0, 2.0);
        assert_close(RadiansPerSecond(std::f64::consts::TAU).to_hertz().0, 1.0);
    }

    #[test]
    fn periods_and_cycles_are_inverse() {
        let frequency = Kilohertz(2.0);
        assert_close(frequency.period().to_microseconds().0, 500.0);
        assert_close(frequency.cycles_in(&Milliseconds(10.0)), 20.0);
        assert_close(from_period(&frequency.period()).0, 2_000.0);
    }
}
