//! Public-surface tests for lengths, percentages, and length-percentage expressions.

use assertr::prelude::*;
use typed_css::*;

#[test]
fn every_length_unit_serializes() {
    let cases = [
        (px(10), "10px"),
        (em(1.5), "1.5em"),
        (rem(2), "2rem"),
        (vw(100), "100vw"),
        (vh(50), "50vh"),
        (vmin(25), "25vmin"),
        (vmax(75), "75vmax"),
        (ch(3), "3ch"),
        (dvw(50), "50dvw"),
        (dvh(100), "100dvh"),
        (svw(80), "80svw"),
        (svh(90), "90svh"),
        (lvw(100), "100lvw"),
        (lvh(100), "100lvh"),
        (cqw(50), "50cqw"),
        (cqh(25), "25cqh"),
        (cqi(25), "25cqi"),
        (cqb(2), "2cqb"),
        (cqmin(3), "3cqmin"),
        (cqmax(4), "4cqmax"),
        (lh(1.5), "1.5lh"),
        (rlh(2), "2rlh"),
        (ex(1), "1ex"),
        (pt(12), "12pt"),
    ];
    for (length, expected) in cases {
        assert_that!(length).has_display_value(expected);
    }
    assert_that!(pct(75)).has_display_value("75%");
}

#[test]
fn lengths_and_percentages_are_const_constructible_and_copy() {
    const GUTTER: Length = Length::px(8.0);
    const HALF: Percentage = Percentage::new(50.0);
    let copied = GUTTER;
    assert_that!(GUTTER).is_equal_to(copied);
    assert_that!(HALF).has_display_value("50%");
    assert_that!(Length::try_px(f64::NAN)).is_err();
    assert_that!(Percentage::try_new(f64::INFINITY)).is_err();
    assert_that!(Length::try_rem(1.25).unwrap()).has_display_value("1.25rem");
}

#[test]
#[should_panic(expected = "CSS px value must be finite")]
fn px_nan_panics() {
    let _ = px(f64::NAN);
}

#[test]
fn negation_and_value_mapping_keep_the_unit() {
    assert_that!(-px(10)).has_display_value("-10px");
    assert_that!(-pct(5)).has_display_value("-5%");
    assert_that!(em(2).map_value(|v| v * 2.0)).has_display_value("4em");
    assert_that!(px(1).try_map_value(|_| f64::NAN)).is_err();
    assert_that!(em(2).unit()).is_equal_to("em");
}

#[test]
fn every_dimension_scales_by_a_number() {
    assert_that!(px(10) * 2).has_display_value("20px");
    assert_that!(pct(10) / 4).has_display_value("2.5%");
    assert_that!(deg(10) * 3).has_display_value("30deg");
    assert_that!(-turn(0.25)).has_display_value("-0.25turn");
    assert_that!(ms(300) / 2).has_display_value("150ms");
    assert_that!(s(1.5).map_value(|v| v * 2.0)).has_display_value("3s");
}

#[test]
fn same_unit_arithmetic_folds() {
    assert_that!(px(10) + px(5)).has_display_value("15px");
    assert_that!(px(10) - px(15)).has_display_value("-5px");
    assert_that!(pct(50) + pct(25)).has_display_value("75%");
}

#[test]
fn folding_overflow_keeps_the_expression_instead_of_panicking() {
    assert_that!(px(f64::MAX) + px(f64::MAX)).has_display_value(format!(
        "calc({} + {})",
        px(f64::MAX),
        px(f64::MAX)
    ));
}

#[test]
fn mixed_unit_arithmetic_builds_calc_sums() {
    assert_that!(pct(100) - px(20)).has_display_value("calc(100% - 20px)");
    assert_that!(pct(100) - px(20) + rem(1)).has_display_value("calc(100% - 20px + 1rem)");
    assert_that!(vh(100) - (px(10) + rem(2))).has_display_value("calc(100vh - (10px + 2rem))");
}

#[test]
fn math_functions_take_sums_as_bare_arguments() {
    assert_that!(min(pct(100), px(600))).has_display_value("min(100%, 600px)");
    assert_that!(max(px(200), pct(50) - px(10))).has_display_value("max(200px, 50% - 10px)");
    assert_that!(clamp(rem(1), vw(2.5), rem(2))).has_display_value("clamp(1rem, 2.5vw, 2rem)");
    assert_that!(env(EnvironmentVariable::SafeAreaInsetTop))
        .has_display_value("env(safe-area-inset-top)");
    assert_that!(min(pct(100), px(600)) + px(16))
        .has_display_value("calc(min(100%, 600px) + 16px)");
}

#[test]
fn non_negative_contexts_wrap_values_that_may_be_negative() {
    let non_negative = |value: LengthPercentage| NonNegativeLengthPercentage::from(value);

    assert_that!(non_negative(px(4).into())).has_display_value("4px");
    assert_that!(non_negative(px(-4).into())).has_display_value("calc(-4px)");
    assert_that!(non_negative(px(10) - px(20))).has_display_value("calc(-10px)");
    assert_that!(non_negative(pct(100) - px(20))).has_display_value("calc(100% - 20px)");
    assert_that!(non_negative(min(px(-1), pct(10)))).has_display_value("min(-1px, 10%)");
    assert_that!(non_negative(env(EnvironmentVariable::SafeAreaInsetBottom)))
        .has_display_value("env(safe-area-inset-bottom)");
}

#[test]
fn unrestricted_contexts_keep_negative_values_plain() {
    assert_that!(LengthPercentageAuto::from(px(-8))).has_display_value("-8px");
    assert_that!(LengthPercentageAuto::Auto).has_display_value("auto");
    assert_that!(LengthPercentage::from(px(-1))).has_display_value("-1px");
}

#[test]
fn products_fold_direct_values_and_wrap_expressions() {
    assert_that!(px(10) * 2).has_display_value("20px");
    assert_that!(pct(50) / 4).has_display_value("12.5%");
    assert_that!((pct(100) - px(20)) / 3).has_display_value("calc((100% - 20px) / 3)");
    assert_that!(px(4) + (pct(100) - px(20)) * 2)
        .has_display_value("calc(4px + (100% - 20px) * 2)");
    assert_that!(-(pct(100) - px(20))).has_display_value("calc((100% - 20px) * -1)");
}

#[test]
fn folding_rounds_away_float_noise() {
    assert_that!(px(0.1) + px(0.2)).has_display_value("0.3px");
    assert_that!(px(1) / 3).has_display_value("0.333333333333333px");
    assert_that!(px(1e-10) + px(0)).has_display_value("0.0000000001px");
}

#[test]
#[should_panic(expected = "CSS divisor must not be zero")]
fn division_by_zero_panics() {
    let _ = px(1) / 0;
}
