//! Segmented navigation with animated sliding highlight indicator.

use gpui::{prelude::FluentBuilder as _, *};
use std::rc::Rc;
use std::time::Duration;

use crate::animations::{durations, easings};
use crate::theme::use_theme;

#[derive(Clone)]
struct SegmentedNavItem {
    id: SharedString,
    label: SharedString,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SegmentedNavSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl SegmentedNavSize {
    fn height(&self) -> Pixels {
        match self {
            Self::Sm => px(32.0),
            Self::Md => px(40.0),
            Self::Lg => px(48.0),
        }
    }

    fn text_size(&self) -> Pixels {
        match self {
            Self::Sm => px(12.0),
            Self::Md => px(14.0),
            Self::Lg => px(16.0),
        }
    }

    fn padding_x(&self) -> Pixels {
        match self {
            Self::Sm => px(8.0),
            Self::Md => px(12.0),
            Self::Lg => px(16.0),
        }
    }
}

pub struct SegmentedNavState {
    active: SharedString,
    previous_active: Option<SharedString>,
    items: Vec<SegmentedNavItem>,
    animation_version: usize,
}

impl SegmentedNavState {
    pub fn new(active: impl Into<SharedString>) -> Self {
        Self {
            active: active.into(),
            previous_active: None,
            items: Vec::new(),
            animation_version: 0,
        }
    }

    pub fn set_active(&mut self, id: impl Into<SharedString>, cx: &mut Context<Self>) {
        let new_id = id.into();
        if self.active != new_id {
            self.previous_active = Some(self.active.clone());
            self.active = new_id;
            self.animation_version = self.animation_version.wrapping_add(1);
            cx.notify();
        }
    }

    pub fn active(&self) -> &SharedString {
        &self.active
    }

    fn active_index(&self) -> Option<usize> {
        self.items.iter().position(|item| item.id == self.active)
    }
}

#[derive(IntoElement)]
pub struct SegmentedNav {
    id: ElementId,
    state: Entity<SegmentedNavState>,
    items: Vec<SegmentedNavItem>,
    nav_size: SegmentedNavSize,
    on_change: Option<Rc<dyn Fn(SharedString, &mut Window, &mut App)>>,
    duration: Duration,
    style: StyleRefinement,
}

impl SegmentedNav {
    pub fn new(id: impl Into<ElementId>, state: Entity<SegmentedNavState>) -> Self {
        Self {
            id: id.into(),
            state,
            items: Vec::new(),
            nav_size: SegmentedNavSize::default(),
            on_change: None,
            duration: durations::NORMAL,
            style: StyleRefinement::default(),
        }
    }

    pub fn item(mut self, id: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        self.items.push(SegmentedNavItem {
            id: id.into(),
            label: label.into(),
        });
        self
    }

    pub fn size(mut self, size: SegmentedNavSize) -> Self {
        self.nav_size = size;
        self
    }

    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    pub fn on_change<F>(mut self, handler: F) -> Self
    where
        F: Fn(SharedString, &mut Window, &mut App) + 'static,
    {
        self.on_change = Some(Rc::new(handler));
        self
    }
}

impl Styled for SegmentedNav {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for SegmentedNav {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let user_style = self.style;
        let state = self.state.read(cx);
        let active_id = state.active.clone();
        let item_count = self.items.len();
        let active_index = self.items.iter().position(|i| i.id == active_id);
        let animation_version = state.animation_version;
        let duration = self.duration;

        self.state.update(cx, |state, _| {
            state.items = self.items.clone();
        });

        let item_fraction = if item_count > 0 {
            1.0 / item_count as f32
        } else {
            1.0
        };

        div()
            .id(self.id)
            .flex()
            .items_center()
            .relative()
            .bg(theme.tokens.muted)
            .rounded(theme.tokens.radius_md)
            .p(px(4.0))
            .h(self.nav_size.height())
            .when(active_index.is_some(), |this| {
                let idx = active_index.unwrap();
                this.child(
                    div()
                        .id("segmented-indicator")
                        .absolute()
                        .top(px(4.0))
                        .bottom(px(4.0))
                        .rounded(theme.tokens.radius_sm)
                        .bg(theme.tokens.background)
                        .shadow(smallvec::smallvec![BoxShadow {
                            color: hsla(0.0, 0.0, 0.0, 0.08),
                            offset: point(px(0.0), px(1.0)),
                            blur_radius: px(3.0),
                            spread_radius: px(0.0),
                            inset: false,
                        }])
                        .with_animation(
                            ElementId::Name(format!("seg-slide-{}", animation_version).into()),
                            Animation::new(duration).with_easing(easings::ease_out_cubic),
                            move |el, delta| {
                                let frac = item_fraction;
                                let left_pct = idx as f32 * frac * 100.0;
                                let width_pct = frac * 100.0;
                                el.left(relative(
                                    left_pct / 100.0 * delta + left_pct / 100.0 * (1.0 - delta),
                                ))
                                .w(relative(width_pct / 100.0))
                            },
                        ),
                )
            })
            .children(self.items.iter().enumerate().map(|(idx, item)| {
                let item_id = item.id.clone();
                let is_active = item.id == active_id;
                let on_change = self.on_change.clone();
                let state = self.state.clone();
                let click_id = item_id.clone();

                div()
                    .id(ElementId::Name(format!("seg-item-{}", idx).into()))
                    .flex_1()
                    .flex()
                    .items_center()
                    .justify_center()
                    .h_full()
                    .px(self.nav_size.padding_x())
                    .text_size(self.nav_size.text_size())
                    .font_weight(if is_active {
                        FontWeight::MEDIUM
                    } else {
                        FontWeight::NORMAL
                    })
                    .text_color(if is_active {
                        theme.tokens.foreground
                    } else {
                        theme.tokens.muted_foreground
                    })
                    .cursor_pointer()
                    .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                        state.update(cx, |state, cx| {
                            state.set_active(click_id.clone(), cx);
                        });
                        if let Some(handler) = on_change.as_ref() {
                            handler(item_id.clone(), window, cx);
                        }
                    })
                    .child(item.label.clone())
            }))
            .map(|this| {
                let mut el = this;
                el.style().refine(&user_style);
                el
            })
    }
}
