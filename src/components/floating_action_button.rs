//! Floating action button with expandable action items.

use gpui::{prelude::FluentBuilder as _, *};
use std::rc::Rc;
use std::time::Duration;

use crate::animations::{durations, easings};
use crate::theme::use_theme;

struct FABAction {
    id: SharedString,
    icon: SharedString,
    handler: Rc<dyn Fn(&mut Window, &mut App)>,
}

pub struct FABState {
    is_expanded: bool,
    animation_version: usize,
}

impl FABState {
    pub fn new() -> Self {
        Self {
            is_expanded: false,
            animation_version: 0,
        }
    }

    pub fn is_expanded(&self) -> bool {
        self.is_expanded
    }

    pub fn toggle(&mut self, cx: &mut Context<Self>) {
        self.is_expanded = !self.is_expanded;
        self.animation_version = self.animation_version.wrapping_add(1);
        cx.notify();
    }

    pub fn expand(&mut self, cx: &mut Context<Self>) {
        if !self.is_expanded {
            self.toggle(cx);
        }
    }

    pub fn collapse(&mut self, cx: &mut Context<Self>) {
        if self.is_expanded {
            self.toggle(cx);
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum FABSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl FABSize {
    fn main_size(&self) -> Pixels {
        match self {
            Self::Sm => px(48.0),
            Self::Md => px(56.0),
            Self::Lg => px(64.0),
        }
    }

    fn action_size(&self) -> Pixels {
        match self {
            Self::Sm => px(36.0),
            Self::Md => px(44.0),
            Self::Lg => px(52.0),
        }
    }

    fn icon_size(&self) -> Pixels {
        match self {
            Self::Sm => px(18.0),
            Self::Md => px(22.0),
            Self::Lg => px(26.0),
        }
    }

    fn action_icon_size(&self) -> Pixels {
        match self {
            Self::Sm => px(14.0),
            Self::Md => px(18.0),
            Self::Lg => px(22.0),
        }
    }
}

#[derive(IntoElement)]
pub struct FloatingActionButton {
    id: ElementId,
    state: Entity<FABState>,
    icon: SharedString,
    actions: Vec<FABAction>,
    fab_size: FABSize,
    stagger: Duration,
    style: StyleRefinement,
}

impl FloatingActionButton {
    pub fn new(id: impl Into<ElementId>, state: Entity<FABState>) -> Self {
        Self {
            id: id.into(),
            state,
            icon: "+".into(),
            actions: Vec::new(),
            fab_size: FABSize::default(),
            stagger: Duration::from_millis(50),
            style: StyleRefinement::default(),
        }
    }

    pub fn icon(mut self, icon: impl Into<SharedString>) -> Self {
        self.icon = icon.into();
        self
    }

    pub fn action<F>(
        mut self,
        id: impl Into<SharedString>,
        icon: impl Into<SharedString>,
        handler: F,
    ) -> Self
    where
        F: Fn(&mut Window, &mut App) + 'static,
    {
        self.actions.push(FABAction {
            id: id.into(),
            icon: icon.into(),
            handler: Rc::new(handler),
        });
        self
    }

    pub fn size(mut self, size: FABSize) -> Self {
        self.fab_size = size;
        self
    }

    pub fn stagger(mut self, stagger: Duration) -> Self {
        self.stagger = stagger;
        self
    }
}

impl Styled for FloatingActionButton {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for FloatingActionButton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let user_style = self.style;
        let state = self.state.read(cx);
        let is_expanded = state.is_expanded;
        let animation_version = state.animation_version;
        let main_size = self.fab_size.main_size();
        let action_size = self.fab_size.action_size();
        let icon_size = self.fab_size.icon_size();
        let action_icon_size = self.fab_size.action_icon_size();
        let stagger = self.stagger;
        let state_for_toggle = self.state.clone();

        div()
            .id(self.id)
            .flex()
            .flex_col_reverse()
            .items_center()
            .gap(px(12.0))
            .child(
                div()
                    .id("fab-main")
                    .flex()
                    .items_center()
                    .justify_center()
                    .size(main_size)
                    .rounded_full()
                    .bg(theme.tokens.primary)
                    .text_color(theme.tokens.primary_foreground)
                    .text_size(icon_size)
                    .font_weight(FontWeight::BOLD)
                    .cursor_pointer()
                    .shadow(smallvec::smallvec![BoxShadow {
                        color: hsla(0.0, 0.0, 0.0, 0.2),
                        offset: point(px(0.0), px(4.0)),
                        blur_radius: px(12.0),
                        spread_radius: px(0.0),
                        inset: false,
                    }])
                    .hover(|s| s.opacity(0.9))
                    .active(|s| s.opacity(0.8))
                    .on_mouse_down(MouseButton::Left, move |_, _, cx| {
                        state_for_toggle.update(cx, |s, cx| s.toggle(cx));
                    })
                    .child(self.icon.clone())
                    .with_animation(
                        ElementId::Name(format!("fab-rotate-{}", animation_version).into()),
                        Animation::new(durations::FAST).with_easing(easings::ease_out_cubic),
                        move |el, delta| {
                            if is_expanded {
                                el.opacity(0.7 + 0.3 * delta)
                            } else {
                                el.opacity(1.0)
                            }
                        },
                    ),
            )
            .when(is_expanded, |this| {
                let action_count = self.actions.len();
                this.children(
                    self.actions
                        .into_iter()
                        .enumerate()
                        .map(move |(idx, action)| {
                            let handler = action.handler.clone();
                            let delay = Duration::from_millis(
                                stagger.as_millis() as u64 * (action_count - 1 - idx) as u64,
                            );

                            div()
                                .id(ElementId::Name(format!("fab-action-{}", action.id).into()))
                                .flex()
                                .items_center()
                                .justify_center()
                                .size(action_size)
                                .rounded_full()
                                .bg(theme.tokens.secondary)
                                .text_color(theme.tokens.secondary_foreground)
                                .text_size(action_icon_size)
                                .cursor_pointer()
                                .shadow(smallvec::smallvec![BoxShadow {
                                    color: hsla(0.0, 0.0, 0.0, 0.15),
                                    offset: point(px(0.0), px(2.0)),
                                    blur_radius: px(8.0),
                                    spread_radius: px(0.0),
                                    inset: false,
                                }])
                                .hover(|s| s.opacity(0.9))
                                .active(|s| s.opacity(0.8))
                                .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                                    (handler)(window, cx);
                                })
                                .child(action.icon.clone())
                                .with_animation(
                                    ElementId::Name(
                                        format!("fab-action-anim-{}-{}", idx, animation_version)
                                            .into(),
                                    ),
                                    Animation::new(durations::FAST + delay)
                                        .with_easing(easings::ease_out_cubic),
                                    move |el, delta| {
                                        let adjusted = (delta
                                            - delay.as_secs_f32()
                                                / (durations::FAST + delay).as_secs_f32())
                                        .max(0.0)
                                            / (1.0
                                                - delay.as_secs_f32()
                                                    / (durations::FAST + delay).as_secs_f32())
                                            .max(0.001);
                                        el.opacity(adjusted).mt(px(-8.0 * (1.0 - adjusted)))
                                    },
                                )
                        }),
                )
            })
            .map(|this| {
                let mut el = this;
                el.style().refine(&user_style);
                el
            })
    }
}
