use std::{fmt, sync::Arc};

use crate::auto::auto_sources;
use crate::checked_value::{impl_checked_value, impl_display, keyword_enum};
use crate::custom_property::declaration_sources;
use crate::property::definition::property_keywords;
use crate::{
    DeclarationValue, Length, Number, Percentage, Var,
    number::{assert_finite, round_noise},
    serialization::ToCss,
};

/// A length, a percentage, or a calculation that combines them.
///
/// A [`Length`] is a distance with a unit, such as `16px` or `2rem`. A [`Percentage`] is a share
/// of some reference size, such as `50%` of the containing block's width. Most size properties,
/// such as `width`, `padding`, and `margin`, accept both. They also accept a value that the browser
/// computes from both, such as `calc(100% - 20px)`, because the browser only knows the reference
/// size of a percentage when it lays out the page. CSS calls this type `<length-percentage>`, and
/// `LengthPercentage` models it.
///
/// You rarely need to name this type. A `Length` or a `Percentage` converts into it, and so does a
/// `var()` reference to a custom property of either type. Adding or subtracting lengths and
/// percentages creates a `LengthPercentage`, and so do [`min`], [`max`], [`clamp`], and [`env`](fn@env).
/// When both operands use the same unit, the sum is folded into one value.
///
/// ```rust
/// use typed_css::{min, pct, property::WIDTH, px, rem};
///
/// assert_eq!((px(10) + px(5)).to_string(), "15px");
/// assert_eq!((pct(100) - px(20)).to_string(), "calc(100% - 20px)");
/// assert_eq!(min(pct(100), rem(40)).to_string(), "min(100%, 40rem)");
///
/// // A property accepts each of these forms directly.
/// assert_eq!(WIDTH.declare(px(120)).value(), "120px");
/// assert_eq!(WIDTH.declare(pct(100) - px(20)).value(), "calc(100% - 20px)");
/// ```
///
/// [CSS Values Level 4 `<length-percentage>`](https://www.w3.org/TR/css-values-4/#mixed-percentages)
#[derive(Clone, Debug, PartialEq)]
pub struct LengthPercentage(Node);

/// One node of the expression tree. The operands are shared, so that cloning an expression only
/// counts references. Shorthands such as `Padding::all` clone their value for every component.
#[derive(Clone, Debug, PartialEq)]
enum Node {
    Length(Length),
    Percentage(Percentage),
    Variable(Var<LengthPercentage>),
    Sum(Arc<LengthPercentage>, SumOperator, Arc<LengthPercentage>),
    Product(Arc<LengthPercentage>, ProductOperator, Number),
    Min(Arc<LengthPercentage>, Arc<LengthPercentage>),
    Max(Arc<LengthPercentage>, Arc<LengthPercentage>),
    Clamp {
        min: Arc<LengthPercentage>,
        preferred: Arc<LengthPercentage>,
        max: Arc<LengthPercentage>,
    },
    Env(EnvironmentVariable),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SumOperator {
    Add,
    Subtract,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ProductOperator {
    Multiply,
    Divide,
}

impl ProductOperator {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Multiply => " * ",
            Self::Divide => " / ",
        }
    }

    fn apply(self, value: f64, factor: f64) -> f64 {
        match self {
            Self::Multiply => value * factor,
            Self::Divide => value / factor,
        }
    }
}

/// How strongly a node binds inside `calc()`. A node is parenthesized when its context needs a
/// stronger binding than the node has.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Binding {
    Sum,
    Product,
    Atom,
}

impl SumOperator {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Add => " + ",
            Self::Subtract => " - ",
        }
    }

    fn apply(self, lhs: f64, rhs: f64) -> f64 {
        match self {
            Self::Add => lhs + rhs,
            Self::Subtract => lhs - rhs,
        }
    }
}

