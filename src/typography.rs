use std::fmt;

use crate::checked_value::{impl_checked_value, impl_display};
use crate::custom_property::declaration_sources;
use crate::length_percentage::length_percentage_sources;
use crate::number::try_in_range;
use crate::property::definition::property_keywords;
use crate::serialization::ToCss;
use crate::{Angle, InvalidNumber, NonNegativeLengthPercentage, NonNegativeNumber, Number};

/// A `font-weight` value.
///
/// ```rust
/// use typed_css::FontWeight;
///
/// assert_eq!(FontWeight::Bold.to_string(), "bold");
/// assert_eq!(FontWeight::new(450).to_string(), "450");
/// ```
///
/// [CSS Fonts Level 4 `font-weight` grammar](https://www.w3.org/TR/css-fonts-4/#font-weight-prop)
#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub enum FontWeight {
    /// `normal`, the same as weight 400.
    Normal,
    /// `bold`, the same as weight 700.
    Bold,
    /// `bolder`, one step bolder than the parent's weight.
    Bolder,
    /// `lighter`, one step lighter than the parent's weight.
    Lighter,
    /// A numeric weight from 1 to 1000.
    Number(FontWeightNumber),
}

impl FontWeight {
    /// Create a numeric weight.
    ///
    /// # Panics
    ///
    /// Panics if `weight` is outside `1..=1000` or not finite. Use [`FontWeight::try_new`] for
    /// runtime-derived values.
    #[track_caller]
    #[must_use]
    pub fn new(weight: impl Into<f64>) -> Self {
        Self::Number(FontWeightNumber::new(weight))
    }

    /// Attempt to create a numeric weight.
    ///
    /// # Errors
    ///
    /// Returns an error if `weight` is outside `1..=1000` or not finite.
    pub fn try_new(weight: impl Into<f64>) -> Result<Self, InvalidNumber> {
        FontWeightNumber::try_new(weight).map(Self::Number)
    }
}

impl From<FontWeightNumber> for FontWeight {
    fn from(value: FontWeightNumber) -> Self {
        Self::Number(value)
    }
}

declaration_sources!(FontWeight: FontWeightNumber);

impl ToCss for FontWeight {
    fn to_css<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match self {
            Self::Normal => w.write_str("normal"),
            Self::Bold => w.write_str("bold"),
            Self::Bolder => w.write_str("bolder"),
            Self::Lighter => w.write_str("lighter"),
            Self::Number(weight) => weight.to_css(w),
        }
    }
}

/// A numeric font weight from 1 to 1000.
///
/// [CSS Fonts Level 4 `font-weight` grammar](https://www.w3.org/TR/css-fonts-4/#font-weight-prop)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FontWeightNumber(Number);

impl FontWeightNumber {
    /// Create a numeric weight.
    ///
    /// # Panics
    ///
    /// Panics if `weight` is outside `1..=1000` or not finite.
    #[track_caller]
    #[must_use]
    pub fn new(weight: impl Into<f64>) -> Self {
        Self::try_new(weight).unwrap_or_else(|error| panic!("{error}"))
    }

    /// Attempt to create a numeric weight.
    ///
    /// # Errors
    ///
    /// Returns an error if `weight` is outside `1..=1000` or not finite.
    pub fn try_new(weight: impl Into<f64>) -> Result<Self, InvalidNumber> {
        try_in_range(weight.into(), "font weight", 1.0, 1000.0).map(Self)
    }

    /// Return the numeric weight.
    #[must_use]
    pub const fn get(self) -> f64 {
        self.0.get()
    }
}

impl ToCss for FontWeightNumber {
    fn to_css<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        self.0.to_css(w)
    }
}

