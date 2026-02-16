//! Drawer navigation - Side panel that slides in with backdrop overlay.

use gpui::{prelude::FluentBuilder as _, *};
use std::rc::Rc;
use std::time::Duration;

use crate::animations::{durations, easings};
use crate::theme::use_theme;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DrawerSide {
    #[default]
    Left,
    Right,
}

pub struct DrawerState {
    is_open: bool,
    is_animating: bool,
    is_dismissing: bool,
    animation_version: usize,
}

impl DrawerState {
    pub fn new() -> Self {
        Self {
            is_open: false,
            is_animating: false,
            is_dismissing: false,
            animation_version: 0,
        }
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }

    pub fn open(&mut self, cx: &mut Context<Self>) {
        if !self.is_open {
            self.is_open = true;
            self.is_dismissing = false;
            self.is_animating = true;
            self.animation_version = self.animation_version.wrapping_add(1);
            cx.notify();
        }
    }

    pub fn close(&mut self, cx: &mut Context<Self>) {
        if self.is_open && !self.is_dismissing {
            self.is_dismissing = true;
            self.animation_version = self.animation_version.wrapping_add(1);
            cx.notify();

            cx.spawn(async move |this, cx| {
                cx.background_executor()
                    .timer(Duration::from_millis(250))
                    .await;
                _ = this.update(cx, |state, cx| {
                    state.is_open = false;
                    state.is_dismissing = false;
                    state.is_animating = false;
                    cx.notify();
                });
            })
            .detach();
        }
    }

    pub fn toggle(&mut self, cx: &mut Context<Self>) {
        if self.is_open {
            self.close(cx);
        } else {
            self.open(cx);
        }
    }
}

#[derive(IntoElement)]
pub struct DrawerNavigation {
    id: ElementId,
    state: Entity<DrawerState>,
    side: DrawerSide,
    drawer_width: Pixels,
    show_backdrop: bool,
    children: Vec<AnyElement>,
    on_close: Option<Rc<dyn Fn(&mut Window, &mut App)>>,
    duration: Duration,
    style: StyleRefinement,
}

impl DrawerNavigation {
    pub fn new(id: impl Into<ElementId>, state: Entity<DrawerState>) -> Self {
        Self {
            id: id.into(),
            state,
            side: DrawerSide::default(),
            drawer_width: px(280.0),
            show_backdrop: true,
            children: Vec::new(),
            on_close: None,
            duration: durations::NORMAL,
            style: StyleRefinement::default(),
        }
    }

    pub fn side(mut self, side: DrawerSide) -> Self {
        self.side = side;
        self
    }

    pub fn width(mut self, width: impl Into<Pixels>) -> Self {
        self.drawer_width = width.into();
        self
    }

    pub fn show_backdrop(mut self, show: bool) -> Self {
        self.show_backdrop = show;
        self
    }

    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    pub fn on_close<F>(mut self, handler: F) -> Self
    where
        F: Fn(&mut Window, &mut App) + 'static,
    {
        self.on_close = Some(Rc::new(handler));
        self
    }
}

impl Styled for DrawerNavigation {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for DrawerNavigation {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for DrawerNavigation {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let user_style = self.style;
        let state = self.state.read(cx);
        let is_open = state.is_open;
        let is_dismissing = state.is_dismissing;
        let animation_version = state.animation_version;
        let duration = self.duration;
        let drawer_width = self.drawer_width;
        let side = self.side;

        if !is_open {
            return div().into_any_element();
        }

        let state_for_backdrop = self.state.clone();
        let on_close = self.on_close.clone();

        deferred(
            div()
                .id(self.id)
                .absolute()
                .inset_0()
                .when(self.show_backdrop, |this| {
                    this.child(
                        div()
                            .id("drawer-backdrop")
                            .absolute()
                            .inset_0()
                            .bg(hsla(0.0, 0.0, 0.0, 0.5))
                            .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                                state_for_backdrop.update(cx, |s, cx| s.close(cx));
                                if let Some(handler) = on_close.as_ref() {
                                    handler(window, cx);
                                }
                            })
                            .with_animation(
                                ElementId::Name(
                                    format!("drawer-backdrop-{}", animation_version).into(),
                                ),
                                Animation::new(duration).with_easing(easings::ease_out_cubic),
                                move |el, delta| {
                                    if is_dismissing {
                                        el.opacity(1.0 - delta)
                                    } else {
                                        el.opacity(delta)
                                    }
                                },
                            ),
                    )
                })
                .child(
                    div()
                        .id("drawer-panel")
                        .occlude()
                        .absolute()
                        .top_0()
                        .bottom_0()
                        .w(drawer_width)
                        .when(side == DrawerSide::Left, |this| this.left_0())
                        .when(side == DrawerSide::Right, |this| this.right_0())
                        .bg(theme.tokens.background)
                        .border_color(theme.tokens.border)
                        .when(side == DrawerSide::Left, |this| this.border_r_1())
                        .when(side == DrawerSide::Right, |this| this.border_l_1())
                        .shadow(smallvec::smallvec![BoxShadow {
                            color: hsla(0.0, 0.0, 0.0, 0.15),
                            offset: point(
                                if side == DrawerSide::Left {
                                    px(4.0)
                                } else {
                                    px(-4.0)
                                },
                                px(0.0)
                            ),
                            blur_radius: px(16.0),
                            spread_radius: px(0.0),
                            inset: false,
                        }])
                        .flex()
                        .flex_col()
                        .overflow_hidden()
                        .children(self.children)
                        .map(|this| {
                            let mut el = this;
                            el.style().refine(&user_style);
                            el
                        })
                        .on_mouse_down(MouseButton::Left, |_, _, _| {})
                        .with_animation(
                            ElementId::Name(format!("drawer-slide-{}", animation_version).into()),
                            Animation::new(duration).with_easing(easings::ease_out_cubic),
                            move |el, delta| {
                                let offset = drawer_width / px(1.0);
                                if is_dismissing {
                                    match side {
                                        DrawerSide::Left => el.ml(px(-offset * delta)),
                                        DrawerSide::Right => el.mr(px(-offset * delta)),
                                    }
                                } else {
                                    match side {
                                        DrawerSide::Left => el.ml(px(-offset * (1.0 - delta))),
                                        DrawerSide::Right => el.mr(px(-offset * (1.0 - delta))),
                                    }
                                }
                            },
                        ),
                ),
        )
        .with_priority(1)
        .into_any_element()
    }
}
