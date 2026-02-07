use crate::theme::use_theme;
use gpui::{prelude::FluentBuilder as _, Animation, AnimationExt, *};
use std::rc::Rc;
use std::time::Duration;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum CarouselTransition {
    #[default]
    Slide,
    Fade,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum CarouselSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl CarouselSize {
    fn arrow_size(&self) -> Pixels {
        match self {
            CarouselSize::Sm => px(28.0),
            CarouselSize::Md => px(36.0),
            CarouselSize::Lg => px(44.0),
        }
    }

    fn dot_size(&self) -> Pixels {
        match self {
            CarouselSize::Sm => px(6.0),
            CarouselSize::Md => px(8.0),
            CarouselSize::Lg => px(10.0),
        }
    }

    fn icon_size(&self) -> Pixels {
        match self {
            CarouselSize::Sm => px(14.0),
            CarouselSize::Md => px(18.0),
            CarouselSize::Lg => px(22.0),
        }
    }
}

pub struct CarouselState {
    current_index: usize,
    previous_index: Option<usize>,
    slide_count: usize,
    focus_handle: FocusHandle,
    transition_id: usize,
}

impl CarouselState {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            current_index: 0,
            previous_index: None,
            slide_count: 0,
            focus_handle: cx.focus_handle(),
            transition_id: 0,
        }
    }

    pub fn previous_index(&self) -> Option<usize> {
        self.previous_index
    }

    pub fn transition_id(&self) -> usize {
        self.transition_id
    }

    pub fn current_index(&self) -> usize {
        self.current_index
    }

    pub fn slide_count(&self) -> usize {
        self.slide_count
    }

    pub fn set_slide_count(&mut self, count: usize, cx: &mut Context<Self>) {
        self.slide_count = count;
        if self.current_index >= count && count > 0 {
            self.current_index = count - 1;
        }
        cx.notify();
    }

    pub fn go_to(&mut self, index: usize, infinite: bool, cx: &mut Context<Self>) {
        if self.slide_count == 0 {
            return;
        }

        let new_index = if infinite {
            index % self.slide_count
        } else {
            index.min(self.slide_count.saturating_sub(1))
        };

        if new_index != self.current_index {
            self.previous_index = Some(self.current_index);
            self.current_index = new_index;
            self.transition_id = self.transition_id.wrapping_add(1);
            cx.notify();
        }
    }

    pub fn next(&mut self, infinite: bool, cx: &mut Context<Self>) {
        if self.slide_count == 0 {
            return;
        }

        let next_index = if infinite {
            (self.current_index + 1) % self.slide_count
        } else if self.current_index < self.slide_count - 1 {
            self.current_index + 1
        } else {
            return;
        };

        self.previous_index = Some(self.current_index);
        self.current_index = next_index;
        self.transition_id = self.transition_id.wrapping_add(1);
        cx.notify();
    }

    pub fn prev(&mut self, infinite: bool, cx: &mut Context<Self>) {
        if self.slide_count == 0 {
            return;
        }

        let prev_index = if infinite {
            if self.current_index == 0 {
                self.slide_count - 1
            } else {
                self.current_index - 1
            }
        } else if self.current_index > 0 {
            self.current_index - 1
        } else {
            return;
        };

        self.previous_index = Some(self.current_index);
        self.current_index = prev_index;
        self.transition_id = self.transition_id.wrapping_add(1);
        cx.notify();
    }

    fn can_go_prev(&self, infinite: bool) -> bool {
        if self.slide_count == 0 {
            return false;
        }
        infinite || self.current_index > 0
    }

    fn can_go_next(&self, infinite: bool) -> bool {
        if self.slide_count == 0 {
            return false;
        }
        infinite || self.current_index < self.slide_count - 1
    }
}

