use adabraka_ui::{
    animations::easings,
    components::{
        button::{Button, ButtonVariant},
        icon_button::IconButton,
        input::{Input, InputState},
        scrollable::scrollable_vertical,
    },
    content_transition::{ContentTransition, ContentTransitionState},
    layout::{HStack, ScrollContainer, ScrollDirection, VStack},
    overlays::{
        popover::{Popover, PopoverContent},
        toast::{ToastItem, ToastManager, ToastVariant},
    },
    prelude::*,
    responsive::{current_breakpoint, responsive_value, Breakpoint},
};
use gpui::*;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

struct Assets {
    base: PathBuf,
}

impl gpui::AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<std::borrow::Cow<'static, [u8]>>> {
        std::fs::read(self.base.join(path))
            .map(|data| Some(std::borrow::Cow::Owned(data)))
            .map_err(|err| err.into())
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        std::fs::read_dir(self.base.join(path))
            .map(|entries| {
                entries
                    .filter_map(|entry| {
                        entry
                            .ok()
                            .and_then(|e| e.file_name().into_string().ok())
                            .map(SharedString::from)
                    })
                    .collect()
            })
            .map_err(|err| err.into())
    }
}

actions!(polish_v2_demo, [Quit]);

static TOAST_COUNTER: AtomicU64 = AtomicU64::new(1);

struct PolishV2Demo {
    input_state: Entity<InputState>,
    toast_manager: Entity<ToastManager>,
    content_key: u64,
    transition_state: ContentTransitionState,
}

