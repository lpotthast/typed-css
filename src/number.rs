use std::cmp::Ordering;
use std::fmt;

use crate::checked_value::{impl_checked_value, impl_display};
use crate::custom_property::declaration_sources;
use crate::serialization::ToCss;

/// Error returned when a number is not a valid CSS value.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum InvalidNumber {
    /// The number is `NaN` or infinite.
    NonFinite {
        /// The rejected number.
        value: f64,
        /// What the number was meant to be, such as `px` or `hsl hue`.
        context: &'static str,
    },
    /// The number is outside the allowed range.
    OutOfRange {
        /// The rejected number.
        value: f64,
        /// What the number was meant to be, such as `px` or `hsl hue`.
        context: &'static str,
        /// The smallest allowed number.
        min: f64,
        /// The largest allowed number.
        max: f64,
    },
    /// The number is negative, but the value must not be.
    Negative {
        /// The rejected number.
        value: f64,
        /// What the number was meant to be, such as `px` or `hsl hue`.
        context: &'static str,
    },
}

impl fmt::Display for InvalidNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFinite { value, context } => {
                write!(f, "CSS {context} value must be finite, got {value}")
            }
            Self::OutOfRange {
                value,
                context,
                min,
                max,
            } => write!(
                f,
                "CSS {context} must be in range [{min}, {max}], got {value}"
            ),
            Self::Negative { value, context } => {
                write!(f, "CSS {context} must be non-negative, got {value}")
            }
        }
    }
}

impl std::error::Error for InvalidNumber {}

/// A CSS `<number>`, which is a finite `f64`.
///
/// `NaN` and infinity cannot be represented, because CSS has no syntax for them.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Number(f64);

pub(crate) const fn try_finite(value: f64, context: &'static str) -> Result<Number, InvalidNumber> {
    if value.is_finite() {
        Ok(Number(value))
    } else {
        Err(InvalidNumber::NonFinite { value, context })
    }
}

impl Number {
    /// Create a number.
    ///
    /// # Panics
    ///
    /// Panics, or fails const evaluation, if `value` is `NaN` or infinite. Use
    /// [`Number::try_new`] for runtime-derived values.
    #[track_caller]
    #[must_use]
    pub const fn new(value: f64) -> Self {
        assert!(value.is_finite(), "CSS number must be finite");
        Self(value)
    }

    /// Attempt to create a number.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidNumber::NonFinite`] if `value` is `NaN` or infinite.
    pub const fn try_new(value: f64) -> Result<Self, InvalidNumber> {
        try_finite(value, "number")
    }

    /// Return the number as an `f64`.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.0
    }
}

impl TryFrom<f64> for Number {
    type Error = InvalidNumber;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

impl From<Number> for f64 {
    fn from(value: Number) -> Self {
        value.value()
    }
}

/// `2^53`. Every integer of a smaller magnitude has an exact `f64`.
const EXACT_INTEGER_LIMIT: f64 = 9_007_199_254_740_992.0;

/// The most digits after the decimal point that [`short_decimal`] finds.
const MAX_FRACTION_DIGITS: u32 = 8;

/// `2^50`. A number whose scaled value is below this limit has a short decimal that
/// [`short_decimal`] can find.
const SCALED_LIMIT: f64 = 1_125_899_906_842_624.0;

impl ToCss for Number {
    /// Write the number as `f64`'s `Display` does, which is the shortest decimal that reads back
    /// as the same `f64`. `Display` never uses an exponent. Unlike `Display`, `-0.0` is written as
    /// `0`.
    ///
    /// Most CSS numbers are whole numbers or have a few digits after the decimal point, such as
    /// `16` or `1.5`. Integer arithmetic writes those about twice as fast as the shortest-decimal
    /// algorithm of `Display`. Other numbers use `Display`.
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        let value = self.0;
        if value == 0.0 {
            // `-0.0` is written as `0`, because it compares equal to `0.0`. `Display` would keep
            // the sign.
            return dest.write_str("0");
        }
        let magnitude = value.abs();
        let decimal = if value.fract() == 0.0 && magnitude < EXACT_INTEGER_LIMIT {
            #[expect(
                clippy::cast_possible_truncation,
                clippy::cast_sign_loss,
                reason = "The magnitude is a whole number below 2^53, so the u64 holds it exactly."
            )]
            Some((magnitude as u64, 0))
        } else {
            short_decimal(magnitude)
        };
        let Some((digits, fraction_digits)) = decimal else {
            return write!(dest, "{value}");
        };
        if value < 0.0 {
            dest.write_char('-')?;
        }
        let scale = 10_u64.pow(fraction_digits);
        write_digits(dest, digits / scale, 1)?;
        if fraction_digits > 0 {
            dest.write_char('.')?;
            // The fraction keeps its leading zeros, such as the `05` in `1.05`.
            write_digits(dest, digits % scale, fraction_digits)?;
        }
        Ok(())
    }
}

