use std::{
    fmt,
    num::{NonZeroI32, NonZeroU32},
};

use crate::auto::auto_sources;
use crate::checked_value::{impl_checked_value, impl_display, keyword_enum};
use crate::custom_property::declaration_sources;
use crate::length_percentage::length_percentage_sources;
use crate::property::definition::property_keywords;
use crate::serialization::{ToCss, write_spaced};
use crate::{InvalidNumber, NonNegativeLengthPercentage, NonNegativeNumber, number::write_integer};

/// The size of a grid track, such as `200px`, `1fr`, or `auto`.
///
/// [CSS Grid Level 2 track sizing](https://www.w3.org/TR/css-grid-2/#track-sizing)
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum TrackBreadth {
    /// A non-negative length, percentage, or math expression.
    LengthPercentage(NonNegativeLengthPercentage),
    /// A share of the space that is left after the other tracks are sized, written in `fr`.
    Fr(NonNegativeNumber),
    /// `min-content`.
    MinContent,
    /// `max-content`.
    MaxContent,
    /// `auto`.
    Auto,
}

impl TrackBreadth {
    /// Attempt to create a flexible `fr` breadth.
    ///
    /// # Errors
    ///
    /// Returns an error if `value` is negative or not finite.
    pub fn try_fr(value: impl Into<f64>) -> Result<Self, InvalidNumber> {
        NonNegativeNumber::try_new(value.into()).map(Self::Fr)
    }

    const fn is_fixed(&self) -> bool {
        matches!(self, Self::LengthPercentage(_))
    }
}

length_percentage_sources!(@non_negative_conversions_only TrackBreadth, |value| Self::LengthPercentage(value.into()));

impl ToCss for TrackBreadth {
    fn to_css<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match self {
            Self::LengthPercentage(value) => value.to_css(w),
            Self::Fr(value) => {
                value.to_css(w)?;
                w.write_str("fr")
            }
            Self::MinContent => w.write_str("min-content"),
            Self::MaxContent => w.write_str("max-content"),
            Self::Auto => w.write_str("auto"),
        }
    }
}

/// Create a grid track size in `fr`, a share of the space that the other tracks leave free.
///
/// # Panics
///
/// Panics if `value` is negative or not finite. Use [`TrackBreadth::try_fr`] for
/// runtime-derived values.
#[track_caller]
#[must_use]
pub fn fr(value: impl Into<f64>) -> TrackBreadth {
    TrackBreadth::try_fr(value).unwrap_or_else(|error| panic!("{error}"))
}

/// The minimum size of a `minmax()` track. Unlike a [`TrackBreadth`], it cannot be in `fr`.
///
/// [CSS Grid Level 2 track sizing](https://www.w3.org/TR/css-grid-2/#track-sizing)
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum InflexibleBreadth {
    /// A non-negative length, percentage, or math expression.
    LengthPercentage(NonNegativeLengthPercentage),
    /// `min-content`.
    MinContent,
    /// `max-content`.
    MaxContent,
    /// `auto`.
    Auto,
}

length_percentage_sources!(@non_negative_conversions_only InflexibleBreadth, |value| Self::LengthPercentage(
    value.into()
));

impl ToCss for InflexibleBreadth {
    fn to_css<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match self {
            Self::LengthPercentage(value) => value.to_css(w),
            Self::MinContent => w.write_str("min-content"),
            Self::MaxContent => w.write_str("max-content"),
            Self::Auto => w.write_str("auto"),
        }
    }
}

/// The size of a grid track, which is one size, a `minmax()` range, or a `fit-content()` limit.
///
/// ```rust
/// use typed_css::{InflexibleBreadth, TrackSize, fr, px};
///
/// assert_eq!(TrackSize::minmax(px(100), fr(1)).to_string(), "minmax(100px, 1fr)");
/// assert_eq!(TrackSize::fit_content(px(200)).to_string(), "fit-content(200px)");
/// ```
///
/// [CSS Grid Level 2 track sizing](https://www.w3.org/TR/css-grid-2/#track-sizing)
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum TrackSize {
    /// One size.
    Breadth(TrackBreadth),
    /// `minmax(min, max)`.
    MinMax(InflexibleBreadth, TrackBreadth),
    /// `fit-content(limit)`.
    FitContent(NonNegativeLengthPercentage),
}

