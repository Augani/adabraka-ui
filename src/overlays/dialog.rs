//! Dialog component with focus trap and backdrop.

use gpui::{prelude::FluentBuilder as _, *};
use std::rc::Rc;

use crate::theme::use_theme;
use crate::components::button::{Button, ButtonVariant, ButtonSize};

actions!(dialog, [DialogCancel]);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DialogSize {
    Sm,
    Md,
    Lg,
    Xl,
    Full,
}

impl DialogSize {
    fn width(&self) -> Length {
        match self {
            Self::Sm => px(400.0).into(),
            Self::Md => px(500.0).into(),
            Self::Lg => px(600.0).into(),
            Self::Xl => px(800.0).into(),
            Self::Full => relative(0.95).into(),
        }
    }

    fn max_height(&self) -> Length {
        match self {
            Self::Full => relative(0.95).into(),
            _ => relative(0.85).into(),
        }
    }
}

pub struct Dialog {
    focus_handle: FocusHandle,
    header: Option<AnyElement>,
    title: Option<SharedString>,
    description: Option<SharedString>,
    size: DialogSize,
    children: Vec<AnyElement>,
    footer: Option<AnyElement>,
    show_close_button: bool,
    close_on_backdrop_click: bool,
    close_on_escape: bool,
    on_close: Option<Rc<dyn Fn(&mut Window, &mut App)>>,
    focused: bool,
    style: StyleRefinement,
}

impl Dialog {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            header: None,
            title: None,
            description: None,
            size: DialogSize::Md,
            children: vec![],
            footer: None,
            show_close_button: true,
            close_on_backdrop_click: true,
            close_on_escape: true,
            on_close: None,
            focused: false,
            style: StyleRefinement::default(),
        }
    }

    pub fn header(mut self, header: impl IntoElement) -> Self {
        self.header = Some(header.into_any_element());
        self
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn size(mut self, size: DialogSize) -> Self {
        self.size = size;
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    pub fn children<I>(mut self, children: impl IntoIterator<Item = I>) -> Self
    where
        I: IntoElement,
    {
        for child in children {
            self.children.push(child.into_any_element());
        }
        self
    }

    pub fn footer(mut self, footer: impl IntoElement) -> Self {
        self.footer = Some(footer.into_any_element());
        self
    }

    pub fn show_close_button(mut self, show: bool) -> Self {
        self.show_close_button = show;
        self
    }

    pub fn close_on_backdrop_click(mut self, close: bool) -> Self {
        self.close_on_backdrop_click = close;
        self
    }

    pub fn close_on_escape(mut self, close: bool) -> Self {
        self.close_on_escape = close;
        self
    }

    pub fn on_close(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_close = Some(Rc::new(handler));
        self
    }

    fn handle_close(&self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(handler) = &self.on_close {
            (handler)(window, cx);
        }
    }
}

impl Styled for Dialog {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl Render for Dialog {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();
        let has_slot_header = self.header.is_some();
        let has_header = has_slot_header || self.title.is_some() || self.description.is_some() || self.show_close_button;

        let dialog_entity = cx.entity().clone();
        let user_style = self.style.clone();

        if !self.focused {
            window.focus(&self.focus_handle);
            self.focused = true;
        }

        div()
            .id("dialog-overlay")
            .absolute()
            .inset_0()
            .flex()
            .items_center()
            .justify_center()
            .bg(gpui::black().opacity(0.5))
            .child(
                div()
                    .id("dialog-content")
                    .occlude()
                    .key_context("Dialog")
                    .track_focus(&self.focus_handle)
                    .when(self.close_on_backdrop_click, |this| {
                        this.on_mouse_down_out(cx.listener(|this, _, window, cx| {
                            this.handle_close(window, cx);
                        }))
                    })
                    .on_action(cx.listener(|this, _: &DialogCancel, window, cx| {
                        if this.close_on_escape {
                            this.handle_close(window, cx);
                        }
                    }))
                    .w(self.size.width())
                    .max_h(self.size.max_height())
                    .flex()
                    .flex_col()
                    .bg(theme.tokens.card)
                    .border_1()
                    .border_color(theme.tokens.border)
                    .rounded(theme.tokens.radius_lg)
                    .shadow_xl()
                    .overflow_hidden()
                    .when(has_header, |this| {
                        if has_slot_header {
                            let header = self.header.take().unwrap();
                            this.child(header)
                        } else {
                            this.child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.0))
                                    .px(px(24.0))
                                    .pt(px(24.0))
                                    .pb(px(16.0))
                                    .when(self.footer.is_none() && self.children.is_empty(), |this| {
                                        this.pb(px(24.0))
                                    })
                                    .child(
                                        div()
                                            .flex()
                                            .items_start()
                                            .justify_between()
                                            .gap(px(16.0))
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .gap(px(4.0))
                                                    .flex_1()
                                                    .when_some(self.title.clone(), |this, title| {
                                                        this.child(
                                                            div()
                                                                .text_size(px(18.0))
                                                                .font_family(theme.tokens.font_family.clone())
                                                                .font_weight(FontWeight::SEMIBOLD)
                                                                .text_color(theme.tokens.foreground)
                                                                .line_height(relative(1.2))
                                                                .child(title)
                                                        )
                                                    })
                                                    .when_some(self.description.clone(), |this, desc| {
                                                        this.child(
                                                            div()
                                                                .text_size(px(14.0))
                                                                .font_family(theme.tokens.font_family.clone())
                                                                .text_color(theme.tokens.muted_foreground)
                                                                .line_height(relative(1.5))
                                                                .child(desc)
                                                        )
                                                    })
                                            )
                                            .when(self.show_close_button, |this| {
                                                let dialog_entity = dialog_entity.clone();
                                                this.child(
                                                    Button::new("dialog-close-btn", "×")
                                                        .variant(ButtonVariant::Ghost)
                                                        .size(ButtonSize::Icon)
                                                        .on_click(move |_, window, cx| {
                                                            cx.update_entity(&dialog_entity, |dialog, cx| {
                                                                dialog.handle_close(window, cx);
                                                            });
                                                        })
                                                )
                                            })
                                    )
                            )
                        }
                    })
                    .when(!self.children.is_empty(), |this| {
                        let children = std::mem::take(&mut self.children);
                        this.child(
                            div()
                                .flex()
                                .flex_col()
                                .gap(px(16.0))
                                .px(px(24.0))
                                .py(px(16.0))
                                .flex_1()
                                .children(children)
                        )
                    })
                    .map(|this| {
                        let mut div = this;
                        div.style().refine(&user_style);
                        div
                    })
                    .when_some(self.footer.take(), |this, footer| {
                        this.child(
                            div()
                                .flex()
                                .items_center()
                                .justify_end()
                                .gap(px(8.0))
                                .px(px(24.0))
                                .py(px(16.0))
                                .border_t_1()
                                .border_color(theme.tokens.border)
                                .child(footer)
                        )
                    })
            )
    }
}

impl Focusable for Dialog {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<()> for Dialog {}

pub fn init_dialog(cx: &mut App) {
    cx.bind_keys([KeyBinding::new("escape", DialogCancel, Some("Dialog"))]);
}
