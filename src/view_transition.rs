use std::fmt;

use crate::checked_value::{impl_checked_value, impl_display};
use crate::property::definition::property_keywords;
use crate::{GlobalKeyword, identifier::is_checked_identifier, serialization::ToCss};

/// Error returned when a view-transition name is not an accepted identifier.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InvalidViewTransitionName {
    name: String,
}

impl InvalidViewTransitionName {
    /// Return the rejected identifier.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for InvalidViewTransitionName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "view-transition-name must be an identifier other than none, auto, match-element, default, or a CSS-wide keyword, got {:?}",
            self.name
        )
    }
}

impl std::error::Error for InvalidViewTransitionName {}

fn is_reserved_custom_identifier(identifier: &str) -> bool {
    identifier.eq_ignore_ascii_case("default")
        || GlobalKeyword::ALL
            .iter()
            .any(|keyword| identifier.eq_ignore_ascii_case(keyword.as_str()))
}

/// A `view-transition-name` value, which is `none` or a name.
///
/// A name must start with an ASCII letter or underscore and contain only ASCII letters, digits,
/// hyphens, and underscores. It must not be one of the keywords `none`, `auto`, `match-element`,
/// or `default`, or a CSS-wide keyword.
///
/// ```rust
/// use typed_css::ViewTransitionName;
///
/// assert_eq!(ViewTransitionName::named("hero-card").to_string(), "hero-card");
/// assert!(ViewTransitionName::try_named("auto").is_err());
/// ```
///
/// [CSS View Transitions Level 1 `view-transition-name`](https://www.w3.org/TR/css-view-transitions-1/#view-transition-name-prop)
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ViewTransitionName(ViewTransitionNameValue);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum ViewTransitionNameValue {
    None,
    Named(String),
}

impl ViewTransitionName {
    /// Create the `none` keyword.
    #[must_use]
    pub const fn none() -> Self {
        Self(ViewTransitionNameValue::None)
    }

    /// Create a named view transition.
    ///
    /// # Panics
    ///
    /// Panics if `name` is not an accepted name. Use [`ViewTransitionName::try_named`] for
    /// runtime-derived names.
    #[track_caller]
    #[must_use]
    pub fn named(name: impl Into<String>) -> Self {
        Self::try_named(name).unwrap_or_else(|error| panic!("{error}"))
    }

    /// Attempt to create a named view transition.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidViewTransitionName`] if `name` is not an accepted name.
    pub fn try_named(name: impl Into<String>) -> Result<Self, InvalidViewTransitionName> {
        let name = name.into();
        let property_reserved = ["none", "auto", "match-element"]
            .iter()
            .any(|keyword| name.eq_ignore_ascii_case(keyword));
        if is_checked_identifier(&name)
            && !is_reserved_custom_identifier(&name)
            && !property_reserved
        {
            Ok(Self(ViewTransitionNameValue::Named(name)))
        } else {
            Err(InvalidViewTransitionName { name })
        }
    }
}

impl Default for ViewTransitionName {
    fn default() -> Self {
        Self::none()
    }
}

impl ToCss for ViewTransitionName {
    fn to_css<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match &self.0 {
            ViewTransitionNameValue::None => w.write_str("none"),
            ViewTransitionNameValue::Named(name) => w.write_str(name),
        }
    }
}

impl_display!(ViewTransitionName);
impl_checked_value!(ViewTransitionName);

property_keywords! {
    ViewTransitionName {
        none: "none" => ViewTransitionName::none(),
    }
}