/// Return the shortest decimal with at most [`MAX_FRACTION_DIGITS`] digits after the decimal
/// point that reads back as `magnitude`. The decimal is returned as its digits and the number of
/// digits after the decimal point, so `1.25` is `(125, 2)`.
///
/// The scaled value is below [`SCALED_LIMIT`], so the gap between `magnitude` and its neighboring
/// `f64` values is smaller than the step between two decimals with `MAX_FRACTION_DIGITS` digits.
/// So at most one such decimal reads back as `magnitude`, and the rounded scaled value is that
/// decimal. A shorter decimal that reads back is the same decimal without its trailing zeros. The
/// division rounds correctly, so it reads the decimal back as a parser does.
fn short_decimal(magnitude: f64) -> Option<(u64, u32)> {
    let scale = POWERS_OF_TEN[MAX_FRACTION_DIGITS as usize];
    let scaled = magnitude * scale;
    if scaled >= SCALED_LIMIT {
        return None;
    }
    // `scaled` is below 2^50, so adding `0.5` is exact, and the cast rounds to the nearest
    // integer. The cast avoids `f64::round`, which is a library call on WASM.
    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "The scaled value is positive and below 2^50, so the u64 holds it."
    )]
    let mut digits = (scaled + 0.5) as u64;
    #[expect(
        clippy::cast_precision_loss,
        reason = "The digits are below 2^50, so the f64 holds them exactly."
    )]
    let decimal = digits as f64 / scale;
    #[expect(
        clippy::float_cmp,
        reason = "Only a decimal that reads back as exactly the same f64 may replace it."
    )]
    if decimal != magnitude {
        return None;
    }
    let mut fraction_digits = MAX_FRACTION_DIGITS;
    while fraction_digits > 0 && digits.is_multiple_of(10) {
        digits /= 10;
        fraction_digits -= 1;
    }
    Some((digits, fraction_digits))
}

/// Write `value` in decimal digits, with leading zeros up to `width` digits.
///
/// Writing the digits directly is faster than `write!`, which goes through a `Formatter`.
fn write_digits<W: fmt::Write>(dest: &mut W, mut value: u64, width: u32) -> fmt::Result {
    // `u64::MAX` has 20 digits.
    let mut text = [b'0'; 20];
    let mut start = text.len();
    while value > 0 || text.len() - start < width as usize {
        start -= 1;
        text[start] = b'0' + (value % 10) as u8;
        value /= 10;
    }
    // The text consists of ASCII digits, so it is valid UTF-8.
    dest.write_str(std::str::from_utf8(&text[start..]).unwrap_or_default())
}

/// Write an integer in decimal digits, as its `Display` does.
pub(crate) fn write_integer<W: fmt::Write>(dest: &mut W, value: impl Into<i64>) -> fmt::Result {
    let value = value.into();
    if value < 0 {
        dest.write_char('-')?;
    }
    write_digits(dest, value.unsigned_abs(), 1)
}

impl std::ops::Neg for Number {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

/// A CSS `<number [0,∞]>`, for properties such as `flex-grow`.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NonNegativeNumber(Number);

impl NonNegativeNumber {
    /// Create a non-negative number.
    ///
    /// # Panics
    ///
    /// Panics, or fails const evaluation, if `value` is negative, `NaN`, or infinite. Use
    /// [`NonNegativeNumber::try_new`] for runtime-derived values.
    #[track_caller]
    #[must_use]
    pub const fn new(value: f64) -> Self {
        match Self::try_new(value) {
            Ok(number) => number,
            Err(_) => panic!("CSS number must be finite and non-negative"),
        }
    }

