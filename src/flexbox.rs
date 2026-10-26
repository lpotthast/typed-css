use std::fmt;

use crate::auto::auto_sources;
use crate::checked_value::{impl_checked_value, impl_display, keyword_enum};
use crate::custom_property::declaration_sources;
use crate::length_percentage::length_percentage_sources;
use crate::property::definition::property_keywords;
use crate::serialization::write_list;
use crate::{
    DeclarationValue, InvalidNumber, Length, LengthPercentage, NonNegativeLengthPercentage,
    NonNegativeNumber, Percentage, Size, pct, serialization::ToCss,
};

keyword_enum! {
    /// A `flex-direction` value.
    ///
    /// [CSS Flexbox Level 1 `flex-direction` grammar](https://www.w3.org/TR/css-flexbox-1/#flex-direction-property)
    pub enum FlexDirection {
        Row => "row" as row,
        RowReverse => "row-reverse" as row_reverse,
        Column => "column" as column,
        ColumnReverse => "column-reverse" as column_reverse,
    }
}

keyword_enum! {
    /// A `flex-wrap` value.
    ///
    /// [CSS Flexbox Level 1 `flex-wrap` grammar](https://www.w3.org/TR/css-flexbox-1/#flex-wrap-property)
    pub enum FlexWrap {
        Nowrap => "nowrap" as nowrap,
        Wrap => "wrap" as wrap,
        WrapReverse => "wrap-reverse" as wrap_reverse,
    }
}

/// A `flex-flow` shorthand value, which combines a direction and a wrap mode.
///
/// The constructors set one component and leave the other at its initial value, which is `row`
/// for the direction and `nowrap` for the wrap mode. The value is written in the shortest form.
///
/// ```rust
/// use typed_css::{FlexDirection, FlexFlow, FlexWrap};
///
/// let flow = FlexFlow::direction(FlexDirection::Column).with_wrap(FlexWrap::Wrap);
/// assert_eq!(flow.to_string(), "column wrap");
/// assert_eq!(FlexFlow::wrap(FlexWrap::Wrap).to_string(), "wrap");
/// ```
///
/// [CSS Flexbox Level 1 `flex-flow` grammar](https://www.w3.org/TR/css-flexbox-1/#flex-flow-property)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FlexFlow {
    direction: FlexDirection,
    wrap: FlexWrap,
}

impl FlexFlow {
    /// Set the direction, with the wrap mode `nowrap`.
    #[must_use]
    pub const fn direction(direction: FlexDirection) -> Self {
        Self {
            direction,
            wrap: FlexWrap::Nowrap,
        }
    }

    /// Set the wrap mode, with the direction `row`.
    #[must_use]
    pub const fn wrap(wrap: FlexWrap) -> Self {
        Self {
            direction: FlexDirection::Row,
            wrap,
        }
    }

    /// Replace the direction.
    #[must_use]
    pub const fn with_direction(mut self, direction: FlexDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Replace the wrap mode.
    #[must_use]
    pub const fn with_wrap(mut self, wrap: FlexWrap) -> Self {
        self.wrap = wrap;
        self
    }
}

impl From<FlexDirection> for FlexFlow {
    fn from(direction: FlexDirection) -> Self {
        Self::direction(direction)
    }
}

impl From<FlexWrap> for FlexFlow {
    fn from(wrap: FlexWrap) -> Self {
        Self::wrap(wrap)
    }
}

declaration_sources!(FlexFlow: FlexDirection, FlexWrap);

impl ToCss for FlexFlow {
    fn to_css<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match (self.direction, self.wrap) {
            (direction, FlexWrap::Nowrap) => direction.to_css(w),
            (FlexDirection::Row, wrap) => wrap.to_css(w),
            (direction, wrap) => {
                direction.to_css(w)?;
                w.write_char(' ')?;
                wrap.to_css(w)
            }
        }
    }
}

/// A `flex-basis` value, which is `content` or any `width` value.
///
/// [CSS Flexbox Level 1 `flex-basis` grammar](https://www.w3.org/TR/css-flexbox-1/#flex-basis-property)
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum FlexBasis {
    /// The `content` keyword.
    Content,
    /// A `width` value, including `auto`.
    Width(Size),
}

impl From<Size> for FlexBasis {
    fn from(size: Size) -> Self {
        Self::Width(size)
    }
}

declaration_sources!(FlexBasis: Size);
length_percentage_sources!(@non_negative FlexBasis, |value| Self::Width(value.into()));

impl ToCss for FlexBasis {
    fn to_css<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match self {
            Self::Content => w.write_str("content"),
            Self::Width(size) => size.to_css(w),
        }
    }
}

