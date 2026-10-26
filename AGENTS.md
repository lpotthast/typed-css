# Repository guidance

`typed-css` is a single-crate library that writes CSS declarations from checked Rust values. `Cargo.toml` lists the
development-only files that the published package excludes.

## Commands

Day-to-day work goes through `just`, and the recipes live in the `Justfile`. After installing `just`, run `just once`
for the one-time setup (`enable-wasm` and `install-tools`).

| Task                                       | Command                |
|--------------------------------------------|------------------------|
| Format                                     | `just fmt`             |
| Update dependencies, sort, and format      | `just tidy`            |
| Type-check with all features               | `just check`           |
| Type-check without the `leptos` feature    | `just check-no-leptos` |
| Type-check for `wasm32-unknown-unknown`    | `just check-wasm`      |
| Run Clippy with warnings denied            | `just clippy`          |
| Run unit tests                             | `just test-lib`        |
| Run doc tests                              | `just test-doc`        |
| Run all native tests                       | `just test`            |
| Build the documentation, warnings denied   | `just doc`             |
| Run the full verification before a change  | `just verify`          |

`just verify` changes no files. It checks the formatting and the order of the dependencies in `Cargo.toml`. It
type-checks with and without the `leptos` feature and for WASM, and runs Clippy and the native tests in both feature
shapes. It also builds the documentation with warnings denied. CI additionally
runs a locked release build, checks the MSRV with Rust 1.89.0, and runs `cargo-deny` as a blocking job.
`cargo-semver-checks` runs too, but does not block. While the crate is below 1.0, a breaking release bumps the minor
version. CI sets `RUSTFLAGS=-D warnings` and `RUSTDOCFLAGS=-D warnings`, so every new compiler or Rustdoc warning fails
CI.

## Architecture

### Modules

A module holds one concept and everything about it: its types, invariants, conversions, serialization, and checked
registration. Name a module after its concept, as a noun. Do not create grab-bag modules such as `utils` or `helpers`.
Use the narrowest visibility that works, which is `pub(crate)` unless an item belongs to the published API.

`lib.rs` is the public facade. Every public value type has one import path at the crate root, and the property
constants live in `typed_css::property`. Concept modules are private, so adding an implementation detail never adds
to the public API by accident.

- `serialization.rs` defines the open `ToCss` trait. Any type may implement it. The module also holds the
  crate-private `same_css`, which compares the text of two values without allocating, and `write_pair`, which writes
  the two values of a shorthand such as `gap` and leaves out a second value with the same text.
- `checked_value.rs` defines the sealed `CheckedValue` trait. Only the value types of properties and the primitives
  that custom properties hold implement it, and only those types can be declared. The module also holds three
  crate-private macros. `impl_checked_value!` marks a type as checked, `impl_display!` implements `Display` through
  `ToCss`, and `keyword_enum!` defines an enum of keywords with both traits except the checked mark. When every
  variant of a `keyword_enum!` names a method with `as`, the macro also generates the keyword methods of the
  properties.
- `number`, `length`, `percentage`, `angle`, and `time` hold the primitive values and their unit helpers. The
  crate-private `dimension!` macro in `number.rs` defines `Length`, `Angle`, and `Time` with the same constructors,
  accessors, and operators, and `unit_functions!` defines their free functions, such as `px` and `deg`.
- `length_percentage.rs` holds `LengthPercentage` with its private expression tree, the arithmetic and math functions,
  `NonNegativeLengthPercentage`, and `LengthPercentageAuto`. Its crate-private `length_percentage_sources!` macro lets
  a grammar accept lengths, percentages, and their `var()` references directly.
- `custom_property.rs` holds `CustomProperty<T>`, its name validation, `DeclarationValue<T>`, `Var<T>` with `var()` and
  `var_or()`, and the crate-private `declaration_sources!` macro. Other modules must not implement behavior of these
  types.
- `declaration.rs` holds `Declaration`. Its fields are private, so the property and the value cannot be separated.
  Behind the `leptos` feature, the module also implements Leptos' `IntoStyle` for `Declaration`.
- `property_name.rs` holds the crate-private `PropertyName`, which is a static name or a shared `Arc<str>`.
  `Declaration` and `CustomProperty` store it, so that copying a name created at runtime only counts a reference.
- `property/definition.rs` holds `Property<T>`. Only this crate can create one. `property/mod.rs` is the catalog, one
  `properties!` table of constants with the name and the specification link of each property.
- `property/definition.rs` also holds the crate-private `property_keywords!` macro. A concept module calls it next
  to its grammar, so that each keyword becomes a method on the properties of that grammar, such as `DISPLAY.grid()`.
- `auto.rs` holds `Auto` and the crate-private `auto_sources!` macro. A grammar with an `auto` keyword calls the
  macro in its own module, so that properties accept `Auto` directly. Its `@conversions_only` form is for parts of a
  grammar that are not checked values.