    /// Attempt to create a non-negative number.
    ///
    /// # Errors
    ///
    /// Returns an error if `value` is negative, `NaN`, or infinite.
    pub const fn try_new(value: f64) -> Result<Self, InvalidNumber> {
        Self::try_for(value, "number")
    }

    /// Attempt to create a non-negative number. An error names the number as `context`.
    pub(crate) const fn try_for(value: f64, context: &'static str) -> Result<Self, InvalidNumber> {
        match try_non_negative(value, context) {
            Ok(number) => Ok(Self(number)),
            Err(error) => Err(error),
        }
    }

    /// Return the number as an `f64`.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.0.value()
    }
}

/// An unsigned integer converts exactly, so `FLEX_GROW.declare(1)` works.
impl From<u32> for NonNegativeNumber {
    fn from(value: u32) -> Self {
        Self(Number(f64::from(value)))
    }
}

declaration_sources!(NonNegativeNumber: u32);

impl TryFrom<f64> for NonNegativeNumber {
    type Error = InvalidNumber;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

impl ToCss for NonNegativeNumber {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        self.0.to_css(dest)
    }
}

/// A number from `0` to `1`, for values such as an alpha channel.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UnitInterval(Number);

impl UnitInterval {
    /// Create a number from `0` to `1`.
    ///
    /// # Panics
    ///
    /// Panics, or fails const evaluation, if `value` is outside the range or not finite. Use
    /// [`UnitInterval::try_new`] for runtime-derived values.
    #[track_caller]
    #[must_use]
    pub const fn new(value: f64) -> Self {
        match Self::try_new(value) {
            Ok(number) => number,
            Err(_) => panic!("CSS unit interval must be finite and in range [0, 1]"),
        }
    }

    /// Attempt to create a number from `0` to `1`.
    ///
    /// # Errors
    ///
    /// Returns an error if `value` is outside the range or not finite.
    pub const fn try_new(value: f64) -> Result<Self, InvalidNumber> {
        match try_in_range(value, "unit interval", 0.0, 1.0) {
            Ok(number) => Ok(Self(number)),
            Err(error) => Err(error),
        }
    }

    /// Return the number as an `f64`.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.0.value()
    }
}

impl TryFrom<f64> for UnitInterval {
    type Error = InvalidNumber;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

impl ToCss for UnitInterval {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        self.0.to_css(dest)
    }
}

/// A percentage from `0%` to `100%`, for values such as the saturation of an HSL color.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PercentageChannel(pub(crate) Number);

impl PercentageChannel {
    /// Create a percentage from `0%` to `100%`. `PercentageChannel::new(50.0)` is `50%`.
    ///
    /// # Panics
    ///
    /// Panics, or fails const evaluation, if `value` is outside the range or not finite. Use
    /// [`PercentageChannel::try_new`] for runtime-derived values.
    #[track_caller]
    #[must_use]
    pub const fn new(value: f64) -> Self {
        match Self::try_new(value) {
            Ok(percentage) => percentage,
            Err(_) => panic!("CSS percentage channel must be finite and in range [0, 100]"),
        }
    }

    /// Attempt to create a percentage from `0%` to `100%`.
    ///
    /// # Errors
    ///
    /// Returns an error if `value` is outside the range or not finite.
    pub const fn try_new(value: f64) -> Result<Self, InvalidNumber> {
        match try_in_range(value, "percentage channel", 0.0, 100.0) {
            Ok(number) => Ok(Self(number)),
            Err(error) => Err(error),
        }
    }

    /// Return the number of percent.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.0.value()
    }
}

impl TryFrom<f64> for PercentageChannel {
    type Error = InvalidNumber;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

impl ToCss for PercentageChannel {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        self.0.to_css(dest)?;
        dest.write_char('%')
    }
}

