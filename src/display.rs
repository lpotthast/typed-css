use crate::checked_value::{impl_checked_value, keyword_enum};

keyword_enum! {
    /// A `display` value, written in its shortest form.
    ///
    /// [CSS Display Level 3 `display` grammar](https://www.w3.org/TR/css-display-3/#the-display-properties)
    pub enum Display {
        None => "none" as none,
        /// The element generates no box, and its children are laid out as children of its parent.
        Contents => "contents" as contents,
        Block => "block" as block,
        Inline => "inline" as inline,
        InlineBlock => "inline-block" as inline_block,
        /// A block that contains its floats and the margins of its children.
        FlowRoot => "flow-root" as flow_root,
        Flex => "flex" as flex,
        InlineFlex => "inline-flex" as inline_flex,
        Grid => "grid" as grid,
        InlineGrid => "inline-grid" as inline_grid,
        ListItem => "list-item" as list_item,
        /// An inline box that generates a list marker, the same as `inline flow list-item`.
        InlineListItem => "inline list-item" as inline_list_item,
        /// A list item that contains its floats, the same as `block flow-root list-item`.
        FlowRootListItem => "flow-root list-item" as flow_root_list_item,
        /// An inline list item that contains its floats.
        InlineFlowRootListItem => "inline flow-root list-item" as inline_flow_root_list_item,
        Table => "table" as table,
        InlineTable => "inline-table" as inline_table,
        TableRowGroup => "table-row-group" as table_row_group,
        TableHeaderGroup => "table-header-group" as table_header_group,
        TableFooterGroup => "table-footer-group" as table_footer_group,
        TableRow => "table-row" as table_row,
        TableCell => "table-cell" as table_cell,
        TableColumnGroup => "table-column-group" as table_column_group,
        TableColumn => "table-column" as table_column,
        TableCaption => "table-caption" as table_caption,
    }
}

keyword_enum! {
    /// A `visibility` value.
    ///
    /// [CSS Display Level 4 `visibility` grammar](https://www.w3.org/TR/css-display-4/#visibility)
    pub enum Visibility {
        Visible => "visible" as visible,
        Hidden => "hidden" as hidden,
        /// The element is hidden, and a table row or column also gives up its space.
        Collapse => "collapse" as collapse,
    }
}
impl_checked_value!(Display, Visibility);
