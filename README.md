# leptos-css

[![crates.io](https://img.shields.io/crates/v/leptos-css.svg)](https://crates.io/crates/leptos-css)
[![docs.rs](https://docs.rs/leptos-css/badge.svg)](https://docs.rs/leptos-css)
[![CI](https://github.com/lpotthast/leptos-css/actions/workflows/ci.yml/badge.svg)](https://github.com/lpotthast/leptos-css/actions/workflows/ci.yml)
[![MSRV](https://img.shields.io/badge/rust-1.89%2B-blue.svg)](https://www.rust-lang.org/)

> [!IMPORTANT]
> **`leptos-css` has been renamed to [`typed-css`](https://crates.io/crates/typed-css).** This is the final
> `leptos-css` release and receives no further updates. `typed-css` is the same library under a framework-neutral
> name, with its Leptos integration behind the `leptos` feature:
>
> ```toml
> [dependencies]
> typed-css = { version = "0.2", features = ["leptos"] }
> ```
>
> To migrate, change your imports from `leptos_css::...` to `typed_css::...`. `typed-css` 0.2 also redesigns the
> API and adds many properties. Its [changelog](https://github.com/lpotthast/typed-css/blob/main/CHANGELOG.md) lists
> every change since `leptos-css` 0.1.
>
> The documentation below describes `leptos-css` 0.1.

Typed CSS primitives and property-specific declaration grammars for
[Leptos](https://leptos.dev). The checked API is deliberately fail-closed: unsupported CSS cannot be smuggled in as
an arbitrary value string, and a value can become a declaration only through a selector for a property whose
grammar it satisfies.

`leptos-css` is the typed foundation of the [`leptos-styles`](https://crates.io/crates/leptos-styles) family. Its
checked boundary is the owned `CheckedDeclaration` returned by a property selector. Integrations can store and pass
that concrete type without separating its checked property/value pairing.

Applications can use handwritten CSS or Leptos string styles for declarations outside the checked API.

## Features

- Closed value types for lengths, percentages, colors, angles, times, global keywords, and selected property grammars
- Constructor helpers (`px`, `pct`, `em`, `rem`, `vh`, `vw`, `deg`, `rgb`, `hsl`, ...) with finite-input validation,
  plus fallible `try_*` counterparts for runtime-derived values. Helpers return the narrowest modeled grammar, so
  `rgb()` and `hsl()` feed every selector accepting `CssColor` directly.
- Typed arithmetic and typed `css_min()`, `css_max()`, `css_clamp()`, and `css_env()` constructors; function bodies are
  expression trees rather than strings
- Validated, grammar-typed custom properties and `var()` references with mandatory typed fallbacks
- [Property selectors](https://docs.rs/leptos-css/latest/leptos_css/property/) for checked Leptos declarations,
  including padding, margin, inset, sizing and radius properties, colors, opacity, color adjustment, view transitions,
  font weight, touch action, and z-index
- A curated, non-exhaustive `PropertyName` catalog with kebab-case `as_str()` output; it is neither a declaration API
  nor a complete CSS property index

## Installation

```bash
cargo add leptos-css
```

`leptos-css` is compatible with Leptos 0.8 and requires Rust 1.89 or newer.

Enable the `nightly` feature when the application also enables `leptos/nightly`:

```bash
cargo add leptos-css --features nightly
```

## Example

```rust
use leptos::prelude::{ElementChild, StyleAttribute, view};
use leptos_css::{
    CssColor, CssColorName, CssWriteTo, LengthPercentageCalculation, Padding, Size, TouchAction,
    TouchActionGestures, TouchActionHorizontalPan, TouchActionVerticalPan, css_custom_property, pct,
    px, rgb, try_px, var,
    property::{ColorProperty, PaddingProperty, TouchActionProperty, WidthProperty},
};

css_custom_property!(ACCENT_COLOR: CssColor = "--accent-color");

let mut buf = String::new();
px(16).write_to(&mut buf);
assert_eq!(buf, "16px");

let measured_width = try_px(24.5).expect("24.5 is a finite CSS length");
assert_eq!(measured_width.to_string(), "24.5px");

let padding = Padding::all(px(16));
assert_eq!(padding.to_string(), "16px");
let padding_declaration = PaddingProperty.declare(padding);
let _view = view! {
    <div style=padding_declaration>"Typed padding"</div>
};
let _rgb_declaration = ColorProperty.declare(rgb(12, 34, 56));

let width = Size::Calculation(LengthPercentageCalculation::new(pct(100) - px(20)));
assert_eq!(width.to_string(), "calc(100% - 20px)");
let _width_declaration = WidthProperty.declare(width);

let touch_gestures = TouchActionGestures::horizontal(TouchActionHorizontalPan::PanX)
    .with_vertical(TouchActionVerticalPan::PanY)
    .with_pinch_zoom();
let _touch_declaration = TouchActionProperty.declare(TouchAction::Gestures(touch_gestures));

let _accent_declaration = ACCENT_COLOR.declare(CssColor::Named(CssColorName::Aqua));
let _color_declaration = ColorProperty.declare(var(
    &ACCENT_COLOR,
    CssColor::Named(CssColorName::CurrentColor),
));
```

## Usage notes

- Convenience constructors (`px`, `pct`, `deg`, `s`, `hsl`, ...) panic on invalid input. Use their fallible `try_*`
  counterparts for runtime-derived values.
- Property selectors enforce their CSS grammars. Margin and inset accept `LengthPercentageAuto`, while padding and gap
  accept non-negative length-percentage values, so `auto` and directly negative values work only where CSS permits.
- Ordinary property selectors accept CSS-wide keywords through `declare_global()`. `AllProperty` accepts only a
  CSS-wide keyword.
- Public enums are `#[non_exhaustive]`. Downstream matches must include a wildcard arm.

## License

Dual licensed under either of:

- MIT License ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

at your option.
