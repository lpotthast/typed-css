use std::fmt;

use crate::serialization::{write_list, write_separated, write_spaced};

use crate::checked_value::{impl_checked_value, impl_display};
use crate::custom_property::declaration_sources;
use crate::length_percentage::length_percentage_sources;
use crate::property::definition::property_keywords;
use crate::{
    Angle, InvalidNumber, Length, LengthPercentage, Number,
    number::{assert_finite, try_non_negative},
    serialization::ToCss,
};

/// One CSS transform function, such as `rotate(45deg)`.
///
/// The constructors, such as [`TransformFunction::rotate`], create the functions and check their
/// arguments. A length-percentage argument, such as the `x` of `translate()`, can also be a
/// `var()` reference:
///
/// ```rust
/// use typed_css::{Length, TransformFunction, custom_property, px, var};
///
/// custom_property!(SHIFT: Length = "--shift");
/// let shift = TransformFunction::translate(var(&SHIFT), px(0));
/// assert_eq!(shift.to_string(), "translate(var(--shift), 0px)");
/// ```
///
/// [CSS Transforms Level 1 functions](https://www.w3.org/TR/css-transforms-1/#transform-functions)
/// and [Level 2 3D functions](https://www.w3.org/TR/css-transforms-2/#transform-functions)
#[derive(Clone, Debug, PartialEq)]
pub struct TransformFunction(Function);

#[derive(Clone, Debug, PartialEq)]
enum Function {
    /// `translate(x, y)`.
    Translate(LengthPercentage, LengthPercentage),
    /// `translateX(x)`.
    TranslateX(LengthPercentage),
    /// `translateY(y)`.
    TranslateY(LengthPercentage),
    /// `translateZ(z)`.
    TranslateZ(Length),
    /// `translate3d(x, y, z)`.
    Translate3d(LengthPercentage, LengthPercentage, Length),
    /// `scale(x, y)`.
    Scale(Number, Number),
    /// `scaleX(x)`.
    ScaleX(Number),
    /// `scaleY(y)`.
    ScaleY(Number),
    /// `scaleZ(z)`.
    ScaleZ(Number),
    /// `scale3d(x, y, z)`.
    Scale3d(Number, Number, Number),
    /// `rotate(angle)`.
    Rotate(Angle),
    /// `rotateX(angle)`.
    RotateX(Angle),
    /// `rotateY(angle)`.
    RotateY(Angle),
    /// `rotateZ(angle)`.
    RotateZ(Angle),
    /// `rotate3d(x, y, z, angle)`.
    Rotate3d(Number, Number, Number, Angle),
    /// `skew(x, y)`.
    Skew(Angle, Angle),
    /// `skewX(angle)`.
    SkewX(Angle),
    /// `skewY(angle)`.
    SkewY(Angle),
    /// `matrix(a, b, c, d, e, f)`.
    Matrix([Number; 6]),
    /// `matrix3d(a1, b1, c1, d1, a2, …, d4)`, a 4×4 matrix in column-major order.
    Matrix3d([Number; 16]),
    /// `perspective(d)`, which makes 3D transforms after it appear in perspective, as seen from
    /// the distance `d`.
    Perspective(Length),
}

/// Convert a factor into a [`Number`], panicking with a clear message if it is not finite.
#[track_caller]
fn factor(value: impl Into<f64>) -> Number {
    assert_finite(value.into(), "transform factor")
}

impl TransformFunction {
    /// Create `translate(x, y)`.
    #[must_use]
    pub fn translate(x: impl Into<LengthPercentage>, y: impl Into<LengthPercentage>) -> Self {
        Self(Function::Translate(x.into(), y.into()))
    }

    /// Create `translateX(x)`.
    #[must_use]
    pub fn translate_x(x: impl Into<LengthPercentage>) -> Self {
        Self(Function::TranslateX(x.into()))
    }

