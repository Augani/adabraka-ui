use adabraka_ui::{
    components::{
        scrollable::scrollable_vertical,
        sparkline::{Sparkline, SparklineSize},
        text::{caption, h1, h2},
    },
    theme::{install_theme, use_theme, Theme},
};
use gpui::*;

struct SparklineDemo;

impl SparklineDemo {
    fn new() -> Self {
        Self
    }

    fn upward_trend_data() -> Vec<f64> {
        vec![10.0, 12.0, 15.0, 14.0, 18.0, 22.0, 25.0, 28.0, 32.0, 35.0]
    }

    fn downward_trend_data() -> Vec<f64> {
        vec![35.0, 32.0, 28.0, 30.0, 25.0, 22.0, 18.0, 15.0, 12.0, 10.0]
    }

    fn volatile_data() -> Vec<f64> {
        vec![20.0, 35.0, 15.0, 40.0, 25.0, 45.0, 30.0, 50.0, 20.0, 35.0]
    }

    fn flat_data() -> Vec<f64> {
        vec![25.0, 24.0, 26.0, 25.0, 25.0, 24.0, 26.0, 25.0, 24.0, 25.0]
    }

    fn stock_price_data() -> Vec<f64> {
        vec![
            142.5, 145.2, 143.8, 148.1, 152.3, 149.7, 155.2, 158.9, 156.4, 162.1, 165.8, 163.2,
            168.5, 172.1, 169.8, 175.3, 178.9, 176.4, 182.1, 185.7,
        ]
    }

    fn cpu_usage_data() -> Vec<f64> {
        vec![
            45.0, 52.0, 68.0, 72.0, 65.0, 48.0, 55.0, 78.0, 85.0, 62.0, 45.0, 38.0, 42.0, 58.0,
            65.0,
        ]
    }

    fn memory_data() -> Vec<f64> {
        vec![
            2.1, 2.3, 2.5, 2.8, 3.2, 3.5, 3.8, 4.2, 4.5, 4.8, 5.1, 5.4, 5.7, 6.0, 6.3,
        ]
    }

    fn network_traffic_data() -> Vec<f64> {
        vec![
            120.0, 450.0, 280.0, 890.0, 560.0, 320.0, 780.0, 1100.0, 650.0, 420.0, 980.0, 540.0,
        ]
    }
}

