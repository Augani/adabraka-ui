use adabraka_ui::{
    components::scrollable::scrollable_vertical,
    components::text::{caption, h1, h2, h3, muted},
    theme::{install_theme, use_theme, Theme},
};
use gpui::*;
use std::time::Duration;

struct ShowcaseApp {
    animation_tick: usize,
}

impl ShowcaseApp {
    fn new(cx: &mut Context<Self>) -> Self {
        cx.spawn(async move |this, cx| loop {
            cx.background_executor()
                .timer(Duration::from_millis(50))
                .await;
            let result = this.update(cx, |state, cx| {
                state.animation_tick = state.animation_tick.wrapping_add(1);
                cx.notify();
            });
            if result.is_err() {
                break;
            }
        })
        .detach();

        Self { animation_tick: 0 }
    }
}

fn stop(color: Hsla, pct: f32) -> LinearColorStop {
    LinearColorStop {
        color,
        percentage: pct,
    }
}

fn section_card(theme: &Theme) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(16.0))
        .p(px(24.0))
        .bg(theme.tokens.card)
        .border_1()
        .border_color(theme.tokens.border)
        .rounded(px(12.0))
}

fn demo_box() -> Div {
    div()
        .w(px(100.0))
        .h(px(100.0))
        .rounded(px(8.0))
        .flex()
        .items_center()
        .justify_center()
}

impl Render for ShowcaseApp {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();
        let tick = self.animation_tick;
        let time = tick as f32 * 0.05;

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(theme.tokens.background)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(4.0))
                    .p(px(24.0))
                    .border_b_1()
                    .border_color(theme.tokens.border)
                    .child(h1("GPUI Extensions Showcase"))
                    .child(muted(
                        "Element transforms, advanced gradients, and blend modes",
                    )),
            )
            .child(scrollable_vertical(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(24.0))
                    .p(px(24.0))
                    .child(self.render_transforms_section(&theme, time))
                    .child(self.render_gradients_section(&theme))
                    .child(self.render_blend_modes_section(&theme))
                    .child(self.render_combined_section(&theme, time)),
            ))
    }
}