/// A `font-style` value.
///
/// [CSS Fonts Level 4 `font-style` grammar](https://www.w3.org/TR/css-fonts-4/#font-style-prop)
#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub enum FontStyle {
    /// `normal`, the upright face.
    Normal,
    /// `italic`, the italic face. The browser slants the upright face if the font has no italic.
    Italic,
    /// `oblique` with an optional slant. Without an angle, the slant is `14deg`.
    Oblique(Option<ObliqueAngle>),
}

impl FontStyle {
    /// Create `oblique` with a slant.
    ///
    /// # Panics
    ///
    /// Panics if `angle` is outside `-90deg..=90deg`. Use [`ObliqueAngle::try_new`] for
    /// runtime-derived angles.
    #[track_caller]
    #[must_use]
    pub fn oblique(angle: Angle) -> Self {
        Self::Oblique(Some(ObliqueAngle::new(angle)))
    }
}

impl ToCss for FontStyle {
    fn to_css<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match self {
            Self::Normal => w.write_str("normal"),
            Self::Italic => w.write_str("italic"),
            Self::Oblique(None) => w.write_str("oblique"),
            Self::Oblique(Some(angle)) => {
                w.write_str("oblique ")?;
                angle.to_css(w)
            }
        }
    }
}

/// The slant of an oblique [`FontStyle`], from `-90deg` to `90deg`.
///
/// ```rust
/// use typed_css::{FontStyle, ObliqueAngle, deg};
///
/// assert_eq!(FontStyle::oblique(deg(10)).to_string(), "oblique 10deg");
/// assert!(ObliqueAngle::try_new(deg(100)).is_err());
/// ```
///
/// [CSS Fonts Level 4 `font-style` grammar](https://www.w3.org/TR/css-fonts-4/#font-style-prop)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ObliqueAngle(Angle);

impl ObliqueAngle {
    /// The smallest and largest slant in degrees.
    const RANGE: (f64, f64) = (-90.0, 90.0);

    /// Create an oblique slant.
    ///
    /// # Panics
    ///
    /// Panics if `angle` lies outside `-90deg..=90deg`.
    #[track_caller]
    #[must_use]
    pub fn new(angle: Angle) -> Self {
        match Self::try_new(angle) {
            Ok(angle) => angle,
            Err(error) => panic!("{error}"),
        }
    }

    /// Attempt to create an oblique slant.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidNumber::OutOfRange`] with the angle in degrees if `angle` lies outside
    /// `-90deg..=90deg`.
    pub fn try_new(angle: Angle) -> Result<Self, InvalidNumber> {
        let degrees = angle.degrees();
        let (min, max) = Self::RANGE;
        if (min..=max).contains(&degrees) {
            Ok(Self(angle))
        } else {
            Err(InvalidNumber::OutOfRange {
                value: degrees,
                context: "oblique deg",
                min,
                max,
            })
        }
    }
}

impl ToCss for ObliqueAngle {
    fn to_css<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        self.0.to_css(w)
    }
}

/// A `font-size` value.
///
/// [CSS Fonts Level 4 `font-size` grammar](https://www.w3.org/TR/css-fonts-4/#font-size-prop)
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum FontSize {
    /// `xx-small`.
    XxSmall,
    /// `x-small`.
    XSmall,
    /// `small`.
    Small,
    /// `medium`.
    Medium,
    /// `large`.
    Large,
    /// `x-large`.
    XLarge,
    /// `xx-large`.
    XxLarge,
    /// `xxx-large`.
    XxxLarge,
    /// `larger`, relative to the parent's font size.
    Larger,
    /// `smaller`, relative to the parent's font size.
    Smaller,
    /// `math`, which shrinks the font size of nested math formulas.
    Math,
    /// A non-negative length, percentage, or math expression.
    LengthPercentage(NonNegativeLengthPercentage),
}

length_percentage_sources!(@non_negative FontSize, |value| Self::LengthPercentage(value.into()));

