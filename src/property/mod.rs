//! The supported CSS properties.
//!
//! Each constant is a [`Property`] that pairs a property name with the grammar of its values.
//! `declare` creates a [`Declaration`](crate::Declaration) from a value of that grammar. Each
//! keyword that a property accepts on its own also has a method, such as `DISPLAY.grid()`. This
//! crate supports a selection of modern properties, not every property of CSS. Write unsupported
//! properties as plain CSS.
//!
//! ```rust
//! use typed_css::{pct, px, property::{DISPLAY, PADDING, WIDTH}};
//!
//! assert_eq!(WIDTH.declare(px(120)).value(), "120px");
//! assert_eq!(WIDTH.declare(pct(100) - px(20)).value(), "calc(100% - 20px)");
//! assert_eq!(PADDING.declare(px(16)).value(), "16px");
//! assert_eq!(DISPLAY.grid().value(), "grid");
//! ```

pub(crate) mod definition;

use crate::{
    AlignContent, AlignItems, AlignSelf, AspectRatio, BorderRadius, BoxSizing, Color, CornerRadius,
    Cursor, Display, Flex, FlexBasis, FlexDirection, FlexFlow, FlexWrap, FontSize, FontStyle,
    FontWeight, ForcedColorAdjust, Gap, GapValue, GlobalKeywordsOnly, GridAutoFlow, GridAutoTracks,
    GridLine, GridLineRange, GridTemplate, Inset, InsetAxis, JustifyContent, JustifyItems,
    JustifySelf, LengthPercentageAuto, LineHeight, Margin, MarginAxis, MaxSize,
    NonNegativeLengthPercentage, NonNegativeNumber, Opacity, Overflow, OverflowKeyword,
    OverscrollBehavior, OverscrollKeyword, Padding, PaddingAxis, PlaceContent, PlaceItems,
    PlaceSelf, PointerEvents, Position, PrintColorAdjust, Property, Rotate, Scale, ScrollBehavior,
    ScrollbarGutter, Size, TextAlign, TextDecorationLine, TextDecorationStyle,
    TextDecorationThickness, TextUnderlineOffset, TouchAction, Transform, Translate, UserSelect,
    ViewTransitionName, Visibility, ZIndex,
};

macro_rules! properties {
    ($($constant:ident: $grammar:ty = $name:literal, $spec:literal;)+) => {$(
        #[doc = concat!("The `", $name, "` property, whose values are [`", stringify!($grammar), "`].")]
        #[doc = ""]
        #[doc = concat!("[Specification](", $spec, ")")]
        pub const $constant: Property<$grammar> = Property::new($name);
    )+};
}