    /// Create `translateY(y)`.
    #[must_use]
    pub fn translate_y(y: impl Into<LengthPercentage>) -> Self {
        Self(Function::TranslateY(y.into()))
    }

    /// Create `translateZ(z)`.
    #[must_use]
    pub fn translate_z(z: Length) -> Self {
        Self(Function::TranslateZ(z))
    }

    /// Create `translate3d(x, y, z)`.
    #[must_use]
    pub fn translate_3d(
        x: impl Into<LengthPercentage>,
        y: impl Into<LengthPercentage>,
        z: Length,
    ) -> Self {
        Self(Function::Translate3d(x.into(), y.into(), z))
    }

    /// Create `scale(x, y)`. A factor of `1` keeps the size.
    ///
    /// # Panics
    ///
    /// Panics if a factor is not finite.
    #[track_caller]
    #[must_use]
    pub fn scale(x: impl Into<f64>, y: impl Into<f64>) -> Self {
        Self(Function::Scale(factor(x), factor(y)))
    }

    /// Create `scaleX(x)`.
    ///
    /// # Panics
    ///
    /// Panics if `x` is not finite.
    #[track_caller]
    #[must_use]
    pub fn scale_x(x: impl Into<f64>) -> Self {
        Self(Function::ScaleX(factor(x)))
    }

    /// Create `scaleY(y)`.
    ///
    /// # Panics
    ///
    /// Panics if `y` is not finite.
    #[track_caller]
    #[must_use]
    pub fn scale_y(y: impl Into<f64>) -> Self {
        Self(Function::ScaleY(factor(y)))
    }

    /// Create `scaleZ(z)`.
    ///
    /// # Panics
    ///
    /// Panics if `z` is not finite.
    #[track_caller]
    #[must_use]
    pub fn scale_z(z: impl Into<f64>) -> Self {
        Self(Function::ScaleZ(factor(z)))
    }

    /// Create `scale3d(x, y, z)`.
    ///
    /// # Panics
    ///
    /// Panics if a factor is not finite.
    #[track_caller]
    #[must_use]
    pub fn scale_3d(x: impl Into<f64>, y: impl Into<f64>, z: impl Into<f64>) -> Self {
        Self(Function::Scale3d(factor(x), factor(y), factor(z)))
    }

    /// Create `rotate(angle)`, a rotation in the plane of the screen.
    #[must_use]
    pub fn rotate(angle: Angle) -> Self {
        Self(Function::Rotate(angle))
    }

    /// Create `rotateX(angle)`.
    #[must_use]
    pub fn rotate_x(angle: Angle) -> Self {
        Self(Function::RotateX(angle))
    }

    /// Create `rotateY(angle)`.
    #[must_use]
    pub fn rotate_y(angle: Angle) -> Self {
        Self(Function::RotateY(angle))
    }

    /// Create `rotateZ(angle)`.
    #[must_use]
    pub fn rotate_z(angle: Angle) -> Self {
        Self(Function::RotateZ(angle))
    }

    /// Create `rotate3d(x, y, z, angle)`, a rotation around the axis from the origin to
    /// `(x, y, z)`.
    ///
    /// # Panics
    ///
    /// Panics if a coordinate is not finite.
    #[track_caller]
    #[must_use]
    pub fn rotate_3d(
        x: impl Into<f64>,
        y: impl Into<f64>,
        z: impl Into<f64>,
        angle: Angle,
    ) -> Self {
        Self(Function::Rotate3d(factor(x), factor(y), factor(z), angle))
    }

    /// Create `skew(x, y)`.
    #[must_use]
    pub fn skew(x: Angle, y: Angle) -> Self {
        Self(Function::Skew(x, y))
    }

    /// Create `skewX(angle)`.
    #[must_use]
    pub fn skew_x(angle: Angle) -> Self {
        Self(Function::SkewX(angle))
    }

    /// Create `skewY(angle)`.
    #[must_use]
    pub fn skew_y(angle: Angle) -> Self {
        Self(Function::SkewY(angle))
    }

