//! Public-surface tests for numbers, angles, times, and CSS-wide keywords.

use assertr::prelude::*;
use typed_css::*;

#[test]
fn angle_helpers_return_angles_in_every_unit() {
    assert_that!(deg(45)).has_display_value("45deg");
    assert_that!(rad(1.5)).has_display_value("1.5rad");
    assert_that!(turn(0.25)).has_display_value("0.25turn");
    assert_that!(grad(100)).has_display_value("100grad");
    assert_that!(-deg(90)).has_display_value("-90deg");
}

#[test]
fn time_helpers_return_times() {
    assert_that!(s(0.3)).has_display_value("0.3s");
    assert_that!(ms(300)).has_display_value("300ms");
    assert_that!(-ms(10)).has_display_value("-10ms");
}

#[test]
fn fallible_angle_and_time_constructors_reject_non_finite_values() {
    assert_that!(Angle::try_deg(f64::NAN)).is_err();
    assert_that!(Angle::try_rad(f64::INFINITY)).is_err();
    assert_that!(Angle::try_turn(f64::NEG_INFINITY)).is_err();
    assert_that!(Angle::try_grad(f64::NAN)).is_err();
    assert_that!(Time::try_s(f64::NAN)).is_err();
    assert_that!(Time::try_ms(f64::INFINITY)).is_err();
    assert_that!(Angle::try_deg(30.0).unwrap()).has_display_value("30deg");
}

#[test]
fn angles_and_times_are_const_constructible() {
    const TILT: Angle = Angle::deg(3.0);
    const FADE: Time = Time::ms(150.0);
    assert_that!(TILT).has_display_value("3deg");
    assert_that!(FADE).has_display_value("150ms");
}

#[test]
#[should_panic(expected = "CSS deg value must be finite")]
fn deg_nan_panics() {
    let _ = deg(f64::NAN);
}

#[test]
#[should_panic(expected = "CSS ms value must be finite")]
fn ms_infinity_panics() {
    let _ = ms(f64::INFINITY);
}

#[test]
fn finite_numbers_reject_nan_and_infinity() {
    assert_that!(Number::try_new(f64::NAN)).is_err();
    assert_that!(Number::try_from(1.5).unwrap()).has_display_value("1.5");
    assert_that!(f64::from(Number::new(2.0))).is_equal_to(2.0);
}

#[test]
fn constrained_numbers_report_their_violation() {
    assert_that!(NonNegativeNumber::try_new(-1.0)).is_equal_to(Err(InvalidNumber::Negative {
        value: -1.0,
        context: "number",
    }));
    assert_that!(UnitInterval::try_new(1.5)).is_equal_to(Err(InvalidNumber::OutOfRange {
        value: 1.5,
        context: "unit interval",
        min: 0.0,
        max: 1.0,
    }));
    assert_that!(PercentageChannel::try_new(101.0)).is_err();
    assert_that!(PercentageChannel::new(50.0)).has_display_value("50%");
    assert_that!(NonNegativeNumber::new(3)).has_display_value("3");
}

#[test]
fn global_keywords_serialize() {
    let cases = [
        (GlobalKeyword::Inherit, "inherit"),
        (GlobalKeyword::Initial, "initial"),
        (GlobalKeyword::Unset, "unset"),
        (GlobalKeyword::Revert, "revert"),
        (GlobalKeyword::RevertLayer, "revert-layer"),
    ];
    for (keyword, expected) in cases {
        assert_that!(keyword).has_display_value(expected);
    }
}

#[test]
fn to_css_appends_to_an_existing_buffer() {
    let mut buffer = String::from("width:");
    px(16).to_css(&mut buffer).unwrap();
    assert_that!(buffer).is_equal_to("width:16px");
    assert_that!(rem(2).to_css_string()).is_equal_to("2rem");
}
