use std::fmt;

use crate::checked_value::{impl_checked_value, impl_display, keyword_enum};
use crate::custom_property::declaration_sources;
use crate::{
    Angle, InvalidNumber, NonNegativeNumber, Number, PercentageChannel, UnitInterval, Var,
    number::{round_noise, try_finite, write_integer},
    serialization::ToCss,
};

keyword_enum! {
    /// A named color of CSS, such as `rebeccapurple`, or the keywords `transparent` and
    /// `currentcolor`.
    ///
    /// [CSS Color Level 4 named colors](https://www.w3.org/TR/css-color-4/#named-colors)
    pub enum NamedColor {
        Transparent => "transparent",
        /// The value of the element's `color` property.
        CurrentColor => "currentcolor",
        AliceBlue => "aliceblue",
        AntiqueWhite => "antiquewhite",
        Aqua => "aqua",
        Aquamarine => "aquamarine",
        Azure => "azure",
        Beige => "beige",
        Bisque => "bisque",
        Black => "black",
        BlanchedAlmond => "blanchedalmond",
        Blue => "blue",
        BlueViolet => "blueviolet",
        Brown => "brown",
        Burlywood => "burlywood",
        CadetBlue => "cadetblue",
        Chartreuse => "chartreuse",
        Chocolate => "chocolate",
        Coral => "coral",
        CornflowerBlue => "cornflowerblue",
        Cornsilk => "cornsilk",
        Crimson => "crimson",
        Cyan => "cyan",
        DarkBlue => "darkblue",
        DarkCyan => "darkcyan",
        DarkGoldenrod => "darkgoldenrod",
        DarkGray => "darkgray",
        DarkGreen => "darkgreen",
        DarkGrey => "darkgrey",
        DarkKhaki => "darkkhaki",
        DarkMagenta => "darkmagenta",
        DarkOliveGreen => "darkolivegreen",
        DarkOrange => "darkorange",
        DarkOrchid => "darkorchid",
        DarkRed => "darkred",
        DarkSalmon => "darksalmon",
        DarkSeaGreen => "darkseagreen",
        DarkSlateBlue => "darkslateblue",
        DarkSlateGray => "darkslategray",
        DarkSlateGrey => "darkslategrey",
        DarkTurquoise => "darkturquoise",
        DarkViolet => "darkviolet",
        DeepPink => "deeppink",
        DeepSkyBlue => "deepskyblue",
        DimGray => "dimgray",
        DimGrey => "dimgrey",
        DodgerBlue => "dodgerblue",
        Firebrick => "firebrick",
        FloralWhite => "floralwhite",
        ForestGreen => "forestgreen",
        Fuchsia => "fuchsia",
        Gainsboro => "gainsboro",
        GhostWhite => "ghostwhite",
        Gold => "gold",
        Goldenrod => "goldenrod",
        Gray => "gray",
        Green => "green",
        GreenYellow => "greenyellow",
        Grey => "grey",
        Honeydew => "honeydew",
        HotPink => "hotpink",
        IndianRed => "indianred",
        Indigo => "indigo",
        Ivory => "ivory",
        Khaki => "khaki",
        Lavender => "lavender",
        LavenderBlush => "lavenderblush",
        LawnGreen => "lawngreen",
        LemonChiffon => "lemonchiffon",
        LightBlue => "lightblue",
        LightCoral => "lightcoral",
        LightCyan => "lightcyan",
        LightGoldenrodYellow => "lightgoldenrodyellow",
        LightGray => "lightgray",
        LightGreen => "lightgreen",
        LightGrey => "lightgrey",
        LightPink => "lightpink",
        LightSalmon => "lightsalmon",
        LightSeaGreen => "lightseagreen",
        LightSkyBlue => "lightskyblue",
        LightSlateGray => "lightslategray",
        LightSlateGrey => "lightslategrey",
        LightSteelBlue => "lightsteelblue",
        LightYellow => "lightyellow",
        Lime => "lime",
        LimeGreen => "limegreen",
        Linen => "linen",
        Magenta => "magenta",
        Maroon => "maroon",
        MediumAquamarine => "mediumaquamarine",
        MediumBlue => "mediumblue",
        MediumOrchid => "mediumorchid",
        MediumPurple => "mediumpurple",
        MediumSeaGreen => "mediumseagreen",
        MediumSlateBlue => "mediumslateblue",
        MediumSpringGreen => "mediumspringgreen",
        MediumTurquoise => "mediumturquoise",
        MediumVioletRed => "mediumvioletred",
        MidnightBlue => "midnightblue",
        MintCream => "mintcream",
        MistyRose => "mistyrose",
        Moccasin => "moccasin",
        NavajoWhite => "navajowhite",
        Navy => "navy",
        OldLace => "oldlace",
        Olive => "olive",
        OliveDrab => "olivedrab",
        Orange => "orange",
        OrangeRed => "orangered",
        Orchid => "orchid",
        PaleGoldenrod => "palegoldenrod",
        PaleGreen => "palegreen",
        PaleTurquoise => "paleturquoise",
        PaleVioletRed => "palevioletred",
        PapayaWhip => "papayawhip",
        PeachPuff => "peachpuff",
        Peru => "peru",
        Pink => "pink",
        Plum => "plum",
        PowderBlue => "powderblue",
        Purple => "purple",
        RebeccaPurple => "rebeccapurple",
        Red => "red",
        RosyBrown => "rosybrown",
        RoyalBlue => "royalblue",
        SaddleBrown => "saddlebrown",
        Salmon => "salmon",
        SandyBrown => "sandybrown",
        SeaGreen => "seagreen",
        Seashell => "seashell",
        Sienna => "sienna",
        Silver => "silver",
        SkyBlue => "skyblue",
        SlateBlue => "slateblue",
        SlateGray => "slategray",
        SlateGrey => "slategrey",
        Snow => "snow",
        SpringGreen => "springgreen",
        SteelBlue => "steelblue",
        Tan => "tan",
        Teal => "teal",
        Thistle => "thistle",
        Tomato => "tomato",
        Turquoise => "turquoise",
        Violet => "violet",
        Wheat => "wheat",
        White => "white",
        WhiteSmoke => "whitesmoke",
        Yellow => "yellow",
        YellowGreen => "yellowgreen",
    }
}