impl ToCss for FontSize {
    fn to_css<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        w.write_str(match self {
            Self::XxSmall => "xx-small",
            Self::XSmall => "x-small",
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
            Self::XLarge => "x-large",
            Self::XxLarge => "xx-large",
            Self::XxxLarge => "xxx-large",
            Self::Larger => "larger",
            Self::Smaller => "smaller",
            Self::Math => "math",
            Self::LengthPercentage(value) => return value.to_css(w),
        })
    }
}

/// A `line-height` value.
///
/// ```rust
/// use typed_css::{LineHeight, px};
///
/// assert_eq!(LineHeight::number(1.5).to_string(), "1.5");
/// assert_eq!(LineHeight::from(px(24)).to_string(), "24px");
/// ```
///
/// [CSS Inline Layout Level 3 `line-height` grammar](https://www.w3.org/TR/css-inline-3/#line-height-property)
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum LineHeight {
    /// The `normal` keyword.
    Normal,
    /// A non-negative multiple of the element's font size.
    Number(NonNegativeNumber),
    /// A non-negative length, percentage, or math expression.
    LengthPercentage(NonNegativeLengthPercentage),
}

impl LineHeight {
    /// Create a line height as a multiple of the font size.
    ///
    /// # Panics
    ///
    /// Panics if `factor` is negative or not finite. Use [`LineHeight::try_number`] for
    /// runtime-derived values.
    #[track_caller]
    #[must_use]
    pub fn number(factor: impl Into<f64>) -> Self {
        Self::try_number(factor).unwrap_or_else(|error| panic!("{error}"))
    }

    /// Attempt to create a line height as a multiple of the font size.
    ///
    /// # Errors
    ///
    /// Returns an error if `factor` is negative or not finite.
    pub fn try_number(factor: impl Into<f64>) -> Result<Self, InvalidNumber> {
        NonNegativeNumber::try_new(factor).map(Self::Number)
    }
}

impl From<NonNegativeNumber> for LineHeight {
    fn from(factor: NonNegativeNumber) -> Self {
        Self::Number(factor)
    }
}

impl From<u32> for LineHeight {
    fn from(factor: u32) -> Self {
        Self::Number(NonNegativeNumber::from(factor))
    }
}

declaration_sources!(LineHeight: NonNegativeNumber, u32);
length_percentage_sources!(@non_negative LineHeight, |value| Self::LengthPercentage(value.into()));

impl ToCss for LineHeight {
    fn to_css<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match self {
            Self::Normal => w.write_str("normal"),
            Self::Number(factor) => factor.to_css(w),
            Self::LengthPercentage(value) => value.to_css(w),
        }
    }
}

impl_display!(
    FontWeight,
    FontWeightNumber,
    FontStyle,
    ObliqueAngle,
    FontSize,
    LineHeight
);
impl_checked_value!(FontWeight, FontStyle, FontSize, LineHeight);

property_keywords! {
    FontSize {
        xx_small: "xx-small" => FontSize::XxSmall,
        x_small: "x-small" => FontSize::XSmall,
        small: "small" => FontSize::Small,
        medium: "medium" => FontSize::Medium,
        large: "large" => FontSize::Large,
        x_large: "x-large" => FontSize::XLarge,
        xx_large: "xx-large" => FontSize::XxLarge,
        xxx_large: "xxx-large" => FontSize::XxxLarge,
        larger: "larger" => FontSize::Larger,
        smaller: "smaller" => FontSize::Smaller,
        math: "math" => FontSize::Math,
    }
}

property_keywords! {
    FontStyle {
        normal: "normal" => FontStyle::Normal,
        italic: "italic" => FontStyle::Italic,
        oblique: "oblique" => FontStyle::Oblique(None),
    }
}

property_keywords! {
    FontWeight {
        normal: "normal" => FontWeight::Normal,
        bold: "bold" => FontWeight::Bold,
        bolder: "bolder" => FontWeight::Bolder,
        lighter: "lighter" => FontWeight::Lighter,
    }
}

property_keywords! {
    LineHeight {
        normal: "normal" => LineHeight::Normal,
    }
}
