//! The type of a property constant.

use std::{fmt, hash, marker::PhantomData};

use crate::global_keyword::global_keyword_methods;
use crate::{
    CheckedValue, Declaration, DeclarationValue, GlobalKeyword, property_name::PropertyName,
};

/// A standard CSS property together with the grammar of its values.
///
/// Every supported property is a constant in [`crate::property`], such as
/// `property::WIDTH: Property<Size>`. Only this crate can create a `Property`, so a property
/// name is always paired with the right grammar.
///
/// A property has one method for each keyword that its grammar accepts on its own, so that
/// `DISPLAY.grid()` declares `display:grid`. Other values go through `declare`:
///
/// ```rust
/// use typed_css::{px, property::{DISPLAY, JUSTIFY_CONTENT, WIDTH}};
///
/// assert_eq!(DISPLAY.grid().to_string(), "display:grid;");
/// assert_eq!(JUSTIFY_CONTENT.space_between().to_string(), "justify-content:space-between;");
/// assert_eq!(WIDTH.auto().to_string(), "width:auto;");
/// assert_eq!(WIDTH.declare(px(120)).to_string(), "width:120px;");
/// ```
///
/// Properties with the same grammar have the same type. So a function can take the property to
/// declare as a parameter:
///
/// ```rust
/// use typed_css::{
///     Declaration, Color, Property, rgb,
///     property::{BACKGROUND_COLOR, COLOR},
/// };
///
/// fn tint(target: Property<Color>) -> Declaration {
///     target.declare(rgb(255, 0, 0))
/// }
///
/// assert_eq!(tint(COLOR).property_name(), "color");
/// assert_eq!(tint(BACKGROUND_COLOR).property_name(), "background-color");
/// ```
///
/// ```compile_fail
/// use typed_css::{Property, Size};
///
/// // Only the crate's catalog can create a `Property`.
/// let _ = Property::<Size>::new("width");
/// ```
pub struct Property<T> {
    name: &'static str,
    grammar: PhantomData<fn() -> T>,
}

impl<T> Property<T> {
    pub(crate) const fn new(name: &'static str) -> Self {
        Self {
            name,
            grammar: PhantomData,
        }
    }

    /// Return the CSS property name.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        self.name
    }

    /// Declare this property with a CSS-wide keyword, such as `inherit`.
    ///
    /// Each keyword also has its own method, such as [`Property::inherit`].
    #[must_use]
    pub fn declare_global(&self, keyword: GlobalKeyword) -> Declaration {
        Declaration::new(PropertyName::Static(self.name), &keyword)
    }

    global_keyword_methods!();
}

impl<T: CheckedValue> Property<T> {
    /// Declare this property with a value of its grammar.
    ///
    /// The value can be a value of the grammar, a common part of the grammar such as `px(4)` for
    /// `width`, or a `var()` reference to a custom property whose grammar converts into this
    /// grammar, such as a `Length` custom property for `width`.
    #[must_use]
    pub fn declare(&self, value: impl Into<DeclarationValue<T>>) -> Declaration {
        Declaration::new(PropertyName::Static(self.name), &value.into())
    }
}

/// Add a method to the properties of each listed grammar for each keyword of the table.
///
/// Each entry names the method, the CSS text of the keyword, and the value that the method
/// declares. The value must convert into the grammar, so the compiler checks each keyword. The
/// tests compare the CSS text of each entry with the declared value.
macro_rules! property_keywords {
    ($grammar:ident, $($rest:ident),+ { $($entries:tt)* }) => {
        $crate::property::definition::property_keywords!($grammar { $($entries)* });
        $crate::property::definition::property_keywords!($($rest),+ { $($entries)* });
    };
    ($grammar:ident { $($method:ident: $css:literal => $value:expr),+ $(,)? }) => {
        impl $crate::Property<$grammar> {
            $(
                #[doc = concat!("Declare this property as `", $css, "`.")]
                #[must_use]
                pub fn $method(&self) -> $crate::Declaration {
                    self.declare($value)
                }
            )+

            #[cfg(test)]
            pub(crate) const KEYWORDS: &'static [(&'static str, fn(&Self) -> $crate::Declaration)] =
                &[$(($css, Self::$method)),+];
        }
    };
}

pub(crate) use property_keywords;

impl<T> Clone for Property<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Property<T> {}

impl<T> fmt::Debug for Property<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Property").field(&self.name).finish()
    }
}

impl<T> PartialEq for Property<T> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl<T> Eq for Property<T> {}

impl<T> hash::Hash for Property<T> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
