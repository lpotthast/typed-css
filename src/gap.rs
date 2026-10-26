use std::fmt;

use crate::NonNegativeLengthPercentage;
use crate::checked_value::{impl_checked_value, impl_display};
use crate::custom_property::declaration_sources;
use crate::length_percentage::length_percentage_sources;
use crate::property::definition::property_keywords;
use crate::serialization::{ToCss, write_pair};

/// A `row-gap` or `column-gap` value, which is also one half of the `gap` shorthand.
///
/// [CSS Gaps Level 1 `row-gap` and `column-gap`](https://www.w3.org/TR/css-gaps-1/#column-row-gap)
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum GapValue {
    /// The `normal` keyword. It is `0` in flex and grid layouts and `1em` in multi-column layouts.
    Normal,
    /// A non-negative length, percentage, or math expression.
    LengthPercentage(NonNegativeLengthPercentage),
}

length_percentage_sources!(@non_negative GapValue, |value| Self::LengthPercentage(value.into()));

impl ToCss for GapValue {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        match self {
            Self::Normal => dest.write_str("normal"),
            Self::LengthPercentage(value) => value.to_css(dest),
        }
    }
}

/// A `gap` shorthand value, with a gap between rows and a gap between columns.
///
/// ```rust
/// use typed_css::{Gap, GapValue, px, rem};
///
/// assert_eq!(Gap::both(rem(1)).to_string(), "1rem");
/// assert_eq!(Gap::new(px(8), GapValue::Normal).to_string(), "8px normal");
/// ```
///
/// [CSS Gaps Level 1 `gap`](https://www.w3.org/TR/css-gaps-1/#gap-shorthand)
#[derive(Clone, Debug, PartialEq)]
pub struct Gap {
    row: GapValue,
    column: GapValue,
}

impl Gap {
    /// Use one gap between rows and between columns.
    #[must_use]
    pub fn both(value: impl Into<GapValue>) -> Self {
        let value = value.into();
        Self {
            row: value.clone(),
            column: value,
        }
    }

    /// Use separate gaps between rows and between columns.
    #[must_use]
    pub fn new(row: impl Into<GapValue>, column: impl Into<GapValue>) -> Self {
        Self {
            row: row.into(),
            column: column.into(),
        }
    }

    /// Replace the gap between rows.
    #[must_use]
    pub fn with_row(mut self, value: impl Into<GapValue>) -> Self {
        self.row = value.into();
        self
    }

    /// Replace the gap between columns.
    #[must_use]
    pub fn with_column(mut self, value: impl Into<GapValue>) -> Self {
        self.column = value.into();
        self
    }
}

impl From<GapValue> for Gap {
    fn from(value: GapValue) -> Self {
        Self::both(value)
    }
}

declaration_sources!(Gap: GapValue);

length_percentage_sources!(@non_negative Gap, |value| Self::from(GapValue::from(value)));

impl ToCss for Gap {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        write_pair(dest, &self.row, &self.column)
    }
}

impl_display!(GapValue, Gap);
impl_checked_value!(GapValue, Gap);

property_keywords! {
    GapValue, Gap {
        normal: "normal" => GapValue::Normal,
    }
}