/// A `flex` shorthand value, with a grow factor, a shrink factor, and a basis.
///
/// The grow factor says how much of the free space an item takes, and the shrink factor says how
/// much it gives up when space is missing. The basis is the item's size before growing or
/// shrinking. The value is written in the shortest equivalent form.
///
/// ```rust
/// use typed_css::{Flex, px};
///
/// assert_eq!(Flex::grow(1).to_string(), "1");
/// assert_eq!(Flex::grow(2).with_shrink(0).with_basis(px(120)).to_string(), "2 0 120px");
/// assert_eq!(Flex::basis(px(80)).to_string(), "80px");
/// assert_eq!(Flex::auto().to_string(), "auto");
/// assert_eq!(Flex::none().to_string(), "none");
/// ```
///
/// [CSS Flexbox Level 1 `flex` grammar](https://www.w3.org/TR/css-flexbox-1/#flex-property)
#[derive(Clone, Debug, PartialEq)]
pub struct Flex {
    grow: NonNegativeNumber,
    shrink: NonNegativeNumber,
    basis: FlexBasis,
}

fn zero_percent_basis() -> FlexBasis {
    FlexBasis::from(pct(0))
}

fn auto_basis() -> FlexBasis {
    FlexBasis::Width(Size::Auto)
}

impl Flex {
    /// Create a value from all three components.
    ///
    /// # Panics
    ///
    /// Panics if `grow` or `shrink` is negative or not finite. Use [`Flex::try_new`] for
    /// runtime-derived values.
    #[track_caller]
    #[must_use]
    pub fn new(grow: impl Into<f64>, shrink: impl Into<f64>, basis: impl Into<FlexBasis>) -> Self {
        Self::try_new(grow, shrink, basis).unwrap_or_else(|error| panic!("{error}"))
    }

    /// Attempt to create a value from all three components.
    ///
    /// # Errors
    ///
    /// Returns an error if `grow` or `shrink` is negative or not finite.
    pub fn try_new(
        grow: impl Into<f64>,
        shrink: impl Into<f64>,
        basis: impl Into<FlexBasis>,
    ) -> Result<Self, InvalidNumber> {
        Ok(Self {
            grow: NonNegativeNumber::try_for(grow.into(), "flex grow factor")?,
            shrink: NonNegativeNumber::try_for(shrink.into(), "flex shrink factor")?,
            basis: basis.into(),
        })
    }

    /// The `none` keyword, the same as `0 0 auto`. The item keeps its size.
    #[must_use]
    pub fn none() -> Self {
        Self::new(0, 0, auto_basis())
    }

    /// The `auto` keyword, the same as `1 1 auto`. The item starts at its size and then grows or
    /// shrinks.
    #[must_use]
    pub fn auto() -> Self {
        Self::new(1, 1, auto_basis())
    }

    /// Use the grow factor `grow`, the shrink factor `1`, and the basis `0%`, as `flex: <grow>`
    /// does.
    ///
    /// # Panics
    ///
    /// Panics if `grow` is negative or not finite.
    #[track_caller]
    #[must_use]
    pub fn grow(grow: impl Into<f64>) -> Self {
        Self::new(grow, 1, zero_percent_basis())
    }

    /// Use the grow factor `1`, the shrink factor `1`, and `basis`, as `flex: <basis>` does.
    #[must_use]
    pub fn basis(basis: impl Into<FlexBasis>) -> Self {
        Self::new(1, 1, basis)
    }

    /// Replace the grow factor.
    ///
    /// # Panics
    ///
    /// Panics if `grow` is negative or not finite. Use [`Flex::try_with_grow`] for
    /// runtime-derived values.
    #[must_use]
    #[track_caller]
    pub fn with_grow(self, grow: impl Into<f64>) -> Self {
        self.try_with_grow(grow)
            .unwrap_or_else(|error| panic!("{error}"))
    }

    /// Attempt to replace the grow factor.
    ///
    /// # Errors
    ///
    /// Returns an error if `grow` is negative or not finite.
    pub fn try_with_grow(mut self, grow: impl Into<f64>) -> Result<Self, InvalidNumber> {
        self.grow = NonNegativeNumber::try_for(grow.into(), "flex grow factor")?;
        Ok(self)
    }

    /// Replace the shrink factor.
    ///
    /// # Panics
    ///
    /// Panics if `shrink` is negative or not finite. Use [`Flex::try_with_shrink`] for
    /// runtime-derived values.
    #[must_use]
    #[track_caller]
    pub fn with_shrink(self, shrink: impl Into<f64>) -> Self {
        self.try_with_shrink(shrink)
            .unwrap_or_else(|error| panic!("{error}"))
    }