keyword_enum! {
    /// The color space in which `color-mix()` mixes two colors.
    ///
    /// Mixing in a perceptual space such as [`ColorSpace::Oklch`] usually gives the most even
    /// results.
    ///
    /// [CSS Color Level 5 color interpolation](https://www.w3.org/TR/css-color-5/#color-mix)
    pub enum ColorSpace {
        Srgb => "srgb",
        SrgbLinear => "srgb-linear",
        DisplayP3 => "display-p3",
        A98Rgb => "a98-rgb",
        ProphotoRgb => "prophoto-rgb",
        Rec2020 => "rec2020",
        Lab => "lab",
        Oklab => "oklab",
        Xyz => "xyz",
        XyzD50 => "xyz-d50",
        XyzD65 => "xyz-d65",
        Hsl => "hsl",
        Hwb => "hwb",
        Lch => "lch",
        Oklch => "oklch",
    }
}

keyword_enum! {
    /// A polar color space, which describes a color by a hue angle. Only these spaces accept a
    /// [`HueInterpolationMethod`].
    ///
    /// [CSS Color Level 4 color interpolation](https://www.w3.org/TR/css-color-4/#interpolation-space)
    pub enum PolarColorSpace {
        Hsl => "hsl",
        Hwb => "hwb",
        Lch => "lch",
        Oklch => "oklch",
    }
}

impl PolarColorSpace {
    const fn color_space(self) -> ColorSpace {
        match self {
            Self::Hsl => ColorSpace::Hsl,
            Self::Hwb => ColorSpace::Hwb,
            Self::Lch => ColorSpace::Lch,
            Self::Oklch => ColorSpace::Oklch,
        }
    }
}

impl From<PolarColorSpace> for ColorSpace {
    fn from(space: PolarColorSpace) -> Self {
        space.color_space()
    }
}

keyword_enum! {
    /// The way to interpolate between two hues, which are two points on the color wheel.
    ///
    /// [CSS Color Level 4 hue interpolation](https://www.w3.org/TR/css-color-4/#hue-interpolation)
    pub enum HueInterpolationMethod {
        /// Take the shorter arc between the hues. This is the default.
        Shorter => "shorter",
        /// Take the longer arc between the hues.
        Longer => "longer",
        /// Move to larger hue angles.
        Increasing => "increasing",
        /// Move to smaller hue angles.
        Decreasing => "decreasing",
    }
}