properties! {
    ALL: GlobalKeywordsOnly = "all", "https://www.w3.org/TR/css-cascade-6/#all-shorthand";

    DISPLAY: Display = "display", "https://www.w3.org/TR/css-display-3/#the-display-properties";
    VISIBILITY: Visibility = "visibility", "https://www.w3.org/TR/css-display-4/#visibility";
    ORDER: i32 = "order", "https://www.w3.org/TR/css-display-3/#order-property";
    POSITION: Position = "position", "https://www.w3.org/TR/css-position-3/#position-property";
    Z_INDEX: ZIndex = "z-index", "https://www.w3.org/TR/CSS22/visuren.html#z-index";

    PADDING: Padding = "padding", "https://www.w3.org/TR/css-box-4/#padding-shorthand";
    PADDING_TOP: NonNegativeLengthPercentage = "padding-top", "https://www.w3.org/TR/css-box-4/#padding-physical";
    PADDING_RIGHT: NonNegativeLengthPercentage = "padding-right", "https://www.w3.org/TR/css-box-4/#padding-physical";
    PADDING_BOTTOM: NonNegativeLengthPercentage = "padding-bottom", "https://www.w3.org/TR/css-box-4/#padding-physical";
    PADDING_LEFT: NonNegativeLengthPercentage = "padding-left", "https://www.w3.org/TR/css-box-4/#padding-physical";
    PADDING_BLOCK: PaddingAxis = "padding-block", "https://www.w3.org/TR/css-logical-1/#padding-properties";
    PADDING_BLOCK_START: NonNegativeLengthPercentage = "padding-block-start", "https://www.w3.org/TR/css-logical-1/#padding-properties";
    PADDING_BLOCK_END: NonNegativeLengthPercentage = "padding-block-end", "https://www.w3.org/TR/css-logical-1/#padding-properties";
    PADDING_INLINE: PaddingAxis = "padding-inline", "https://www.w3.org/TR/css-logical-1/#padding-properties";
    PADDING_INLINE_START: NonNegativeLengthPercentage = "padding-inline-start", "https://www.w3.org/TR/css-logical-1/#padding-properties";
    PADDING_INLINE_END: NonNegativeLengthPercentage = "padding-inline-end", "https://www.w3.org/TR/css-logical-1/#padding-properties";

    MARGIN: Margin = "margin", "https://www.w3.org/TR/css-box-4/#margin-shorthand";
    MARGIN_TOP: LengthPercentageAuto = "margin-top", "https://www.w3.org/TR/css-box-4/#margin-physical";
    MARGIN_RIGHT: LengthPercentageAuto = "margin-right", "https://www.w3.org/TR/css-box-4/#margin-physical";
    MARGIN_BOTTOM: LengthPercentageAuto = "margin-bottom", "https://www.w3.org/TR/css-box-4/#margin-physical";
    MARGIN_LEFT: LengthPercentageAuto = "margin-left", "https://www.w3.org/TR/css-box-4/#margin-physical";
    MARGIN_BLOCK: MarginAxis = "margin-block", "https://www.w3.org/TR/css-logical-1/#margin-properties";
    MARGIN_BLOCK_START: LengthPercentageAuto = "margin-block-start", "https://www.w3.org/TR/css-logical-1/#margin-properties";
    MARGIN_BLOCK_END: LengthPercentageAuto = "margin-block-end", "https://www.w3.org/TR/css-logical-1/#margin-properties";
    MARGIN_INLINE: MarginAxis = "margin-inline", "https://www.w3.org/TR/css-logical-1/#margin-properties";
    MARGIN_INLINE_START: LengthPercentageAuto = "margin-inline-start", "https://www.w3.org/TR/css-logical-1/#margin-properties";
    MARGIN_INLINE_END: LengthPercentageAuto = "margin-inline-end", "https://www.w3.org/TR/css-logical-1/#margin-properties";

    TOP: LengthPercentageAuto = "top", "https://www.w3.org/TR/css-position-3/#inset-properties";
    RIGHT: LengthPercentageAuto = "right", "https://www.w3.org/TR/css-position-3/#inset-properties";
    BOTTOM: LengthPercentageAuto = "bottom", "https://www.w3.org/TR/css-position-3/#inset-properties";
    LEFT: LengthPercentageAuto = "left", "https://www.w3.org/TR/css-position-3/#inset-properties";
    INSET: Inset = "inset", "https://www.w3.org/TR/css-logical-1/#propdef-inset";
    INSET_BLOCK: InsetAxis = "inset-block", "https://www.w3.org/TR/css-logical-1/#inset-properties";
    INSET_BLOCK_START: LengthPercentageAuto = "inset-block-start", "https://www.w3.org/TR/css-logical-1/#inset-properties";
    INSET_BLOCK_END: LengthPercentageAuto = "inset-block-end", "https://www.w3.org/TR/css-logical-1/#inset-properties";
    INSET_INLINE: InsetAxis = "inset-inline", "https://www.w3.org/TR/css-logical-1/#inset-properties";
    INSET_INLINE_START: LengthPercentageAuto = "inset-inline-start", "https://www.w3.org/TR/css-logical-1/#inset-properties";
    INSET_INLINE_END: LengthPercentageAuto = "inset-inline-end", "https://www.w3.org/TR/css-logical-1/#inset-properties";

    WIDTH: Size = "width", "https://www.w3.org/TR/css-sizing-3/#preferred-size-properties";
    HEIGHT: Size = "height", "https://www.w3.org/TR/css-sizing-3/#preferred-size-properties";
    BLOCK_SIZE: Size = "block-size", "https://www.w3.org/TR/css-logical-1/#dimension-properties";
    INLINE_SIZE: Size = "inline-size", "https://www.w3.org/TR/css-logical-1/#dimension-properties";
    MIN_WIDTH: Size = "min-width", "https://www.w3.org/TR/css-sizing-3/#min-size-properties";
    MIN_HEIGHT: Size = "min-height", "https://www.w3.org/TR/css-sizing-3/#min-size-properties";
    MIN_BLOCK_SIZE: Size = "min-block-size", "https://www.w3.org/TR/css-logical-1/#dimension-properties";
    MIN_INLINE_SIZE: Size = "min-inline-size", "https://www.w3.org/TR/css-logical-1/#dimension-properties";
    MAX_WIDTH: MaxSize = "max-width", "https://www.w3.org/TR/css-sizing-3/#max-size-properties";
    MAX_HEIGHT: MaxSize = "max-height", "https://www.w3.org/TR/css-sizing-3/#max-size-properties";
    MAX_BLOCK_SIZE: MaxSize = "max-block-size", "https://www.w3.org/TR/css-logical-1/#dimension-properties";
    MAX_INLINE_SIZE: MaxSize = "max-inline-size", "https://www.w3.org/TR/css-logical-1/#dimension-properties";
    BOX_SIZING: BoxSizing = "box-sizing", "https://www.w3.org/TR/css-sizing-3/#box-sizing";
    ASPECT_RATIO: AspectRatio = "aspect-ratio", "https://www.w3.org/TR/css-sizing-4/#aspect-ratio";

    OVERFLOW: Overflow = "overflow", "https://www.w3.org/TR/css-overflow-3/#propdef-overflow";
    OVERFLOW_X: OverflowKeyword = "overflow-x", "https://www.w3.org/TR/css-overflow-3/#overflow-control";
    OVERFLOW_Y: OverflowKeyword = "overflow-y", "https://www.w3.org/TR/css-overflow-3/#overflow-control";
    OVERFLOW_BLOCK: OverflowKeyword = "overflow-block", "https://www.w3.org/TR/css-overflow-3/#overflow-control";
    OVERFLOW_INLINE: OverflowKeyword = "overflow-inline", "https://www.w3.org/TR/css-overflow-3/#overflow-control";
    SCROLL_BEHAVIOR: ScrollBehavior = "scroll-behavior", "https://www.w3.org/TR/css-overflow-3/#smooth-scrolling";
    SCROLLBAR_GUTTER: ScrollbarGutter = "scrollbar-gutter", "https://www.w3.org/TR/css-overflow-3/#scrollbar-gutter-property";
    OVERSCROLL_BEHAVIOR: OverscrollBehavior = "overscroll-behavior", "https://www.w3.org/TR/css-overscroll-1/#propdef-overscroll-behavior";
    OVERSCROLL_BEHAVIOR_X: OverscrollKeyword = "overscroll-behavior-x", "https://www.w3.org/TR/css-overscroll-1/#overscroll-behavior-properties";
    OVERSCROLL_BEHAVIOR_Y: OverscrollKeyword = "overscroll-behavior-y", "https://www.w3.org/TR/css-overscroll-1/#overscroll-behavior-properties";
    OVERSCROLL_BEHAVIOR_BLOCK: OverscrollKeyword = "overscroll-behavior-block", "https://www.w3.org/TR/css-overscroll-1/#overscroll-behavior-properties";
    OVERSCROLL_BEHAVIOR_INLINE: OverscrollKeyword = "overscroll-behavior-inline", "https://www.w3.org/TR/css-overscroll-1/#overscroll-behavior-properties";

    FLEX: Flex = "flex", "https://www.w3.org/TR/css-flexbox-1/#flex-property";
    FLEX_BASIS: FlexBasis = "flex-basis", "https://www.w3.org/TR/css-flexbox-1/#flex-basis-property";
    FLEX_DIRECTION: FlexDirection = "flex-direction", "https://www.w3.org/TR/css-flexbox-1/#flex-direction-property";
    FLEX_FLOW: FlexFlow = "flex-flow", "https://www.w3.org/TR/css-flexbox-1/#flex-flow-property";
    FLEX_GROW: NonNegativeNumber = "flex-grow", "https://www.w3.org/TR/css-flexbox-1/#flex-grow-property";
    FLEX_SHRINK: NonNegativeNumber = "flex-shrink", "https://www.w3.org/TR/css-flexbox-1/#flex-shrink-property";
    FLEX_WRAP: FlexWrap = "flex-wrap", "https://www.w3.org/TR/css-flexbox-1/#flex-wrap-property";

    ALIGN_CONTENT: AlignContent = "align-content", "https://www.w3.org/TR/css-align-3/#align-justify-content";
    ALIGN_ITEMS: AlignItems = "align-items", "https://www.w3.org/TR/css-align-3/#align-items-property";
    ALIGN_SELF: AlignSelf = "align-self", "https://www.w3.org/TR/css-align-3/#align-self-property";
    JUSTIFY_CONTENT: JustifyContent = "justify-content", "https://www.w3.org/TR/css-align-3/#align-justify-content";
    JUSTIFY_ITEMS: JustifyItems = "justify-items", "https://www.w3.org/TR/css-align-3/#justify-items-property";
    JUSTIFY_SELF: JustifySelf = "justify-self", "https://www.w3.org/TR/css-align-3/#justify-self-property";
    PLACE_CONTENT: PlaceContent = "place-content", "https://www.w3.org/TR/css-align-3/#place-content";
    PLACE_ITEMS: PlaceItems = "place-items", "https://www.w3.org/TR/css-align-3/#place-items-property";
    PLACE_SELF: PlaceSelf = "place-self", "https://www.w3.org/TR/css-align-3/#place-self-property";
    GAP: Gap = "gap", "https://www.w3.org/TR/css-gaps-1/#gap-shorthand";
    ROW_GAP: GapValue = "row-gap", "https://www.w3.org/TR/css-gaps-1/#column-row-gap";
    COLUMN_GAP: GapValue = "column-gap", "https://www.w3.org/TR/css-gaps-1/#column-row-gap";

    GRID_TEMPLATE_COLUMNS: GridTemplate = "grid-template-columns", "https://www.w3.org/TR/css-grid-2/#track-sizing";
    GRID_TEMPLATE_ROWS: GridTemplate = "grid-template-rows", "https://www.w3.org/TR/css-grid-2/#track-sizing";
    GRID_AUTO_COLUMNS: GridAutoTracks = "grid-auto-columns", "https://www.w3.org/TR/css-grid-2/#auto-tracks";
    GRID_AUTO_ROWS: GridAutoTracks = "grid-auto-rows", "https://www.w3.org/TR/css-grid-2/#auto-tracks";
    GRID_AUTO_FLOW: GridAutoFlow = "grid-auto-flow", "https://www.w3.org/TR/css-grid-2/#grid-auto-flow-property";
    GRID_ROW: GridLineRange = "grid-row", "https://www.w3.org/TR/css-grid-2/#placement-shorthands";
    GRID_ROW_START: GridLine = "grid-row-start", "https://www.w3.org/TR/css-grid-2/#line-placement";
    GRID_ROW_END: GridLine = "grid-row-end", "https://www.w3.org/TR/css-grid-2/#line-placement";
    GRID_COLUMN: GridLineRange = "grid-column", "https://www.w3.org/TR/css-grid-2/#placement-shorthands";
    GRID_COLUMN_START: GridLine = "grid-column-start", "https://www.w3.org/TR/css-grid-2/#line-placement";
    GRID_COLUMN_END: GridLine = "grid-column-end", "https://www.w3.org/TR/css-grid-2/#line-placement";

    TRANSFORM: Transform = "transform", "https://www.w3.org/TR/css-transforms-1/#transform-property";
    TRANSLATE: Translate = "translate", "https://www.w3.org/TR/css-transforms-2/#propdef-translate";
    ROTATE: Rotate = "rotate", "https://www.w3.org/TR/css-transforms-2/#propdef-rotate";
    SCALE: Scale = "scale", "https://www.w3.org/TR/css-transforms-2/#propdef-scale";

    COLOR: Color = "color", "https://www.w3.org/TR/css-color-4/#propdef-color";
    BACKGROUND_COLOR: Color = "background-color", "https://www.w3.org/TR/css-backgrounds-3/#background-color";
    OPACITY: Opacity = "opacity", "https://www.w3.org/TR/css-color-4/#transparency";
    FORCED_COLOR_ADJUST: ForcedColorAdjust = "forced-color-adjust", "https://www.w3.org/TR/css-color-adjust-1/#forced-color-adjust-prop";
    PRINT_COLOR_ADJUST: PrintColorAdjust = "print-color-adjust", "https://www.w3.org/TR/css-color-adjust-1/#print-color-adjust";

    BORDER_RADIUS: BorderRadius = "border-radius", "https://www.w3.org/TR/css-backgrounds-3/#border-radius";
    BORDER_TOP_LEFT_RADIUS: CornerRadius = "border-top-left-radius", "https://www.w3.org/TR/css-backgrounds-3/#border-radius";
    BORDER_TOP_RIGHT_RADIUS: CornerRadius = "border-top-right-radius", "https://www.w3.org/TR/css-backgrounds-3/#border-radius";
    BORDER_BOTTOM_RIGHT_RADIUS: CornerRadius = "border-bottom-right-radius", "https://www.w3.org/TR/css-backgrounds-3/#border-radius";
    BORDER_BOTTOM_LEFT_RADIUS: CornerRadius = "border-bottom-left-radius", "https://www.w3.org/TR/css-backgrounds-3/#border-radius";
    BORDER_START_START_RADIUS: CornerRadius = "border-start-start-radius", "https://www.w3.org/TR/css-logical-1/#border-radius-properties";
    BORDER_START_END_RADIUS: CornerRadius = "border-start-end-radius", "https://www.w3.org/TR/css-logical-1/#border-radius-properties";
    BORDER_END_START_RADIUS: CornerRadius = "border-end-start-radius", "https://www.w3.org/TR/css-logical-1/#border-radius-properties";
    BORDER_END_END_RADIUS: CornerRadius = "border-end-end-radius", "https://www.w3.org/TR/css-logical-1/#border-radius-properties";

    FONT_SIZE: FontSize = "font-size", "https://www.w3.org/TR/css-fonts-4/#font-size-prop";
    FONT_STYLE: FontStyle = "font-style", "https://www.w3.org/TR/css-fonts-4/#font-style-prop";
    FONT_WEIGHT: FontWeight = "font-weight", "https://www.w3.org/TR/css-fonts-4/#font-weight-prop";
    LINE_HEIGHT: LineHeight = "line-height", "https://www.w3.org/TR/css-inline-3/#line-height-property";
    TEXT_ALIGN: TextAlign = "text-align", "https://www.w3.org/TR/css-text-3/#text-align-property";
    TEXT_DECORATION_LINE: TextDecorationLine = "text-decoration-line", "https://www.w3.org/TR/css-text-decor-4/#text-decoration-line-property";
    TEXT_DECORATION_COLOR: Color = "text-decoration-color", "https://www.w3.org/TR/css-text-decor-4/#text-decoration-color-property";
    TEXT_DECORATION_STYLE: TextDecorationStyle = "text-decoration-style", "https://www.w3.org/TR/css-text-decor-4/#text-decoration-style-property";
    TEXT_DECORATION_THICKNESS: TextDecorationThickness = "text-decoration-thickness", "https://drafts.csswg.org/css-text-decor-4/#text-decoration-width-property";
    TEXT_UNDERLINE_OFFSET: TextUnderlineOffset = "text-underline-offset", "https://drafts.csswg.org/css-text-decor-4/#underline-offset";

    CURSOR: Cursor = "cursor", "https://www.w3.org/TR/css-ui-4/#cursor";
    POINTER_EVENTS: PointerEvents = "pointer-events", "https://www.w3.org/TR/css-ui-4/#pointer-events-control";
    USER_SELECT: UserSelect = "user-select", "https://www.w3.org/TR/css-ui-4/#content-selection";
    TOUCH_ACTION: TouchAction = "touch-action", "https://www.w3.org/TR/pointerevents4/#the-touch-action-css-property";
    VIEW_TRANSITION_NAME: ViewTransitionName = "view-transition-name", "https://www.w3.org/TR/css-view-transitions-1/#view-transition-name-prop";
}

