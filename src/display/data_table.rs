//! DataTable - High-performance table component with virtual scrolling and sorting.

use gpui::{prelude::FluentBuilder as _, *};
use std::collections::{HashMap, HashSet};
use std::ops::Range;
use std::rc::Rc;
use crate::theme::use_theme;
use crate::components::input::{Input, InputState, InputSize};
use crate::components::select::{Select, SelectOption, SelectEvent};
use crate::components::icon_source::IconSource;
use crate::virtual_list::{vlist_uniform_view};

#[derive(Clone)]
pub struct RowAction {
    pub id: SharedString,
    pub label: SharedString,
    pub icon: Option<IconSource>,
    pub destructive: bool,
    pub on_click: Rc<dyn Fn(usize, &mut Window, &mut App)>,
}

impl RowAction {
    pub fn new<S: Into<SharedString>, F: Fn(usize, &mut Window, &mut App) + 'static>(
        id: S,
        label: S,
        handler: F,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon: None,
            destructive: false,
            on_click: Rc::new(handler),
        }
    }

    pub fn icon(mut self, icon: impl Into<IconSource>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn destructive(mut self) -> Self {
        self.destructive = true;
        self
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SortDirection {
    Ascending,
    Descending,
}

#[derive(Debug, Clone)]
struct ViewportState {
    viewport_height: f32,
    row_height: f32,
}

impl ViewportState {
    fn new(row_height: f32, viewport_height: f32) -> Self {
        Self {
            viewport_height,
            row_height,
        }
    }
}

struct VirtualScroller {
    viewport: ViewportState,
    total_items: usize,
}

impl VirtualScroller {
    fn new(total_items: usize, viewport: ViewportState) -> Self {
        Self {
            viewport,
            total_items,
        }
    }

    fn total_height(&self) -> f32 {
        self.total_items as f32 * self.viewport.row_height
    }

    fn set_total_items(&mut self, count: usize) {
        self.total_items = count;
    }
}

pub struct ColumnDef<T: 'static> {
    pub id: SharedString,
    pub header: SharedString,
    pub accessor: Rc<dyn Fn(&T) -> SharedString>,
    pub width: Pixels,
    pub min_width: Pixels,
    pub resizable: bool,
    pub sortable: bool,
    pub editable: bool,
}

impl<T: 'static> ColumnDef<T> {
    pub fn new<S: Into<SharedString>, F: Fn(&T) -> SharedString + 'static>(
        id: S,
        header: S,
        accessor: F,
    ) -> Self {
        let id_string: SharedString = id.into();
        let header_string: SharedString = header.into();

        Self {
            id: id_string,
            header: header_string,
            accessor: Rc::new(accessor),
            width: px(150.0),
            min_width: px(80.0),
            resizable: true,
            sortable: true,
            editable: false,
        }
    }

    pub fn width(mut self, width: impl Into<Pixels>) -> Self {
        self.width = width.into();
        self
    }

    pub fn min_width(mut self, width: impl Into<Pixels>) -> Self {
        self.min_width = width.into();
        self
    }

    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    pub fn sortable(mut self, sortable: bool) -> Self {
        self.sortable = sortable;
        self
    }

    pub fn editable(mut self, editable: bool) -> Self {
        self.editable = editable;
        self
    }
}

enum DataBacking<T: Clone + 'static> {
    InMemory { data: Vec<T> },
    Virtual {
        total_items: usize,
        cache: HashMap<usize, T>,
        in_flight_pages: HashSet<usize>,
        page_size: usize,
    },
}

pub struct DataTableState<T: Clone + 'static> {
    columns: Vec<ColumnDef<T>>,
    column_widths: Vec<Pixels>,
    sort_column: Option<usize>,
    sort_direction: SortDirection,
    scroller: VirtualScroller,
    selected_rows: Vec<usize>,
    backing: DataBacking<T>,
}

