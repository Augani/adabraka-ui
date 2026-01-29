use adabraka_ui::{components::scrollable::scrollable_vertical, prelude::*};
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
                        title: Some("Bar Chart Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(1000.0), px(900.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|_| BarChartDemo::new()),
            )
            .unwrap();
        });
}

struct BarChartDemo {}

impl BarChartDemo {
    fn new() -> Self {
        Self {}
    }
}

impl Render for BarChartDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        let basic_data = vec![
            BarChartData::new("Jan", 65.0),
            BarChartData::new("Feb", 59.0),
            BarChartData::new("Mar", 80.0),
            BarChartData::new("Apr", 81.0),
            BarChartData::new("May", 56.0),
            BarChartData::new("Jun", 55.0),
        ];

        let colored_data = vec![
            BarChartData::new("React", 85.0).color(rgb(0x61dafb).into()),
            BarChartData::new("Vue", 72.0).color(rgb(0x42b883).into()),
            BarChartData::new("Angular", 58.0).color(rgb(0xdd0031).into()),
            BarChartData::new("Svelte", 45.0).color(rgb(0xff3e00).into()),
            BarChartData::new("Solid", 35.0).color(rgb(0x2c4f7c).into()),
        ];

        let horizontal_data = vec![
            BarChartData::new("United States", 331.0),
            BarChartData::new("China", 1412.0),
            BarChartData::new("India", 1380.0),
            BarChartData::new("Indonesia", 273.0),
            BarChartData::new("Pakistan", 220.0),
            BarChartData::new("Brazil", 212.0),
        ];

        let multi_labels = vec!["Q1", "Q2", "Q3", "Q4"];
        let multi_series = vec![
            BarChartSeries::new("Product A", vec![120.0, 150.0, 180.0, 200.0])
                .color(rgb(0x3b82f6).into()),
            BarChartSeries::new("Product B", vec![80.0, 95.0, 110.0, 130.0])
                .color(rgb(0x22c55e).into()),
            BarChartSeries::new("Product C", vec![60.0, 70.0, 85.0, 95.0])
                .color(rgb(0xf59e0b).into()),
        ];

        let stacked_labels = vec!["2021", "2022", "2023", "2024"];
        let stacked_series = vec![
            BarChartSeries::new("Desktop", vec![400.0, 380.0, 350.0, 320.0])
                .color(rgb(0x8b5cf6).into()),
            BarChartSeries::new("Mobile", vec![300.0, 420.0, 500.0, 580.0])
                .color(rgb(0x06b6d4).into()),
            BarChartSeries::new("Tablet", vec![100.0, 120.0, 130.0, 140.0])
                .color(rgb(0xf97316).into()),
        ];

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
                    .gap(px(40.0))
                    .child(
                        VStack::new()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .text_size(px(28.0))
                                    .font_weight(FontWeight::BOLD)
                                    .child("Bar Chart Component Demo"),
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Comprehensive bar chart examples with various configurations"),
                            ),
                    )
                    .child(
                        VStack::new()
                            .gap(px(16.0))
                            .child(
                                div()
                                    .text_size(px(20.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("1. Basic Vertical Bar Chart"),
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Simple bar chart with default styling"),
                            )
                            .child(
                                div()
                                    .p(px(24.0))
                                    .bg(theme.tokens.card)
                                    .rounded(theme.tokens.radius_lg)
                                    .border_1()
                                    .border_color(theme.tokens.border)
                                    .child(BarChart::new(basic_data.clone()).show_grid(true)),
                            ),
                    )
                    .child(
                        VStack::new()
                            .gap(px(16.0))
                            .child(
                                div()
                                    .text_size(px(20.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("2. Vertical Bar Chart with Value Labels"),
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Shows values above each bar"),
                            )
                            .child(
                                div()
                                    .p(px(24.0))
                                    .bg(theme.tokens.card)
                                    .rounded(theme.tokens.radius_lg)
                                    .border_1()
                                    .border_color(theme.tokens.border)
                                    .child(
                                        BarChart::new(basic_data.clone())
                                            .show_values(true)
                                            .show_grid(true),
                                    ),
                            ),
                    )
                    .child(
                        VStack::new()
                            .gap(px(16.0))
                            .child(
                                div()
                                    .text_size(px(20.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("3. Custom Colors Bar Chart"),
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Each bar with its own custom color"),
                            )
                            .child(
                                div()
                                    .p(px(24.0))
                                    .bg(theme.tokens.card)
                                    .rounded(theme.tokens.radius_lg)
                                    .border_1()
                                    .border_color(theme.tokens.border)
                                    .child(
                                        BarChart::new(colored_data)
                                            .show_values(true)
                                            .bar_width(px(50.0)),
                                    ),
                            ),
                    )
                    .child(
                        VStack::new()
                            .gap(px(16.0))
                            .child(
                                div()
                                    .text_size(px(20.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("4. Horizontal Bar Chart"),
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Bars rendered horizontally with labels on the left"),
                            )
                            .child(
                                div()
                                    .p(px(24.0))
                                    .bg(theme.tokens.card)
                                    .rounded(theme.tokens.radius_lg)
                                    .border_1()
                                    .border_color(theme.tokens.border)
                                    .child(
                                        BarChart::new(horizontal_data)
                                            .horizontal()
                                            .show_values(true)
                                            .show_grid(true),
                                    ),
                            ),
                    )
                    .child(
                        VStack::new()
                            .gap(px(16.0))
                            .child(
                                div()
                                    .text_size(px(20.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("5. Grouped Multi-Series Bar Chart"),
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Multiple data series displayed side by side with legend"),
                            )
                            .child(
                                div()
                                    .p(px(24.0))
                                    .bg(theme.tokens.card)
                                    .rounded(theme.tokens.radius_lg)
                                    .border_1()
                                    .border_color(theme.tokens.border)
                                    .child(
                                        BarChart::multi_series(
                                            multi_labels.clone(),
                                            multi_series.clone(),
                                        )
                                        .grouped()
                                        .show_values(true)
                                        .show_grid(true)
                                        .show_legend(true)
                                        .bar_width(px(80.0)),
                                    ),
                            ),
                    )
                    .child(
                        VStack::new()
                            .gap(px(16.0))
                            .child(
                                div()
                                    .text_size(px(20.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("6. Stacked Multi-Series Bar Chart"),
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Multiple data series stacked on top of each other"),
                            )
                            .child(
                                div()
                                    .p(px(24.0))
                                    .bg(theme.tokens.card)
                                    .rounded(theme.tokens.radius_lg)
                                    .border_1()
                                    .border_color(theme.tokens.border)
                                    .child(
                                        BarChart::multi_series(stacked_labels.clone(), stacked_series.clone())
                                            .stacked()
                                            .show_values(true)
                                            .show_grid(true)
                                            .show_legend(true)
                                            .bar_width(px(60.0)),
                                    ),
                            ),
                    )
                    .child(
                        VStack::new()
                            .gap(px(16.0))
                            .child(
                                div()
                                    .text_size(px(20.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("7. Horizontal Grouped Bar Chart"),
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Grouped series rendered horizontally"),
                            )
                            .child(
                                div()
                                    .p(px(24.0))
                                    .bg(theme.tokens.card)
                                    .rounded(theme.tokens.radius_lg)
                                    .border_1()
                                    .border_color(theme.tokens.border)
                                    .child(
                                        BarChart::multi_series(multi_labels.clone(), multi_series.clone())
                                            .horizontal()
                                            .grouped()
                                            .show_values(true)
                                            .show_legend(true)
                                            .bar_width(px(48.0)),
                                    ),
                            ),
                    )
                    .child(
                        VStack::new()
                            .gap(px(16.0))
                            .child(
                                div()
                                    .text_size(px(20.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("8. Horizontal Stacked Bar Chart"),
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Stacked series rendered horizontally"),
                            )
                            .child(
                                div()
                                    .p(px(24.0))
                                    .bg(theme.tokens.card)
                                    .rounded(theme.tokens.radius_lg)
                                    .border_1()
                                    .border_color(theme.tokens.border)
                                    .child(
                                        BarChart::multi_series(stacked_labels, stacked_series)
                                            .horizontal()
                                            .stacked()
                                            .show_values(true)
                                            .show_legend(true),
                                    ),
                            ),
                    )
                    .child(
                        VStack::new()
                            .gap(px(16.0))
                            .child(
                                div()
                                    .text_size(px(20.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("9. Different Sizes"),
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Bar charts with different heights and bar widths"),
                            )
                            .child(
                                HStack::new()
                                    .gap(px(16.0))
                                    .child(
                                        div()
                                            .flex_1()
                                            .p(px(16.0))
                                            .bg(theme.tokens.card)
                                            .rounded(theme.tokens.radius_lg)
                                            .border_1()
                                            .border_color(theme.tokens.border)
                                            .child(
                                                VStack::new()
                                                    .gap(px(8.0))
                                                    .child(
                                                        div()
                                                            .text_sm()
                                                            .font_weight(FontWeight::MEDIUM)
                                                            .child("Small"),
                                                    )
                                                    .child(
                                                        BarChart::new(basic_data.clone())
                                                            .chart_height(px(150.0))
                                                            .bar_width(px(20.0))
                                                            .gap(px(4.0)),
                                                    ),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .flex_1()
                                            .p(px(16.0))
                                            .bg(theme.tokens.card)
                                            .rounded(theme.tokens.radius_lg)
                                            .border_1()
                                            .border_color(theme.tokens.border)
                                            .child(
                                                VStack::new()
                                                    .gap(px(8.0))
                                                    .child(
                                                        div()
                                                            .text_sm()
                                                            .font_weight(FontWeight::MEDIUM)
                                                            .child("Medium"),
                                                    )
                                                    .child(
                                                        BarChart::new(basic_data.clone())
                                                            .chart_height(px(200.0))
                                                            .bar_width(px(30.0)),
                                                    ),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .flex_1()
                                            .p(px(16.0))
                                            .bg(theme.tokens.card)
                                            .rounded(theme.tokens.radius_lg)
                                            .border_1()
                                            .border_color(theme.tokens.border)
                                            .child(
                                                VStack::new()
                                                    .gap(px(8.0))
                                                    .child(
                                                        div()
                                                            .text_sm()
                                                            .font_weight(FontWeight::MEDIUM)
                                                            .child("Large"),
                                                    )
                                                    .child(
                                                        BarChart::new(basic_data)
                                                            .chart_height(px(250.0))
                                                            .bar_width(px(45.0))
                                                            .show_values(true),
                                                    ),
                                            ),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .mt(px(16.0))
                            .p(px(16.0))
                            .bg(theme.tokens.accent)
                            .rounded(px(8.0))
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.accent_foreground)
                                    .child("BarChart supports vertical/horizontal orientation, single/grouped/stacked modes, custom colors, value labels, grid lines, and legends."),
                            ),
                    ),
            ))
    }
}
