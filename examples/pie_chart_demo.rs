use adabraka_ui::{
    charts::pie_chart::{PieChart, PieChartLabelPosition, PieChartSegment, PieChartSize},
    components::{
        scrollable::scrollable_vertical,
        text::{caption, h1, h2},
    },
    theme::{install_theme, use_theme, Theme},
};
use gpui::*;

struct PieChartDemo;

impl PieChartDemo {
    fn new() -> Self {
        Self
    }

    fn sample_data() -> Vec<PieChartSegment> {
        vec![
            PieChartSegment::new("Chrome", 65.0),
            PieChartSegment::new("Safari", 18.0),
            PieChartSegment::new("Firefox", 10.0),
            PieChartSegment::new("Edge", 5.0),
            PieChartSegment::new("Other", 2.0),
        ]
    }

    fn sales_data() -> Vec<PieChartSegment> {
        vec![
            PieChartSegment::new("Electronics", 45000.0),
            PieChartSegment::new("Clothing", 28000.0),
            PieChartSegment::new("Food", 18000.0),
            PieChartSegment::new("Books", 8000.0),
            PieChartSegment::new("Sports", 12000.0),
        ]
    }

    fn custom_color_data() -> Vec<PieChartSegment> {
        vec![
            PieChartSegment::new("Success", 60.0).color(rgb(0x22c55e).into()),
            PieChartSegment::new("Warning", 25.0).color(rgb(0xf59e0b).into()),
            PieChartSegment::new("Error", 15.0).color(rgb(0xef4444).into()),
        ]
    }

    fn budget_data() -> Vec<PieChartSegment> {
        vec![
            PieChartSegment::new("Housing", 1800.0),
            PieChartSegment::new("Transportation", 600.0),
            PieChartSegment::new("Food", 500.0),
            PieChartSegment::new("Utilities", 300.0),
            PieChartSegment::new("Entertainment", 200.0),
            PieChartSegment::new("Healthcare", 150.0),
            PieChartSegment::new("Savings", 450.0),
        ]
    }

    fn simple_data() -> Vec<PieChartSegment> {
        vec![
            PieChartSegment::new("Yes", 70.0),
            PieChartSegment::new("No", 30.0),
        ]
    }
}

impl Render for PieChartDemo {
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
                    .child(h1("Pie Chart / Donut Chart"))
                    .child(caption(
                        "Visualize proportions and percentages with pie and donut charts",
                    )),
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
                            .child(h2("Basic Pie Chart"))
                            .child(caption("A simple pie chart showing browser market share"))
                            .child(
                                div()
                                    .flex()
                                    .gap(px(32.0))
                                    .child(PieChart::pie(Self::sample_data())),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(h2("Donut Chart"))
                            .child(caption("Donut chart variant with a center hole"))
                            .child(
                                div()
                                    .flex()
                                    .gap(px(32.0))
                                    .child(PieChart::donut(Self::sample_data())),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(h2("With Percentages"))
                            .child(caption("Pie chart with percentage labels in the legend"))
                            .child(
                                div().flex().gap(px(32.0)).child(
                                    PieChart::pie(Self::sample_data())
                                        .label_position(PieChartLabelPosition::Legend)
                                        .show_percentages(true),
                                ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(h2("Donut with Center Label"))
                            .child(caption("Donut chart with a center label showing the total"))
                            .child(
                                div().flex().gap(px(32.0)).child(
                                    PieChart::donut(Self::budget_data())
                                        .center_label("$4,000")
                                        .label_position(PieChartLabelPosition::Legend)
                                        .show_percentages(true),
                                ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(h2("Custom Colors"))
                            .child(caption("Using custom colors for each segment"))
                            .child(
                                div().flex().gap(px(32.0)).child(
                                    PieChart::pie(Self::custom_color_data())
                                        .label_position(PieChartLabelPosition::Legend)
                                        .show_percentages(true),
                                ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(h2("Size Variants"))
                            .child(caption("Small, medium, large, and custom sizes"))
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
                                            .gap(px(8.0))
                                            .child(
                                                PieChart::pie(Self::simple_data())
                                                    .size(PieChartSize::Sm),
                                            )
                                            .child(caption("Small")),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .items_center()
                                            .gap(px(8.0))
                                            .child(
                                                PieChart::pie(Self::simple_data())
                                                    .size(PieChartSize::Md),
                                            )
                                            .child(caption("Medium")),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .items_center()
                                            .gap(px(8.0))
                                            .child(
                                                PieChart::pie(Self::simple_data())
                                                    .size(PieChartSize::Lg),
                                            )
                                            .child(caption("Large")),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .items_center()
                                            .gap(px(8.0))
                                            .child(PieChart::pie(Self::simple_data()).size_px(100))
                                            .child(caption("Custom (100px)")),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(h2("Donut Thickness Variants"))
                            .child(caption("Customizable donut hole size"))
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
                                            .gap(px(8.0))
                                            .child(
                                                PieChart::donut(Self::simple_data())
                                                    .donut_thickness(0.2),
                                            )
                                            .child(caption("Thin (0.2)")),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .items_center()
                                            .gap(px(8.0))
                                            .child(
                                                PieChart::donut(Self::simple_data())
                                                    .donut_thickness(0.35),
                                            )
                                            .child(caption("Default (0.35)")),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .items_center()
                                            .gap(px(8.0))
                                            .child(
                                                PieChart::donut(Self::simple_data())
                                                    .donut_thickness(0.5),
                                            )
                                            .child(caption("Medium (0.5)")),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .items_center()
                                            .gap(px(8.0))
                                            .child(
                                                PieChart::donut(Self::simple_data())
                                                    .donut_thickness(0.7),
                                            )
                                            .child(caption("Thick (0.7)")),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(h2("Complete Example"))
                            .child(caption("Full-featured donut chart with all options"))
                            .child(
                                div()
                                    .p(px(24.0))
                                    .rounded(theme.tokens.radius_lg)
                                    .bg(theme.tokens.card)
                                    .border_1()
                                    .border_color(theme.tokens.border)
                                    .child(
                                        PieChart::donut(Self::sales_data())
                                            .size(PieChartSize::Lg)
                                            .center_label("$111K")
                                            .label_position(PieChartLabelPosition::Legend)
                                            .show_percentages(true)
                                            .donut_thickness(0.4),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(h2("Edge Cases"))
                            .child(caption("Single segment and empty data handling"))
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
                                            .gap(px(8.0))
                                            .child(PieChart::pie(vec![PieChartSegment::new(
                                                "Complete", 100.0,
                                            )]))
                                            .child(caption("Single Segment")),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .items_center()
                                            .gap(px(8.0))
                                            .child(PieChart::pie(vec![]))
                                            .child(caption("Empty Data")),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .items_center()
                                            .gap(px(8.0))
                                            .child(PieChart::donut(vec![PieChartSegment::new(
                                                "Only Item",
                                                50.0,
                                            )]))
                                            .child(caption("Single Donut")),
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

        let bounds = Bounds::centered(None, size(px(1200.0), px(900.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("Pie Chart Demo".into()),
                    appears_transparent: false,
                    traffic_light_position: None,
                }),
                ..Default::default()
            },
            |_, cx| cx.new(|_| PieChartDemo::new()),
        )
        .unwrap();

        cx.activate(true);
    });
}