keyword_enum! {
    /// A length that the browser provides about the device, read with [`env`](fn@env).
    ///
    /// Each variable describes the safe area of the viewport. The safe area is the part of the
    /// screen that a device's rounded corners, camera notch, or system bars do not cover. Each
    /// variable is the distance from one edge of the viewport to the edge of the safe area. On a
    /// device without such obstructions, every distance is `0px`.
    ///
    /// ```rust
    /// use typed_css::{EnvironmentVariable, env, property::PADDING_BOTTOM, rem};
    ///
    /// // Keep a toolbar's content above the home indicator of a phone.
    /// let inset = env(EnvironmentVariable::SafeAreaInsetBottom);
    /// let padding = PADDING_BOTTOM.declare(inset + rem(1));
    /// assert_eq!(padding.value(), "calc(env(safe-area-inset-bottom) + 1rem)");
    /// ```
    ///
    /// [CSS Environment Variables Level 1](https://www.w3.org/TR/css-env-1/#safe-area-insets)
    pub enum EnvironmentVariable {
        /// The distance from the top edge of the viewport to the safe area.
        SafeAreaInsetTop => "safe-area-inset-top",
        /// The distance from the right edge of the viewport to the safe area.
        SafeAreaInsetRight => "safe-area-inset-right",
        /// The distance from the bottom edge of the viewport to the safe area.
        SafeAreaInsetBottom => "safe-area-inset-bottom",
        /// The distance from the left edge of the viewport to the safe area.
        SafeAreaInsetLeft => "safe-area-inset-left",
        /// The largest value that `safe-area-inset-top` can take. The current value can be
        /// smaller, for example while browser UI covers part of the screen.
        SafeAreaMaxInsetTop => "safe-area-max-inset-top",
        /// The largest value that `safe-area-inset-right` can take.
        SafeAreaMaxInsetRight => "safe-area-max-inset-right",
        /// The largest value that `safe-area-inset-bottom` can take.
        SafeAreaMaxInsetBottom => "safe-area-max-inset-bottom",
        /// The largest value that `safe-area-inset-left` can take.
        SafeAreaMaxInsetLeft => "safe-area-max-inset-left",
    }
}

impl LengthPercentage {
    fn sum(lhs: Self, operator: SumOperator, rhs: Self) -> Self {
        let folded = match (&lhs.0, &rhs.0) {
            (Node::Length(a), Node::Length(b)) if a.unit() == b.unit() => a
                .try_map_value(|value| round_noise(operator.apply(value, b.value())))
                .ok()
                .map(Node::Length),
            (Node::Percentage(a), Node::Percentage(b)) => {
                Percentage::try_new(round_noise(operator.apply(a.value(), b.value())))
                    .ok()
                    .map(Node::Percentage)
            }
            _ => None,
        };
        // A sum that overflows to infinity stays an expression instead of failing.
        Self(folded.unwrap_or_else(|| Node::Sum(Arc::new(lhs), operator, Arc::new(rhs))))
    }

    fn product(value: Self, operator: ProductOperator, factor: Number) -> Self {
        let apply = |number: f64| round_noise(operator.apply(number, factor.value()));
        let folded = match &value.0 {
            Node::Length(length) => length.try_map_value(apply).ok().map(Node::Length),
            Node::Percentage(percentage) => Percentage::try_new(apply(percentage.value()))
                .ok()
                .map(Node::Percentage),
            _ => None,
        };
        Self(folded.unwrap_or_else(|| Node::Product(Arc::new(value), operator, factor)))
    }

    const fn binding(&self) -> Binding {
        match &self.0 {
            Node::Sum(..) => Binding::Sum,
            Node::Product(..) => Binding::Product,
            _ => Binding::Atom,
        }
    }

