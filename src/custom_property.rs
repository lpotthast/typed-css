use std::{borrow::Cow, fmt, marker::PhantomData};

use crate::global_keyword::global_keyword_methods;
use crate::{
    CheckedValue, Declaration, GlobalKeyword,
    identifier::is_checked_custom_property_name,
    property_name::PropertyName,
    serialization::{ToCss, same_css},
};

/// Error returned when a custom-property name is outside the accepted subset.
///
/// An accepted name is `--` followed by ASCII letters, digits, hyphens, and underscores, such as
/// `--accent` or `--2xl`. CSS also allows names with escapes or non-ASCII characters. This crate
/// rejects those, so that a name never needs escaping.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InvalidCustomPropertyName {
    name: String,
}

impl InvalidCustomPropertyName {
    /// Return the rejected name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for InvalidCustomPropertyName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "custom property name must match --[A-Za-z0-9_-]+, got {:?}",
            self.name
        )
    }
}

impl std::error::Error for InvalidCustomPropertyName {}

/// A CSS custom property, such as `--accent`, that holds values of one grammar `T`.
///
/// The grammar type keeps Rust code consistent. A `CustomProperty<Color>` is assigned colors, and
/// a `var()` reference to it is accepted only where a color is. The browser does not know about
/// this type, though. Another `CustomProperty` with the same name and a different grammar, or any
/// stylesheet, can assign a value of another type. A `var()` with a value of the wrong type makes
/// the declaration that uses it invalid, and a `var()` fallback does not help in that case.
///
/// The [`custom_property!`](crate::custom_property) macro declares a custom property as a
/// constant and checks its name at compile time.
///
/// ```rust
/// use typed_css::{Color, CustomProperty, NamedColor, custom_property, property::COLOR, var};
///
/// custom_property!(ACCENT: Color = "--accent");
///
/// let assignment = ACCENT.declare(NamedColor::Teal);
/// let usage = COLOR.declare(var(&ACCENT));
/// assert_eq!(assignment.to_string(), "--accent:teal;");
/// assert_eq!(usage.to_string(), "color:var(--accent);");
///
/// let runtime = CustomProperty::<Color>::try_new(format!("--theme-{}", "primary")).unwrap();
/// assert_eq!(runtime.name(), "--theme-primary");
/// ```
///
/// [CSS Custom Properties Level 1](https://www.w3.org/TR/css-variables-1/)
pub struct CustomProperty<T> {
    name: PropertyName,
    grammar: PhantomData<fn() -> T>,
}

impl<T> Clone for CustomProperty<T> {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            grammar: PhantomData,
        }
    }
}

impl<T> fmt::Debug for CustomProperty<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("CustomProperty").field(&self.name).finish()
    }
}

impl<T> PartialEq for CustomProperty<T> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl<T> Eq for CustomProperty<T> {}

impl<T> std::hash::Hash for CustomProperty<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl<T: CheckedValue> CustomProperty<T> {
    /// Create a custom property from a static name.
    ///
    /// This constructor is const, so it can create constants. The
    /// [`custom_property!`](crate::custom_property) macro is a shorter way to do that.
    ///
    /// # Panics
    ///
    /// Panics, or fails const evaluation, if `name` is not an accepted custom-property name.
    #[track_caller]
    #[must_use]
    pub const fn from_static(name: &'static str) -> Self {
        assert!(
            is_checked_custom_property_name(name),
            "invalid CSS custom-property name"
        );
        Self {
            name: PropertyName::Static(name),
            grammar: PhantomData,
        }
    }

    /// Create a custom property.
    ///
    /// # Panics
    ///
    /// Panics if `name` is not an accepted custom-property name. Use [`CustomProperty::try_new`]
    /// for runtime-derived names.
    #[track_caller]
    #[must_use]
    pub fn new(name: impl Into<Cow<'static, str>>) -> Self {
        Self::try_new(name).unwrap_or_else(|error| panic!("{error}"))
    }

    /// Attempt to create a custom property.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidCustomPropertyName`] if `name` is not an accepted custom-property name.
    pub fn try_new(name: impl Into<Cow<'static, str>>) -> Result<Self, InvalidCustomPropertyName> {
        let name = name.into();
        if is_checked_custom_property_name(&name) {
            Ok(Self {
                name: name.into(),
                grammar: PhantomData,
            })
        } else {
            Err(InvalidCustomPropertyName {
                name: name.into_owned(),
            })
        }
    }

    /// Return the name, including the leading `--`. Custom-property names are case-sensitive.
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Assign a value to this custom property.
    ///
    /// The value can be a value of the grammar, or a `var()` reference to another custom property
    /// whose grammar converts into this grammar.
    #[must_use]
    pub fn declare(&self, value: impl Into<DeclarationValue<T>>) -> Declaration {
        Declaration::new(self.name.clone(), &value.into())
    }