impl<T: Clone + 'static> DataTableState<T> {
    pub fn new(data: Vec<T>, columns: Vec<ColumnDef<T>>) -> Self {
        let column_widths = columns.iter().map(|col| col.width).collect();
        let total_items = data.len();
        let viewport = ViewportState::new(48.0, 600.0);

        Self {
            column_widths,
            columns,
            sort_column: None,
            sort_direction: SortDirection::Ascending,
            scroller: VirtualScroller::new(total_items, viewport),
            selected_rows: Vec::new(),
            backing: DataBacking::InMemory { data },
        }
    }

    fn total_height(&self) -> f32 {
        self.scroller.total_height()
    }

    fn row_height(&self) -> f32 {
        self.scroller.viewport.row_height
    }

    fn viewport_height(&self) -> f32 {
        self.scroller.viewport.viewport_height
    }

    fn total_items(&self) -> usize {
        match &self.backing {
            DataBacking::InMemory { data } => data.len(),
            DataBacking::Virtual { total_items, .. } => *total_items,
        }
    }

    fn get_row(&self, index: usize) -> Option<&T> {
        match &self.backing {
            DataBacking::InMemory { data } => data.get(index),
            DataBacking::Virtual { cache, .. } => cache.get(&index),
        }
    }

    fn replace_in_memory_data(&mut self, data: Vec<T>) {
        let count = data.len();
        self.backing = DataBacking::InMemory { data };
        self.scroller.set_total_items(count);
    }

    fn virtual_reset(&mut self, total_items: usize, page_size: Option<usize>) {
        match &mut self.backing {
            DataBacking::Virtual {
                total_items: t,
                cache,
                in_flight_pages,
                page_size: ps,
            } => {
                *t = total_items;
                cache.clear();
                in_flight_pages.clear();
                if let Some(s) = page_size { *ps = s.max(1); }
            }
            DataBacking::InMemory { .. } => {
                self.backing = DataBacking::Virtual {
                    total_items,
                    cache: HashMap::new(),
                    in_flight_pages: HashSet::new(),
                    page_size: page_size.unwrap_or(200).max(1),
                };
            }
        }
        self.scroller.set_total_items(total_items);
    }

    fn virtual_set_page(&mut self, page_start: usize, rows: Vec<T>) {
        if let DataBacking::Virtual { cache, in_flight_pages, .. } = &mut self.backing {
            for (i, row) in rows.into_iter().enumerate() {
                cache.insert(page_start + i, row);
            }
            in_flight_pages.remove(&page_start);
        }
    }

    pub fn sort_by_column(&mut self, column_index: usize, direction: SortDirection) {
        self.sort_column = Some(column_index);
        self.sort_direction = direction;

        if let DataBacking::InMemory { data } = &mut self.backing {
            if let Some(column) = self.columns.get(column_index) {
                let mut indexed_values: Vec<(usize, String)> = data
                    .iter()
                    .enumerate()
                    .map(|(idx, row)| (idx, (column.accessor)(row).to_string()))
                    .collect();

                indexed_values.sort_by(|(_, a), (_, b)| {
                    match direction {
                        SortDirection::Ascending => a.cmp(b),
                        SortDirection::Descending => b.cmp(a),
                    }
                });

                let sorted_data: Vec<T> = indexed_values
                    .into_iter()
                    .filter_map(|(idx, _)| data.get(idx).cloned())
                    .collect();

                *data = sorted_data;
            }
        }
    }

    pub fn toggle_row(&mut self, row_index: usize) {
        if let Some(pos) = self.selected_rows.iter().position(|&i| i == row_index) {
            self.selected_rows.remove(pos);
        } else {
            self.selected_rows.push(row_index);
        }
    }

    pub fn is_row_selected(&self, row_index: usize) -> bool {
        self.selected_rows.contains(&row_index)
    }

    pub fn resize_column(&mut self, column_index: usize, new_width: Pixels) {
        if let Some(width) = self.column_widths.get_mut(column_index) {
            *width = new_width;
        }
    }
}

pub struct DataTable<T: Clone + 'static> {
    state: DataTableState<T>,
    resizing_column: Option<usize>,
    resize_start_x: f32,
    resize_start_width: Pixels,
    sticky_header: bool,
    on_load_more: Option<Box<dyn Fn(&mut Window, &mut Context<Self>) + 'static>>,
    load_more_threshold: f32,
    load_more_triggered: bool,
    scroll_handle: ScrollHandle,
    editing_cell: Option<(usize, usize)>,
    edit_input: Option<Entity<InputState>>,
    edit_column_id: SharedString,
    edit_old_value: SharedString,
    use_edit_dialog: bool,
    on_cell_edit: Option<Box<dyn Fn(usize, SharedString, SharedString, SharedString, &mut Context<Self>) + 'static>>,
    on_cell_double_click: Option<Box<dyn Fn(&T, SharedString, SharedString, &mut Window, &mut Context<Self>) + 'static>>,
    on_fetch_page: Option<Box<dyn Fn(usize, usize, &mut Window, &mut Context<Self>) + 'static>>,
    on_row_click: Option<Box<dyn Fn(usize, &T, &mut Window, &mut Context<Self>) + 'static>>,
    search_query: String,
    search_column: Option<usize>,
    show_search: bool,
    search_column_select: Entity<Select<usize>>,
    search_input: Entity<InputState>,
    show_selection: bool,
    on_selection_change: Option<Box<dyn Fn(&[usize], &mut Window, &mut Context<Self>) + 'static>>,
    row_actions: Vec<RowAction>,
    context_menu: Option<(usize, Point<Pixels>)>,
    is_dragging_horizontal: bool,
    drag_start_x: f32,
    drag_scroll_start_x: f32,
    style: StyleRefinement,
}