- Each property grammar lives in a concept module, such as `box_model`, `sizing`, `gap`, `display`, `position`,
  `overflow`, `flexbox`, `alignment`, `grid`, `transform`, `color`, `typography`, `text`, `text_decoration`,
  `user_interface`, `touch_action`, `color_adjust`, `view_transition`, `opacity`, and `stacking`.

### Invariants

- Constructors reject invalid numbers, such as `NaN`, infinity, or a value outside the allowed range, before the value
  can reach CSS text. A convenience constructor such as `px` panics. Its fallible counterpart, such as
  `Length::try_px`, returns an error for runtime-derived input. Every constructor that can panic has a fallible
  counterpart, usually with the same name and a `try_` prefix. Its `# Panics` documentation names that counterpart.
- The constructors of the numeric types in `number`, `percentage`, `length`, `angle`, and `time` are `const fn`s that
  take an `f64`, so they work in constants. The convenience functions, such as `px`, and the constructors of property
  values, such as `Opacity::new`, take `impl Into<f64>`. Each numeric type returns its number with `value()`.
- `Number` writes every zero as `0`, so `-0.0` and `0.0` have the same CSS text and compare equal.
- `Length`, `Percentage`, and `LengthPercentage` model `<length>`, `<percentage>`, and `<length-percentage>`. A grammar
  that also accepts `auto`, such as `margin`, uses `LengthPercentageAuto`. A grammar that accepts only non-negative
  values, such as `padding`, uses `NonNegativeLengthPercentage`. It writes a negative length or percentage inside
  `calc()`, so that the browser clamps it instead of ignoring the declaration. Calculations are clamped the same way.
  It writes a `var()` or an `env()` as it is, because `calc(var(--x))` becomes invalid when `--x` is a unitless `0`.
  Never reject a calculation because one of its terms is negative.
- Shorthands such as `Padding`, `BorderRadius`, `Gap`, `Overflow`, and `Flex` store every component and are written
  in the shortest equivalent form.
- Mark a type with `impl_checked_value!` only if a property takes it or a custom property commonly holds it. Parts
  of a grammar, such as `TrackSize` or `RepeatCount`, are not checked values. CSS-wide keywords are not checked values
  either, because a custom property never holds one as its value. `declare_global` and the methods of the crate-private
  `global_keyword_methods!` macro, such as `inherit`, take them instead. A `property_keywords!` table must not use
  the name of a CSS-wide keyword, because every property already has that method.
- When a type converts into a grammar, also convert it into the grammar's `DeclarationValue` with
  `declaration_sources!`, so that properties accept it directly.
- When a grammar gains a keyword, add the keyword to the grammar's `property_keywords!` table, or name its method with
  `as` in the grammar's `keyword_enum!`. The methods cover every value with fixed CSS text and no arguments that the
  property accepts directly. Colors are left out, because they
  would bury the other methods. The test `every_keyword_method_writes_its_keyword` lists each table and checks that
  each method writes its keyword. A keyword that a shorthand writes in another form, such as `flex-flow: nowrap`,
  gets no method.
- A `From` between two checked value types must keep the meaning of the CSS text. A `var()` of the source type
  converts into the target type, so the browser reads the source text in the target property. `CornerRadius` does
  not convert into `BorderRadius`, for example, because `4px 2px` means one ellipse in a corner longhand but two
  circular corners in `border-radius`. A grammar that only accepts non-negative values may convert from
  `NonNegativeLengthPercentage`, and other grammars must not. The test
  `every_property_accepts_the_common_parts_of_its_grammar` covers each grammar.
- `CustomProperty<T>` checks its name and fixes one grammar `T`. `var()` and `var_or()` take a `CustomProperty`, and a
  fallback must have the same grammar. Do not add constructors that take a raw name or an untyped fallback.
- No type accepts arbitrary CSS text, and no type converts into a property name that could be paired with any value.
  Typed values do not implement Leptos' `IntoStyleValue`. Implementing `ToCss` must never make a type declarable.
- Every public enum with variants is `#[non_exhaustive]`, so a `match` in examples or downstream code needs a wildcard
  arm. The empty `GlobalKeywordsOnly` has no variants to add.
- Shorthands with two values, such as `Gap`, `Overflow`, `PaddingAxis`, `FlexFlow`, and `GridLineRange`, have `new`
  for two values and a `with_` method for each value. Those whose values share one type also have `both` for one
  repeated value. A list value, such as `GridTemplate` or `GridAutoTracks`, is created with `new` and `try_new`.
- Every public item has a doc comment. `missing_docs = "warn"` becomes an error in the warning-denied Rustdoc job.
- Clippy's `pedantic` group is denied, and `Cargo.toml` allows no lint. Satisfy a lint where it fires. If a lint is
  wrong in one place, use `#[expect(lint, reason = "...")]` on the smallest item and say why.

### Compatibility

