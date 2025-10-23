//! Combobox component - Searchable dropdown combining Input and Select functionality.
//!
//! Features:
//! - Searchable input with live filtering
//! - Keyboard navigation (Up/Down arrows, Enter, Escape)
//! - Custom render functions for items
//! - Multiple selection mode
//! - Virtual scrolling for large lists
//! - Clear button to reset selection
//! - Full Styled trait support for customization

use gpui::{prelude::*, *};
use crate::theme::use_theme;
use crate::components::scrollable::scrollable_vertical;
use crate::components::icon::Icon;

actions!(combobox, [
    ComboboxUp,
    ComboboxDown,
    ComboboxConfirm,
    ComboboxCancel,
    ComboboxClear
]);

const DROPDOWN_MARGIN: Pixels = px(4.0);

/// Events emitted by the Combobox
#[derive(Clone, Debug)]
pub enum ComboboxEvent {
    Change,
    Search,
}

/// Combobox state for managing selections and search
pub struct ComboboxState<T: Clone + 'static> {
    pub selected: Vec<T>,
    pub search_text: String,
    pub is_open: bool,
    pub focused_index: Option<usize>,
}

impl<T: Clone + 'static> ComboboxState<T> {
    pub fn new() -> Self {
        Self {
            selected: Vec::new(),
            search_text: String::new(),
            is_open: false,
            focused_index: None,
        }
    }

    pub fn with_selected(mut self, item: T) -> Self {
        self.selected.push(item);
        self
    }

    pub fn toggle_open(&mut self) {
        self.is_open = !self.is_open;
        if !self.is_open {
            self.search_text.clear();
            self.focused_index = None;
        }
    }

    pub fn close(&mut self) {
        self.is_open = false;
        self.search_text.clear();
        self.focused_index = None;
    }

    pub fn select_item(&mut self, item: T, multi_select: bool) {
        if multi_select {
            self.selected.push(item);
        } else {
            self.selected = vec![item];
            self.is_open = false;
        }
    }

    pub fn clear(&mut self) {
        self.selected.clear();
        self.search_text.clear();
    }

    pub fn is_selected(&self, item: &T) -> bool
    where
        T: PartialEq,
    {
        self.selected.iter().any(|s| s == item)
    }
}

impl<T: Clone + 'static> Default for ComboboxState<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Combobox component with searchable dropdown
pub struct Combobox<T: Clone + 'static> {
    focus_handle: FocusHandle,
    items: Vec<T>,
    state: Entity<ComboboxState<T>>,
    placeholder: SharedString,
    disabled: bool,
    clearable: bool,
    multi_select: bool,
    max_height: Pixels,

    filter_fn: Box<dyn Fn(&T, &str) -> bool>,
    render_item: Box<dyn Fn(&T) -> SharedString>,
    render_selected: Option<Box<dyn Fn(&[T]) -> SharedString>>,

    on_select: Option<Box<dyn Fn(&T, &mut Window, &mut App)>>,
    on_search: Option<Box<dyn Fn(&str, &mut App)>>,

    bounds: Bounds<Pixels>,

    style: StyleRefinement,
}