impl TrackSize {
    /// Create `minmax(min, max)`.
    #[must_use]
    pub fn minmax(min: impl Into<InflexibleBreadth>, max: impl Into<TrackBreadth>) -> Self {
        Self::MinMax(min.into(), max.into())
    }

    /// Create `fit-content(limit)`.
    #[must_use]
    pub fn fit_content(limit: impl Into<NonNegativeLengthPercentage>) -> Self {
        Self::FitContent(limit.into())
    }

    /// Whether this is a `<fixed-size>`, as required inside `repeat(auto-fill, ...)`.
    fn is_fixed(&self) -> bool {
        match self {
            Self::Breadth(breadth) => breadth.is_fixed(),
            Self::MinMax(min, max) => {
                matches!(min, InflexibleBreadth::LengthPercentage(_)) || max.is_fixed()
            }
            Self::FitContent(_) => false,
        }
    }
}

impl From<TrackBreadth> for TrackSize {
    fn from(breadth: TrackBreadth) -> Self {
        Self::Breadth(breadth)
    }
}

length_percentage_sources!(@non_negative_conversions_only TrackSize, |value| Self::Breadth(value.into()));

impl ToCss for TrackSize {
    fn to_css<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match self {
            Self::Breadth(breadth) => breadth.to_css(w),
            Self::MinMax(min, max) => {
                w.write_str("minmax(")?;
                min.to_css(w)?;
                w.write_str(", ")?;
                max.to_css(w)?;
                w.write_char(')')
            }
            Self::FitContent(limit) => {
                w.write_str("fit-content(")?;
                limit.to_css(w)?;
                w.write_char(')')
            }
        }
    }
}

/// Error returned when a grid track list is not valid CSS.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum InvalidGridTracks {
    /// A track list or `repeat()` contains no tracks.
    Empty,
    /// A `repeat()` count is zero.
    ZeroRepeatCount,
    /// A track list contains more than one `repeat(auto-fill | auto-fit, ...)`.
    MultipleAutoRepeats,
    /// A track list with an automatic repetition contains a track that is not a fixed size.
    AutoRepeatRequiresFixedSizes,
}

impl fmt::Display for InvalidGridTracks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Empty => "grid track list must contain at least one track",
            Self::ZeroRepeatCount => "grid repeat() count must be at least 1",
            Self::MultipleAutoRepeats => {
                "grid track list may contain at most one auto-fill or auto-fit repeat()"
            }
            Self::AutoRepeatRequiresFixedSizes => {
                "grid track list with auto-fill or auto-fit repeat() may only contain fixed sizes"
            }
        })
    }
}

impl std::error::Error for InvalidGridTracks {}

/// The repetition count of a grid `repeat()`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum RepeatCount {
    /// A positive integer.
    Count(NonZeroU32),
    /// `auto-fill`.
    AutoFill,
    /// `auto-fit`.
    AutoFit,
}

impl ToCss for RepeatCount {
    fn to_css<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match self {
            Self::Count(count) => write_integer(w, count.get()),
            Self::AutoFill => w.write_str("auto-fill"),
            Self::AutoFit => w.write_str("auto-fit"),
        }
    }
}

/// A grid `repeat(count, tracks)` with a non-empty track list.
///
/// [CSS Grid Level 2 `repeat()`](https://www.w3.org/TR/css-grid-2/#repeat-notation)
#[derive(Clone, Debug, PartialEq)]
pub struct GridRepeat {
    count: RepeatCount,
    tracks: Vec<TrackSize>,
}