- The MSRV is Rust 1.89.0, and CI checks it. Do not use newer language features or standard-library APIs unless the
  MSRV is bumped in `Cargo.toml`, the README, and CI together.
- Leptos is an optional dependency behind the `leptos` feature, which is off by default. Keep the default build free
  of dependencies and of framework-specific code. Leptos code belongs behind `#[cfg(feature = "leptos")]` in
  `declaration.rs`. The `just` recipes and CI build with `--all-features`, and the `*-no-leptos` recipes cover the
  default build.
- `Cargo.toml` requires `leptos = "0.8.22"` with default features disabled. Change the Leptos version together with
  `leptos-styles`, so that a downstream lockfile does not end up with two Leptos versions.

## CSS support policy

- Target modern CSS. Coverage of properties is partial on purpose. Never claim that the crate covers every property
  of CSS.
- The `Property<T>` constants are the only way to declare a standard property. Do not add a list of property names or
  a type that holds untyped values.
- Keep the checked API closed. It must not accept arbitrary property names, value text, function bodies, `var()`
  fallbacks, or identifiers that were not validated.
- Custom properties and `var()` are first-class features. Their names, values, references, and fallbacks stay typed
  and validated.
- Do not add legacy aliases or deprecated properties only for completeness.
- Do not add speech properties, SVG paint properties, or other niche families without an explicit need.
- Prefer completing the grammar of a supported property, and its logical and physical relatives, over adding many
  new property names.
- Link the specification in the documentation of each property and grammar. The Rust types are the source of truth.
  Do not keep a separate coverage list unless it drives generation or tests.

### Standards baseline

The catalog admits a property only if its name is the current, non-deprecated name of a general-purpose feature. The
property must also appear in the [CSS Snapshot 2026 property index](https://www.w3.org/TR/css-2026/#properties) or
come from one of these modern modules:

- [CSS Basic User Interface Level 4](https://www.w3.org/TR/css-ui-4/) for `cursor`, `appearance`, `pointer-events`,
  and `user-select`.
- [CSS Color Level 4](https://www.w3.org/TR/css-color-4/) and [Level 5](https://www.w3.org/TR/css-color-5/) for the
  modern color syntax, `oklab()`, `oklch()`, `light-dark()`, and `color-mix()`.
- [CSS Grid Level 2](https://www.w3.org/TR/css-grid-2/), [CSS Flexbox Level 1](https://www.w3.org/TR/css-flexbox-1/),
  and [CSS Box Alignment Level 3](https://www.w3.org/TR/css-align-3/) for layout.
- [CSS Transforms Level 2](https://www.w3.org/TR/css-transforms-2/) for 3D transform functions and the `translate`,
  `rotate`, and `scale` properties.
- [CSS Sizing Level 4](https://www.w3.org/TR/css-sizing-4/) for `aspect-ratio`.
- [Filter Effects Level 2](https://drafts.csswg.org/filter-effects-2/) for `backdrop-filter`.
- [CSS Overflow Level 3](https://www.w3.org/TR/css-overflow-3/) for `overflow-x`, `overflow-y`, `scroll-behavior`, and
  `scrollbar-gutter`.
- [CSS Overscroll Behavior Level 1](https://www.w3.org/TR/css-overscroll-1/) for the `overscroll-behavior` family.
- [CSS Text Decoration Level 4](https://www.w3.org/TR/css-text-decor-4/) for `text-decoration-skip-ink`,
  `text-decoration-thickness`, and `text-underline-offset`.
- [Pointer Events Level 4](https://www.w3.org/TR/pointerevents4/) for `touch-action`.
- [CSS Values Level 4](https://www.w3.org/TR/css-values-4/) for math functions and modern units.
- [CSS Environment Variables Level 1](https://www.w3.org/TR/css-env-1/) for `env()`.
- [CSS Custom Properties Level 1](https://www.w3.org/TR/css-variables-1/) for custom properties and `var()`.
- [CSS View Transitions Level 1](https://www.w3.org/TR/css-view-transitions-1/) for `view-transition-name`.

Whether browsers support a property is a separate question from whether its grammar is valid.

## Writing

These rules apply to everything written in the repository: the README, the CHANGELOG, this file, doc comments, and
code comments.

- Write complete sentences with a subject and a verb, in the active voice. Say who does what.
- Use one idea per sentence, and split long sentences.
- Use plain, common words. A reader should understand each sentence on the first read.
- Name things instead of writing "it" and "its", and avoid chains of possessives.
- Use connecting words such as "because", "so", and "so that". Use a colon only to introduce a list or an example.
- Do not use semicolons in prose. Split the sentence or join the parts with a connecting word. Semicolons in code are
  fine.
- Documentation describes what the code is and why. It does not describe history, such as what an item replaced. The
  CHANGELOG is the place for history.
- Only document what a reader cannot infer from a name. Document units, ranges, the meaning of absent values, and
  invariants.
- A doc comment starts with a sentence that says what the item is or does. Keep the examples short and runnable.
