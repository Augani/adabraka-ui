//! Table - Simple table component for structured data display.

use gpui::{prelude::FluentBuilder as _, *};
use crate::theme::use_theme;

#[derive(Clone)]
pub struct TableColumn {
    pub header: SharedString,
    pub width: Option<Pixels>,
}

impl TableColumn {
    pub fn new<T: Into<SharedString>>(header: T) -> Self {
        Self {
            header: header.into(),
            width: None,
        }
    }

    pub fn width(mut self, width: impl Into<Pixels>) -> Self {
        self.width = Some(width.into());
        self
    }
}

#[derive(Clone)]
pub struct TableRow {
    pub cells: Vec<SharedString>,
    pub selected: bool,
}

impl TableRow {
    pub fn new(cells: Vec<SharedString>) -> Self {
        Self {
            cells,
            selected: false,
        }
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

pub struct Table {
    columns: Vec<TableColumn>,
    rows: Vec<TableRow>,
    style: StyleRefinement,
}

impl Table {
    pub fn new() -> Self {
        Self {
            columns: Vec::new(),
            rows: Vec::new(),
            style: StyleRefinement::default(),
        }
    }

    pub fn columns(mut self, columns: Vec<TableColumn>) -> Self {
        self.columns = columns;
        self
    }

    pub fn rows(mut self, rows: Vec<TableRow>) -> Self {
        self.rows = rows;
        self
    }
}

impl Styled for Table {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl IntoElement for Table {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        let theme = use_theme();
        let user_style = self.style;

        let header_cells = self.columns.iter().map(|column| {
            let width = column.width.unwrap_or(px(120.0));
            div()
                .flex()
                .items_center()
                .px(px(12.0))
                .py(px(8.0))
                .w(width)
                .text_size(px(12.0))
                .font_family(theme.tokens.font_family.clone())
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(theme.tokens.muted_foreground)
                .border_b_1()
                .border_color(theme.tokens.border)
                .child(column.header.clone())
        });

        let header = div()
            .flex()
            .children(header_cells);

        let row_elements = self.rows.into_iter().map(|row| {
            let cell_elements = row.cells.iter().enumerate().map(|(col_index, cell)| {
                let column = &self.columns[col_index];
                let width = column.width.unwrap_or(px(120.0));

                div()
                    .flex()
                    .items_center()
                    .px(px(12.0))
                    .py(px(8.0))
                    .w(width)
                    .text_size(px(13.0))
                    .font_family(theme.tokens.font_family.clone())
                    .text_color(theme.tokens.foreground)
                    .border_b_1()
                    .border_color(theme.tokens.border.opacity(0.5))
                    .bg(if row.selected {
                        theme.tokens.accent.opacity(0.3)
                    } else {
                        gpui::transparent_black()
                    })
                    .child(cell.clone())
            });

            div()
                .flex()
                .hover(|style| style.bg(theme.tokens.accent.opacity(0.1)))
                .children(cell_elements)
        });

        div()
            .flex()
            .flex_col()
            .border_1()
            .border_color(theme.tokens.border)
            .rounded(theme.tokens.radius_md)
            .overflow_hidden()
            .child(header)
            .children(row_elements)
            .map(|this| {
                let mut div = this;
                div.style().refine(&user_style);
                div
            })
    }
}