/// An `i32` is a CSS `<integer>`, for properties such as `order`.
impl ToCss for i32 {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        write_integer(dest, *self)
    }
}

/// Exact powers of ten. `1e22` is the largest power of ten that an `f64` holds exactly.
const POWERS_OF_TEN: [f64; 23] = [
    1e0, 1e1, 1e2, 1e3, 1e4, 1e5, 1e6, 1e7, 1e8, 1e9, 1e10, 1e11, 1e12, 1e13, 1e14, 1e15, 1e16,
    1e17, 1e18, 1e19, 1e20, 1e21, 1e22,
];

/// Round `value` to 15 significant digits.
///
/// Arithmetic on `f64` leaves noise in the last digits, so `0.1 + 0.2` is `0.30000000000000004`.
/// A decimal with at most 15 significant digits survives the conversion to `f64` and back. So
/// rounding to 15 digits removes the noise and keeps such a decimal unchanged.
///
/// The function scales `value` by an exact power of ten and rounds the scaled value to an integer.
/// FMA gives the exact error of the scaling, so the result is the same as the result of
/// `format!("{value:.14e}").parse()`. The function avoids that round trip through text, because
/// formatting and parsing are slow and add the float parser to every WASM binary. Only powers of
/// ten up to `1e22` are exact, so a value below about `1e-8` or above about `1e36` stays
/// unchanged. CSS has no use for values of that size.
pub(crate) fn round_noise(value: f64) -> f64 {
    if value == 0.0 || !value.is_finite() {
        return value;
    }
    let magnitude = value.abs();
    #[expect(
        clippy::cast_possible_truncation,
        reason = "The decimal exponent of a finite f64 is between -324 and 308."
    )]
    let mut exponent = magnitude.log10().floor() as i32;
    loop {
        let shift = 14 - exponent;
        let Some(&scale) = usize::try_from(shift.unsigned_abs())
            .ok()
            .and_then(|index| POWERS_OF_TEN.get(index))
        else {
            return value;
        };
        // `scaled + error` is the exact result, because `scale` is exact and FMA rounds once.
        let (scaled, error) = if shift >= 0 {
            let scaled = magnitude * scale;
            (scaled, magnitude.mul_add(scale, -scaled))
        } else {
            let scaled = magnitude / scale;
            (scaled, -scaled.mul_add(scale, -magnitude))
        };
        let mut digits = scaled.floor();
        let fraction = scaled - digits;
        let rounds_up = match fraction.total_cmp(&0.5) {
            Ordering::Less => false,
            Ordering::Greater => true,
            // A zero `error` means an exact tie, which rounds to an even digit like `format!`.
            Ordering::Equal => error > 0.0 || (error >= 0.0 && digits % 2.0 > 0.5),
        };
        if rounds_up {
            digits += 1.0;
        }
        // `log10` can be off by one next to a power of ten, so the loop corrects the exponent.
        if digits >= 1e15 {
            exponent += 1;
        } else if digits < 1e14 {
            exponent -= 1;
        } else {
            let rounded = if shift >= 0 {
                digits / scale
            } else {
                digits * scale
            };
            return rounded.copysign(value);
        }
    }
}

#[inline]
#[track_caller]
pub(crate) fn assert_finite(value: f64, context: &'static str) -> Number {
    try_finite(value, context).unwrap_or_else(|error| panic!("{error}"))
}

#[inline]
pub(crate) const fn try_in_range(
    value: f64,
    context: &'static str,
    min: f64,
    max: f64,
) -> Result<Number, InvalidNumber> {
    let number = match try_finite(value, context) {
        Ok(number) => number,
        Err(error) => return Err(error),
    };
    if min <= value && value <= max {
        Ok(number)
    } else {
        Err(InvalidNumber::OutOfRange {
            value,
            context,
            min,
            max,
        })
    }
}

#[inline]
pub(crate) const fn try_non_negative(
    value: f64,
    context: &'static str,
) -> Result<Number, InvalidNumber> {
    let number = match try_finite(value, context) {
        Ok(number) => number,
        Err(error) => return Err(error),
    };
    if value >= 0.0 {
        Ok(number)
    } else {
        Err(InvalidNumber::Negative { value, context })
    }
}