    /// Assign a CSS-wide keyword, such as `inherit`, to this custom property.
    ///
    /// Each keyword also has its own method, such as [`CustomProperty::inherit`].
    #[must_use]
    pub fn declare_global(&self, keyword: GlobalKeyword) -> Declaration {
        Declaration::new(self.name.clone(), &keyword)
    }

    global_keyword_methods!();
}

impl<T> CustomProperty<T> {
    /// Reinterpret the property as a grammar that contains every value of `T`.
    fn widen<U>(self) -> CustomProperty<U> {
        CustomProperty {
            name: self.name,
            grammar: PhantomData,
        }
    }
}

/// Declare a custom property as a constant.
///
/// The macro checks the name at compile time.
///
/// ```rust
/// use typed_css::{Color, custom_property};
///
/// custom_property!(pub ACCENT_COLOR: Color = "--accent-color");
/// assert_eq!(ACCENT_COLOR.name(), "--accent-color");
/// ```
///
/// An invalid name fails to compile:
///
/// ```compile_fail
/// use typed_css::{Color, custom_property};
///
/// custom_property!(ACCENT_COLOR: Color = "accent-color");
/// ```
#[macro_export]
macro_rules! custom_property {
    ($(#[$meta:meta])* $vis:vis $name:ident : $value:ty = $css_name:literal $(;)?) => {
        $(#[$meta])*
        $vis const $name: $crate::CustomProperty<$value> =
            $crate::CustomProperty::from_static($css_name);
    };
}

impl<T> fmt::Display for CustomProperty<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name.as_str())
    }
}

/// The value of a declaration, which is a value of the grammar `T` or a `var()` reference to a
/// custom property whose grammar converts into `T`.
///
/// Properties accept anything that converts into a `DeclarationValue`. Many grammars also
/// convert from their common parts, so that `WIDTH.declare(px(120))` works without wrapping the
/// length in a [`Size`](crate::Size) first.
#[derive(Clone, Debug)]
pub struct DeclarationValue<T>(DeclarationValueKind<T>);

#[derive(Clone, Debug)]
enum DeclarationValueKind<T> {
    Literal(T),
    Variable(Var<T>),
}

impl<T: CheckedValue> From<T> for DeclarationValue<T> {
    fn from(value: T) -> Self {
        Self(DeclarationValueKind::Literal(value))
    }
}

impl<T> DeclarationValue<T> {
    /// Convert the value into a grammar that contains `T`, such as a `DeclarationValue<Angle>`
    /// into the value of `rotate`.
    ///
    /// ```rust
    /// use typed_css::{Angle, DeclarationValue, Rotate, deg, property::ROTATE};
    ///
    /// let tilt: DeclarationValue<Angle> = deg(3).into();
    /// assert_eq!(ROTATE.declare(tilt.into_grammar::<Rotate>()).value(), "3deg");
    /// ```
    #[must_use]
    pub fn into_grammar<G: CheckedValue + From<T>>(self) -> DeclarationValue<G> {
        self.map_literals(&G::from)
    }

    /// Return the literal, or the `var()` reference as the error.
    pub(crate) fn into_literal(self) -> Result<T, Var<T>> {
        match self.0 {
            DeclarationValueKind::Literal(value) => Ok(value),
            DeclarationValueKind::Variable(reference) => Err(reference),
        }
    }

    /// Convert each literal with `f`, including the fallbacks of `var()` references.
    pub(crate) fn map_literals<U>(self, f: &impl Fn(T) -> U) -> DeclarationValue<U> {
        DeclarationValue(match self.0 {
            DeclarationValueKind::Literal(value) => DeclarationValueKind::Literal(f(value)),
            DeclarationValueKind::Variable(reference) => {
                DeclarationValueKind::Variable(reference.map_literals(f))
            }
        })
    }
}

/// Implement `From<Source> for DeclarationValue<Target>` through `Target: From<Source>`.
///
/// This lets a property accept the common parts of its grammar directly.
macro_rules! declaration_sources {
    ($target:ty: $($source:ty),+ $(,)?) => {$(
        impl From<$source> for $crate::DeclarationValue<$target> {
            fn from(source: $source) -> Self {
                Self::from(<$target>::from(source))
            }
        }
    )+};
}

pub(crate) use declaration_sources;

/// A `var()` reference to a custom property of the grammar `T`, with an optional fallback.
///
/// [`var`] and [`var_or`] create references.
#[derive(Clone, Debug)]
pub struct Var<T> {
    property: CustomProperty<T>,
    fallback: Option<Box<DeclarationValue<T>>>,
}

impl<T> Var<T> {
    /// Reinterpret the reference as a grammar that contains every value of `T`.
    pub(crate) fn widen<U: From<T>>(self) -> Var<U> {
        self.map_literals(&U::from)
    }

