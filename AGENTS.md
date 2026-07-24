# Repository Guidelines

## Project Structure & Module Organization

This repository is an early-stage Rust library for type-safe units of measure. At present, the root contains project documentation (`README.md`), the Apache 2.0 license (`LICENSE`), and Git ignore rules. When adding the crate, follow the conventional Cargo layout:

- `Cargo.toml` — package metadata and dependencies.
- `src/lib.rs` — public API and module declarations.
- `src/<quantity>.rs` — quantity-specific implementations, such as `distance.rs`, `mass.rs`, or `time.rs`.
- `tests/*.rs` — integration and compile-time API tests. Liberally use compile_fail doc tests.
- `examples/*.rs` — small, runnable usage demonstrations.

Keep conversion logic near its quantity type. Re-export intended public types and traits from `lib.rs`; leave implementation details private.

## Build, Test, and Development Commands

Once `Cargo.toml` is present, use the standard Rust toolchain:

- `cargo check` — quickly type-check the crate.
- `cargo build` — compile the debug build.
- `cargo test` — run unit, integration, and documentation tests.
- `cargo fmt --all -- --check` — verify canonical Rust formatting.
- `cargo clippy --all-targets --all-features -- -D warnings` — catch common mistakes and treat warnings as failures.
- `cargo doc --no-deps` — build local API documentation.

Run formatting, Clippy, and tests before submitting changes.

## Coding Style & Naming Conventions

Use `rustfmt` defaults (four-space indentation). Follow Rust naming conventions: `snake_case` for modules and functions, `UpperCamelCase` for types and traits, and `SCREAMING_SNAKE_CASE` for constants. Prefer explicit unit names such as `to_miles` and `from_kilograms`. Public APIs should include `///` documentation and examples where practical. Avoid unchecked or lossy conversions unless their behavior is clearly documented.

## Testing Guidelines

Place focused unit tests beside implementation code in `#[cfg(test)]` modules and public-behavior tests under `tests/`. Name tests after behavior, for example `meters_convert_to_miles`. Cover conversion identities, known reference values, round trips, zero, negatives where meaningful, and floating-point tolerance. Add compile-fail documentation tests when demonstrating rejected unit misuse.

## Commit & Pull Request Guidelines

History currently contains only `Initial commit`, so no established convention exists. Use short, imperative subjects such as `Add distance conversion trait`, with one logical change per commit. Pull requests should explain the motivation, summarize API changes, list verification commands, and link relevant issues. Include examples or updated documentation for user-facing behavior; screenshots are unnecessary unless visual documentation is introduced.

## Objective

Create a Rust crate for publication to crates.io and use in other projects.
