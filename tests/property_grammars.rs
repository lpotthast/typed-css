//! Public-surface tests for property value grammars.

use assertr::prelude::*;
use typed_css::*;

#[test]
fn box_shorthands_are_written_in_the_shortest_form() {
    assert_that!(Padding::all(px(4))).has_display_value("4px");
    assert_that!(Padding::symmetric(px(4), pct(5))).has_display_value("4px 5%");
    assert_that!(Padding::symmetric(px(1), px(2)).with_bottom(px(3)))
        .has_display_value("1px 2px 3px");
    assert_that!(Padding::edges(px(1), px(2), px(3), px(4))).has_display_value("1px 2px 3px 4px");
    assert_that!(Padding::edges(px(1), px(1), px(1), px(1))).has_display_value("1px");
    assert_that!(Padding::all(px(4)).with_left(px(0))).has_display_value("4px 4px 4px 0px");
    assert_that!(Margin::symmetric(px(0), LengthPercentageAuto::Auto))
        .has_display_value("0px auto");
    assert_that!(Margin::all(px(-8))).has_display_value("-8px");
    assert_that!(Inset::all(pct(50) - px(20))).has_display_value("calc(50% - 20px)");
}

#[test]
fn padding_clamps_negative_components_through_calc() {
    assert_that!(Padding::symmetric(px(-4), px(4))).has_display_value("calc(-4px) 4px");
    assert_that!(PaddingAxis::new(px(8), px(-1))).has_display_value("8px calc(-1px)");
}

#[test]
fn logical_axis_shorthands_serialize() {
    assert_that!(PaddingAxis::both(px(8))).has_display_value("8px");
    assert_that!(PaddingAxis::new(px(8), px(8))).has_display_value("8px");
    assert_that!(MarginAxis::new(LengthPercentageAuto::Auto, px(-8)))
        .has_display_value("auto -8px");
    assert_that!(InsetAxis::both(px(0))).has_display_value("0px");
}

#[test]
fn border_radii_serialize_circular_and_elliptical_corners() {
    assert_that!(CornerRadius::elliptical(px(8), pct(50))).has_display_value("8px 50%");
    assert_that!(BorderRadius::all(px(8))).has_display_value("8px");
    assert_that!(BorderRadius::all(CornerRadius::elliptical(px(8), px(4))))
        .has_display_value("8px / 4px");
    assert_that!(BorderRadius::corners(px(1), px(2), px(3), px(4)))
        .has_display_value("1px 2px 3px 4px");
}

#[test]
fn sizing_keywords_and_fit_content_serialize() {
    assert_that!(Size::from(px(120))).has_display_value("120px");
    assert_that!(Size::Auto).has_display_value("auto");
    assert_that!(Size::Stretch).has_display_value("stretch");
    assert_that!(Size::fit_content(pct(50))).has_display_value("fit-content(50%)");
    assert_that!(MaxSize::None).has_display_value("none");
    assert_that!(MaxSize::Contain).has_display_value("contain");
    assert_that!(Size::from(px(-10))).has_display_value("calc(-10px)");
}

#[test]
fn gaps_serialize() {
    assert_that!(Gap::from(px(8))).has_display_value("8px");
    assert_that!(Gap::new(GapValue::Normal, pct(2))).has_display_value("normal 2%");
    assert_that!(Gap::new(px(4), px(4))).has_display_value("4px");
}

#[test]
fn view_transition_names_are_checked() {
    assert_that!(ViewTransitionName::none()).has_display_value("none");
    assert_that!(ViewTransitionName::named("hero-card")).has_display_value("hero-card");
    for reserved in [
        "none",
        "auto",
        "default",
        "inherit",
        "initial",
        "revert",
        "revert-layer",
        "unset",
        "hero card",
    ] {
        assert_that!(ViewTransitionName::try_named(reserved)).is_err();
    }
}

#[test]
fn touch_action_gesture_composition_covers_normative_grammar() {
    assert_that!(TouchAction::Manipulation).has_display_value("manipulation");
    let gestures = TouchActionGestures::horizontal(TouchActionHorizontalPan::PanLeft)
        .with_vertical(TouchActionVerticalPan::PanUp)
        .with_pinch_zoom();
    assert_that!(TouchAction::from(gestures)).has_display_value("pan-left pan-up pinch-zoom");
    assert_that!(TouchActionGestures::vertical(
        TouchActionVerticalPan::PanDown
    ))
    .has_display_value("pan-down");
    assert_that!(TouchActionGestures::pinch_zoom()).has_display_value("pinch-zoom");
}

#[test]
fn font_weights_accept_keywords_and_numbers_in_range() {
    assert_that!(FontWeight::Bold).has_display_value("bold");
    assert_that!(FontWeight::new(450)).has_display_value("450");
    assert_that!(FontWeightNumber::try_new(0)).is_err();
    assert_that!(FontWeightNumber::try_new(1000.5)).is_err();
    assert_that!(FontWeightNumber::try_new(1).unwrap().value()).is_equal_to(1.0);
}