    /// Create `matrix(a, b, c, d, e, f)`, the 2D matrix with the columns `(a, b)`, `(c, d)`, and
    /// the translation `(e, f)` in pixels.
    ///
    /// # Panics
    ///
    /// Panics if a number is not finite.
    #[track_caller]
    #[must_use]
    pub fn matrix(values: [f64; 6]) -> Self {
        Self(Function::Matrix(values.map(factor)))
    }

    /// Create `matrix3d(..)` from a 4×4 matrix in column-major order. The last column holds the
    /// translation in pixels.
    ///
    /// # Panics
    ///
    /// Panics if a number is not finite.
    #[track_caller]
    #[must_use]
    pub fn matrix_3d(values: [f64; 16]) -> Self {
        Self(Function::Matrix3d(values.map(factor)))
    }

    /// Create `perspective(distance)`. A smaller distance gives a stronger perspective.
    ///
    /// # Panics
    ///
    /// Panics if `distance` is negative. Use [`TransformFunction::try_perspective`] for
    /// runtime-derived values.
    #[track_caller]
    #[must_use]
    pub fn perspective(distance: Length) -> Self {
        Self::try_perspective(distance).unwrap_or_else(|error| panic!("{error}"))
    }

    /// Attempt to create `perspective(distance)`.
    ///
    /// # Errors
    ///
    /// Returns an error if `distance` is negative.
    pub fn try_perspective(distance: Length) -> Result<Self, InvalidNumber> {
        try_non_negative(distance.value(), "perspective distance")?;
        Ok(Self(Function::Perspective(distance)))
    }
}

/// Write a transform function with its arguments separated by commas, such as
/// `translate(10px, 20px)`.
macro_rules! write_function {
    ($w:expr, $name:literal; $($argument:expr),+) => {{
        $w.write_str(concat!($name, "("))?;
        write_list!($w, ", "; $($argument),+)?;
        $w.write_char(')')
    }};
}

impl ToCss for TransformFunction {
    #[expect(
        clippy::many_single_char_names,
        reason = "the names follow the arguments of the CSS functions"
    )]
    fn to_css<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match &self.0 {
            Function::Translate(x, y) => write_function!(w, "translate"; x, y),
            Function::TranslateX(x) => write_function!(w, "translateX"; x),
            Function::TranslateY(y) => write_function!(w, "translateY"; y),
            Function::TranslateZ(z) => write_function!(w, "translateZ"; z),
            Function::Translate3d(x, y, z) => write_function!(w, "translate3d"; x, y, z),
            Function::Scale(x, y) => write_function!(w, "scale"; x, y),
            Function::ScaleX(x) => write_function!(w, "scaleX"; x),
            Function::ScaleY(y) => write_function!(w, "scaleY"; y),
            Function::ScaleZ(z) => write_function!(w, "scaleZ"; z),
            Function::Scale3d(x, y, z) => write_function!(w, "scale3d"; x, y, z),
            Function::Rotate(angle) => write_function!(w, "rotate"; angle),
            Function::RotateX(angle) => write_function!(w, "rotateX"; angle),
            Function::RotateY(angle) => write_function!(w, "rotateY"; angle),
            Function::RotateZ(angle) => write_function!(w, "rotateZ"; angle),
            Function::Rotate3d(x, y, z, angle) => write_function!(w, "rotate3d"; x, y, z, angle),
            Function::Skew(x, y) => write_function!(w, "skew"; x, y),
            Function::SkewX(angle) => write_function!(w, "skewX"; angle),
            Function::SkewY(angle) => write_function!(w, "skewY"; angle),
            Function::Matrix([a, b, c, d, e, f]) => {
                write_function!(w, "matrix"; a, b, c, d, e, f)
            }
            Function::Matrix3d(values) => {
                w.write_str("matrix3d(")?;
                write_separated(w, ", ", values)?;
                w.write_char(')')
            }
            Function::Perspective(distance) => write_function!(w, "perspective"; distance),
        }
    }
}

