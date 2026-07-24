# Using `units-of-measure`

This guide is for coding agents working in projects that depend on this crate.
The crate models physical quantities as traits and individual units as public
`f64` tuple structs.

## Import the trait and concrete units

Conversion methods come from quantity traits, so keep the trait in scope:

```rust
use units_of_measure::distance::{Distance, Feet, Meters};

let metric: Meters = Feet(3.0).to_meters();
```

The traits are also re-exported from the crate root. Concrete units remain in
their quantity modules:

- `distance::{Distance, Feet, Meters, Miles, ...}`
- `area::{Area, SquareMeters, Acres, Hectares, ...}`
- `time::{Time, Seconds, Hours, ...}`
- `speed::{Speed, MetersPerSecond, MilesPerHour, ...}`
- `acceleration::{Acceleration, MetersPerSecondSquared, ...}`
- `mass::{Mass, Kilograms, Pounds, ...}`
- `weight::{Weight, Newtons, Pounds, ...}`
- `frequency::{Frequency, Hertz, Kilohertz, ...}`

Read a raw value with `.0` or `.value()`. Construct values with `Feet(3.0)` or
`Feet::new(3.0)`.

## Conversion and arithmetic rules

Trait conversions return typed units rather than raw numbers:

```rust
use units_of_measure::distance::{Distance, Kilometers, Miles};

let miles: Miles = Kilometers(5.0).to_miles();
```

Addition, subtraction, and comparison accept different units of the same
quantity. The result preserves the left operand's unit:

```rust
use units_of_measure::distance::{Feet, Meters};

let result: Feet = Feet(3.0) + Meters(1.0);
```

Dimensional operations on built-in units return canonical SI structs:

| Operation | Result |
| --- | --- |
| `Distance * Distance` | `area::SquareMeters` |
| `Area / Distance` | `distance::Meters` |
| `Distance / Time` | `speed::MetersPerSecond` |
| `Distance / Speed` | `time::Seconds` |
| `Speed * Time` | `distance::Meters` |
| `Speed / Time` | `acceleration::MetersPerSecondSquared` |
| `Mass * Acceleration` | `weight::Newtons` |
| `Weight / Acceleration` | `mass::Kilograms` |
| `Frequency * Time` | cycle count as `f64` |

Convert the result afterward when a non-SI unit is required.

## Keep mass and weight distinct

`mass::Pounds` is pounds-mass. `weight::Pounds` is pounds-force.
`mass::Kilograms` is mass, while `weight::Kilograms` is the colloquial
kilogram-force scale unit. Alias imports when both quantities are used:

```rust
use units_of_measure::{
    mass::Pounds as PoundsMass,
    weight::Pounds as PoundsForce,
};
```

Do not add or compare values from these two modules. Convert mass to weight with
`Mass::weight_at` or `Mass::weight_at_standard_gravity`.

## JSON and YAML

Serde support is enabled by default for every built-in unit. The serialized form
is a plain number:

```rust
use units_of_measure::distance::Meters;

let json = serde_json::to_string(&Meters(2.5))?;
let restored: Meters = serde_json::from_str(&json)?;
# Ok::<(), serde_json::Error>(())
```

YAML works through any Serde-compatible YAML crate:

```rust
use units_of_measure::time::Seconds;

let yaml = serde_yaml::to_string(&Seconds(2.5))?;
let restored: Seconds = serde_yaml::from_str(&yaml)?;
# Ok::<(), serde_yaml::Error>(())
```

The examples use the maintained `serde_yaml_ng` package under a `serde_yaml`
dependency alias:

```toml
serde_yaml = { package = "serde_yaml_ng", version = "0.10" }
```

The payload does not carry a unit tag. The deserialization target determines
the unit, so deserialize into the exact type required by the surrounding API.

## Custom units

Quantity traits are open and object-safe. Implement only the canonical SI
conversion:

```rust
use units_of_measure::distance::{Distance, Meters};

struct Smoots(f64);

impl Distance for Smoots {
    fn to_meters(&self) -> Meters {
        Meters(self.0 * 1.7018)
    }
}
```

The custom type inherits all conversion and named calculation methods. Standard
operators are generated only for built-in units, so use methods such as
`speed_over`, `area_with`, `weight_at`, and `cycles_in` for custom types.

## Contextual assumptions

- `speed::Mach` uses `340.294 m/s`, standard sea-level sound speed.
- `time::MeanGregorianYears` uses `365.2425` days.
- `time::MeanGregorianMonths` uses one twelfth of that mean year.
- `time::JulianYears` uses exactly `365.25` days.
- `acceleration::StandardGravities` uses `9.80665 m/s²`.

Do not use these fixed reference units when the application requires local
atmospheric conditions or calendar-aware date arithmetic.

## Agent checklist

1. Import the appropriate quantity trait before calling conversion methods.
2. Keep concrete unit types in public function signatures whenever the required
   input unit is intentionally strict; accept `impl Trait` or `&dyn Trait` when
   callers may choose units.
3. Expect dimensional operators to return canonical SI unit structs.
4. Never mix `mass` and `weight` values directly.
5. Use approximate comparisons for values that have undergone floating-point
   conversion.
6. Run `cargo test`, `cargo clippy --all-targets --all-features -- -D warnings`,
   and the documentation tests after changing units or operators.
