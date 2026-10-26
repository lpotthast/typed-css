use std::fmt;

use crate::auto::auto_sources;
use crate::checked_value::{impl_checked_value, impl_display};
use crate::custom_property::declaration_sources;
use crate::length_percentage::length_percentage_sources;
use crate::property::definition::property_keywords;
use crate::serialization::{ToCss, write_pair};
use crate::{LengthPercentageAuto, NonNegativeLengthPercentage};

/// Write four values in the order top, right, bottom, left, in the shortest equivalent form.
///
/// CSS repeats omitted values. The left value repeats the right one, the bottom value repeats the
/// top one, and the right value repeats the top one.
fn write_four<W: fmt::Write, T: ToCss + PartialEq>(
    dest: &mut W,
    [top, right, bottom, left]: [&T; 4],
) -> fmt::Result {
    let count = if left != right {
        4
    } else if bottom != top {
        3
    } else if right != top {
        2
    } else {
        1
    };
    for (index, value) in [top, right, bottom, left]
        .into_iter()
        .take(count)
        .enumerate()
    {
        if index > 0 {
            dest.write_char(' ')?;
        }
        value.to_css(dest)?;
    }
    Ok(())
}

macro_rules! box_shorthand {
    ($(#[$meta:meta])* $name:ident, $value:ty) => {
        $(#[$meta])*
        #[derive(Clone, Debug, PartialEq)]
        pub struct $name {
            top: $value,
            right: $value,
            bottom: $value,
            left: $value,
        }

        impl $name {
            /// Use one value for all four edges.
            #[must_use]
            pub fn all(value: impl Into<$value>) -> Self {
                let value = value.into();
                Self {
                    top: value.clone(),
                    right: value.clone(),
                    bottom: value.clone(),
                    left: value,
                }
            }

            /// Use one value for the top and bottom edges and another for the left and right
            /// edges.
            #[must_use]
            pub fn symmetric(vertical: impl Into<$value>, horizontal: impl Into<$value>) -> Self {
                let vertical = vertical.into();
                let horizontal = horizontal.into();
                Self {
                    top: vertical.clone(),
                    right: horizontal.clone(),
                    bottom: vertical,
                    left: horizontal,
                }
            }

            /// Use a separate value for each edge.
            #[must_use]
            pub fn edges(
                top: impl Into<$value>,
                right: impl Into<$value>,
                bottom: impl Into<$value>,
                left: impl Into<$value>,
            ) -> Self {
                Self {
                    top: top.into(),
                    right: right.into(),
                    bottom: bottom.into(),
                    left: left.into(),
                }
            }

            /// Replace the value of the top edge.
            #[must_use]
            pub fn with_top(mut self, value: impl Into<$value>) -> Self {
                self.top = value.into();
                self
            }

            /// Replace the value of the right edge.
            #[must_use]
            pub fn with_right(mut self, value: impl Into<$value>) -> Self {
                self.right = value.into();
                self
            }

            /// Replace the value of the bottom edge.
            #[must_use]
            pub fn with_bottom(mut self, value: impl Into<$value>) -> Self {
                self.bottom = value.into();
                self
            }

            /// Replace the value of the left edge.
            #[must_use]
            pub fn with_left(mut self, value: impl Into<$value>) -> Self {
                self.left = value.into();
                self
            }
        }

        impl ToCss for $name {
            fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
                write_four(dest, [&self.top, &self.right, &self.bottom, &self.left])
            }
        }
    };
}

box_shorthand!(
    /// A `padding` shorthand value, with a non-negative length-percentage for each edge.
    ///
    /// The value is written in the shortest form that CSS expands to the same four edges.
    ///
    /// ```rust
    /// use typed_css::{Padding, px, rem};
    ///
    /// assert_eq!(Padding::all(px(8)).to_string(), "8px");
    /// assert_eq!(Padding::symmetric(px(8), rem(1)).to_string(), "8px 1rem");
    /// assert_eq!(Padding::all(px(8)).with_bottom(px(0)).to_string(), "8px 8px 0px");
    /// ```
    ///
    /// [CSS Box Model Level 4 `padding`](https://www.w3.org/TR/css-box-4/#padding-shorthand)
    Padding,
    NonNegativeLengthPercentage
);

box_shorthand!(
    /// A `margin` shorthand value, with a length-percentage or `auto` for each edge.
    ///
    /// The value is written in the shortest form that CSS expands to the same four edges.
    ///
    /// ```rust
    /// use typed_css::{LengthPercentageAuto, Margin, px};
    ///
    /// assert_eq!(Margin::symmetric(px(0), LengthPercentageAuto::Auto).to_string(), "0px auto");
    /// ```
    ///
    /// [CSS Box Model Level 4 `margin`](https://www.w3.org/TR/css-box-4/#margin-shorthand)
    Margin,
    LengthPercentageAuto
);

box_shorthand!(
    /// An `inset` shorthand value, with a length-percentage or `auto` for each edge.
    ///
    /// The value is written in the shortest form that CSS expands to the same four edges.
    ///
    /// [CSS Logical Properties `inset`](https://www.w3.org/TR/css-logical-1/#propdef-inset)
    Inset,
    LengthPercentageAuto
);

macro_rules! axis_shorthand {
    ($(#[$meta:meta])* $name:ident, $value:ty) => {
        $(#[$meta])*
        #[derive(Clone, Debug, PartialEq)]
        pub struct $name {
            start: $value,
            end: $value,
        }

        impl $name {
            /// Use one value for the start and the end edge.
            #[must_use]
            pub fn both(value: impl Into<$value>) -> Self {
                let value = value.into();
                Self {
                    start: value.clone(),
                    end: value,
                }
            }

            /// Use separate values for the start and the end edge.
            #[must_use]
            pub fn new(start: impl Into<$value>, end: impl Into<$value>) -> Self {
                Self {
                    start: start.into(),
                    end: end.into(),
                }
            }

            /// Replace the value of the start edge.
            #[must_use]
            pub fn with_start(mut self, value: impl Into<$value>) -> Self {
                self.start = value.into();
                self
            }

            /// Replace the value of the end edge.
            #[must_use]
            pub fn with_end(mut self, value: impl Into<$value>) -> Self {
                self.end = value.into();
                self
            }
        }

        impl ToCss for $name {
            fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
                write_pair(dest, &self.start, &self.end)
            }
        }
    };
}

axis_shorthand!(
    /// A `padding-block` or `padding-inline` value.
    ///
    /// [CSS Logical Properties padding](https://www.w3.org/TR/css-logical-1/#padding-properties)
    PaddingAxis,
    NonNegativeLengthPercentage
);

axis_shorthand!(
    /// A `margin-block` or `margin-inline` value.
    ///
    /// [CSS Logical Properties margin](https://www.w3.org/TR/css-logical-1/#margin-properties)
    MarginAxis,
    LengthPercentageAuto
);

axis_shorthand!(
    /// An `inset-block` or `inset-inline` value.
    ///
    /// [CSS Logical Properties inset](https://www.w3.org/TR/css-logical-1/#inset-properties)
    InsetAxis,
    LengthPercentageAuto
);

/// The radius of one corner, for properties such as `border-top-left-radius`.
///
/// A corner is circular with one radius or elliptical with a horizontal and a vertical radius.
///
/// [CSS Backgrounds and Borders Level 3 `border-radius`](https://www.w3.org/TR/css-backgrounds-3/#border-radius)
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum CornerRadius {
    /// One radius in both directions.
    Circular(NonNegativeLengthPercentage),
    /// A horizontal radius and a vertical radius.
    Elliptical(NonNegativeLengthPercentage, NonNegativeLengthPercentage),
}

impl CornerRadius {
    /// Create an elliptical corner.
    #[must_use]
    pub fn elliptical(
        horizontal: impl Into<NonNegativeLengthPercentage>,
        vertical: impl Into<NonNegativeLengthPercentage>,
    ) -> Self {
        Self::Elliptical(horizontal.into(), vertical.into())
    }

    fn horizontal(&self) -> &NonNegativeLengthPercentage {
        match self {
            Self::Circular(radius) | Self::Elliptical(radius, _) => radius,
        }
    }

    fn vertical(&self) -> &NonNegativeLengthPercentage {
        match self {
            Self::Circular(radius) | Self::Elliptical(_, radius) => radius,
        }
    }
}

length_percentage_sources!(@non_negative CornerRadius, |value| Self::Circular(value.into()));

impl ToCss for CornerRadius {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        match self {
            Self::Circular(radius) => radius.to_css(dest),
            Self::Elliptical(horizontal, vertical) if horizontal == vertical => {
                horizontal.to_css(dest)
            }
            Self::Elliptical(horizontal, vertical) => {
                horizontal.to_css(dest)?;
                dest.write_char(' ')?;
                vertical.to_css(dest)
            }
        }
    }
}

/// A `border-radius` shorthand value, with a radius for each corner.
///
/// The value is written in the shortest equivalent form. The vertical radii follow a `/` when they
/// differ from the horizontal radii.
///
/// ```rust
/// use typed_css::{BorderRadius, CornerRadius, px};
///
/// assert_eq!(BorderRadius::all(px(8)).to_string(), "8px");
/// assert_eq!(
///     BorderRadius::all(px(8)).with_bottom_right(CornerRadius::elliptical(px(4), px(2))).to_string(),
///     "8px 8px 4px / 8px 8px 2px",
/// );
/// ```
///
/// [CSS Backgrounds and Borders Level 3 `border-radius`](https://www.w3.org/TR/css-backgrounds-3/#border-radius)
#[derive(Clone, Debug, PartialEq)]
pub struct BorderRadius {
    top_left: CornerRadius,
    top_right: CornerRadius,
    bottom_right: CornerRadius,
    bottom_left: CornerRadius,
}

impl BorderRadius {
    /// Use one radius for every corner.
    #[must_use]
    pub fn all(radius: impl Into<CornerRadius>) -> Self {
        let radius = radius.into();
        Self {
            top_left: radius.clone(),
            top_right: radius.clone(),
            bottom_right: radius.clone(),
            bottom_left: radius,
        }
    }

    /// Use a separate radius for each corner.
    #[must_use]
    pub fn corners(
        top_left: impl Into<CornerRadius>,
        top_right: impl Into<CornerRadius>,
        bottom_right: impl Into<CornerRadius>,
        bottom_left: impl Into<CornerRadius>,
    ) -> Self {
        Self {
            top_left: top_left.into(),
            top_right: top_right.into(),
            bottom_right: bottom_right.into(),
            bottom_left: bottom_left.into(),
        }
    }

    /// Replace the radius of the top-left corner.
    #[must_use]
    pub fn with_top_left(mut self, radius: impl Into<CornerRadius>) -> Self {
        self.top_left = radius.into();
        self
    }

    /// Replace the radius of the top-right corner.
    #[must_use]
    pub fn with_top_right(mut self, radius: impl Into<CornerRadius>) -> Self {
        self.top_right = radius.into();
        self
    }

    /// Replace the radius of the bottom-right corner.
    #[must_use]
    pub fn with_bottom_right(mut self, radius: impl Into<CornerRadius>) -> Self {
        self.bottom_right = radius.into();
        self
    }

    /// Replace the radius of the bottom-left corner.
    #[must_use]
    pub fn with_bottom_left(mut self, radius: impl Into<CornerRadius>) -> Self {
        self.bottom_left = radius.into();
        self
    }

    const fn corners_in_order(&self) -> [&CornerRadius; 4] {
        [
            &self.top_left,
            &self.top_right,
            &self.bottom_right,
            &self.bottom_left,
        ]
    }
}

// `CornerRadius` deliberately does not convert into `BorderRadius`. The text `4px 2px` is one
// elliptical corner in a corner longhand, but two circular corners in `border-radius`, so a
// `var()` of a `CornerRadius` would change its meaning there. `BorderRadius::all` is explicit.

length_percentage_sources!(@non_negative BorderRadius, |value| Self::all(CornerRadius::Circular(
    value.into()
)));

impl ToCss for BorderRadius {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        let corners = self.corners_in_order();
        let horizontal = corners.map(CornerRadius::horizontal);
        let vertical = corners.map(CornerRadius::vertical);
        write_four(dest, horizontal)?;
        if vertical != horizontal {
            dest.write_str(" / ")?;
            write_four(dest, vertical)?;
        }
        Ok(())
    }
}

