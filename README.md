# typed-css

[![crates.io](https://img.shields.io/crates/v/typed-css.svg)](https://crates.io/crates/typed-css)
[![docs.rs](https://docs.rs/typed-css/badge.svg)](https://docs.rs/typed-css)
[![CI](https://github.com/lpotthast/typed-css/actions/workflows/ci.yml/badge.svg)](https://github.com/lpotthast/typed-css/actions/workflows/ci.yml)
[![MSRV](https://img.shields.io/badge/rust-1.89%2B-blue.svg)](https://www.rust-lang.org/)

`typed-css` writes CSS declarations from Rust values that the compiler checks. Each property knows which values it
accepts, so `WIDTH.declare(px(120))` compiles and `WIDTH.declare(NamedColor::Red)` does not. Numbers are checked when
you create a value, so `NaN` or an opacity above 1 never reaches CSS. There is no way to pass arbitrary text, so a
declaration cannot contain broken or injected CSS.

The result is a `Declaration` such as `width:120px;`. You can write it into a `style` attribute, an HTML template, or
any UI framework. With the `leptos` feature, a declaration also renders directly as a [Leptos](https://leptos.dev)
style attribute.

If you build a Leptos application, use [`leptos-styles`](https://crates.io/crates/leptos-styles). It builds on
`typed-css` and collects the declarations of an element in one value. Components can pass that value on and add to
it, declarations can update reactively, and CSS that `typed-css` does not support can be mixed in.

## Installation

```bash
cargo add typed-css
```

`typed-css` requires Rust 1.89 or newer and has no dependencies by default.

## Example

```rust
use typed_css::{
    LengthPercentage, NamedColor, ToCss, TransformFunction, custom_property, hex, min, pct, px,
    rem, var, var_or,
    property::{BACKGROUND_COLOR, COLOR, DISPLAY, GAP, JUSTIFY_CONTENT, PADDING, TRANSFORM, WIDTH},
};

custom_property!(SPACE: LengthPercentage = "--space");

let declarations = [
    DISPLAY.flex(),
    JUSTIFY_CONTENT.center(),
    WIDTH.declare(min(pct(100) - px(32), rem(40))),
    PADDING.declare(var_or(&SPACE, px(16))),
    GAP.declare(var(&SPACE)),
    COLOR.declare(NamedColor::White),
    BACKGROUND_COLOR.declare(hex(0x1e90ff)),
    TRANSFORM.declare(TransformFunction::translate(px(0), px(-4))),
];
let mut style = String::new();
for declaration in &declarations {
    declaration.to_css(&mut style).unwrap();
}

assert_eq!(
    style,
    "display:flex;justify-content:center;width:min(100% - 32px, 40rem);\
     padding:var(--space, 16px);gap:var(--space);color:white;\
     background-color:rgb(30 144 255);transform:translate(0px, -4px);",
);
```

## How it works

### Properties and declarations

Each supported property is a constant in the `property` module, such as `property::WIDTH`. The constant is a
`Property<Size>`, so it knows that `width` takes a `Size`. `declare` turns a value into a `Declaration`.

A property also accepts the common parts of its grammar directly. `WIDTH.declare(px(120))` works without writing
`Size::from(px(120))`. Properties with the same grammar have the same type, so a function can take a
`Property<Color>` and declare either `COLOR` or `BACKGROUND_COLOR`.

Each keyword that a property accepts on its own also has a method. `DISPLAY.grid()` declares `display:grid`, and
`JUSTIFY_CONTENT.space_between()` declares `justify-content:space-between`. The methods cover keywords such as
`auto`, `none`, and `min-content`, but not colors. Two methods use raw identifiers, because their keywords are Rust
keywords: `POSITION.r#static()` and `CURSOR.r#move()`.

Every property and custom property also accepts the CSS-wide keywords. Each keyword has a method, such as
`PADDING.inherit()` or `ALL.revert_layer()`. `declare_global` takes a `GlobalKeyword` that is only known at runtime.

### Lengths, percentages, and calculations

A `Length` is a distance with a unit, such as `px(16)` or `rem(2)`. A `Percentage` is a share of a reference size,
such as `pct(50)`. Most size properties accept both, and also a calculation that combines them, such as
`calc(100% - 20px)`. CSS calls this type `<length-percentage>`, and `LengthPercentage` models it.

You rarely name `LengthPercentage` yourself. Lengths and percentages convert into it, and arithmetic creates it. For
example, `pct(100) - px(20)` is `calc(100% - 20px)`, `var(&SPACE) * 2` is `calc(var(--space) * 2)`, and
`px(10) + px(5)` is folded into `15px`. The functions `min`, `max`, `clamp`, and `env` create the CSS math functions of
the same names.

Some properties, such as `padding`, `gap`, and `width`, do not accept negative values. For these properties, a
negative length or percentage is written inside `calc()`. The browser then clamps the result to zero instead of
ignoring the declaration, so `PADDING.declare(px(4) - px(8))` becomes `padding:calc(-4px)`. Calculations are clamped
the same way. A `var()` is written as it is, so a negative custom property makes such a declaration invalid.

### Custom properties

`custom_property!` declares a custom property such as `--space` with the grammar of its values. `var` and `var_or`
reference it, and a property accepts the reference wherever it accepts the grammar. A `LengthPercentage` custom
property can therefore be used for `padding`, `margin`, `gap`, and `width` alike. Color custom properties also work
inside `light_dark` and `color_mix`.

The grammar type only keeps Rust code consistent. The browser does not know it, so a stylesheet can still assign a
value of another type to the same name.

### Colors

`rgb`, `rgba`, `hex`, `hsl`, `hwb`, `oklab`, and `oklch` create colors. `NamedColor` has every named color of CSS, and
`SystemColor` has the colors of the user's theme, which stay readable in forced-colors mode. `light_dark` picks a
color by the color scheme, `color_mix` mixes two colors, and `with_alpha` makes any color translucent. Colors are
written in the modern space-separated syntax, such as `rgb(255 0 0 / 0.5)`. A hue is an `Angle`, such as `deg(200)`, and
the channels and weights that CSS writes as percentages are numbers from 0 to 100.

### Valid values

The convenience functions, such as `px` and `rgba`, panic on invalid input such as `NaN`. For values that are only
known at runtime, use a fallible counterpart, such as `Length::try_px` or `Color::try_rgba`. A function without one,
such as `TransformFunction::scale`, takes an `f64` that `Number::try_new` can check first.

`typed-css` is stricter than CSS in a few places. CSS clamps an alpha or an opacity above 1, for example, but this
crate rejects it, so that a mistake shows up where it happens. Other ranges are checked in the same way, such as a
negative flex factor or a negative distance for `perspective()`. Lengths and percentages for non-negative properties
are the exception. Arithmetic can fold `px(4) - px(8)` into a negative literal, so these properties clamp a negative
value with `calc()` instead, as described above.

## Leptos integration

Enable the `leptos` feature to use declarations directly as Leptos style attributes:

```bash
cargo add typed-css --features leptos
```

A `Declaration` then implements Leptos' `IntoStyle`. It renders during server-side rendering, hydrates in place, and
updates its property in the browser through the element's style object:

```rust,ignore
use leptos::prelude::*;
use typed_css::{NamedColor, px, property::{COLOR, PADDING}};

let (highlighted, _set_highlighted) = signal(false);
let _view = view! {
    <div style=PADDING.declare(px(16))>"Padded"</div>
    <p style=move || COLOR.declare(if highlighted.get() { NamedColor::Crimson } else { NamedColor::Black })>
        "Reactive color"
    </p>
};
```

`typed-css` works with Leptos 0.8. To combine several declarations on one element, pass them through components, or
mix in CSS that this crate does not support, use [`leptos-styles`](https://crates.io/crates/leptos-styles).

## Supported CSS

`typed-css` supports a selection of modern properties, not all of CSS. Write other properties as plain CSS.

- **Layout:** `display`, `position`, `visibility`, `order`, `z-index`, flexbox, box alignment with the `place-*`
  shorthands, and grid track lists and placement.
- **Box model:** `padding`, `margin`, `inset`, and their physical and logical longhands, the preferred, minimum, and
  maximum sizes, `box-sizing`, `aspect-ratio`, `gap`, and `border-radius`.
- **Overflow and scrolling:** `overflow` and its longhands, `scroll-behavior`, `scrollbar-gutter`, and
  `overscroll-behavior` and its longhands.
- **Transforms:** `transform` with 2D and 3D functions, including `perspective()`, and `translate`, `rotate`, and
  `scale`.
- **Color:** `color`, `background-color`, `opacity`, `forced-color-adjust`, and `print-color-adjust`.
- **Text:** `font-size`, `font-weight`, `font-style`, `line-height`, `text-align`, and the `text-decoration-*`
  longhands with `text-underline-offset`.
- **Interaction:** `cursor`, `pointer-events`, `user-select`, `touch-action`, and `view-transition-name`.

## License

Dual licensed under either of:

- MIT License ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

at your option.