    /// Attempt to replace the shrink factor.
    ///
    /// # Errors
    ///
    /// Returns an error if `shrink` is negative or not finite.
    pub fn try_with_shrink(mut self, shrink: impl Into<f64>) -> Result<Self, InvalidNumber> {
        self.shrink = NonNegativeNumber::try_for(shrink.into(), "flex shrink factor")?;
        Ok(self)
    }

    /// Replace the basis.
    #[must_use]
    pub fn with_basis(mut self, basis: impl Into<FlexBasis>) -> Self {
        self.basis = basis.into();
        self
    }
}

/// A single number is the grow factor, so `flex: 2` is `Flex::grow(2)`.
impl From<NonNegativeNumber> for Flex {
    fn from(grow: NonNegativeNumber) -> Self {
        Self::grow(grow.get())
    }
}

impl From<u32> for Flex {
    fn from(grow: u32) -> Self {
        Self::grow(grow)
    }
}

declaration_sources!(Flex: NonNegativeNumber, u32);

// A basis converts into the declaration value of `flex`, but not into `Flex` itself. With
// `From<FlexBasis> for Flex`, a `var()` of a basis would widen into `flex: var(--basis)`, and a
// unitless `0` in that custom property would then mean a grow factor. `Flex::basis` writes a
// `var()` basis with both factors instead.
macro_rules! basis_sources {
    ($($source:ty),+) => {$(
        impl From<$source> for DeclarationValue<Flex> {
            fn from(basis: $source) -> Self {
                Flex::basis(basis).into()
            }
        }
    )+};
}

basis_sources!(
    FlexBasis,
    Length,
    Percentage,
    LengthPercentage,
    NonNegativeLengthPercentage
);

impl Flex {
    fn basis_is_variable(&self) -> bool {
        matches!(
            &self.basis,
            FlexBasis::Width(Size::LengthPercentage(value))
                if value.as_length_percentage().is_variable()
        )
    }
}

impl ToCss for Flex {
    #[expect(
        clippy::float_cmp,
        reason = "only exactly the default factors may be left out of the shorthand"
    )]
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        let grow = self.grow.get();
        let shrink = self.shrink.get();
        // A unitless `0` that a `var()` basis substitutes would be read as a flex factor, unless
        // both factors come before it.
        if self.basis_is_variable() {
            return write_list!(dest, " "; &self.grow, &self.shrink, &self.basis);
        }
        let is_auto_basis = self.basis == auto_basis();
        if is_auto_basis && grow == 0.0 && shrink == 0.0 {
            return dest.write_str("none");
        }
        if is_auto_basis && grow == 1.0 && shrink == 1.0 {
            return dest.write_str("auto");
        }
        // An omitted shrink factor is `1`, and an omitted basis is `0%`. An omitted grow factor is
        // `1`, but the grow factor can only be omitted when the shrink factor is omitted too.
        if self.basis == zero_percent_basis() {
            self.grow.to_css(dest)?;
            if shrink != 1.0 {
                dest.write_char(' ')?;
                self.shrink.to_css(dest)?;
            }
            return Ok(());
        }
        if grow != 1.0 || shrink != 1.0 {
            self.grow.to_css(dest)?;
            dest.write_char(' ')?;
            if shrink != 1.0 {
                self.shrink.to_css(dest)?;
                dest.write_char(' ')?;
            }
        }
        self.basis.to_css(dest)
    }
}

auto_sources!(FlexBasis => Self::Width(Size::Auto), Flex => Self::auto());

impl_display!(FlexFlow, FlexBasis, Flex);
impl_checked_value!(FlexDirection, FlexWrap, FlexFlow, FlexBasis, Flex);

// `flex-flow: nowrap` is written as the equivalent `row`, so `flex-flow` has no `nowrap` method.
property_keywords! {
    FlexFlow {
        row: "row" => FlexDirection::Row,
        row_reverse: "row-reverse" => FlexDirection::RowReverse,
        column: "column" => FlexDirection::Column,
        column_reverse: "column-reverse" => FlexDirection::ColumnReverse,
        wrap: "wrap" => FlexWrap::Wrap,
        wrap_reverse: "wrap-reverse" => FlexWrap::WrapReverse,
    }
}

property_keywords! {
    FlexBasis {
        auto: "auto" => crate::Auto,
        content: "content" => FlexBasis::Content,
        min_content: "min-content" => Size::MinContent,
        max_content: "max-content" => Size::MaxContent,
        fit_content: "fit-content" => Size::FitContent,
    }
}

property_keywords! {
    Flex {
        auto: "auto" => Flex::auto(),
        none: "none" => Flex::none(),
        content: "content" => FlexBasis::Content,
        min_content: "min-content" => FlexBasis::Width(Size::MinContent),
        max_content: "max-content" => FlexBasis::Width(Size::MaxContent),
        fit_content: "fit-content" => FlexBasis::Width(Size::FitContent),
    }
}
