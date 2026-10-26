# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1]

### Changed

- `leptos-css` is renamed to [`typed-css`](https://crates.io/crates/typed-css). This is the final `leptos-css`
  release, and the README points to the new crate. The library code is unchanged.

## [0.1.0] - 2026-07-17

### Added

- Typed CSS primitives with validated convenience constructors and fallible `try_*` variants for runtime-derived
  finite numbers, lengths, percentages, colors, angles, times, and fractional units.
- Closed, property-specific declaration grammars for supported box-model, sizing, color, typography, interaction,
  stacking, and view-transition properties, including CSS-wide keywords where valid.
- Typed arithmetic and helpers for CSS `min()`, `max()`, `clamp()`, and supported `env()` variables, with range
  constraints preserved through calculations.
- Grammar-typed custom properties and `var()` references with mandatory, matching, recursively nestable fallbacks.
- A fail-closed `CheckedDeclaration` that preserves its validated property/value pairing and integrates directly with
  Leptos style attributes for server rendering, hydration, browser updates, and resets.
- A `PropertyName` catalog for converting curated modern property names to kebab-case text, separate from the checked
  declaration API.
- Open `CssWriteTo` serialization for application-defined values and `Display` rendering for built-in grammars.
  Checked declarations remain limited to crate-validated values.
- Compatibility with Leptos 0.8 and Rust 1.89 or newer, with an optional `nightly` feature forwarded to Leptos.
  Public enums are non-exhaustive.

[Unreleased]: https://github.com/lpotthast/leptos-css/compare/v0.1.1...HEAD

[0.1.1]: https://github.com/lpotthast/leptos-css/compare/v0.1.0...v0.1.1

[0.1.0]: https://github.com/lpotthast/leptos-css/releases/tag/v0.1.0