impl<T: Clone + 'static> DataTable<T> {
    pub fn new(data: Vec<T>, columns: Vec<ColumnDef<T>>, cx: &mut Context<Self>) -> Self {
        let mut select_options = vec![
            SelectOption::new(usize::MAX, "All Columns")
        ];
        for (idx, column) in columns.iter().enumerate() {
            select_options.push(SelectOption::new(idx, column.header.clone()));
        }

        let search_column_select = cx.new(|cx| {
            Select::new(cx)
                .options(select_options)
                .selected_index(Some(0))
                .placeholder("Select column...")
        });

        cx.subscribe(&search_column_select, |this, _select, event: &SelectEvent, cx| {
            match event {
                SelectEvent::Change => {
                    let selected = this.search_column_select.read(cx).selected_value().copied();
                    this.search_column = if selected == Some(usize::MAX) {
                        None
                    } else {
                        selected
                    };
                    cx.notify();
                }
            }
        }).detach();

        let search_input = cx.new(|cx| InputState::new(cx));

        Self {
            state: DataTableState::new(data, columns),
            resizing_column: None,
            resize_start_x: 0.0,
            resize_start_width: px(0.0),
            sticky_header: true,
            on_load_more: None,
            load_more_threshold: 0.7,
            load_more_triggered: false,
            scroll_handle: ScrollHandle::new(),
            editing_cell: None,
            edit_input: None,
            edit_column_id: SharedString::from(""),
            edit_old_value: SharedString::from(""),
            use_edit_dialog: true,
            on_cell_edit: None,
            on_cell_double_click: None,
            on_fetch_page: None,
            on_row_click: None,
            search_query: String::new(),
            search_column: None,
            show_search: true,
            search_column_select,
            search_input,
            show_selection: false,
            on_selection_change: None,
            row_actions: Vec::new(),
            context_menu: None,
            is_dragging_horizontal: false,
            drag_start_x: 0.0,
            drag_scroll_start_x: 0.0,
            style: StyleRefinement::default(),
        }
    }

    pub fn sticky_header(mut self, sticky: bool) -> Self {
        self.sticky_header = sticky;
        self
    }

    pub fn show_selection(mut self, show: bool) -> Self {
        self.show_selection = show;
        self
    }

    pub fn on_selection_change<F>(mut self, callback: F) -> Self
    where
        F: Fn(&[usize], &mut Window, &mut Context<Self>) + 'static,
    {
        self.on_selection_change = Some(Box::new(callback));
        self
    }

    pub fn new_virtual(total_items: usize, columns: Vec<ColumnDef<T>>, page_size: usize, cx: &mut Context<Self>) -> Self {
        let mut select_options = vec![
            SelectOption::new(usize::MAX, "All Columns")
        ];
        for (idx, column) in columns.iter().enumerate() {
            select_options.push(SelectOption::new(idx, column.header.clone()));
        }

        let search_column_select = cx.new(|cx| {
            Select::new(cx)
                .options(select_options)
                .selected_index(Some(0))
                .placeholder("Select column...")
        });

        cx.subscribe(&search_column_select, |this, _select, event: &SelectEvent, cx| {
            match event {
                SelectEvent::Change => {
                    let selected = this.search_column_select.read(cx).selected_value().copied();
                    this.search_column = if selected == Some(usize::MAX) {
                        None
                    } else {
                        selected
                    };
                    cx.notify();
                }
            }
        }).detach();

        let search_input = cx.new(|cx| InputState::new(cx));

        Self {
            state: DataTableState::new(Vec::new(), columns),
            resizing_column: None,
            resize_start_x: 0.0,
            resize_start_width: px(0.0),
            sticky_header: true,
            on_load_more: None,
            load_more_threshold: 0.7,
            load_more_triggered: false,
            scroll_handle: ScrollHandle::new(),
            editing_cell: None,
            edit_input: None,
            edit_column_id: SharedString::from(""),
            edit_old_value: SharedString::from(""),
            use_edit_dialog: true,
            on_cell_edit: None,
            on_cell_double_click: None,
            on_fetch_page: None,
            on_row_click: None,
            search_query: String::new(),
            search_column: None,
            show_search: true,
            search_column_select,
            search_input,
            show_selection: false,
            on_selection_change: None,
            row_actions: Vec::new(),
            context_menu: None,
            is_dragging_horizontal: false,
            drag_start_x: 0.0,
            drag_scroll_start_x: 0.0,
            style: StyleRefinement::default(),
        }
        .with_virtual_backing(total_items, page_size)
    }

    fn with_virtual_backing(mut self, total_items: usize, page_size: usize) -> Self {
        self.state.virtual_reset(total_items, Some(page_size));
        self
    }

    /// Set callback for when user scrolls near the end
    ///
    /// This is useful for infinite scrolling / pagination - load more data when needed
    pub fn on_load_more<F>(mut self, callback: F) -> Self
    where
        F: Fn(&mut Window, &mut Context<Self>) + 'static,
    {
        self.on_load_more = Some(Box::new(callback));
        self
    }

    /// Set the threshold (0.0-1.0) for when to trigger load_more
    ///
    /// Default is 0.7 (70%) - callback fires when user scrolls past 70% of loaded data
    pub fn load_more_threshold(mut self, threshold: f32) -> Self {
        self.load_more_threshold = threshold.clamp(0.0, 1.0);
        self
    }

    pub fn on_fetch_page<F>(mut self, callback: F) -> Self
    where
        F: Fn(usize, usize, &mut Window, &mut Context<Self>) + 'static,
    {
        self.on_fetch_page = Some(Box::new(callback));
        self
    }

    /// Set callback for when a cell is edited
    ///
    /// The callback receives: (row_index, column_id, old_value, new_value, context)
    pub fn on_cell_edit<F>(mut self, callback: F) -> Self
    where
        F: Fn(usize, SharedString, SharedString, SharedString, &mut Context<Self>) + 'static,
    {
        self.on_cell_edit = Some(Box::new(callback));
        self
    }

    /// Set callback for cell double-click: (row_data, column_id, cell_value)
    pub fn on_cell_double_click<F>(mut self, callback: F) -> Self
    where
        F: Fn(&T, SharedString, SharedString, &mut Window, &mut Context<Self>) + 'static,
    {
        self.on_cell_double_click = Some(Box::new(callback));
        self
    }

    /// Set whether to use a confirmation dialog when editing cells
    ///
    /// - `true` (default): Shows a dialog with Save/Cancel buttons before applying changes
    pub fn use_edit_dialog(mut self, use_dialog: bool) -> Self {
        self.use_edit_dialog = use_dialog;
        self
    }

    /// Set callback for when a row is clicked
    ///
    pub fn on_row_click<F>(mut self, callback: F) -> Self
    where
        F: Fn(usize, &T, &mut Window, &mut Context<Self>) + 'static,
    {
        self.on_row_click = Some(Box::new(callback));
        self
    }

    pub fn show_search(mut self, show: bool) -> Self {
        self.show_search = show;
        self
    }

    pub fn row_actions(mut self, actions: Vec<RowAction>) -> Self {
        self.row_actions = actions;
        self
    }

    pub fn set_search(&mut self, query: String, cx: &mut Context<Self>) {
        self.search_query = query;
        cx.notify();
    }

    pub fn set_search_column(&mut self, column_index: Option<usize>, cx: &mut Context<Self>) {
        self.search_column = column_index;
        cx.notify();
    }

    fn row_matches_search(&self, row: &T) -> bool {
        if self.search_query.is_empty() {
            return true;
        }

        let query_lower = self.search_query.to_lowercase();

        if let Some(col_idx) = self.search_column {
            if let Some(column) = self.state.columns.get(col_idx) {
                let cell_value = (column.accessor)(row);
                cell_value.to_string().to_lowercase().contains(&query_lower)
            } else {
                false
            }
        } else {
            self.state.columns.iter().any(|column| {
                let cell_value = (column.accessor)(row);
                cell_value.to_string().to_lowercase().contains(&query_lower)
            })
        }
    }

    fn get_filtered_indices(&self) -> Vec<usize> {
        if let DataBacking::InMemory { data } = &self.state.backing {
            data.iter()
                .enumerate()
                .filter(|(_, row)| self.row_matches_search(row))
                .map(|(idx, _)| idx)
                .collect()
        } else {
            (0..self.state.total_items()).collect()
        }
    }

    pub fn set_data(&mut self, data: Vec<T>, cx: &mut Context<Self>) {
        let _new_count = data.len();
        self.state.replace_in_memory_data(data);
        self.load_more_triggered = false;
        cx.notify();
    }

    pub fn append_data(&mut self, mut new_data: Vec<T>, cx: &mut Context<Self>) {
        if let DataBacking::InMemory { data } = &mut self.state.backing {
            data.append(&mut new_data);
            let new_count = data.len();
            self.state.scroller.set_total_items(new_count);
            self.load_more_triggered = false;
            cx.notify();
        }
    }

    pub fn virtual_reset(&mut self, total_items: usize, page_size: Option<usize>, cx: &mut Context<Self>) {
        self.state.virtual_reset(total_items, page_size);
        self.load_more_triggered = false;
        cx.notify();
    }

    pub fn set_page_data(&mut self, page_start: usize, rows: Vec<T>, cx: &mut Context<Self>) {
        self.state.virtual_set_page(page_start, rows);
        self.load_more_triggered = false;
        cx.notify();
    }

    pub fn data(&self) -> &[T] {
        match &self.state.backing {
            DataBacking::InMemory { data } => data,
            _ => &[],
        }
    }

    pub fn data_count(&self) -> usize {
        self.state.total_items()
    }

    pub fn selected_rows(&self) -> &[usize] {
        &self.state.selected_rows
    }

    pub fn toggle_row_selection(&mut self, row_index: usize, window: &mut Window, cx: &mut Context<Self>) {
        self.state.toggle_row(row_index);

        if let Some(ref callback) = self.on_selection_change {
            callback(&self.state.selected_rows, window, cx);
        }

        cx.notify();
    }

    pub fn select_all(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let total = self.state.total_items();
        self.state.selected_rows = (0..total).collect();

        if let Some(ref callback) = self.on_selection_change {
            callback(&self.state.selected_rows, window, cx);
        }

        cx.notify();
    }

    pub fn clear_selection(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.state.selected_rows.clear();

        if let Some(ref callback) = self.on_selection_change {
            callback(&self.state.selected_rows, window, cx);
        }

        cx.notify();
    }

    fn is_all_selected(&self) -> bool {
        let total = self.state.total_items();
        total > 0 && self.state.selected_rows.len() == total
    }

    fn total_table_width(&self) -> Pixels {
        let mut total: f32 = self.state.column_widths.iter()
            .map(|w| {
                let w_f32: f32 = (*w).into();
                w_f32
            })
            .sum();

        if self.show_selection {
            total += 50.0;
        }

        px(total)
    }

    fn save_edit(&mut self, cx: &mut Context<Self>) {
        if let Some((row_idx, _col_idx)) = self.editing_cell {
            let new_value_string: String = if let Some(ref input) = self.edit_input {
                input.read(cx).content().to_string()
            } else {
                String::new()
            };

            let row_idx_copy = row_idx;
            let column_id = self.edit_column_id.clone();
            let old_value = self.edit_old_value.clone();

            self.editing_cell = None;
            self.edit_input = None;

            if let Some(ref callback) = self.on_cell_edit {
                callback(
                    row_idx_copy,
                    column_id,
                    old_value,
                    new_value_string.into(),
                    cx,
                );
            }

            cx.notify();
        }
    }

    fn render_search_bar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        div()
            .flex()
            .items_center()
            .gap(px(12.0))
            .px(px(16.0))
            .py(px(12.0))
            .border_b_1()
            .border_color(theme.tokens.border)
            .bg(theme.tokens.muted.opacity(0.3))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .child(
                        div()
                            .text_size(px(13.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("Search in:")
                    )
                    .child(
                        div()
                            .w(px(200.0))
                            .child(self.search_column_select.clone())
                    )
            )
            .child(
                div()
                    .w(px(300.0))
                    .child(
                        Input::new(&self.search_input)
                            .size(InputSize::Sm)
                            .placeholder("Type to search...")
                            .on_change({
                                let entity = cx.entity();
                                move |value: SharedString, cx| {
                                    entity.update(cx, |this, cx| {
                                        this.search_query = value.to_string();
                                        cx.notify();
                                    });
                                }
                            })
                    )
            )
    }

    fn render_header(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        let total_width = self.total_table_width();
        let mut header_row = div()
            .flex()
            .w(total_width)
            .min_w(total_width);

        if self.show_selection {
            let all_selected = self.is_all_selected();

            header_row = header_row.child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .w(px(50.0))
                    .px(px(16.0))
                    .py(px(12.0))
                    .text_size(px(13.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(theme.tokens.muted_foreground)
                    .border_b_1()
                    .border_r_1()
                    .border_color(theme.tokens.border)
                    .bg(theme.tokens.muted.opacity(0.5))
                    .cursor(CursorStyle::PointingHand)
                    .hover(|style| style.bg(theme.tokens.muted.opacity(0.7)))
                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, window, cx| {
                        if this.is_all_selected() {
                            this.clear_selection(window, cx);
                        } else {
                            this.select_all(window, cx);
                        }
                    }))
                    .child(
                        div()
                            .w(px(16.0))
                            .h(px(16.0))
                            .rounded(px(3.0))
                            .border_1()
                            .border_color(if all_selected {
                                theme.tokens.primary
                            } else {
                                theme.tokens.border
                            })
                            .bg(if all_selected {
                                theme.tokens.primary
                            } else {
                                theme.tokens.background
                            })
                    )
            );
        }

        let header_cells = self.state.columns.iter().enumerate().map(|(col_idx, column)| {
            let width = self.state.column_widths[col_idx];
            let is_sorted = self.state.sort_column == Some(col_idx);
            let sortable = column.sortable;

            let mut header_cell = div()
                .flex()
                .items_center()
                .justify_between()
                .px(px(16.0))
                .py(px(12.0))
                .w(width)
                .text_size(px(13.0))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(theme.tokens.muted_foreground)
                .border_b_1()
                .border_r_1()
                .border_color(theme.tokens.border)
                .bg(theme.tokens.muted.opacity(0.5))
                .hover(|style| {
                    if sortable {
                        style.bg(theme.tokens.muted.opacity(0.7)).cursor(CursorStyle::PointingHand)
                    } else {
                        style
                    }
                })
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(8.0))
                        .child(column.header.clone())
                        .when(is_sorted, |el| {
                            el.child(
                                div()
                                    .text_size(px(10.0))
                                    .child(match self.state.sort_direction {
                                        SortDirection::Ascending => "▲",
                                        SortDirection::Descending => "▼",
                                    })
                            )
                        })
                );

            if sortable {
                header_cell = header_cell.on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, _window, cx| {
                    let new_direction = if this.state.sort_column == Some(col_idx) {
                        match this.state.sort_direction {
                            SortDirection::Ascending => SortDirection::Descending,
                            SortDirection::Descending => SortDirection::Ascending,
                        }
                    } else {
                        SortDirection::Ascending
                    };

                    this.state.sort_by_column(col_idx, new_direction);
                    cx.notify();
                }));
            }

            header_cell = header_cell.when(column.resizable, |el| {
                el.child(
                    div()
                        .w(px(4.0))
                        .h_full()
                        .absolute()
                        .right(px(0.0))
                        .top(px(0.0))
                        .cursor(CursorStyle::ResizeLeftRight)
                        .bg(gpui::transparent_black())
                        .hover(|style| style.bg(theme.tokens.primary.opacity(0.5)))
                        .on_mouse_down(MouseButton::Left, cx.listener(move |this, event: &MouseDownEvent, _window, cx| {
                            this.resizing_column = Some(col_idx);
                            this.resize_start_x = event.position.x.into();
                            this.resize_start_width = this.state.column_widths[col_idx];
                            cx.notify();
                        }))
                )
            });

            header_cell
        });

        header_row.children(header_cells)
    }
}