/// A `<color-interpolation-method>`, which is a color space and, for a polar color space, an
/// optional way to interpolate hues.
///
/// A [`ColorSpace`] and a [`PolarColorSpace`] convert into a method without a hue interpolation.
///
/// ```rust
/// use typed_css::{
///     ColorInterpolationMethod, HueInterpolationMethod, NamedColor, PolarColorSpace, color_mix,
/// };
///
/// let method = ColorInterpolationMethod::with_hue(
///     PolarColorSpace::Oklch,
///     HueInterpolationMethod::Longer,
/// );
/// assert_eq!(
///     color_mix(method, NamedColor::Red, NamedColor::Blue).to_string(),
///     "color-mix(in oklch longer hue, red, blue)",
/// );
/// ```
///
/// [CSS Color Level 4 color interpolation](https://www.w3.org/TR/css-color-4/#color-interpolation-method)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ColorInterpolationMethod {
    space: ColorSpace,
    hue: Option<HueInterpolationMethod>,
}

impl ColorInterpolationMethod {
    /// Interpolate in a polar color space with the given hue interpolation.
    #[must_use]
    pub const fn with_hue(space: PolarColorSpace, hue: HueInterpolationMethod) -> Self {
        Self {
            space: space.color_space(),
            hue: Some(hue),
        }
    }
}

impl From<ColorSpace> for ColorInterpolationMethod {
    fn from(space: ColorSpace) -> Self {
        Self { space, hue: None }
    }
}

impl From<PolarColorSpace> for ColorInterpolationMethod {
    fn from(space: PolarColorSpace) -> Self {
        Self::from(ColorSpace::from(space))
    }
}

impl ToCss for ColorInterpolationMethod {
    fn to_css<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        w.write_str("in ")?;
        self.space.to_css(w)?;
        if let Some(hue) = self.hue {
            w.write_char(' ')?;
            hue.to_css(w)?;
            w.write_str(" hue")?;
        }
        Ok(())
    }
}

keyword_enum! {
    /// A system color, which the browser takes from the user's operating system or theme.
    ///
    /// System colors keep a page readable in forced-colors mode, where the browser replaces other
    /// colors. Use them in pairs, such as `Canvas` with `CanvasText`.
    ///
    /// [CSS Color Level 4 system colors](https://www.w3.org/TR/css-color-4/#css-system-colors)
    pub enum SystemColor {
        /// The background of accented user interface controls.
        AccentColor => "AccentColor",
        /// Text on a background of `AccentColor`.
        AccentColorText => "AccentColorText",
        /// Text of an active link.
        ActiveText => "ActiveText",
        /// The border of a button.
        ButtonBorder => "ButtonBorder",
        /// The background of a button.
        ButtonFace => "ButtonFace",
        /// Text on a button.
        ButtonText => "ButtonText",
        /// The background of the page.
        Canvas => "Canvas",
        /// Text on the page.
        CanvasText => "CanvasText",
        /// The background of an input field.
        Field => "Field",
        /// Text in an input field.
        FieldText => "FieldText",
        /// Text that is disabled.
        GrayText => "GrayText",
        /// The background of selected text.
        Highlight => "Highlight",
        /// Selected text.
        HighlightText => "HighlightText",
        /// Text of a link that has not been visited.
        LinkText => "LinkText",
        /// The background of marked text, such as a search result.
        Mark => "Mark",
        /// Marked text.
        MarkText => "MarkText",
        /// The background of a selected item, such as a selected option.
        SelectedItem => "SelectedItem",
        /// Text of a selected item.
        SelectedItemText => "SelectedItemText",
        /// Text of a visited link.
        VisitedText => "VisitedText",
    }
}