    /// Write this node as an operand inside `calc()`. The node is parenthesized if `context` needs
    /// a stronger binding than the node has.
    fn write_operand<W: fmt::Write>(&self, dest: &mut W, context: Binding) -> fmt::Result {
        let parenthesize = self.binding() < context;
        if parenthesize {
            dest.write_char('(')?;
        }
        match &self.0 {
            // Sums and products associate to the left, so only a right operand of the same kind
            // needs parentheses.
            Node::Sum(lhs, operator, rhs) => {
                lhs.write_operand(dest, Binding::Sum)?;
                dest.write_str(operator.as_str())?;
                rhs.write_operand(dest, Binding::Product)?;
            }
            Node::Product(value, operator, factor) => {
                value.write_operand(dest, Binding::Product)?;
                dest.write_str(operator.as_str())?;
                factor.to_css(dest)?;
            }
            _ => self.to_css(dest)?,
        }
        if parenthesize {
            dest.write_char(')')?;
        }
        Ok(())
    }

    fn write_function<W: fmt::Write>(dest: &mut W, name: &str, arguments: &[&Self]) -> fmt::Result {
        dest.write_str(name)?;
        dest.write_char('(')?;
        for (index, argument) in arguments.iter().enumerate() {
            if index > 0 {
                dest.write_str(", ")?;
            }
            argument.write_operand(dest, Binding::Sum)?;
        }
        dest.write_char(')')
    }

    /// Serialize for a grammar that accepts only non-negative values.
    ///
    /// A directly negative value is written inside `calc()`. CSS clamps the result of a top-level
    /// math function to the property's range, so the declaration stays valid where a plain
    /// negative value would make it invalid. Calculations and math functions are clamped the same
    /// way already. A `var()` and an `env()` are written as they are, because `calc(var(--x))`
    /// would become invalid if `--x` is a unitless `0`.
    ///
    /// [CSS math-function range checking](https://www.w3.org/TR/css-values-4/#calc-range)
    pub(crate) fn to_css_non_negative<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        let is_negative = match &self.0 {
            Node::Length(length) => length.value() < 0.0,
            Node::Percentage(percentage) => percentage.value() < 0.0,
            Node::Variable(variable) => {
                return variable.write_literals_with(dest, &|fallback, dest| {
                    fallback.to_css_non_negative(dest)
                });
            }
            _ => false,
        };
        if is_negative {
            dest.write_str("calc(")?;
            self.to_css(dest)?;
            dest.write_char(')')
        } else {
            self.to_css(dest)
        }
    }
}

impl ToCss for LengthPercentage {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        match &self.0 {
            Node::Length(length) => length.to_css(dest),
            Node::Percentage(percentage) => percentage.to_css(dest),
            Node::Variable(variable) => variable.to_css(dest),
            Node::Sum(..) | Node::Product(..) => {
                dest.write_str("calc(")?;
                self.write_operand(dest, Binding::Sum)?;
                dest.write_char(')')
            }
            Node::Min(lhs, rhs) => Self::write_function(dest, "min", &[lhs, rhs]),
            Node::Max(lhs, rhs) => Self::write_function(dest, "max", &[lhs, rhs]),
            Node::Clamp {
                min,
                preferred,
                max,
            } => Self::write_function(dest, "clamp", &[min, preferred, max]),
            Node::Env(variable) => {
                dest.write_str("env(")?;
                variable.to_css(dest)?;
                dest.write_char(')')
            }
        }
    }
}

impl From<Length> for LengthPercentage {
    fn from(length: Length) -> Self {
        Self(Node::Length(length))
    }
}

impl From<Percentage> for LengthPercentage {
    fn from(percentage: Percentage) -> Self {
        Self(Node::Percentage(percentage))
    }
}

impl From<Var<LengthPercentage>> for LengthPercentage {
    fn from(variable: Var<LengthPercentage>) -> Self {
        Self(Node::Variable(variable))
    }
}

impl From<Var<Length>> for LengthPercentage {
    fn from(variable: Var<Length>) -> Self {
        Self(Node::Variable(variable.widen()))
    }
}

impl From<Var<Percentage>> for LengthPercentage {
    fn from(variable: Var<Percentage>) -> Self {
        Self(Node::Variable(variable.widen()))
    }
}

declaration_sources!(LengthPercentage: Length, Percentage);