impl<T: Clone + 'static> Styled for DataTable<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl<T: Clone + 'static> Render for DataTable<T> {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        let user_style = self.style.clone();

        let viewport_height = self.state.viewport_height();

        let (total_items, filtered_indices): (usize, Option<Rc<Vec<usize>>>) = match &self.state.backing {
            DataBacking::InMemory { .. } => {
                let indices = Rc::new(self.get_filtered_indices());
                (indices.len(), Some(indices))
            }
            DataBacking::Virtual { .. } => (self.state.total_items(), None),
        };
        let row_extent = px(self.state.row_height());
        let total_width = self.total_table_width();

        let view_entity = cx.entity().clone();
        let filtered_indices_for_render = filtered_indices.clone();
        let renderer = move |this: &mut DataTable<T>, range: Range<usize>, _window: &mut Window, cx: &mut Context<DataTable<T>>| {
            let theme = use_theme();
            range
                .map(|row_idx| {
                    let actual_idx = if let Some(ref map) = filtered_indices_for_render {
                        map.get(row_idx).copied().unwrap_or(row_idx)
                    } else { row_idx };

                    if let Some(row_data) = this.state.get_row(actual_idx) {
                        let is_selected = this.state.is_row_selected(actual_idx);

                        let mut row_div = div()
                            .flex()
                            .w(total_width)
                            .min_w(total_width)
                            .h(row_extent)
                            .bg(if is_selected { theme.tokens.accent.opacity(0.2) } else if row_idx % 2 == 0 { theme.tokens.background } else { theme.tokens.muted.opacity(0.3) })
                            .hover(|style| style.bg(theme.tokens.accent.opacity(0.1)));

                        if !this.row_actions.is_empty() {
                            row_div = row_div.on_mouse_down(MouseButton::Right, cx.listener(move |this, event: &MouseDownEvent, _window, cx| {
                                this.context_menu = Some((actual_idx, event.position));
                                cx.notify();
                            }));
                        }

                        if this.on_row_click.is_some() {
                            row_div = row_div.on_mouse_down(MouseButton::Left, cx.listener(move |this, event: &MouseDownEvent, window, cx| {
                                if event.click_count > 1 { return; }
                                if let Some(row) = this.state.get_row(actual_idx) {
                                    if let Some(ref cb) = this.on_row_click {
                                        (cb)(actual_idx, row, window, cx);
                                    }
                                }
                            }));
                        }

                        if this.show_selection {
                            row_div = row_div.child(
                                div()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .w(px(50.0))
                                    .px(px(16.0))
                                    .py(px(12.0))
                                    .border_b_1()
                                    .border_r_1()
                                    .border_color(theme.tokens.border.opacity(0.5))
                                    .cursor(CursorStyle::PointingHand)
                                    .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, window, cx| {
                                        this.toggle_row_selection(actual_idx, window, cx);
                                    }))
                                    .child(
                                        div()
                                            .w(px(16.0))
                                            .h(px(16.0))
                                            .rounded(px(3.0))
                                            .border_1()
                                            .border_color(if is_selected { theme.tokens.primary } else { theme.tokens.border })
                                            .bg(if is_selected { theme.tokens.primary } else { theme.tokens.background })
                                    )
                            );
                        }

                        let cells = this.state.columns.iter().enumerate().map(|(col_idx, column)| {
                            let width = this.state.column_widths[col_idx];
                            let cell_value = (column.accessor)(row_data);
                            let is_editable = column.editable;
                            let is_editing = this.editing_cell == Some((actual_idx, col_idx));

                            let mut cell_div = div()
                                .flex()
                                .items_center()
                                .px(px(16.0))
                                .py(px(12.0))
                                .w(width)
                                .text_size(px(13.0))
                                .text_color(theme.tokens.foreground)
                                .border_b_1()
                                .border_r_1()
                                .border_color(theme.tokens.border.opacity(0.5))
                                .overflow_hidden()
                                .text_ellipsis();

                            if is_editable && !is_editing {
                                let cell_value_for_closure = cell_value.clone();
                                let column_id = column.id.clone();
                                let row_data_clone = row_data.clone();
                                cell_div = cell_div
                                    .cursor(CursorStyle::IBeam)
                                    .on_mouse_down(MouseButton::Left, cx.listener(move |this, event: &MouseDownEvent, window, cx| {
                                        if event.click_count < 2 { return; }

                                        if this.on_cell_double_click.is_some() {
                                            if let Some(ref cb) = this.on_cell_double_click {
                                                (cb)(&row_data_clone, column_id.clone(), cell_value_for_closure.clone(), window, cx);
                                            }
                                            return;
                                        }

                                        let input_state = cx.new(|cx| {
                                            let mut state = InputState::new(cx);
                                            state.set_value(cell_value_for_closure.clone(), window, cx);
                                            state
                                        });
                                        use crate::components::input::InputEvent;
                                        cx.subscribe(&input_state, |this, _, event: &InputEvent, cx| {
                                            match event {
                                                InputEvent::Enter => this.save_edit(cx),
                                                InputEvent::Blur => { if !this.use_edit_dialog { this.save_edit(cx); } }
                                                _ => {}
                                            }
                                        }).detach();
                                        this.editing_cell = Some((actual_idx, col_idx));
                                        this.edit_input = Some(input_state);
                                        this.edit_column_id = column_id.clone();
                                        this.edit_old_value = cell_value_for_closure.clone();
                                        if let Some(ref input) = this.edit_input { window.focus(&input.read(cx).focus_handle(cx)); }
                                        cx.notify();
                                    }));
                            }

                            if is_editing {
                                if let Some(ref input_state) = this.edit_input {
                                    cell_div.child(Input::new(input_state).size(InputSize::Sm))
                                } else {
                                    cell_div.child(cell_value)
                                }
                            } else {
                                cell_div.child(cell_value)
                            }
                        });

                        row_div.children(cells)
                    } else {
                        let mut skeleton_row = div()
                            .flex()
                            .w(total_width)
                            .min_w(total_width)
                            .h(row_extent)
                            .bg(if row_idx % 2 == 0 { theme.tokens.background } else { theme.tokens.muted.opacity(0.3) });
                        if this.show_selection {
                            skeleton_row = skeleton_row.child(
                                div()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .w(px(50.0))
                                    .px(px(16.0))
                                    .py(px(12.0))
                                    .border_b_1()
                                    .border_r_1()
                                    .border_color(theme.tokens.border.opacity(0.5))
                            );
                        }
                        let cells = this.state.columns.iter().enumerate().map(|(col_idx, _)| {
                            let width = this.state.column_widths[col_idx];
                            div()
                                .flex()
                                .items_center()
                                .px(px(16.0))
                                .py(px(12.0))
                                .w(width)
                                .border_b_1()
                                .border_r_1()
                                .border_color(theme.tokens.border.opacity(0.5))
                                .child(div().w(px(96.0)).h(px(12.0)).rounded(px(4.0)).bg(theme.tokens.muted.opacity(0.6)))
                        });
                        skeleton_row.children(cells)
                    }
                })
                .collect::<Vec<_>>()
        };

        let view_for_visible = view_entity.clone();
        let view_for_near_end = view_entity.clone();
        let body_scroll = vlist_uniform_view(view_entity, "data-table-body", total_items, row_extent, renderer)
            .track_scroll(&self.scroll_handle)
            .overscan(8)
            .h(px(viewport_height))
            .on_visible_range(move |range, window, app| {
                let start = range.start;
                let end = range.end;
                let _ = window;
                view_for_visible.update(app, |this: &mut DataTable<T>, cx| {
                    let total_items = match &this.state.backing {
                        DataBacking::InMemory { .. } => this.get_filtered_indices().len(),
                        DataBacking::Virtual { .. } => this.state.total_items(),
                    };
                    if total_items > 0 && !this.load_more_triggered {
                        let progress = end as f32 / total_items as f32;
                        if progress >= this.load_more_threshold {
                            if let Some(ref callback) = this.on_load_more {
                                this.load_more_triggered = true;
                                callback(window, cx);
                            }
                        }
                    }

                    if let DataBacking::Virtual { page_size, in_flight_pages, cache, .. } = &mut this.state.backing {
                        if let Some(ref fetch_cb) = this.on_fetch_page {
                            let first_page_start = (start / *page_size) * *page_size;
                            let last_index = end.saturating_sub(1);
                            let last_page_start = (last_index / *page_size) * *page_size;
                            let mut page = first_page_start;
                            while page <= last_page_start {
                                let mut needs_fetch = false;
                                for i in page..(page + *page_size).min(total_items) {
                                    if !cache.contains_key(&i) { needs_fetch = true; break; }
                                }
                                if needs_fetch && !in_flight_pages.contains(&page) {
                                    in_flight_pages.insert(page);
                                    fetch_cb(page, *page_size, window, cx);
                                }
                                page += *page_size;
                            }
                        }
                    }
                });
            })
            .on_near_end(self.load_more_threshold, move |window, app| {
                view_for_near_end.update(app, |this: &mut DataTable<T>, cx| {
                        if let Some(ref callback) = this.on_load_more {
                            this.load_more_triggered = true;
                            callback(window, cx);
                        }
                    });
            });

        let body_container = div()
            .id("data-table-body-container")
            .h(px(viewport_height))
            .on_scroll_wheel(cx.listener(|view, event: &ScrollWheelEvent, _window, cx| {
                let delta_y: f32 = match &event.delta {
                    ScrollDelta::Lines(delta) => delta.y,
                    ScrollDelta::Pixels(delta) => delta.y.into(),
                };

                let scroll_offset = view.scroll_handle.offset();
                let scroll_y: f32 = (-scroll_offset.y).into();
                let max_scroll = view.state.total_height() - view.state.viewport_height();

                let can_scroll_down = scroll_y < max_scroll && delta_y < 0.0;
                let can_scroll_up = scroll_y > 0.0 && delta_y > 0.0;

                let at_top = scroll_y <= 0.0 && delta_y > 0.0;
                let at_bottom = scroll_y >= max_scroll && delta_y < 0.0;

                if can_scroll_down || can_scroll_up || at_top || at_bottom {
                    cx.stop_propagation();
                }

                cx.notify();
            }))
            .child(body_scroll);

        let scrollable_content = div()
            .id("data-table-content")
            .flex()
            .flex_col()
            .overflow_x_scroll()
            .w_full()
            .cursor(CursorStyle::PointingHand)
            .on_mouse_down(MouseButton::Left, cx.listener(|this, event: &MouseDownEvent, _window, cx| {
                this.is_dragging_horizontal = true;
                this.drag_start_x = event.position.x.into();
                this.drag_scroll_start_x = 0.0;
                cx.notify();
            }))
            .on_mouse_move(cx.listener(|this, event: &MouseMoveEvent, _window, cx| {
                if this.is_dragging_horizontal {
                    let current_x: f32 = event.position.x.into();
                    let delta_x = this.drag_start_x - current_x;

                    let _new_scroll_x = this.drag_scroll_start_x + delta_x;

                    cx.notify();
                }
            }))
            .on_mouse_up(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                if this.is_dragging_horizontal {
                    this.is_dragging_horizontal = false;
                    cx.notify();
                }
            }))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .w(total_width)
                    .min_w(total_width)
                    .child(self.render_header(cx))
                    .child(body_container)
            );

        let table_div = if self.sticky_header {
            div()
                .flex()
                .flex_col()
                .w_full()
                .border_1()
                .border_color(theme.tokens.border)
                .rounded(theme.tokens.radius_lg)
                .overflow_hidden()
                .bg(theme.tokens.card)
                .shadow_sm()
                .when(self.show_search, |div| {
                    div.child(self.render_search_bar(cx))
                })
                .child(scrollable_content)
                .map(|mut this| {
                    this.style().refine(&user_style);
                    this
                })
        } else {
            div()
                .flex()
                .flex_col()
                .w_full()
                .border_1()
                .border_color(theme.tokens.border)
                .rounded(theme.tokens.radius_lg)
                .overflow_hidden()
                .bg(theme.tokens.card)
                .shadow_sm()
                .when(self.show_search, |div| {
                    div.child(self.render_search_bar(cx))
                })
                .child(scrollable_content)
                .map(|mut this| {
                    this.style().refine(&user_style);
                    this
                })
        };

        let context_menu_elem = self.context_menu.map(|(row_idx, position)| {
            self.render_context_menu(row_idx, position, cx)
        });

        div()
            .relative()
            .w_full()
            .on_mouse_move(cx.listener(|this, event: &MouseMoveEvent, _window, cx| {
                if let Some(col_idx) = this.resizing_column {
                    let current_x: f32 = event.position.x.into();
                    let delta_x = current_x - this.resize_start_x;
                    let new_width_f32: f32 = this.resize_start_width.into();
                    let new_width = px(new_width_f32 + delta_x);

                    let min_width = this.state.columns[col_idx].min_width;
                    let final_width = if new_width > min_width {
                        new_width
                    } else {
                        min_width
                    };

                    this.state.resize_column(col_idx, final_width);
                    cx.notify();
                }
            }))
            .on_mouse_up(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                if this.resizing_column.is_some() {
                    this.resizing_column = None;
                    cx.notify();
                }
            }))
            .child(table_div)
            .children(context_menu_elem)
    }

}