/// A `transform` value, which is `none` or a list of at least one transform function.
///
/// ```rust
/// use typed_css::{Transform, TransformFunction, deg, px};
///
/// let transform = Transform::from(TransformFunction::translate(px(10), px(20)))
///     .then(TransformFunction::rotate(deg(45)));
/// assert_eq!(transform.to_string(), "translate(10px, 20px) rotate(45deg)");
/// assert_eq!(Transform::none().to_string(), "none");
/// ```
///
/// [CSS Transforms Level 1 `transform` grammar](https://www.w3.org/TR/css-transforms-1/#transform-property)
#[derive(Clone, Debug, PartialEq)]
pub struct Transform(Vec<TransformFunction>);

impl Transform {
    /// The `none` keyword.
    #[must_use]
    pub const fn none() -> Self {
        Self(Vec::new())
    }

    /// Append a transform function.
    ///
    /// CSS applies the functions from right to left. So `translate(10px, 0) rotate(45deg)` first
    /// rotates the element and then moves the rotated element.
    #[must_use]
    pub fn then(mut self, function: TransformFunction) -> Self {
        self.0.push(function);
        self
    }
}

impl FromIterator<TransformFunction> for Transform {
    fn from_iter<I: IntoIterator<Item = TransformFunction>>(functions: I) -> Self {
        Self(functions.into_iter().collect())
    }
}

impl From<TransformFunction> for Transform {
    fn from(function: TransformFunction) -> Self {
        Self(vec![function])
    }
}

declaration_sources!(Transform: TransformFunction);

impl ToCss for Transform {
    fn to_css<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        if self.0.is_empty() {
            return w.write_str("none");
        }
        write_spaced(w, &self.0)
    }
}

/// A `translate` property value.
///
/// [CSS Transforms Level 2 `translate` grammar](https://www.w3.org/TR/css-transforms-2/#propdef-translate)
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum Translate {
    /// The `none` keyword.
    None,
    /// A horizontal translation.
    X(LengthPercentage),
    /// Horizontal and vertical translations.
    XY(LengthPercentage, LengthPercentage),
    /// Horizontal, vertical, and depth translations.
    XYZ(LengthPercentage, LengthPercentage, Length),
}

impl Translate {
    /// Create horizontal and vertical translations.
    #[must_use]
    pub fn xy(x: impl Into<LengthPercentage>, y: impl Into<LengthPercentage>) -> Self {
        Self::XY(x.into(), y.into())
    }

    /// Create horizontal, vertical, and depth translations.
    #[must_use]
    pub fn xyz(x: impl Into<LengthPercentage>, y: impl Into<LengthPercentage>, z: Length) -> Self {
        Self::XYZ(x.into(), y.into(), z)
    }
}

length_percentage_sources!(Translate, |value| Self::X(value));

impl ToCss for Translate {
    fn to_css<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match self {
            Self::None => w.write_str("none"),
            Self::X(x) => x.to_css(w),
            Self::XY(x, y) => write_list!(w, " "; x, y),
            Self::XYZ(x, y, z) => write_list!(w, " "; x, y, z),
        }
    }
}

/// A `rotate` property value.
///
/// [CSS Transforms Level 2 `rotate` grammar](https://www.w3.org/TR/css-transforms-2/#propdef-rotate)
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum Rotate {
    /// The `none` keyword.
    None,
    /// A rotation around the z axis.
    Angle(Angle),
    /// A rotation around the x axis.
    X(Angle),
    /// A rotation around the y axis.
    Y(Angle),
    /// A rotation around the axis `(x, y, z)`.
    Vector(Number, Number, Number, Angle),
}

impl From<Angle> for Rotate {
    fn from(angle: Angle) -> Self {
        Self::Angle(angle)
    }
}

