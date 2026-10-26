use std::fmt;

use crate::checked_value::{impl_checked_value, impl_display};
use crate::number::scaling_operators;
use crate::{InvalidNumber, Number, number::try_finite, serialization::ToCss};

/// A CSS `<percentage>`, such as `50%`.
///
/// The size that a percentage refers to depends on the property. For `width`, it is the width of
/// the containing block. Most size properties accept a length, a percentage, or a calculation of
/// both, which [`LengthPercentage`](crate::LengthPercentage) describes. A `Percentage` converts
/// into a `LengthPercentage`, so those properties accept it directly.
///
/// [CSS Values Level 4 percentages](https://www.w3.org/TR/css-values-4/#percentages)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Percentage(Number);

impl Percentage {
    /// Create a percentage. `Percentage::new(50.0)` is `50%`.
    ///
    /// # Panics
    ///
    /// Panics, or fails const evaluation, if `value` is `NaN` or infinite. Use
    /// [`Percentage::try_new`] for runtime-derived values.
    #[track_caller]
    #[must_use]
    pub const fn new(value: f64) -> Self {
        match Self::try_new(value) {
            Ok(percentage) => percentage,
            Err(_) => panic!("CSS % value must be finite"),
        }
    }

    /// Attempt to create a percentage.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidNumber::NonFinite`] if `value` is `NaN` or infinite.
    pub const fn try_new(value: f64) -> Result<Self, InvalidNumber> {
        match try_finite(value, "%") {
            Ok(number) => Ok(Self(number)),
            Err(error) => Err(error),
        }
    }

    /// Return the number of percent.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.0.get()
    }

    /// Attempt to transform the number of percent.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidNumber::NonFinite`] if `f` returns `NaN` or infinity.
    pub fn try_map_value(self, f: impl FnOnce(f64) -> f64) -> Result<Self, InvalidNumber> {
        Self::try_new(f(self.value()))
    }

    /// Transform the number of percent.
    ///
    /// # Panics
    ///
    /// Panics if `f` returns `NaN` or infinity. Use [`Percentage::try_map_value`] when the result
    /// can be invalid at runtime.
    #[must_use]
    #[track_caller]
    pub fn map_value(self, f: impl FnOnce(f64) -> f64) -> Self {
        self.try_map_value(f)
            .unwrap_or_else(|error| panic!("{error}"))
    }
}

impl ToCss for Percentage {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        self.0.to_css(dest)?;
        dest.write_char('%')
    }
}

scaling_operators!(Percentage);

/// Create a percentage. `pct(50)` is `50%`.
///
/// # Panics
///
/// Panics if `value` is `NaN` or infinite. Use [`Percentage::try_new`] for runtime-derived values.
#[track_caller]
#[must_use]
pub fn pct(value: impl Into<f64>) -> Percentage {
    Percentage::new(value.into())
}

impl_display!(Percentage);
impl_checked_value!(Percentage);
