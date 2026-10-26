use crate::serialization::ToCss;

pub(crate) mod sealed {
    pub trait Sealed {}
}

/// A value type that properties and custom properties accept.
///
/// The value types of the properties in [`crate::property`] implement `CheckedValue`, and so do the
/// primitive types that custom properties commonly hold, such as [`Length`](crate::Length),
/// [`Color`](crate::Color), and [`Time`](crate::Time). [`Property`](crate::Property),
/// [`CustomProperty`](crate::CustomProperty), and [`var`](crate::var) accept only such types.
/// The trait is sealed, so other crates cannot implement it. An application can implement
/// [`ToCss`] for its own types, but it cannot declare them.
///
/// ```compile_fail
/// use std::fmt;
/// use typed_css::{CustomProperty, ToCss};
///
/// struct ExternalValue;
///
/// impl ToCss for ExternalValue {
///     fn to_css<W: fmt::Write>(&self, _dest: &mut W) -> fmt::Result {
///         Ok(())
///     }
/// }
///
/// let _ = CustomProperty::<ExternalValue>::new("--external");
/// ```
pub trait CheckedValue: ToCss + sealed::Sealed {}

impl<T> CheckedValue for T where T: ToCss + sealed::Sealed {}

/// Implement `Display` through [`ToCss`].
macro_rules! impl_display {
    ($($ty:ty),+ $(,)?) => {$ (
        impl ::std::fmt::Display for $ty {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                $crate::serialization::ToCss::to_css(self, f)
            }
        }
    )+};
}

pub(crate) use impl_display;

/// Mark value types as checked values, so that properties and custom properties accept them.
macro_rules! impl_checked_value {
    ($($ty:ty),+ $(,)?) => {$ (
        impl $crate::checked_value::sealed::Sealed for $ty {}
    )+};
}

pub(crate) use impl_checked_value;

/// Define an enum of CSS keywords with `as_str`, `ToCss`, and `Display`.
///
/// The documentation of each variant starts with its CSS keyword. Attributes written on a variant
/// are kept.
///
/// When every variant names a method with `as`, such as `Block => "block" as block`, the macro also
/// adds the keyword methods to the properties of the enum, such as `DISPLAY.block()`. The grammars
/// after `for`, such as `pub enum OverflowKeyword for Overflow`, get the same methods.
macro_rules! keyword_enum {
    (
        $(#[$meta:meta])*
        pub enum $name:ident $(for $($grammar:ident),+)? {
            $($(#[$variant_meta:meta])* $variant:ident => $css:literal as $method:ident),+ $(,)?
        }
    ) => {
        $crate::checked_value::keyword_enum! {
            $(#[$meta])*
            pub enum $name {
                $($(#[$variant_meta])* $variant => $css),+
            }
        }

        $crate::property::definition::property_keywords!(
            $name $($(, $grammar)+)? { $($method: $css => $name::$variant),+ }
        );
    };
    (
        $(#[$meta:meta])*
        pub enum $name:ident {
            $($(#[$variant_meta:meta])* $variant:ident => $css:literal),+ $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        #[non_exhaustive]
        pub enum $name {
            $(
                #[doc = concat!("`", $css, "`.")]
                $(#[$variant_meta])*
                $variant
            ),+
        }

        impl $name {
            /// Return the CSS text of the value, which can be more than one keyword.
            #[must_use]
            pub const fn as_str(self) -> &'static str {
                match self {
                    $(Self::$variant => $css),+
                }
            }
        }

        impl $crate::serialization::ToCss for $name {
            fn to_css<W: ::std::fmt::Write>(&self, dest: &mut W) -> ::std::fmt::Result {
                dest.write_str(self.as_str())
            }
        }

        $crate::checked_value::impl_display!($name);
    };
}

pub(crate) use keyword_enum;
