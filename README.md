# units-of-measure

A dependency-free Rust library for typed units, explicit conversions, and
dimensional arithmetic.

Each physical quantity is an open Rust trait. Individual units are small public
tuple structs implementing that trait:

```rust
use units_of_measure::distance::{Distance, Feet, Meters};

let original = Feet(3.0);
let metric: Meters = original.to_meters();

assert!((metric.0 - 0.9144).abs() < 1e-12);
```

This makes unit requirements visible in function signatures while allowing
callers to use whichever supported unit is convenient:

```rust
use units_of_measure::distance::{Distance, Kilometers, Meters};

fn construction_length(distance: &dyn Distance) -> Meters {
    distance.to_meters()
}

assert_eq!(construction_length(&Kilometers(2.0)), Meters(2_000.0));
```

## Install

```toml
[dependencies]
units-of-measure = "0.1"
```

The crate has no runtime or build dependencies and forbids unsafe code.

## Traits and units

Traits are available at the crate root and within their modules. Unit structs
live in quantity modules to keep imports unambiguous:

```rust
use units_of_measure::Distance;
use units_of_measure::distance::{Feet, Miles};

fn miles<D: Distance>(distance: &D) -> Miles {
    distance.to_miles()
}

assert_eq!(miles(&Feet(5_280.0)), Miles(1.0));
```

Every unit is `Copy`, `Clone`, `Debug`, and `Default`, and provides:

- a public `f64` field, `new`, and `value`;
- `ZERO`, `abs`, and `is_finite`;
- display using its own unit symbol;
- addition and subtraction with any unit of the same quantity;
- scalar multiplication and division;
- mixed-unit comparisons and dimensionless ratios.

Mixed arithmetic preserves the unit on the left:

```rust
use units_of_measure::distance::{Feet, Meters};

let imperial: Feet = Feet(3.0) + Meters(1.0);
let metric: Meters = Meters(1.0) + Feet(3.0);

assert!((imperial.0 - 6.280839895013123).abs() < 1e-12);
assert!((metric.0 - 1.9144).abs() < 1e-12);
assert!(Meters(1.0) > Feet(3.0));
```

## Distance and area

Distance includes SI units from picometers through kilometers, ångströms,
inches, feet, yards, statute and nautical miles, astronomical units,
light-years, and parsecs.

Area mirrors the squared distance units and also includes ares, hectares,
acres, and barns:

```rust
use units_of_measure::{
    area::{Acres, Area, SquareFeet, SquareMeters},
    distance::{Distance, Feet, Meters},
};

// Dimensional results use canonical SI types.
let floor: SquareMeters = Feet(12.0) * Feet(15.0);
assert!((floor.to_square_feet().0 - 180.0).abs() < 1e-10);

let remaining_side: Meters = SquareFeet(180.0) / Feet(12.0);
assert!((remaining_side.to_feet().0 - 15.0).abs() < 1e-12);

let lot = Acres(1.0);
assert!((lot.to_square_meters().0 - 4_046.8564224).abs() < 1e-9);
```

Scientific definitions are explicit:

```rust
use units_of_measure::{
    area::{Area, Barns},
    distance::{AstronomicalUnits, Distance, LightYears, Parsecs},
};

assert_eq!(Barns(1.0).to_square_meters().0, 1e-28);
assert_eq!(
    AstronomicalUnits(1.0).to_meters().0,
    149_597_870_700.0
);
assert_eq!(LightYears(1.0).to_meters().0, 9_460_730_472_580_800.0);
assert!((Parsecs(1.0).to_light_years().0 - 3.261563777167434).abs() < 1e-12);
```

## Time, speed, and acceleration

Time covers femtoseconds through weeks, Julian years, and explicitly named mean
Gregorian months and years. Speed includes common metric, customary, nautical,
and standard-atmosphere Mach units. Acceleration includes SI/customary units,
gals, milligals, and standard gravity.