// One value converts into a shorthand that repeats it for every edge.
length_percentage_sources!(@non_negative Padding, |value| Self::all(value));
length_percentage_sources!(@non_negative PaddingAxis, |value| Self::both(value));
length_percentage_sources!(Margin, |value| Self::all(value));
length_percentage_sources!(MarginAxis, |value| Self::both(value));
length_percentage_sources!(Inset, |value| Self::all(value));
length_percentage_sources!(InsetAxis, |value| Self::both(value));

macro_rules! repeated_sources {
    ($($name:ident => $repeat:ident),+ $(,)?) => {$(
        impl From<LengthPercentageAuto> for $name {
            fn from(value: LengthPercentageAuto) -> Self {
                Self::$repeat(value)
            }
        }

        declaration_sources!($name: LengthPercentageAuto);
    )+};
}

repeated_sources!(Margin => all, MarginAxis => both, Inset => all, InsetAxis => both);

auto_sources!(
    Margin => Self::all(LengthPercentageAuto::Auto),
    MarginAxis => Self::both(LengthPercentageAuto::Auto),
    Inset => Self::all(LengthPercentageAuto::Auto),
    InsetAxis => Self::both(LengthPercentageAuto::Auto),
);

impl_display!(
    Margin,
    Inset,
    Padding,
    PaddingAxis,
    MarginAxis,
    InsetAxis,
    CornerRadius,
    BorderRadius,
);
impl_checked_value!(
    Padding,
    Margin,
    Inset,
    PaddingAxis,
    MarginAxis,
    InsetAxis,
    CornerRadius,
    BorderRadius
);

property_keywords! {
    Margin, MarginAxis, Inset, InsetAxis {
        auto: "auto" => crate::Auto,
    }
}