impl PolishV2Demo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            input_state: cx.new(|cx| InputState::new(cx)),
            toast_manager: cx.new(|cx| ToastManager::new(cx)),
            content_key: 0,
            transition_state: ContentTransitionState::new(),
        }
    }

    fn section_header(title: &str, subtitle: &str) -> impl IntoElement {
        VStack::new()
            .gap(px(4.0))
            .child(
                div()
                    .text_size(px(20.0))
                    .font_weight(FontWeight::BOLD)
                    .child(title.to_string()),
            )
            .child(
                div()
                    .text_size(px(13.0))
                    .text_color(hsla(0.0, 0.0, 0.5, 1.0))
                    .child(subtitle.to_string()),
            )
    }

    fn render_ripple_buttons(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        VStack::new()
            .gap(px(16.0))
            .child(Self::section_header(
                "1. Ripple Buttons",
                "Material-style ripple effect on click via .ripple(true)",
            ))
            .child(
                HStack::new()
                    .gap(px(12.0))
                    .flex_wrap()
                    .items_center()
                    .child(
                        Button::new("ripple-default", "Default Ripple")
                            .ripple(true)
                            .on_click(|_, _, _| {}),
                    )
                    .child(
                        Button::new("ripple-secondary", "Secondary Ripple")
                            .variant(ButtonVariant::Secondary)
                            .ripple(true)
                            .on_click(|_, _, _| {}),
                    )
                    .child(
                        Button::new("ripple-destructive", "Destructive Ripple")
                            .variant(ButtonVariant::Destructive)
                            .ripple(true)
                            .on_click(|_, _, _| {}),
                    )
                    .child(
                        Button::new("ripple-outline", "Outline Ripple")
                            .variant(ButtonVariant::Outline)
                            .ripple(true)
                            .on_click(|_, _, _| {}),
                    )
                    .child(
                        Button::new("ripple-ghost", "Ghost Ripple")
                            .variant(ButtonVariant::Ghost)
                            .ripple(true)
                            .on_click(|_, _, _| {}),
                    ),
            )
            .child(
                HStack::new()
                    .gap(px(12.0))
                    .items_center()
                    .child(div().text_size(px(13.0)).child("IconButtons with ripple:"))
                    .child(
                        IconButton::new("chevron-right")
                            .ripple(true)
                            .on_click(|_, _, _| {}),
                    )
                    .child(IconButton::new("plus").ripple(true).on_click(|_, _, _| {}))
                    .child(
                        IconButton::new("settings")
                            .ripple(true)
                            .on_click(|_, _, _| {}),
                    ),
            )
    }

    fn render_input_shake(&self, cx: &mut Context<Self>) -> impl IntoElement {
        VStack::new()
            .gap(px(16.0))
            .child(Self::section_header(
                "2. Input Shake on Validation",
                "Trigger a shake animation when input validation fails",
            ))
            .child(
                VStack::new()
                    .gap(px(12.0))
                    .child(
                        Input::new(&self.input_state)
                            .placeholder("Type something, then click Validate empty...")
                            .w(px(400.0)),
                    )
                    .child(
                        HStack::new()
                            .gap(px(8.0))
                            .child(
                                Button::new("shake-btn", "Validate (shake if empty)")
                                    .variant(ButtonVariant::Destructive)
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.input_state.update(cx, |state, _| {
                                            if state.content().is_empty() {
                                                state.trigger_shake();
                                            }
                                        });
                                    })),
                            )
                            .child(
                                Button::new("shake-force", "Force Shake")
                                    .variant(ButtonVariant::Outline)
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.input_state.update(cx, |state, _| {
                                            state.trigger_shake();
                                        });
                                    })),
                            ),
                    ),
            )
    }

    fn render_toast_animations(&self, cx: &mut Context<Self>) -> impl IntoElement {
        VStack::new()
            .gap(px(16.0))
            .child(Self::section_header(
                "3. Toast Entry & Exit Animations",
                "Toasts slide in and fade out with smooth easing transitions",
            ))
            .child(
                HStack::new()
                    .gap(px(8.0))
                    .flex_wrap()
                    .child(
                        Button::new("toast-success", "Success Toast")
                            .variant(ButtonVariant::Default)
                            .on_click(cx.listener(|this, _, window, cx| {
                                let id = TOAST_COUNTER.fetch_add(1, Ordering::SeqCst);
                                let toast = ToastItem::new(id, "Operation Successful")
                                    .description("The action completed with entry animation")
                                    .variant(ToastVariant::Success)
                                    .duration(Duration::from_secs(3));
                                this.toast_manager.update(cx, |mgr, cx| {
                                    mgr.add_toast(toast, window, cx);
                                });
                            })),
                    )
                    .child(
                        Button::new("toast-error", "Error Toast")
                            .variant(ButtonVariant::Destructive)
                            .on_click(cx.listener(|this, _, window, cx| {
                                let id = TOAST_COUNTER.fetch_add(1, Ordering::SeqCst);
                                let toast = ToastItem::new(id, "Something Went Wrong")
                                    .description("Watch for the exit animation after 3s")
                                    .variant(ToastVariant::Error)
                                    .duration(Duration::from_secs(3));
                                this.toast_manager.update(cx, |mgr, cx| {
                                    mgr.add_toast(toast, window, cx);
                                });
                            })),
                    )
                    .child(
                        Button::new("toast-warning", "Warning Toast")
                            .variant(ButtonVariant::Secondary)
                            .on_click(cx.listener(|this, _, window, cx| {
                                let id = TOAST_COUNTER.fetch_add(1, Ordering::SeqCst);
                                let toast = ToastItem::new(id, "Warning Notice")
                                    .description("Animated entry and auto-dismiss")
                                    .variant(ToastVariant::Warning)
                                    .duration(Duration::from_secs(3));
                                this.toast_manager.update(cx, |mgr, cx| {
                                    mgr.add_toast(toast, window, cx);
                                });
                            })),
                    )
                    .child(
                        Button::new("toast-clear", "Clear All")
                            .variant(ButtonVariant::Outline)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.toast_manager.update(cx, |mgr, cx| {
                                    mgr.clear_all(cx);
                                });
                            })),
                    ),
            )
    }

    fn render_popover_animations(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        VStack::new()
            .gap(px(16.0))
            .child(Self::section_header(
                "4. Popover Enter/Exit Animations",
                "Popovers fade and slide in on open, reverse on close (press Escape)",
            ))
            .child(
                HStack::new()
                    .gap(px(12.0))
                    .child(
                        Popover::new("popover-demo-1")
                            .trigger(
                                Button::new("pop-trigger-1", "Open Popover")
                                    .variant(ButtonVariant::Outline),
                            )
                            .content(|window, cx| {
                                cx.new(|cx| {
                                    PopoverContent::new(window, cx, |_, _| {
                                        VStack::new()
                                            .gap(px(8.0))
                                            .child(
                                                div()
                                                    .text_size(px(14.0))
                                                    .font_weight(FontWeight::SEMIBOLD)
                                                    .child("Animated Popover"),
                                            )
                                            .child(
                                                div()
                                                    .text_size(px(13.0))
                                                    .child("Notice the smooth fade + slide entry animation."),
                                            )
                                            .child(
                                                div()
                                                    .text_size(px(13.0))
                                                    .child("Press Escape to see the exit animation."),
                                            )
                                            .into_any_element()
                                    })
                                })
                            }),
                    )
                    .child(
                        Popover::new("popover-demo-2")
                            .anchor(Corner::TopRight)
                            .trigger(
                                Button::new("pop-trigger-2", "Popover (Top Right)")
                                    .variant(ButtonVariant::Secondary),
                            )
                            .content(|window, cx| {
                                cx.new(|cx| {
                                    PopoverContent::new(window, cx, |_, _| {
                                        VStack::new()
                                            .gap(px(8.0))
                                            .p(px(4.0))
                                            .child(
                                                div()
                                                    .text_size(px(14.0))
                                                    .font_weight(FontWeight::SEMIBOLD)
                                                    .child("Anchored to Top Right"),
                                            )
                                            .child(
                                                div()
                                                    .text_size(px(13.0))
                                                    .child("Entry and exit animations work from any anchor position."),
                                            )
                                            .into_any_element()
                                    })
                                })
                            }),
                    ),
            )
    }

    fn render_easing_showcase(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();
        let easings_list: Vec<(&str, fn(f32) -> f32)> = vec![
            ("ease_out_cubic", easings::ease_out_cubic),
            ("ease_out_quart", easings::ease_out_quart),
            ("ease_out_expo", easings::ease_out_expo),
            ("ease_out_back", easings::ease_out_back),
            ("spring", easings::spring),
            ("smooth_spring", easings::smooth_spring),
            ("ease_out_circ", easings::ease_out_circ),
            ("ease_out_elastic", easings::ease_out_elastic),
        ];

        let mut easing_rows = VStack::new().gap(px(8.0));
        for (name, easing_fn) in easings_list {
            easing_rows = easing_rows.child(
                HStack::new()
                    .gap(px(12.0))
                    .items_center()
                    .child(
                        div()
                            .w(px(140.0))
                            .text_size(px(12.0))
                            .font_family("monospace")
                            .child(name.to_string()),
                    )
                    .child(
                        div()
                            .w(px(300.0))
                            .h(px(28.0))
                            .rounded(px(4.0))
                            .bg(theme.tokens.muted)
                            .overflow_hidden()
                            .child(
                                div()
                                    .id(ElementId::Name(format!("easing-{}", name).into()))
                                    .h_full()
                                    .rounded(px(4.0))
                                    .bg(theme.tokens.primary)
                                    .with_animation(
                                        ElementId::Name(format!("easing-anim-{}", name).into()),
                                        Animation::new(Duration::from_millis(800))
                                            .with_easing(easing_fn),
                                        |el, delta| el.w(px(300.0 * delta)),
                                    ),
                            ),
                    ),
            );
        }

        VStack::new()
            .gap(px(16.0))
            .child(Self::section_header(
                "5. Easing Functions Showcase",
                "Side-by-side comparison of available easing curves",
            ))
            .child(easing_rows)
    }

    fn render_content_transition(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();
        let key = self.content_key;

        let current_content = match key % 3 {
            0 => div()
                .p(px(20.0))
                .bg(rgb(0x3b82f6))
                .rounded(px(8.0))
                .child(
                    div()
                        .text_size(px(16.0))
                        .text_color(gpui::white())
                        .font_weight(FontWeight::SEMIBOLD)
                        .child("State A - Blue"),
                )
                .child(
                    div()
                        .text_size(px(13.0))
                        .text_color(gpui::white())
                        .child("This content crossfades when toggled"),
                ),
            1 => div()
                .p(px(20.0))
                .bg(rgb(0x10b981))
                .rounded(px(8.0))
                .child(
                    div()
                        .text_size(px(16.0))
                        .text_color(gpui::white())
                        .font_weight(FontWeight::SEMIBOLD)
                        .child("State B - Green"),
                )
                .child(
                    div()
                        .text_size(px(13.0))
                        .text_color(gpui::white())
                        .child("Smooth crossfade transition between states"),
                ),
            _ => div()
                .p(px(20.0))
                .bg(rgb(0x8b5cf6))
                .rounded(px(8.0))
                .child(
                    div()
                        .text_size(px(16.0))
                        .text_color(gpui::white())
                        .font_weight(FontWeight::SEMIBOLD)
                        .child("State C - Purple"),
                )
                .child(
                    div()
                        .text_size(px(13.0))
                        .text_color(gpui::white())
                        .child("ContentTransition handles the fade automatically"),
                ),
        };

        let transition = if key > 0 {
            ContentTransition::new("content-transition-demo", current_content)
                .duration(Duration::from_millis(300))
                .crossfade_from(
                    div()
                        .p(px(20.0))
                        .rounded(px(8.0))
                        .bg(theme.tokens.muted)
                        .child(div().text_size(px(13.0)).child("...")),
                )
        } else {
            ContentTransition::new("content-transition-demo", current_content)
        };

        VStack::new()
            .gap(px(16.0))
            .child(Self::section_header(
                "6. Content Transition (Crossfade)",
                "Smooth crossfade between different content states",
            ))
            .child(
                Button::new("toggle-content", "Toggle Content State")
                    .variant(ButtonVariant::Default)
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.content_key += 1;
                        this.transition_state.set_key(this.content_key);
                        cx.notify();
                    })),
            )
            .child(div().w(px(400.0)).h(px(90.0)).child(transition))
    }

    fn render_responsive_info(&self, window: &Window) -> impl IntoElement {
        let breakpoint = current_breakpoint(window);
        let columns: usize = responsive_value(window, 1, 2, 3, 4);
        let bp_label = match breakpoint {
            Breakpoint::Xs => "Xs (< 640px)",
            Breakpoint::Sm => "Sm (640px+)",
            Breakpoint::Md => "Md (768px+)",
            Breakpoint::Lg => "Lg (1024px+)",
            Breakpoint::Xl => "Xl (1280px+)",
            Breakpoint::Xxl => "Xxl (1536px+)",
        };

        let theme = use_theme();

        VStack::new()
            .gap(px(16.0))
            .child(Self::section_header(
                "7. Responsive Breakpoints",
                "Resize the window to see breakpoint changes",
            ))
            .child(
                HStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .p(px(16.0))
                            .bg(theme.tokens.card)
                            .border_1()
                            .border_color(theme.tokens.border)
                            .rounded(px(8.0))
                            .child(
                                VStack::new()
                                    .gap(px(4.0))
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("Current Breakpoint"),
                                    )
                                    .child(
                                        div()
                                            .text_size(px(18.0))
                                            .font_weight(FontWeight::BOLD)
                                            .child(bp_label.to_string()),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .p(px(16.0))
                            .bg(theme.tokens.card)
                            .border_1()
                            .border_color(theme.tokens.border)
                            .rounded(px(8.0))
                            .child(
                                VStack::new()
                                    .gap(px(4.0))
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("Responsive Columns"),
                                    )
                                    .child(
                                        div()
                                            .text_size(px(18.0))
                                            .font_weight(FontWeight::BOLD)
                                            .child(format!("{} columns", columns)),
                                    ),
                            ),
                    ),
            )
    }

    fn render_elevation_shadows(&self) -> impl IntoElement {
        let theme = use_theme();

        let mut cards = HStack::new().gap(px(16.0)).flex_wrap();

        for level in 0u8..=5 {
            let shadows = theme.tokens.elevation_shadow(level);

            cards = cards.child(
                div()
                    .w(px(120.0))
                    .h(px(80.0))
                    .bg(theme.tokens.card)
                    .border_1()
                    .border_color(theme.tokens.border)
                    .rounded(px(8.0))
                    .shadow(shadows.into())
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .text_size(px(13.0))
                            .text_color(theme.tokens.foreground)
                            .child(format!("Level {}", level)),
                    ),
            );
        }

        VStack::new()
            .gap(px(16.0))
            .child(Self::section_header(
                "8. Elevation Shadows",
                "theme.tokens.elevation_shadow(level) provides layered shadow depth",
            ))
            .child(cards)
    }

    fn render_gradient_presets(&self) -> impl IntoElement {
        let theme = use_theme();

        VStack::new()
            .gap(px(16.0))
            .child(Self::section_header(
                "9. Gradient Presets & Glow Shadows",
                "Built-in gradient_primary/surface/accent/destructive and glow_shadow",
            ))
            .child(
                HStack::new()
                    .gap(px(12.0))
                    .flex_wrap()
                    .child(
                        div()
                            .w(px(140.0))
                            .h(px(70.0))
                            .rounded(px(8.0))
                            .bg(theme.tokens.gradient_primary())
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .text_color(theme.tokens.primary_foreground)
                                    .child("gradient_primary"),
                            ),
                    )
                    .child(
                        div()
                            .w(px(140.0))
                            .h(px(70.0))
                            .rounded(px(8.0))
                            .bg(theme.tokens.gradient_surface())
                            .border_1()
                            .border_color(theme.tokens.border)
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .text_color(theme.tokens.foreground)
                                    .child("gradient_surface"),
                            ),
                    )
                    .child(
                        div()
                            .w(px(140.0))
                            .h(px(70.0))
                            .rounded(px(8.0))
                            .bg(theme.tokens.gradient_accent())
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .text_color(theme.tokens.accent_foreground)
                                    .child("gradient_accent"),
                            ),
                    )
                    .child(
                        div()
                            .w(px(140.0))
                            .h(px(70.0))
                            .rounded(px(8.0))
                            .bg(theme.tokens.gradient_destructive())
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .text_color(theme.tokens.destructive_foreground)
                                    .child("gradient_destructive"),
                            ),
                    ),
            )
            .child(
                HStack::new()
                    .gap(px(12.0))
                    .child(
                        div()
                            .w(px(140.0))
                            .h(px(70.0))
                            .rounded(px(8.0))
                            .bg(theme.tokens.primary)
                            .shadow(smallvec::smallvec![theme
                                .tokens
                                .glow_shadow(theme.tokens.primary, 1.0)])
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .text_color(theme.tokens.primary_foreground)
                                    .child("glow (1.0)"),
                            ),
                    )
                    .child(
                        div()
                            .w(px(140.0))
                            .h(px(70.0))
                            .rounded(px(8.0))
                            .bg(rgb(0x3b82f6))
                            .shadow(smallvec::smallvec![theme
                                .tokens
                                .glow_shadow(hsla(217.0 / 360.0, 0.91, 0.60, 1.0), 1.5)])
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .text_color(gpui::white())
                                    .child("blue glow (1.5)"),
                            ),
                    )
                    .child(
                        div()
                            .w(px(140.0))
                            .h(px(70.0))
                            .rounded(px(8.0))
                            .bg(theme.tokens.destructive)
                            .shadow(smallvec::smallvec![theme
                                .tokens
                                .glow_shadow(theme.tokens.destructive, 2.0)])
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .text_color(theme.tokens.destructive_foreground)
                                    .child("red glow (2.0)"),
                            ),
                    ),
            )
    }

    fn render_focus_ring_animated(&self) -> impl IntoElement {
        let theme = use_theme();

        VStack::new()
            .gap(px(16.0))
            .child(Self::section_header(
                "10. Animated Focus Ring",
                "focus_ring_animated(progress) grows the ring as the animation progresses",
            ))
            .child(
                HStack::new()
                    .gap(px(24.0))
                    .child(
                        div()
                            .id("focus-ring-demo")
                            .w(px(120.0))
                            .h(px(48.0))
                            .rounded(px(8.0))
                            .bg(theme.tokens.card)
                            .border_1()
                            .border_color(theme.tokens.border)
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(div().text_size(px(12.0)).child("Animated Ring"))
                            .with_animation(
                                "focus-ring-anim",
                                Animation::new(Duration::from_millis(600))
                                    .with_easing(easings::ease_out_cubic),
                                {
                                    let tokens = theme.tokens.clone();
                                    move |el, delta| {
                                        let ring = tokens.focus_ring_animated(delta);
                                        el.shadow(smallvec::smallvec![ring])
                                    }
                                },
                            ),
                    )
                    .child(
                        div()
                            .id("colored-shadow-demo")
                            .w(px(120.0))
                            .h(px(48.0))
                            .rounded(px(8.0))
                            .bg(rgb(0x8b5cf6))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .text_color(gpui::white())
                                    .child("Colored Shadow"),
                            )
                            .with_animation(
                                "colored-shadow-anim",
                                Animation::new(Duration::from_millis(600))
                                    .with_easing(easings::ease_out_cubic),
                                {
                                    let tokens = theme.tokens.clone();
                                    move |el, delta| {
                                        let shadow = tokens.colored_shadow(
                                            hsla(262.0 / 360.0, 0.83, 0.58, 1.0),
                                            delta * 2.0,
                                        );
                                        el.shadow(smallvec::smallvec![shadow])
                                    }
                                },
                            ),
                    ),
            )
    }

    fn render_momentum_scroll(&self) -> impl IntoElement {
        let theme = use_theme();

        let mut scroll_content = VStack::new().gap(px(4.0)).p(px(8.0));
        for i in 0..50 {
            scroll_content = scroll_content.child(
                div()
                    .px(px(12.0))
                    .py(px(8.0))
                    .bg(if i % 2 == 0 {
                        theme.tokens.card
                    } else {
                        theme.tokens.muted
                    })
                    .rounded(px(4.0))
                    .child(
                        div()
                            .text_size(px(13.0))
                            .child(format!("Scroll item {} - flick to see momentum", i + 1)),
                    ),
            );
        }

        VStack::new()
            .gap(px(16.0))
            .child(Self::section_header(
                "11. Momentum Scroll",
                "ScrollContainer with .momentum(true) for physics-based inertia scrolling",
            ))
            .child(
                ScrollContainer::new(ScrollDirection::Vertical)
                    .momentum(true)
                    .h(px(250.0))
                    .w_full()
                    .rounded(px(8.0))
                    .border_1()
                    .border_color(theme.tokens.border)
                    .child(scroll_content),
            )
    }

    fn render_spring_presets(&self) -> impl IntoElement {
        let theme = use_theme();
        let presets: Vec<(&str, &str)> = vec![
            ("gentle", "stiffness: 120, damping: 14, mass: 1.0"),
            ("wobbly", "stiffness: 180, damping: 12, mass: 1.0"),
            ("stiff", "stiffness: 210, damping: 20, mass: 1.0"),
            ("slow", "stiffness: 280, damping: 60, mass: 1.0"),
            ("snappy", "stiffness: 400, damping: 30, mass: 1.0"),
        ];

        let mut rows = VStack::new().gap(px(8.0));
        for (name, params) in presets {
            rows = rows.child(
                HStack::new()
                    .gap(px(12.0))
                    .items_center()
                    .child(
                        div()
                            .w(px(80.0))
                            .text_size(px(12.0))
                            .font_family("monospace")
                            .child(name.to_string()),
                    )
                    .child(
                        div()
                            .flex_1()
                            .h(px(32.0))
                            .rounded(px(6.0))
                            .bg(theme.tokens.muted)
                            .px(px(8.0))
                            .flex()
                            .items_center()
                            .child(
                                div()
                                    .text_size(px(11.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child(params.to_string()),
                            ),
                    ),
            );
        }

        VStack::new()
            .gap(px(16.0))
            .child(Self::section_header(
                "12. Spring Physics Presets",
                "Pre-configured Spring presets for programmatic animations",
            ))
            .child(rows)
            .child(
                div()
                    .p(px(12.0))
                    .bg(theme.tokens.accent)
                    .rounded(px(6.0))
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(theme.tokens.accent_foreground)
                            .child("Spring physics drive momentum scrolling, popover animations, and can be used for custom programmatic animations via Spring::tick()"),
                    ),
            )
    }
}

