use adabraka_ui::{
    components::range_slider::{RangeSlider, RangeSliderState},
    components::slider::SliderSize,
    layout::VStack,
    theme::{install_theme, use_theme, Theme},
};
use gpui::{prelude::FluentBuilder as _, *};

struct RangeSliderDemo {
    price_slider: Entity<RangeSliderState>,
    age_slider: Entity<RangeSliderState>,
    vertical_slider: Entity<RangeSliderState>,
    current_price_range: (f32, f32),
    current_age_range: (f32, f32),
}

impl RangeSliderDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        let price_slider = cx.new(|cx| {
            let mut state = RangeSliderState::new(cx);
            state.set_min(0.0, cx);
            state.set_max(1000.0, cx);
            state.set_range(200.0, 800.0, cx);
            state.set_step(10.0, cx);
            state
        });

        let age_slider = cx.new(|cx| {
            let mut state = RangeSliderState::new(cx);
            state.set_min(18.0, cx);
            state.set_max(65.0, cx);
            state.set_range(25.0, 45.0, cx);
            state.set_step(1.0, cx);
            state
        });

        let vertical_slider = cx.new(|cx| {
            let mut state = RangeSliderState::new(cx);
            state.set_min(0.0, cx);
            state.set_max(100.0, cx);
            state.set_range(20.0, 80.0, cx);
            state
        });

        Self {
            price_slider,
            age_slider,
            vertical_slider,
            current_price_range: (200.0, 800.0),
            current_age_range: (25.0, 45.0),
        }
    }
}

impl Render for RangeSliderDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        div()
            .size_full()
            .bg(theme.tokens.background)
            .text_color(theme.tokens.foreground)
            .font_family(theme.tokens.font_family.clone())
            .child(
                VStack::new()
                    .p(px(32.0))
                    .gap(px(32.0))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .text_size(px(32.0))
                                    .font_weight(FontWeight::BOLD)
                                    .child("Range Slider Demo"),
                            )
                            .child(
                                div()
                                    .text_size(px(16.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Dual-handle sliders for selecting value ranges"),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(24.0))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(12.0))
                                    .child(
                                        div()
                                            .flex()
                                            .justify_between()
                                            .child(
                                                div()
                                                    .text_size(px(16.0))
                                                    .font_weight(FontWeight::SEMIBOLD)
                                                    .child("Price Range"),
                                            )
                                            .child(
                                                div()
                                                    .text_size(px(14.0))
                                                    .text_color(theme.tokens.primary)
                                                    .child(format!(
                                                        "${:.0} - ${:.0}",
                                                        self.current_price_range.0,
                                                        self.current_price_range.1
                                                    )),
                                            ),
                                    )
                                    .child({
                                        let entity = cx.entity().clone();
                                        RangeSlider::new(self.price_slider.clone())
                                            .size(SliderSize::Lg)
                                            .on_change(move |start, end, _, cx| {
                                                entity.update(cx, |this, cx| {
                                                    this.current_price_range = (start, end);
                                                    cx.notify();
                                                });
                                            })
                                    }),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(12.0))
                                    .child(
                                        div()
                                            .flex()
                                            .justify_between()
                                            .child(
                                                div()
                                                    .text_size(px(16.0))
                                                    .font_weight(FontWeight::SEMIBOLD)
                                                    .child("Age Range"),
                                            )
                                            .child(
                                                div()
                                                    .text_size(px(14.0))
                                                    .text_color(theme.tokens.primary)
                                                    .child(format!(
                                                        "{:.0} - {:.0} years",
                                                        self.current_age_range.0,
                                                        self.current_age_range.1
                                                    )),
                                            ),
                                    )
                                    .child({
                                        let entity = cx.entity().clone();
                                        RangeSlider::new(self.age_slider.clone())
                                            .size(SliderSize::Md)
                                            .show_values(true)
                                            .on_change(move |start, end, _, cx| {
                                                entity.update(cx, |this, cx| {
                                                    this.current_age_range = (start, end);
                                                    cx.notify();
                                                });
                                            })
                                    }),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(12.0))
                                    .child(
                                        div()
                                            .text_size(px(16.0))
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .child("Small Size (Disabled)"),
                                    )
                                    .child({
                                        RangeSlider::new(self.price_slider.clone())
                                            .size(SliderSize::Sm)
                                            .disabled(true)
                                    }),
                            )
                            .child(
                                div().flex().gap(px(32.0)).child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .gap(px(12.0))
                                        .child(
                                            div()
                                                .text_size(px(16.0))
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .child("Vertical Orientation"),
                                        )
                                        .child(
                                            div().h(px(200.0)).child(
                                                RangeSlider::new(self.vertical_slider.clone())
                                                    .vertical()
                                                    .show_values(true),
                                            ),
                                        ),
                                ),
                            ),
                    ),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        adabraka_ui::init(cx);
        install_theme(cx, Theme::dark());

        let bounds = Bounds::centered(None, size(px(800.0), px(700.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("Range Slider Demo".into()),
                    appears_transparent: false,
                    traffic_light_position: None,
                }),
                ..Default::default()
            },
            |_, cx| cx.new(|cx| RangeSliderDemo::new(cx)),
        )
        .unwrap();

        cx.activate(true);
    });
}
