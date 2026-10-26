//! Public-surface tests for colors.

use assertr::prelude::*;
use typed_css::*;

#[test]
fn srgb_colors_use_modern_syntax() {
    assert_that!(rgb(255, 128, 0)).has_display_value("rgb(255 128 0)");
    assert_that!(rgba(255, 128, 0, 0.5)).has_display_value("rgb(255 128 0 / 0.5)");
    assert_that!(hex(0x001e_90ff)).has_display_value("rgb(30 144 255)");
    assert_that!(hex(0x001e_90ff)).is_equal_to(rgb(30, 144, 255));
    assert_that!(rgb(0, 0, 0).with_alpha(0.25)).has_display_value("rgb(0 0 0 / 0.25)");
}

#[test]
fn hex_colors_are_const() {
    const BRAND: Color = hex(0x0033_6699);
    assert_that!(BRAND).has_display_value("rgb(51 102 153)");
}

#[test]
#[should_panic(expected = "hex color must be at most 0xFFFFFF")]
fn hex_with_alpha_digits_panics() {
    let _ = hex(0x1122_3344);
}

#[test]
fn polar_and_perceptual_color_functions_serialize() {
    assert_that!(hsl(deg(120), 100, 50)).has_display_value("hsl(120deg 100% 50%)");
    assert_that!(hsla(turn(0.5), 100, 50, 0.3)).has_display_value("hsl(0.5turn 100% 50% / 0.3)");
    assert_that!(hwb(deg(200), 10, 20)).has_display_value("hwb(200deg 10% 20%)");
    assert_that!(oklab(0.6, -0.1, 0.05)).has_display_value("oklab(0.6 -0.1 0.05)");
    assert_that!(oklch(0.7, 0.15, deg(250))).has_display_value("oklch(0.7 0.15 250deg)");
    assert_that!(oklch(0.7, 0.15, deg(250)).with_alpha(0.8))
        .has_display_value("oklch(0.7 0.15 250deg / 0.8)");
}

#[test]
fn fallible_color_constructors_reject_out_of_range_channels() {
    assert_that!(Color::try_rgba(0, 0, 0, 1.5)).is_err();
    assert_that!(Color::try_hsl(deg(0), f64::NAN, 50)).is_err();
    assert_that!(Color::try_hsl(deg(0), 101, 50)).is_err();
    assert_that!(Color::try_hsla(deg(0), 50, 50, -0.1)).is_err();
    assert_that!(Color::try_hwb(deg(0), 50, 120)).is_err();
    assert_that!(Color::try_oklab(1.2, 0, 0)).is_err();
    assert_that!(Color::try_oklch(0.5, -0.1, deg(0))).is_err();
    assert_that!(Color::try_color_mix_weighted(
        ColorSpace::Srgb,
        rgb(0, 0, 0),
        120,
        rgb(1, 1, 1)
    ))
    .is_err();
}

#[test]
#[should_panic(expected = "CSS unit interval must be in range [0, 1], got 2")]
fn rgba_alpha_out_of_range_panics() {
    let _ = rgba(0, 0, 0, 2.0);
}

#[test]
fn named_colors_cover_the_full_keyword_table() {
    assert_that!(Color::from(NamedColor::RebeccaPurple)).has_display_value("rebeccapurple");
    assert_that!(Color::Named(NamedColor::CurrentColor)).has_display_value("currentcolor");
    assert_that!(NamedColor::LightGoldenrodYellow.as_str()).is_equal_to("lightgoldenrodyellow");
    assert_that!(NamedColor::Transparent.as_str()).is_equal_to("transparent");
}

#[test]
fn color_mix_accepts_a_hue_interpolation_method_for_polar_spaces() {
    let longer =
        ColorInterpolationMethod::with_hue(PolarColorSpace::Oklch, HueInterpolationMethod::Longer);
    assert_that!(color_mix(longer, NamedColor::Red, NamedColor::Blue))
        .has_display_value("color-mix(in oklch longer hue, red, blue)");
    assert_that!(color_mix_weighted(
        ColorInterpolationMethod::with_hue(
            PolarColorSpace::Hsl,
            HueInterpolationMethod::Increasing
        ),
        NamedColor::Red,
        25,
        NamedColor::Blue,
    ))
    .has_display_value("color-mix(in hsl increasing hue, red 25%, blue)");
    assert_that!(color_mix(
        PolarColorSpace::Lch,
        NamedColor::Red,
        NamedColor::Blue
    ))
    .has_display_value("color-mix(in lch, red, blue)");
    assert_that!(ColorSpace::from(PolarColorSpace::Hwb)).is_equal_to(ColorSpace::Hwb);
}

#[test]
fn light_dark_and_color_mix_compose_colors() {
    assert_that!(light_dark(NamedColor::White, hex(0x0011_1111)))
        .has_display_value("light-dark(white, rgb(17 17 17))");
    assert_that!(color_mix(
        ColorSpace::Oklch,
        NamedColor::Red,
        NamedColor::Blue
    ))
    .has_display_value("color-mix(in oklch, red, blue)");
    assert_that!(color_mix_weighted(
        ColorSpace::SrgbLinear,
        NamedColor::Red,
        30,
        NamedColor::Blue
    ))
    .has_display_value("color-mix(in srgb-linear, red 30%, blue)");
}

#[test]
fn color_adjustment_keywords_serialize() {
    assert_that!(ForcedColorAdjust::PreserveParentColor).has_display_value("preserve-parent-color");
    assert_that!(PrintColorAdjust::Exact).has_display_value("exact");
}