impl Rotate {
    /// Rotate around the x axis.
    #[must_use]
    pub const fn x(angle: Angle) -> Self {
        Self::X(angle)
    }

    /// Rotate around the y axis.
    #[must_use]
    pub const fn y(angle: Angle) -> Self {
        Self::Y(angle)
    }

    /// Rotate around the axis that points from the origin to `(x, y, z)`.
    ///
    /// # Panics
    ///
    /// Panics if `x`, `y`, or `z` is not finite.
    #[track_caller]
    #[must_use]
    pub fn vector(x: impl Into<f64>, y: impl Into<f64>, z: impl Into<f64>, angle: Angle) -> Self {
        Self::Vector(factor(x), factor(y), factor(z), angle)
    }
}

declaration_sources!(Rotate: Angle);

impl ToCss for Rotate {
    fn to_css<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match self {
            Self::None => w.write_str("none"),
            Self::Angle(angle) => angle.to_css(w),
            Self::X(angle) => {
                w.write_str("x ")?;
                angle.to_css(w)
            }
            Self::Y(angle) => {
                w.write_str("y ")?;
                angle.to_css(w)
            }
            Self::Vector(x, y, z, angle) => write_list!(w, " "; x, y, z, angle),
        }
    }
}

/// A `scale` property value.
///
/// [CSS Transforms Level 2 `scale` grammar](https://www.w3.org/TR/css-transforms-2/#propdef-scale)
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum Scale {
    /// The `none` keyword.
    None,
    /// One factor for both axes.
    Uniform(Number),
    /// Separate horizontal and vertical factors.
    XY(Number, Number),
    /// Separate horizontal, vertical, and depth factors.
    XYZ(Number, Number, Number),
}

impl From<Number> for Scale {
    fn from(factor: Number) -> Self {
        Self::Uniform(factor)
    }
}

impl From<i32> for Scale {
    fn from(factor: i32) -> Self {
        Self::Uniform(Number::new(f64::from(factor)))
    }
}

declaration_sources!(Scale: Number, i32);

impl Scale {
    /// Use one factor for both axes. A factor of `1` keeps the size.
    ///
    /// # Panics
    ///
    /// Panics if `value` is not finite.
    #[track_caller]
    #[must_use]
    pub fn uniform(value: impl Into<f64>) -> Self {
        Self::Uniform(factor(value))
    }

    /// Use separate horizontal and vertical factors.
    ///
    /// # Panics
    ///
    /// Panics if `x` or `y` is not finite.
    #[track_caller]
    #[must_use]
    pub fn xy(x: impl Into<f64>, y: impl Into<f64>) -> Self {
        Self::XY(factor(x), factor(y))
    }

    /// Use separate horizontal, vertical, and depth factors.
    ///
    /// # Panics
    ///
    /// Panics if `x`, `y`, or `z` is not finite.
    #[track_caller]
    #[must_use]
    pub fn xyz(x: impl Into<f64>, y: impl Into<f64>, z: impl Into<f64>) -> Self {
        Self::XYZ(factor(x), factor(y), factor(z))
    }
}

impl ToCss for Scale {
    fn to_css<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match self {
            Self::None => w.write_str("none"),
            Self::Uniform(factor) => factor.to_css(w),
            Self::XY(x, y) => write_list!(w, " "; x, y),
            Self::XYZ(x, y, z) => write_list!(w, " "; x, y, z),
        }
    }
}

impl_display!(TransformFunction, Transform, Translate, Rotate, Scale);
impl_checked_value!(Transform, Translate, Rotate, Scale);

property_keywords! {
    Transform {
        none: "none" => Transform::none(),
    }
}

property_keywords! {
    Translate {
        none: "none" => Translate::None,
    }
}

property_keywords! {
    Rotate {
        none: "none" => Rotate::None,
    }
}

property_keywords! {
    Scale {
        none: "none" => Scale::None,
    }
}
