//! Public-surface tests for custom properties and `var()`.

use std::collections::HashSet;

use assertr::prelude::*;
use typed_css::{property::*, *};

custom_property!(SPACE: LengthPercentage = "--space");
custom_property!(ACCENT: Color = "--accent");

#[test]
fn custom_property_names_are_checked() {
    for valid in ["--accent-color", "--2xl", "---x", "--_private", "--A"] {
        assert_that!(CustomProperty::<Color>::try_new(valid)).is_ok();
    }
    for invalid in [
        "accent-color",
        "--",
        "--accent;color",
        "--accent color",
        "--accént",
        "-accent",
    ] {
        assert_that!(CustomProperty::<Color>::try_new(invalid)).is_err();
    }
    assert_that!(CustomProperty::<Color>::new("--accent-color").name())
        .is_equal_to("--accent-color");
}

#[test]
fn custom_properties_compare_and_hash_by_name_for_any_grammar() {
    let names: HashSet<CustomProperty<LengthPercentage>> = [
        SPACE,
        CustomProperty::new("--space"),
        CustomProperty::new("--gap"),
    ]
    .into_iter()
    .collect();
    assert_that!(names.len()).is_equal_to(2);
}

#[test]
fn var_references_render_with_and_without_fallbacks() {
    assert_that!(var(&ACCENT)).has_display_value("var(--accent)");
    let default_accent = CustomProperty::<Color>::new("--default-accent");
    let nested = var_or(&ACCENT, var_or(&default_accent, NamedColor::CurrentColor));
    assert_that!(nested).has_display_value("var(--accent, var(--default-accent, currentcolor))");
}

#[test]
fn custom_properties_declare_their_grammar() {
    let gap = CustomProperty::<LengthPercentage>::new("--gap");
    assert_that!(ACCENT.declare(rgb(1, 2, 3)).value()).is_equal_to("rgb(1 2 3)");
    assert_that!(SPACE.declare(px(-4)).value()).is_equal_to("-4px");
    assert_that!(SPACE.declare(var_or(&gap, px(8))).value()).is_equal_to("var(--gap, 8px)");
}

#[test]
fn length_percentage_tokens_feed_every_length_grammar() {
    // A `var()` is written plainly, so that a token value of `0` stays valid.
    assert_that!(PADDING.declare(var(&SPACE)).value()).is_equal_to("var(--space)");
    assert_that!(MARGIN.declare(var(&SPACE)).value()).is_equal_to("var(--space)");
    assert_that!(GAP.declare(var_or(&SPACE, px(4))).value()).is_equal_to("var(--space, 4px)");
    assert_that!(WIDTH.declare(pct(100) - var(&SPACE)).value())
        .is_equal_to("calc(100% - var(--space))");
}

#[test]
fn var_references_support_arithmetic() {
    assert_that!(var(&SPACE) * 2).has_display_value("calc(var(--space) * 2)");
    assert_that!(var(&SPACE) / 2 + px(1)).has_display_value("calc(var(--space) / 2 + 1px)");
    assert_that!(-var(&SPACE)).has_display_value("calc(var(--space) * -1)");
    assert_that!((var(&SPACE) + px(4)) * 3).has_display_value("calc((var(--space) + 4px) * 3)");
}

#[test]
fn length_tokens_widen_into_length_percentages() {
    let sidebar = CustomProperty::<Length>::new("--sidebar-width");
    assert_that!(pct(100) - var_or(&sidebar, px(320)))
        .has_display_value("calc(100% - var(--sidebar-width, 320px))");
    assert_that!(WIDTH.declare(var_or(&sidebar, px(320))).value())
        .is_equal_to("var(--sidebar-width, 320px)");
}

#[test]
fn custom_properties_accept_global_keywords_and_percentage_tokens() {
    let ratio = CustomProperty::<Percentage>::new("--ratio");

    assert_that!(SPACE.declare_global(GlobalKeyword::Inherit).to_string())
        .is_equal_to("--space:inherit;");
    assert_that!(SPACE.revert_layer().to_string()).is_equal_to("--space:revert-layer;");
    assert_that!(WIDTH.declare(var_or(&ratio, pct(50))).value()).is_equal_to("var(--ratio, 50%)");
}

