use crate::number::{dimension, unit_functions};

dimension! {
    /// A CSS `<length>` with its unit.
    ///
    /// The unit helpers such as [`px`] and [`rem`] create lengths. Most size properties accept a
    /// length, a percentage, or a calculation of both, which
    /// [`LengthPercentage`](crate::LengthPercentage) describes. A `Length` converts into a
    /// `LengthPercentage`, so those properties accept it directly. Adding or subtracting lengths
    /// also creates a `LengthPercentage`.
    ///
    /// [CSS Values Level 4 lengths](https://www.w3.org/TR/css-values-4/#lengths)
    pub struct Length(LengthUnit), "a length";
    Px => px, try_px, "px";
    /// The unit is the element's font size. Inside `font-size`, it is the parent's font size.
    Em => em, try_em, "em";
    /// The unit is the root element's font size.
    Rem => rem, try_rem, "rem";
    /// The unit is 1% of the viewport width.
    Vw => vw, try_vw, "vw";
    /// The unit is 1% of the viewport height.
    Vh => vh, try_vh, "vh";
    /// The unit is 1% of the smaller viewport dimension.
    Vmin => vmin, try_vmin, "vmin";
    /// The unit is 1% of the larger viewport dimension.
    Vmax => vmax, try_vmax, "vmax";
    /// The unit is the width of the `0` glyph in the element's font.
    Ch => ch, try_ch, "ch";
    /// The unit is 1% of the dynamic viewport width, which changes as browser UI such as an
    /// address bar appears or disappears.
    Dvw => dvw, try_dvw, "dvw";
    /// The unit is 1% of the dynamic viewport height, which changes as browser UI such as an
    /// address bar appears or disappears.
    Dvh => dvh, try_dvh, "dvh";
    /// The unit is 1% of the small viewport width, measured with all browser UI expanded.
    Svw => svw, try_svw, "svw";
    /// The unit is 1% of the small viewport height, measured with all browser UI expanded.
    Svh => svh, try_svh, "svh";
    /// The unit is 1% of the large viewport width, measured with all browser UI retracted.
    Lvw => lvw, try_lvw, "lvw";
    /// The unit is 1% of the large viewport height, measured with all browser UI retracted.
    Lvh => lvh, try_lvh, "lvh";
    /// The unit is 1% of the width of the nearest size container.
    Cqw => cqw, try_cqw, "cqw";
    /// The unit is 1% of the height of the nearest size container.
    Cqh => cqh, try_cqh, "cqh";
    /// One point is 1/72 of an inch, or 4/3 of a pixel.
    Pt => pt, try_pt, "pt";
    /// The unit is the x-height of the element's font.
    Ex => ex, try_ex, "ex";
    /// The unit is the element's line height.
    Lh => lh, try_lh, "lh";
    /// The unit is the root element's line height.
    Rlh => rlh, try_rlh, "rlh";
    /// The unit is 1% of the inline size of the nearest size container.
    Cqi => cqi, try_cqi, "cqi";
    /// The unit is 1% of the block size of the nearest size container.
    Cqb => cqb, try_cqb, "cqb";
    /// The unit is 1% of the smaller dimension of the nearest size container.
    Cqmin => cqmin, try_cqmin, "cqmin";
    /// The unit is 1% of the larger dimension of the nearest size container.
    Cqmax => cqmax, try_cqmax, "cqmax";
}

unit_functions! {
    Length;
    /// Create a length in pixels.
    px, try_px;
    /// Create a length relative to the element's font size.
    em, try_em;
    /// Create a length relative to the root element's font size.
    rem, try_rem;
    /// Create a length in percent of the viewport width.
    vw, try_vw;
    /// Create a length in percent of the viewport height.
    vh, try_vh;
    /// Create a length in percent of the smaller viewport dimension.
    vmin, try_vmin;
    /// Create a length in percent of the larger viewport dimension.
    vmax, try_vmax;
    /// Create a length in widths of the `0` glyph in the element's font.
    ch, try_ch;
    /// Create a length in percent of the dynamic viewport width.
    dvw, try_dvw;
    /// Create a length in percent of the dynamic viewport height.
    dvh, try_dvh;
    /// Create a length in percent of the small viewport width.
    svw, try_svw;
    /// Create a length in percent of the small viewport height.
    svh, try_svh;
    /// Create a length in percent of the large viewport width.
    lvw, try_lvw;
    /// Create a length in percent of the large viewport height.
    lvh, try_lvh;
    /// Create a length in percent of the nearest size container's width.
    cqw, try_cqw;
    /// Create a length in percent of the nearest size container's height.
    cqh, try_cqh;
    /// Create a length in points. One point is 1/72 of an inch.
    pt, try_pt;
    /// Create a length relative to the x-height of the element's font.
    ex, try_ex;
    /// Create a length relative to the element's line height.
    lh, try_lh;
    /// Create a length relative to the root element's line height.
    rlh, try_rlh;
    /// Create a length in percent of the nearest size container's inline size.
    cqi, try_cqi;
    /// Create a length in percent of the nearest size container's block size.
    cqb, try_cqb;
    /// Create a length in percent of the smaller dimension of the nearest size container.
    cqmin, try_cqmin;
    /// Create a length in percent of the larger dimension of the nearest size container.
    cqmax, try_cqmax;
}
