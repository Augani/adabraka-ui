use crate::components::icon::Icon;
use crate::components::icon_source::IconSource;
use crate::theme::use_theme;
use gpui::{prelude::FluentBuilder as _, *};
use std::rc::Rc;

#[derive(Clone)]
pub struct DropdownItem {
    id: SharedString,
    label: SharedString,
    icon: Option<IconSource>,
    disabled: bool,
    destructive: bool,
    on_click: Option<Rc<dyn Fn(&mut Window, &mut App)>>,
}

impl DropdownItem {
    pub fn new(id: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon: None,
            disabled: false,
            destructive: false,
            on_click: None,
        }
    }

    pub fn separator() -> Self {
        Self {
            id: "__separator__".into(),
            label: "".into(),
            icon: None,
            disabled: true,
            destructive: false,
            on_click: None,
        }
    }

    pub fn icon(mut self, icon: impl Into<IconSource>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn destructive(mut self, destructive: bool) -> Self {
        self.destructive = destructive;
        self
    }

    pub fn on_click(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }

    fn is_separator(&self) -> bool {
        self.id.as_ref() == "__separator__"
    }
}

pub struct DropdownState {
    open: bool,
    focus_handle: FocusHandle,
}

impl DropdownState {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            open: false,
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn is_open(&self) -> bool {
        self.open
    }

    pub fn toggle(&mut self, cx: &mut Context<Self>) {
        self.open = !self.open;
        cx.notify();
    }

    pub fn open(&mut self, cx: &mut Context<Self>) {
        self.open = true;
        cx.notify();
    }

    pub fn close(&mut self, cx: &mut Context<Self>) {
        self.open = false;
        cx.notify();
    }
}

impl Focusable for DropdownState {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for DropdownState {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DropdownAlign {
    Start,
    End,
}

#[derive(IntoElement)]
pub struct Dropdown {
    state: Entity<DropdownState>,
    trigger: AnyElement,
    items: Vec<DropdownItem>,
    align: DropdownAlign,
    min_width: Option<Pixels>,
    style: StyleRefinement,
}

impl Dropdown {
    pub fn new(state: Entity<DropdownState>, trigger: impl IntoElement) -> Self {
        Self {
            state,
            trigger: trigger.into_any_element(),
            items: Vec::new(),
            align: DropdownAlign::Start,
            min_width: Some(px(180.0)),
            style: StyleRefinement::default(),
        }
    }

    pub fn items(mut self, items: Vec<DropdownItem>) -> Self {
        self.items = items;
        self
    }

    pub fn align(mut self, align: DropdownAlign) -> Self {
        self.align = align;
        self
    }

    pub fn min_width(mut self, width: Pixels) -> Self {
        self.min_width = Some(width);
        self
    }
}

impl Styled for Dropdown {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Dropdown {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let user_style = self.style;
        let is_open = self.state.read(cx).open;
        let state = self.state.clone();

        div()
            .relative()
            .child(
                div()
                    .id("dropdown-trigger")
                    .cursor_pointer()
                    .on_click({
                        let state = state.clone();
                        move |_, _, cx| {
                            state.update(cx, |s, cx| s.toggle(cx));
                        }
                    })
                    .child(self.trigger),
            )
            .when(is_open, |this| {
                this.child(
                    div()
                        .absolute()
                        .top_full()
                        .mt(px(4.0))
                        .when(self.align == DropdownAlign::Start, |d| d.left_0())
                        .when(self.align == DropdownAlign::End, |d| d.right_0())
                        .when_some(self.min_width, |d, w| d.min_w(w))
                        .bg(theme.tokens.popover)
                        .border_1()
                        .border_color(theme.tokens.border)
                        .rounded(theme.tokens.radius_md)
                        .shadow(smallvec::smallvec![BoxShadow {
                            color: hsla(0.0, 0.0, 0.0, 0.15),
                            offset: point(px(0.0), px(4.0)),
                            blur_radius: px(12.0),
                            spread_radius: px(0.0),
                            inset: false,
                        }])
                        .py(px(4.0))
                        .children(self.items.iter().map(|item| {
                            if item.is_separator() {
                                return div()
                                    .h(px(1.0))
                                    .mx(px(8.0))
                                    .my(px(4.0))
                                    .bg(theme.tokens.border)
                                    .into_any_element();
                            }

                            let text_color = if item.disabled {
                                theme.tokens.muted_foreground
                            } else if item.destructive {
                                theme.tokens.destructive
                            } else {
                                theme.tokens.foreground
                            };

                            let hover_bg = if item.destructive {
                                theme.tokens.destructive.opacity(0.1)
                            } else {
                                theme.tokens.accent
                            };

                            let on_click = item.on_click.clone();
                            let state_for_click = state.clone();
                            let disabled = item.disabled;

                            div()
                                .id(item.id.clone())
                                .flex()
                                .items_center()
                                .gap(px(8.0))
                                .px(px(12.0))
                                .py(px(8.0))
                                .mx(px(4.0))
                                .rounded(px(4.0))
                                .text_size(px(14.0))
                                .text_color(text_color)
                                .font_family(theme.tokens.font_family.clone())
                                .when(!disabled, |d| {
                                    d.cursor_pointer().hover(move |s| s.bg(hover_bg))
                                })
                                .when(!disabled, move |d| {
                                    d.on_click(move |_, window, cx| {
                                        if let Some(ref handler) = on_click {
                                            handler(window, cx);
                                        }
                                        state_for_click.update(cx, |s, cx| s.close(cx));
                                    })
                                })
                                .when_some(item.icon.as_ref(), |d, icon| {
                                    d.child(
                                        Icon::new(icon.clone()).size(px(16.0)).color(text_color),
                                    )
                                })
                                .child(item.label.clone())
                                .into_any_element()
                        })),
                )
            })
            .map(|this| {
                let mut div = this;
                div.style().refine(&user_style);
                div
            })
    }
}