#[test]
fn font_style_and_oblique_angles_are_checked() {
    assert_that!(FontStyle::Italic).has_display_value("italic");
    assert_that!(FontStyle::Left).has_display_value("left");
    assert_that!(FontStyle::Right).has_display_value("right");
    assert_that!(FontStyle::Oblique(None)).has_display_value("oblique");
    assert_that!(FontStyle::Oblique(Some(ObliqueAngle::new(deg(-10)))))
        .has_display_value("oblique -10deg");
    for valid in [deg(90), deg(-90), turn(0.25), rad(1.5), grad(100)] {
        assert_that!(ObliqueAngle::try_new(valid)).is_ok();
    }
    for invalid in [deg(90.5), turn(-0.3), rad(1.6), grad(101)] {
        assert_that!(ObliqueAngle::try_new(invalid)).is_err();
    }
}

#[test]
fn font_size_and_line_height_serialize() {
    assert_that!(FontSize::XxxLarge).has_display_value("xxx-large");
    assert_that!(FontSize::from(rem(1.25))).has_display_value("1.25rem");
    assert_that!(FontSize::from(clamp(rem(1), vw(2), rem(2))))
        .has_display_value("clamp(1rem, 2vw, 2rem)");
    assert_that!(LineHeight::Normal).has_display_value("normal");
    assert_that!(LineHeight::number(1.5)).has_display_value("1.5");
    assert_that!(LineHeight::try_number(-1)).is_err();
    assert_that!(LineHeight::from(pct(120))).has_display_value("120%");
}

#[test]
fn text_decoration_lines_serialize_in_specification_order() {
    let lines = TextDecorationLines::line_through()
        .with_overline()
        .with_underline();
    assert_that!(TextDecorationLine::from(lines))
        .has_display_value("underline overline line-through");
    assert_that!(TextDecorationLine::SpellingError).has_display_value("spelling-error");
}

#[test]
fn opacity_and_z_index_serialize() {
    assert_that!(Opacity::new(0.5)).has_display_value("0.5");
    assert_that!(Opacity::try_new(1.5)).is_err();
    assert_that!(ZIndex::Integer(-1)).has_display_value("-1");
    assert_that!(ZIndex::Auto).has_display_value("auto");
}

#[test]
fn layout_keywords_serialize() {
    assert_that!(Display::InlineFlex).has_display_value("inline-flex");
    assert_that!(Display::FlowRoot).has_display_value("flow-root");
    assert_that!(Visibility::Collapse).has_display_value("collapse");
    assert_that!(Position::Sticky).has_display_value("sticky");
    assert_that!(Cursor::NotAllowed).has_display_value("not-allowed");
    assert_that!(Overflow::from(OverflowKeyword::Clip)).has_display_value("clip");
    assert_that!(Overflow::new(
        OverflowKeyword::Hidden,
        OverflowKeyword::Auto
    ))
    .has_display_value("hidden auto");
}

#[test]
fn flex_grammars_serialize() {
    assert_that!(FlexFlow::direction(FlexDirection::ColumnReverse))
        .has_display_value("column-reverse");
    assert_that!(FlexFlow::wrap(FlexWrap::WrapReverse).with_direction(FlexDirection::Row))
        .has_display_value("wrap-reverse");
    assert_that!(FlexFlow::wrap(FlexWrap::Wrap).with_direction(FlexDirection::Column))
        .has_display_value("column wrap");
    assert_that!(FlexFlow::direction(FlexDirection::Row)).has_display_value("row");
    assert_that!(FlexFlow::new(FlexDirection::Column, FlexWrap::Wrap))
        .has_display_value("column wrap");
    assert_that!(FlexFlow::new(FlexDirection::Row, FlexWrap::Nowrap)).has_display_value("row");
    assert_that!(FlexBasis::Content).has_display_value("content");
    assert_that!(FlexBasis::from(Size::Auto)).has_display_value("auto");
    assert_that!(Flex::none()).has_display_value("none");
    assert_that!(Flex::grow(1)).has_display_value("1");
    assert_that!(Flex::basis(px(80)).with_shrink(0)).has_display_value("1 0 80px");
    assert_that!(Flex::none().with_basis(pct(50))).has_display_value("0 0 50%");
    assert_that!(Flex::grow(2).with_shrink(1)).has_display_value("2");
    assert_that!(Flex::grow(2).with_basis(px(10))).has_display_value("2 10px");
    assert_that!(Flex::grow(1).with_shrink(0)).has_display_value("1 0");
    assert_that!(Flex::auto()).has_display_value("auto");
    assert_that!(Flex::basis(px(80)).with_grow(3)).has_display_value("3 80px");
    assert_that!(Flex::try_new(-1, 1, px(0))).is_err();
}

