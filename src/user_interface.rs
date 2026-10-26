use crate::auto::auto_sources;
use crate::checked_value::{impl_checked_value, keyword_enum};

keyword_enum! {
    /// CSS `cursor` keywords.
    ///
    /// Custom cursor images are not modeled, because they would require an unchecked URL.
    ///
    /// [CSS Basic User Interface Level 4 `cursor` grammar](https://www.w3.org/TR/css-ui-4/#cursor)
    pub enum Cursor {
        Auto => "auto" as auto,
        Default => "default" as default,
        None => "none" as none,
        ContextMenu => "context-menu" as context_menu,
        Help => "help" as help,
        Pointer => "pointer" as pointer,
        Progress => "progress" as progress,
        Wait => "wait" as wait,
        Cell => "cell" as cell,
        Crosshair => "crosshair" as crosshair,
        Text => "text" as text,
        VerticalText => "vertical-text" as vertical_text,
        Alias => "alias" as alias,
        Copy => "copy" as copy,
        Move => "move" as r#move,
        NoDrop => "no-drop" as no_drop,
        NotAllowed => "not-allowed" as not_allowed,
        Grab => "grab" as grab,
        Grabbing => "grabbing" as grabbing,
        EResize => "e-resize" as e_resize,
        NResize => "n-resize" as n_resize,
        NeResize => "ne-resize" as ne_resize,
        NwResize => "nw-resize" as nw_resize,
        SResize => "s-resize" as s_resize,
        SeResize => "se-resize" as se_resize,
        SwResize => "sw-resize" as sw_resize,
        WResize => "w-resize" as w_resize,
        EwResize => "ew-resize" as ew_resize,
        NsResize => "ns-resize" as ns_resize,
        NeswResize => "nesw-resize" as nesw_resize,
        NwseResize => "nwse-resize" as nwse_resize,
        ColResize => "col-resize" as col_resize,
        RowResize => "row-resize" as row_resize,
        AllScroll => "all-scroll" as all_scroll,
        ZoomIn => "zoom-in" as zoom_in,
        ZoomOut => "zoom-out" as zoom_out,
    }
}
keyword_enum! {
    /// Whether an element can be the target of pointer events, such as clicks and hovers.
    ///
    /// With `none`, the events go to the element behind it. Descendants inherit the value, but a
    /// descendant with `auto` can still be a target.
    ///
    /// [CSS Basic User Interface Level 4 `pointer-events`](https://www.w3.org/TR/css-ui-4/#pointer-events-control)
    pub enum PointerEvents {
        Auto => "auto" as auto,
        None => "none" as none,
    }
}

keyword_enum! {
    /// Whether and how the user can select the text of an element.
    ///
    /// [CSS Basic User Interface Level 4 `user-select`](https://www.w3.org/TR/css-ui-4/#content-selection)
    pub enum UserSelect {
        /// The browser decides, which usually means that the text is selectable.
        Auto => "auto" as auto,
        /// The text is selectable.
        Text => "text" as text,
        /// The text is not selectable.
        None => "none" as none,
        /// A selection that starts inside the element stays inside it.
        Contain => "contain" as contain,
        /// A selection that touches the element selects all of its text at once.
        All => "all" as all,
    }
}

auto_sources!(
    Cursor => Self::Auto,
    PointerEvents => Self::Auto,
    UserSelect => Self::Auto,
);

impl_checked_value!(Cursor, PointerEvents, UserSelect);