/// Implement `+` and `-` with a length-percentage, which create a `LengthPercentage`.
macro_rules! sum_operators {
    ($($lhs:ty),+) => {$(
        impl<R: Into<LengthPercentage>> std::ops::Add<R> for $lhs {
            type Output = LengthPercentage;

            /// Add two length-percentages. A sum of the same unit folds into one value.
            /// Other sums become a `calc()` expression.
            fn add(self, rhs: R) -> LengthPercentage {
                LengthPercentage::sum(self.into(), SumOperator::Add, rhs.into())
            }
        }

        impl<R: Into<LengthPercentage>> std::ops::Sub<R> for $lhs {
            type Output = LengthPercentage;

            /// Subtract two length-percentages. A difference of the same unit folds into one
            /// value. Other differences become a `calc()` expression.
            fn sub(self, rhs: R) -> LengthPercentage {
                LengthPercentage::sum(self.into(), SumOperator::Subtract, rhs.into())
            }
        }
    )+};
}

/// Implement every operator of a `LengthPercentage`, including multiplication and division by a
/// number and negation.
macro_rules! arithmetic {
    ($($lhs:ty),+) => {$(
        sum_operators!($lhs);

        impl<F: Into<f64>> std::ops::Mul<F> for $lhs {
            type Output = LengthPercentage;

            /// Multiply a length-percentage by a number. A length or percentage folds into one
            /// value. Other values become a `calc()` expression.
            ///
            /// # Panics
            ///
            /// Panics if `factor` is `NaN` or infinite.
            #[track_caller]
            fn mul(self, factor: F) -> LengthPercentage {
                let factor = assert_finite(factor.into(), "factor");
                LengthPercentage::product(self.into(), ProductOperator::Multiply, factor)
            }
        }

        impl<F: Into<f64>> std::ops::Div<F> for $lhs {
            type Output = LengthPercentage;

            /// Divide a length-percentage by a number. A length or percentage folds into one
            /// value. Other values become a `calc()` expression.
            ///
            /// # Panics
            ///
            /// Panics if `divisor` is zero, `NaN`, or infinite.
            #[track_caller]
            fn div(self, divisor: F) -> LengthPercentage {
                let divisor = assert_finite(divisor.into(), "divisor");
                assert!(divisor.value() != 0.0, "CSS divisor must not be zero");
                LengthPercentage::product(self.into(), ProductOperator::Divide, divisor)
            }
        }

        impl std::ops::Neg for $lhs {
            type Output = LengthPercentage;

            /// Negate a length-percentage. A length or percentage folds into one value. Other
            /// values become `calc(value * -1)`.
            fn neg(self) -> LengthPercentage {
                LengthPercentage::product(self.into(), ProductOperator::Multiply, Number::new(-1.0))
            }
        }
    )+};
}

arithmetic!(
    LengthPercentage,
    Var<LengthPercentage>,
    Var<Length>,
    Var<Percentage>
);

sum_operators!(Length, Percentage);

/// Create the CSS `min()` of two length-percentages.
#[must_use]
pub fn min(a: impl Into<LengthPercentage>, b: impl Into<LengthPercentage>) -> LengthPercentage {
    LengthPercentage(Node::Min(Arc::new(a.into()), Arc::new(b.into())))
}

/// Create the CSS `max()` of two length-percentages.
#[must_use]
pub fn max(a: impl Into<LengthPercentage>, b: impl Into<LengthPercentage>) -> LengthPercentage {
    LengthPercentage(Node::Max(Arc::new(a.into()), Arc::new(b.into())))
}

/// Create the CSS `clamp(min, preferred, max)`, which resolves to `preferred` limited to the
/// range from `min` to `max`.
///
/// ```rust
/// use typed_css::{clamp, rem, vw};
///
/// assert_eq!(clamp(rem(1), vw(2.5), rem(2)).to_string(), "clamp(1rem, 2.5vw, 2rem)");
/// ```
#[must_use]
pub fn clamp(
    min: impl Into<LengthPercentage>,
    preferred: impl Into<LengthPercentage>,
    max: impl Into<LengthPercentage>,
) -> LengthPercentage {
    LengthPercentage(Node::Clamp {
        min: Arc::new(min.into()),
        preferred: Arc::new(preferred.into()),
        max: Arc::new(max.into()),
    })
}

