use std::fmt;

use crate::serialization::write_spaced_keywords;

use crate::auto::auto_sources;
use crate::checked_value::{impl_checked_value, impl_display, keyword_enum};
use crate::custom_property::declaration_sources;
use crate::length_percentage::length_percentage_sources;
use crate::property::definition::property_keywords;
use crate::{LengthPercentage, serialization::ToCss};

/// A combination of the lines that `text-decoration-line` draws.
///
/// A combination contains at least one line, because every constructor starts with one. The
/// lines are written in the order of the specification, which is `underline`, `overline`, and
/// `line-through`. The deprecated `blink` is not supported.
///
/// ```rust
/// use typed_css::TextDecorationLines;
///
/// let lines = TextDecorationLines::line_through().with_underline();
/// assert_eq!(lines.to_string(), "underline line-through");
/// ```
///
/// [CSS Text Decoration Level 4 `text-decoration-line` grammar](https://www.w3.org/TR/css-text-decor-4/#text-decoration-line-property)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TextDecorationLines {
    underline: bool,
    overline: bool,
    line_through: bool,
}

impl TextDecorationLines {
    /// Start a combination with a line below the text.
    #[must_use]
    pub const fn underline() -> Self {
        Self {
            underline: true,
            overline: false,
            line_through: false,
        }
    }

    /// Start a combination with a line above the text.
    #[must_use]
    pub const fn overline() -> Self {
        Self {
            underline: false,
            overline: true,
            line_through: false,
        }
    }

    /// Start a combination with a line through the middle of the text.
    #[must_use]
    pub const fn line_through() -> Self {
        Self {
            underline: false,
            overline: false,
            line_through: true,
        }
    }

    /// Add a line below the text.
    #[must_use]
    pub const fn with_underline(mut self) -> Self {
        self.underline = true;
        self
    }

    /// Add a line above the text.
    #[must_use]
    pub const fn with_overline(mut self) -> Self {
        self.overline = true;
        self
    }

    /// Add a line through the middle of the text.
    #[must_use]
    pub const fn with_line_through(mut self) -> Self {
        self.line_through = true;
        self
    }
}

impl ToCss for TextDecorationLines {
    fn to_css<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        let lines = [
            (self.underline, "underline"),
            (self.overline, "overline"),
            (self.line_through, "line-through"),
        ];
        write_spaced_keywords(
            w,
            lines
                .into_iter()
                .filter_map(|(drawn, keyword)| drawn.then_some(keyword)),
        )
    }
}

/// A `text-decoration-line` value.
///
/// [CSS Text Decoration Level 4 `text-decoration-line` grammar](https://www.w3.org/TR/css-text-decor-4/#text-decoration-line-property)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TextDecorationLine {
    /// `none`.
    None,
    /// A combination of `underline`, `overline`, and `line-through`.
    Lines(TextDecorationLines),
    /// `spelling-error`. The browser draws its style for misspelled text.
    SpellingError,
    /// `grammar-error`. The browser draws its style for grammar mistakes.
    GrammarError,
}

impl From<TextDecorationLines> for TextDecorationLine {
    fn from(value: TextDecorationLines) -> Self {
        Self::Lines(value)
    }
}

declaration_sources!(TextDecorationLine: TextDecorationLines);

impl ToCss for TextDecorationLine {
    fn to_css<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match self {
            Self::None => w.write_str("none"),
            Self::Lines(lines) => lines.to_css(w),
            Self::SpellingError => w.write_str("spelling-error"),
            Self::GrammarError => w.write_str("grammar-error"),
        }
    }
}

keyword_enum! {
    /// How the browser draws the lines of `text-decoration-line`.
    ///
    /// [CSS Text Decoration Level 4 `text-decoration-style`](https://www.w3.org/TR/css-text-decor-4/#text-decoration-style-property)
    pub enum TextDecorationStyle {
        Solid => "solid" as solid,
        Double => "double" as double,
        Dotted => "dotted" as dotted,
        Dashed => "dashed" as dashed,
        Wavy => "wavy" as wavy,
    }
}

macro_rules! decoration_length {
    (
        $(#[$meta:meta])*
        $name:ident { $($(#[$keyword_meta:meta])* $keyword:ident => $css:literal),+ $(,)? }
    ) => {
        $(#[$meta])*
        #[derive(Clone, Debug, PartialEq)]
        #[non_exhaustive]
        pub enum $name {
            $($(#[$keyword_meta])* $keyword,)+
            /// A length, a percentage of the font size, or a math expression.
            LengthPercentage(LengthPercentage),
        }

        length_percentage_sources!($name, |value| Self::LengthPercentage(value));

        impl ToCss for $name {
            fn to_css<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
                match self {
                    $(Self::$keyword => w.write_str($css),)+
                    Self::LengthPercentage(value) => value.to_css(w),
                }
            }
        }
    };
}

decoration_length! {
    /// A `text-decoration-thickness` value, which is the stroke width of the decoration lines.
    ///
    /// The type follows the editor's draft, which accepts any `<length-percentage>`.
    ///
    /// [CSS Text Decoration Level 4 `text-decoration-thickness`](https://drafts.csswg.org/css-text-decor-4/#text-decoration-width-property)
    TextDecorationThickness {
        /// The `auto` keyword. The browser chooses the thickness.
        Auto => "auto",
        /// The `from-font` keyword. The font chooses the thickness, if it says one.
        FromFont => "from-font",
    }
}

decoration_length! {
    /// A `text-underline-offset` value, which is the distance of an underline from its usual
    /// position. A positive offset moves the underline away from the text.
    ///
    /// The type follows the editor's draft, which accepts any `<length-percentage>`.
    ///
    /// [CSS Text Decoration Level 4 `text-underline-offset`](https://drafts.csswg.org/css-text-decor-4/#underline-offset)
    TextUnderlineOffset {
        /// The `auto` keyword. The browser chooses the offset.
        Auto => "auto",
    }
}

auto_sources!(
    TextDecorationThickness => Self::Auto,
    TextUnderlineOffset => Self::Auto,
);

impl_display!(
    TextDecorationLines,
    TextDecorationLine,
    TextDecorationThickness,
    TextUnderlineOffset
);
impl_checked_value!(
    TextDecorationLine,
    TextDecorationStyle,
    TextDecorationThickness,
    TextUnderlineOffset
);

property_keywords! {
    TextDecorationLine {
        none: "none" => TextDecorationLine::None,
        underline: "underline" => TextDecorationLines::underline(),
        overline: "overline" => TextDecorationLines::overline(),
        line_through: "line-through" => TextDecorationLines::line_through(),
        spelling_error: "spelling-error" => TextDecorationLine::SpellingError,
        grammar_error: "grammar-error" => TextDecorationLine::GrammarError,
    }
}

property_keywords! {
    TextDecorationThickness {
        auto: "auto" => TextDecorationThickness::Auto,
        from_font: "from-font" => TextDecorationThickness::FromFont,
    }
}

property_keywords! {
    TextUnderlineOffset {
        auto: "auto" => TextUnderlineOffset::Auto,
    }
}
