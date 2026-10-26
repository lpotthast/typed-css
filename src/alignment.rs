use std::fmt;

use crate::auto::auto_sources;
use crate::checked_value::{impl_checked_value, impl_display, keyword_enum};
use crate::custom_property::declaration_sources;
use crate::property::definition::property_keywords;
use crate::serialization::{ToCss, write_pair};

keyword_enum! {
    /// What the browser does when aligned content is larger than its container.
    ///
    /// [CSS Box Alignment Level 3 overflow alignment](https://www.w3.org/TR/css-align-3/#overflow-values)
    pub enum OverflowPosition {
        /// If the content would overflow, the browser aligns it to the start instead, so that
        /// no part of it becomes unreachable.
        Safe => "safe",
        /// The browser keeps the requested alignment, even if part of the content becomes
        /// unreachable.
        Unsafe => "unsafe",
    }
}

keyword_enum! {
    /// How the browser distributes extra space between items or grid tracks.
    ///
    /// [CSS Box Alignment Level 3 distributed alignment](https://www.w3.org/TR/css-align-3/#distribution-values)
    pub enum ContentDistribution {
        SpaceBetween => "space-between",
        SpaceAround => "space-around",
        SpaceEvenly => "space-evenly",
        Stretch => "stretch",
    }
}

keyword_enum! {
    /// Where the browser places the content of a container as a whole.
    ///
    /// [CSS Box Alignment Level 3 positional alignment](https://www.w3.org/TR/css-align-3/#positional-values)
    pub enum ContentPosition {
        Center => "center",
        Start => "start",
        End => "end",
        FlexStart => "flex-start",
        FlexEnd => "flex-end",
    }
}

keyword_enum! {
    /// Where the browser places an item inside its alignment area.
    ///
    /// [CSS Box Alignment Level 3 positional alignment](https://www.w3.org/TR/css-align-3/#positional-values)
    pub enum SelfPosition {
        Center => "center",
        Start => "start",
        End => "end",
        SelfStart => "self-start",
        SelfEnd => "self-end",
        FlexStart => "flex-start",
        FlexEnd => "flex-end",
    }
}

keyword_enum! {
    /// The physical positions `left` and `right`, which only the justification properties accept.
    ///
    /// [CSS Box Alignment Level 3 positional alignment](https://www.w3.org/TR/css-align-3/#positional-values)
    pub enum PhysicalPosition {
        Left => "left",
        Right => "right",
    }
}

keyword_enum! {
    /// The position that `legacy` in `justify-items` passes on to descendants.
    ///
    /// [CSS Box Alignment Level 3 `justify-items` grammar](https://www.w3.org/TR/css-align-3/#justify-items-property)
    pub enum LegacyPosition {
        Left => "left",
        Right => "right",
        Center => "center",
    }
}

keyword_enum! {
    /// Which baseline of a line of text the browser aligns items to.
    ///
    /// [CSS Box Alignment Level 3 baseline alignment](https://www.w3.org/TR/css-align-3/#baseline-values)
    pub enum BaselinePosition {
        /// It is the same as `first baseline`.
        First => "baseline",
        Last => "last baseline",
    }
}

/// A position with an explicit overflow position, such as `safe center`.
///
/// The methods `safe` and `with_overflow` of [`ContentPosition`], [`SelfPosition`], and
/// [`PhysicalPosition`] create it. It converts into every alignment grammar that accepts its
/// position.
///
/// ```rust
/// use typed_css::{ContentPosition, OverflowPosition, SelfPosition, property::{ALIGN_SELF, JUSTIFY_CONTENT}};
///
/// assert_eq!(JUSTIFY_CONTENT.declare(ContentPosition::Center.safe()).value(), "safe center");
/// let end = SelfPosition::End.with_overflow(OverflowPosition::Unsafe);
/// assert_eq!(ALIGN_SELF.declare(end).value(), "unsafe end");
/// ```
///
/// [CSS Box Alignment Level 3 overflow alignment](https://www.w3.org/TR/css-align-3/#overflow-values)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct OverflowAlignment<P> {
    overflow: OverflowPosition,
    position: P,
}

impl<P: ToCss> ToCss for OverflowAlignment<P> {
    fn to_css<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        write_positioned(w, Some(self.overflow), &self.position)
    }
}

