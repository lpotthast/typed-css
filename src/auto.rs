/// The `auto` keyword, which converts into every grammar that has it.
///
/// Each grammar with `auto` also has its own variant for it, such as `Size::Auto`. This type lets
/// a property take `auto` without naming the grammar.
///
/// ```rust
/// use typed_css::{Auto, property::{MARGIN_INLINE, OVERFLOW, WIDTH}};
///
/// assert_eq!(MARGIN_INLINE.declare(Auto).value(), "auto");
/// assert_eq!(WIDTH.declare(Auto).value(), "auto");
/// assert_eq!(OVERFLOW.declare(Auto).value(), "auto");
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Auto;

impl crate::ToCss for Auto {
    fn to_css<W: std::fmt::Write>(&self, dest: &mut W) -> std::fmt::Result {
        dest.write_str("auto")
    }
}

crate::checked_value::impl_display!(Auto);

/// Convert [`Auto`] into checked value types and into their declaration values.
///
/// The `@conversions_only` form is for parts of a grammar that are not checked values.
macro_rules! auto_sources {
    (@conversions_only $($target:ty => $value:expr),+ $(,)?) => {$(
        impl From<$crate::Auto> for $target {
            fn from(_: $crate::Auto) -> Self {
                $value
            }
        }
    )+};
    ($($target:ty => $value:expr),+ $(,)?) => {
        $crate::auto::auto_sources!(@conversions_only $($target => $value),+);
        $(
            $crate::custom_property::declaration_sources!($target: $crate::Auto);
        )+
    };
}

pub(crate) use auto_sources;
