use crate::checked_value::keyword_enum;

keyword_enum! {
    /// A CSS-wide keyword, which every property accepts.
    ///
    /// Properties and custom properties take a CSS-wide keyword through `declare_global` or a
    /// method such as `inherit`, because the keyword is not part of a property's own grammar.
    ///
    /// [CSS Cascading and Inheritance Level 5 CSS-wide keywords](https://www.w3.org/TR/css-cascade-5/#defaulting-keywords)
    pub enum GlobalKeyword {
        /// Use the parent's computed value.
        Inherit => "inherit",
        /// Use the property's initial value.
        Initial => "initial",
        /// Use `inherit` for an inherited property and `initial` for any other property.
        Unset => "unset",
        /// Roll back to the value of the previous cascade origin, such as the browser's default style.
        Revert => "revert",
        /// Roll back to the value of the previous cascade layer.
        RevertLayer => "revert-layer",
    }
}

impl GlobalKeyword {
    /// Every CSS-wide keyword.
    pub(crate) const ALL: [Self; 5] = [
        Self::Inherit,
        Self::Initial,
        Self::Unset,
        Self::Revert,
        Self::RevertLayer,
    ];
}

/// The value type of [`property::ALL`](crate::property::ALL), which has no values.
///
/// The `all` property accepts only CSS-wide keywords. `ALL.unset()` or `ALL.declare_global(..)`
/// declares it, and `ALL.declare(..)` is not available, because no value of this type exists.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GlobalKeywordsOnly {}

/// Define a method for each CSS-wide keyword in an `impl` block of a type with `declare_global`.
macro_rules! global_keyword_methods {
    () => {
        $crate::global_keyword::global_keyword_methods!(
            inherit: "inherit" => Inherit,
            initial: "initial" => Initial,
            unset: "unset" => Unset,
            revert: "revert" => Revert,
            revert_layer: "revert-layer" => RevertLayer,
        );
    };
    ($($method:ident: $css:literal => $variant:ident),+ $(,)?) => {$(
        #[doc = concat!(
            "Declare this property as `", $css, "`, which [`GlobalKeyword::", stringify!($variant),
            "`](crate::GlobalKeyword::", stringify!($variant), ") describes."
        )]
        #[must_use]
        pub fn $method(&self) -> $crate::Declaration {
            self.declare_global($crate::GlobalKeyword::$variant)
        }
    )+};
}

pub(crate) use global_keyword_methods;