/// Define a dimension type, which is a number with one unit out of a fixed set.
///
/// The macro defines the type with private fields, a private enum of its units, a const
/// constructor and a fallible `try_` constructor for each unit, and the operators that keep the
/// unit. The documentation of each unit becomes part of the documentation of its constructor.
macro_rules! dimension {
    (
        $(#[$meta:meta])*
        pub struct $ty:ident($unit:ident), $noun:literal;
        $($(#[$unit_meta:meta])* $variant:ident => $name:ident, $try_name:ident, $css:literal;)+
    ) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub struct $ty {
            number: $crate::Number,
            unit: $unit,
        }

        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        enum $unit {
            $($variant),+
        }

        impl $ty {
            $(
                #[doc = concat!("Create ", $noun, " in `", $css, "`.")]
                ///
                $(#[$unit_meta])*
                ///
                /// # Panics
                ///
                #[doc = concat!(
                    "Panics, or fails const evaluation, if `value` is `NaN` or infinite. Use [`",
                    stringify!($ty), "::", stringify!($try_name), "`] for runtime-derived values."
                )]
                #[track_caller]
                #[must_use]
                pub const fn $name(value: f64) -> Self {
                    match Self::$try_name(value) {
                        Ok(value) => value,
                        Err(_) => panic!(concat!("CSS ", $css, " value must be finite")),
                    }
                }

                #[doc = concat!("Attempt to create ", $noun, " in `", $css, "`.")]
                ///
                /// # Errors
                ///
                /// Returns [`InvalidNumber::NonFinite`](crate::InvalidNumber::NonFinite) if
                /// `value` is `NaN` or infinite.
                pub const fn $try_name(value: f64) -> Result<Self, $crate::InvalidNumber> {
                    match $crate::number::try_finite(value, $css) {
                        Ok(number) => Ok(Self {
                            number,
                            unit: $unit::$variant,
                        }),
                        Err(error) => Err(error),
                    }
                }
            )+

            /// Return the number without its unit.
            #[must_use]
            pub const fn value(self) -> f64 {
                self.number.value()
            }

            /// Return the unit as CSS text, such as `px` or `deg`.
            #[must_use]
            pub const fn unit(self) -> &'static str {
                match self.unit {
                    $($unit::$variant => $css),+
                }
            }

            /// Attempt to transform the number while keeping the unit.
            ///
            /// # Errors
            ///
            /// Returns [`InvalidNumber::NonFinite`](crate::InvalidNumber::NonFinite) if `f`
            /// returns `NaN` or infinity.
            pub fn try_map_value(
                self,
                f: impl FnOnce(f64) -> f64,
            ) -> Result<Self, $crate::InvalidNumber> {
                Ok(Self {
                    number: $crate::number::try_finite(f(self.value()), self.unit())?,
                    unit: self.unit,
                })
            }

            /// Transform the number while keeping the unit.
            ///
            /// # Panics
            ///
            #[doc = concat!(
                "Panics if `f` returns `NaN` or infinity. Use [`", stringify!($ty),
                "::try_map_value`] when the result can be invalid at runtime."
            )]
            #[must_use]
            #[track_caller]
            pub fn map_value(self, f: impl FnOnce(f64) -> f64) -> Self {
                self.try_map_value(f)
                    .unwrap_or_else(|error| panic!("{error}"))
            }
        }

        impl $crate::serialization::ToCss for $ty {
            fn to_css<W: ::std::fmt::Write>(&self, dest: &mut W) -> ::std::fmt::Result {
                $crate::serialization::ToCss::to_css(&self.number, dest)?;
                dest.write_str(self.unit())
            }
        }

        $crate::number::scaling_operators!($ty);
        $crate::checked_value::impl_display!($ty);
        $crate::checked_value::impl_checked_value!($ty);
    };
}

pub(crate) use dimension;