#[test]
fn color_custom_properties_combine_with_color_functions() {
    let dark_accent = CustomProperty::<Color>::new("--dark-accent");
    assert_that!(light_dark(var(&ACCENT), var(&dark_accent)))
        .has_display_value("light-dark(var(--accent), var(--dark-accent))");
    assert_that!(color_mix(
        ColorSpace::Oklch,
        var(&ACCENT),
        NamedColor::White
    ))
    .has_display_value("color-mix(in oklch, var(--accent), white)");
    assert_that!(Color::from(var(&ACCENT)).with_alpha(0.5))
        .has_display_value("color-mix(in srgb, var(--accent) 50%, transparent)");
}

#[test]
fn negative_fallbacks_are_clamped_in_non_negative_grammars() {
    assert_that!(PADDING.declare(var_or(&SPACE, px(-4))).value())
        .is_equal_to("var(--space, calc(-4px))");
    assert_that!(MARGIN.declare(var_or(&SPACE, px(-4))).value()).is_equal_to("var(--space, -4px)");
}

#[test]
fn references_to_primitive_grammars_work_in_every_containing_grammar() {
    custom_property!(TILT: Angle = "--tilt");
    custom_property!(FADE: UnitInterval = "--fade");
    custom_property!(LEADING: NonNegativeNumber = "--leading");
    custom_property!(INSET: NonNegativeLengthPercentage = "--inset");
    custom_property!(WIDE: Size = "--wide");

    assert_that!(ROTATE.declare(var(&TILT)).value()).is_equal_to("var(--tilt)");
    assert_that!(OPACITY.declare(var(&FADE)).value()).is_equal_to("var(--fade)");
    assert_that!(LINE_HEIGHT.declare(var(&LEADING)).value()).is_equal_to("var(--leading)");
    assert_that!(PADDING.declare(var(&INSET)).value()).is_equal_to("var(--inset)");
    assert_that!(WIDTH.declare(var_or(&WIDE, Size::Auto)).value()).is_equal_to("var(--wide, auto)");
}

#[test]
fn a_var_basis_keeps_both_flex_factors() {
    assert_that!(Flex::basis(var(&SPACE))).has_display_value("1 1 var(--space)");
    assert_that!(Flex::new(2, 1, var(&SPACE))).has_display_value("2 1 var(--space)");
    assert_that!(FLEX.declare(px(80)).value()).is_equal_to("80px");
}

#[test]
fn references_reach_every_length_percentage_argument() {
    custom_property!(INSET: NonNegativeLengthPercentage = "--inset");
    custom_property!(OUTER: NonNegativeLengthPercentage = "--outer");

    assert_that!(TransformFunction::translate_3d(var(&SPACE), px(0), px(4)))
        .has_display_value("translate3d(var(--space), 0px, 4px)");
    assert_that!(Translate::xyz(px(1), var(&SPACE), px(3)))
        .has_display_value("1px var(--space) 3px");
    assert_that!(Padding::from(var(&INSET))).has_display_value("var(--inset)");
    assert_that!(NonNegativeLengthPercentage::from(var_or(
        &INSET,
        var_or(&OUTER, px(-4))
    )))
    .has_display_value("var(--inset, var(--outer, calc(-4px)))");
}

#[test]
fn declaration_values_convert_between_grammars() {
    let width: DeclarationValue<LengthPercentage> = var_or(&SPACE, px(8)).into();
    assert_that!(width.clone().into_grammar::<Size>()).has_display_value("var(--space, 8px)");
    assert_that!(LengthPercentage::from(width) * 2)
        .has_display_value("calc(var(--space, 8px) * 2)");
}

#[test]
fn integer_literals_work_for_numeric_properties() {
    assert_that!(ASPECT_RATIO.declare(2).value()).is_equal_to("2");
    assert_that!(LINE_HEIGHT.declare(2).value()).is_equal_to("2");
    assert_that!(SCALE.declare(2).value()).is_equal_to("2");
    assert_that!(Auto).has_display_value("auto");
}
