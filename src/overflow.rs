use std::fmt;

use crate::auto::auto_sources;
use crate::checked_value::{impl_checked_value, impl_display, keyword_enum};
use crate::custom_property::declaration_sources;
use crate::serialization::{ToCss, write_pair};

keyword_enum! {
    /// How a box handles content that overflows it in one direction.
    ///
    /// The `overflow-x`, `overflow-y`, `overflow-block`, and `overflow-inline` properties take
    /// one value each.
    ///
    /// [CSS Overflow Level 3](https://www.w3.org/TR/css-overflow-3/#overflow-control)
    pub enum OverflowKeyword for Overflow {
        /// The content can overflow and stays visible.
        Visible => "visible" as visible,
        /// The overflowing content is hidden, but the box can still be scrolled by script.
        Hidden => "hidden" as hidden,
        /// The overflowing content is hidden, and the box cannot be scrolled at all.
        Clip => "clip" as clip,
        /// The box always shows scroll bars, where the platform has them.
        Scroll => "scroll" as scroll,
        /// The box shows scroll bars only when its content overflows.
        Auto => "auto" as auto,
    }
}

/// An `overflow` shorthand value, with the horizontal and the vertical overflow behavior.
///
/// ```rust
/// use typed_css::{Overflow, OverflowKeyword};
///
/// assert_eq!(Overflow::both(OverflowKeyword::Hidden).to_string(), "hidden");
/// assert_eq!(
///     Overflow::new(OverflowKeyword::Hidden, OverflowKeyword::Auto).to_string(),
///     "hidden auto",
/// );
/// ```
///
/// [CSS Overflow Level 3 `overflow`](https://www.w3.org/TR/css-overflow-3/#propdef-overflow)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Overflow {
    x: OverflowKeyword,
    y: OverflowKeyword,
}

impl Overflow {
    /// Use one overflow behavior for both directions.
    #[must_use]
    pub const fn both(value: OverflowKeyword) -> Self {
        Self { x: value, y: value }
    }

    /// Use separate horizontal and vertical overflow behaviors.
    #[must_use]
    pub const fn new(x: OverflowKeyword, y: OverflowKeyword) -> Self {
        Self { x, y }
    }

    /// Replace the horizontal overflow behavior.
    #[must_use]
    pub const fn with_x(mut self, x: OverflowKeyword) -> Self {
        self.x = x;
        self
    }

    /// Replace the vertical overflow behavior.
    #[must_use]
    pub const fn with_y(mut self, y: OverflowKeyword) -> Self {
        self.y = y;
        self
    }
}

impl From<OverflowKeyword> for Overflow {
    fn from(value: OverflowKeyword) -> Self {
        Self::both(value)
    }
}

declaration_sources!(Overflow: OverflowKeyword);

impl ToCss for Overflow {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        write_pair(dest, &self.x, &self.y)
    }
}

keyword_enum! {
    /// How a scroll container scrolls when script or navigation, such as a link to an anchor,
    /// scrolls it.
    ///
    /// [CSS Overflow Level 3 `scroll-behavior`](https://www.w3.org/TR/css-overflow-3/#smooth-scrolling)
    pub enum ScrollBehavior {
        /// The container jumps to the new position.
        Auto => "auto" as auto,
        /// The container scrolls smoothly to the new position.
        Smooth => "smooth" as smooth,
    }
}

keyword_enum! {
    /// Whether a scroll container reserves space for its scroll bar.
    ///
    /// The space only matters for classic scroll bars, which take up room. Overlay scroll bars
    /// never take room.
    ///
    /// [CSS Overflow Level 3 `scrollbar-gutter`](https://www.w3.org/TR/css-overflow-3/#scrollbar-gutter-property)
    pub enum ScrollbarGutter {
        /// The container reserves space only while it shows a classic scroll bar.
        Auto => "auto" as auto,
        /// The container always reserves space, so that its content does not shift when a scroll
        /// bar appears.
        Stable => "stable" as stable,
        /// The container reserves the space on both inline edges, so that its content stays
        /// centered.
        StableBothEdges => "stable both-edges" as stable_both_edges,
    }
}

keyword_enum! {
    /// What happens when a scroll container reaches its end in one direction.
    ///
    /// The `overscroll-behavior-x`, `overscroll-behavior-y`, `overscroll-behavior-block`, and
    /// `overscroll-behavior-inline` properties take one value each.
    ///
    /// [CSS Overscroll Behavior Level 1](https://www.w3.org/TR/css-overscroll-1/#overscroll-behavior-properties)
    pub enum OverscrollKeyword for OverscrollBehavior {
        /// Further scrolling moves the nearest ancestor that can scroll, and the browser may
        /// show effects such as a bounce or pull-to-refresh.
        Auto => "auto" as auto,
        /// Further scrolling does not move an ancestor, but the browser may still show effects
        /// such as a bounce.
        Contain => "contain" as contain,
        /// Further scrolling does not move an ancestor, and the browser shows no effects.
        None => "none" as none,
    }
}

/// An `overscroll-behavior` shorthand value, with the horizontal and the vertical behavior.
///
/// ```rust
/// use typed_css::{OverscrollBehavior, OverscrollKeyword};
///
/// assert_eq!(OverscrollBehavior::both(OverscrollKeyword::Contain).to_string(), "contain");
/// assert_eq!(
///     OverscrollBehavior::new(OverscrollKeyword::Auto, OverscrollKeyword::None).to_string(),
///     "auto none",
/// );
/// ```
///
/// [CSS Overscroll Behavior Level 1 `overscroll-behavior`](https://www.w3.org/TR/css-overscroll-1/#propdef-overscroll-behavior)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct OverscrollBehavior {
    x: OverscrollKeyword,
    y: OverscrollKeyword,
}

impl OverscrollBehavior {
    /// Use one behavior for both directions.
    #[must_use]
    pub const fn both(value: OverscrollKeyword) -> Self {
        Self { x: value, y: value }
    }

    /// Use separate horizontal and vertical behaviors.
    #[must_use]
    pub const fn new(x: OverscrollKeyword, y: OverscrollKeyword) -> Self {
        Self { x, y }
    }

    /// Replace the horizontal behavior.
    #[must_use]
    pub const fn with_x(mut self, x: OverscrollKeyword) -> Self {
        self.x = x;
        self
    }

    /// Replace the vertical behavior.
    #[must_use]
    pub const fn with_y(mut self, y: OverscrollKeyword) -> Self {
        self.y = y;
        self
    }
}

impl From<OverscrollKeyword> for OverscrollBehavior {
    fn from(value: OverscrollKeyword) -> Self {
        Self::both(value)
    }
}

declaration_sources!(OverscrollBehavior: OverscrollKeyword);

impl ToCss for OverscrollBehavior {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        write_pair(dest, &self.x, &self.y)
    }
}

auto_sources!(
    OverflowKeyword => Self::Auto,
    Overflow => Self::both(OverflowKeyword::Auto),
    ScrollBehavior => Self::Auto,
    ScrollbarGutter => Self::Auto,
    OverscrollKeyword => Self::Auto,
    OverscrollBehavior => Self::both(OverscrollKeyword::Auto),
);

impl_display!(Overflow, OverscrollBehavior);
impl_checked_value!(
    OverflowKeyword,
    Overflow,
    ScrollBehavior,
    ScrollbarGutter,
    OverscrollKeyword,
    OverscrollBehavior
);