impl GridRepeat {
    /// Attempt to repeat `tracks` a fixed number of times.
    ///
    /// # Errors
    ///
    /// Returns an error if `count` is zero or `tracks` is empty.
    pub fn try_count(
        count: u32,
        tracks: impl IntoIterator<Item = impl Into<TrackSize>>,
    ) -> Result<Self, InvalidGridTracks> {
        let count = NonZeroU32::new(count).ok_or(InvalidGridTracks::ZeroRepeatCount)?;
        Self::try_new(RepeatCount::Count(count), tracks)
    }

    /// Repeat `tracks` a fixed number of times.
    ///
    /// # Panics
    ///
    /// Panics if `count` is zero or `tracks` is empty.
    #[track_caller]
    #[must_use]
    pub fn count(count: u32, tracks: impl IntoIterator<Item = impl Into<TrackSize>>) -> Self {
        Self::try_count(count, tracks).unwrap_or_else(|error| panic!("{error}"))
    }

    /// Repeat `tracks` as often as they fit, keeping empty repetitions (`auto-fill`).
    ///
    /// # Panics
    ///
    /// Panics if `tracks` is empty or contains a track that is not a fixed size, such as `1fr`.
    #[track_caller]
    #[must_use]
    pub fn auto_fill(tracks: impl IntoIterator<Item = impl Into<TrackSize>>) -> Self {
        Self::try_new(RepeatCount::AutoFill, tracks).unwrap_or_else(|error| panic!("{error}"))
    }

    /// Repeat `tracks` as often as they fit, collapsing empty repetitions (`auto-fit`).
    ///
    /// # Panics
    ///
    /// Panics if `tracks` is empty or contains a track that is not a fixed size, such as `1fr`.
    #[track_caller]
    #[must_use]
    pub fn auto_fit(tracks: impl IntoIterator<Item = impl Into<TrackSize>>) -> Self {
        Self::try_new(RepeatCount::AutoFit, tracks).unwrap_or_else(|error| panic!("{error}"))
    }

    /// Attempt to repeat `tracks` with any repetition count.
    ///
    /// An automatic repetition, which is `auto-fill` or `auto-fit`, needs a fixed size for each
    /// track, so that the browser can compute how many repetitions fit.
    ///
    /// # Errors
    ///
    /// Returns an error if `tracks` is empty, or if the repetition is automatic and a track is not
    /// a fixed size.
    pub fn try_new(
        count: RepeatCount,
        tracks: impl IntoIterator<Item = impl Into<TrackSize>>,
    ) -> Result<Self, InvalidGridTracks> {
        let tracks: Vec<TrackSize> = tracks.into_iter().map(Into::into).collect();
        if tracks.is_empty() {
            return Err(InvalidGridTracks::Empty);
        }
        let repeat = Self { count, tracks };
        if repeat.is_automatic() && !repeat.tracks.iter().all(TrackSize::is_fixed) {
            return Err(InvalidGridTracks::AutoRepeatRequiresFixedSizes);
        }
        Ok(repeat)
    }

    const fn is_automatic(&self) -> bool {
        matches!(self.count, RepeatCount::AutoFill | RepeatCount::AutoFit)
    }
}

impl ToCss for GridRepeat {
    fn to_css<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        w.write_str("repeat(")?;
        self.count.to_css(w)?;
        w.write_str(", ")?;
        write_spaced(w, &self.tracks)?;
        w.write_char(')')
    }
}

/// One entry of a grid track list, which is a track size or a `repeat()`.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum GridTrack {
    /// One track.
    Size(TrackSize),
    /// A repetition of tracks.
    Repeat(GridRepeat),
}

impl From<TrackSize> for GridTrack {
    fn from(size: TrackSize) -> Self {
        Self::Size(size)
    }
}

impl From<TrackBreadth> for GridTrack {
    fn from(breadth: TrackBreadth) -> Self {
        Self::Size(breadth.into())
    }
}

impl From<GridRepeat> for GridTrack {
    fn from(repeat: GridRepeat) -> Self {
        Self::Repeat(repeat)
    }
}