/// A CSS `<color>` from CSS Color Levels 4 and 5.
///
/// Color functions are written in the modern space-separated syntax, such as `rgb(255 0 0 / 0.5)`.
/// An `alpha` of `None` means a fully opaque color. A color can also be a `var()` reference, so
/// that `light_dark` and `color_mix` can combine custom properties.
///
/// ```rust
/// use typed_css::{ColorSpace, NamedColor, color_mix, deg, hex, light_dark, oklch, rgba};
///
/// assert_eq!(hex(0xff8800).to_string(), "rgb(255 136 0)");
/// assert_eq!(rgba(0, 0, 0, 0.5).to_string(), "rgb(0 0 0 / 0.5)");
/// assert_eq!(oklch(0.7, 0.15, deg(250)).to_string(), "oklch(0.7 0.15 250deg)");
/// assert_eq!(
///     light_dark(NamedColor::White, NamedColor::Black).to_string(),
///     "light-dark(white, black)",
/// );
/// assert_eq!(
///     color_mix(ColorSpace::Oklch, hex(0xff0000), hex(0x0000ff)).to_string(),
///     "color-mix(in oklch, rgb(255 0 0), rgb(0 0 255))",
/// );
/// ```
///
/// [CSS Color Level 4 color syntax](https://www.w3.org/TR/css-color-4/#color-syntax)
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum Color {
    /// An sRGB color, written as `rgb(red green blue / alpha)`.
    Rgb {
        /// The red channel, from 0 to 255.
        red: u8,
        /// The green channel, from 0 to 255.
        green: u8,
        /// The blue channel, from 0 to 255.
        blue: u8,
        /// The opacity, from `0` (transparent) to `1` (opaque). `None` means opaque.
        alpha: Option<UnitInterval>,
    },
    /// An HSL color, written as `hsl(hue saturation lightness / alpha)`.
    Hsl {
        /// The hue. CSS wraps any angle around the color wheel.
        hue: Angle,
        /// The saturation, from `0%` (gray) to `100%` (fully saturated).
        saturation: PercentageChannel,
        /// The lightness, from `0%` (black) to `100%` (white).
        lightness: PercentageChannel,
        /// The opacity, from `0` (transparent) to `1` (opaque). `None` means opaque.
        alpha: Option<UnitInterval>,
    },
    /// An HWB color, written as `hwb(hue whiteness blackness / alpha)`.
    Hwb {
        /// The hue. CSS wraps any angle around the color wheel.
        hue: Angle,
        /// The amount of white mixed in, from `0%` to `100%`.
        whiteness: PercentageChannel,
        /// The amount of black mixed in, from `0%` to `100%`.
        blackness: PercentageChannel,
        /// The opacity, from `0` (transparent) to `1` (opaque). `None` means opaque.
        alpha: Option<UnitInterval>,
    },
    /// An Oklab color, written as `oklab(lightness a b / alpha)`.
    Oklab {
        /// Perceptual lightness from `0` (black) to `1` (white).
        lightness: UnitInterval,
        /// The position between green (negative) and red (positive), usually from about `-0.4` to
        /// `0.4`.
        a: Number,
        /// The position between blue (negative) and yellow (positive), usually from about `-0.4`
        /// to `0.4`.
        b: Number,
        /// The opacity, from `0` (transparent) to `1` (opaque). `None` means opaque.
        alpha: Option<UnitInterval>,
    },
    /// An Oklch color, written as `oklch(lightness chroma hue / alpha)`.
    Oklch {
        /// Perceptual lightness from `0` (black) to `1` (white).
        lightness: UnitInterval,
        /// The colorfulness, from `0` (gray) upwards. Visible colors rarely exceed about `0.4`.
        chroma: NonNegativeNumber,
        /// The hue. CSS wraps any angle around the color wheel.
        hue: Angle,
        /// The opacity, from `0` (transparent) to `1` (opaque). `None` means opaque.
        alpha: Option<UnitInterval>,
    },
    /// A named color, `transparent`, or `currentcolor`.
    Named(NamedColor),
    /// A system color.
    System(SystemColor),
    /// `light-dark(light, dark)`, chosen by the element's used color scheme.
    LightDark(Box<Color>, Box<Color>),
    /// A `color-mix()` of two colors.
    Mix(Box<ColorMix>),
    /// A `var()` reference to a color custom property.
    Var(Var<Color>),
}

impl From<NamedColor> for Color {
    fn from(name: NamedColor) -> Self {
        Self::Named(name)
    }
}

impl From<SystemColor> for Color {
    fn from(color: SystemColor) -> Self {
        Self::System(color)
    }
}

impl From<Var<Color>> for Color {
    fn from(variable: Var<Color>) -> Self {
        Self::Var(variable)
    }
}

declaration_sources!(Color: NamedColor, SystemColor);