impl Render for SparklineDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(theme.tokens.background)
            .overflow_hidden()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .p(px(24.0))
                    .border_b_1()
                    .border_color(theme.tokens.border)
                    .child(h1("Sparkline"))
                    .child(caption("Compact inline charts for visualizing data trends")),
            )
            .child(scrollable_vertical(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(48.0))
                    .p(px(24.0))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(h2("Variant: Line"))
                            .child(caption("Default line sparkline for continuous data"))
                            .child(
                                div()
                                    .flex()
                                    .flex_wrap()
                                    .gap(px(32.0))
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .child(Sparkline::line(Self::upward_trend_data()))
                                            .child(caption("Upward Trend")),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .child(Sparkline::line(Self::downward_trend_data()))
                                            .child(caption("Downward Trend")),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .child(Sparkline::line(Self::volatile_data()))
                                            .child(caption("Volatile Data")),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .child(Sparkline::line(Self::flat_data()))
                                            .child(caption("Flat/Stable")),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(h2("Variant: Bar"))
                            .child(caption("Bar sparkline for discrete data points"))
                            .child(
                                div()
                                    .flex()
                                    .flex_wrap()
                                    .gap(px(32.0))
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .child(Sparkline::bar(Self::upward_trend_data()))
                                            .child(caption("Upward Trend")),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .child(Sparkline::bar(Self::downward_trend_data()))
                                            .child(caption("Downward Trend")),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .child(Sparkline::bar(Self::volatile_data()))
                                            .child(caption("Volatile Data")),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .child(Sparkline::bar(Self::cpu_usage_data()))
                                            .child(caption("CPU Usage")),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(h2("Variant: Area"))
                            .child(caption("Area sparkline with filled region"))
                            .child(
                                div()
                                    .flex()
                                    .flex_wrap()
                                    .gap(px(32.0))
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .child(Sparkline::area(Self::upward_trend_data()))
                                            .child(caption("Upward Trend")),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .child(Sparkline::area(Self::downward_trend_data()))
                                            .child(caption("Downward Trend")),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .child(Sparkline::area(Self::stock_price_data()))
                                            .child(caption("Stock Price")),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .child(Sparkline::area(Self::memory_data()))
                                            .child(caption("Memory Usage")),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(h2("Size Variants"))
                            .child(caption("Small, Medium, and Large sizes"))
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
                                            .gap(px(8.0))
                                            .child(
                                                Sparkline::line(Self::stock_price_data())
                                                    .size(SparklineSize::Sm),
                                            )
                                            .child(caption("Small")),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .child(
                                                Sparkline::line(Self::stock_price_data())
                                                    .size(SparklineSize::Md),
                                            )
                                            .child(caption("Medium (Default)")),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .child(
                                                Sparkline::line(Self::stock_price_data())
                                                    .size(SparklineSize::Lg),
                                            )
                                            .child(caption("Large")),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .child(
                                                Sparkline::line(Self::stock_price_data())
                                                    .width(px(200.0))
                                                    .height(px(40.0)),
                                            )
                                            .child(caption("Custom (200x40)")),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(h2("Min/Max Indicators"))
                            .child(caption("Show dots at minimum and maximum values"))
                            .child(
                                div()
                                    .flex()
                                    .flex_wrap()
                                    .gap(px(32.0))
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .child(
                                                Sparkline::line(Self::volatile_data())
                                                    .size(SparklineSize::Lg)
                                                    .show_min_max(true),
                                            )
                                            .child(caption("Line with Min/Max")),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .child(
                                                Sparkline::area(Self::stock_price_data())
                                                    .size(SparklineSize::Lg)
                                                    .show_min_max(true),
                                            )
                                            .child(caption("Area with Min/Max")),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .child(
                                                Sparkline::bar(Self::cpu_usage_data())
                                                    .size(SparklineSize::Lg)
                                                    .show_min_max(true),
                                            )
                                            .child(caption("Bar with Min/Max")),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(h2("Trend Indicator"))
                            .child(caption("Color-coded trend with direction indicator"))
                            .child(
                                div()
                                    .flex()
                                    .flex_wrap()
                                    .gap(px(32.0))
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .child(
                                                Sparkline::line(Self::upward_trend_data())
                                                    .size(SparklineSize::Lg)
                                                    .show_trend(true),
                                            )
                                            .child(caption("Upward (Green)")),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .child(
                                                Sparkline::line(Self::downward_trend_data())
                                                    .size(SparklineSize::Lg)
                                                    .show_trend(true),
                                            )
                                            .child(caption("Downward (Red)")),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .child(
                                                Sparkline::line(Self::flat_data())
                                                    .size(SparklineSize::Lg)
                                                    .show_trend(true),
                                            )
                                            .child(caption("Neutral")),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .child(
                                                Sparkline::area(Self::stock_price_data())
                                                    .size(SparklineSize::Lg)
                                                    .show_trend(true),
                                            )
                                            .child(caption("Stock (Area + Trend)")),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(h2("Custom Colors"))
                            .child(caption("Override line and fill colors"))
                            .child(
                                div()
                                    .flex()
                                    .flex_wrap()
                                    .gap(px(32.0))
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .child(
                                                Sparkline::line(Self::upward_trend_data())
                                                    .size(SparklineSize::Lg)
                                                    .line_color(rgb(0x8b5cf6).into()),
                                            )
                                            .child(caption("Purple Line")),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .child(
                                                Sparkline::area(Self::stock_price_data())
                                                    .size(SparklineSize::Lg)
                                                    .line_color(rgb(0xf59e0b).into())
                                                    .fill_color(rgba(0xf59e0b4d).into()),
                                            )
                                            .child(caption("Orange Area")),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .child(
                                                Sparkline::bar(Self::cpu_usage_data())
                                                    .size(SparklineSize::Lg)
                                                    .line_color(rgb(0x06b6d4).into()),
                                            )
                                            .child(caption("Cyan Bars")),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .child(
                                                Sparkline::line(Self::volatile_data())
                                                    .size(SparklineSize::Lg)
                                                    .line_color(rgb(0xec4899).into())
                                                    .show_min_max(true)
                                                    .min_max_color(rgb(0xfbbf24).into()),
                                            )
                                            .child(caption("Pink with Yellow Markers")),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(h2("Dashboard Example"))
                            .child(caption("Sparklines in a typical dashboard context"))
                            .child(
                                div()
                                    .flex()
                                    .flex_wrap()
                                    .gap(px(16.0))
                                    .child(
                                        div()
                                            .p(px(16.0))
                                            .rounded(theme.tokens.radius_lg)
                                            .bg(theme.tokens.card)
                                            .border_1()
                                            .border_color(theme.tokens.border)
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .child(
                                                div()
                                                    .flex()
                                                    .justify_between()
                                                    .items_center()
                                                    .child(
                                                        div()
                                                            .text_sm()
                                                            .text_color(
                                                                theme.tokens.muted_foreground,
                                                            )
                                                            .child("Stock Price"),
                                                    )
                                                    .child(
                                                        div()
                                                            .text_lg()
                                                            .font_weight(FontWeight::SEMIBOLD)
                                                            .text_color(theme.tokens.foreground)
                                                            .child("$185.70"),
                                                    ),
                                            )
                                            .child(
                                                Sparkline::area(Self::stock_price_data())
                                                    .width(px(180.0))
                                                    .height(px(40.0))
                                                    .show_trend(true)
                                                    .show_min_max(true),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .p(px(16.0))
                                            .rounded(theme.tokens.radius_lg)
                                            .bg(theme.tokens.card)
                                            .border_1()
                                            .border_color(theme.tokens.border)
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .child(
                                                div()
                                                    .flex()
                                                    .justify_between()
                                                    .items_center()
                                                    .child(
                                                        div()
                                                            .text_sm()
                                                            .text_color(
                                                                theme.tokens.muted_foreground,
                                                            )
                                                            .child("CPU Usage"),
                                                    )
                                                    .child(
                                                        div()
                                                            .text_lg()
                                                            .font_weight(FontWeight::SEMIBOLD)
                                                            .text_color(theme.tokens.foreground)
                                                            .child("65%"),
                                                    ),
                                            )
                                            .child(
                                                Sparkline::bar(Self::cpu_usage_data())
                                                    .width(px(180.0))
                                                    .height(px(40.0))
                                                    .line_color(rgb(0xf59e0b).into())
                                                    .show_min_max(true),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .p(px(16.0))
                                            .rounded(theme.tokens.radius_lg)
                                            .bg(theme.tokens.card)
                                            .border_1()
                                            .border_color(theme.tokens.border)
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .child(
                                                div()
                                                    .flex()
                                                    .justify_between()
                                                    .items_center()
                                                    .child(
                                                        div()
                                                            .text_sm()
                                                            .text_color(
                                                                theme.tokens.muted_foreground,
                                                            )
                                                            .child("Memory"),
                                                    )
                                                    .child(
                                                        div()
                                                            .text_lg()
                                                            .font_weight(FontWeight::SEMIBOLD)
                                                            .text_color(theme.tokens.foreground)
                                                            .child("6.3 GB"),
                                                    ),
                                            )
                                            .child(
                                                Sparkline::line(Self::memory_data())
                                                    .width(px(180.0))
                                                    .height(px(40.0))
                                                    .line_color(rgb(0x8b5cf6).into())
                                                    .show_trend(true),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .p(px(16.0))
                                            .rounded(theme.tokens.radius_lg)
                                            .bg(theme.tokens.card)
                                            .border_1()
                                            .border_color(theme.tokens.border)
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .child(
                                                div()
                                                    .flex()
                                                    .justify_between()
                                                    .items_center()
                                                    .child(
                                                        div()
                                                            .text_sm()
                                                            .text_color(
                                                                theme.tokens.muted_foreground,
                                                            )
                                                            .child("Network"),
                                                    )
                                                    .child(
                                                        div()
                                                            .text_lg()
                                                            .font_weight(FontWeight::SEMIBOLD)
                                                            .text_color(theme.tokens.foreground)
                                                            .child("540 MB/s"),
                                                    ),
                                            )
                                            .child(
                                                Sparkline::area(Self::network_traffic_data())
                                                    .width(px(180.0))
                                                    .height(px(40.0))
                                                    .line_color(rgb(0x06b6d4).into())
                                                    .fill_color(rgba(0x06b6d433).into()),
                                            ),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(h2("Combined Features"))
                            .child(caption("Sparklines with multiple features enabled"))
                            .child(
                                div()
                                    .flex()
                                    .flex_wrap()
                                    .gap(px(32.0))
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .child(
                                                Sparkline::area(Self::stock_price_data())
                                                    .size(SparklineSize::Lg)
                                                    .show_trend(true)
                                                    .show_min_max(true),
                                            )
                                            .child(caption("Trend + Min/Max (Area)")),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .child(
                                                Sparkline::line(Self::volatile_data())
                                                    .size(SparklineSize::Lg)
                                                    .show_trend(true)
                                                    .show_min_max(true)
                                                    .line_color(rgb(0xec4899).into()),
                                            )
                                            .child(caption("All Features (Line)")),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .child(
                                                Sparkline::bar(Self::upward_trend_data())
                                                    .size(SparklineSize::Lg)
                                                    .show_trend(true)
                                                    .show_min_max(true),
                                            )
                                            .child(caption("All Features (Bar)")),
                                    ),
                            ),
                    ),
            ))
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        adabraka_ui::init(cx);
        install_theme(cx, Theme::dark());

        let bounds = Bounds::centered(None, size(px(1000.0), px(800.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("Sparkline Demo".into()),
                    appears_transparent: false,
                    traffic_light_position: None,
                }),
                ..Default::default()
            },
            |_, cx| cx.new(|_| SparklineDemo::new()),
        )
        .unwrap();

        cx.activate(true);
    });
}