```rust
use units_of_measure::{
    acceleration::{Acceleration, StandardGravities},
    distance::{Distance, Miles},
    speed::{MilesPerHour, Speed},
    time::{Hours, Time},
};

let speed = Miles(120.0) / Hours(2.0);
assert!((speed.to_miles_per_hour().0 - 60.0).abs() < 1e-12);

let trip = MilesPerHour(60.0) * Hours(2.0);
assert!((trip.to_miles().0 - 120.0).abs() < 1e-12);

assert_eq!(
    StandardGravities(1.0).to_meters_per_second_squared().0,
    9.80665
);
```

Context-dependent names use documented reference assumptions:

- `Mach(1.0)` uses standard sea-level sound speed, `340.294 m/s`.
- `MeanGregorianYears(1.0)` is `365.2425` days.
- `MeanGregorianMonths(1.0)` is one twelfth of a mean Gregorian year.
- `JulianYears(1.0)` is exactly `365.25` days.

```rust
use units_of_measure::{
    speed::{Mach, Speed},
    time::{MeanGregorianMonths, MeanGregorianYears, Time},
};

assert_eq!(Mach(1.0).to_meters_per_second().0, 340.294);
assert_eq!(MeanGregorianYears(1.0).to_days().0, 365.2425);
assert_eq!(
    MeanGregorianMonths(12.0).to_mean_gregorian_years().0,
    1.0
);
```

## Mass and weight

Mass and weight are distinct traits and types:

- `mass::Kilograms` describes mass independent of gravity.
- `weight::Newtons` describes force.
- `mass::Pounds` and `mass::Ounces` are pounds-mass and ounces-mass.
- `weight::Pounds` and `weight::Ounces` are pounds-force and ounces-force.
- `weight::Kilograms` is the colloquial scale unit kilogram-force.

```rust
use units_of_measure::{
    acceleration::MetersPerSecondSquared,
    mass::{Kilograms, Mass},
    weight::Weight,
};

let astronaut = Kilograms(80.0);
let earth_weight = astronaut.weight_at_standard_gravity();
let moon_weight = astronaut.weight_at(&MetersPerSecondSquared(1.62));

assert!((earth_weight.to_newtons().0 - 784.532).abs() < 1e-12);
assert!((moon_weight.to_newtons().0 - 129.6).abs() < 1e-12);
assert!((moon_weight.mass_at(&MetersPerSecondSquared(1.62)).0 - 80.0).abs() < 1e-12);
```

Mass also supports SI units from nanograms through metric tonnes, carats,
grains, stones, short and long tons, troy ounces, slugs, daltons, and unified
atomic mass units. Dalton conversions use the 2022 CODATA central value.

## Audio and electronic frequency

Frequency covers microhertz through petahertz, revolutions, beats, and angular
frequency. Multiplying frequency by time returns a cycle count:

```rust
use units_of_measure::{
    frequency::{Frequency, Kilohertz, Megahertz},
    time::{Milliseconds, Time},
};

let samples = Kilohertz(48.0) * Milliseconds(10.0);
assert!((samples - 480.0).abs() < 1e-12);

let clock = Megahertz(16.0);
assert!((clock.period().to_nanoseconds().0 - 62.5).abs() < 1e-12);
```

The audio helpers accept any frequency implementation:

```rust
use units_of_measure::{audio, frequency::Kilohertz};

let a4 = audio::midi_note_frequency(69);
assert_eq!(a4, audio::CONCERT_A4);
assert!(audio::is_nominally_audible(&a4));
assert!(audio::is_nominally_audible(&Kilohertz(20.0)));
assert!((audio::midi_note_number(&a4) - 69.0).abs() < 1e-12);
```

## Dimensional arithmetic

Built-in units support every valid pairing below. Result types are canonical SI
structs regardless of the operand units.