/// A `color-mix()` of two colors with one color interpolation method.
///
/// The optional weight is the percentage of the first color. The second color makes up the rest.
///
/// [CSS Color Level 5 `color-mix()`](https://www.w3.org/TR/css-color-5/#color-mix)
#[derive(Clone, Debug, PartialEq)]
pub struct ColorMix {
    method: ColorInterpolationMethod,
    first: Color,
    first_weight: Option<PercentageChannel>,
    second: Color,
}

fn write_alpha<W: fmt::Write>(w: &mut W, alpha: Option<UnitInterval>) -> fmt::Result {
    match alpha {
        Some(alpha) => {
            w.write_str(" / ")?;
            alpha.to_css(w)?;
            w.write_char(')')
        }
        None => w.write_char(')'),
    }
}

/// Write a color function with three channels and an optional alpha, such as `hsl(200deg 50% 40%)`.
fn write_color_function<W: fmt::Write>(
    w: &mut W,
    name: &str,
    channels: (&impl ToCss, &impl ToCss, &impl ToCss),
    alpha: Option<UnitInterval>,
) -> fmt::Result {
    w.write_str(name)?;
    w.write_char('(')?;
    channels.0.to_css(w)?;
    w.write_char(' ')?;
    channels.1.to_css(w)?;
    w.write_char(' ')?;
    channels.2.to_css(w)?;
    write_alpha(w, alpha)
}

impl ToCss for Color {
    fn to_css<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        match self {
            Self::Rgb {
                red,
                green,
                blue,
                alpha,
            } => {
                w.write_str("rgb(")?;
                write_integer(w, *red)?;
                w.write_char(' ')?;
                write_integer(w, *green)?;
                w.write_char(' ')?;
                write_integer(w, *blue)?;
                write_alpha(w, *alpha)
            }
            Self::Hsl {
                hue,
                saturation,
                lightness,
                alpha,
            } => write_color_function(w, "hsl", (hue, saturation, lightness), *alpha),
            Self::Hwb {
                hue,
                whiteness,
                blackness,
                alpha,
            } => write_color_function(w, "hwb", (hue, whiteness, blackness), *alpha),
            Self::Oklab {
                lightness,
                a,
                b,
                alpha,
            } => write_color_function(w, "oklab", (lightness, a, b), *alpha),
            Self::Oklch {
                lightness,
                chroma,
                hue,
                alpha,
            } => write_color_function(w, "oklch", (lightness, chroma, hue), *alpha),
            Self::Named(name) => name.to_css(w),
            Self::System(color) => color.to_css(w),
            Self::Var(variable) => variable.to_css(w),
            Self::LightDark(light, dark) => {
                w.write_str("light-dark(")?;
                light.to_css(w)?;
                w.write_str(", ")?;
                dark.to_css(w)?;
                w.write_char(')')
            }
            Self::Mix(mix) => mix.to_css(w),
        }
    }
}

impl ToCss for ColorMix {
    fn to_css<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        w.write_str("color-mix(")?;
        self.method.to_css(w)?;
        w.write_str(", ")?;
        self.first.to_css(w)?;
        if let Some(weight) = self.first_weight {
            w.write_char(' ')?;
            weight.to_css(w)?;
        }
        w.write_str(", ")?;
        self.second.to_css(w)?;
        w.write_char(')')
    }
}

impl Color {
    /// Attempt to create an sRGB color from a `0xRRGGBB` number.
    ///
    /// # Errors
    ///
    /// Returns an error if `value` is larger than `0xFFFFFF`.
    pub const fn try_hex(value: u32) -> Result<Self, InvalidNumber> {
        if value > 0x00FF_FFFF {
            return Err(InvalidNumber::OutOfRange {
                value: value as f64,
                context: "hex color",
                min: 0.0,
                max: 16_777_215.0,
            });
        }
        Ok(hex(value))
    }

    /// Attempt to create an sRGB color with alpha.
    ///
    /// # Errors
    ///
    /// Returns an error if `alpha` is `NaN`, infinite, or outside `[0.0, 1.0]`.
    pub fn try_rgba(
        red: u8,
        green: u8,
        blue: u8,
        alpha: impl Into<f64>,
    ) -> Result<Self, InvalidNumber> {
        Ok(Self::Rgb {
            red,
            green,
            blue,
            alpha: Some(UnitInterval::try_new(alpha.into())?),
        })
    }

