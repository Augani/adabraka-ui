//! Toast notification component with auto-dismiss.

use gpui::{prelude::FluentBuilder as _, *};
use std::time::Duration;
use smol::Timer;

use crate::theme::use_theme;
use crate::components::icon::Icon;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ToastVariant {
    Default,
    Success,
    Warning,
    Error,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ToastPosition {
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

#[derive(Clone, Debug)]
pub struct ToastItem {
    pub id: u64,
    pub title: SharedString,
    pub description: Option<SharedString>,
    pub variant: ToastVariant,
    pub duration: Option<Duration>,
    pub style: StyleRefinement,
}

impl ToastItem {
    pub fn new(id: u64, title: impl Into<SharedString>) -> Self {
        Self {
            id,
            title: title.into(),
            description: None,
            variant: ToastVariant::Default,
            duration: Some(Duration::from_secs(5)),
            style: StyleRefinement::default(),
        }
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn variant(mut self, variant: ToastVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = Some(duration);
        self
    }

    pub fn persistent(mut self) -> Self {
        self.duration = None;
        self
    }
}

impl Styled for ToastItem {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

pub struct ToastManager {
    toasts: Vec<ToastItem>,
    position: ToastPosition,
    max_toasts: usize,
}

impl ToastManager {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            toasts: vec![],
            position: ToastPosition::BottomRight,
            max_toasts: 5,
        }
    }

    pub fn position(mut self, position: ToastPosition) -> Self {
        self.position = position;
        self
    }

    pub fn max_toasts(mut self, max: usize) -> Self {
        self.max_toasts = max;
        self
    }

    pub fn add_toast(&mut self, toast: ToastItem, window: &mut Window, cx: &mut Context<Self>) {
        if self.toasts.len() >= self.max_toasts {
            self.toasts.remove(0);
        }

        let id = toast.id;
        let duration = toast.duration;

        self.toasts.push(toast);

        if let Some(duration) = duration {
            cx.spawn_in(window, async move |this, cx| {
                Timer::after(duration).await;
                let _ = this.update(cx, |this, cx| {
                    this.dismiss_toast(id, cx);
                });
            })
            .detach();
        }

        cx.notify();
    }

    pub fn add_toast_no_dismiss(&mut self, toast: ToastItem, cx: &mut Context<Self>) {
        if self.toasts.len() >= self.max_toasts {
            self.toasts.remove(0);
        }

        self.toasts.push(toast);
        cx.notify();
    }

    pub fn dismiss_toast(&mut self, id: u64, cx: &mut Context<Self>) {
        self.toasts.retain(|t| t.id != id);
        cx.notify();
    }

    pub fn clear_all(&mut self, cx: &mut Context<Self>) {
        self.toasts.clear();
        cx.notify();
    }
}

impl Render for ToastManager {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        if self.toasts.is_empty() {
            return div().into_any_element();
        }

        let (v_pos, h_pos, v_anchor, items_order) = match self.position {
            ToastPosition::TopLeft => ("top", "left", "flex_col", false),
            ToastPosition::TopCenter => ("top", "center", "flex_col", false),
            ToastPosition::TopRight => ("top", "right", "flex_col", false),
            ToastPosition::BottomLeft => ("bottom", "left", "flex_col_reverse", true),
            ToastPosition::BottomCenter => ("bottom", "center", "flex_col_reverse", true),
            ToastPosition::BottomRight => ("bottom", "right", "flex_col_reverse", true),
        };

        let mut container = div()
            .absolute()
            .flex()
            .gap(px(12.0))
            .p(px(16.0))
            .max_w(px(420.0));

        container = match v_pos {
            "top" => container.top_0(),
            "bottom" => container.bottom_0(),
            _ => container,
        };

        container = match h_pos {
            "left" => container.left_0(),
            "right" => container.right_0(),
            "center" => container.left_0().right_0().mx_auto(),
            _ => container,
        };

        container = match v_anchor {
            "flex_col" => container.flex_col(),
            "flex_col_reverse" => container.flex_col_reverse(),
            _ => container,
        };

        let mut toasts_to_show = self.toasts.clone();
        if items_order {
            toasts_to_show.reverse();
        }

        container.children(
            toasts_to_show
                .into_iter()
                .map(|toast| {
                    let (bg_color, border_color, icon, icon_color) = match toast.variant {
                        ToastVariant::Default => (
                            theme.tokens.card,
                            theme.tokens.border,
                            "info",
                            theme.tokens.foreground,
                        ),
                        ToastVariant::Success => (
                            theme.tokens.card,
                            theme.tokens.border,
                            "check-circle",
                            gpui::hsla(142.0 / 360.0, 0.71, 0.45, 1.0), // green-500
                        ),
                        ToastVariant::Warning => (
                            theme.tokens.card,
                            theme.tokens.border,
                            "alert-circle",
                            gpui::hsla(48.0 / 360.0, 0.96, 0.53, 1.0), // yellow-500
                        ),
                        ToastVariant::Error => (
                            theme.tokens.destructive.opacity(0.1),
                            theme.tokens.destructive,
                            "x-circle",
                            theme.tokens.destructive,
                        ),
                    };

                    let user_style = toast.style.clone();

                    div()
                        .id(("toast", toast.id))
                        .flex()
                        .items_start()
                        .gap(px(12.0))
                        .w_full()
                        .min_w(px(300.0))
                        .bg(bg_color)
                        .border_1()
                        .border_color(border_color)
                        .rounded(theme.tokens.radius_md)
                        .p(px(16.0))
                        .shadow_lg()
                        .map(|this| {
                            let mut div = this;
                            div.style().refine(&user_style);
                            div
                        })
                        .child(
                            Icon::new(icon)
                                .size(px(20.0))
                                .color(icon_color)
                        )
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap(px(4.0))
                                .flex_1()
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .font_family(theme.tokens.font_family.clone())
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(theme.tokens.foreground)
                                        .line_height(relative(1.4))
                                        .child(toast.title)
                                )
                                .when_some(toast.description, |this, desc| {
                                    this.child(
                                        div()
                                            .text_size(px(13.0))
                                            .font_family(theme.tokens.font_family.clone())
                                            .text_color(theme.tokens.muted_foreground)
                                            .line_height(relative(1.4))
                                            .child(desc)
                                    )
                                })
                        )
                        .child(
                            div()
                                .w(px(20.0))
                                .h(px(20.0))
                                .flex()
                                .items_center()
                                .justify_center()
                                .rounded(theme.tokens.radius_sm)
                                .cursor(CursorStyle::PointingHand)
                                .text_color(theme.tokens.muted_foreground)
                                .text_size(px(16.0))
                                .font_family(theme.tokens.font_family.clone())
                                .hover(|style| {
                                    style.bg(theme.tokens.accent)
                                })
                                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _, cx| {
                                    this.dismiss_toast(toast.id, cx);
                                }))
                                .child("×")
                        )
                })
                .collect::<Vec<_>>(),
        )
        .into_any_element()
    }
}

impl EventEmitter<()> for ToastManager {}