/// ```compile_fail
/// use typed_css::{LengthPercentageAuto, Padding, property::PADDING};
///
/// // `padding` does not accept `auto`.
/// PADDING.declare(LengthPercentageAuto::Auto);
/// ```
///
/// ```compile_fail
/// use typed_css::{px, property::TOUCH_ACTION};
///
/// // `touch-action` does not accept a length.
/// TOUCH_ACTION.declare(px(10));
/// ```
///
/// ```compile_fail
/// use typed_css::{Color, CustomProperty, property::WIDTH, var};
///
/// let accent = CustomProperty::<Color>::new("--accent-color");
/// // A color custom property cannot be used where a width is expected.
/// WIDTH.declare(var(&accent));
/// ```
///
/// ```compile_fail
/// use typed_css::{Color, CustomProperty, Size, var_or};
///
/// let accent = CustomProperty::<Color>::new("--accent-color");
/// // The fallback must have the grammar of the custom property.
/// let _ = var_or(&accent, Size::Auto);
/// ```
///
/// ```compile_fail
/// use typed_css::{Color, NamedColor, var};
///
/// // `var()` takes a custom property, not a name.
/// let _ = var("--accent-color");
/// ```
///
/// ```compile_fail
/// use typed_css::{Color, CustomProperty, Size};
///
/// let accent = CustomProperty::<Color>::new("--accent-color");
/// // A custom property accepts only values of its grammar.
/// let _ = accent.declare(Size::Auto);
/// ```
///
/// ```compile_fail
/// use typed_css::{LengthPercentage, custom_property, property::FLEX, var};
///
/// // `flex: var(--space)` would read a unitless `0` as a grow factor. Use `Flex::basis`.
/// custom_property!(SPACE: LengthPercentage = "--space");
/// let _ = FLEX.declare(var(&SPACE));
/// ```
///
/// ```compile_fail
/// // Legacy aliases are not supported.
/// use typed_css::property::GRID_GAP;
/// ```
///
/// ```compile_fail
/// // Speech properties are not supported.
/// use typed_css::property::SPEAK;
/// ```
///
/// ```compile_fail
/// // SVG paint properties are not supported.
/// use typed_css::property::FLOOD_COLOR;
/// ```
///
/// ```compile_fail
/// use typed_css::{CornerRadius, custom_property, property::BORDER_RADIUS, var};
///
/// // `4px 2px` is one elliptical corner, but two circular corners in `border-radius`.
/// custom_property!(CORNER: CornerRadius = "--corner");
/// let _ = BORDER_RADIUS.declare(var(&CORNER));
/// ```
///
/// ```compile_fail
/// use typed_css::property::WIDTH;
///
/// // A `Property` does not convert into a name that could be paired with any value.
/// let _: &str = WIDTH.as_ref();
/// ```
///
/// ```compile_fail
/// use typed_css::{GlobalKeyword, custom_property};
///
/// // A custom property never holds a CSS-wide keyword as its value.
/// custom_property!(KEYWORD: GlobalKeyword = "--keyword");
/// ```
///
/// ```compile_fail
/// use typed_css::{GlobalKeyword, property::ALL};
///
/// // `all` accepts CSS-wide keywords only through `declare_global` and methods such as `unset`.
/// let _ = ALL.declare(GlobalKeyword::Inherit);
/// ```
///
/// ```compile_fail
/// use typed_css::{RepeatCount, custom_property};
///
/// // Parts of a grammar, such as a grid repeat count, are not values of their own.
/// custom_property!(COUNT: RepeatCount = "--count");
/// ```
const _: () = ();

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;
    use crate::Declaration;

    type Keywords<T> = [(&'static str, fn(&Property<T>) -> Declaration)];

    /// Return each keyword whose method declares other CSS text than the keyword.
    fn mismatches<T>(
        property: Property<T>,
        keywords: &Keywords<T>,
    ) -> Vec<(Declaration, &'static str)> {
        keywords
            .iter()
            .map(|(css, method)| (method(&property), *css))
            .filter(|(declaration, css)| declaration.value() != *css)
            .collect()
    }

    /// Each grammar with keyword methods must be listed here, because an unused `KEYWORDS`
    /// constant is a warning.
    #[test]
    fn every_keyword_method_writes_its_keyword() {
        let mismatches: Vec<_> = [
            mismatches(DISPLAY, Property::<Display>::KEYWORDS),
            mismatches(VISIBILITY, Property::<Visibility>::KEYWORDS),
            mismatches(POSITION, Property::<Position>::KEYWORDS),
            mismatches(Z_INDEX, Property::<ZIndex>::KEYWORDS),
            mismatches(MARGIN, Property::<Margin>::KEYWORDS),
            mismatches(MARGIN_BLOCK, Property::<MarginAxis>::KEYWORDS),
            mismatches(INSET, Property::<Inset>::KEYWORDS),
            mismatches(INSET_INLINE, Property::<InsetAxis>::KEYWORDS),
            mismatches(MARGIN_TOP, Property::<LengthPercentageAuto>::KEYWORDS),
            mismatches(WIDTH, Property::<Size>::KEYWORDS),
            mismatches(MAX_WIDTH, Property::<MaxSize>::KEYWORDS),
            mismatches(BOX_SIZING, Property::<BoxSizing>::KEYWORDS),
            mismatches(ASPECT_RATIO, Property::<AspectRatio>::KEYWORDS),
            mismatches(OVERFLOW, Property::<Overflow>::KEYWORDS),
            mismatches(OVERFLOW_X, Property::<OverflowKeyword>::KEYWORDS),
            mismatches(SCROLL_BEHAVIOR, Property::<ScrollBehavior>::KEYWORDS),
            mismatches(SCROLLBAR_GUTTER, Property::<ScrollbarGutter>::KEYWORDS),
            mismatches(
                OVERSCROLL_BEHAVIOR,
                Property::<OverscrollBehavior>::KEYWORDS,
            ),
            mismatches(
                OVERSCROLL_BEHAVIOR_X,
                Property::<OverscrollKeyword>::KEYWORDS,
            ),
            mismatches(FLEX, Property::<Flex>::KEYWORDS),
            mismatches(FLEX_BASIS, Property::<FlexBasis>::KEYWORDS),
            mismatches(FLEX_DIRECTION, Property::<FlexDirection>::KEYWORDS),
            mismatches(FLEX_FLOW, Property::<FlexFlow>::KEYWORDS),
            mismatches(FLEX_WRAP, Property::<FlexWrap>::KEYWORDS),
            mismatches(ALIGN_CONTENT, Property::<AlignContent>::KEYWORDS),
            mismatches(ALIGN_ITEMS, Property::<AlignItems>::KEYWORDS),
            mismatches(ALIGN_SELF, Property::<AlignSelf>::KEYWORDS),
            mismatches(JUSTIFY_CONTENT, Property::<JustifyContent>::KEYWORDS),
            mismatches(JUSTIFY_ITEMS, Property::<JustifyItems>::KEYWORDS),
            mismatches(JUSTIFY_SELF, Property::<JustifySelf>::KEYWORDS),
            mismatches(PLACE_CONTENT, Property::<PlaceContent>::KEYWORDS),
            mismatches(PLACE_ITEMS, Property::<PlaceItems>::KEYWORDS),
            mismatches(PLACE_SELF, Property::<PlaceSelf>::KEYWORDS),
            mismatches(GAP, Property::<Gap>::KEYWORDS),
            mismatches(ROW_GAP, Property::<GapValue>::KEYWORDS),
            mismatches(GRID_TEMPLATE_COLUMNS, Property::<GridTemplate>::KEYWORDS),
            mismatches(GRID_AUTO_ROWS, Property::<GridAutoTracks>::KEYWORDS),
            mismatches(GRID_AUTO_FLOW, Property::<GridAutoFlow>::KEYWORDS),
            mismatches(GRID_ROW, Property::<GridLineRange>::KEYWORDS),
            mismatches(GRID_ROW_START, Property::<GridLine>::KEYWORDS),
            mismatches(TRANSFORM, Property::<Transform>::KEYWORDS),
            mismatches(TRANSLATE, Property::<Translate>::KEYWORDS),
            mismatches(ROTATE, Property::<Rotate>::KEYWORDS),
            mismatches(SCALE, Property::<Scale>::KEYWORDS),
            mismatches(FORCED_COLOR_ADJUST, Property::<ForcedColorAdjust>::KEYWORDS),
            mismatches(PRINT_COLOR_ADJUST, Property::<PrintColorAdjust>::KEYWORDS),
            mismatches(FONT_SIZE, Property::<FontSize>::KEYWORDS),
            mismatches(FONT_STYLE, Property::<FontStyle>::KEYWORDS),
            mismatches(FONT_WEIGHT, Property::<FontWeight>::KEYWORDS),
            mismatches(LINE_HEIGHT, Property::<LineHeight>::KEYWORDS),
            mismatches(TEXT_ALIGN, Property::<TextAlign>::KEYWORDS),
            mismatches(
                TEXT_DECORATION_LINE,
                Property::<TextDecorationLine>::KEYWORDS,
            ),
            mismatches(
                TEXT_DECORATION_STYLE,
                Property::<TextDecorationStyle>::KEYWORDS,
            ),
            mismatches(
                TEXT_DECORATION_THICKNESS,
                Property::<TextDecorationThickness>::KEYWORDS,
            ),
            mismatches(
                TEXT_UNDERLINE_OFFSET,
                Property::<TextUnderlineOffset>::KEYWORDS,
            ),
            mismatches(CURSOR, Property::<Cursor>::KEYWORDS),
            mismatches(POINTER_EVENTS, Property::<PointerEvents>::KEYWORDS),
            mismatches(USER_SELECT, Property::<UserSelect>::KEYWORDS),
            mismatches(TOUCH_ACTION, Property::<TouchAction>::KEYWORDS),
            mismatches(
                VIEW_TRANSITION_NAME,
                Property::<ViewTransitionName>::KEYWORDS,
            ),
        ]
        .into_iter()
        .flatten()
        .collect();
        assert_that!(mismatches).is_empty();
    }
}
