// The README's Leptos example is marked `ignore`, because it needs the `leptos` feature. The test
// `readme_example_compiles` in `tests/direct_leptos.rs` compiles the same code.
#![doc = include_str!("../README.md")]

mod alignment;
mod angle;
mod auto;
mod box_model;
mod checked_value;
mod color;
mod color_adjust;
mod custom_property;
mod declaration;
mod display;
mod flexbox;
mod gap;
mod global_keyword;
mod grid;
mod identifier;
mod length;
mod length_percentage;
mod number;
mod opacity;
mod overflow;
mod percentage;
mod position;
pub mod property;
mod property_name;
mod serialization;
mod sizing;
mod stacking;
mod text;
mod text_decoration;
mod time;
mod touch_action;
mod transform;
mod typography;
mod user_interface;
mod view_transition;

pub use alignment::{
    AlignContent, AlignItems, AlignSelf, BaselinePosition, ContentDistribution, ContentPosition,
    JustifyContent, JustifyItems, JustifySelf, LegacyPosition, OverflowAlignment, OverflowPosition,
    PhysicalPosition, PlaceContent, PlaceItems, PlaceSelf, SelfPosition,
};
pub use angle::{Angle, deg, grad, rad, turn};
pub use auto::Auto;
pub use box_model::{
    BorderRadius, CornerRadius, Inset, InsetAxis, Margin, MarginAxis, Padding, PaddingAxis,
};
pub use checked_value::CheckedValue;
pub use color::{
    Color, ColorInterpolationMethod, ColorMix, ColorSpace, HueInterpolationMethod, NamedColor,
    PolarColorSpace, SystemColor, color_mix, color_mix_weighted, hex, hsl, hsla, hwb, light_dark,
    oklab, oklch, rgb, rgba,
};
pub use color_adjust::{ForcedColorAdjust, PrintColorAdjust};
pub use custom_property::{
    CustomProperty, DeclarationValue, InvalidCustomPropertyName, Var, var, var_or,
};
pub use declaration::Declaration;
pub use display::{Display, Visibility};
pub use flexbox::{Flex, FlexBasis, FlexDirection, FlexFlow, FlexWrap};
pub use gap::{Gap, GapValue};
pub use global_keyword::{GlobalKeyword, GlobalKeywordsOnly};
pub use grid::{
    GridAutoFlow, GridAutoTracks, GridLine, GridLineRange, GridRepeat, GridTemplate, GridTrack,
    InflexibleBreadth, InvalidGridLine, InvalidGridTracks, RepeatCount, TrackBreadth, TrackSize,
    fr,
};
pub use length::{
    Length, ch, cqb, cqh, cqi, cqmax, cqmin, cqw, dvh, dvw, em, ex, lh, lvh, lvw, pt, px, rem, rlh,
    svh, svw, vh, vmax, vmin, vw,
};
pub use length_percentage::{
    EnvironmentVariable, LengthPercentage, LengthPercentageAuto, NonNegativeLengthPercentage,
    clamp, env, max, min,
};
pub use number::{InvalidNumber, NonNegativeNumber, Number, PercentageChannel, UnitInterval};
pub use opacity::Opacity;
pub use overflow::{
    Overflow, OverflowKeyword, OverscrollBehavior, OverscrollKeyword, ScrollBehavior,
    ScrollbarGutter,
};
pub use percentage::{Percentage, pct};
pub use position::Position;
pub use property::definition::Property;
pub use serialization::ToCss;
pub use sizing::{AspectRatio, BoxSizing, MaxSize, Size};
pub use stacking::ZIndex;
pub use text::TextAlign;
pub use text_decoration::{
    TextDecorationLine, TextDecorationLines, TextDecorationStyle, TextDecorationThickness,
    TextUnderlineOffset,
};
pub use time::{Time, ms, s};
pub use touch_action::{
    TouchAction, TouchActionGestures, TouchActionHorizontalPan, TouchActionVerticalPan,
};
pub use transform::{Rotate, Scale, Transform, TransformFunction, Translate};
pub use typography::{FontSize, FontStyle, FontWeight, FontWeightNumber, LineHeight, ObliqueAngle};
pub use user_interface::{Cursor, PointerEvents, UserSelect};
pub use view_transition::{InvalidViewTransitionName, ViewTransitionName};

// Values, properties, and declarations are passed between threads, for example in the props and
// signals of UI frameworks. These assertions make a change that loses `Send` or `Sync` fail here.
const _: () = {
    const fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<LengthPercentage>();
    assert_send_sync::<Color>();
    assert_send_sync::<CustomProperty<Color>>();
    assert_send_sync::<DeclarationValue<Color>>();
    assert_send_sync::<Padding>();
    assert_send_sync::<ViewTransitionName>();
    assert_send_sync::<TouchAction>();
    assert_send_sync::<Transform>();
    assert_send_sync::<GridTemplate>();
    assert_send_sync::<Property<Size>>();
    assert_send_sync::<Declaration>();
};
