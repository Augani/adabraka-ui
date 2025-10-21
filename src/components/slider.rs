//! Slider component - Range input slider for numeric value selection.

use gpui::{prelude::FluentBuilder as _, *};
use std::rc::Rc;
use crate::theme::use_theme;

#[derive(IntoElement)]
pub struct Slider {
    base: Stateful<Div>,
    id: ElementId,
    min: f32,
    max: f32,
    step: f32,
    value: f32,
    disabled: bool,
    on_change: Option<Rc<dyn Fn(f32, &mut App)>>,
}

impl Slider {
    pub fn new(id: impl Into<ElementId>) -> Self {
        let id = id.into();
        Self {
            id: id.clone(),
            base: div().id(id),
            min: 0.0,
            max: 100.0,
            step: 1.0,
            value: 0.0,
            disabled: false,
            on_change: None,
        }
    }

    pub fn min(mut self, min: f32) -> Self {
        self.min = min;
        self
    }

    pub fn max(mut self, max: f32) -> Self {
        self.max = max;
        self
    }

    pub fn step(mut self, step: f32) -> Self {
        self.step = step;
        self
    }

    pub fn value(mut self, value: f32) -> Self {
        self.value = value.clamp(self.min, self.max);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_change(mut self, handler: impl Fn(f32, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }

    fn percentage(&self) -> f32 {
        if self.max == self.min {
            return 0.0;
        }
        ((self.value - self.min) / (self.max - self.min)).clamp(0.0, 1.0)
    }
}

impl InteractiveElement for Slider {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl StatefulInteractiveElement for Slider {}

impl RenderOnce for Slider {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = use_theme();

        let focus_handle = window
            .use_keyed_state(self.id.clone(), cx, |_, cx| cx.focus_handle())
            .read(cx)
            .clone();
        let is_focused = focus_handle.is_focused(window);

        let percentage = self.percentage();
        let _thumb_position = percentage * 100.0;

        let (track_color, active_color, thumb_color) = if self.disabled {
            (
                theme.tokens.muted.opacity(0.3),
                theme.tokens.primary.opacity(0.5),
                theme.tokens.primary.opacity(0.5),
            )
        } else {
            (
                theme.tokens.muted,
                theme.tokens.primary,
                theme.tokens.primary,
            )
        };

        let shadow_xs = BoxShadow {
            offset: theme.tokens.shadow_xs.offset,
            blur_radius: theme.tokens.shadow_xs.blur_radius,
            spread_radius: theme.tokens.shadow_xs.spread_radius,
            color: theme.tokens.shadow_xs.color,
        };
        let focus_ring = theme.tokens.focus_ring_light();

        self.base
            .when(!self.disabled, |this| {
                this.track_focus(&focus_handle.tab_index(0).tab_stop(true))
            })
            .relative()
            .w_full()
            .h(px(20.0))
            .flex()
            .items_center()
            .when(is_focused && !self.disabled, |this| {
                this.shadow(vec![focus_ring])
            })
            .rounded(theme.tokens.radius_md)
            .child(
                div()
                    .relative()
                    .w_full()
                    .h(px(4.0))
                    .rounded_full()
                    .bg(track_color)
                    .overflow_hidden()
                    .child(
                        div()
                            .absolute()
                            .top_0()
                            .left_0()
                            .h_full()
                            .w(relative(percentage))
                            .bg(active_color),
                    )
                    .child(
                        div()
                            .absolute()
                            .top(px(-6.0))
                            .left(relative(percentage))
                            .size(px(16.0))
                            .rounded_full()
                            .bg(thumb_color)
                            .border_2()
                            .border_color(theme.tokens.background)
                            .when(!self.disabled, |this| this.shadow(vec![shadow_xs]))
                            .when(!self.disabled, |this| {
                                this.cursor(CursorStyle::PointingHand)
                            }),
                    ),
            )
    }
}
