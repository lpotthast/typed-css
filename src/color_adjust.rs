use crate::auto::auto_sources;
use crate::checked_value::{impl_checked_value, keyword_enum};

keyword_enum! {
    /// A `forced-color-adjust` value, which says whether the browser may replace the colors of an
    /// element in forced-colors mode, such as a high-contrast theme.
    ///
    /// [CSS Color Adjustment Level 1 `forced-color-adjust`](https://www.w3.org/TR/css-color-adjust-1/#forced-color-adjust-prop)
    pub enum ForcedColorAdjust {
        /// The browser may replace the colors.
        Auto => "auto" as auto,
        /// The browser keeps the colors.
        None => "none" as none,
        /// The element keeps the color of its parent and otherwise behaves like `none`.
        PreserveParentColor => "preserve-parent-color" as preserve_parent_color,
    }
}

keyword_enum! {
    /// A `print-color-adjust` value, which says whether the browser may change colors and
    /// backgrounds to save ink when it prints.
    ///
    /// [CSS Color Adjustment Level 1 `print-color-adjust`](https://www.w3.org/TR/css-color-adjust-1/#print-color-adjust)
    pub enum PrintColorAdjust {
        /// The browser may change colors and drop backgrounds.
        Economy => "economy" as economy,
        /// The browser keeps the colors and backgrounds as specified.
        Exact => "exact" as exact,
    }
}

auto_sources!(ForcedColorAdjust => Self::Auto);

impl_checked_value!(ForcedColorAdjust, PrintColorAdjust);