    /// Attempt to create an HSL color.
    ///
    /// # Errors
    ///
    /// Returns an error if `saturation` or `lightness` is outside `[0.0, 100.0]`.
    pub fn try_hsl(
        hue: Angle,
        saturation: impl Into<f64>,
        lightness: impl Into<f64>,
    ) -> Result<Self, InvalidNumber> {
        Ok(Self::Hsl {
            hue,
            saturation: PercentageChannel::try_new(saturation.into())?,
            lightness: PercentageChannel::try_new(lightness.into())?,
            alpha: None,
        })
    }

    /// Attempt to create an HSL color with alpha.
    ///
    /// # Errors
    ///
    /// Returns an error if any channel is non-finite or outside its documented range.
    pub fn try_hsla(
        hue: Angle,
        saturation: impl Into<f64>,
        lightness: impl Into<f64>,
        alpha: impl Into<f64>,
    ) -> Result<Self, InvalidNumber> {
        Ok(Self::Hsl {
            hue,
            saturation: PercentageChannel::try_new(saturation.into())?,
            lightness: PercentageChannel::try_new(lightness.into())?,
            alpha: Some(UnitInterval::try_new(alpha.into())?),
        })
    }

    /// Attempt to create an HWB color.
    ///
    /// # Errors
    ///
    /// Returns an error if `whiteness` or `blackness` is outside `[0.0, 100.0]`.
    pub fn try_hwb(
        hue: Angle,
        whiteness: impl Into<f64>,
        blackness: impl Into<f64>,
    ) -> Result<Self, InvalidNumber> {
        Ok(Self::Hwb {
            hue,
            whiteness: PercentageChannel::try_new(whiteness.into())?,
            blackness: PercentageChannel::try_new(blackness.into())?,
            alpha: None,
        })
    }

    /// Attempt to create an Oklab color.
    ///
    /// # Errors
    ///
    /// Returns an error if `lightness` is outside `[0.0, 1.0]` or any channel is not finite.
    pub fn try_oklab(
        lightness: impl Into<f64>,
        a: impl Into<f64>,
        b: impl Into<f64>,
    ) -> Result<Self, InvalidNumber> {
        Ok(Self::Oklab {
            lightness: UnitInterval::try_new(lightness.into())?,
            a: try_finite(a.into(), "oklab a")?,
            b: try_finite(b.into(), "oklab b")?,
            alpha: None,
        })
    }

    /// Attempt to create an Oklch color.
    ///
    /// # Errors
    ///
    /// Returns an error if `lightness` is outside `[0.0, 1.0]`, or if `chroma` is negative or not
    /// finite.
    pub fn try_oklch(
        lightness: impl Into<f64>,
        chroma: impl Into<f64>,
        hue: Angle,
    ) -> Result<Self, InvalidNumber> {
        Ok(Self::Oklch {
            lightness: UnitInterval::try_new(lightness.into())?,
            chroma: NonNegativeNumber::try_new(chroma.into())?,
            hue,
            alpha: None,
        })
    }

    /// Attempt to mix two colors, giving `first_weight` percent of the result to the first color.
    ///
    /// # Errors
    ///
    /// Returns an error if `first_weight` is outside `[0.0, 100.0]`.
    pub fn try_color_mix_weighted(
        method: impl Into<ColorInterpolationMethod>,
        first: impl Into<Color>,
        first_weight: impl Into<f64>,
        second: impl Into<Color>,
    ) -> Result<Self, InvalidNumber> {
        Ok(Self::Mix(Box::new(ColorMix {
            method: method.into(),
            first: first.into(),
            first_weight: Some(PercentageChannel::try_new(first_weight.into())?),
            second: second.into(),
        })))
    }