length_percentage_sources!(@non_negative_conversions_only GridTrack, |value| Self::Size(value.into()));

impl ToCss for GridTrack {
    fn to_css<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match self {
            Self::Size(size) => size.to_css(w),
            Self::Repeat(repeat) => repeat.to_css(w),
        }
    }
}

/// A `grid-template-columns` or `grid-template-rows` value.
///
/// Named grid lines and `subgrid` are not modeled.
///
/// ```rust
/// use typed_css::{GridRepeat, GridTemplate, GridTrack, TrackSize, fr, px};
///
/// let columns = GridTemplate::tracks([GridTrack::from(px(200)), fr(1).into()]);
/// assert_eq!(columns.to_string(), "200px 1fr");
///
/// let cards = GridTemplate::tracks([GridRepeat::auto_fill([TrackSize::minmax(px(160), fr(1))])]);
/// assert_eq!(cards.to_string(), "repeat(auto-fill, minmax(160px, 1fr))");
/// ```
///
/// [CSS Grid Level 2 explicit grid](https://www.w3.org/TR/css-grid-2/#track-sizing)
#[derive(Clone, Debug, PartialEq)]
pub struct GridTemplate(Vec<GridTrack>);

impl GridTemplate {
    /// The `none` keyword.
    #[must_use]
    pub const fn none() -> Self {
        Self(Vec::new())
    }

    /// Create an explicit track list.
    ///
    /// # Panics
    ///
    /// Panics if the list is not valid. [`GridTemplate::try_tracks`] describes the rules.
    #[track_caller]
    #[must_use]
    pub fn tracks(tracks: impl IntoIterator<Item = impl Into<GridTrack>>) -> Self {
        Self::try_tracks(tracks).unwrap_or_else(|error| panic!("{error}"))
    }

    /// Attempt to create an explicit track list.
    ///
    /// # Errors
    ///
    /// Returns an error if the list is empty, contains more than one automatic repetition, or
    /// combines an automatic repetition with a track that is not a fixed size.
    pub fn try_tracks(
        tracks: impl IntoIterator<Item = impl Into<GridTrack>>,
    ) -> Result<Self, InvalidGridTracks> {
        let tracks: Vec<GridTrack> = tracks.into_iter().map(Into::into).collect();
        if tracks.is_empty() {
            return Err(InvalidGridTracks::Empty);
        }
        let automatic = tracks
            .iter()
            .filter(|track| matches!(track, GridTrack::Repeat(repeat) if repeat.is_automatic()))
            .count();
        if automatic > 1 {
            return Err(InvalidGridTracks::MultipleAutoRepeats);
        }
        let all_fixed = tracks.iter().all(|track| match track {
            GridTrack::Size(size) => size.is_fixed(),
            GridTrack::Repeat(repeat) => repeat.tracks.iter().all(TrackSize::is_fixed),
        });
        if automatic == 1 && !all_fixed {
            return Err(InvalidGridTracks::AutoRepeatRequiresFixedSizes);
        }
        Ok(Self(tracks))
    }
}

/// A single track converts into a track list.
impl From<GridTrack> for GridTemplate {
    fn from(track: GridTrack) -> Self {
        Self(vec![track])
    }
}

impl From<TrackSize> for GridTemplate {
    fn from(size: TrackSize) -> Self {
        Self::from(GridTrack::Size(size))
    }
}

impl From<TrackBreadth> for GridTemplate {
    fn from(breadth: TrackBreadth) -> Self {
        Self::from(GridTrack::Size(breadth.into()))
    }
}

impl From<GridRepeat> for GridTemplate {
    fn from(repeat: GridRepeat) -> Self {
        Self::from(GridTrack::Repeat(repeat))
    }
}

declaration_sources!(GridTemplate: GridTrack, TrackSize, TrackBreadth, GridRepeat);
length_percentage_sources!(@non_negative GridTemplate, |value| Self::from(GridTrack::from(value)));