#[test]
fn alignment_grammars_serialize_positions_and_overflow_safety() {
    assert_that!(JustifyContent::from(ContentDistribution::SpaceBetween))
        .has_display_value("space-between");
    assert_that!(JustifyContent::from(PhysicalPosition::Left)).has_display_value("left");
    assert_that!(JustifyContent::Position(
        Some(OverflowPosition::Safe),
        ContentPosition::Center
    ))
    .has_display_value("safe center");
    assert_that!(AlignContent::from(BaselinePosition::Last)).has_display_value("last baseline");
    assert_that!(AlignItems::from(SelfPosition::FlexStart)).has_display_value("flex-start");
    assert_that!(AlignItems::Baseline(BaselinePosition::First)).has_display_value("baseline");
    assert_that!(AlignSelf::Auto).has_display_value("auto");
    assert_that!(JustifyItems::Legacy(Some(LegacyPosition::Center)))
        .has_display_value("legacy center");
    assert_that!(JustifySelf::Physical(
        Some(OverflowPosition::Unsafe),
        PhysicalPosition::Right
    ))
    .has_display_value("unsafe right");
}

#[test]
fn grid_track_lists_serialize() {
    let columns = GridTemplate::new([
        GridTrack::from(px(200)),
        GridRepeat::count(2, [fr(1)]).into(),
        TrackSize::fit_content(px(300)).into(),
    ]);
    assert_that!(columns).has_display_value("200px repeat(2, 1fr) fit-content(300px)");
    assert_that!(GridTemplate::none()).has_display_value("none");
    assert_that!(GridAutoTracks::new([TrackSize::minmax(
        InflexibleBreadth::MinContent,
        fr(1)
    )]))
    .has_display_value("minmax(min-content, 1fr)");
    assert_that!(GridAutoFlow::ColumnDense).has_display_value("column dense");
}

#[test]
fn grid_track_lists_enforce_the_auto_repeat_rules() {
    let auto_fit = || GridRepeat::auto_fit([TrackSize::minmax(px(160), fr(1))]);

    assert_that!(GridTemplate::try_new([
        GridTrack::from(px(10)),
        auto_fit().into()
    ]))
    .is_ok();
    assert_that!(GridTemplate::try_new([
        GridTrack::from(fr(1)),
        auto_fit().into()
    ]))
    .is_equal_to(Err(InvalidGridTracks::AutoRepeatRequiresFixedSizes));
    assert_that!(GridTemplate::try_new([auto_fit(), auto_fit()]))
        .is_equal_to(Err(InvalidGridTracks::MultipleAutoRepeats));
    assert_that!(GridTemplate::try_new(Vec::<GridTrack>::new()))
        .is_equal_to(Err(InvalidGridTracks::Empty));
    assert_that!(GridRepeat::try_count(0, [fr(1)]))
        .is_equal_to(Err(InvalidGridTracks::ZeroRepeatCount));
    assert_that!(GridRepeat::try_new(
        RepeatCount::AutoFill,
        Vec::<TrackSize>::new()
    ))
    .is_equal_to(Err(InvalidGridTracks::Empty));
}

#[test]
fn grid_lines_serialize_and_reject_zero() {
    assert_that!(GridLineRange::from(GridLine::line(-1))).has_display_value("-1");
    assert_that!(GridLineRange::new(GridLine::line(2), GridLine::span(3)))
        .has_display_value("2 / span 3");
    assert_that!(GridLine::Auto).has_display_value("auto");
    assert_that!(GridLine::try_line(0)).is_equal_to(Err(InvalidGridLine));
    assert_that!(GridLine::try_span(0)).is_equal_to(Err(InvalidGridLine));
}

#[test]
fn transforms_serialize_function_lists_and_individual_properties() {
    let transform = Transform::from(TransformFunction::translate(pct(-50), px(10)))
        .then(TransformFunction::rotate(turn(0.5)))
        .then(TransformFunction::scale(2, 1))
        .then(TransformFunction::matrix([1.0, 0.0, 0.0, 1.0, 5.0, 6.0]));
    assert_that!(transform).has_display_value(
        "translate(-50%, 10px) rotate(0.5turn) scale(2, 1) matrix(1, 0, 0, 1, 5, 6)",
    );
    assert_that!(Transform::none()).has_display_value("none");
    assert_that!(TransformFunction::translate_x(pct(100) - px(8)))
        .has_display_value("translateX(calc(100% - 8px))");
    assert_that!(Translate::xy(px(1), px(2))).has_display_value("1px 2px");
    assert_that!(Rotate::from(deg(45))).has_display_value("45deg");
    assert_that!(Rotate::x(deg(10))).has_display_value("x 10deg");
    assert_that!(Scale::uniform(1.5)).has_display_value("1.5");
    assert_that!(Scale::None).has_display_value("none");
}

#[test]
fn border_radius_builders_replace_single_corners() {
    let radius = BorderRadius::all(px(8))
        .with_top_left(px(0))
        .with_bottom_left(px(0));
    assert_that!(radius).has_display_value("0px 8px 8px 0px");
    assert_that!(BorderRadius::corners(px(4), px(4), px(4), px(4))).has_display_value("4px");
}