/// Read a length that the browser provides about the device, with CSS `env()`.
///
/// See [`EnvironmentVariable`] for the available lengths.
#[must_use]
pub fn env(variable: EnvironmentVariable) -> LengthPercentage {
    LengthPercentage(Node::Env(variable))
}

/// Implement conversions from every length-percentage source into a type.
///
/// The sources are [`Length`], [`Percentage`], [`LengthPercentage`], and typed `var()` references
/// of the three. For a checked value type, each source also converts into the type's
/// [`DeclarationValue`](crate::DeclarationValue), so that a property accepts the source directly.
/// The `@conversions_only` form is for parts of a grammar that are not checked values. The
/// `@non_negative` forms also convert a [`NonNegativeLengthPercentage`](crate::NonNegativeLengthPercentage)
/// and its `var()` references.
macro_rules! length_percentage_sources {
    (@conversions_only $target:ty, |$value:ident| $convert:expr) => {
        $crate::length_percentage::length_percentage_sources!(
            @each $target, |$value| $convert;
            $crate::Length,
            $crate::Percentage,
            $crate::LengthPercentage,
            $crate::Var<$crate::LengthPercentage>,
            $crate::Var<$crate::Length>,
            $crate::Var<$crate::Percentage>
        );
    };
    (@each $target:ty, |$value:ident| $convert:expr; $($source:ty),+) => {$(
        impl From<$source> for $target {
            fn from(source: $source) -> Self {
                let $value = $crate::LengthPercentage::from(source);
                $convert
            }
        }
    )+};
    // A non-negative grammar also accepts a `NonNegativeLengthPercentage`. Other grammars do not,
    // because a value written for a non-negative grammar may rely on the clamping of `calc()`.
    (@non_negative_conversions_only $target:ty, |$value:ident| $convert:expr) => {
        $crate::length_percentage::length_percentage_sources!(
            @conversions_only $target, |$value| $convert
        );

        impl From<$crate::NonNegativeLengthPercentage> for $target {
            fn from(source: $crate::NonNegativeLengthPercentage) -> Self {
                // The non-negative grammar clamps the value again when it writes it.
                let $value = source.into_length_percentage();
                $convert
            }
        }

        impl From<$crate::Var<$crate::NonNegativeLengthPercentage>> for $target {
            fn from(source: $crate::Var<$crate::NonNegativeLengthPercentage>) -> Self {
                let $value =
                    $crate::NonNegativeLengthPercentage::from(source).into_length_percentage();
                $convert
            }
        }
    };
    (@non_negative $target:ty, |$value:ident| $convert:expr) => {
        $crate::length_percentage::length_percentage_sources!(
            @non_negative_conversions_only $target, |$value| $convert
        );
        $crate::custom_property::declaration_sources!(
            $target: $crate::Length,
            $crate::Percentage,
            $crate::LengthPercentage,
            $crate::NonNegativeLengthPercentage
        );
    };
    // A `var()` reaches the declaration value through the generic conversion of `Var`.
    ($target:ty, |$value:ident| $convert:expr) => {
        $crate::length_percentage::length_percentage_sources!(
            @conversions_only $target, |$value| $convert
        );
        $crate::custom_property::declaration_sources!(
            $target: $crate::Length, $crate::Percentage, $crate::LengthPercentage
        );
    };
}

pub(crate) use length_percentage_sources;