impl ToCss for GridTemplate {
    fn to_css<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        if self.0.is_empty() {
            w.write_str("none")
        } else {
            write_spaced(w, &self.0)
        }
    }
}

/// A `grid-auto-columns` or `grid-auto-rows` value, which is one or more track sizes.
///
/// [CSS Grid Level 2 implicit tracks](https://www.w3.org/TR/css-grid-2/#auto-tracks)
#[derive(Clone, Debug, PartialEq)]
pub struct GridAutoTracks(Vec<TrackSize>);

impl GridAutoTracks {
    /// Create a repeating pattern of implicit track sizes.
    ///
    /// # Panics
    ///
    /// Panics if `tracks` is empty.
    #[track_caller]
    #[must_use]
    pub fn new(tracks: impl IntoIterator<Item = impl Into<TrackSize>>) -> Self {
        Self::try_new(tracks).unwrap_or_else(|error| panic!("{error}"))
    }

    /// Attempt to create a repeating pattern of implicit track sizes.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidGridTracks::Empty`] if `tracks` is empty.
    pub fn try_new(
        tracks: impl IntoIterator<Item = impl Into<TrackSize>>,
    ) -> Result<Self, InvalidGridTracks> {
        let tracks: Vec<TrackSize> = tracks.into_iter().map(Into::into).collect();
        if tracks.is_empty() {
            Err(InvalidGridTracks::Empty)
        } else {
            Ok(Self(tracks))
        }
    }
}

impl From<TrackSize> for GridAutoTracks {
    fn from(size: TrackSize) -> Self {
        Self(vec![size])
    }
}

impl From<TrackBreadth> for GridAutoTracks {
    fn from(breadth: TrackBreadth) -> Self {
        Self(vec![breadth.into()])
    }
}

declaration_sources!(GridAutoTracks: TrackSize, TrackBreadth);
length_percentage_sources!(@non_negative GridAutoTracks, |value| Self(vec![value.into()]));

impl ToCss for GridAutoTracks {
    fn to_css<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        write_spaced(w, &self.0)
    }
}

keyword_enum! {
    /// A `grid-auto-flow` value.
    ///
    /// [CSS Grid Level 2 `grid-auto-flow` grammar](https://www.w3.org/TR/css-grid-2/#grid-auto-flow-property)
    pub enum GridAutoFlow {
        Row => "row" as row,
        Column => "column" as column,
        RowDense => "row dense" as row_dense,
        ColumnDense => "column dense" as column_dense,
    }
}

/// Error returned when a grid line number or span is zero.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InvalidGridLine;

impl fmt::Display for InvalidGridLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("grid line numbers and spans must not be zero")
    }
}

impl std::error::Error for InvalidGridLine {}

/// A `<grid-line>` placement for the grid row and column start and end properties.
///
/// Named lines are not modeled.
///
/// [CSS Grid Level 2 line-based placement](https://www.w3.org/TR/css-grid-2/#line-placement)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum GridLine {
    /// `auto`.
    Auto,
    /// A line number. Negative numbers count from the end of the explicit grid.
    Line(NonZeroI32),
    /// `span n`.
    Span(NonZeroU32),
}

impl GridLine {
    /// Return the range from this line to `end`, written as `start / end`. An `auto` end is left
    /// out, because it is the default.
    #[must_use]
    pub const fn to(self, end: Self) -> GridLineRange {
        GridLineRange::new(self).to(end)
    }

    /// Attempt to place at a line number.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidGridLine`] if `line` is zero.
    pub fn try_line(line: i32) -> Result<Self, InvalidGridLine> {
        NonZeroI32::new(line).map(Self::Line).ok_or(InvalidGridLine)
    }

    /// Place at a line number.
    ///
    /// # Panics
    ///
    /// Panics if `line` is zero.
    #[track_caller]
    #[must_use]
    pub fn line(line: i32) -> Self {
        Self::try_line(line).unwrap_or_else(|error| panic!("{error}"))
    }

