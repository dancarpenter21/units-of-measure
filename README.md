# units-of-measure

A Rust library for explicitly typed physical quantities. Version 0.2 represents
derived quantities by retaining their physical components instead of reducing
them to a scalar compound unit.

```toml
[dependencies]
units-of-measure = "0.2"
```

## One-dimensional units

Distance, time, mass, and angle units are lightweight `f64` newtypes. Their traits
provide explicit conversions and same-quantity arithmetic.

```rust
use units_of_measure::distance::{Distance, Feet, Meters};

let length = Feet::new(3.0);
let meters: Meters = length.to_meters();

assert!((meters.value() - 0.9144).abs() < 1e-12);
```

Applications can implement `Distance`, `Time`, or `Mass` for their own unit
types by defining the conversion to the canonical unit.

## Composed quantities

Derived values retain the exact units supplied at construction. `Speed`, for
example, is distance divided by time rather than an opaque meters-per-second
number:

```rust
use units_of_measure::{
    distance::{Distance, Meters},
    speed::Speed,
    time::{Seconds, Time},
};

let speed = Speed::new(Meters::new(10.0), Seconds::new(1.0));
let miles_per_hour = speed.distance().to_miles().value()
    / speed.time().to_hours().value();

assert!((miles_per_hour - 22.369_362_920_544).abs() < 1e-12);
```

Functions can accept any concrete component units while preserving type safety:

```rust
use units_of_measure::{
    distance::Distance,
    speed::Speed,
    time::Time,
};

fn miles_per_hour<D: Distance, T: Time>(speed: &Speed<D, T>) -> f64 {
    speed.distance().to_miles().value() / speed.time().to_hours().value()
}
```

The available composed concepts are:

- `Area<W, H>` — width and height.
- `Speed<D, T>` — distance and time.
- `Acceleration<D, T1, T2>` — distance divided by two time components.
- `Weight<M, D, T1, T2>` — mass times distance divided by two time components.
- `Frequency<T>` — a dimensionless cycle count and duration.
- `AngularVelocity<A, T>` — angle and time.
- `AngularAcceleration<A, T1, T2>` — angle divided by two time components.
- `Torque<M, D1, D2, T1, T2>` — mass times two distances divided by two time components.

All component accessors return references in their original units. Composed
quantities deliberately do not provide canonical scalar values or
cross-dimensional operators: convert the components relevant to the
calculation, then call `value()`.

Flattened concepts also provide constructors that accept their immediate
physical source concepts. Use `Acceleration::from_speed_and_time`,
`Weight::from_mass_and_acceleration`,
`AngularAcceleration::from_angular_velocity_and_time`, and
`Torque::from_weight_and_distance` when those inputs are already available.

```rust
use units_of_measure::{
    acceleration::Acceleration,
    distance::Meters,
    mass::Kilograms,
    time::Seconds,
    weight::Weight,
};

let acceleration = Acceleration::new(
    Meters::new(9.80665),
    Seconds::new(1.0),
    Seconds::new(1.0),
);
let weight = Weight::new(
    Kilograms::new(80.0),
    Meters::new(9.80665),
    Seconds::new(1.0),
    Seconds::new(1.0),
);

assert_eq!(acceleration.distance().value(), weight.distance().value());
```

```rust
use units_of_measure::{
    acceleration::Acceleration,
    distance::Meters,
    mass::Kilograms,
    speed::Speed,
    time::Seconds,
    weight::Weight,
};

let acceleration = Acceleration::from_speed_and_time(
    Speed::new(Meters::new(9.80665), Seconds::new(1.0)),
    Seconds::new(1.0),
);
let weight = Weight::from_mass_and_acceleration(Kilograms::new(80.0), acceleration);
```

`STANDARD_GRAVITY` is provided as an
`Acceleration<Meters, Seconds, Seconds>`.

## Rotational quantities

Angles convert explicitly between degrees and radians. Rotational quantities
retain the same typed factors as the rest of the crate:

```rust
use units_of_measure::{
    angle::{Angle, Degrees},
    angular_velocity::AngularVelocity,
    time::{Seconds, Time},
};

let rotation = AngularVelocity::new(Degrees::new(180.0), Seconds::new(2.0));
let radians_per_second = rotation.angle().to_radians().value()
    / rotation.time().to_seconds().value();

assert!((radians_per_second - std::f64::consts::PI / 2.0).abs() < 1e-12);
```

## Frequency and audio

Frequency records cycles over a duration. For example, 440 cycles over one
second is concert A4:

```rust
use units_of_measure::{audio, frequency::Frequency, time::Seconds};

let a4 = Frequency::new(440.0, Seconds::new(1.0));
assert!(audio::is_nominally_audible(&a4));
assert_eq!(audio::midi_note_frequency(69), audio::CONCERT_A4);
```

## Serialization

The optional default `serde` feature keeps primitive units as numbers. Composed
values serialize as their named components, so a
`Speed<Meters, Seconds>` serializes as `{"distance":10.0,"time":1.0}`.

## Intentional type errors

Invalid component combinations are rejected by the compiler:

```compile_fail
use units_of_measure::{
    distance::Meters,
    mass::Kilograms,
    speed::Speed,
};

let invalid = Speed::new(Meters::new(10.0), Kilograms::new(1.0));
```

The crate forbids unsafe code.
