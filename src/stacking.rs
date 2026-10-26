use std::fmt;

use crate::auto::auto_sources;
use crate::checked_value::{impl_checked_value, impl_display};
use crate::custom_property::declaration_sources;
use crate::property::definition::property_keywords;
use crate::serialization::ToCss;

/// A `z-index` value.
///
/// A plain integer converts into a `ZIndex`, so `Z_INDEX.declare(10)` works.
///
/// [CSS 2 `z-index`](https://www.w3.org/TR/CSS22/visuren.html#z-index)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ZIndex {
    /// The `auto` keyword. The box is drawn at stack level 0 of the current stacking context, and
    /// `z-index` does not make it create a new stacking context.
    Auto,
    /// A stack level. Boxes with a higher level are drawn in front.
    Integer(i32),
}

impl From<i32> for ZIndex {
    fn from(level: i32) -> Self {
        Self::Integer(level)
    }
}

declaration_sources!(ZIndex: i32);

impl ToCss for ZIndex {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        match self {
            Self::Auto => dest.write_str("auto"),
            Self::Integer(level) => level.to_css(dest),
        }
    }
}

auto_sources!(ZIndex => Self::Auto);

impl_display!(ZIndex);
impl_checked_value!(ZIndex);

property_keywords! {
    ZIndex {
        auto: "auto" => ZIndex::Auto,
    }
}