/// Implement multiplication and division by a number, and negation, for types with `map_value`.
/// Each operator keeps the unit.
macro_rules! scaling_operators {
    ($($ty:ty),+) => {$(
        impl<F: Into<f64>> std::ops::Mul<F> for $ty {
            type Output = Self;

            /// Multiply by a number, keeping the unit.
            ///
            /// # Panics
            ///
            /// Panics if `factor` is `NaN` or infinite, or if the result is not finite.
            #[track_caller]
            fn mul(self, factor: F) -> Self {
                let factor = $crate::number::assert_finite(factor.into(), "factor").value();
                self.map_value(|number| $crate::number::round_noise(number * factor))
            }
        }

        impl<F: Into<f64>> std::ops::Div<F> for $ty {
            type Output = Self;

            /// Divide by a number, keeping the unit.
            ///
            /// # Panics
            ///
            /// Panics if `divisor` is zero, `NaN`, or infinite, or if the result is not finite.
            #[track_caller]
            fn div(self, divisor: F) -> Self {
                let divisor = $crate::number::assert_finite(divisor.into(), "divisor").value();
                assert!(divisor != 0.0, "CSS divisor must not be zero");
                self.map_value(|number| $crate::number::round_noise(number / divisor))
            }
        }

        impl std::ops::Neg for $ty {
            type Output = Self;

            fn neg(self) -> Self {
                self.map_value(|number| -number)
            }
        }
    )+};
}

pub(crate) use scaling_operators;

/// Define a free function for each unit constructor of a dimension type, such as `px` for
/// `Length::px`. The free function takes any number that converts into an `f64`.
macro_rules! unit_functions {
    ($ty:ident; $($(#[$meta:meta])* $name:ident, $try_name:ident;)+) => {$(
        $(#[$meta])*
        ///
        /// # Panics
        ///
        #[doc = concat!(
            "Panics if `value` is `NaN` or infinite. Use [`", stringify!($ty), "::",
            stringify!($try_name), "`] for runtime-derived values."
        )]
        #[track_caller]
        #[must_use]
        pub fn $name(value: impl Into<f64>) -> $ty {
            $ty::$name(value.into())
        }
    )+};
}

pub(crate) use unit_functions;