impl<T: Clone + 'static> Combobox<T> {
    /// Create a new combobox with items and state entity
    pub fn new(items: Vec<T>, state: &Entity<ComboboxState<T>>, cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            items,
            state: state.clone(),
            placeholder: "Search...".into(),
            disabled: false,
            clearable: true,
            multi_select: false,
            max_height: px(300.0),
            filter_fn: Box::new(|_, _| true),
            render_item: Box::new(|_| "Item".into()),
            render_selected: None,
            on_select: None,
            on_search: None,
            bounds: Bounds::default(),
            style: StyleRefinement::default(),
        }
    }

    /// Set the placeholder text
    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Enable/disable clear button
    pub fn clearable(mut self, clearable: bool) -> Self {
        self.clearable = clearable;
        self
    }

    /// Enable multiple selection mode
    pub fn multi_select(mut self, multi_select: bool) -> Self {
        self.multi_select = multi_select;
        self
    }

    /// Set maximum dropdown height
    pub fn max_height(mut self, height: impl Into<Pixels>) -> Self {
        self.max_height = height.into();
        self
    }

    /// Set custom filter function
    ///
    /// # Example
    /// ```rust
    /// combobox.filter_fn(|item, search| {
    ///     item.name.to_lowercase().contains(&search.to_lowercase())
    /// })
    /// ```
    pub fn filter_fn<F>(mut self, f: F) -> Self
    where
        F: Fn(&T, &str) -> bool + 'static,
    {
        self.filter_fn = Box::new(f);
        self
    }

    /// Set custom item render function
    ///
    /// # Example
    /// ```rust
    /// combobox.render_item(|item| item.name.clone().into())
    /// ```
    pub fn render_item<F>(mut self, f: F) -> Self
    where
        F: Fn(&T) -> SharedString + 'static,
    {
        self.render_item = Box::new(f);
        self
    }

    /// Set custom selected items render function
    pub fn render_selected<F>(mut self, f: F) -> Self
    where
        F: Fn(&[T]) -> SharedString + 'static,
    {
        self.render_selected = Some(Box::new(f));
        self
    }

    /// Set callback when item is selected
    pub fn on_select<F>(mut self, callback: F) -> Self
    where
        F: Fn(&T, &mut Window, &mut App) + 'static,
    {
        self.on_select = Some(Box::new(callback));
        self
    }

    /// Set callback when search text changes
    pub fn on_search<F>(mut self, callback: F) -> Self
    where
        F: Fn(&str, &mut App) + 'static,
    {
        self.on_search = Some(Box::new(callback));
        self
    }

    /// Get filtered items based on current search text
    fn filtered_items(&self, cx: &App) -> Vec<(usize, &T)> {
        let state = self.state.read(cx);
        let search = state.search_text.to_lowercase();

        self.items
            .iter()
            .enumerate()
            .filter(|(_, item)| (self.filter_fn)(item, &search))
            .collect()
    }

    /// Toggle dropdown open/close
    fn toggle_dropdown(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.disabled {
            self.state.update(cx, |state, cx| {
                state.toggle_open();
                if state.is_open {
                    window.focus(&self.focus_handle);
                    state.focused_index = Some(0);
                }
                cx.notify();  // Trigger re-render
            });
        }
    }

    /// Close the dropdown
    fn close_dropdown(&mut self, cx: &mut Context<Self>) {
        self.state.update(cx, |state, cx| {
            state.close();
            cx.notify();  // Trigger re-render
        });
    }

    /// Clear all selections
    fn clear_selection(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.state.update(cx, |state, cx| {
            state.clear();
            cx.notify();  // Trigger re-render
        });
        cx.emit(ComboboxEvent::Change);
    }

    /// Select an item
    fn select_item(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        let filtered = self.filtered_items(cx);
        if let Some(&(original_idx, _)) = filtered.get(index) {
            if let Some(item) = self.items.get(original_idx).cloned() {
                self.state.update(cx, |state, cx| {
                    state.select_item(item.clone(), self.multi_select);
                    cx.notify();  // Trigger re-render
                });

                cx.emit(ComboboxEvent::Change);

                if let Some(ref callback) = self.on_select {
                    (callback)(&item, window, cx);
                }
            }
        }
    }

    /// Handle up arrow key
    fn combobox_up(&mut self, _: &ComboboxUp, _: &mut Window, cx: &mut Context<Self>) {
        let filtered_count = self.filtered_items(cx).len();
        if filtered_count == 0 {
            return;
        }

        self.state.update(cx, |state, _| {
            if !state.is_open {
                state.is_open = true;
                state.focused_index = Some(0);
            } else {
                let current = state.focused_index.unwrap_or(0);
                state.focused_index = Some(if current == 0 {
                    filtered_count - 1
                } else {
                    current - 1
                });
            }
        });
    }

    /// Handle down arrow key
    fn combobox_down(&mut self, _: &ComboboxDown, _: &mut Window, cx: &mut Context<Self>) {
        let filtered_count = self.filtered_items(cx).len();
        if filtered_count == 0 {
            return;
        }

        self.state.update(cx, |state, _| {
            if !state.is_open {
                state.is_open = true;
                state.focused_index = Some(0);
            } else {
                let current = state.focused_index.unwrap_or(0);
                state.focused_index = Some(if current >= filtered_count - 1 {
                    0
                } else {
                    current + 1
                });
            }
        });
    }

    /// Handle enter key (confirm selection)
    fn combobox_confirm(&mut self, _: &ComboboxConfirm, window: &mut Window, cx: &mut Context<Self>) {
        let state = self.state.read(cx);
        if state.is_open {
            if let Some(idx) = state.focused_index {
                self.select_item(idx, window, cx);
            }
        } else {
            self.toggle_dropdown(window, cx);
        }
    }

    /// Handle escape key (cancel/close)
    fn combobox_cancel(&mut self, _: &ComboboxCancel, _: &mut Window, cx: &mut Context<Self>) {
        if self.state.read(cx).is_open {
            self.close_dropdown(cx);
        }
    }

    /// Handle clear action
    fn combobox_clear(&mut self, _: &ComboboxClear, window: &mut Window, cx: &mut Context<Self>) {
        self.clear_selection(window, cx);
    }
}

