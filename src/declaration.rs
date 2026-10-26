use std::fmt;

use crate::{property_name::PropertyName, serialization::ToCss};

/// A CSS declaration, such as `width:120px`, whose value was checked against the property.
///
/// Properties and custom properties create declarations with `declare`. The fields are
/// private, so the property and the value cannot be separated and paired with something else.
/// Declarations of different properties have the same type, so code can store them together.
///
/// `Display` and [`ToCss`] write the declaration as `property:value;`, ready for a `style`
/// attribute. Writing several declarations into one `String` with [`ToCss::to_css`] avoids a
/// separate string for each declaration.
///
/// ```rust
/// use typed_css::{
///     Color, Declaration, ToCss, custom_property, px, rgb, property::{COLOR, PADDING},
/// };
///
/// custom_property!(ACCENT: Color = "--accent");
///
/// let declarations: Vec<Declaration> = vec![
///     PADDING.declare(px(16)),
///     COLOR.declare(rgb(255, 0, 0)),
///     ACCENT.declare(rgb(0, 0, 255)),
/// ];
/// let mut style = String::new();
/// for declaration in &declarations {
///     declaration.to_css(&mut style).unwrap();
/// }
///
/// assert_eq!(style, "padding:16px;color:rgb(255 0 0);--accent:rgb(0 0 255);");
/// assert_eq!(declarations[0].property_name(), "padding");
/// assert_eq!(declarations[0].value(), "16px");
/// ```
///
/// With the `leptos` feature, a declaration renders directly as a Leptos style attribute.
///
/// Code outside this crate cannot create a declaration from a name and a value:
///
/// ```compile_fail
/// use std::borrow::Cow;
/// use typed_css::{Declaration, rgb};
///
/// let _ = Declaration::new(Cow::Borrowed("padding"), &rgb(255, 0, 0));
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Declaration {
    property: PropertyName,
    value: String,
}

impl Declaration {
    pub(crate) fn new(property: PropertyName, value: &impl ToCss) -> Self {
        Self {
            property,
            value: value.to_css_string(),
        }
    }

    /// Return the property name, such as `padding` or `--accent`.
    #[must_use]
    pub fn property_name(&self) -> &str {
        self.property.as_str()
    }

    /// Return the value as CSS text, such as `16px`.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl ToCss for Declaration {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        dest.write_str(self.property.as_str())?;
        dest.write_char(':')?;
        dest.write_str(&self.value)?;
        dest.write_char(';')
    }
}

impl fmt::Display for Declaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_css(f)
    }
}

#[cfg(feature = "leptos")]
mod leptos_style {
    use leptos::tachys::{
        html::{
            attribute::Attribute,
            style::{IntoStyle, style},
        },
        renderer::{Rndr, dom::CssStyleDeclaration, types::Element},
    };

    use super::Declaration;
    use crate::ToCss;

    impl Declaration {
        /// Convert this declaration into a Leptos attribute that can be spread onto an element.
        ///
        /// This method needs the `leptos` feature. The feature also implements Leptos'
        /// `IntoStyle` for `Declaration`, so that `style=declaration` works in `view!`.
        #[must_use]
        pub fn into_attribute(self) -> impl Attribute {
            style(self)
        }
    }

    fn set_property(style: &CssStyleDeclaration, declaration: &Declaration) {
        Rndr::set_css_property(style, declaration.property_name(), declaration.value());
    }

    impl IntoStyle for Declaration {
        type AsyncOutput = Self;
        type State = (CssStyleDeclaration, Self);
        type Cloneable = Self;
        type CloneableOwned = Self;

        fn to_html(self, style: &mut String) {
            // Writing to a `String` cannot fail.
            let _ = self.to_css(style);
        }

        fn hydrate<const FROM_SERVER: bool>(self, el: &Element) -> Self::State {
            (Rndr::style(el), self)
        }

        fn build(self, el: &Element) -> Self::State {
            let style = Rndr::style(el);
            set_property(&style, &self);
            (style, self)
        }

        fn rebuild(self, state: &mut Self::State) {
            let (style, current) = state;
            if self.property != current.property {
                Rndr::remove_css_property(style, current.property_name());
                set_property(style, &self);
            } else if self.value != current.value {
                set_property(style, &self);
            }
            *current = self;
        }

        fn into_cloneable(self) -> Self::Cloneable {
            self
        }

        fn into_cloneable_owned(self) -> Self::CloneableOwned {
            self
        }

        fn dry_resolve(&mut self) {}

        fn resolve(self) -> impl Future<Output = Self::AsyncOutput> {
            std::future::ready(self)
        }

        fn reset(state: &mut Self::State) {
            Rndr::remove_css_property(&state.0, state.1.property_name());
        }
    }
}
