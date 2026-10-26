//! Direct Leptos integration tests for checked declarations.

use assertr::prelude::*;
use leptos::tachys::html::{attribute::Attribute, style::IntoStyle};
use typed_css::{property::*, *};

custom_property!(ACCENT_COLOR: Color = "--accent-color");

fn render_style_attribute(attribute: impl Attribute) -> String {
    let mut html = String::new();
    let mut class = String::new();
    let mut styles = String::new();
    let mut inner_html = String::new();

    attribute.to_html(&mut html, &mut class, &mut styles, &mut inner_html);
    styles
}

fn render_style(style: impl IntoStyle) -> String {
    let mut output = String::new();
    style.to_html(&mut output);
    output
}

#[test]
fn all_property_accepts_only_a_global_keyword() {
    let attribute = ALL
        .declare_global(GlobalKeyword::RevertLayer)
        .into_attribute();

    assert_that!(render_style_attribute(attribute)).is_equal_to("all:revert-layer;");
}

#[test]
fn ordinary_properties_accept_global_keywords_through_a_separate_method() {
    assert_that!(render_style(PADDING.declare_global(GlobalKeyword::Inherit)))
        .is_equal_to("padding:inherit;");
}

#[test]
fn properties_accept_grammar_components_directly() {
    assert_that!(render_style(WIDTH.declare(px(120)))).is_equal_to("width:120px;");
    assert_that!(render_style(WIDTH.declare(pct(100) - px(20))))
        .is_equal_to("width:calc(100% - 20px);");
    assert_that!(render_style(PADDING.declare(px(16)))).is_equal_to("padding:16px;");
    assert_that!(render_style(PADDING_TOP.declare(px(12)))).is_equal_to("padding-top:12px;");
    assert_that!(render_style(MARGIN_INLINE.declare(px(-8)))).is_equal_to("margin-inline:-8px;");
    assert_that!(render_style(GAP.declare(rem(1)))).is_equal_to("gap:1rem;");
    assert_that!(render_style(BORDER_RADIUS.declare(px(6)))).is_equal_to("border-radius:6px;");
    assert_that!(render_style(
        FONT_WEIGHT.declare(FontWeightNumber::new(600))
    ))
    .is_equal_to("font-weight:600;");
    assert_that!(render_style(ROTATE.declare(deg(15)))).is_equal_to("rotate:15deg;");
    assert_that!(render_style(OVERFLOW.declare(OverflowKeyword::Hidden)))
        .is_equal_to("overflow:hidden;");
    assert_that!(render_style(
        JUSTIFY_CONTENT.declare(ContentPosition::Center)
    ))
    .is_equal_to("justify-content:center;");
    assert_that!(render_style(GRID_COLUMN.declare(GridLine::span(2))))
        .is_equal_to("grid-column:span 2;");
    assert_that!(render_style(GRID_AUTO_ROWS.declare(fr(1)))).is_equal_to("grid-auto-rows:1fr;");
    assert_that!(render_style(ORDER.declare(-1))).is_equal_to("order:-1;");
    assert_that!(render_style(FLEX_GROW.declare(NonNegativeNumber::new(2.0))))
        .is_equal_to("flex-grow:2;");
}

#[test]
fn layout_properties_render_their_grammars() {
    assert_that!(render_style(DISPLAY.declare(Display::Grid))).is_equal_to("display:grid;");
    assert_that!(render_style(POSITION.declare(Position::Absolute)))
        .is_equal_to("position:absolute;");
    assert_that!(render_style(
        TRANSFORM.declare(TransformFunction::translate(px(4), px(8)))
    ))
    .is_equal_to("transform:translate(4px, 8px);");
    assert_that!(render_style(FONT_SIZE.declare(rem(1.5)))).is_equal_to("font-size:1.5rem;");
    assert_that!(render_style(LINE_HEIGHT.declare(LineHeight::number(1.4))))
        .is_equal_to("line-height:1.4;");
    assert_that!(render_style(CURSOR.declare(Cursor::Pointer))).is_equal_to("cursor:pointer;");
    assert_that!(render_style(FLEX.declare(Flex::grow(1)))).is_equal_to("flex:1;");
}

#[test]
fn properties_sharing_a_grammar_share_a_type() {
    fn tint(target: Property<Color>) -> Declaration {
        target.declare(hex(0x000c_2238))
    }

    assert_that!(COLOR.name()).is_equal_to("color");
    assert_that!(render_style(tint(COLOR))).is_equal_to("color:rgb(12 34 56);");
    assert_that!(render_style(tint(BACKGROUND_COLOR)))
        .is_equal_to("background-color:rgb(12 34 56);");
    assert_that!(tint(COLOR)).is_not_equal_to(tint(BACKGROUND_COLOR));
}

#[test]
fn typed_custom_properties_and_var_render_during_ssr() {
    let runtime_accent = CustomProperty::<Color>::new("--runtime-accent-color");

    assert_that!(render_style(ACCENT_COLOR.declare(NamedColor::Aqua)))
        .is_equal_to("--accent-color:aqua;");
    assert_that!(render_style(runtime_accent.declare(NamedColor::Red)))
        .is_equal_to("--runtime-accent-color:red;");
    assert_that!(render_style(
        COLOR.declare(var_or(&ACCENT_COLOR, NamedColor::CurrentColor))
    ))
    .is_equal_to("color:var(--accent-color, currentcolor);");
}

#[test]
fn checked_declarations_expose_their_serialized_parts() {
    let declaration = BACKGROUND_COLOR.declare(oklch(0.7, 0.1, deg(120)));

    assert_that!(declaration.property_name()).is_equal_to("background-color");
    assert_that!(declaration.value()).is_equal_to("oklch(0.7 0.1 120deg)");
    assert_that!(declaration.to_string()).is_equal_to("background-color:oklch(0.7 0.1 120deg);");
}

/// The same code as the Leptos example in the README.
#[test]
fn readme_example_compiles() {
    use leptos::prelude::*;

    let owner = Owner::new();
    owner.with(|| {
        let (highlighted, _set_highlighted) = signal(false);
        let _view = view! {
            <div style=PADDING.declare(px(16))>"Padded"</div>
            <p style=move || COLOR.declare(if highlighted.get() { NamedColor::Crimson } else { NamedColor::Black })>
                "Reactive color"
            </p>
        };
    });
}
