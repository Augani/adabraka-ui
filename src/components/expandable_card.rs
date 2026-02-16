//! Expandable card with animated expand/collapse transitions.

use gpui::{prelude::FluentBuilder as _, *};
use std::time::Duration;

use crate::animations::{durations, easings};
use crate::theme::use_theme;

pub struct ExpandableCardState {
    is_expanded: bool,
    is_animating: bool,
    is_expanding: bool,
    animation_version: usize,
}

impl ExpandableCardState {
    pub fn new() -> Self {
        Self {
            is_expanded: false,
            is_animating: false,
            is_expanding: false,
            animation_version: 0,
        }
    }

    pub fn is_expanded(&self) -> bool {
        self.is_expanded
    }

    pub fn toggle(&mut self, cx: &mut Context<Self>) {
        if self.is_expanded {
            self.collapse(cx);
        } else {
            self.expand(cx);
        }
    }

    pub fn expand(&mut self, cx: &mut Context<Self>) {
        if !self.is_expanded && !self.is_animating {
            self.is_expanded = true;
            self.is_expanding = true;
            self.is_animating = true;
            self.animation_version = self.animation_version.wrapping_add(1);
            cx.notify();

            cx.spawn(async move |this, cx| {
                cx.background_executor()
                    .timer(Duration::from_millis(300))
                    .await;
                _ = this.update(cx, |state, cx| {
                    state.is_animating = false;
                    state.is_expanding = false;
                    cx.notify();
                });
            })
            .detach();
        }
    }

    pub fn collapse(&mut self, cx: &mut Context<Self>) {
        if self.is_expanded && !self.is_animating {
            self.is_expanding = false;
            self.is_animating = true;
            self.animation_version = self.animation_version.wrapping_add(1);
            cx.notify();

            cx.spawn(async move |this, cx| {
                cx.background_executor()
                    .timer(Duration::from_millis(300))
                    .await;
                _ = this.update(cx, |state, cx| {
                    state.is_expanded = false;
                    state.is_animating = false;
                    cx.notify();
                });
            })
            .detach();
        }
    }
}

#[derive(IntoElement)]
pub struct ExpandableCard {
    id: ElementId,
    state: Entity<ExpandableCardState>,
    collapsed_content: Option<AnyElement>,
    expanded_content: Option<AnyElement>,
    duration: Duration,
    style: StyleRefinement,
}

impl ExpandableCard {
    pub fn new(id: impl Into<ElementId>, state: Entity<ExpandableCardState>) -> Self {
        Self {
            id: id.into(),
            state,
            collapsed_content: None,
            expanded_content: None,
            duration: durations::NORMAL,
            style: StyleRefinement::default(),
        }
    }

    pub fn collapsed(mut self, content: impl IntoElement) -> Self {
        self.collapsed_content = Some(content.into_any_element());
        self
    }

    pub fn expanded(mut self, content: impl IntoElement) -> Self {
        self.expanded_content = Some(content.into_any_element());
        self
    }

    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }
}

impl Styled for ExpandableCard {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for ExpandableCard {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let user_style = self.style;
        let state = self.state.read(cx);
        let is_expanded = state.is_expanded;
        let is_animating = state.is_animating;
        let is_expanding = state.is_expanding;
        let animation_version = state.animation_version;
        let duration = self.duration;
        let state_for_click = self.state.clone();

        let shadow = BoxShadow {
            color: hsla(0.0, 0.0, 0.0, 0.08),
            offset: point(px(0.0), px(1.0)),
            blur_radius: px(3.0),
            spread_radius: px(0.0),
            inset: false,
        };

        div()
            .id(self.id)
            .bg(theme.tokens.card)
            .border_1()
            .border_color(theme.tokens.border)
            .rounded(theme.tokens.radius_lg)
            .shadow(smallvec::smallvec![shadow])
            .overflow_hidden()
            .cursor_pointer()
            .on_mouse_down(MouseButton::Left, move |_, _, cx| {
                state_for_click.update(cx, |s, cx| s.toggle(cx));
            })
            .when(!is_expanded && !is_animating, |this| {
                this.when_some(self.collapsed_content, |this, content| {
                    this.child(div().px(px(24.0)).py(px(16.0)).child(content))
                })
            })
            .when(is_expanded || is_animating, |this| {
                this.when_some(self.expanded_content, |this, content| {
                    this.child(
                        div()
                            .id("expandable-content")
                            .px(px(24.0))
                            .py(px(16.0))
                            .overflow_hidden()
                            .child(content)
                            .with_animation(
                                ElementId::Name(format!("expand-{}", animation_version).into()),
                                Animation::new(duration).with_easing(easings::ease_out_cubic),
                                move |el, delta| {
                                    if is_expanding {
                                        el.opacity(delta)
                                    } else if is_animating && !is_expanding {
                                        el.opacity(1.0 - delta)
                                    } else {
                                        el.opacity(1.0)
                                    }
                                },
                            ),
                    )
                })
            })
            .map(|this| {
                let mut el = this;
                el.style().refine(&user_style);
                el
            })
    }
}
