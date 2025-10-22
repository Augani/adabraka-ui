use gpui::{prelude::FluentBuilder as _, *};
use std::time::Duration;
use adabraka_ui::{
    animations::{self, presets, durations, pulse_scale, pulse_opacity, shake_offset},
    theme::{install_theme, Theme, use_theme},
    components::{
        button::{Button, ButtonVariant},
        scrollable::scrollable_vertical,
        text::{h1, h2, body, caption, muted},
    },
};

struct AnimationsDemo {
    show_fade: bool,
    show_scale: bool,
    show_slide: bool,
    show_shake: bool,
}

impl AnimationsDemo {
    fn new() -> Self {
        Self {
            show_fade: true,
            show_scale: true,
            show_slide: true,
            show_shake: false,
        }
    }
}

impl Render for AnimationsDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(theme.tokens.background)
            .child(
                // Header
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .p(px(24.0))
                    .border_b_1()
                    .border_color(theme.tokens.border)
                    .child(h1("Animation System"))
                    .child(caption("Smooth, polished animations with professional easing functions"))
            )
            .child(
                // Properly constrained scroll container
                div()
                    .flex_1()
                    .flex()
                    .overflow_hidden()  // Important for proper scroll containment
                    .child(
                        scrollable_vertical(
                            div()
                                .flex()
                                .flex_col()
                                .gap(px(32.0))
                                .p(px(24.0))
                    // Fade Animations
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(h2("Fade Animations"))
                            .child(body("Smooth cubic easing for natural fade effects"))
                            .child(
                                div()
                                    .flex()
                                    .gap(px(16.0))
                                    .items_center()
                                    .child(
                                        Button::new("toggle-fade-btn", "Toggle Fade")
                                            .variant(ButtonVariant::Outline)
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.show_fade = !this.show_fade;
                                                cx.notify();
                                            }))
                                    )
                                    .when(self.show_fade, |this| {
                                        this.child(
                                            div()
                                                .flex()
                                                .gap(px(12.0))
                                                .child(
                                                    div()
                                                        .id("fade-box-1")
                                                        .size(px(80.0))
                                                        .rounded(theme.tokens.radius_md)
                                                        .bg(theme.tokens.primary)
                                                        .flex()
                                                        .items_center()
                                                        .justify_center()
                                                        .text_color(theme.tokens.primary_foreground)
                                                        .font_family(theme.tokens.font_family.clone())
                                                        .child("Fast")
                                                        .with_animation(
                                                            "fade-1",
                                                            presets::fade_in_quick(),
                                                            |div, delta| div.opacity(delta)
                                                        )
                                                )
                                                .child(
                                                    div()
                                                        .id("fade-box-2")
                                                        .size(px(80.0))
                                                        .rounded(theme.tokens.radius_md)
                                                        .bg(theme.tokens.secondary)
                                                        .flex()
                                                        .items_center()
                                                        .justify_center()
                                                        .font_family(theme.tokens.font_family.clone())
                                                        .child("Normal")
                                                        .with_animation(
                                                            "fade-2",
                                                            presets::fade_in_normal(),
                                                            |div, delta| div.opacity(delta)
                                                        )
                                                )
                                                .child(
                                                    div()
                                                        .id("fade-box-3")
                                                        .size(px(80.0))
                                                        .rounded(theme.tokens.radius_md)
                                                        .bg(theme.tokens.accent)
                                                        .flex()
                                                        .items_center()
                                                        .justify_center()
                                                        .font_family(theme.tokens.font_family.clone())
                                                        .child("Slow")
                                                        .with_animation(
                                                            "fade-3",
                                                            presets::fade_in_slow(),
                                                            |div, delta| div.opacity(delta)
                                                        )
                                                )
                                        )
                                    })
                            )
                    )
                    // Scale Animations
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(h2("Scale Animations"))
                            .child(body("Back easing with subtle overshoot for emphasis"))
                            .child(
                                div()
                                    .flex()
                                    .gap(px(16.0))
                                    .items_center()
                                    .child(
                                        Button::new("toggle-scale-btn", "Toggle Scale")
                                            .variant(ButtonVariant::Outline)
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.show_scale = !this.show_scale;
                                                cx.notify();
                                            }))
                                    )
                                    .when(self.show_scale, |this| {
                                        this.child(
                                            div()
                                                .flex()
                                                .gap(px(12.0))
                                                .child(
                                                    div()
                                                        .id("scale-box-1")
                                                        .size(px(80.0))
                                                        .rounded(theme.tokens.radius_md)
                                                        .bg(theme.tokens.accent)
                                                        .flex()
                                                        .items_center()
                                                        .justify_center()
                                                        .font_family(theme.tokens.font_family.clone())
                                                        .child("Overshoot")
                                                        .with_animation(
                                                            "scale-1",
                                                            presets::scale_up(),
                                                            |div, delta| {
                                                                let scale = 0.5 + (0.5 * delta);
                                                                div.size(px(80.0 * scale))
                                                            }
                                                        )
                                                )
                                                .child(
                                                    div()
                                                        .id("scale-box-2")
                                                        .size(px(80.0))
                                                        .rounded(theme.tokens.radius_md)
                                                        .bg(theme.tokens.destructive)
                                                        .text_color(theme.tokens.destructive_foreground)
                                                        .flex()
                                                        .items_center()
                                                        .justify_center()
                                                        .font_family(theme.tokens.font_family.clone())
                                                        .child("Smooth")
                                                        .with_animation(
                                                            "scale-2",
                                                            presets::scale_up_smooth(),
                                                            |div, delta| {
                                                                let scale = 0.5 + (0.5 * delta);
                                                                div.size(px(80.0 * scale))
                                                            }
                                                        )
                                                )
                                                .child(
                                                    div()
                                                        .id("scale-box-3")
                                                        .size(px(80.0))
                                                        .rounded(theme.tokens.radius_md)
                                                        .bg(rgb(0x10b981))
                                                        .text_color(gpui::white())
                                                        .flex()
                                                        .items_center()
                                                        .justify_center()
                                                        .font_family(theme.tokens.font_family.clone())
                                                        .child("Spring")
                                                        .with_animation(
                                                            "scale-3",
                                                            presets::bounce_in(),
                                                            |div, delta| {
                                                                let scale = 0.3 + (0.7 * delta);
                                                                div.size(px(80.0 * scale))
                                                            }
                                                        )
                                                )
                                        )
                                    })
                            )
                    )
                    // Slide Animations
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(h2("Slide Animations"))
                            .child(body("Cubic easing for smooth, natural sliding motion"))
                            .child(
                                div()
                                    .flex()
                                    .gap(px(16.0))
                                    .items_center()
                                    .child(
                                        Button::new("toggle-slide-btn", "Toggle Slide")
                                            .variant(ButtonVariant::Outline)
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.show_slide = !this.show_slide;
                                                cx.notify();
                                            }))
                                    )
                                    .when(self.show_slide, |this| {
                                        this.child(
                                            div()
                                                .flex()
                                                .gap(px(12.0))
                                                .child(
                                                    div()
                                                        .id("slide-left")
                                                        .w(px(100.0))
                                                        .h(px(80.0))
                                                        .rounded(theme.tokens.radius_md)
                                                        .bg(rgb(0x10b981))
                                                        .flex()
                                                        .items_center()
                                                        .justify_center()
                                                        .text_color(gpui::white())
                                                        .font_family(theme.tokens.font_family.clone())
                                                        .child("← Left")
                                                        .with_animation(
                                                            "slide-left-anim",
                                                            presets::slide_in_left(),
                                                            |div, delta| div.ml(px(-200.0 * (1.0 - delta)))
                                                        )
                                                )
                                                .child(
                                                    div()
                                                        .id("slide-right")
                                                        .w(px(100.0))
                                                        .h(px(80.0))
                                                        .rounded(theme.tokens.radius_md)
                                                        .bg(rgb(0x3b82f6))
                                                        .flex()
                                                        .items_center()
                                                        .justify_center()
                                                        .text_color(gpui::white())
                                                        .font_family(theme.tokens.font_family.clone())
                                                        .child("Right →")
                                                        .with_animation(
                                                            "slide-right-anim",
                                                            presets::slide_in_right(),
                                                            |div, delta| div.ml(px(200.0 * (1.0 - delta)))
                                                        )
                                                )
                                                .child(
                                                    div()
                                                        .id("slide-spring")
                                                        .w(px(100.0))
                                                        .h(px(80.0))
                                                        .rounded(theme.tokens.radius_md)
                                                        .bg(rgb(0xf59e0b))
                                                        .flex()
                                                        .items_center()
                                                        .justify_center()
                                                        .text_color(gpui::white())
                                                        .font_family(theme.tokens.font_family.clone())
                                                        .child("Spring →")
                                                        .with_animation(
                                                            "slide-spring-anim",
                                                            presets::spring_slide_right(),
                                                            |div, delta| div.ml(px(200.0 * (1.0 - delta)))
                                                        )
                                                )
                                        )
                                    })
                            )
                    )
                    // Continuous Animations
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(h2("Continuous Animations"))
                            .child(body("Smooth looping animations for loading and attention"))
                            .child(
                                div()
                                    .flex()
                                    .gap(px(24.0))
                                    .items_center()
                                    .flex_wrap()
                                    // Smooth Pulse
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .items_center()
                                            .child(
                                                div()
                                                    .id("pulse-smooth")
                                                    .size(px(80.0))
                                                    .rounded(px(40.0))
                                                    .bg(rgb(0x10b981))
                                                    .flex()
                                                    .items_center()
                                                    .justify_center()
                                                    .text_color(gpui::white())
                                                    .font_family(theme.tokens.font_family.clone())
                                                    .text_size(px(24.0))
                                                    .child("●")
                                                    .with_animation(
                                                        "pulse-smooth-anim",
                                                        presets::pulse(),
                                                        |div, delta| {
                                                            let scale = pulse_scale(delta, 1.0, 1.15);
                                                            div.size(px(80.0 * scale))
                                                        }
                                                    )
                                            )
                                            .child(caption("Smooth Pulse"))
                                    )
                                    // Fast Pulse
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .items_center()
                                            .child(
                                                div()
                                                    .id("pulse-fast")
                                                    .size(px(80.0))
                                                    .rounded(px(40.0))
                                                    .bg(rgb(0xf59e0b))
                                                    .flex()
                                                    .items_center()
                                                    .justify_center()
                                                    .text_color(gpui::white())
                                                    .font_family(theme.tokens.font_family.clone())
                                                    .text_size(px(24.0))
                                                    .child("●")
                                                    .with_animation(
                                                        "pulse-fast-anim",
                                                        presets::pulse_fast(),
                                                        |div, delta| {
                                                            let scale = pulse_scale(delta, 1.0, 1.2);
                                                            div.size(px(80.0 * scale))
                                                        }
                                                    )
                                            )
                                            .child(caption("Fast Pulse"))
                                    )
                                    // Opacity Pulse
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .items_center()
                                            .child(
                                                div()
                                                    .id("pulse-opacity")
                                                    .size(px(80.0))
                                                    .rounded(px(40.0))
                                                    .bg(theme.tokens.primary)
                                                    .flex()
                                                    .items_center()
                                                    .justify_center()
                                                    .text_color(theme.tokens.primary_foreground)
                                                    .font_family(theme.tokens.font_family.clone())
                                                    .text_size(px(24.0))
                                                    .child("●")
                                                    .with_animation(
                                                        "pulse-opacity-anim",
                                                        presets::pulse_slow(),
                                                        |div, delta| {
                                                            let opacity = pulse_opacity(delta, 0.4, 1.0);
                                                            div.opacity(opacity)
                                                        }
                                                    )
                                            )
                                            .child(caption("Opacity Pulse"))
                                    )
                            )
                    )
                    // Interactive Animations
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(h2("Interactive Animations"))
                            .child(body("Trigger animations on user interaction"))
                            .child(
                                div()
                                    .flex()
                                    .gap(px(12.0))
                                    .flex_wrap()
                                    .items_center()
                                    .child(
                                        Button::new("trigger-shake-btn", "Trigger Shake")
                                            .variant(ButtonVariant::Destructive)
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.show_shake = true;
                                                cx.notify();

                                                // Reset after animation
                                                cx.spawn(async move |this, cx| {
                                                    cx.background_executor().timer(Duration::from_millis(400)).await;
                                                    let _ = this.update(cx, |this, cx| {
                                                        this.show_shake = false;
                                                        cx.notify();
                                                    });
                                                }).detach();
                                            }))
                                    )
                                    .when(self.show_shake, |this| {
                                        this.child(
                                            div()
                                                .id("shake-box")
                                                .px(px(16.0))
                                                .py(px(8.0))
                                                .rounded(theme.tokens.radius_md)
                                                .bg(theme.tokens.destructive.opacity(0.1))
                                                .text_color(theme.tokens.destructive)
                                                .font_family(theme.tokens.font_family.clone())
                                                .child("⚠️ Error! Natural shake decay")
                                                .with_animation(
                                                    "shake-anim",
                                                    presets::shake(),
                                                    |div, delta| {
                                                        let offset = shake_offset(delta, 12.0);
                                                        div.ml(px(offset))
                                                    }
                                                )
                                        )
                                    })
                            )
                    )
                    // Easing Comparison
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .p(px(16.0))
                            .rounded(theme.tokens.radius_lg)
                            .bg(theme.tokens.muted.opacity(0.3))
                            .child(h2("Improved Features"))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.0))
                                    .child(body("✓ Professional cubic-bezier easing functions"))
                                    .child(body("✓ Spring physics for natural motion"))
                                    .child(body("✓ Helper functions for smooth pulse, shake, and bounce"))
                                    .child(body("✓ Multiple timing presets (ultra-fast to extra-slow)"))
                                    .child(body("✓ Back easing with subtle overshoot"))
                                    .child(body("✓ Exponential and elastic easing options"))
                            )
                        )
                    )
                )
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        // Initialize the UI library
        adabraka_ui::init(cx);

        // Install dark theme
        install_theme(cx, Theme::dark());

        let bounds = Bounds::centered(None, size(px(900.0), px(800.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("Animation System Demo".into()),
                    appears_transparent: false,
                    traffic_light_position: None,
                }),
                ..Default::default()
            },
            |_, cx| cx.new(|_| AnimationsDemo::new()),
        )
        .unwrap();

        cx.activate(true);
    });
}