    /// Reinterpret the reference as the grammar `U`, converting the literals of the fallback
    /// with `f`.
    pub(crate) fn map_literals<U>(self, f: &impl Fn(T) -> U) -> Var<U> {
        Var {
            property: self.property.widen(),
            fallback: self
                .fallback
                .map(|fallback| Box::new(fallback.map_literals(f))),
        }
    }
}

/// A `var()` reference converts into the declaration value of every grammar that contains the
/// grammar of the reference. A `var()` of a `Length`, for example, works for `width`.
impl<T, G> From<Var<T>> for DeclarationValue<G>
where
    G: CheckedValue + From<T>,
{
    fn from(value: Var<T>) -> Self {
        Self(DeclarationValueKind::Variable(value.widen()))
    }
}

impl<T> DeclarationValue<T> {
    /// Write the value, with `write_literal` writing each literal, including the fallbacks of
    /// `var()` references.
    pub(crate) fn write_literals_with<W: fmt::Write>(
        &self,
        dest: &mut W,
        write_literal: &impl Fn(&T, &mut W) -> fmt::Result,
    ) -> fmt::Result {
        match &self.0 {
            DeclarationValueKind::Literal(value) => write_literal(value, dest),
            DeclarationValueKind::Variable(value) => value.write_literals_with(dest, write_literal),
        }
    }
}

impl<T> Var<T> {
    /// Write the reference, with `write_literal` writing each literal of the fallback.
    pub(crate) fn write_literals_with<W: fmt::Write>(
        &self,
        dest: &mut W,
        write_literal: &impl Fn(&T, &mut W) -> fmt::Result,
    ) -> fmt::Result {
        dest.write_str("var(")?;
        dest.write_str(self.property.name.as_str())?;
        if let Some(fallback) = &self.fallback {
            dest.write_str(", ")?;
            fallback.write_literals_with(dest, write_literal)?;
        }
        dest.write_char(')')
    }
}

impl<T: CheckedValue> ToCss for DeclarationValue<T> {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        self.write_literals_with(dest, &|value, dest| value.to_css(dest))
    }
}

impl<T: CheckedValue> ToCss for Var<T> {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        self.write_literals_with(dest, &|value, dest| value.to_css(dest))
    }
}

/// Two values are equal when they have the same CSS text. A `var()` can be stored as a reference
/// or inside a value of the grammar, such as a `LengthPercentage`, and both forms are equal.
impl<T: CheckedValue> PartialEq for DeclarationValue<T> {
    fn eq(&self, other: &Self) -> bool {
        same_css(self, other)
    }
}

/// Two references are equal when they have the same CSS text.
impl<T: CheckedValue> PartialEq for Var<T> {
    fn eq(&self, other: &Self) -> bool {
        same_css(self, other)
    }
}

impl<T: CheckedValue> fmt::Display for DeclarationValue<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_css(f)
    }
}

impl<T: CheckedValue> fmt::Display for Var<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_css(f)
    }
}

/// Reference a custom property with `var()`.
///
/// If the custom property is not defined where the reference is used, the declaration becomes
/// invalid and the property falls back to its inherited or initial value. Use [`var_or`] to give a
/// fallback instead.
///
/// ```rust
/// use typed_css::{Color, custom_property, var};
///
/// custom_property!(ACCENT: Color = "--accent");
/// assert_eq!(var(&ACCENT).to_string(), "var(--accent)");
/// ```
///
/// [CSS variable substitution](https://www.w3.org/TR/css-variables-1/#using-variables)
#[must_use]
pub fn var<T: CheckedValue>(property: &CustomProperty<T>) -> Var<T> {
    Var {
        property: property.clone(),
        fallback: None,
    }
}

/// Reference a custom property with `var()` and a fallback of the same grammar.
///
/// The browser uses the fallback when the custom property is not defined. The fallback can be
/// another `var()` reference. A custom property must not reference itself, as in
/// `ACCENT.declare(var_or(&ACCENT, ..))`, because CSS treats such a cycle as invalid and ignores the
/// fallback.
///
/// ```rust
/// use typed_css::{Color, NamedColor, custom_property, var_or};
///
/// custom_property!(ACCENT: Color = "--accent");
/// let reference = var_or(&ACCENT, NamedColor::CurrentColor);
/// assert_eq!(reference.to_string(), "var(--accent, currentcolor)");
/// ```
///
/// A fallback must be a checked value, not just a value that implements [`ToCss`]:
///
/// ```compile_fail
/// use typed_css::{CustomProperty, ToCss, var_or};
///
/// fn reference<T: ToCss>(property: &CustomProperty<T>, fallback: T) {
///     let _ = var_or(property, fallback);
/// }
/// ```
///
/// [CSS variable substitution](https://www.w3.org/TR/css-variables-1/#using-variables)
#[must_use]
pub fn var_or<T: CheckedValue>(
    property: &CustomProperty<T>,
    fallback: impl Into<DeclarationValue<T>>,
) -> Var<T> {
    Var {
        property: property.clone(),
        fallback: Some(Box::new(fallback.into())),
    }
}