/// A `<length-percentage [0,∞]>` for grammars such as `padding`, `gap`, and `width`.
///
/// Every length-percentage converts into this type. A negative length or percentage is written
/// inside `calc()`, so that CSS clamps it to zero instead of rejecting the declaration. This also
/// applies to the fallback of a `var()`. A calculation is clamped the same way. A `var()` and an
/// `env()` are written as they are, because `calc(var(--x))` becomes invalid when `--x` is a
/// unitless `0`.
///
/// ```rust
/// use typed_css::{CustomProperty, LengthPercentage, NonNegativeLengthPercentage, px, var_or};
///
/// let space = CustomProperty::<LengthPercentage>::new("--space");
/// assert_eq!(NonNegativeLengthPercentage::from(px(4)).to_string(), "4px");
/// assert_eq!(NonNegativeLengthPercentage::from(px(-4)).to_string(), "calc(-4px)");
/// assert_eq!(
///     NonNegativeLengthPercentage::from(var_or(&space, px(-4))).to_string(),
///     "var(--space, calc(-4px))",
/// );
/// ```
///
/// [CSS Values Level 4 range checking](https://www.w3.org/TR/css-values-4/#calc-range)
#[derive(Clone, Debug, PartialEq)]
pub struct NonNegativeLengthPercentage(LengthPercentage);

impl NonNegativeLengthPercentage {
    /// Return the value without the range restriction.
    #[must_use]
    pub(crate) fn as_length_percentage(&self) -> &LengthPercentage {
        &self.0
    }

    pub(crate) fn into_length_percentage(self) -> LengthPercentage {
        self.0
    }
}

length_percentage_sources!(NonNegativeLengthPercentage, |value| Self(value));

/// A declaration value converts back into a length-percentage, so that a value or a `var()` can
/// take part in a calculation.
impl From<DeclarationValue<LengthPercentage>> for LengthPercentage {
    fn from(value: DeclarationValue<LengthPercentage>) -> Self {
        value.into_literal().unwrap_or_else(Self::from)
    }
}

impl From<DeclarationValue<NonNegativeLengthPercentage>> for NonNegativeLengthPercentage {
    fn from(value: DeclarationValue<NonNegativeLengthPercentage>) -> Self {
        value.into_literal().unwrap_or_else(Self::from)
    }
}

impl LengthPercentage {
    /// Return whether the value is a `var()` reference, without a calculation around it.
    pub(crate) const fn is_variable(&self) -> bool {
        matches!(self.0, Node::Variable(_))
    }
}

/// A `var()` of a non-negative custom property converts into this type, but not into
/// [`LengthPercentage`], because grammars such as `margin` do not clamp it.
impl From<Var<NonNegativeLengthPercentage>> for NonNegativeLengthPercentage {
    fn from(variable: Var<NonNegativeLengthPercentage>) -> Self {
        // The non-negative grammar clamps the fallback again when it writes it.
        Self(LengthPercentage(Node::Variable(
            variable.map_literals(&Self::into_length_percentage),
        )))
    }
}

impl ToCss for NonNegativeLengthPercentage {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        self.0.to_css_non_negative(dest)
    }
}

/// A `<length-percentage> | auto` for grammars such as `margin` and `top`.
///
/// [CSS Box Model Level 4 margin grammar](https://www.w3.org/TR/css-box-4/#margin-physical)
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum LengthPercentageAuto {
    /// A length, a percentage, or a math expression. Negative values are allowed.
    LengthPercentage(LengthPercentage),
    /// The `auto` keyword.
    Auto,
}

length_percentage_sources!(LengthPercentageAuto, |value| Self::LengthPercentage(value));

impl ToCss for LengthPercentageAuto {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        match self {
            Self::LengthPercentage(value) => value.to_css(dest),
            Self::Auto => dest.write_str("auto"),
        }
    }
}

auto_sources!(LengthPercentageAuto => Self::Auto);

impl_display!(
    LengthPercentage,
    NonNegativeLengthPercentage,
    LengthPercentageAuto
);
impl_checked_value!(
    LengthPercentage,
    NonNegativeLengthPercentage,
    LengthPercentageAuto
);

property_keywords! {
    LengthPercentageAuto {
        auto: "auto" => crate::Auto,
    }
}