impl<T: Clone + 'static> Styled for Combobox<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl<T: Clone + PartialEq + 'static> Render for Combobox<T> {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();
        let user_style = self.style.clone();

        let state = self.state.read(cx);
        let is_open = state.is_open;
        let search_text = state.search_text.clone();
        let focused_idx = state.focused_index;
        let has_selection = !state.selected.is_empty();

        let display_text: SharedString = if !search_text.is_empty() {
            search_text.clone().into()
        } else if !state.selected.is_empty() {
            if let Some(ref render_fn) = self.render_selected {
                render_fn(&state.selected)
            } else if self.multi_select {
                format!("{} selected", state.selected.len()).into()
            } else {
                (self.render_item)(&state.selected[0])
            }
        } else {
            self.placeholder.clone()
        };

        let bounds = self.bounds;
        let is_searching = !search_text.is_empty();

        let trigger = div()
            .id("combobox-trigger")
            .relative()
            .flex()
            .items_center()
            .justify_between()
            .h(px(40.0))
            .px(px(12.0))
            .bg(theme.tokens.background)
            .border_1()
            .border_color(if is_open { theme.tokens.ring } else { theme.tokens.input })
            .rounded(theme.tokens.radius_md)
            .text_color(if has_selection && !is_searching {
                theme.tokens.foreground
            } else {
                theme.tokens.muted_foreground
            })
            .text_size(px(14.0))
            .font_family(theme.tokens.font_family.clone())
            .cursor(if self.disabled { CursorStyle::Arrow } else { CursorStyle::PointingHand })
            .when(!self.disabled, |div: Stateful<Div>| {
                div.hover(|mut style| {
                    style.border_color = Some(theme.tokens.ring.into());
                    style
                })
            })
            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, window, cx| {
                if !this.disabled {
                    window.focus(&this.focus_handle);
                    this.state.update(cx, |state, _| {
                        if !state.is_open {
                            state.is_open = true;
                            state.focused_index = Some(0);
                        }
                    });
                }
            }))
            .child(
                div()
                    .flex_1()
                    .child(display_text)
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(4.0))
                    .when(self.clearable && has_selection && !self.disabled, |this_div| {
                        this_div.child(
                            div()
                                .w(px(20.0))
                                .h(px(20.0))
                                .flex()
                                .items_center()
                                .justify_center()
                                .rounded(px(10.0))
                                .cursor(CursorStyle::PointingHand)
                                .hover(|mut style| {
                                    style.background = Some(theme.tokens.muted.into());
                                    style
                                })
                                .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, window, cx| {
                                    this.clear_selection(window, cx);
                                }))
                                .child(
                                    Icon::new("x")
                                        .size(px(14.0))
                                        .color(theme.tokens.muted_foreground)
                                )
                        )
                    })
                    .child(
                        Icon::new(if is_open { "chevron-up" } else { "chevron-down" })
                            .size(px(14.0))
                            .color(theme.tokens.muted_foreground)
                    )
            )
            .child({
                let entity = cx.entity().clone();
                canvas(
                    move |bounds, _, cx| {
                        entity.update(cx, |this, _| {
                            this.bounds = bounds;
                        })
                    },
                    |_, _, _, _| {},
                )
                .absolute()
                .size_full()
            });

        div()
            .relative()
            .w_full()
            .key_context("Combobox")
            .track_focus(&self.focus_handle)
            .when(is_open && !self.disabled, |this: Div| {
                this.on_action(cx.listener(Combobox::combobox_up))
                    .on_action(cx.listener(Combobox::combobox_down))
                    .on_action(cx.listener(Combobox::combobox_confirm))
                    .on_action(cx.listener(Combobox::combobox_cancel))
                    .on_action(cx.listener(Combobox::combobox_clear))
            })
            .when(is_open, |this: Div| {
                this.on_key_down(cx.listener(|this, event: &KeyDownEvent, _, cx| {
                    if event.keystroke.key == "backspace" {
                        this.state.update(cx, |state, cx| {
                            state.search_text.pop();
                            cx.notify();  // Trigger re-render
                        });
                        cx.emit(ComboboxEvent::Search);
                    } else if event.keystroke.key.len() == 1
                        && !event.keystroke.modifiers.control
                        && !event.keystroke.modifiers.platform
                        && !event.keystroke.modifiers.alt {
                        this.state.update(cx, |state, cx| {
                            state.search_text.push_str(&event.keystroke.key);
                            state.focused_index = Some(0);
                            cx.notify();  // Trigger re-render
                        });
                        cx.emit(ComboboxEvent::Search);

                        if let Some(ref callback) = this.on_search {
                            let search = this.state.read(cx).search_text.clone();
                            callback(&search, cx);
                        }
                    }
                }))
            })
            .on_mouse_down_out(cx.listener(|this, _, _, cx| {
                if this.state.read(cx).is_open {
                    this.close_dropdown(cx);
                }
            }))
            .child(trigger)
            .when(is_open, |this| {
                this.child(
                    deferred(
                        anchored()
                            .snap_to_window_with_margin(Edges::all(DROPDOWN_MARGIN))
                            .child(
                                div()
                                    .occlude()
                                    .w(bounds.size.width)
                                    .child(
                                        div()
                                            .occlude()
                                            .mt(DROPDOWN_MARGIN)
                                            .bg(theme.tokens.popover)
                                            .border_2()
                                            .border_color(theme.tokens.ring)
                                            .rounded(theme.tokens.radius_md)
                                            .shadow_xl()
                                            .overflow_hidden()
                                            .child(
                                                div()
                                                    .max_h(self.max_height)
                                                    .child({
                                                        let filtered = self.filtered_items(cx);

                                                        scrollable_vertical(
                                                            div()
                                                                .py(px(4.0))
                                                                .when(filtered.is_empty(), |this| {
                                                                    this.child(
                                                                        div()
                                                                            .px(px(12.0))
                                                                            .py(px(16.0))
                                                                            .flex()
                                                                            .items_center()
                                                                            .justify_center()
                                                                            .text_size(px(13.0))
                                                                            .font_family(theme.tokens.font_family.clone())
                                                                            .text_color(theme.tokens.muted_foreground)
                                                                            .child("No results found")
                                                                    )
                                                                })
                                                                .when(!filtered.is_empty(), |this| {
                                                                    this.children(
                                                                        filtered.iter().enumerate().map(|(display_idx, &(_original_idx, item))| {
                                                                            let is_focused = focused_idx == Some(display_idx);
                                                                            let is_selected = state.is_selected(item);
                                                                            let item_text = (self.render_item)(item);

                                                                            div()
                                                                                .px(px(12.0))
                                                                                .py(px(8.0))
                                                                                .flex()
                                                                                .items_center()
                                                                                .justify_between()
                                                                                .text_color(if is_selected {
                                                                                    theme.tokens.primary
                                                                                } else {
                                                                                    theme.tokens.popover_foreground
                                                                                })
                                                                                .bg(if is_focused {
                                                                                    theme.tokens.accent
                                                                                } else {
                                                                                    gpui::transparent_black()
                                                                                })
                                                                                .hover(|mut style| {
                                                                                    style.background = Some(theme.tokens.accent.into());
                                                                                    style
                                                                                })
                                                                                .cursor(CursorStyle::PointingHand)
                                                                                .text_size(px(14.0))
                                                                                .font_family(theme.tokens.font_family.clone())
                                                                                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, window, cx| {
                                                                                    this.select_item(display_idx, window, cx);
                                                                                }))
                                                                                .child(item_text)
                                                                                .when(is_selected, |div| {
                                                                                    div.child(
                                                                                        Icon::new("check")
                                                                                            .size(px(14.0))
                                                                                            .color(theme.tokens.primary)
                                                                                    )
                                                                                })
                                                                        })
                                                                    )
                                                                })
                                                        )
                                                    })
                                            )
                                    )
                            )
                    )
                    .with_priority(1)
                )
            })
            .map(|this| {
                let mut div = this;
                div.style().refine(&user_style);
                div
            })
    }
}

/// Initialize combobox key bindings
pub fn init_combobox(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("up", ComboboxUp, Some("Combobox")),
        KeyBinding::new("down", ComboboxDown, Some("Combobox")),
        KeyBinding::new("enter", ComboboxConfirm, Some("Combobox")),
        KeyBinding::new("escape", ComboboxCancel, Some("Combobox")),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-k", ComboboxClear, Some("Combobox")),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-k", ComboboxClear, Some("Combobox")),
    ]);
}

impl<T: Clone + 'static> Focusable for Combobox<T> {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl<T: Clone + 'static> EventEmitter<ComboboxEvent> for Combobox<T> {}