#[test]
fn integers_and_unit_intervals_convert_into_their_grammars() {
    use typed_css::property::{OPACITY, ORDER, Z_INDEX};

    assert_that!(Z_INDEX.declare(10).value()).is_equal_to("10");
    assert_that!(ORDER.declare(-1).value()).is_equal_to("-1");
    assert_that!(OPACITY.declare(UnitInterval::new(0.25)).value()).is_equal_to("0.25");
    assert_that!(FontStyle::oblique(deg(-12))).has_display_value("oblique -12deg");
}

#[test]
fn declarations_display_as_style_attribute_text() {
    use typed_css::property::{DISPLAY, GAP};

    let declarations = [DISPLAY.declare(Display::Grid), GAP.declare(rem(1))];
    let style: String = declarations.iter().map(Declaration::to_string).collect();
    assert_that!(style).is_equal_to("display:grid;gap:1rem;");
    assert_that!(declarations[1].to_css_string()).is_equal_to("gap:1rem;");
}

#[test]
fn keyword_methods_declare_their_keyword() {
    use typed_css::property::{
        ALL, COLOR, CURSOR, DISPLAY, FLEX, JUSTIFY_CONTENT, JUSTIFY_ITEMS, MARGIN_INLINE, PADDING,
        PLACE_ITEMS, POSITION, TEXT_DECORATION_LINE, TOUCH_ACTION, WIDTH,
    };

    let cases = [
        (DISPLAY.grid(), "display:grid;"),
        (POSITION.r#static(), "position:static;"),
        (CURSOR.r#move(), "cursor:move;"),
        (JUSTIFY_CONTENT.center(), "justify-content:center;"),
        (
            JUSTIFY_CONTENT.space_between(),
            "justify-content:space-between;",
        ),
        (
            JUSTIFY_ITEMS.legacy_center(),
            "justify-items:legacy center;",
        ),
        (PLACE_ITEMS.last_baseline(), "place-items:last baseline;"),
        (WIDTH.min_content(), "width:min-content;"),
        (MARGIN_INLINE.auto(), "margin-inline:auto;"),
        (FLEX.none(), "flex:none;"),
        (
            TEXT_DECORATION_LINE.line_through(),
            "text-decoration-line:line-through;",
        ),
        (TOUCH_ACTION.pan_x(), "touch-action:pan-x;"),
        (PADDING.inherit(), "padding:inherit;"),
        (COLOR.initial(), "color:initial;"),
        (DISPLAY.unset(), "display:unset;"),
        (WIDTH.revert(), "width:revert;"),
        (ALL.revert_layer(), "all:revert-layer;"),
    ];
    for (declaration, expected) in cases {
        assert_that!(declaration.to_string()).is_equal_to(expected);
    }
}

#[test]
#[expect(clippy::too_many_lines, reason = "one table lists every property")]
fn every_property_accepts_the_common_parts_of_its_grammar() {
    use typed_css::property::*;

    let space = CustomProperty::<LengthPercentage>::new("--space");
    let cases = [
        (DISPLAY.declare(Display::Grid), "display:grid;"),
        (VISIBILITY.declare(Visibility::Hidden), "visibility:hidden;"),
        (ORDER.declare(2), "order:2;"),
        (POSITION.declare(Position::Sticky), "position:sticky;"),
        (Z_INDEX.declare(10), "z-index:10;"),
        (PADDING.declare(px(4)), "padding:4px;"),
        (
            PADDING_TOP.declare(var(&space)),
            "padding-top:var(--space);",
        ),
        (PADDING_BLOCK.declare(rem(1)), "padding-block:1rem;"),
        (MARGIN.declare(LengthPercentageAuto::Auto), "margin:auto;"),
        (
            MARGIN_INLINE.declare(LengthPercentageAuto::Auto),
            "margin-inline:auto;",
        ),
        (INSET.declare(px(0)), "inset:0px;"),
        (
            INSET_BLOCK.declare(LengthPercentageAuto::Auto),
            "inset-block:auto;",
        ),
        (TOP.declare(pct(50)), "top:50%;"),
        (WIDTH.declare(pct(100) - px(20)), "width:calc(100% - 20px);"),
        (MAX_WIDTH.declare(MaxSize::None), "max-width:none;"),
        (
            OVERFLOW.declare(OverflowKeyword::Hidden),
            "overflow:hidden;",
        ),
        (FLEX.declare(px(80)), "flex:80px;"),
        (FLEX_BASIS.declare(pct(25)), "flex-basis:25%;"),
        (FLEX_FLOW.declare(FlexWrap::Wrap), "flex-flow:wrap;"),
        (FLEX_GROW.declare(1), "flex-grow:1;"),
        (FLEX_SHRINK.declare(0), "flex-shrink:0;"),
        (
            ALIGN_ITEMS.declare(SelfPosition::Center),
            "align-items:center;",
        ),
        (
            ALIGN_CONTENT.declare(ContentDistribution::SpaceAround),
            "align-content:space-around;",
        ),
        (
            JUSTIFY_CONTENT.declare(PhysicalPosition::Right),
            "justify-content:right;",
        ),
        (
            JUSTIFY_SELF.declare(BaselinePosition::Last),
            "justify-self:last baseline;",
        ),
        (GAP.declare(GapValue::Normal), "gap:normal;"),
        (ROW_GAP.declare(px(8)), "row-gap:8px;"),
        (
            GRID_TEMPLATE_COLUMNS.declare(px(200)),
            "grid-template-columns:200px;",
        ),
        (GRID_TEMPLATE_ROWS.declare(fr(1)), "grid-template-rows:1fr;"),
        (
            GRID_TEMPLATE_COLUMNS.declare(GridRepeat::count(3, [fr(1)])),
            "grid-template-columns:repeat(3, 1fr);",
        ),
        (GRID_AUTO_ROWS.declare(px(40)), "grid-auto-rows:40px;"),
        (
            GRID_COLUMN.declare(GridLine::span(2)),
            "grid-column:span 2;",
        ),
        (
            TRANSFORM.declare(TransformFunction::rotate(deg(3))),
            "transform:rotate(3deg);",
        ),
        (TRANSLATE.declare(px(4)), "translate:4px;"),
        (ROTATE.declare(deg(15)), "rotate:15deg;"),
        (COLOR.declare(NamedColor::Red), "color:red;"),
        (
            BACKGROUND_COLOR.declare(SystemColor::Canvas),
            "background-color:Canvas;",
        ),
        (OPACITY.declare(UnitInterval::new(0.5)), "opacity:0.5;"),
        (BORDER_RADIUS.declare(px(6)), "border-radius:6px;"),
        (
            BORDER_RADIUS.declare(BorderRadius::all(CornerRadius::elliptical(px(6), px(3)))),
            "border-radius:6px / 3px;",
        ),
        (
            BORDER_TOP_LEFT_RADIUS.declare(px(2)),
            "border-top-left-radius:2px;",
        ),
        (FONT_SIZE.declare(rem(1.25)), "font-size:1.25rem;"),
        (
            FONT_WEIGHT.declare(FontWeightNumber::new(600)),
            "font-weight:600;",
        ),
        (LINE_HEIGHT.declare(px(24)), "line-height:24px;"),
        (
            TEXT_DECORATION_LINE.declare(TextDecorationLines::underline()),
            "text-decoration-line:underline;",
        ),
        (CURSOR.declare(Cursor::Pointer), "cursor:pointer;"),
        (
            TOUCH_ACTION.declare(TouchActionGestures::pinch_zoom()),
            "touch-action:pinch-zoom;",
        ),
        (
            VIEW_TRANSITION_NAME.declare(ViewTransitionName::named("hero")),
            "view-transition-name:hero;",
        ),
        (ALL.declare_global(GlobalKeyword::Unset), "all:unset;"),
        (
            BOX_SIZING.declare(BoxSizing::BorderBox),
            "box-sizing:border-box;",
        ),
        (
            ASPECT_RATIO.declare(AspectRatio::new(16, 9)),
            "aspect-ratio:16 / 9;",
        ),
        (ASPECT_RATIO.declare(Auto), "aspect-ratio:auto;"),
        (TEXT_ALIGN.declare(TextAlign::Center), "text-align:center;"),
        (
            POINTER_EVENTS.declare(PointerEvents::None),
            "pointer-events:none;",
        ),
        (USER_SELECT.declare(UserSelect::None), "user-select:none;"),
        (
            SCROLL_BEHAVIOR.declare(ScrollBehavior::Smooth),
            "scroll-behavior:smooth;",
        ),
        (
            SCROLLBAR_GUTTER.declare(ScrollbarGutter::StableBothEdges),
            "scrollbar-gutter:stable both-edges;",
        ),
        (
            OVERSCROLL_BEHAVIOR.declare(OverscrollKeyword::Contain),
            "overscroll-behavior:contain;",
        ),
        (
            OVERSCROLL_BEHAVIOR_Y.declare(OverscrollKeyword::None),
            "overscroll-behavior-y:none;",
        ),
        (
            PLACE_CONTENT.declare(ContentPosition::Center),
            "place-content:center;",
        ),
        (
            PLACE_ITEMS.declare(SelfPosition::Start),
            "place-items:start;",
        ),
        (PLACE_SELF.declare(Auto), "place-self:auto;"),
        (
            TEXT_DECORATION_COLOR.declare(NamedColor::Red),
            "text-decoration-color:red;",
        ),
        (
            TEXT_DECORATION_STYLE.declare(TextDecorationStyle::Wavy),
            "text-decoration-style:wavy;",
        ),
        (
            TEXT_DECORATION_THICKNESS.declare(px(2)),
            "text-decoration-thickness:2px;",
        ),
        (
            TEXT_UNDERLINE_OFFSET.declare(em(0.2)),
            "text-underline-offset:0.2em;",
        ),
        (
            TEXT_UNDERLINE_OFFSET.declare(em(0.1) + px(1)),
            "text-underline-offset:calc(0.1em + 1px);",
        ),
        (
            TEXT_DECORATION_THICKNESS.declare(var(&space)),
            "text-decoration-thickness:var(--space);",
        ),
        (
            TRANSFORM.declare(TransformFunction::perspective(px(800))),
            "transform:perspective(800px);",
        ),
        (WIDTH.declare(Auto), "width:auto;"),
        (MARGIN_INLINE.declare(Auto), "margin-inline:auto;"),
        (INSET.declare(Auto), "inset:auto;"),
        (Z_INDEX.declare(Auto), "z-index:auto;"),
        (FLEX.declare(Auto), "flex:auto;"),
        (FLEX_BASIS.declare(Auto), "flex-basis:auto;"),
        (GRID_COLUMN.declare(Auto), "grid-column:auto;"),
        (GRID_AUTO_ROWS.declare(Auto), "grid-auto-rows:auto;"),
        (OVERFLOW.declare(Auto), "overflow:auto;"),
        (CURSOR.declare(Auto), "cursor:auto;"),
        (SCALE.declare(Number::new(1.5)), "scale:1.5;"),
        (FONT_STYLE.declare(FontStyle::Italic), "font-style:italic;"),
        (
            FLEX_DIRECTION.declare(FlexDirection::Column),
            "flex-direction:column;",
        ),
        (FLEX_WRAP.declare(FlexWrap::Wrap), "flex-wrap:wrap;"),
        (
            GRID_AUTO_FLOW.declare(GridAutoFlow::ColumnDense),
            "grid-auto-flow:column dense;",
        ),
        (ALIGN_SELF.declare(Auto), "align-self:auto;"),
        (
            JUSTIFY_ITEMS.declare(SelfPosition::Center),
            "justify-items:center;",
        ),
        (
            OVERFLOW_X.declare(OverflowKeyword::Scroll),
            "overflow-x:scroll;",
        ),
        (
            GRID_ROW_START.declare(GridLine::span(2)),
            "grid-row-start:span 2;",
        ),
        (
            FORCED_COLOR_ADJUST.declare(ForcedColorAdjust::None),
            "forced-color-adjust:none;",
        ),
        (
            PRINT_COLOR_ADJUST.declare(PrintColorAdjust::Exact),
            "print-color-adjust:exact;",
        ),
        (FLEX.declare(2), "flex:2;"),
        (
            GRID_TEMPLATE_COLUMNS.declare(TrackSize::minmax(Auto, fr(1))),
            "grid-template-columns:minmax(auto, 1fr);",
        ),
        (
            WIDTH.declare(NonNegativeLengthPercentage::from(px(-4))),
            "width:calc(-4px);",
        ),
    ];
    for (declaration, expected) in cases {
        assert_that!(declaration.to_string()).is_equal_to(expected);
    }
}

#[test]
fn shorthands_are_written_in_their_shortest_form() {
    assert_that!(BorderRadius::all(CornerRadius::elliptical(px(1), px(1))))
        .has_display_value("1px");
    assert_that!(CornerRadius::elliptical(px(2), px(2))).has_display_value("2px");
}

#[test]
fn view_transition_names_reject_match_element() {
    assert_that!(ViewTransitionName::try_named("match-element")).is_err();
}

#[test]
fn automatic_repeats_need_fixed_track_sizes() {
    assert_that!(GridRepeat::try_new(RepeatCount::AutoFill, [fr(1)])).is_err();
    assert_that!(GridRepeat::try_new(
        RepeatCount::AutoFit,
        [TrackSize::fit_content(px(10))]
    ))
    .is_err();
    assert_that!(GridRepeat::try_new(
        RepeatCount::AutoFill,
        [TrackSize::minmax(px(80), fr(1))]
    ))
    .is_ok();
}

#[test]
fn grid_line_ranges_leave_out_an_auto_end() {
    assert_that!(GridLine::line(1).to(GridLine::Auto)).has_display_value("1");
    assert_that!(GridLine::line(1).to(GridLine::span(2))).has_display_value("1 / span 2");
}

#[test]
fn translucent_named_colors_have_a_clean_percentage() {
    assert_that!(Color::from(NamedColor::Red).with_alpha(0.07))
        .has_display_value("color-mix(in srgb, red 7%, transparent)");
}

#[test]
fn place_shorthands_keep_both_values_unless_they_match() {
    assert_that!(PlaceContent::new(
        ContentPosition::Center,
        ContentPosition::Center
    ))
    .has_display_value("center");
    assert_that!(PlaceContent::new(
        BaselinePosition::First,
        ContentPosition::Start
    ))
    .has_display_value("baseline start");
    assert_that!(PlaceItems::new(
        SelfPosition::Center,
        PhysicalPosition::Left
    ))
    .has_display_value("center left");
}

#[test]
fn aspect_ratios_and_perspective_check_their_numbers() {
    assert_that!(AspectRatio::new(1, 1)).has_display_value("1");
    assert_that!(AspectRatio::new(4, 3).or_natural()).has_display_value("auto 4 / 3");
    assert_that!(AspectRatio::try_new(-1, 1)).is_err();
    assert_that!(TransformFunction::try_perspective(px(-1))).is_err();
    assert_that!(TransformFunction::matrix_3d([
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0
    ]))
    .has_display_value("matrix3d(1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1)");
}

#[test]
fn transform_properties_have_constructors_for_every_form() {
    assert_that!(Translate::xyz(px(1), pct(2), px(3))).has_display_value("1px 2% 3px");
    assert_that!(Scale::xy(2, 0.5)).has_display_value("2 0.5");
    assert_that!(Scale::xyz(1, 2, 3)).has_display_value("1 2 3");
    assert_that!(Rotate::x(deg(10))).has_display_value("x 10deg");
    assert_that!(Rotate::vector(1, 1, 0, deg(45))).has_display_value("1 1 0 45deg");
}

#[test]
fn fallible_transform_constructors_reject_non_finite_numbers() {
    assert_that!(TransformFunction::try_scale(2, f64::NAN)).is_err();
    assert_that!(TransformFunction::try_scale_x(f64::INFINITY)).is_err();
    assert_that!(TransformFunction::try_scale_y(f64::NAN)).is_err();
    assert_that!(TransformFunction::try_scale_z(f64::NAN)).is_err();
    assert_that!(TransformFunction::try_scale_3d(1, 1, f64::NAN)).is_err();
    assert_that!(TransformFunction::try_rotate_3d(f64::NAN, 0, 0, deg(1))).is_err();
    assert_that!(TransformFunction::try_matrix([
        1.0,
        0.0,
        0.0,
        1.0,
        f64::NAN,
        0.0
    ]))
    .is_err();
    let mut identity = [0.0; 16];
    identity[15] = f64::INFINITY;
    assert_that!(TransformFunction::try_matrix_3d(identity)).is_err();
    assert_that!(Rotate::try_vector(0, f64::NAN, 1, deg(1))).is_err();
    assert_that!(Scale::try_uniform(f64::NAN)).is_err();
    assert_that!(Scale::try_xy(1, f64::NAN)).is_err();
    assert_that!(Scale::try_xyz(1, 1, f64::INFINITY)).is_err();

    assert_that!(TransformFunction::try_scale(2, 3).unwrap()).has_display_value("scale(2, 3)");
    assert_that!(TransformFunction::try_matrix([1.0, 0.0, 0.0, 1.0, 5.0, 0.0]).unwrap())
        .has_display_value("matrix(1, 0, 0, 1, 5, 0)");
    assert_that!(Scale::try_xy(2, 0.5).unwrap()).has_display_value("2 0.5");
}

#[test]
fn display_writes_multi_keyword_values_in_the_shortest_form() {
    assert_that!(Display::InlineListItem).has_display_value("inline list-item");
    assert_that!(Display::FlowRootListItem).has_display_value("flow-root list-item");
    assert_that!(Display::InlineFlowRootListItem).has_display_value("inline flow-root list-item");
}

#[test]
fn non_negative_custom_properties_work_in_every_non_negative_grammar() {
    use typed_css::property::{BORDER_RADIUS, FONT_SIZE, GAP, SCALE, WIDTH};

    custom_property!(INSET: NonNegativeLengthPercentage = "--inset");
    custom_property!(FACTOR: Number = "--factor");

    assert_that!(WIDTH.declare(var(&INSET)).value()).is_equal_to("var(--inset)");
    assert_that!(GAP.declare(var(&INSET)).value()).is_equal_to("var(--inset)");
    assert_that!(FONT_SIZE.declare(var(&INSET)).value()).is_equal_to("var(--inset)");
    assert_that!(BORDER_RADIUS.declare(var(&INSET)).value()).is_equal_to("var(--inset)");
    assert_that!(SCALE.declare(var(&FACTOR)).value()).is_equal_to("var(--factor)");
}

#[test]
fn every_transform_function_has_its_css_name() {
    let cases = [
        (
            TransformFunction::translate(px(1), px(2)),
            "translate(1px, 2px)",
        ),
        (TransformFunction::translate_x(px(1)), "translateX(1px)"),
        (TransformFunction::translate_y(pct(2)), "translateY(2%)"),
        (TransformFunction::translate_z(px(3)), "translateZ(3px)"),
        (
            TransformFunction::translate_3d(px(1), px(2), px(3)),
            "translate3d(1px, 2px, 3px)",
        ),
        (TransformFunction::scale(2, 3), "scale(2, 3)"),
        (TransformFunction::scale_x(2), "scaleX(2)"),
        (TransformFunction::scale_y(2), "scaleY(2)"),
        (TransformFunction::scale_z(2), "scaleZ(2)"),
        (TransformFunction::scale_3d(1, 2, 3), "scale3d(1, 2, 3)"),
        (TransformFunction::rotate(deg(1)), "rotate(1deg)"),
        (TransformFunction::rotate_x(deg(1)), "rotateX(1deg)"),
        (TransformFunction::rotate_y(deg(1)), "rotateY(1deg)"),
        (TransformFunction::rotate_z(deg(1)), "rotateZ(1deg)"),
        (
            TransformFunction::rotate_3d(1, 0, 0, deg(1)),
            "rotate3d(1, 0, 0, 1deg)",
        ),
        (TransformFunction::skew(deg(1), deg(2)), "skew(1deg, 2deg)"),
        (TransformFunction::skew_x(deg(1)), "skewX(1deg)"),
        (TransformFunction::skew_y(deg(1)), "skewY(1deg)"),
        (
            TransformFunction::perspective(px(500)),
            "perspective(500px)",
        ),
    ];
    for (function, expected) in cases {
        assert_that!(function.to_string()).is_equal_to(expected);
    }
}

#[test]
fn fallible_color_helpers_check_their_input() {
    assert_that!(Color::try_hex(0x001E_90FF).unwrap()).has_display_value("rgb(30 144 255)");
    assert_that!(Color::try_hex(0x0100_0000)).is_err();
    assert_that!(rgb(1, 2, 3).try_with_alpha(1.5)).is_err();
    assert_that!(light_dark(NamedColor::White, rgb(0, 0, 0)).with_alpha(0.5)).has_display_value(
        "light-dark(color-mix(in srgb, white 50%, transparent), rgb(0 0 0 / 0.5))",
    );
}

#[test]
fn non_negative_references_work_in_constructors() {
    custom_property!(SPACE: NonNegativeLengthPercentage = "--space");
    assert_that!(Padding::symmetric(var(&SPACE), px(4))).has_display_value("var(--space) 4px");
    assert_that!(TrackSize::minmax(var(&SPACE), fr(1)))
        .has_display_value("minmax(var(--space), 1fr)");
    assert_that!(NonNegativeLengthPercentage::from(var_or(&SPACE, px(-2))))
        .has_display_value("var(--space, calc(-2px))");
}

#[test]
fn aspect_ratios_convert_from_a_number() {
    use typed_css::property::ASPECT_RATIO;
    assert_that!(ASPECT_RATIO.declare(NonNegativeNumber::new(1.5)).value()).is_equal_to("1.5");
}

#[test]
fn two_value_shorthands_share_their_constructors() {
    assert_that!(Gap::both(px(4)).with_column(GapValue::Normal)).has_display_value("4px normal");
    assert_that!(Gap::new(px(4), px(8)).with_row(px(8))).has_display_value("8px");
    assert_that!(Overflow::both(OverflowKeyword::Hidden).with_y(OverflowKeyword::Auto))
        .has_display_value("hidden auto");
    assert_that!(
        OverscrollBehavior::new(OverscrollKeyword::Auto, OverscrollKeyword::None)
            .with_x(OverscrollKeyword::None)
    )
    .has_display_value("none");
    assert_that!(PaddingAxis::both(px(4)).with_end(px(8))).has_display_value("4px 8px");
    assert_that!(InsetAxis::new(px(1), px(2)).with_start(px(2))).has_display_value("2px");
    assert_that!(PlaceItems::both(SelfPosition::Center).with_justify(SelfPosition::End))
        .has_display_value("center end");
    assert_that!(PlaceSelf::new(AlignSelf::Auto, JustifySelf::Auto).with_align(SelfPosition::End))
        .has_display_value("end auto");
}

#[test]
fn positions_take_an_overflow_position() {
    use typed_css::property::{JUSTIFY_ITEMS, PLACE_CONTENT};

    assert_that!(JUSTIFY_ITEMS.declare(PhysicalPosition::Left.safe()).value())
        .is_equal_to("safe left");
    assert_that!(PLACE_CONTENT.declare(ContentPosition::End.safe()).value())
        .is_equal_to("safe end");
    assert_that!(SelfPosition::Start.with_overflow(OverflowPosition::Unsafe))
        .has_display_value("unsafe start");
}

#[test]
fn values_with_the_same_css_are_equal() {
    custom_property!(SPACE: LengthPercentage = "--space");
    let reference: DeclarationValue<LengthPercentage> = var(&SPACE).into();
    let inside: DeclarationValue<LengthPercentage> = LengthPercentage::from(var(&SPACE)).into();
    assert_that!(reference == inside).is_true();
    assert_that!(GridLine::line(1).to(GridLine::Auto) == GridLineRange::from(GridLine::line(1)))
        .is_true();
}

#[test]
fn numeric_values_have_fallible_counterparts() {
    assert_that!(FontWeight::try_new(0)).is_err();
    assert_that!(Flex::grow(1).try_with_grow(-1)).is_err();
    assert_that!(Flex::grow(1).try_with_shrink(f64::NAN)).is_err();
    assert_that!(Opacity::percent(50)).has_display_value("50%");
    assert_that!(Opacity::try_percent(101)).is_err();
}