impl<T: Clone + 'static> DataTable<T> {
    fn render_context_menu(&self, row_idx: usize, position: Point<Pixels>, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        deferred(
            anchored()
                .position(position)
                .snap_to_window_with_margin(px(8.))
                .anchor(Corner::TopLeft)
                .child(
                    div()
                        .occlude()
                        .min_w(px(200.0))
                        .bg(theme.tokens.popover)
                        .border_1()
                        .border_color(theme.tokens.border)
                        .rounded(theme.tokens.radius_md)
                        .shadow_xl()
                        .p(px(4.0))
                        .on_mouse_down_out(cx.listener(|this, _, _, cx| {
                            this.context_menu = None;
                            cx.notify();
                        }))
                        .children(self.row_actions.iter().map(|action| {
                            let action = action.clone();
                            div()
                                .flex()
                                .items_center()
                                .gap(px(8.0))
                                .px(px(12.0))
                                .py(px(8.0))
                                .rounded(theme.tokens.radius_sm)
                                .cursor(CursorStyle::PointingHand)
                                .hover(|style| style.bg(theme.tokens.accent))
                                .text_size(px(14.0))
                                .text_color(if action.destructive {
                                    theme.tokens.destructive
                                } else {
                                    theme.tokens.popover_foreground
                                })
                                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, window, cx| {
                                    (action.on_click)(row_idx, window, cx);
                                    this.context_menu = None;
                                    cx.notify();
                                }))
                                .when_some(action.icon, |div, icon| {
                                    div.child(
                                        crate::components::icon::Icon::new(icon)
                                            .size(px(16.0))
                                            .color(if action.destructive {
                                                theme.tokens.destructive
                                            } else {
                                                theme.tokens.popover_foreground
                                            })
                                    )
                                })
                                .child(action.label)
                                .into_any_element()
                        }))
                )
        )
    }
}