    /// Attempt to span a number of tracks.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidGridLine`] if `tracks` is zero.
    pub fn try_span(tracks: u32) -> Result<Self, InvalidGridLine> {
        NonZeroU32::new(tracks)
            .map(Self::Span)
            .ok_or(InvalidGridLine)
    }

    /// Span a number of tracks.
    ///
    /// # Panics
    ///
    /// Panics if `tracks` is zero.
    #[track_caller]
    #[must_use]
    pub fn span(tracks: u32) -> Self {
        Self::try_span(tracks).unwrap_or_else(|error| panic!("{error}"))
    }
}

impl ToCss for GridLine {
    fn to_css<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match self {
            Self::Auto => w.write_str("auto"),
            Self::Line(line) => write_integer(w, line.get()),
            Self::Span(tracks) => {
                w.write_str("span ")?;
                write_integer(w, tracks.get())
            }
        }
    }
}

/// A `grid-row` or `grid-column` shorthand value, which is a start line and an optional end line.
///
/// ```rust
/// use typed_css::{GridLine, GridLineRange};
///
/// let range = GridLineRange::new(GridLine::line(1)).to(GridLine::span(2));
/// assert_eq!(range.to_string(), "1 / span 2");
/// ```
///
/// [CSS Grid Level 2 placement shorthands](https://www.w3.org/TR/css-grid-2/#placement-shorthands)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GridLineRange {
    start: GridLine,
    /// The end line. `None` means `auto`, which spans one track. `Some` never holds `auto`.
    end: Option<GridLine>,
}

impl GridLineRange {
    /// Start at a grid line.
    #[must_use]
    pub const fn new(start: GridLine) -> Self {
        Self { start, end: None }
    }

    /// End at a grid line. An `auto` end is the same as no end.
    #[must_use]
    pub const fn to(mut self, end: GridLine) -> Self {
        self.end = match end {
            GridLine::Auto => None,
            end => Some(end),
        };
        self
    }
}

impl From<GridLine> for GridLineRange {
    fn from(start: GridLine) -> Self {
        Self::new(start)
    }
}

declaration_sources!(GridLineRange: GridLine);

impl ToCss for GridLineRange {
    fn to_css<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        self.start.to_css(w)?;
        // An omitted end line means `auto` unless the start line is a name, and a `GridLine` is
        // never a name.
        if let Some(end) = self.end {
            w.write_str(" / ")?;
            end.to_css(w)?;
        }
        Ok(())
    }
}

auto_sources!(
    @conversions_only
    TrackBreadth => Self::Auto,
    InflexibleBreadth => Self::Auto,
    TrackSize => Self::Breadth(TrackBreadth::Auto),
    GridTrack => Self::Size(TrackSize::Breadth(TrackBreadth::Auto)),
);

auto_sources!(
    GridLine => Self::Auto,
    GridLineRange => Self::new(GridLine::Auto),
    GridTemplate => Self::from(TrackBreadth::Auto),
    GridAutoTracks => Self(vec![TrackBreadth::Auto.into()]),
);

impl_display!(
    TrackBreadth,
    InflexibleBreadth,
    TrackSize,
    RepeatCount,
    GridRepeat,
    GridTrack,
    GridTemplate,
    GridAutoTracks,
    GridLine,
    GridLineRange,
);
impl_checked_value!(
    GridTemplate,
    GridAutoTracks,
    GridAutoFlow,
    GridLine,
    GridLineRange
);

property_keywords! {
    GridTemplate {
        none: "none" => GridTemplate::none(),
        auto: "auto" => TrackBreadth::Auto,
        min_content: "min-content" => TrackBreadth::MinContent,
        max_content: "max-content" => TrackBreadth::MaxContent,
    }
}

property_keywords! {
    GridAutoTracks {
        auto: "auto" => TrackBreadth::Auto,
        min_content: "min-content" => TrackBreadth::MinContent,
        max_content: "max-content" => TrackBreadth::MaxContent,
    }
}

property_keywords! {
    GridLine, GridLineRange {
        auto: "auto" => GridLine::Auto,
    }
}