impl_display!(Number, NonNegativeNumber, UnitInterval, PercentageChannel);
impl_checked_value!(Number, NonNegativeNumber, UnitInterval, i32);

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::{EXACT_INTEGER_LIMIT, Number, POWERS_OF_TEN, round_noise};
    use crate::ToCss;

    /// Round `value` to 15 significant digits through text. `round_noise` must match it.
    fn round_through_text(value: f64) -> f64 {
        format!("{value:.14e}").parse().unwrap_or(value)
    }

    /// Return a pseudo-random sequence of bit patterns, so that the test is reproducible.
    fn random_bits(count: usize) -> impl Iterator<Item = u64> {
        let mut state: u64 = 0x2545_F491_4F6C_DD1D;
        std::iter::repeat_with(move || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        })
        .take(count)
    }

    /// Return `value` and the next `f64` below and above it.
    fn with_neighbors(value: f64) -> [f64; 3] {
        let bits = value.to_bits();
        [f64::from_bits(bits - 1), value, f64::from_bits(bits + 1)]
    }

    fn mismatches(values: impl IntoIterator<Item = f64>) -> Vec<(f64, f64, f64)> {
        values
            .into_iter()
            .filter(|value| (1e-7..=1e35).contains(&value.abs()))
            .map(|value| (value, round_noise(value), round_through_text(value)))
            .filter(|(_, actual, expected)| actual.to_bits() != expected.to_bits())
            .collect()
    }

    #[test]
    fn rounding_removes_noise() {
        assert_that!(round_noise(0.1 + 0.2)).is_equal_to(0.3);
        assert_that!(round_noise(0.07 * 100.0)).is_equal_to(7.0);
        assert_that!(round_noise(-(1.1 - 1.0))).is_equal_to(-0.1);
        assert_that!(round_noise(1.0 / 3.0)).is_equal_to(0.333_333_333_333_333);
    }

    #[test]
    fn rounding_matches_rounding_through_text_for_random_values() {
        let values = random_bits(200_000)
            .map(f64::from_bits)
            .chain(random_bits(200_000).map(|bits| {
                // The low 52 bits pick the mantissa, and the exponent covers the supported range.
                let exponent = 1023 - 27 + (bits >> 52) % 147;
                f64::from_bits(exponent << 52 | bits & ((1 << 52) - 1))
            }));
        assert_that!(mismatches(values)).is_empty();
    }

    #[test]
    fn rounding_matches_rounding_through_text_next_to_ties() {
        // A 16-digit integer that ends in 5 lies exactly between two 15-digit decimals.
        let ties = random_bits(20_000).flat_map(|bits| {
            #[expect(
                clippy::cast_precision_loss,
                reason = "The integer is below 2^53, so the f64 holds the integer exactly."
            )]
            let tie = ((bits % 800_000_000_000_000 + 100_000_000_000_000) * 10 + 5) as f64;
            [tie, tie * 1e-12, tie * 1e12, tie * 1e-20, tie * 1e20]
                .into_iter()
                .flat_map(with_neighbors)
        });
        assert_that!(mismatches(ties)).is_empty();
    }

    #[test]
    fn rounding_keeps_special_values() {
        for value in [0.0, -0.0, 5e-324, 1e-9, -1e-12, 1e38, f64::MAX, f64::MIN] {
            assert_that!(round_noise(value).to_bits()).is_equal_to(value.to_bits());
        }
        assert_that!(round_noise(f64::INFINITY)).is_equal_to(f64::INFINITY);
        assert_that!(round_noise(f64::NAN).is_nan()).is_true();
    }

    #[test]
    fn rounding_keeps_the_sign() {
        for value in [0.1 + 0.2, 1.0 / 3.0, 123.456, 1e-7, 1e35] {
            assert_that!(round_noise(-value)).is_equal_to(-round_noise(value));
        }
    }

    #[test]
    fn numbers_are_written_like_display() {
        let limit = EXACT_INTEGER_LIMIT;
        let edge_cases = [
            0.0,
            -0.0,
            1.0,
            -1.0,
            0.5,
            -0.5,
            16.0,
            1e15,
            1e21,
            limit - 1.0,
            limit,
            -limit,
            limit + 2.0,
            1e300,
            5e-324,
            f64::MAX,
            f64::MIN,
        ];
        #[expect(
            clippy::cast_precision_loss,
            reason = "The integers are below 2^21, so the f64 holds each one exactly."
        )]
        let integers = random_bits(100_000).map(|bits| (bits % 2_000_001) as f64 - 1e6);
        // Decimals with up to 10 digits after the decimal point, and the `f64` values next to
        // them. Dividing an exact integer by an exact power of ten reads the decimal back.
        #[expect(
            clippy::cast_precision_loss,
            reason = "The digits are below 2^37, so the f64 holds them exactly."
        )]
        let decimals = random_bits(100_000)
            .map(|bits| {
                ((bits >> 8) % 100_000_000_000) as f64 / POWERS_OF_TEN[(bits % 11) as usize]
            })
            .flat_map(with_neighbors);
        // The gap below a power of two is half the gap above it.
        let powers_of_two = (-40..40)
            .map(|exponent| 2_f64.powi(exponent))
            .flat_map(|power| (1..200).map(move |factor| power * f64::from(factor)))
            .flat_map(with_neighbors);
        let mismatches: Vec<(f64, String)> = edge_cases
            .into_iter()
            .chain(integers)
            .chain(decimals)
            .chain(powers_of_two)
            .flat_map(|value| [value, -value])
            .chain(random_bits(100_000).map(f64::from_bits))
            .filter(|value| value.is_finite())
            .map(|value| (value, Number::new(value).to_css_string()))
            // `Display` writes `-0.0` as `-0`, and `Number` writes `0`.
            .filter(|(value, css)| *css != (*value + 0.0).to_string())
            .collect();
        assert_that!(mismatches).is_empty();
    }

    #[test]
    fn negative_zero_is_written_as_zero() {
        assert_that!(Number::new(-0.0).to_css_string()).is_equal_to("0");
        assert_that!((-crate::px(0)).to_css_string()).is_equal_to("0px");
        assert_that!((crate::px(-3) * std::hint::black_box(0)).to_css_string()).is_equal_to("0px");
    }
}