impl ShowcaseApp {
    fn render_transforms_section(&self, theme: &Theme, time: f32) -> Div {
        let purple = hsla(0.75, 0.8, 0.6, 1.0);
        let blue = hsla(0.6, 0.8, 0.55, 1.0);
        let cyan = hsla(0.5, 0.8, 0.55, 1.0);
        let green = hsla(0.35, 0.7, 0.5, 1.0);
        let orange = hsla(0.08, 0.9, 0.55, 1.0);

        let rotation_angle = (time * 60.0) % 360.0;
        let pulse_scale = 0.8 + (time * 2.0).sin().abs() * 0.4;

        section_card(theme)
            .child(h2("Element Transforms"))
            .child(caption(
                ".rotate(), .scale(), .scale_xy(), .transform_origin()",
            ))
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap(px(32.0))
                    .items_end()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap(px(12.0))
                            .child(
                                demo_box().bg(purple).rotate(rotation_angle).child(
                                    div()
                                        .text_size(px(13.0))
                                        .text_color(white())
                                        .child("Rotate"),
                                ),
                            )
                            .child(
                                div()
                                    .text_size(px(11.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child(format!("{:.0}deg", rotation_angle)),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap(px(12.0))
                            .child(demo_box().bg(blue).scale(pulse_scale).child(
                                div().text_size(px(13.0)).text_color(white()).child("Scale"),
                            ))
                            .child(
                                div()
                                    .text_size(px(11.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child(format!("{:.2}x", pulse_scale)),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap(px(12.0))
                            .child(
                                demo_box().bg(cyan).scale_xy(1.3, 0.7).child(
                                    div()
                                        .text_size(px(13.0))
                                        .text_color(white())
                                        .child("ScaleXY"),
                                ),
                            )
                            .child(
                                div()
                                    .text_size(px(11.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("1.3x, 0.7y"),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap(px(12.0))
                            .child(
                                demo_box().bg(green).rotate(45.0).scale(0.85).child(
                                    div()
                                        .text_size(px(11.0))
                                        .text_color(white())
                                        .child("Rot+Scale"),
                                ),
                            )
                            .child(
                                div()
                                    .text_size(px(11.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("45deg + 0.85x"),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap(px(12.0))
                            .child(
                                demo_box()
                                    .bg(orange)
                                    .rotate(rotation_angle * 0.5)
                                    .transform_origin(0.0, 0.0)
                                    .child(
                                        div()
                                            .text_size(px(10.0))
                                            .text_color(white())
                                            .child("Origin(0,0)"),
                                    ),
                            )
                            .child(
                                div()
                                    .text_size(px(11.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("top-left pivot"),
                            ),
                    ),
            )
    }

    fn render_gradients_section(&self, theme: &Theme) -> Div {
        let purple = hsla(0.75, 0.8, 0.6, 1.0);
        let blue = hsla(0.6, 0.8, 0.55, 1.0);
        let cyan = hsla(0.5, 0.8, 0.55, 1.0);
        let green = hsla(0.35, 0.7, 0.5, 1.0);
        let orange = hsla(0.08, 0.9, 0.55, 1.0);
        let red = hsla(0.0, 0.8, 0.55, 1.0);
        let pink = hsla(0.9, 0.7, 0.6, 1.0);
        let yellow = hsla(0.15, 0.9, 0.55, 1.0);

        let gradient_box = || {
            div()
                .w(px(140.0))
                .h(px(100.0))
                .rounded(px(8.0))
                .overflow_hidden()
        };

        section_card(theme)
            .child(h2("Advanced Gradients"))
            .child(caption(
                "multi_stop_linear_gradient(), radial_gradient(), conic_gradient()",
            ))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(16.0))
                    .child(h3("Multi-stop Linear Gradients (up to 4 stops)")),
            )
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap(px(16.0))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(6.0))
                            .child(gradient_box().bg(multi_stop_linear_gradient(
                                90.0,
                                &[
                                    stop(red, 0.0),
                                    stop(yellow, 0.33),
                                    stop(green, 0.66),
                                    stop(blue, 1.0),
                                ],
                            )))
                            .child(
                                div()
                                    .text_size(px(11.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Rainbow 4-stop (90deg)"),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(6.0))
                            .child(gradient_box().bg(multi_stop_linear_gradient(
                                135.0,
                                &[stop(purple, 0.0), stop(pink, 0.5), stop(orange, 1.0)],
                            )))
                            .child(
                                div()
                                    .text_size(px(11.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Sunset 3-stop (135deg)"),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(6.0))
                            .child(gradient_box().bg(multi_stop_linear_gradient(
                                0.0,
                                &[
                                    stop(hsla(0.6, 0.9, 0.3, 1.0), 0.0),
                                    stop(hsla(0.6, 0.9, 0.5, 1.0), 0.4),
                                    stop(hsla(0.55, 0.8, 0.7, 1.0), 0.7),
                                    stop(hsla(0.5, 0.7, 0.9, 1.0), 1.0),
                                ],
                            )))
                            .child(
                                div()
                                    .text_size(px(11.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Ocean depth (0deg)"),
                            ),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(16.0))
                    .child(h3("Radial Gradients")),
            )
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap(px(16.0))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(6.0))
                            .child(gradient_box().bg(radial_gradient(
                                0.5,
                                0.5,
                                0.7,
                                &[stop(yellow, 0.0), stop(orange, 0.5), stop(red, 1.0)],
                            )))
                            .child(
                                div()
                                    .text_size(px(11.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Sun burst (center)"),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(6.0))
                            .child(gradient_box().bg(radial_gradient(
                                0.3,
                                0.3,
                                0.5,
                                &[
                                    stop(white(), 0.0),
                                    stop(cyan, 0.6),
                                    stop(hsla(0.6, 0.9, 0.2, 1.0), 1.0),
                                ],
                            )))
                            .child(
                                div()
                                    .text_size(px(11.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Spotlight (offset)"),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(6.0))
                            .child(gradient_box().bg(radial_gradient(
                                0.5,
                                0.5,
                                1.0,
                                &[
                                    stop(purple, 0.0),
                                    stop(hsla(0.75, 0.5, 0.3, 1.0), 0.5),
                                    stop(hsla(0.0, 0.0, 0.05, 1.0), 1.0),
                                ],
                            )))
                            .child(
                                div()
                                    .text_size(px(11.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Nebula (large radius)"),
                            ),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(16.0))
                    .child(h3("Conic Gradients")),
            )
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap(px(16.0))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(6.0))
                            .child(gradient_box().bg(conic_gradient(
                                0.5,
                                0.5,
                                0.0,
                                &[
                                    stop(red, 0.0),
                                    stop(yellow, 0.33),
                                    stop(green, 0.66),
                                    stop(red, 1.0),
                                ],
                            )))
                            .child(
                                div()
                                    .text_size(px(11.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Color wheel"),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(6.0))
                            .child(
                                gradient_box()
                                    .bg(conic_gradient(
                                        0.5,
                                        0.5,
                                        45.0,
                                        &[stop(blue, 0.0), stop(purple, 0.5), stop(blue, 1.0)],
                                    ))
                                    .rounded(px(70.0)),
                            )
                            .child(
                                div()
                                    .text_size(px(11.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Progress ring"),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(6.0))
                            .child(gradient_box().bg(conic_gradient(
                                0.5,
                                0.5,
                                90.0,
                                &[
                                    stop(hsla(0.0, 0.0, 0.95, 1.0), 0.0),
                                    stop(hsla(0.0, 0.0, 0.7, 1.0), 0.25),
                                    stop(hsla(0.0, 0.0, 0.95, 1.0), 0.5),
                                    stop(hsla(0.0, 0.0, 0.7, 1.0), 1.0),
                                ],
                            )))
                            .child(
                                div()
                                    .text_size(px(11.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Metallic sweep"),
                            ),
                    ),
            )
    }

    fn render_blend_modes_section(&self, theme: &Theme) -> Div {
        let base_color = hsla(0.6, 0.7, 0.5, 1.0);

        let blend_box = |label: &str, mode: BlendMode| {
            div()
                .flex()
                .flex_col()
                .items_center()
                .gap(px(6.0))
                .child(
                    div()
                        .w(px(100.0))
                        .h(px(80.0))
                        .rounded(px(8.0))
                        .bg(base_color)
                        .blend_mode(mode)
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(
                            div()
                                .text_size(px(12.0))
                                .text_color(white())
                                .font_weight(FontWeight::SEMIBOLD)
                                .child(label.to_string()),
                        ),
                )
                .child(
                    div()
                        .text_size(px(11.0))
                        .text_color(theme.tokens.muted_foreground)
                        .child(label.to_string()),
                )
        };

        section_card(theme)
            .child(h2("Blend Modes"))
            .child(caption(
                ".blend_mode(BlendMode::*) — color transformation before alpha compositing",
            ))
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap(px(16.0))
                    .child(blend_box("Normal", BlendMode::Normal))
                    .child(blend_box("Multiply", BlendMode::Multiply))
                    .child(blend_box("Screen", BlendMode::Screen))
                    .child(blend_box("Overlay", BlendMode::Overlay))
                    .child(blend_box("SoftLight", BlendMode::SoftLight))
                    .child(blend_box("Difference", BlendMode::Difference)),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .child(h3("Blend Modes on Gradients"))
                    .child(
                        div()
                            .flex()
                            .flex_wrap()
                            .gap(px(16.0))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .items_center()
                                    .gap(px(6.0))
                                    .child(
                                        div()
                                            .w(px(140.0))
                                            .h(px(80.0))
                                            .rounded(px(8.0))
                                            .bg(linear_gradient(
                                                90.0,
                                                stop(hsla(0.0, 0.8, 0.5, 1.0), 0.0),
                                                stop(hsla(0.6, 0.8, 0.5, 1.0), 1.0),
                                            ))
                                            .blend_mode(BlendMode::Multiply),
                                    )
                                    .child(
                                        div()
                                            .text_size(px(11.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("Gradient + Multiply"),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .items_center()
                                    .gap(px(6.0))
                                    .child(
                                        div()
                                            .w(px(140.0))
                                            .h(px(80.0))
                                            .rounded(px(8.0))
                                            .bg(radial_gradient(
                                                0.5,
                                                0.5,
                                                0.6,
                                                &[
                                                    stop(hsla(0.15, 0.9, 0.6, 1.0), 0.0),
                                                    stop(hsla(0.0, 0.8, 0.4, 1.0), 1.0),
                                                ],
                                            ))
                                            .blend_mode(BlendMode::Screen),
                                    )
                                    .child(
                                        div()
                                            .text_size(px(11.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("Radial + Screen"),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .items_center()
                                    .gap(px(6.0))
                                    .child(
                                        div()
                                            .w(px(140.0))
                                            .h(px(80.0))
                                            .rounded(px(8.0))
                                            .bg(conic_gradient(
                                                0.5,
                                                0.5,
                                                0.0,
                                                &[
                                                    stop(hsla(0.0, 0.9, 0.5, 1.0), 0.0),
                                                    stop(hsla(0.3, 0.9, 0.5, 1.0), 0.5),
                                                    stop(hsla(0.0, 0.9, 0.5, 1.0), 1.0),
                                                ],
                                            ))
                                            .blend_mode(BlendMode::Overlay),
                                    )
                                    .child(
                                        div()
                                            .text_size(px(11.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("Conic + Overlay"),
                                    ),
                            ),
                    ),
            )
    }

    fn render_combined_section(&self, theme: &Theme, time: f32) -> Div {
        let rotation = (time * 30.0) % 360.0;
        let breath = 0.9 + (time * 1.5).sin().abs() * 0.2;

        section_card(theme)
            .child(h2("Combined: Transforms + Gradients + Blend"))
            .child(caption("All three features working together"))
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap(px(32.0))
                    .items_center()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap(px(12.0))
                            .child(
                                div()
                                    .w(px(120.0))
                                    .h(px(120.0))
                                    .rounded(px(60.0))
                                    .bg(conic_gradient(
                                        0.5,
                                        0.5,
                                        rotation,
                                        &[
                                            stop(hsla(0.0, 0.9, 0.5, 1.0), 0.0),
                                            stop(hsla(0.15, 0.9, 0.5, 1.0), 0.25),
                                            stop(hsla(0.6, 0.9, 0.5, 1.0), 0.5),
                                            stop(hsla(0.0, 0.9, 0.5, 1.0), 1.0),
                                        ],
                                    ))
                                    .scale(breath),
                            )
                            .child(
                                div()
                                    .text_size(px(11.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Spinning conic + breathing"),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap(px(12.0))
                            .child(
                                div()
                                    .w(px(120.0))
                                    .h(px(120.0))
                                    .rounded(px(12.0))
                                    .bg(multi_stop_linear_gradient(
                                        rotation * 2.0,
                                        &[
                                            stop(hsla(0.75, 0.8, 0.4, 1.0), 0.0),
                                            stop(hsla(0.85, 0.8, 0.5, 1.0), 0.33),
                                            stop(hsla(0.95, 0.8, 0.5, 1.0), 0.66),
                                            stop(hsla(0.75, 0.8, 0.4, 1.0), 1.0),
                                        ],
                                    ))
                                    .rotate(rotation * 0.5)
                                    .blend_mode(BlendMode::Screen),
                            )
                            .child(
                                div()
                                    .text_size(px(11.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Rotating gradient + Screen"),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap(px(12.0))
                            .child(
                                div()
                                    .w(px(120.0))
                                    .h(px(120.0))
                                    .rounded(px(8.0))
                                    .bg(radial_gradient(
                                        0.5,
                                        0.5,
                                        0.8,
                                        &[
                                            stop(hsla(0.1, 0.9, 0.7, 1.0), 0.0),
                                            stop(hsla(0.0, 0.9, 0.5, 1.0), 0.5),
                                            stop(hsla(0.85, 0.7, 0.3, 1.0), 1.0),
                                        ],
                                    ))
                                    .scale_xy(
                                        1.0 + (time * 2.0).sin() * 0.15,
                                        1.0 + (time * 2.0).cos() * 0.15,
                                    )
                                    .blend_mode(BlendMode::SoftLight),
                            )
                            .child(
                                div()
                                    .text_size(px(11.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Pulsing radial + SoftLight"),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap(px(12.0))
                            .child({
                                let inner_rotation = (time * 45.0) % 360.0;
                                div()
                                    .w(px(120.0))
                                    .h(px(120.0))
                                    .rounded(px(60.0))
                                    .bg(conic_gradient(
                                        0.5,
                                        0.5,
                                        inner_rotation,
                                        &[
                                            stop(hsla(0.55, 0.9, 0.6, 1.0), 0.0),
                                            stop(hsla(0.65, 0.9, 0.4, 1.0), 0.5),
                                            stop(hsla(0.55, 0.9, 0.6, 1.0), 1.0),
                                        ],
                                    ))
                                    .rotate(-inner_rotation * 0.5)
                                    .blend_mode(BlendMode::Difference)
                            })
                            .child(
                                div()
                                    .text_size(px(11.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Counter-rotate + Difference"),
                            ),
                    ),
            )
    }
}

fn main() {
    Application::new().run(move |cx| {
        let bounds = Bounds::centered(None, size(px(900.0), px(800.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("GPUI Extensions Showcase".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_, cx| {
                install_theme(cx, Theme::dark());
                cx.new(ShowcaseApp::new)
            },
        )
        .unwrap();
    });
}
