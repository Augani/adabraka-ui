use super::state::CompletionState;
use super::SymbolKind;
use adabraka_ui::components::icon::Icon;
use adabraka_ui::components::scrollable::scrollable_vertical;
use adabraka_ui::theme::use_theme;
use gpui::prelude::FluentBuilder as _;
use gpui::*;
use std::rc::Rc;

const MAX_VISIBLE_ITEMS: usize = 8;
const ITEM_HEIGHT: f32 = 28.0;
const MENU_WIDTH: f32 = 280.0;

pub struct CompletionMenu {
    state: Entity<CompletionState>,
    on_accept: Option<Rc<dyn Fn(&mut Window, &mut App)>>,
}

impl CompletionMenu {
    pub fn new(state: Entity<CompletionState>) -> Self {
        Self {
            state,
            on_accept: None,
        }
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
            on_accept: self.on_accept,
        }
    }
}

pub struct CompletionMenuElement {
    state: Entity<CompletionState>,
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

        let theme = use_theme();
        let anchor = state.anchor_position();
        let selected_idx = state.selected_display_index();
        let on_accept = self.on_accept.clone();

        let items: Vec<_> = state
            .filtered_items()
            .take(50)
            .map(|(display_idx, item)| {
                let is_selected = display_idx == selected_idx;
                let label = item.label.clone();
                let kind = item.kind;
                let state_for_click = state_entity.clone();
                let on_accept_click = on_accept.clone();

                div()
                    .id(SharedString::from(format!("completion-{}", display_idx)))
                    .w_full()
                    .h(px(ITEM_HEIGHT))
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .px(px(8.0))
                    .cursor_pointer()
                    .when(is_selected, |el| el.bg(theme.tokens.accent))
                    .when(!is_selected, |el| {
                        el.hover(|s| s.bg(theme.tokens.accent.opacity(0.5)))
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
                    .child(render_kind_icon(kind, &theme))
                    .child(
                        div()
                            .flex_1()
                            .text_size(px(13.0))
                            .text_color(if is_selected {
                                theme.tokens.accent_foreground
                            } else {
                                theme.tokens.popover_foreground
                            })
                            .overflow_x_hidden()
                            .text_ellipsis()
                            .child(label),
                    )
                    .child(
                        div()
                            .text_size(px(10.0))
                            .text_color(theme.tokens.muted_foreground.opacity(0.7))
                            .child(kind.label()),
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
                        .bg(theme.tokens.popover)
                        .border_1()
                        .border_color(theme.tokens.border)
                        .rounded(theme.tokens.radius_md)
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

fn render_kind_icon(kind: SymbolKind, theme: &adabraka_ui::theme::Theme) -> impl IntoElement {
    let icon_color = match kind {
        SymbolKind::Function | SymbolKind::Method => theme.tokens.primary,
        SymbolKind::Variable | SymbolKind::Field => theme.tokens.secondary_foreground,
        SymbolKind::Struct | SymbolKind::Class => theme.tokens.accent_foreground,
        SymbolKind::Enum => theme.tokens.destructive,
        SymbolKind::Const => theme.tokens.ring,
        SymbolKind::Type => theme.tokens.primary,
        SymbolKind::Module => theme.tokens.muted_foreground,
    };

    div()
        .w(px(16.0))
        .h(px(16.0))
        .flex()
        .items_center()
        .justify_center()
        .child(Icon::new(kind.icon_name()).size(px(14.0)).color(icon_color))
}