| Expression | Result |
| --- | --- |
| `Distance * Distance` | `SquareMeters` |
| `Area / Distance` | `Meters` |
| `Distance / Time` | `MetersPerSecond` |
| `Distance / Speed` | `Seconds` |
| `Speed * Time` or `Time * Speed` | `Meters` |
| `Speed / Time` | `MetersPerSecondSquared` |
| `Speed / Acceleration` | `Seconds` |
| `Acceleration * Time` or `Time * Acceleration` | `MetersPerSecond` |
| `Mass * Acceleration` or `Acceleration * Mass` | `Newtons` |
| `Weight / Mass` | `MetersPerSecondSquared` |
| `Weight / Acceleration` | `mass::Kilograms` |
| `Frequency * Time` or `Time * Frequency` | cycles as `f64` |

Unrelated arithmetic is rejected:

```compile_fail
use units_of_measure::{
    distance::Meters,
    mass::Kilograms,
};

let invalid = Meters(10.0) + Kilograms(10.0);
```

The documentation suite also checks the broader incompatible-operation matrix:

```compile_fail
use units_of_measure::{
    mass::Pounds as PoundsMass,
    weight::Pounds as PoundsForce,
};

let invalid = PoundsMass(10.0) + PoundsForce(10.0);
```

```compile_fail
use units_of_measure::{
    mass::Pounds as PoundsMass,
    weight::Pounds as PoundsForce,
};

let invalid = PoundsForce(10.0) + PoundsMass(10.0);
```

```compile_fail
use units_of_measure::{
    mass::Kilograms as MassKilograms,
    weight::Kilograms as WeightKilograms,
};

let invalid = MassKilograms(10.0) + WeightKilograms(10.0);
```

```compile_fail
use units_of_measure::{distance::Meters, time::Seconds};

let invalid = Meters(1.0) + Seconds(1.0);
```

```compile_fail
use units_of_measure::{area::SquareMeters, distance::Meters};

let invalid = SquareMeters(1.0) + Meters(1.0);
```

```compile_fail
use units_of_measure::{frequency::Hertz, time::Seconds};

let invalid = Seconds(1.0) + Hertz(1.0);
```

```compile_fail
use units_of_measure::{
    acceleration::MetersPerSecondSquared,
    speed::MetersPerSecond,
};

let invalid = MetersPerSecond(1.0) + MetersPerSecondSquared(1.0);
```

```compile_fail
use units_of_measure::{
    mass::Kilograms,
    weight::Newtons,
};

let invalid = Newtons(1.0) + Kilograms(1.0);
```

```compile_fail
use units_of_measure::{distance::Meters, mass::Kilograms};

let invalid = Meters(1.0) / Kilograms(1.0);
```

```compile_fail
use units_of_measure::{area::SquareMeters, time::Seconds};

let invalid = SquareMeters(1.0) / Seconds(1.0);
```

```compile_fail
use units_of_measure::{mass::Kilograms, speed::MetersPerSecond};

let invalid = MetersPerSecond(1.0) * Kilograms(1.0);
```

```compile_fail
use units_of_measure::{distance::Meters, frequency::Hertz};

let invalid = Hertz(1.0) * Meters(1.0);
```

```compile_fail
use units_of_measure::{mass::Kilograms, time::Hours};

let invalid = Kilograms(1.0) / Hours(1.0);
```

```compile_fail
use units_of_measure::{time::Seconds, weight::Newtons};

let invalid = Newtons(1.0) * Seconds(1.0);
```

```compile_fail
use units_of_measure::{distance::Meters, time::Seconds};

let invalid = Meters(1.0) == Seconds(1.0);
```

```compile_fail
use units_of_measure::{distance::Meters, time::Seconds};

let mut distance = Meters(1.0);
distance += Seconds(1.0);
```

Canonical result types are also enforced:

```compile_fail
use units_of_measure::{
    distance::Miles,
    speed::MilesPerHour,
    time::Hours,
};

// Distance / Time returns MetersPerSecond, not MilesPerHour.
let invalid: MilesPerHour = Miles(60.0) / Hours(1.0);
```

