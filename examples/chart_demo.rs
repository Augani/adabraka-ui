use adabraka_ui::{
    charts::chart::Axis as ChartAxis, components::scrollable::scrollable_vertical, prelude::*,
};
use gpui::*;
use std::path::PathBuf;

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
                            .and_then(|entry| entry.file_name().into_string().ok())
                            .map(SharedString::from)
                    })
                    .collect()
            })
            .map_err(|err| err.into())
    }
}

fn main() {
    Application::new()
        .with_assets(Assets {
            base: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
        })
        .run(|cx| {
            adabraka_ui::init(cx);
            adabraka_ui::set_icon_base_path("assets/icons");
            install_theme(cx, Theme::dark());

            cx.open_window(
                WindowOptions {
                    titlebar: Some(TitlebarOptions {
                        title: Some("Declarative Chart Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(1200.0), px(900.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|_| ChartDemo::new()),
            )
            .unwrap();
        });
}

struct ChartDemo {}

impl ChartDemo {
    fn new() -> Self {
        Self {}
    }
}

impl Render for ChartDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        div()
            .size_full()
            .bg(theme.tokens.background)
            .overflow_hidden()
            .child(scrollable_vertical(
                div()
                    .flex()
                    .flex_col()
                    .text_color(theme.tokens.foreground)
                    .p(px(32.0))
                    .gap(px(48.0))
                    .child(Self::header_section(&theme))
                    .child(Self::basic_line_chart(&theme))
                    .child(Self::multi_series_chart(&theme))
                    .child(Self::bar_chart(&theme))
                    .child(Self::area_chart(&theme))
                    .child(Self::scatter_chart(&theme))
                    .child(Self::mixed_chart(&theme))
                    .child(Self::custom_axes_chart(&theme)),
            ))
    }
}

impl ChartDemo {
    fn header_section(theme: &Theme) -> impl IntoElement {
        VStack::new()
            .gap(px(8.0))
            .child(
                div()
                    .text_size(px(32.0))
                    .font_weight(FontWeight::BOLD)
                    .child("Declarative Chart Component"),
            )
            .child(
                div()
                    .text_size(px(16.0))
                    .text_color(theme.tokens.muted_foreground)
                    .child("Composable charts with hover tooltips, multiple series types, and full customization"),
            )
    }

    fn basic_line_chart(theme: &Theme) -> impl IntoElement {
        let data = vec![
            DataPoint::labeled(0.0, 45.0, "Jan"),
            DataPoint::labeled(1.0, 52.0, "Feb"),
            DataPoint::labeled(2.0, 38.0, "Mar"),
            DataPoint::labeled(3.0, 65.0, "Apr"),
            DataPoint::labeled(4.0, 78.0, "May"),
            DataPoint::labeled(5.0, 62.0, "Jun"),
        ];

        VStack::new()
            .gap(px(16.0))
            .child(
                div()
                    .text_size(px(20.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .child("1. Basic Line Chart with Hover Points"),
            )
            .child(
                div()
                    .text_size(px(14.0))
                    .text_color(theme.tokens.muted_foreground)
                    .child("Hover over points to see tooltips with values"),
            )
            .child(
                div()
                    .h(px(350.0))
                    .p(px(24.0))
                    .bg(theme.tokens.card)
                    .rounded(theme.tokens.radius_lg)
                    .border_1()
                    .border_color(theme.tokens.border)
                    .child(
                        Chart::new()
                            .series(
                                Series::line("Revenue", data)
                                    .color(rgb(0x3b82f6))
                                    .show_points(true)
                                    .stroke_width(2.5),
                            )
                            .y_axis(ChartAxis::new().label("Revenue ($K)"))
                            .x_axis(ChartAxis::new().label("Month"))
                            .show_legend(false),
                    ),
            )
    }

    fn multi_series_chart(theme: &Theme) -> impl IntoElement {
        let sales_data = vec![
            DataPoint::labeled(0.0, 120.0, "Q1"),
            DataPoint::labeled(1.0, 145.0, "Q2"),
            DataPoint::labeled(2.0, 180.0, "Q3"),
            DataPoint::labeled(3.0, 210.0, "Q4"),
        ];

        let expenses_data = vec![
            DataPoint::labeled(0.0, 80.0, "Q1"),
            DataPoint::labeled(1.0, 95.0, "Q2"),
            DataPoint::labeled(2.0, 110.0, "Q3"),
            DataPoint::labeled(3.0, 125.0, "Q4"),
        ];

        let profit_data = vec![
            DataPoint::labeled(0.0, 40.0, "Q1"),
            DataPoint::labeled(1.0, 50.0, "Q2"),
            DataPoint::labeled(2.0, 70.0, "Q3"),
            DataPoint::labeled(3.0, 85.0, "Q4"),
        ];

        VStack::new()
            .gap(px(16.0))
            .child(
                div()
                    .text_size(px(20.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .child("2. Multi-Series Line Chart"),
            )
            .child(
                div()
                    .text_size(px(14.0))
                    .text_color(theme.tokens.muted_foreground)
                    .child("Multiple data series with automatic legend"),
            )
            .child(
                div()
                    .h(px(400.0))
                    .p(px(24.0))
                    .bg(theme.tokens.card)
                    .rounded(theme.tokens.radius_lg)
                    .border_1()
                    .border_color(theme.tokens.border)
                    .child(
                        Chart::new()
                            .series(
                                Series::line("Sales", sales_data)
                                    .color(rgb(0x22c55e))
                                    .show_points(true),
                            )
                            .series(
                                Series::line("Expenses", expenses_data)
                                    .color(rgb(0xef4444))
                                    .show_points(true),
                            )
                            .series(
                                Series::line("Profit", profit_data)
                                    .color(rgb(0x3b82f6))
                                    .show_points(true),
                            )
                            .y_axis(ChartAxis::new().range(0.0, 250.0)),
                    ),
            )
    }

    fn bar_chart(theme: &Theme) -> impl IntoElement {
        let data = vec![
            DataPoint::labeled(0.0, 85.0, "React"),
            DataPoint::labeled(1.0, 72.0, "Vue"),
            DataPoint::labeled(2.0, 58.0, "Angular"),
            DataPoint::labeled(3.0, 45.0, "Svelte"),
            DataPoint::labeled(4.0, 35.0, "Solid"),
        ];

        VStack::new()
            .gap(px(16.0))
            .child(
                div()
                    .text_size(px(20.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .child("3. Bar Chart"),
            )
            .child(
                div()
                    .text_size(px(14.0))
                    .text_color(theme.tokens.muted_foreground)
                    .child("Bar series with hover detection"),
            )
            .child(
                div()
                    .h(px(350.0))
                    .p(px(24.0))
                    .bg(theme.tokens.card)
                    .rounded(theme.tokens.radius_lg)
                    .border_1()
                    .border_color(theme.tokens.border)
                    .child(
                        Chart::new()
                            .series(
                                Series::bar("Framework Popularity", data)
                                    .color(rgb(0x8b5cf6))
                                    .bar_width(40.0),
                            )
                            .y_axis(ChartAxis::new().range(0.0, 100.0).label("Usage %"))
                            .show_legend(false),
                    ),
            )
    }

    fn area_chart(theme: &Theme) -> impl IntoElement {
        let data: Vec<DataPoint> = (0..12)
            .map(|i| {
                let month = match i {
                    0 => "Jan",
                    1 => "Feb",
                    2 => "Mar",
                    3 => "Apr",
                    4 => "May",
                    5 => "Jun",
                    6 => "Jul",
                    7 => "Aug",
                    8 => "Sep",
                    9 => "Oct",
                    10 => "Nov",
                    _ => "Dec",
                };
                let value = 50.0 + (i as f64 * 0.8).sin() * 30.0 + i as f64 * 5.0;
                DataPoint::labeled(i as f64, value, month)
            })
            .collect();

        VStack::new()
            .gap(px(16.0))
            .child(
                div()
                    .text_size(px(20.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .child("4. Area Chart"),
            )
            .child(
                div()
                    .text_size(px(14.0))
                    .text_color(theme.tokens.muted_foreground)
                    .child("Line chart with filled area under the curve"),
            )
            .child(
                div()
                    .h(px(350.0))
                    .p(px(24.0))
                    .bg(theme.tokens.card)
                    .rounded(theme.tokens.radius_lg)
                    .border_1()
                    .border_color(theme.tokens.border)
                    .child(
                        Chart::new()
                            .series(
                                Series::area("Monthly Users", data)
                                    .color(rgb(0x06b6d4))
                                    .fill_opacity(0.3)
                                    .show_points(true)
                                    .smooth(true),
                            )
                            .y_axis(ChartAxis::new().range(0.0, 120.0)),
                    ),
            )
    }

    fn scatter_chart(theme: &Theme) -> impl IntoElement {
        let data: Vec<DataPoint> = (0..20)
            .map(|i| {
                let x = i as f64 * 5.0;
                let y = x * 0.5 + (i as f64 * 0.7).sin() * 15.0 + 20.0;
                DataPoint::new(x, y)
            })
            .collect();

        VStack::new()
            .gap(px(16.0))
            .child(
                div()
                    .text_size(px(20.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .child("5. Scatter Plot"),
            )
            .child(
                div()
                    .text_size(px(14.0))
                    .text_color(theme.tokens.muted_foreground)
                    .child("Data points with hover interaction"),
            )
            .child(
                div()
                    .h(px(350.0))
                    .p(px(24.0))
                    .bg(theme.tokens.card)
                    .rounded(theme.tokens.radius_lg)
                    .border_1()
                    .border_color(theme.tokens.border)
                    .child(
                        Chart::new()
                            .series(
                                Series::scatter("Data Points", data)
                                    .color(rgb(0xf59e0b))
                                    .point_radius(6.0),
                            )
                            .y_axis(ChartAxis::new().range(0.0, 80.0))
                            .x_axis(ChartAxis::new().range(0.0, 100.0)),
                    ),
            )
    }

    fn mixed_chart(theme: &Theme) -> impl IntoElement {
        let bar_data = vec![
            DataPoint::labeled(0.0, 45.0, "Jan"),
            DataPoint::labeled(1.0, 52.0, "Feb"),
            DataPoint::labeled(2.0, 38.0, "Mar"),
            DataPoint::labeled(3.0, 65.0, "Apr"),
            DataPoint::labeled(4.0, 78.0, "May"),
        ];

        let line_data = vec![
            DataPoint::new(0.0, 30.0),
            DataPoint::new(1.0, 42.0),
            DataPoint::new(2.0, 55.0),
            DataPoint::new(3.0, 48.0),
            DataPoint::new(4.0, 62.0),
        ];

        VStack::new()
            .gap(px(16.0))
            .child(
                div()
                    .text_size(px(20.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .child("6. Mixed Chart (Bar + Line)"),
            )
            .child(
                div()
                    .text_size(px(14.0))
                    .text_color(theme.tokens.muted_foreground)
                    .child("Combine different series types in one chart"),
            )
            .child(
                div()
                    .h(px(400.0))
                    .p(px(24.0))
                    .bg(theme.tokens.card)
                    .rounded(theme.tokens.radius_lg)
                    .border_1()
                    .border_color(theme.tokens.border)
                    .child(
                        Chart::new()
                            .series(
                                Series::bar("Sales Volume", bar_data)
                                    .color(rgb(0x3b82f6))
                                    .bar_width(35.0),
                            )
                            .series(
                                Series::line("Target", line_data)
                                    .color(rgb(0xef4444))
                                    .stroke_width(3.0)
                                    .show_points(true),
                            )
                            .y_axis(ChartAxis::new().range(0.0, 100.0)),
                    ),
            )
    }

    fn custom_axes_chart(theme: &Theme) -> impl IntoElement {
        let data: Vec<DataPoint> = (0..10)
            .map(|i| {
                let x = i as f64 * 1000.0;
                let y = 50000.0 + i as f64 * 8000.0 + (i as f64).sin() * 5000.0;
                DataPoint::new(x, y)
            })
            .collect();

        VStack::new()
            .gap(px(16.0))
            .child(
                div()
                    .text_size(px(20.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .child("7. Custom Axis Formatting"),
            )
            .child(
                div()
                    .text_size(px(14.0))
                    .text_color(theme.tokens.muted_foreground)
                    .child("Custom tick formatters for axes"),
            )
            .child(
                div()
                    .h(px(350.0))
                    .p(px(24.0))
                    .bg(theme.tokens.card)
                    .rounded(theme.tokens.radius_lg)
                    .border_1()
                    .border_color(theme.tokens.border)
                    .child(
                        Chart::new()
                            .series(
                                Series::area("Revenue", data)
                                    .color(rgb(0x22c55e))
                                    .fill_opacity(0.25)
                                    .smooth(true),
                            )
                            .y_axis(
                                ChartAxis::new()
                                    .label("Revenue")
                                    .tick_format(|v| format!("${:.0}K", v / 1000.0)),
                            )
                            .x_axis(ChartAxis::new().label("Time")),
                    ),
            )
            .child(
                div()
                    .mt(px(24.0))
                    .p(px(16.0))
                    .bg(theme.tokens.accent)
                    .rounded(px(8.0))
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.accent_foreground)
                            .child("The declarative Chart API supports: Line, Bar, Area, Scatter series | Hover tooltips | Custom axes | Multiple Y-axes | Legends | Grid customization"),
                    ),
            )
    }
}