impl Focusable for CarouselState {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for CarouselState {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}

pub struct CarouselSlide {
    content: AnyElement,
}

impl CarouselSlide {
    pub fn new(content: impl IntoElement) -> Self {
        Self {
            content: content.into_any_element(),
        }
    }
}

pub use gpui::{bounce, ease_in_out, ease_out_quint, linear, pulsating_between, quadratic};

#[derive(IntoElement)]
pub struct Carousel {
    id: ElementId,
    state: Entity<CarouselState>,
    slides: Vec<CarouselSlide>,
    size: CarouselSize,
    transition: CarouselTransition,
    transition_duration: Duration,
    easing: Rc<dyn Fn(f32) -> f32>,
    infinite: bool,
    auto_play: bool,
    auto_play_interval: Duration,
    show_arrows: bool,
    show_dots: bool,
    disabled: bool,
    on_change: Option<Rc<dyn Fn(usize, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
}

impl Carousel {
    pub fn new(id: impl Into<ElementId>, state: Entity<CarouselState>) -> Self {
        Self {
            id: id.into(),
            state,
            slides: Vec::new(),
            size: CarouselSize::Md,
            transition: CarouselTransition::Slide,
            transition_duration: Duration::from_millis(300),
            easing: Rc::new(ease_in_out),
            infinite: false,
            auto_play: false,
            auto_play_interval: Duration::from_secs(5),
            show_arrows: true,
            show_dots: true,
            disabled: false,
            on_change: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn transition_duration(mut self, duration: Duration) -> Self {
        self.transition_duration = duration;
        self
    }

    pub fn easing(mut self, easing: impl Fn(f32) -> f32 + 'static) -> Self {
        self.easing = Rc::new(easing);
        self
    }

    pub fn slides(mut self, slides: Vec<CarouselSlide>) -> Self {
        self.slides = slides;
        self
    }

    pub fn slide(mut self, slide: CarouselSlide) -> Self {
        self.slides.push(slide);
        self
    }

    pub fn size(mut self, size: CarouselSize) -> Self {
        self.size = size;
        self
    }

    pub fn transition(mut self, transition: CarouselTransition) -> Self {
        self.transition = transition;
        self
    }

    pub fn infinite(mut self, infinite: bool) -> Self {
        self.infinite = infinite;
        self
    }

    pub fn auto_play(mut self, auto_play: bool) -> Self {
        self.auto_play = auto_play;
        self
    }

    pub fn auto_play_interval(mut self, interval: Duration) -> Self {
        self.auto_play_interval = interval;
        self
    }

    pub fn show_arrows(mut self, show: bool) -> Self {
        self.show_arrows = show;
        self
    }

    pub fn show_dots(mut self, show: bool) -> Self {
        self.show_dots = show;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_change(mut self, handler: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }
}

impl Styled for Carousel {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Carousel {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let state = self.state.clone();
        let slide_count = self.slides.len();

        state.update(cx, |s, cx| {
            if s.slide_count != slide_count {
                s.set_slide_count(slide_count, cx);
            }
        });

        let current_index = state.read(cx).current_index();
        let previous_index = state.read(cx).previous_index();
        let transition_id = state.read(cx).transition_id();
        let focus_handle = state.read(cx).focus_handle(cx);
        let is_focused = focus_handle.is_focused(window);

        let can_prev = state.read(cx).can_go_prev(self.infinite);
        let can_next = state.read(cx).can_go_next(self.infinite);

        let arrow_size = self.size.arrow_size();
        let dot_size = self.size.dot_size();
        let icon_size = self.size.icon_size();
        let user_style = self.style.clone();

        if self.auto_play && !self.disabled && slide_count > 1 {
            let state_clone = self.state.clone();
            let infinite = self.infinite;
            let interval = self.auto_play_interval;

            cx.spawn({
                let state_clone = state_clone.clone();
                async move |cx| {
                    cx.background_executor().timer(interval).await;
                    _ = state_clone.update(cx, |s, cx| {
                        s.next(infinite, cx);
                    });
                }
            })
            .detach();
        }

        div()
            .id(self.id.clone())
            .relative()
            .w_full()
            .overflow_hidden()
            .when(!self.disabled, |this| {
                this.track_focus(&focus_handle.clone().tab_index(0).tab_stop(true))
            })
            .when(is_focused && !self.disabled, |this| {
                this.shadow(smallvec::smallvec![theme.tokens.focus_ring_light()])
            })
            .rounded(theme.tokens.radius_lg)
            .bg(theme.tokens.background)
            .map(|this| {
                let mut div = this;
                div.style().refine(&user_style);
                div
            })
            .when(!self.disabled, |this| {
                let state_key = self.state.clone();
                let infinite = self.infinite;
                let on_change_key = self.on_change.clone();

                this.on_key_down(window.listener_for(
                    &state_key,
                    move |state, event: &KeyDownEvent, window, cx| {
                        match event.keystroke.key.as_str() {
                            "left" | "ArrowLeft" => {
                                state.prev(infinite, cx);
                                if let Some(ref handler) = on_change_key {
                                    handler(state.current_index, window, cx);
                                }
                                cx.stop_propagation();
                            }
                            "right" | "ArrowRight" => {
                                state.next(infinite, cx);
                                if let Some(ref handler) = on_change_key {
                                    handler(state.current_index, window, cx);
                                }
                                cx.stop_propagation();
                            }
                            "home" | "Home" => {
                                state.go_to(0, infinite, cx);
                                if let Some(ref handler) = on_change_key {
                                    handler(state.current_index, window, cx);
                                }
                                cx.stop_propagation();
                            }
                            "end" | "End" => {
                                let last = state.slide_count.saturating_sub(1);
                                state.go_to(last, infinite, cx);
                                if let Some(ref handler) = on_change_key {
                                    handler(state.current_index, window, cx);
                                }
                                cx.stop_propagation();
                            }
                            _ => {}
                        }
                    },
                ))
            })
            .child(
                div()
                    .relative()
                    .w_full()
                    .overflow_hidden()
                    .child(match self.transition {
                        CarouselTransition::Slide => {
                            let track_width_pct = slide_count as f32 * 100.0;
                            let slide_width_pct = if slide_count > 0 {
                                100.0 / slide_count as f32
                            } else {
                                100.0
                            };
                            let offset = -(current_index as f32);

                            div()
                                .flex()
                                .w(relative(track_width_pct / 100.0))
                                .left(relative(offset))
                                .children(self.slides.into_iter().map(move |slide| {
                                    div()
                                        .flex_shrink_0()
                                        .w(relative(slide_width_pct / 100.0))
                                        .child(slide.content)
                                }))
                        }
                        CarouselTransition::Fade => {
                            let tid = transition_id as u64;
                            let duration = self.transition_duration;
                            let easing = self.easing.clone();
                            let easing2 = self.easing.clone();

                            div().relative().w_full().children(
                                self.slides
                                    .into_iter()
                                    .enumerate()
                                    .map(move |(idx, slide)| {
                                        let is_current = idx == current_index;
                                        let is_previous = previous_index == Some(idx);
                                        let easing_fn = easing.clone();
                                        let easing_fn2 = easing2.clone();

                                        if is_current {
                                            div()
                                                .relative()
                                                .w_full()
                                                .child(slide.content)
                                                .with_animation(
                                                    ElementId::NamedInteger("fade-in".into(), tid),
                                                    Animation::new(duration)
                                                        .with_easing(move |t| easing_fn(t)),
                                                    |el, delta| el.opacity(delta),
                                                )
                                                .into_any_element()
                                        } else if is_previous {
                                            div()
                                                .absolute()
                                                .top_0()
                                                .left_0()
                                                .w_full()
                                                .child(slide.content)
                                                .with_animation(
                                                    ElementId::NamedInteger("fade-out".into(), tid),
                                                    Animation::new(duration)
                                                        .with_easing(move |t| easing_fn2(t)),
                                                    |el, delta| el.opacity(1.0 - delta),
                                                )
                                                .into_any_element()
                                        } else {
                                            div()
                                                .absolute()
                                                .top_0()
                                                .left_0()
                                                .w_full()
                                                .opacity(0.0)
                                                .child(slide.content)
                                                .into_any_element()
                                        }
                                    }),
                            )
                        }
                    }),
            )
            .when(self.show_arrows && slide_count > 1, |this| {
                let state_prev = self.state.clone();
                let state_next = self.state.clone();
                let infinite = self.infinite;
                let on_change_prev = self.on_change.clone();
                let on_change_next = self.on_change.clone();
                let disabled = self.disabled;

                this.child(
                    div()
                        .id("carousel-prev-btn")
                        .absolute()
                        .left(px(12.0))
                        .top_1_2()
                        .mt(-(arrow_size / 2.0))
                        .size(arrow_size)
                        .flex()
                        .items_center()
                        .justify_center()
                        .rounded_full()
                        .bg(theme.tokens.background.opacity(0.8))
                        .border_1()
                        .border_color(theme.tokens.border)
                        .when(can_prev && !disabled, |this| {
                            this.cursor(CursorStyle::PointingHand)
                                .hover(|style| style.bg(theme.tokens.accent))
                                .on_mouse_down(MouseButton::Left, |_, window, _| {
                                    window.prevent_default();
                                })
                                .on_click(window.listener_for(
                                    &state_prev,
                                    move |state, _, window, cx| {
                                        state.prev(infinite, cx);
                                        if let Some(ref handler) = on_change_prev {
                                            handler(state.current_index, window, cx);
                                        }
                                    },
                                ))
                        })
                        .when(!can_prev || disabled, |this| {
                            this.opacity(0.5).cursor(CursorStyle::Arrow)
                        })
                        .child(
                            svg()
                                .path("assets/icons/chevron-left.svg")
                                .size(icon_size)
                                .text_color(if can_prev && !disabled {
                                    theme.tokens.foreground
                                } else {
                                    theme.tokens.muted_foreground
                                }),
                        ),
                )
                .child(
                    div()
                        .id("carousel-next-btn")
                        .absolute()
                        .right(px(12.0))
                        .top_1_2()
                        .mt(-(arrow_size / 2.0))
                        .size(arrow_size)
                        .flex()
                        .items_center()
                        .justify_center()
                        .rounded_full()
                        .bg(theme.tokens.background.opacity(0.8))
                        .border_1()
                        .border_color(theme.tokens.border)
                        .when(can_next && !disabled, |this| {
                            this.cursor(CursorStyle::PointingHand)
                                .hover(|style| style.bg(theme.tokens.accent))
                                .on_mouse_down(MouseButton::Left, |_, window, _| {
                                    window.prevent_default();
                                })
                                .on_click(window.listener_for(
                                    &state_next,
                                    move |state, _, window, cx| {
                                        state.next(infinite, cx);
                                        if let Some(ref handler) = on_change_next {
                                            handler(state.current_index, window, cx);
                                        }
                                    },
                                ))
                        })
                        .when(!can_next || disabled, |this| {
                            this.opacity(0.5).cursor(CursorStyle::Arrow)
                        })
                        .child(
                            svg()
                                .path("assets/icons/chevron-right.svg")
                                .size(icon_size)
                                .text_color(if can_next && !disabled {
                                    theme.tokens.foreground
                                } else {
                                    theme.tokens.muted_foreground
                                }),
                        ),
                )
            })
            .when(self.show_dots && slide_count > 1, |this| {
                let state_dots = self.state.clone();
                let infinite = self.infinite;
                let on_change_dots = self.on_change.clone();
                let disabled = self.disabled;

                this.child(
                    div()
                        .absolute()
                        .bottom(px(12.0))
                        .left_0()
                        .right_0()
                        .flex()
                        .justify_center()
                        .gap(px(6.0))
                        .children((0..slide_count).map(|idx| {
                            let is_active = idx == current_index;
                            let state_dot = state_dots.clone();
                            let on_change_dot = on_change_dots.clone();

                            div()
                                .id(("carousel-dot", idx))
                                .h(dot_size)
                                .w(if is_active { dot_size * 2.0 } else { dot_size })
                                .rounded_full()
                                .bg(if is_active {
                                    theme.tokens.primary
                                } else {
                                    theme.tokens.muted
                                })
                                .when(!disabled, |this| {
                                    this.cursor(CursorStyle::PointingHand)
                                        .hover(|style| style.opacity(0.8))
                                        .on_mouse_down(MouseButton::Left, |_, window, _| {
                                            window.prevent_default();
                                        })
                                        .on_click(window.listener_for(
                                            &state_dot,
                                            move |state, _, window, cx| {
                                                state.go_to(idx, infinite, cx);
                                                if let Some(ref handler) = on_change_dot {
                                                    handler(state.current_index, window, cx);
                                                }
                                            },
                                        ))
                                })
                        })),
                )
            })
    }
}