impl<P: ToCss> fmt::Display for OverflowAlignment<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_css(f)
    }
}

/// Add the methods that create an [`OverflowAlignment`] to each position type.
macro_rules! overflow_methods {
    ($($position:ident),+) => {$(
        impl $position {
            /// Align safely. If the content would overflow, the browser aligns it to the start
            /// instead, so that no part of it becomes unreachable.
            #[must_use]
            pub const fn safe(self) -> OverflowAlignment<Self> {
                self.with_overflow(OverflowPosition::Safe)
            }

            /// Align with the given overflow position.
            #[must_use]
            pub const fn with_overflow(self, overflow: OverflowPosition) -> OverflowAlignment<Self> {
                OverflowAlignment {
                    overflow,
                    position: self,
                }
            }
        }
    )+};
}

overflow_methods!(ContentPosition, SelfPosition, PhysicalPosition);

/// Convert an [`OverflowAlignment`] into the variant of each grammar that holds its position.
macro_rules! overflow_alignment_sources {
    ($($name:ident: $($position:ident => $variant:ident),+;)+) => {$($(
        impl From<OverflowAlignment<$position>> for $name {
            fn from(value: OverflowAlignment<$position>) -> Self {
                Self::$variant(Some(value.overflow), value.position)
            }
        }

        declaration_sources!($name: OverflowAlignment<$position>);
    )+)+};
}

fn write_positioned<W: fmt::Write>(
    w: &mut W,
    overflow: Option<OverflowPosition>,
    position: &impl ToCss,
) -> fmt::Result {
    if let Some(overflow) = overflow {
        overflow.to_css(w)?;
        w.write_char(' ')?;
    }
    position.to_css(w)
}

