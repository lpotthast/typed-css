use crate::checked_value::{impl_checked_value, keyword_enum};

keyword_enum! {
    /// How the lines of a block are aligned along the inline axis.
    ///
    /// `start` and `end` follow the writing direction, while `left` and `right` do not.
    ///
    /// [CSS Text Level 3 `text-align`](https://www.w3.org/TR/css-text-3/#text-align-property)
    pub enum TextAlign {
        /// The lines start at the start edge, which is the left edge in left-to-right text.
        Start => "start" as start,
        /// The lines end at the end edge, which is the right edge in left-to-right text.
        End => "end" as end,
        Left => "left" as left,
        Right => "right" as right,
        Center => "center" as center,
        /// The lines fill the line box, except for the last line.
        Justify => "justify" as justify,
        /// The parent's value, with `start` and `end` resolved against the parent's direction.
        MatchParent => "match-parent" as match_parent,
        /// The lines fill the line box, including the last line.
        JustifyAll => "justify-all" as justify_all,
    }
}

impl_checked_value!(TextAlign);