    /// Return this color with the opacity `alpha`, from `0` (transparent) to `1` (opaque).
    ///
    /// A color function such as `rgb()` gets `alpha` as its alpha channel, which replaces the old
    /// one. Each color of `light-dark()` is changed the same way. Any other color, such as a named
    /// color or a `var()` reference, is mixed with `transparent`. The mix multiplies the opacity
    /// of the color by `alpha`, so a color that is already translucent ends up more transparent
    /// than `alpha`.
    ///
    /// ```rust
    /// use typed_css::{NamedColor, rgb, Color};
    ///
    /// assert_eq!(rgb(0, 0, 0).with_alpha(0.5).to_string(), "rgb(0 0 0 / 0.5)");
    /// assert_eq!(
    ///     Color::from(NamedColor::Red).with_alpha(0.25).to_string(),
    ///     "color-mix(in srgb, red 25%, transparent)",
    /// );
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `alpha` is outside `0..=1` or not finite. Use [`Color::try_with_alpha`] for
    /// runtime-derived values.
    #[must_use]
    #[track_caller]
    pub fn with_alpha(self, alpha: impl Into<f64>) -> Self {
        self.try_with_alpha(alpha)
            .unwrap_or_else(|error| panic!("{error}"))
    }

    /// Attempt to return this color with the opacity `alpha`.
    ///
    /// # Errors
    ///
    /// Returns an error if `alpha` is outside `0..=1` or not finite.
    pub fn try_with_alpha(self, alpha: impl Into<f64>) -> Result<Self, InvalidNumber> {
        Ok(self.with_unit_alpha(UnitInterval::try_new(alpha.into())?))
    }

    fn with_unit_alpha(self, alpha: UnitInterval) -> Self {
        let alpha_channel = Some(alpha);
        match self {
            Self::Rgb {
                red, green, blue, ..
            } => Self::Rgb {
                red,
                green,
                blue,
                alpha: alpha_channel,
            },
            Self::Hsl {
                hue,
                saturation,
                lightness,
                ..
            } => Self::Hsl {
                hue,
                saturation,
                lightness,
                alpha: alpha_channel,
            },
            Self::Hwb {
                hue,
                whiteness,
                blackness,
                ..
            } => Self::Hwb {
                hue,
                whiteness,
                blackness,
                alpha: alpha_channel,
            },
            Self::Oklab {
                lightness, a, b, ..
            } => Self::Oklab {
                lightness,
                a,
                b,
                alpha: alpha_channel,
            },
            Self::Oklch {
                lightness,
                chroma,
                hue,
                ..
            } => Self::Oklch {
                lightness,
                chroma,
                hue,
                alpha: alpha_channel,
            },
            Self::LightDark(light, dark) => Self::LightDark(
                Box::new(light.with_unit_alpha(alpha)),
                Box::new(dark.with_unit_alpha(alpha)),
            ),
            other @ (Self::Named(_) | Self::System(_) | Self::Mix(_) | Self::Var(_)) => {
                Self::Mix(Box::new(ColorMix {
                    method: ColorSpace::Srgb.into(),
                    first: other,
                    first_weight: Some(PercentageChannel(Number::new(round_noise(
                        alpha.value() * 100.0,
                    )))),
                    second: Self::Named(NamedColor::Transparent),
                }))
            }
        }
    }
}

/// Create an sRGB color.
#[must_use]
pub const fn rgb(red: u8, green: u8, blue: u8) -> Color {
    Color::Rgb {
        red,
        green,
        blue,
        alpha: None,
    }
}

/// Create an sRGB color from a `0xRRGGBB` number, such as `hex(0x1e90ff)`.
///
/// # Panics
///
/// Panics, or fails const evaluation, if `value` is larger than `0xFFFFFF`. Use
/// [`Color::try_hex`] for runtime-derived values.
#[track_caller]
#[must_use]
pub const fn hex(value: u32) -> Color {
    assert!(value <= 0x00FF_FFFF, "hex color must be at most 0xFFFFFF");
    let [_, red, green, blue] = value.to_be_bytes();
    rgb(red, green, blue)
}

/// Create an sRGB color with alpha.
///
/// # Panics
///
/// Panics if `alpha` is `NaN`, infinite, or outside `[0.0, 1.0]`. Use [`Color::try_rgba`] for
/// runtime-derived values.
#[track_caller]
#[must_use]
pub fn rgba(red: u8, green: u8, blue: u8, alpha: impl Into<f64>) -> Color {
    Color::try_rgba(red, green, blue, alpha).unwrap_or_else(|error| panic!("{error}"))
}

