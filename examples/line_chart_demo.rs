use adabraka_ui::{
    components::scrollable::scrollable_vertical,
    prelude::{LineChart, LineChartPoint, LineChartSeries},
    theme::{install_theme, use_theme, Theme},
};
use gpui::*;

struct LineChartDemo;

fn generate_revenue_data() -> Vec<LineChartPoint> {
    vec![
        LineChartPoint::new(0.0, 4200.0).label("Jan"),
        LineChartPoint::new(1.0, 5800.0).label("Feb"),
        LineChartPoint::new(2.0, 5200.0).label("Mar"),
        LineChartPoint::new(3.0, 7100.0).label("Apr"),
        LineChartPoint::new(4.0, 6800.0).label("May"),
        LineChartPoint::new(5.0, 8500.0).label("Jun"),
        LineChartPoint::new(6.0, 9200.0).label("Jul"),
        LineChartPoint::new(7.0, 8800.0).label("Aug"),
        LineChartPoint::new(8.0, 9800.0).label("Sep"),
        LineChartPoint::new(9.0, 10500.0).label("Oct"),
        LineChartPoint::new(10.0, 11200.0).label("Nov"),
        LineChartPoint::new(11.0, 12800.0).label("Dec"),
    ]
}

fn generate_users_data() -> Vec<LineChartPoint> {
    vec![
        LineChartPoint::new(0.0, 1200.0),
        LineChartPoint::new(1.0, 1450.0),
        LineChartPoint::new(2.0, 1380.0),
        LineChartPoint::new(3.0, 1680.0),
        LineChartPoint::new(4.0, 1890.0),
        LineChartPoint::new(5.0, 2100.0),
        LineChartPoint::new(6.0, 2350.0),
        LineChartPoint::new(7.0, 2580.0),
        LineChartPoint::new(8.0, 2890.0),
        LineChartPoint::new(9.0, 3200.0),
        LineChartPoint::new(10.0, 3650.0),
        LineChartPoint::new(11.0, 4100.0),
    ]
}

fn generate_sessions_data() -> Vec<LineChartPoint> {
    vec![
        LineChartPoint::new(0.0, 3500.0),
        LineChartPoint::new(1.0, 4200.0),
        LineChartPoint::new(2.0, 3800.0),
        LineChartPoint::new(3.0, 4800.0),
        LineChartPoint::new(4.0, 5200.0),
        LineChartPoint::new(5.0, 5800.0),
        LineChartPoint::new(6.0, 6500.0),
        LineChartPoint::new(7.0, 7200.0),
        LineChartPoint::new(8.0, 7800.0),
        LineChartPoint::new(9.0, 8500.0),
        LineChartPoint::new(10.0, 9200.0),
        LineChartPoint::new(11.0, 10500.0),
    ]
}

fn generate_sine_data() -> Vec<LineChartPoint> {
    (0..50)
        .map(|i| {
            let x = i as f64 * 0.2;
            let y = (x * 1.5).sin() * 50.0 + 50.0;
            LineChartPoint::new(x, y)
        })
        .collect()
}

fn generate_cosine_data() -> Vec<LineChartPoint> {
    (0..50)
        .map(|i| {
            let x = i as f64 * 0.2;
            let y = (x * 1.5).cos() * 50.0 + 50.0;
            LineChartPoint::new(x, y)
        })
        .collect()
}

