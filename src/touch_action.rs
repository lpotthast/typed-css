use std::fmt;

use crate::serialization::write_spaced_keywords;

use crate::auto::auto_sources;
use crate::checked_value::{impl_checked_value, impl_display, keyword_enum};
use crate::custom_property::declaration_sources;
use crate::property::definition::property_keywords;
use crate::serialization::ToCss;

keyword_enum! {
    /// A horizontal panning gesture that `touch-action` can allow.
    ///
    /// [Pointer Events Level 4 `touch-action`](https://www.w3.org/TR/pointerevents4/#the-touch-action-css-property)
    pub enum TouchActionHorizontalPan {
        /// Panning in both horizontal directions.
        PanX => "pan-x",
        /// Panning that starts by moving the finger to the left.
        PanLeft => "pan-left",
        /// Panning that starts by moving the finger to the right.
        PanRight => "pan-right",
    }
}

keyword_enum! {
    /// A vertical panning gesture that `touch-action` can allow.
    ///
    /// [Pointer Events Level 4 `touch-action`](https://www.w3.org/TR/pointerevents4/#the-touch-action-css-property)
    pub enum TouchActionVerticalPan {
        /// Panning in both vertical directions.
        PanY => "pan-y",
        /// Panning that starts by moving the finger up.
        PanUp => "pan-up",
        /// Panning that starts by moving the finger down.
        PanDown => "pan-down",
    }
}

/// A combination of the panning and pinch-zoom gestures that `touch-action` allows.
///
/// A combination contains at least one gesture, because every constructor starts with one.
/// Pointer Events Level 4 defines the panning gestures. The WHATWG Compatibility Standard adds
/// `pinch-zoom`.
///
/// ```rust
/// use typed_css::{TouchActionGestures, TouchActionHorizontalPan, TouchActionVerticalPan};
///
/// let gestures = TouchActionGestures::horizontal(TouchActionHorizontalPan::PanX)
///     .with_vertical(TouchActionVerticalPan::PanY)
///     .with_pinch_zoom();
/// assert_eq!(gestures.to_string(), "pan-x pan-y pinch-zoom");
/// ```
///
/// [Pointer Events Level 4 `touch-action` grammar](https://www.w3.org/TR/pointerevents4/#the-touch-action-css-property)
/// [WHATWG Compatibility `touch-action` extension](https://compat.spec.whatwg.org/#touch-action)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TouchActionGestures {
    horizontal: Option<TouchActionHorizontalPan>,
    vertical: Option<TouchActionVerticalPan>,
    pinch_zoom: bool,
}

impl TouchActionGestures {
    /// Start a combination with horizontal panning.
    #[must_use]
    pub const fn horizontal(horizontal: TouchActionHorizontalPan) -> Self {
        Self {
            horizontal: Some(horizontal),
            vertical: None,
            pinch_zoom: false,
        }
    }

    /// Start a combination with vertical panning.
    #[must_use]
    pub const fn vertical(vertical: TouchActionVerticalPan) -> Self {
        Self {
            horizontal: None,
            vertical: Some(vertical),
            pinch_zoom: false,
        }
    }

    /// Start a combination with pinch zooming.
    #[must_use]
    pub const fn pinch_zoom() -> Self {
        Self {
            horizontal: None,
            vertical: None,
            pinch_zoom: true,
        }
    }

    /// Add or replace the horizontal panning gesture.
    #[must_use]
    pub const fn with_horizontal(mut self, horizontal: TouchActionHorizontalPan) -> Self {
        self.horizontal = Some(horizontal);
        self
    }

    /// Add or replace the vertical panning gesture.
    #[must_use]
    pub const fn with_vertical(mut self, vertical: TouchActionVerticalPan) -> Self {
        self.vertical = Some(vertical);
        self
    }

    /// Add pinch zooming.
    #[must_use]
    pub const fn with_pinch_zoom(mut self) -> Self {
        self.pinch_zoom = true;
        self
    }
}

impl ToCss for TouchActionGestures {
    fn to_css<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        let gestures = [
            self.horizontal.map(TouchActionHorizontalPan::as_str),
            self.vertical.map(TouchActionVerticalPan::as_str),
            self.pinch_zoom.then_some("pinch-zoom"),
        ];
        write_spaced_keywords(w, gestures.into_iter().flatten())
    }
}

/// A `touch-action` value, which says which touch gestures the browser handles itself.
///
/// [Pointer Events Level 4 `touch-action` grammar](https://www.w3.org/TR/pointerevents4/#the-touch-action-css-property)
/// [WHATWG Compatibility `touch-action` extension](https://compat.spec.whatwg.org/#touch-action)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TouchAction {
    /// `auto`. The browser handles every gesture.
    Auto,
    /// `none`. The browser handles no gesture, so that the page can handle all of them.
    None,
    /// `manipulation`. The browser handles panning and pinch zooming, but not double-tap zooming.
    Manipulation,
    /// A combination of panning and pinch-zoom gestures.
    Gestures(TouchActionGestures),
}

impl From<TouchActionGestures> for TouchAction {
    fn from(value: TouchActionGestures) -> Self {
        Self::Gestures(value)
    }
}

declaration_sources!(TouchAction: TouchActionGestures);

impl ToCss for TouchAction {
    fn to_css<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match self {
            Self::Auto => w.write_str("auto"),
            Self::None => w.write_str("none"),
            Self::Manipulation => w.write_str("manipulation"),
            Self::Gestures(value) => value.to_css(w),
        }
    }
}

auto_sources!(TouchAction => Self::Auto);

impl_display!(TouchActionGestures, TouchAction,);
impl_checked_value!(TouchAction);

property_keywords! {
    TouchAction {
        auto: "auto" => TouchAction::Auto,
        none: "none" => TouchAction::None,
        manipulation: "manipulation" => TouchAction::Manipulation,
        pan_x: "pan-x" => TouchActionGestures::horizontal(TouchActionHorizontalPan::PanX),
        pan_left: "pan-left" => TouchActionGestures::horizontal(TouchActionHorizontalPan::PanLeft),
        pan_right: "pan-right" => TouchActionGestures::horizontal(TouchActionHorizontalPan::PanRight),
        pan_y: "pan-y" => TouchActionGestures::vertical(TouchActionVerticalPan::PanY),
        pan_up: "pan-up" => TouchActionGestures::vertical(TouchActionVerticalPan::PanUp),
        pan_down: "pan-down" => TouchActionGestures::vertical(TouchActionVerticalPan::PanDown),
        pinch_zoom: "pinch-zoom" => TouchActionGestures::pinch_zoom(),
    }
}
