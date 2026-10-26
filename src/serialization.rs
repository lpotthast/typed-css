use std::fmt;

/// Serialize a value as CSS text.
///
/// Every value in this crate implements `ToCss`, and `Display` writes the same text. Applications
/// may implement `ToCss` for their own types too. Implementing it does not let a type into a checked
/// declaration, because that requires the sealed [`CheckedValue`](crate::CheckedValue) trait.
///
/// ```rust
/// use std::fmt;
/// use typed_css::{ToCss, px};
///
/// struct Spacing(u8);
///
/// impl ToCss for Spacing {
///     fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
///         write!(dest, "{}px", u32::from(self.0) * 4)
///     }
/// }
///
/// assert_eq!(Spacing(3).to_css_string(), "12px");
/// assert_eq!(px(12).to_css_string(), "12px");
/// ```
///
/// A type that only implements `ToCss` cannot be declared through the checked API:
///
/// ```compile_fail
/// use std::fmt;
/// use typed_css::{CustomProperty, ToCss};
///
/// struct Raw(&'static str);
///
/// impl ToCss for Raw {
///     fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
///         dest.write_str(self.0)
///     }
/// }
///
/// let property = CustomProperty::<Raw>::new("--unsafe");
/// let _ = property.declare(Raw("red;color:lime"));
/// ```
pub trait ToCss {
    /// Write the CSS text of this value to `dest`.
    ///
    /// # Errors
    ///
    /// Returns an error if `dest` fails to accept the text. Writing to a `String` never fails.
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result;

    /// Return the CSS text of this value as a new string.
    fn to_css_string(&self) -> String {
        // Most values fit into 24 bytes, such as `calc(100% - 20px)`, so the string rarely grows.
        // Many allocators reserve 24 bytes for even the smallest allocation.
        let mut css = String::with_capacity(24);
        // Writing to a `String` cannot fail.
        let _ = self.to_css(&mut css);
        css
    }
}

/// Write `items` separated by single spaces.
pub(crate) fn write_spaced<'a, W: fmt::Write, T: ToCss + 'a>(
    dest: &mut W,
    items: impl IntoIterator<Item = &'a T>,
) -> fmt::Result {
    write_separated(dest, " ", items)
}

/// Write `items` separated by `separator`.
///
/// The items are written with [`ToCss`], because `write!` and `Display` would add a `Formatter`
/// for each item.
pub(crate) fn write_separated<'a, W: fmt::Write, T: ToCss + 'a>(
    dest: &mut W,
    separator: &str,
    items: impl IntoIterator<Item = &'a T>,
) -> fmt::Result {
    for (index, item) in items.into_iter().enumerate() {
        if index > 0 {
            dest.write_str(separator)?;
        }
        item.to_css(dest)?;
    }
    Ok(())
}

/// Write `keywords` separated by single spaces.
pub(crate) fn write_spaced_keywords<W: fmt::Write>(
    dest: &mut W,
    keywords: impl IntoIterator<Item = &'static str>,
) -> fmt::Result {
    for (index, keyword) in keywords.into_iter().enumerate() {
        if index > 0 {
            dest.write_char(' ')?;
        }
        dest.write_str(keyword)?;
    }
    Ok(())
}

/// Write values of possibly different types, separated by `separator`, such as
/// `write_list!(dest, " "; x, y)`. Each value is a reference to a [`ToCss`] value.
macro_rules! write_list {
    ($dest:expr, $separator:literal; $first:expr $(, $rest:expr)* $(,)?) => {{
        $crate::serialization::ToCss::to_css($first, $dest)?;
        $(
            ::std::fmt::Write::write_str($dest, $separator)?;
            $crate::serialization::ToCss::to_css($rest, $dest)?;
        )*
        ::std::fmt::Result::Ok(())
    }};
}

pub(crate) use write_list;

/// Write the two values of a shorthand that repeats its first value, such as `gap`. The second
/// value is left out when its CSS text is the same as the text of the first value.
pub(crate) fn write_pair<W: fmt::Write>(
    dest: &mut W,
    first: &impl ToCss,
    second: &impl ToCss,
) -> fmt::Result {
    first.to_css(dest)?;
    if !same_css(first, second) {
        dest.write_char(' ')?;
        second.to_css(dest)?;
    }
    Ok(())
}

/// Return whether `a` and `b` have the same CSS text.
///
/// The function writes both values into buffers on the stack, so that comparing short values does
/// not allocate. Only two values that are both too long for the buffers are compared as strings.
pub(crate) fn same_css(a: &impl ToCss, b: &impl ToCss) -> bool {
    let mut a_text = StackText::new();
    let mut b_text = StackText::new();
    match (a.to_css(&mut a_text), b.to_css(&mut b_text)) {
        (Ok(()), Ok(())) => a_text.as_bytes() == b_text.as_bytes(),
        (Err(_), Err(_)) => a.to_css_string() == b.to_css_string(),
        // A text that fits into the buffer is shorter than a text that does not.
        _ => false,
    }
}

/// CSS text of at most `STACK_TEXT_CAPACITY` bytes, stored on the stack.
///
/// Writing more text than fits fails with [`fmt::Error`].
struct StackText {
    bytes: [u8; STACK_TEXT_CAPACITY],
    len: usize,
}

/// The capacity of a [`StackText`]. It holds every keyword value, such as `unsafe flex-start`.
const STACK_TEXT_CAPACITY: usize = 64;

impl StackText {
    const fn new() -> Self {
        Self {
            bytes: [0; STACK_TEXT_CAPACITY],
            len: 0,
        }
    }

    fn as_bytes(&self) -> &[u8] {
        &self.bytes[..self.len]
    }
}

impl fmt::Write for StackText {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        let end = self.len + text.len();
        let target = self.bytes.get_mut(self.len..end).ok_or(fmt::Error)?;
        target.copy_from_slice(text.as_bytes());
        self.len = end;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fmt;

    use assertr::prelude::*;

    use super::{STACK_TEXT_CAPACITY, ToCss, same_css};

    struct Text(String);

    impl ToCss for Text {
        fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
            dest.write_str(&self.0)
        }
    }

    fn text(prefix: &str, len: usize) -> Text {
        Text(format!("{prefix}{}", "x".repeat(len - prefix.len())))
    }

    #[test]
    fn same_css_compares_the_text() {
        let long = STACK_TEXT_CAPACITY + 1;
        assert_that!(same_css(&text("a", 8), &text("a", 8))).is_true();
        assert_that!(same_css(&text("a", 8), &text("b", 8))).is_false();
        assert_that!(same_css(&text("a", 8), &text("a", 9))).is_false();
        assert_that!(same_css(&text("a", STACK_TEXT_CAPACITY), &text("a", long))).is_false();
        assert_that!(same_css(&text("a", long), &text("a", long))).is_true();
        assert_that!(same_css(&text("a", long), &text("b", long))).is_false();
    }
}