```compile_fail
use units_of_measure::{
    area::SquareFeet,
    distance::Feet,
};

// Distance * Distance returns SquareMeters, not SquareFeet.
let invalid: SquareFeet = Feet(10.0) * Feet(10.0);
```

```compile_fail
use units_of_measure::{distance::Feet, time::Hours};

// Distance multiplied by time is not dimensionally meaningful.
let invalid = Feet(1.0) * Hours(1.0);
```

```compile_fail
use units_of_measure::{frequency::Hertz, time::Seconds};

// Time divided by frequency has no supported dimensional result.
let invalid = Seconds(1.0) / Hertz(1.0);
```

```compile_fail
use units_of_measure::{area::Acres, distance::Meters};

// Area multiplied by distance would be volume, which is not yet modeled.
let invalid = Acres(1.0) * Meters(1.0);
```

Operators are intentionally limited to built-in units; open-trait implementations
use the named calculation methods:

```compile_fail
use units_of_measure::{
    distance::{Distance, Meters},
    time::Seconds,
};

struct Smoots(f64);

impl Distance for Smoots {
    fn to_meters(&self) -> Meters {
        Meters(self.0 * 1.7018)
    }
}

let invalid = Smoots(1.0) / Seconds(1.0);
```

## JSON and YAML

The default `serde` feature implements `serde::Serialize` and
`serde::Deserialize` for every unit struct. Units use a transparent numeric
representation; the Rust target type supplies the unit:

```rust
use units_of_measure::distance::Meters;

# #[cfg(feature = "serde")]
# {
let json = serde_json::to_string(&Meters(12.5))?;
assert_eq!(json, "12.5");

let distance: Meters = serde_json::from_str("12.5")?;
assert_eq!(distance, Meters(12.5));
# }
# Ok::<(), serde_json::Error>(())
```

```rust
use units_of_measure::time::Seconds;

# #[cfg(feature = "serde")]
# {
let yaml = serde_yaml::to_string(&Seconds(3.25))?;
let duration: Seconds = serde_yaml::from_str(&yaml)?;
assert_eq!(duration, Seconds(3.25));
# }
# Ok::<(), serde_yaml::Error>(())
```

Applications choose and configure their serializer dependencies:

```toml
[dependencies]
units-of-measure = "0.1"
serde_json = "1"
serde_yaml = { package = "serde_yaml_ng", version = "0.10" }
```

Disable Serde integration with `default-features = false` when serialization is
not needed. JSON cannot represent non-finite floating-point values portably, so
validate external data with `is_finite` when those values are not acceptable.

## Custom units

Quantity traits are open and object-safe. Implement the canonical conversion to
inherit every typed conversion and named calculation:

```rust
use units_of_measure::distance::{Distance, Meters};
use units_of_measure::time::Hours;

struct Leagues(f64);

impl Distance for Leagues {
    fn to_meters(&self) -> Meters {
        Meters(self.0 * 4_828.032)
    }
}

let distance: &dyn Distance = &Leagues(1.0);
assert!((distance.to_miles().0 - 3.0).abs() < 1e-12);
assert!((distance.speed_over(&Hours(1.0)).0 - 1.34112).abs() < 1e-12);
```

Custom implementations use named calculation methods because Rust coherence
prevents overlapping generic operator implementations across multiple open
quantity traits. All crate-provided unit structs support the operator table.

## Numeric behavior

Values are signed `f64`s. Constructors preserve negative values, infinity, and
`NaN`; `is_finite` is available for validation. Division by zero follows normal
IEEE-754 behavior rather than panicking or silently clamping.

Unit types intentionally implement `PartialEq` and `PartialOrd`, not `Eq` or
`Ord`, because `NaN` is unordered.

## Development

```text
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo doc --no-deps
cargo package
```

Licensed under Apache-2.0.
