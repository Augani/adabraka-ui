use adabraka_ui::{
    components::spinner::{Spinner, SpinnerSize, SpinnerVariant},
    layout::VStack,
    theme::{install_theme, use_theme, Theme},
};
use gpui::*;

struct SpinnerDemo;

impl Render for SpinnerDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        div()
            .size_full()
            .bg(theme.tokens.background)
            .text_color(theme.tokens.foreground)
            .font_family(theme.tokens.font_family.clone())
            .child(
                VStack::new()
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
                                    .child("Spinner Demo"),
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Loading indicators with various sizes and variants"),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .font_weight(FontWeight::MEDIUM)
                                    .child("Sizes"),
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_end()
                                    .gap(px(24.0))
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .items_center()
                                            .gap(px(8.0))
                                            .child(Spinner::new().size(SpinnerSize::Xs))
                                            .child(
                                                div()
                                                    .text_size(px(12.0))
                                                    .text_color(theme.tokens.muted_foreground)
                                                    .child("XS"),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .items_center()
                                            .gap(px(8.0))
                                            .child(Spinner::new().size(SpinnerSize::Sm))
                                            .child(
                                                div()
                                                    .text_size(px(12.0))
                                                    .text_color(theme.tokens.muted_foreground)
                                                    .child("SM"),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .items_center()
                                            .gap(px(8.0))
                                            .child(Spinner::new().size(SpinnerSize::Md))
                                            .child(
                                                div()
                                                    .text_size(px(12.0))
                                                    .text_color(theme.tokens.muted_foreground)
                                                    .child("MD"),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .items_center()
                                            .gap(px(8.0))
                                            .child(Spinner::new().size(SpinnerSize::Lg))
                                            .child(
                                                div()
                                                    .text_size(px(12.0))
                                                    .text_color(theme.tokens.muted_foreground)
                                                    .child("LG"),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .items_center()
                                            .gap(px(8.0))
                                            .child(Spinner::new().size(SpinnerSize::Xl))
                                            .child(
                                                div()
                                                    .text_size(px(12.0))
                                                    .text_color(theme.tokens.muted_foreground)
                                                    .child("XL"),
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
                                    .text_size(px(14.0))
                                    .font_weight(FontWeight::MEDIUM)
                                    .child("Variants"),
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(32.0))
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .items_center()
                                            .gap(px(8.0))
                                            .child(
                                                Spinner::new()
                                                    .size(SpinnerSize::Lg)
                                                    .variant(SpinnerVariant::Default),
                                            )
                                            .child(
                                                div()
                                                    .text_size(px(12.0))
                                                    .text_color(theme.tokens.muted_foreground)
                                                    .child("Default"),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .items_center()
                                            .gap(px(8.0))
                                            .child(
                                                Spinner::new()
                                                    .size(SpinnerSize::Lg)
                                                    .variant(SpinnerVariant::Primary),
                                            )
                                            .child(
                                                div()
                                                    .text_size(px(12.0))
                                                    .text_color(theme.tokens.muted_foreground)
                                                    .child("Primary"),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .items_center()
                                            .gap(px(8.0))
                                            .child(
                                                Spinner::new()
                                                    .size(SpinnerSize::Lg)
                                                    .variant(SpinnerVariant::Secondary),
                                            )
                                            .child(
                                                div()
                                                    .text_size(px(12.0))
                                                    .text_color(theme.tokens.muted_foreground)
                                                    .child("Secondary"),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .items_center()
                                            .gap(px(8.0))
                                            .child(
                                                Spinner::new()
                                                    .size(SpinnerSize::Lg)
                                                    .variant(SpinnerVariant::Muted),
                                            )
                                            .child(
                                                div()
                                                    .text_size(px(12.0))
                                                    .text_color(theme.tokens.muted_foreground)
                                                    .child("Muted"),
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
                                    .text_size(px(14.0))
                                    .font_weight(FontWeight::MEDIUM)
                                    .child("With Label"),
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(32.0))
                                    .child(Spinner::new().size(SpinnerSize::Md).label("Loading..."))
                                    .child(
                                        Spinner::new()
                                            .size(SpinnerSize::Md)
                                            .variant(SpinnerVariant::Primary)
                                            .label("Please wait"),
                                    )
                                    .child(
                                        Spinner::new().size(SpinnerSize::Lg).label("Processing"),
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

        let bounds = Bounds::centered(None, size(px(600.0), px(500.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("Spinner Demo".into()),
                    appears_transparent: false,
                    traffic_light_position: None,
                }),
                ..Default::default()
            },
            |_, cx| cx.new(|_| SpinnerDemo),
        )
        .unwrap();

        cx.activate(true);
    });
}
