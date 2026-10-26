use std::fmt;

use crate::auto::auto_sources;
use crate::checked_value::{impl_checked_value, impl_display, keyword_enum};
use crate::custom_property::declaration_sources;
use crate::length_percentage::length_percentage_sources;
use crate::property::definition::property_keywords;
use crate::{InvalidNumber, NonNegativeLengthPercentage, NonNegativeNumber, serialization::ToCss};

macro_rules! size_grammar {
    ($(#[$meta:meta])* $name:ident, $(#[$keyword_meta:meta])* $keyword:ident => $keyword_css:literal) => {
        $(#[$meta])*
        #[derive(Clone, Debug, PartialEq)]
        #[non_exhaustive]
        pub enum $name {
            $(#[$keyword_meta])*
            $keyword,
            /// A non-negative length, percentage, or math expression.
            LengthPercentage(NonNegativeLengthPercentage),
            /// The `min-content` keyword.
            MinContent,
            /// The `max-content` keyword.
            MaxContent,
            /// The bare `fit-content` keyword.
            FitContent,
            /// The `fit-content()` function with a non-negative argument.
            FitContentFunction(NonNegativeLengthPercentage),
            /// The `stretch` keyword.
            Stretch,
            /// The `contain` keyword.
            Contain,
        }

        impl $name {
            /// Create the `fit-content()` function.
            #[must_use]
            pub fn fit_content(value: impl Into<NonNegativeLengthPercentage>) -> Self {
                Self::FitContentFunction(value.into())
            }
        }

        length_percentage_sources!(@non_negative $name, |value| Self::LengthPercentage(value.into()));

        impl ToCss for $name {
            fn to_css<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
                match self {
                    Self::$keyword => w.write_str($keyword_css),
                    Self::LengthPercentage(value) => value.to_css(w),
                    Self::MinContent => w.write_str("min-content"),
                    Self::MaxContent => w.write_str("max-content"),
                    Self::FitContent => w.write_str("fit-content"),
                    Self::FitContentFunction(value) => {
                        w.write_str("fit-content(")?;
                        value.to_css(w)?;
                        w.write_char(')')
                    }
                    Self::Stretch => w.write_str("stretch"),
                    Self::Contain => w.write_str("contain"),
                }
            }
        }
    };
}

size_grammar!(
    /// A value for the preferred and minimum size properties, such as `width` and `min-height`.
    ///
    /// [CSS Sizing Level 4 sizing values](https://www.w3.org/TR/css-sizing-4/#sizing-values)
    Size,
    /// The `auto` keyword.
    Auto => "auto"
);

size_grammar!(
    /// A value for the maximum size properties, such as `max-width`.
    ///
    /// [CSS Sizing Level 4 sizing values](https://www.w3.org/TR/css-sizing-4/#sizing-values)
    MaxSize,
    /// The `none` keyword.
    None => "none"
);

keyword_enum! {
    /// Which box the size properties, such as `width`, describe.
    ///
    /// [CSS Sizing Level 3 `box-sizing`](https://www.w3.org/TR/css-sizing-3/#box-sizing)
    pub enum BoxSizing {
        /// The size describes the content box, without padding and border.
        ContentBox => "content-box" as content_box,
        /// The size describes the border box, including padding and border.
        BorderBox => "border-box" as border_box,
    }
}

/// An `aspect-ratio` value, which is `auto`, a ratio of width to height, or both.
///
/// With a ratio alone, the box has that ratio. With both, an element that has a natural ratio,
/// such as an image, keeps its natural ratio, and other boxes use the given ratio.
///
/// ```rust
/// use typed_css::AspectRatio;
///
/// assert_eq!(AspectRatio::new(16, 9).to_string(), "16 / 9");
/// assert_eq!(AspectRatio::new(1, 1).to_string(), "1");
/// assert_eq!(AspectRatio::new(4, 3).or_natural().to_string(), "auto 4 / 3");
/// ```
///
/// [CSS Sizing Level 4 `aspect-ratio`](https://www.w3.org/TR/css-sizing-4/#aspect-ratio)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AspectRatio {
    natural: bool,
    ratio: Option<(NonNegativeNumber, NonNegativeNumber)>,
}

impl AspectRatio {
    /// The `auto` keyword. Only an element with a natural ratio, such as an image, keeps a
    /// ratio.
    #[must_use]
    pub const fn auto() -> Self {
        Self {
            natural: true,
            ratio: None,
        }
    }

    /// Create the ratio `width / height`.
    ///
    /// # Panics
    ///
    /// Panics if `width` or `height` is negative or not finite. Use [`AspectRatio::try_new`] for
    /// runtime-derived values.
    #[track_caller]
    #[must_use]
    pub fn new(width: impl Into<f64>, height: impl Into<f64>) -> Self {
        Self::try_new(width, height).unwrap_or_else(|error| panic!("{error}"))
    }

    /// Attempt to create the ratio `width / height`.
    ///
    /// # Errors
    ///
    /// Returns an error if `width` or `height` is negative or not finite.
    pub fn try_new(width: impl Into<f64>, height: impl Into<f64>) -> Result<Self, InvalidNumber> {
        Ok(Self {
            natural: false,
            ratio: Some((
                NonNegativeNumber::try_for(width.into(), "aspect ratio width")?,
                NonNegativeNumber::try_for(height.into(), "aspect ratio height")?,
            )),
        })
    }

    /// Prefer the natural ratio of an element, such as an image, and use this ratio only for
    /// boxes without one. This is written as `auto` before the ratio.
    #[must_use]
    pub const fn or_natural(mut self) -> Self {
        self.natural = true;
        self
    }
}

impl ToCss for AspectRatio {
    #[expect(
        clippy::float_cmp,
        reason = "only a height of exactly 1 may be left out of the ratio"
    )]
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        let Some((width, height)) = self.ratio else {
            return dest.write_str("auto");
        };
        if self.natural {
            dest.write_str("auto ")?;
        }
        width.to_css(dest)?;
        if height.get() != 1.0 {
            dest.write_str(" / ")?;
            height.to_css(dest)?;
        }
        Ok(())
    }
}

