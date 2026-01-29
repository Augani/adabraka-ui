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
                        title: Some("Rating Component Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(800.0), px(900.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|cx| RatingDemo::new(cx)),
            )
            .unwrap();
        });
}

struct RatingDemo {
    rating1: Entity<RatingState>,
    rating2: Entity<RatingState>,
    rating3: Entity<RatingState>,
    rating4: Entity<RatingState>,
    rating5: Entity<RatingState>,
    rating6: Entity<RatingState>,
    rating7: Entity<RatingState>,
    rating8: Entity<RatingState>,
    rating9: Entity<RatingState>,
    rating10: Entity<RatingState>,
}

impl RatingDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        let rating1 = cx.new(|cx| {
            let mut state = RatingState::new(cx);
            state.set_value(3.0, cx);
            state
        });

        let rating2 = cx.new(|cx| {
            let mut state = RatingState::new(cx);
            state.set_value(4.0, cx);
            state
        });

        let rating3 = cx.new(|cx| {
            let mut state = RatingState::new(cx);
            state.set_value(2.0, cx);
            state
        });

        let rating4 = cx.new(|cx| {
            let mut state = RatingState::new(cx);
            state.set_allows_half(true, cx);
            state.set_value(3.5, cx);
            state
        });

        let rating5 = cx.new(|cx| {
            let mut state = RatingState::new(cx);
            state.set_max_rating(10, cx);
            state.set_value(7.0, cx);
            state
        });

        let rating6 = cx.new(|cx| {
            let mut state = RatingState::new(cx);
            state.set_value(5.0, cx);
            state
        });

        let rating7 = cx.new(|cx| {
            let mut state = RatingState::new(cx);
            state.set_allows_half(true, cx);
            state.set_value(4.5, cx);
            state
        });

        let rating8 = cx.new(|cx| {
            let mut state = RatingState::new(cx);
            state.set_value(0.0, cx);
            state
        });

        let rating9 = cx.new(|cx| {
            let mut state = RatingState::new(cx);
            state.set_max_rating(3, cx);
            state.set_value(2.0, cx);
            state
        });

        let rating10 = cx.new(|cx| {
            let mut state = RatingState::new(cx);
            state.set_allows_half(true, cx);
            state.set_value(2.5, cx);
            state
        });

        Self {
            rating1,
            rating2,
            rating3,
            rating4,
            rating5,
            rating6,
            rating7,
            rating8,
            rating9,
            rating10,
        }
    }
}

impl Render for RatingDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        let value1 = self.rating1.read(cx).value();
        let value2 = self.rating2.read(cx).value();
        let value3 = self.rating3.read(cx).value();
        let value4 = self.rating4.read(cx).value();
        let value5 = self.rating5.read(cx).value();
        let value6 = self.rating6.read(cx).value();
        let value7 = self.rating7.read(cx).value();
        let value8 = self.rating8.read(cx).value();
        let value9 = self.rating9.read(cx).value();
        let value10 = self.rating10.read(cx).value();

        div()
            .size_full()
            .bg(theme.tokens.background)
            .overflow_hidden()
            .child(
                scrollable_vertical(
                    div()
                        .flex()
                        .flex_col()
                        .text_color(theme.tokens.foreground)
                        .p(px(32.0))
                        .gap(px(32.0))
                        .child(
                            VStack::new()
                                .gap(px(8.0))
                                .child(
                                    div()
                                        .text_size(px(24.0))
                                        .font_weight(FontWeight::BOLD)
                                        .child("Rating Component Demo"),
                                )
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child("Interactive star rating with various configurations"),
                                ),
                        )
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("1. Default Rating (Medium Size)"),
                                )
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child(format!("Value: {}", value1)),
                                )
                                .child(
                                    Rating::new(self.rating1.clone()).on_change(|value, _, _| {
                                        println!("Rating 1 changed to: {}", value);
                                    }),
                                ),
                        )
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("2. Small Size"),
                                )
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child(format!("Value: {}", value2)),
                                )
                                .child(Rating::new(self.rating2.clone()).size(RatingSize::Sm)),
                        )
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("3. Large Size"),
                                )
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child(format!("Value: {}", value3)),
                                )
                                .child(Rating::new(self.rating3.clone()).size(RatingSize::Lg)),
                        )
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("4. Half Rating Support"),
                                )
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child(format!("Value: {} (click for half stars)", value4)),
                                )
                                .child(
                                    Rating::new(self.rating4.clone()).on_change(|value, _, _| {
                                        println!("Half rating changed to: {}", value);
                                    }),
                                ),
                        )
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("5. Custom Max Rating (10 stars)"),
                                )
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child(format!("Value: {} / 10", value5)),
                                )
                                .child(
                                    Rating::new(self.rating5.clone())
                                        .size(RatingSize::Sm)
                                        .on_change(|value, _, _| {
                                            println!("10-star rating changed to: {}", value);
                                        }),
                                ),
                        )
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("6. Read-Only Display"),
                                )
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child(format!("Value: {} (not clickable)", value6)),
                                )
                                .child(Rating::new(self.rating6.clone()).read_only(true)),
                        )
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("7. Custom Colors"),
                                )
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child(format!("Value: {}", value7)),
                                )
                                .child(
                                    Rating::new(self.rating7.clone())
                                        .size(RatingSize::Lg)
                                        .active_color(hsla(0.28, 0.8, 0.45, 1.0))
                                        .inactive_color(hsla(0.28, 0.2, 0.3, 0.5)),
                                ),
                        )
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("8. Interactive with Keyboard"),
                                )
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child(format!(
                                            "Value: {} (focus and use arrow keys)",
                                            value8
                                        )),
                                )
                                .child(
                                    Rating::new(self.rating8.clone()).on_change(|value, _, _| {
                                        println!("Keyboard rating changed to: {}", value);
                                    }),
                                ),
                        )
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("9. Custom Max Rating (3 stars)"),
                                )
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child(format!("Value: {} / 3", value9)),
                                )
                                .child(Rating::new(self.rating9.clone()).size(RatingSize::Lg)),
                        )
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("10. Styled Container"),
                                )
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child(format!("Value: {}", value10)),
                                )
                                .child(
                                    Rating::new(self.rating10.clone())
                                        .size(RatingSize::Lg)
                                        .active_color(hsla(0.08, 0.95, 0.55, 1.0))
                                        .bg(rgb(0x1e293b))
                                        .p(px(16.0))
                                        .rounded(px(12.0))
                                        .border_1()
                                        .border_color(rgb(0x334155)),
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
                                        .child("Features: Click to rate, hover preview, keyboard navigation (arrows), half-star support, custom colors, read-only mode"),
                                ),
                        ),
                ),
            )
    }
}
