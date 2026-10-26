# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

This is the first release of `typed-css`. It continues `leptos-css` 0.1.1 under a name that does not depend on a
framework, so its versions and tags follow on from `leptos-css`. The entries below describe the changes since
`leptos-css` 0.1. Every change is breaking unless it is listed under "Added".

### Changed

- The crate is now called `typed-css`, so imports change from `leptos_css::...` to `typed_css::...`. The Leptos
  integration needs the `leptos` feature, which is off by default. Without it, the crate has no dependencies.
- Types use the names of CSS without a `Css` prefix, such as `Color` for `CssColor` and `Length` for `CssLength`.
  `CheckedDeclaration` is now `Declaration`, `CssWriteTo` is now `ToCss`, and `css_min` and the other function
  helpers lose their `css_` prefix. The fallible constructors are associated functions, such as `Length::try_px`.
- Each property is a `Property<T>` constant, such as `property::WIDTH`, instead of a unit struct, such as
  `WidthProperty`. The type `T` is the grammar of the property's values. Properties with the same grammar have
  the same type, so code can take the property to declare as a parameter. Properties accept the common parts of
  their grammar directly, such as `WIDTH.declare(px(120))` and `Z_INDEX.declare(10)`.
- `px()` and the other length helpers return a `Length`, and `pct()` returns a `Percentage`. `deg()` and `ms()` and
  the other helpers return an `Angle` or a `Time`. These types are `Copy` structs. Arithmetic on lengths and
  percentages creates a `LengthPercentage`, which also holds `var()` references and math expressions.
- `NonNegativeLengthPercentage` converts from every length-percentage. A negative value is written inside `calc()`,
  so that the browser clamps it to zero.
- `Padding`, `Margin`, `Inset`, and `Gap` store every component and are written in the shortest equivalent
  form. The four-edge shorthands are created with `all`, `symmetric`, or `edges` and changed with `with_top` and the
  other edge methods. The two-value shorthands, such as `PaddingAxis` and `Gap`, are created with `both` or `new` and
  changed with a `with_` method for each value. The variants for single sides are removed.
- `Declaration` implements `Display` and `ToCss` as `property:value;`, which replaces `write_declaration_to`.
  `Declaration::value` returns the value alone.
- `var(&property)` creates a reference without a fallback, and `var_or(&property, fallback)` takes a fallback. A
  `var()` of a grammar converts into every grammar that contains it. A custom-property name may be `--` followed by any
  ASCII letters, digits, `-`, and `_`, such as `--2xl`.
- Only the value types of properties and the primitives that custom properties hold can be declared. Parts of a
  grammar, such as `TouchActionHorizontalPan`, and CSS-wide keywords can no longer be declared on their own.
  `property::ALL` takes CSS-wide keywords through `declare_global`.
- `Color` is written in the modern space-separated syntax, such as `rgb(255 0 0 / 0.5)`. Its variants have named
  fields with an optional alpha, so `Rgb` includes `Rgba` and `Hsl` includes `Hsla`. `Color` is no longer `Copy`.
  `hsl` and `hsla` take the hue as an `Angle`, such as `deg(200)`.
- `FontWeight` is `Normal`, `Bold`, `Bolder`, `Lighter`, or a number from 1 to 1000, created with `FontWeight::new`.
  The `W100` to `W900` variants are removed.
- `view-transition-name` rejects `match-element`, which View Transitions Level 2 reserves.

### Added

- Layout properties: `display`, `visibility`, `position`, `order`, `box-sizing`, `aspect-ratio`, and the `overflow`
  family.
- Flexbox, box alignment with the `place-*` shorthands, and grid with track lists, `repeat()`, `minmax()`,
  `fit-content()`, `fr`, auto placement, and line-based placement.
- `transform` with 2D and 3D transform functions, and the `translate`, `rotate`, and `scale` properties.
- `font-size`, `font-style`, `line-height`, `text-align`, the `text-decoration-*` longhands, and
  `text-underline-offset`.
- `border-radius` and the physical corner-radius longhands.
- `cursor`, `pointer-events`, `user-select`, `scroll-behavior`, `scrollbar-gutter`, and the `overscroll-behavior`
  family.
- The colors `hex()`, `hwb()`, `oklab()`, `oklch()`, `light_dark()`, and `color_mix()`, every named color of CSS, and
  the system colors in `SystemColor`. `Color::with_alpha` makes any color translucent.
- A method on each property for each keyword that the property accepts on its own, such as `DISPLAY.grid()`, and a
  method for each CSS-wide keyword, such as `PADDING.inherit()`.
- `Auto`, which converts into every grammar with an `auto` keyword, so `WIDTH.declare(Auto)` works.
- Multiplication and division of lengths, percentages, and length-percentages by a number, negation, and arithmetic on
  `var()` references, such as `var(&SPACE) * 2`.
- `Number`, `NonNegativeNumber`, `UnitInterval`, `Percentage`, and `i32` as values of custom properties.
- The length units `pt`, `ex`, `lh`, `rlh`, `cqi`, `cqb`, `cqmin`, and `cqmax`.
- `Opacity::percent`, which writes the opacity as a percentage.

### Removed

- `CssValue` and its `number()` constructor. `fr()` now creates a grid `TrackBreadth`.
- `NonNegativeLengthPercentageValue`, `InvalidNonNegativeLengthPercentage`, and `LengthPercentageCalculation`.
- The `PropertyName` list. `Property::name` returns the name of a property.
- The `nightly` feature, which only enabled the nightly features of Leptos.

## leptos-css [0.1.1] - 2026-10-26

### Changed

- `leptos-css` is renamed to [`typed-css`](https://crates.io/crates/typed-css). This is the final `leptos-css`
  release, and the README points to the new crate. The library code is unchanged.

## leptos-css [0.1.0] - 2026-07-17

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

[Unreleased]: https://github.com/lpotthast/typed-css/compare/v0.1.1...HEAD

[0.1.1]: https://github.com/lpotthast/typed-css/compare/v0.1.0...v0.1.1

[0.1.0]: https://github.com/lpotthast/typed-css/releases/tag/v0.1.0