/// A single number is the ratio `number / 1`, so `aspect-ratio: 1.5` is `AspectRatio::new(1.5, 1)`.
impl From<NonNegativeNumber> for AspectRatio {
    fn from(ratio: NonNegativeNumber) -> Self {
        Self {
            natural: false,
            ratio: Some((ratio, NonNegativeNumber::from(1))),
        }
    }
}

impl From<u32> for AspectRatio {
    fn from(ratio: u32) -> Self {
        Self::from(NonNegativeNumber::from(ratio))
    }
}

declaration_sources!(AspectRatio: NonNegativeNumber, u32);

auto_sources!(Size => Self::Auto, AspectRatio => Self::auto());

impl_display!(Size, MaxSize, AspectRatio);
impl_checked_value!(Size, MaxSize, BoxSizing, AspectRatio);

property_keywords! {
    Size {
        auto: "auto" => Size::Auto,
        min_content: "min-content" => Size::MinContent,
        max_content: "max-content" => Size::MaxContent,
        fit_content: "fit-content" => Size::FitContent,
        stretch: "stretch" => Size::Stretch,
        contain: "contain" => Size::Contain,
    }
}

property_keywords! {
    MaxSize {
        none: "none" => MaxSize::None,
        min_content: "min-content" => MaxSize::MinContent,
        max_content: "max-content" => MaxSize::MaxContent,
        fit_content: "fit-content" => MaxSize::FitContent,
        stretch: "stretch" => MaxSize::Stretch,
        contain: "contain" => MaxSize::Contain,
    }
}

property_keywords! {
    AspectRatio {
        auto: "auto" => AspectRatio::auto(),
    }
}
