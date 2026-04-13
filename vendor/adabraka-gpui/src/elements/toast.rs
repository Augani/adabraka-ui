use std::time::Duration;

use crate::{
    AnyElement, Context, IntoElement, ParentElement, Render, SharedString, Styled, Timer,
    WeakEntity, Window, WindowAppearance, div, hsla, px,
};

/// Position where toasts appear on screen.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ToastPosition {
    /// Top-right corner of the window.
    #[default]
    TopRight,
    /// Bottom-right corner of the window.
    BottomRight,
    /// Top-center of the window.
    TopCenter,
}

/// Configuration for a single toast notification.
#[derive(Clone)]
pub struct Toast {
    title: SharedString,
    body: Option<SharedString>,
    duration: Duration,
    position: ToastPosition,
}

impl Toast {
    /// Create a new toast with the given title.
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            body: None,
            duration: Duration::from_secs(3),
            position: ToastPosition::default(),
        }
    }

    /// Set the body text of the toast.
    pub fn body(mut self, body: impl Into<SharedString>) -> Self {
        self.body = Some(body.into());
        self
    }

    /// Set how long the toast should be displayed before auto-dismissing.
    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    /// Set the screen position where the toast appears.
    pub fn position(mut self, position: ToastPosition) -> Self {
        self.position = position;
        self
    }
}

struct ToastEntry {
    toast: Toast,
}

/// A stack of toast notifications that manages display and auto-dismissal.
///
/// Create a `ToastStack` as a GPUI entity and render it as part of your
/// window's view tree. Use [`ToastStack::push`] to add new toasts.
pub struct ToastStack {
    toasts: Vec<ToastEntry>,
    position: ToastPosition,
}

impl ToastStack {
    /// Create a new empty toast stack with the default position.
    pub fn new() -> Self {
        Self {
            toasts: Vec::new(),
            position: ToastPosition::default(),
        }
    }

    /// Set the default position for toasts in this stack.
    pub fn with_position(mut self, position: ToastPosition) -> Self {
        self.position = position;
        self
    }

    /// Push a new toast onto the stack and schedule its auto-dismissal.
    pub fn push(&mut self, toast: Toast, window: &Window, cx: &mut Context<Self>) {
        let duration = toast.duration;
        self.toasts.push(ToastEntry { toast });
        cx.notify();

        let index = self.toasts.len() - 1;
        cx.spawn_in(window, async move |this: WeakEntity<Self>, cx| {
            Timer::after(duration).await;
            this.update(cx, |stack, cx| {
                if index < stack.toasts.len() {
                    stack.toasts.remove(index);
                    cx.notify();
                }
            })
            .ok();
        })
        .detach();
    }

    /// Remove all toasts from the stack.
    pub fn clear(&mut self, cx: &mut Context<Self>) {
        self.toasts.clear();
        cx.notify();
    }

    fn is_dark_appearance(window: &Window) -> bool {
        matches!(
            window.appearance(),
            WindowAppearance::Dark | WindowAppearance::VibrantDark
        )
    }
}

impl Render for ToastStack {
    fn render(&mut self, window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let is_dark = Self::is_dark_appearance(window);
        let position = self.position;

        let mut container = div().flex().flex_col().gap_2().p_4().max_w(px(360.0));

        match position {
            ToastPosition::TopRight => {
                container = container.absolute().top_0().right_0();
            }
            ToastPosition::BottomRight => {
                container = container.absolute().bottom_0().right_0();
            }
            ToastPosition::TopCenter => {
                container = container.absolute().top_0().left_auto().right_auto();
            }
        }

        let children: Vec<AnyElement> = self
            .toasts
            .iter()
            .map(|entry| render_toast_item(&entry.toast, is_dark))
            .collect();

        for child in children {
            container = container.child(child);
        }

        container
    }
}

fn render_toast_item(toast: &Toast, is_dark: bool) -> AnyElement {
    let bg_color = if is_dark {
        hsla(0.0, 0.0, 0.1, 0.92)
    } else {
        hsla(0.0, 0.0, 0.0, 0.85)
    };

    let text_color = if is_dark {
        hsla(0.0, 0.0, 0.95, 1.0)
    } else {
        hsla(0.0, 0.0, 1.0, 1.0)
    };

    let secondary_text_color = if is_dark {
        hsla(0.0, 0.0, 0.7, 1.0)
    } else {
        hsla(0.0, 0.0, 0.85, 1.0)
    };

    let title = toast.title.clone();
    let body = toast.body.clone();

    let mut toast_div = div()
        .flex()
        .flex_col()
        .gap_1()
        .py(px(12.0))
        .px(px(16.0))
        .rounded(px(8.0))
        .bg(bg_color)
        .shadow_lg()
        .max_w(px(320.0))
        .min_w(px(200.0))
        .text_color(text_color)
        .text_sm()
        .child(div().font_weight(crate::FontWeight::SEMIBOLD).child(title));

    if let Some(body_text) = body {
        toast_div = toast_div.child(
            div()
                .text_xs()
                .text_color(secondary_text_color)
                .child(body_text),
        );
    }

    toast_div.into_any_element()
}