impl Render for LineChartDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        let x_labels: Vec<&str> = vec![
            "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
        ];

        div()
            .size_full()
            .bg(theme.tokens.background)
            .text_color(theme.tokens.foreground)
            .font_family(theme.tokens.font_family.clone())
            .overflow_hidden()
            .child(scrollable_vertical(
                div()
                    .flex()
                    .flex_col()
                    .p(px(24.0))
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
                                    .child("LineChart Demo"),
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child(
                                        "Line charts for visualizing trends and time series data",
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(
                                div()
                                    .text_size(px(18.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("Basic Single Line Chart"),
                            )
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("A simple line chart showing monthly revenue"),
                            )
                            .child(
                                div()
                                    .h(px(300.0))
                                    .w_full()
                                    .rounded(theme.tokens.radius_lg)
                                    .border_1()
                                    .border_color(theme.tokens.border)
                                    .bg(theme.tokens.card)
                                    .p(px(16.0))
                                    .child(
                                        LineChart::single(
                                            LineChartSeries::new(
                                                "Revenue",
                                                generate_revenue_data(),
                                            )
                                            .color(rgb(0x3b82f6).into()),
                                        )
                                        .x_labels(x_labels.clone()),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(
                                div()
                                    .text_size(px(18.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("With Data Points"),
                            )
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Line chart with visible data point markers"),
                            )
                            .child(
                                div()
                                    .h(px(300.0))
                                    .w_full()
                                    .rounded(theme.tokens.radius_lg)
                                    .border_1()
                                    .border_color(theme.tokens.border)
                                    .bg(theme.tokens.card)
                                    .p(px(16.0))
                                    .child(
                                        LineChart::single(
                                            LineChartSeries::new("Users", generate_users_data())
                                                .color(rgb(0x22c55e).into())
                                                .show_points(true),
                                        )
                                        .x_labels(x_labels.clone()),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(
                                div()
                                    .text_size(px(18.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("Area Fill"),
                            )
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Line chart with area fill under the line"),
                            )
                            .child(
                                div()
                                    .h(px(300.0))
                                    .w_full()
                                    .rounded(theme.tokens.radius_lg)
                                    .border_1()
                                    .border_color(theme.tokens.border)
                                    .bg(theme.tokens.card)
                                    .p(px(16.0))
                                    .child(
                                        LineChart::single(
                                            LineChartSeries::new(
                                                "Sessions",
                                                generate_sessions_data(),
                                            )
                                            .color(rgb(0x8b5cf6).into())
                                            .fill_area(true),
                                        )
                                        .x_labels(x_labels.clone()),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(
                                div()
                                    .text_size(px(18.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("Smooth Curves"),
                            )
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Comparison of smooth curves vs straight lines"),
                            )
                            .child(
                                div()
                                    .flex()
                                    .gap(px(16.0))
                                    .child(
                                        div()
                                            .flex_1()
                                            .h(px(250.0))
                                            .rounded(theme.tokens.radius_lg)
                                            .border_1()
                                            .border_color(theme.tokens.border)
                                            .bg(theme.tokens.card)
                                            .p(px(16.0))
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .size_full()
                                                    .child(
                                                        div()
                                                            .text_sm()
                                                            .font_weight(FontWeight::MEDIUM)
                                                            .text_color(
                                                                theme.tokens.muted_foreground,
                                                            )
                                                            .mb(px(8.0))
                                                            .child("Straight Lines"),
                                                    )
                                                    .child(
                                                        LineChart::single(
                                                            LineChartSeries::new(
                                                                "Sine",
                                                                generate_sine_data(),
                                                            )
                                                            .color(rgb(0xf59e0b).into())
                                                            .show_points(true),
                                                        )
                                                        .smooth(false)
                                                        .show_legend(false),
                                                    ),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .flex_1()
                                            .h(px(250.0))
                                            .rounded(theme.tokens.radius_lg)
                                            .border_1()
                                            .border_color(theme.tokens.border)
                                            .bg(theme.tokens.card)
                                            .p(px(16.0))
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .size_full()
                                                    .child(
                                                        div()
                                                            .text_sm()
                                                            .font_weight(FontWeight::MEDIUM)
                                                            .text_color(
                                                                theme.tokens.muted_foreground,
                                                            )
                                                            .mb(px(8.0))
                                                            .child("Smooth Curves"),
                                                    )
                                                    .child(
                                                        LineChart::single(
                                                            LineChartSeries::new(
                                                                "Sine",
                                                                generate_sine_data(),
                                                            )
                                                            .color(rgb(0xf59e0b).into()),
                                                        )
                                                        .smooth(true)
                                                        .show_legend(false),
                                                    ),
                                            ),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(
                                div()
                                    .text_size(px(18.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("Multi-Series Line Chart"),
                            )
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Multiple data series with automatic legend"),
                            )
                            .child(
                                div()
                                    .h(px(350.0))
                                    .w_full()
                                    .rounded(theme.tokens.radius_lg)
                                    .border_1()
                                    .border_color(theme.tokens.border)
                                    .bg(theme.tokens.card)
                                    .p(px(16.0))
                                    .child(
                                        LineChart::new(vec![
                                            LineChartSeries::new(
                                                "Revenue",
                                                generate_revenue_data(),
                                            )
                                            .color(rgb(0x3b82f6).into()),
                                            LineChartSeries::new("Users", generate_users_data())
                                                .color(rgb(0x22c55e).into()),
                                            LineChartSeries::new(
                                                "Sessions",
                                                generate_sessions_data(),
                                            )
                                            .color(rgb(0xf59e0b).into()),
                                        ])
                                        .x_labels(x_labels.clone()),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(
                                div()
                                    .text_size(px(18.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("Trigonometric Functions"),
                            )
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Smooth sine and cosine waves with area fill"),
                            )
                            .child(
                                div()
                                    .h(px(300.0))
                                    .w_full()
                                    .rounded(theme.tokens.radius_lg)
                                    .border_1()
                                    .border_color(theme.tokens.border)
                                    .bg(theme.tokens.card)
                                    .p(px(16.0))
                                    .child(
                                        LineChart::new(vec![
                                            LineChartSeries::new("sin(x)", generate_sine_data())
                                                .color(rgb(0xef4444).into())
                                                .fill_area(true),
                                            LineChartSeries::new("cos(x)", generate_cosine_data())
                                                .color(rgb(0x06b6d4).into())
                                                .fill_area(true),
                                        ])
                                        .smooth(true)
                                        .y_range(0.0, 100.0),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(
                                div()
                                    .text_size(px(18.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("All Features Combined"),
                            )
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Smooth curves with data points and area fill"),
                            )
                            .child(
                                div()
                                    .h(px(350.0))
                                    .w_full()
                                    .rounded(theme.tokens.radius_lg)
                                    .border_1()
                                    .border_color(theme.tokens.border)
                                    .bg(theme.tokens.card)
                                    .p(px(16.0))
                                    .child(
                                        LineChart::new(vec![
                                            LineChartSeries::new(
                                                "Revenue",
                                                generate_revenue_data(),
                                            )
                                            .color(rgb(0x3b82f6).into())
                                            .show_points(true)
                                            .fill_area(true),
                                            LineChartSeries::new(
                                                "Sessions",
                                                generate_sessions_data(),
                                            )
                                            .color(rgb(0xec4899).into())
                                            .show_points(true)
                                            .fill_area(true),
                                        ])
                                        .smooth(true)
                                        .x_labels(x_labels.clone()),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(
                                div()
                                    .text_size(px(18.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("Without Grid"),
                            )
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Clean chart without grid lines"),
                            )
                            .child(
                                div()
                                    .h(px(250.0))
                                    .w_full()
                                    .rounded(theme.tokens.radius_lg)
                                    .border_1()
                                    .border_color(theme.tokens.border)
                                    .bg(theme.tokens.card)
                                    .p(px(16.0))
                                    .child(
                                        LineChart::single(
                                            LineChartSeries::new(
                                                "Revenue",
                                                generate_revenue_data(),
                                            )
                                            .color(rgb(0x3b82f6).into())
                                            .fill_area(true),
                                        )
                                        .show_grid(false)
                                        .smooth(true)
                                        .x_labels(x_labels.clone()),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(
                                div()
                                    .text_size(px(18.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("Custom Color Palette"),
                            )
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Using custom colors for each series"),
                            )
                            .child(
                                div()
                                    .h(px(300.0))
                                    .w_full()
                                    .rounded(theme.tokens.radius_lg)
                                    .border_1()
                                    .border_color(theme.tokens.border)
                                    .bg(theme.tokens.card)
                                    .p(px(16.0))
                                    .child(
                                        LineChart::new(vec![
                                            LineChartSeries::new(
                                                "Series A",
                                                generate_revenue_data(),
                                            )
                                            .color(hsla(0.85, 0.7, 0.5, 1.0))
                                            .show_points(true),
                                            LineChartSeries::new("Series B", generate_users_data())
                                                .color(hsla(0.55, 0.7, 0.5, 1.0))
                                                .show_points(true),
                                            LineChartSeries::new(
                                                "Series C",
                                                generate_sessions_data(),
                                            )
                                            .color(hsla(0.15, 0.7, 0.5, 1.0))
                                            .show_points(true),
                                        ])
                                        .smooth(true)
                                        .x_labels(x_labels),
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

        let bounds = Bounds::centered(None, size(px(900.0), px(800.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("LineChart Demo".into()),
                    appears_transparent: false,
                    traffic_light_position: None,
                }),
                ..Default::default()
            },
            |_, cx| cx.new(|_| LineChartDemo),
        )
        .unwrap();

        cx.activate(true);
    });
}
