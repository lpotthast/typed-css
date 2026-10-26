use crate::checked_value::{impl_checked_value, keyword_enum};

keyword_enum! {
    /// A `position` value.
    ///
    /// [CSS Positioned Layout Level 3 `position` grammar](https://www.w3.org/TR/css-position-3/#position-property)
    pub enum Position {
        Static => "static" as r#static,
        /// The element stays in the flow and is moved by `top`, `left`, and the other offsets.
        Relative => "relative" as relative,
        /// The element leaves the flow and is placed relative to its nearest positioned ancestor.
        Absolute => "absolute" as absolute,
        /// The element leaves the flow and is placed relative to the viewport.
        Fixed => "fixed" as fixed,
        /// The element scrolls with its container until it reaches the offset given by `top`,
        /// `bottom`, `left`, or `right`, and then sticks there.
        Sticky => "sticky" as sticky,
    }
}
impl_checked_value!(Position);