/// Generate an alignment grammar enum from its variants and their serialization.
macro_rules! alignment_grammar {
    (
        $(#[$meta:meta])*
        $name:ident |$w:ident| {
            $(
                $(#[$variant_meta:meta])*
                $variant:ident $(($($field:ty),+)($($binding:ident),+))? => $write:expr
            ),+ $(,)?
        }
        from { $($source:ident => $source_variant:ident $(($none:expr))?),* $(,)? }
    ) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        #[non_exhaustive]
        pub enum $name {
            $($(#[$variant_meta])* $variant $(($($field),+))?),+
        }

        impl ToCss for $name {
            fn to_css<W: fmt::Write>(&self, $w: &mut W) -> fmt::Result {
                match self {
                    $(Self::$variant $(($($binding),+))? => $write),+
                }
            }
        }

        $(
            impl From<$source> for $name {
                fn from(value: $source) -> Self {
                    alignment_grammar!(@from $source_variant value $(, $none)?)
                }
            }

            declaration_sources!($name: $source);
        )*
    };
    (@from $variant:ident $value:ident, $none:expr) => {
        Self::$variant($none, $value)
    };
    (@from $variant:ident $value:ident) => {
        Self::$variant($value)
    };
}

alignment_grammar! {
    /// An `align-content` value.
    ///
    /// [CSS Box Alignment Level 3 `align-content` grammar](https://www.w3.org/TR/css-align-3/#align-justify-content)
    AlignContent |w| {
        /// `normal`.
        Normal => w.write_str("normal"),
        /// A baseline position.
        Baseline(BaselinePosition)(baseline) => baseline.to_css(w),
        /// A distribution keyword.
        Distribution(ContentDistribution)(distribution) => distribution.to_css(w),
        /// A content position. An overflow position of `None` lets the browser choose, which keeps
        /// content reachable in most cases.
        Position(Option<OverflowPosition>, ContentPosition)(overflow, position) => write_positioned(w, *overflow, position),
    }
    from {
        BaselinePosition => Baseline,
        ContentDistribution => Distribution,
        ContentPosition => Position(None),
    }
}

alignment_grammar! {
    /// A `justify-content` value.
    ///
    /// The position, distribution, and physical position types convert into this type, so that a
    /// property accepts them directly.
    ///
    /// ```rust
    /// use typed_css::{
    ///     ContentDistribution, ContentPosition, property::JUSTIFY_CONTENT,
    /// };
    ///
    /// assert_eq!(JUSTIFY_CONTENT.declare(ContentPosition::Center).value(), "center");
    /// assert_eq!(JUSTIFY_CONTENT.declare(ContentDistribution::SpaceBetween).value(), "space-between");
    /// assert_eq!(JUSTIFY_CONTENT.declare(ContentPosition::Center.safe()).value(), "safe center");
    /// ```
    ///
    /// [CSS Box Alignment Level 3 `justify-content` grammar](https://www.w3.org/TR/css-align-3/#align-justify-content)
    JustifyContent |w| {
        /// `normal`.
        Normal => w.write_str("normal"),
        /// A distribution keyword.
        Distribution(ContentDistribution)(distribution) => distribution.to_css(w),
        /// A content position. An overflow position of `None` lets the browser choose, which keeps
        /// content reachable in most cases.
        Position(Option<OverflowPosition>, ContentPosition)(overflow, position) => write_positioned(w, *overflow, position),
        /// `left` or `right`. An overflow position of `None` lets the browser choose.
        Physical(Option<OverflowPosition>, PhysicalPosition)(overflow, position) => write_positioned(w, *overflow, position),
    }
    from {
        ContentDistribution => Distribution,
        ContentPosition => Position(None),
        PhysicalPosition => Physical(None),
    }
}

alignment_grammar! {
    /// An `align-items` value.
    ///
    /// [CSS Box Alignment Level 3 `align-items` grammar](https://www.w3.org/TR/css-align-3/#align-items-property)
    AlignItems |w| {
        /// `normal`.
        Normal => w.write_str("normal"),
        /// `stretch`.
        Stretch => w.write_str("stretch"),
        /// A baseline position.
        Baseline(BaselinePosition)(baseline) => baseline.to_css(w),
        /// A self position. An overflow position of `None` lets the browser choose, which keeps
        /// content reachable in most cases.
        Position(Option<OverflowPosition>, SelfPosition)(overflow, position) => write_positioned(w, *overflow, position),
    }
    from {
        BaselinePosition => Baseline,
        SelfPosition => Position(None),
    }
}

alignment_grammar! {
    /// An `align-self` value.
    ///
    /// [CSS Box Alignment Level 3 `align-self` grammar](https://www.w3.org/TR/css-align-3/#align-self-property)
    AlignSelf |w| {
        /// `auto`.
        Auto => w.write_str("auto"),
        /// `normal`.
        Normal => w.write_str("normal"),
        /// `stretch`.
        Stretch => w.write_str("stretch"),
        /// A baseline position.
        Baseline(BaselinePosition)(baseline) => baseline.to_css(w),
        /// A self position. An overflow position of `None` lets the browser choose, which keeps
        /// content reachable in most cases.
        Position(Option<OverflowPosition>, SelfPosition)(overflow, position) => write_positioned(w, *overflow, position),
    }
    from {
        BaselinePosition => Baseline,
        SelfPosition => Position(None),
    }
}

alignment_grammar! {
    /// A `justify-items` value.
    ///
    /// [CSS Box Alignment Level 3 `justify-items` grammar](https://www.w3.org/TR/css-align-3/#justify-items-property)
    JustifyItems |w| {
        /// `normal`.
        Normal => w.write_str("normal"),
        /// `stretch`.
        Stretch => w.write_str("stretch"),
        /// A baseline position.
        Baseline(BaselinePosition)(baseline) => baseline.to_css(w),
        /// A self position. An overflow position of `None` lets the browser choose, which keeps
        /// content reachable in most cases.
        Position(Option<OverflowPosition>, SelfPosition)(overflow, position) => write_positioned(w, *overflow, position),
        /// `left` or `right`. An overflow position of `None` lets the browser choose.
        Physical(Option<OverflowPosition>, PhysicalPosition)(overflow, position) => write_positioned(w, *overflow, position),
        /// `legacy`, which descendants with `justify-self: auto` inherit. With `Some` position,
        /// the descendants are aligned that way. With `None`, they use their own default.
        Legacy(Option<LegacyPosition>)(position) => match position {
            Some(position) => {
                w.write_str("legacy ")?;
                position.to_css(w)
            }
            None => w.write_str("legacy"),
        },
    }
    from {
        BaselinePosition => Baseline,
        SelfPosition => Position(None),
        PhysicalPosition => Physical(None),
    }
}

alignment_grammar! {
    /// A `justify-self` value.
    ///
    /// [CSS Box Alignment Level 3 `justify-self` grammar](https://www.w3.org/TR/css-align-3/#justify-self-property)
    JustifySelf |w| {
        /// `auto`.
        Auto => w.write_str("auto"),
        /// `normal`.
        Normal => w.write_str("normal"),
        /// `stretch`.
        Stretch => w.write_str("stretch"),
        /// A baseline position.
        Baseline(BaselinePosition)(baseline) => baseline.to_css(w),
        /// A self position. An overflow position of `None` lets the browser choose, which keeps
        /// content reachable in most cases.
        Position(Option<OverflowPosition>, SelfPosition)(overflow, position) => write_positioned(w, *overflow, position),
        /// `left` or `right`. An overflow position of `None` lets the browser choose.
        Physical(Option<OverflowPosition>, PhysicalPosition)(overflow, position) => write_positioned(w, *overflow, position),
    }
    from {
        BaselinePosition => Baseline,
        SelfPosition => Position(None),
        PhysicalPosition => Physical(None),
    }
}

macro_rules! place_shorthand {
    (
        $(#[$meta:meta])*
        $name:ident: $align:ident, $justify:ident, from { $($source:ty),* $(,)? }
    ) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub struct $name {
            align: $align,
            justify: $justify,
        }

        impl $name {
            /// Use one value for the block axis and the inline axis.
            #[must_use]
            pub fn both<V: Into<$align> + Into<$justify> + Copy>(value: V) -> Self {
                Self::new(value, value)
            }

            /// Use separate values for the block axis and the inline axis.
            #[must_use]
            pub fn new(align: impl Into<$align>, justify: impl Into<$justify>) -> Self {
                Self {
                    align: align.into(),
                    justify: justify.into(),
                }
            }

            /// Replace the value for the block axis.
            #[must_use]
            pub fn with_align(mut self, align: impl Into<$align>) -> Self {
                self.align = align.into();
                self
            }

            /// Replace the value for the inline axis.
            #[must_use]
            pub fn with_justify(mut self, justify: impl Into<$justify>) -> Self {
                self.justify = justify.into();
                self
            }
        }

        $(
            impl From<$source> for $name {
                fn from(value: $source) -> Self {
                    Self::both(value)
                }
            }

            declaration_sources!($name: $source);
        )*

        impl ToCss for $name {
            fn to_css<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
                // A single value sets both axes. It is only written alone when both values are
                // the same, which also excludes the special case of a single baseline value.
                write_pair(w, &self.align, &self.justify)
            }
        }
    };
}

place_shorthand! {
    /// A `place-content` shorthand value, with an `align-content` and a `justify-content` value.
    ///
    /// ```rust
    /// use typed_css::{ContentDistribution, ContentPosition, PlaceContent};
    ///
    /// assert_eq!(PlaceContent::both(ContentPosition::Center).to_string(), "center");
    /// assert_eq!(
    ///     PlaceContent::new(ContentPosition::End, ContentDistribution::SpaceBetween).to_string(),
    ///     "end space-between",
    /// );
    /// ```
    ///
    /// [CSS Box Alignment Level 3 `place-content`](https://www.w3.org/TR/css-align-3/#place-content)
    PlaceContent: AlignContent, JustifyContent, from {
        ContentDistribution,
        ContentPosition,
        OverflowAlignment<ContentPosition>,
    }
}

place_shorthand! {
    /// A `place-items` shorthand value, with an `align-items` and a `justify-items` value.
    ///
    /// [CSS Box Alignment Level 3 `place-items`](https://www.w3.org/TR/css-align-3/#place-items-property)
    PlaceItems: AlignItems, JustifyItems, from {
        BaselinePosition,
        SelfPosition,
        OverflowAlignment<SelfPosition>,
    }
}

place_shorthand! {
    /// A `place-self` shorthand value, with an `align-self` and a `justify-self` value.
    ///
    /// [CSS Box Alignment Level 3 `place-self`](https://www.w3.org/TR/css-align-3/#place-self-property)
    PlaceSelf: AlignSelf, JustifySelf, from {
        BaselinePosition,
        SelfPosition,
        OverflowAlignment<SelfPosition>,
    }
}

overflow_alignment_sources! {
    AlignContent: ContentPosition => Position;
    JustifyContent: ContentPosition => Position, PhysicalPosition => Physical;
    AlignItems: SelfPosition => Position;
    AlignSelf: SelfPosition => Position;
    JustifyItems: SelfPosition => Position, PhysicalPosition => Physical;
    JustifySelf: SelfPosition => Position, PhysicalPosition => Physical;
}

auto_sources!(
    AlignSelf => Self::Auto,
    JustifySelf => Self::Auto,
    PlaceSelf => Self::new(AlignSelf::Auto, JustifySelf::Auto),
);

impl_display!(
    AlignContent,
    JustifyContent,
    AlignItems,
    AlignSelf,
    JustifyItems,
    JustifySelf,
    PlaceContent,
    PlaceItems,
    PlaceSelf
);
impl_checked_value!(
    AlignContent,
    JustifyContent,
    AlignItems,
    AlignSelf,
    JustifyItems,
    JustifySelf,
    PlaceContent,
    PlaceItems,
    PlaceSelf
);

property_keywords! {
    AlignContent {
        normal: "normal" => AlignContent::Normal,
        baseline: "baseline" => BaselinePosition::First,
        last_baseline: "last baseline" => BaselinePosition::Last,
        space_between: "space-between" => ContentDistribution::SpaceBetween,
        space_around: "space-around" => ContentDistribution::SpaceAround,
        space_evenly: "space-evenly" => ContentDistribution::SpaceEvenly,
        stretch: "stretch" => ContentDistribution::Stretch,
        center: "center" => ContentPosition::Center,
        start: "start" => ContentPosition::Start,
        end: "end" => ContentPosition::End,
        flex_start: "flex-start" => ContentPosition::FlexStart,
        flex_end: "flex-end" => ContentPosition::FlexEnd,
    }
}

property_keywords! {
    JustifyContent {
        normal: "normal" => JustifyContent::Normal,
        space_between: "space-between" => ContentDistribution::SpaceBetween,
        space_around: "space-around" => ContentDistribution::SpaceAround,
        space_evenly: "space-evenly" => ContentDistribution::SpaceEvenly,
        stretch: "stretch" => ContentDistribution::Stretch,
        center: "center" => ContentPosition::Center,
        start: "start" => ContentPosition::Start,
        end: "end" => ContentPosition::End,
        flex_start: "flex-start" => ContentPosition::FlexStart,
        flex_end: "flex-end" => ContentPosition::FlexEnd,
        left: "left" => PhysicalPosition::Left,
        right: "right" => PhysicalPosition::Right,
    }
}

property_keywords! {
    AlignItems {
        normal: "normal" => AlignItems::Normal,
        stretch: "stretch" => AlignItems::Stretch,
        baseline: "baseline" => BaselinePosition::First,
        last_baseline: "last baseline" => BaselinePosition::Last,
        center: "center" => SelfPosition::Center,
        start: "start" => SelfPosition::Start,
        end: "end" => SelfPosition::End,
        self_start: "self-start" => SelfPosition::SelfStart,
        self_end: "self-end" => SelfPosition::SelfEnd,
        flex_start: "flex-start" => SelfPosition::FlexStart,
        flex_end: "flex-end" => SelfPosition::FlexEnd,
    }
}

property_keywords! {
    AlignSelf {
        auto: "auto" => AlignSelf::Auto,
        normal: "normal" => AlignSelf::Normal,
        stretch: "stretch" => AlignSelf::Stretch,
        baseline: "baseline" => BaselinePosition::First,
        last_baseline: "last baseline" => BaselinePosition::Last,
        center: "center" => SelfPosition::Center,
        start: "start" => SelfPosition::Start,
        end: "end" => SelfPosition::End,
        self_start: "self-start" => SelfPosition::SelfStart,
        self_end: "self-end" => SelfPosition::SelfEnd,
        flex_start: "flex-start" => SelfPosition::FlexStart,
        flex_end: "flex-end" => SelfPosition::FlexEnd,
    }
}

property_keywords! {
    JustifyItems {
        normal: "normal" => JustifyItems::Normal,
        stretch: "stretch" => JustifyItems::Stretch,
        baseline: "baseline" => BaselinePosition::First,
        last_baseline: "last baseline" => BaselinePosition::Last,
        center: "center" => SelfPosition::Center,
        start: "start" => SelfPosition::Start,
        end: "end" => SelfPosition::End,
        self_start: "self-start" => SelfPosition::SelfStart,
        self_end: "self-end" => SelfPosition::SelfEnd,
        flex_start: "flex-start" => SelfPosition::FlexStart,
        flex_end: "flex-end" => SelfPosition::FlexEnd,
        left: "left" => PhysicalPosition::Left,
        right: "right" => PhysicalPosition::Right,
        legacy: "legacy" => JustifyItems::Legacy(None),
        legacy_left: "legacy left" => JustifyItems::Legacy(Some(LegacyPosition::Left)),
        legacy_right: "legacy right" => JustifyItems::Legacy(Some(LegacyPosition::Right)),
        legacy_center: "legacy center" => JustifyItems::Legacy(Some(LegacyPosition::Center)),
    }
}

property_keywords! {
    JustifySelf {
        auto: "auto" => JustifySelf::Auto,
        normal: "normal" => JustifySelf::Normal,
        stretch: "stretch" => JustifySelf::Stretch,
        baseline: "baseline" => BaselinePosition::First,
        last_baseline: "last baseline" => BaselinePosition::Last,
        center: "center" => SelfPosition::Center,
        start: "start" => SelfPosition::Start,
        end: "end" => SelfPosition::End,
        self_start: "self-start" => SelfPosition::SelfStart,
        self_end: "self-end" => SelfPosition::SelfEnd,
        flex_start: "flex-start" => SelfPosition::FlexStart,
        flex_end: "flex-end" => SelfPosition::FlexEnd,
        left: "left" => PhysicalPosition::Left,
        right: "right" => PhysicalPosition::Right,
    }
}

// A single baseline value in `place-content` sets `justify-content` to `start`, so the shorthand
// has no baseline methods.
property_keywords! {
    PlaceContent {
        normal: "normal" => PlaceContent::new(AlignContent::Normal, JustifyContent::Normal),
        space_between: "space-between" => ContentDistribution::SpaceBetween,
        space_around: "space-around" => ContentDistribution::SpaceAround,
        space_evenly: "space-evenly" => ContentDistribution::SpaceEvenly,
        stretch: "stretch" => ContentDistribution::Stretch,
        center: "center" => ContentPosition::Center,
        start: "start" => ContentPosition::Start,
        end: "end" => ContentPosition::End,
        flex_start: "flex-start" => ContentPosition::FlexStart,
        flex_end: "flex-end" => ContentPosition::FlexEnd,
    }
}

property_keywords! {
    PlaceItems {
        normal: "normal" => PlaceItems::new(AlignItems::Normal, JustifyItems::Normal),
        stretch: "stretch" => PlaceItems::new(AlignItems::Stretch, JustifyItems::Stretch),
        baseline: "baseline" => BaselinePosition::First,
        last_baseline: "last baseline" => BaselinePosition::Last,
        center: "center" => SelfPosition::Center,
        start: "start" => SelfPosition::Start,
        end: "end" => SelfPosition::End,
        self_start: "self-start" => SelfPosition::SelfStart,
        self_end: "self-end" => SelfPosition::SelfEnd,
        flex_start: "flex-start" => SelfPosition::FlexStart,
        flex_end: "flex-end" => SelfPosition::FlexEnd,
    }
}

property_keywords! {
    PlaceSelf {
        auto: "auto" => crate::Auto,
        normal: "normal" => PlaceSelf::new(AlignSelf::Normal, JustifySelf::Normal),
        stretch: "stretch" => PlaceSelf::new(AlignSelf::Stretch, JustifySelf::Stretch),
        baseline: "baseline" => BaselinePosition::First,
        last_baseline: "last baseline" => BaselinePosition::Last,
        center: "center" => SelfPosition::Center,
        start: "start" => SelfPosition::Start,
        end: "end" => SelfPosition::End,
        self_start: "self-start" => SelfPosition::SelfStart,
        self_end: "self-end" => SelfPosition::SelfEnd,
        flex_start: "flex-start" => SelfPosition::FlexStart,
        flex_end: "flex-end" => SelfPosition::FlexEnd,
    }
}
