use super::state::CompletionState;
use super::SymbolKind;
use crate::ide_theme::use_ide_theme;
use adabraka_ui::components::editor::EditorState;
use adabraka_ui::components::icon::Icon;
use adabraka_ui::components::scrollable::scrollable_vertical;
use gpui::prelude::FluentBuilder as _;
use gpui::*;
use std::rc::Rc;

const MAX_VISIBLE_ITEMS: usize = 8;
const ITEM_HEIGHT: f32 = 28.0;
const MENU_WIDTH: f32 = 280.0;

pub struct CompletionMenu {
    state: Entity<CompletionState>,
    editor_state: Option<Entity<EditorState>>,
    on_accept: Option<Rc<dyn Fn(&mut Window, &mut App)>>,
}

impl CompletionMenu {
    pub fn new(state: Entity<CompletionState>) -> Self {
        Self {
            state,
            editor_state: None,
            on_accept: None,
        }
    }

    pub fn editor_state(mut self, editor: Entity<EditorState>) -> Self {
        self.editor_state = Some(editor);
        self
    }

    pub fn on_accept(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_accept = Some(Rc::new(handler));
        self
    }
}

impl IntoElement for CompletionMenu {
    type Element = CompletionMenuElement;

    fn into_element(self) -> Self::Element {
        CompletionMenuElement {
            state: self.state,
            editor_state: self.editor_state,
            on_accept: self.on_accept,
        }
    }
}

pub struct CompletionMenuElement {
    state: Entity<CompletionState>,
    editor_state: Option<Entity<EditorState>>,
    on_accept: Option<Rc<dyn Fn(&mut Window, &mut App)>>,
}

impl IntoElement for CompletionMenuElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

pub struct CompletionMenuPrepaintState {
    menu_element: Option<AnyElement>,
}

impl Element for CompletionMenuElement {
    type RequestLayoutState = Option<AnyElement>;
    type PrepaintState = CompletionMenuPrepaintState;

    fn id(&self) -> Option<ElementId> {
        Some("completion-menu".into())
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let state_entity = self.state.clone();
        let state = self.state.read(cx);

        if !state.is_visible() {
            let style = Style::default();
            let layout_id = window.request_layout(style, [], cx);
            return (layout_id, None);
        }

        let chrome = use_ide_theme().chrome;
        let anchor = self
            .editor_state
            .as_ref()
            .and_then(|es| es.read(cx).cursor_screen_position(px(20.0)))
            .unwrap_or_else(|| state.anchor_position());
        let selected_idx = state.selected_display_index();
        let on_accept = self.on_accept.clone();

        let items: Vec<_> = state
            .filtered_items()
            .take(50)
            .map(|(display_idx, item)| {
                let is_selected = display_idx == selected_idx;
                let label = item.label.clone();
                let kind = item.kind;
                let detail = item.detail.clone();
                let state_for_click = state_entity.clone();
                let on_accept_click = on_accept.clone();

                let right_label = detail
                    .filter(|d| !d.is_empty())
                    .unwrap_or_else(|| kind.label().to_string());

                div()
                    .id(SharedString::from(format!("completion-{}", display_idx)))
                    .w_full()
                    .h(px(ITEM_HEIGHT))
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .px(px(8.0))
                    .cursor_pointer()
                    .when(is_selected, |el| el.bg(chrome.accent))
                    .when(!is_selected, |el| {
                        el.hover(|s| s.bg(chrome.accent.opacity(0.5)))
                    })
                    .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                        state_for_click.update(cx, |s, cx| {
                            for _ in 0..display_idx {
                                s.move_down(cx);
                            }
                        });
                        if let Some(ref handler) = on_accept_click {
                            handler(window, cx);
                        }
                    })
                    .child(render_kind_icon(kind, &chrome))
                    .child(
                        div()
                            .flex_1()
                            .text_size(px(13.0))
                            .text_color(chrome.bright)
                            .overflow_x_hidden()
                            .text_ellipsis()
                            .child(label),
                    )
                    .child(
                        div()
                            .text_size(px(10.0))
                            .text_color(chrome.text_secondary.opacity(0.7))
                            .max_w(px(100.0))
                            .overflow_x_hidden()
                            .text_ellipsis()
                            .child(right_label),
                    )
            })
            .collect();

        let item_count = items.len();
        let menu_height = (item_count.min(MAX_VISIBLE_ITEMS) as f32 * ITEM_HEIGHT) + 8.0;

        let state_for_keys = state_entity.clone();
        let on_accept_key = on_accept.clone();

        let mut menu = deferred(
            anchored()
                .position(anchor)
                .snap_to_window_with_margin(px(8.0))
                .child(
                    div()
                        .id("completion-menu-inner")
                        .key_context("CompletionMenu")
                        .occlude()
                        .mt(px(4.0))
                        .w(px(MENU_WIDTH))
                        .max_h(px(menu_height))
                        .bg(chrome.panel_bg)
                        .border_1()
                        .border_color(chrome.header_border)
                        .rounded(px(8.0))
                        .shadow_lg()
                        .overflow_hidden()
                        .on_key_down({
                            let state = state_for_keys.clone();
                            let on_accept = on_accept_key.clone();
                            move |event: &KeyDownEvent, window, cx| match event
                                .keystroke
                                .key
                                .as_str()
                            {
                                "up" => {
                                    state.update(cx, |s, cx| s.move_up(cx));
                                    cx.stop_propagation();
                                }
                                "down" => {
                                    state.update(cx, |s, cx| s.move_down(cx));
                                    cx.stop_propagation();
                                }
                                "tab" | "enter" => {
                                    if let Some(ref handler) = on_accept {
                                        handler(window, cx);
                                    }
                                    cx.stop_propagation();
                                }
                                "escape" => {
                                    state.update(cx, |s, cx| s.dismiss(cx));
                                    cx.stop_propagation();
                                }
                                _ => {}
                            }
                        })
                        .child(scrollable_vertical(div().py(px(4.0)).children(items))),
                ),
        )
        .with_priority(2)
        .into_any();

        let layout_id = menu.request_layout(window, cx);
        (layout_id, Some(menu))
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        _bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        if let Some(menu) = request_layout.as_mut() {
            menu.prepaint(window, cx);
        }
        CompletionMenuPrepaintState {
            menu_element: request_layout.take(),
        }
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        if let Some(menu) = prepaint.menu_element.as_mut() {
            menu.paint(window, cx);
        }
    }
}

fn render_kind_icon(kind: SymbolKind, chrome: &crate::ide_theme::ChromeColors) -> impl IntoElement {
    let icon_color = match kind {
        SymbolKind::Function | SymbolKind::Method => chrome.accent,
        SymbolKind::Variable | SymbolKind::Field => chrome.bright,
        SymbolKind::Struct | SymbolKind::Class => chrome.bright,
        SymbolKind::Enum => chrome.diff_del_text,
        SymbolKind::Const => chrome.accent,
        SymbolKind::Type => chrome.accent,
        SymbolKind::Module => chrome.text_secondary,
    };

    div()
        .w(px(16.0))
        .h(px(16.0))
        .flex()
        .items_center()
        .justify_center()
        .child(Icon::new(kind.icon_name()).size(px(14.0)).color(icon_color))
}