/// Create an HSL color.
///
/// CSS wraps any `hue` around the color wheel. `saturation` and `lightness` are numbers of percent
/// from 0 to 100.
///
/// # Panics
///
/// Panics if a channel is non-finite or outside its range. Use [`Color::try_hsl`] for
/// runtime-derived values.
#[track_caller]
#[must_use]
pub fn hsl(hue: Angle, saturation: impl Into<f64>, lightness: impl Into<f64>) -> Color {
    Color::try_hsl(hue, saturation, lightness).unwrap_or_else(|error| panic!("{error}"))
}

/// Create an HSL color with alpha.
///
/// # Panics
///
/// Panics if a channel is non-finite or outside its range. Use [`Color::try_hsla`] for
/// runtime-derived values.
#[track_caller]
#[must_use]
pub fn hsla(
    hue: Angle,
    saturation: impl Into<f64>,
    lightness: impl Into<f64>,
    alpha: impl Into<f64>,
) -> Color {
    Color::try_hsla(hue, saturation, lightness, alpha).unwrap_or_else(|error| panic!("{error}"))
}

/// Create an HWB color.
///
/// # Panics
///
/// Panics if a channel is non-finite or outside its range. Use [`Color::try_hwb`] for
/// runtime-derived values.
#[track_caller]
#[must_use]
pub fn hwb(hue: Angle, whiteness: impl Into<f64>, blackness: impl Into<f64>) -> Color {
    Color::try_hwb(hue, whiteness, blackness).unwrap_or_else(|error| panic!("{error}"))
}

/// Create an Oklab color.
///
/// # Panics
///
/// Panics if a channel is non-finite or outside its range. Use [`Color::try_oklab`] for
/// runtime-derived values.
#[track_caller]
#[must_use]
pub fn oklab(lightness: impl Into<f64>, a: impl Into<f64>, b: impl Into<f64>) -> Color {
    Color::try_oklab(lightness, a, b).unwrap_or_else(|error| panic!("{error}"))
}

/// Create an Oklch color.
///
/// # Panics
///
/// Panics if a channel is non-finite or outside its range. Use [`Color::try_oklch`] for
/// runtime-derived values.
#[track_caller]
#[must_use]
pub fn oklch(lightness: impl Into<f64>, chroma: impl Into<f64>, hue: Angle) -> Color {
    Color::try_oklch(lightness, chroma, hue).unwrap_or_else(|error| panic!("{error}"))
}

/// Create `light-dark(light, dark)`. The browser uses `light` in a light color scheme and `dark`
/// in a dark one.
///
/// [CSS Color Level 5 `light-dark()`](https://www.w3.org/TR/css-color-5/#light-dark)
#[must_use]
pub fn light_dark(light: impl Into<Color>, dark: impl Into<Color>) -> Color {
    Color::LightDark(Box::new(light.into()), Box::new(dark.into()))
}

/// Mix two colors in equal parts with the given color interpolation method, such as a
/// [`ColorSpace`].
///
/// [CSS Color Level 5 `color-mix()`](https://www.w3.org/TR/css-color-5/#color-mix)
#[must_use]
pub fn color_mix(
    method: impl Into<ColorInterpolationMethod>,
    first: impl Into<Color>,
    second: impl Into<Color>,
) -> Color {
    Color::Mix(Box::new(ColorMix {
        method: method.into(),
        first: first.into(),
        first_weight: None,
        second: second.into(),
    }))
}

/// Mix two colors, giving `first_weight` percent of the result to the first color.
///
/// ```rust
/// use typed_css::{ColorSpace, NamedColor, color_mix_weighted};
///
/// let tint = color_mix_weighted(ColorSpace::Oklch, NamedColor::Blue, 30, NamedColor::White);
/// assert_eq!(tint.to_string(), "color-mix(in oklch, blue 30%, white)");
/// ```
///
/// # Panics
///
/// Panics if `first_weight` is outside `[0.0, 100.0]`. Use [`Color::try_color_mix_weighted`] for
/// runtime-derived values.
#[track_caller]
#[must_use]
pub fn color_mix_weighted(
    method: impl Into<ColorInterpolationMethod>,
    first: impl Into<Color>,
    first_weight: impl Into<f64>,
    second: impl Into<Color>,
) -> Color {
    Color::try_color_mix_weighted(method, first, first_weight, second)
        .unwrap_or_else(|error| panic!("{error}"))
}

impl_display!(Color, ColorMix, ColorInterpolationMethod);
impl_checked_value!(Color);
