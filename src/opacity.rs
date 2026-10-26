use std::fmt;

use crate::checked_value::{impl_checked_value, impl_display};
use crate::custom_property::declaration_sources;
use crate::{InvalidNumber, PercentageChannel, UnitInterval, serialization::ToCss};

/// An `opacity` value from `0` (fully transparent) to `1` (fully opaque), or from `0%` to `100%`.
///
/// ```rust
/// use typed_css::{Opacity, property::OPACITY};
///
/// assert_eq!(OPACITY.declare(Opacity::new(0.5)).value(), "0.5");
/// assert_eq!(OPACITY.declare(Opacity::percent(50)).value(), "50%");
/// ```
///
/// [CSS Color Level 4 `opacity`](https://www.w3.org/TR/css-color-4/#transparency)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Opacity(OpacityValue);

#[derive(Clone, Copy, Debug, PartialEq)]
enum OpacityValue {
    Number(UnitInterval),
    Percentage(PercentageChannel),
}

impl Opacity {
    /// Create an opacity from a number from `0` to `1`.
    ///
    /// # Panics
    ///
    /// Panics if `value` is outside `0..=1` or not finite. Use [`Opacity::try_new`] for
    /// runtime-derived values.
    #[track_caller]
    #[must_use]
    pub fn new(value: impl Into<f64>) -> Self {
        Self::try_new(value).unwrap_or_else(|error| panic!("{error}"))
    }

    /// Attempt to create an opacity from a number from `0` to `1`.
    ///
    /// # Errors
    ///
    /// Returns an error if `value` is outside `0..=1` or not finite.
    pub fn try_new(value: impl Into<f64>) -> Result<Self, InvalidNumber> {
        UnitInterval::try_new(value).map(Self::from)
    }

    /// Create an opacity from a number of percent from `0` to `100`.
    ///
    /// # Panics
    ///
    /// Panics if `value` is outside `0..=100` or not finite. Use [`Opacity::try_percent`] for
    /// runtime-derived values.
    #[track_caller]
    #[must_use]
    pub fn percent(value: impl Into<f64>) -> Self {
        Self::try_percent(value).unwrap_or_else(|error| panic!("{error}"))
    }

    /// Attempt to create an opacity from a number of percent from `0` to `100`.
    ///
    /// # Errors
    ///
    /// Returns an error if `value` is outside `0..=100` or not finite.
    pub fn try_percent(value: impl Into<f64>) -> Result<Self, InvalidNumber> {
        PercentageChannel::try_new(value).map(|value| Self(OpacityValue::Percentage(value)))
    }
}

impl From<UnitInterval> for Opacity {
    fn from(value: UnitInterval) -> Self {
        Self(OpacityValue::Number(value))
    }
}

declaration_sources!(Opacity: UnitInterval);

impl ToCss for Opacity {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        match self.0 {
            OpacityValue::Number(value) => value.to_css(dest),
            OpacityValue::Percentage(value) => value.to_css(dest),
        }
    }
}

impl_display!(Opacity);
impl_checked_value!(Opacity);