impl Render for PolishV2Demo {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        div()
            .size_full()
            .bg(theme.tokens.background)
            .text_color(theme.tokens.foreground)
            .overflow_hidden()
            .child(
                scrollable_vertical(
                    div()
                        .flex()
                        .flex_col()
                        .p(px(32.0))
                        .gap(px(40.0))
                        .child(
                            VStack::new()
                                .gap(px(8.0))
                                .child(
                                    div()
                                        .text_size(px(28.0))
                                        .font_weight(FontWeight::BOLD)
                                        .child("Polish v2 Demo"),
                                )
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child("Comprehensive showcase of all v2 polish improvements: ripples, shake validation, overlay animations, easings, content transitions, responsive breakpoints, elevation shadows, gradients, glow effects, focus rings, momentum scroll, and spring physics"),
                                ),
                        )
                        .child(self.render_ripple_buttons(cx))
                        .child(render_separator(&theme))
                        .child(self.render_input_shake(cx))
                        .child(render_separator(&theme))
                        .child(self.render_toast_animations(cx))
                        .child(render_separator(&theme))
                        .child(self.render_popover_animations(cx))
                        .child(render_separator(&theme))
                        .child(self.render_easing_showcase(cx))
                        .child(render_separator(&theme))
                        .child(self.render_content_transition(cx))
                        .child(render_separator(&theme))
                        .child(self.render_responsive_info(window))
                        .child(render_separator(&theme))
                        .child(self.render_elevation_shadows())
                        .child(render_separator(&theme))
                        .child(self.render_gradient_presets())
                        .child(render_separator(&theme))
                        .child(self.render_focus_ring_animated())
                        .child(render_separator(&theme))
                        .child(self.render_momentum_scroll())
                        .child(render_separator(&theme))
                        .child(self.render_spring_presets())
                        .child(div().h(px(60.0))),
                ),
            )
            .child(self.toast_manager.clone())
    }
}

fn render_separator(theme: &Theme) -> impl IntoElement {
    div().w_full().h(px(1.0)).bg(theme.tokens.border)
}

fn main() {
    Application::new()
        .with_assets(Assets {
            base: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
        })
        .run(|cx: &mut App| {
            adabraka_ui::init(cx);
            adabraka_ui::set_icon_base_path("assets/icons");
            install_theme(cx, Theme::dark());

            cx.on_action(|_: &Quit, cx| cx.quit());
            cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);

            let bounds = Bounds::centered(None, size(px(1100.0), px(900.0)), cx);
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    titlebar: Some(TitlebarOptions {
                        title: Some("Polish v2 Demo".into()),
                        appears_transparent: false,
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                |_window, cx| cx.new(|cx| PolishV2Demo::new(cx)),
            )
            .unwrap();

            cx.activate(true);
        });
}
