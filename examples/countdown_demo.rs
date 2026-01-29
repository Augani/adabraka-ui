use adabraka_ui::{
    components::countdown::{
        Countdown, CountdownFormat, CountdownSeparator, CountdownSize, CountdownState,
    },
    components::scrollable::scrollable_vertical,
    prelude::*,
};
use gpui::*;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

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
                        title: Some("Countdown Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(900.0), px(800.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|cx| CountdownDemo::new(cx)),
            )
            .unwrap();
        });
}

struct CountdownDemo {
    countdown1: Entity<CountdownState>,
    countdown2: Entity<CountdownState>,
    countdown3: Entity<CountdownState>,
    countdown4: Entity<CountdownState>,
    countdown5: Entity<CountdownState>,
    countdown6: Entity<CountdownState>,
}

impl CountdownDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        let countdown1 = cx.new(|cx| {
            let mut state = CountdownState::new(cx);
            state.set_duration(Duration::from_secs(3661), cx);
            state
        });

        let countdown2 = cx.new(|cx| {
            let mut state = CountdownState::new(cx);
            state.set_duration(Duration::from_secs(86400 * 2 + 3600 * 5 + 60 * 30 + 45), cx);
            state
        });

        let countdown3 = cx.new(|cx| {
            let mut state = CountdownState::new(cx);
            state.set_duration(Duration::from_secs(60 * 5 + 30), cx);
            state
        });

        let countdown4 = cx.new(|cx| {
            let mut state = CountdownState::new(cx);
            state.start_count_up(cx);
            state
        });

        let countdown5 = cx.new(|cx| {
            let mut state = CountdownState::new(cx);
            state.set_duration(Duration::from_secs(3600 * 12 + 60 * 34 + 56), cx);
            state
        });

        let countdown6 = cx.new(|cx| {
            let mut state = CountdownState::new(cx);
            let future = SystemTime::now() + Duration::from_secs(10);
            state.set_target(future, cx);
            state
        });

        Self {
            countdown1,
            countdown2,
            countdown3,
            countdown4,
            countdown5,
            countdown6,
        }
    }
}

impl Render for CountdownDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        let countdown1_state = self.countdown1.read(cx);
        let countdown4_state = self.countdown4.read(cx);
        let countdown6_state = self.countdown6.read(cx);

        let status1 = if countdown1_state.is_completed() {
            "Completed!"
        } else if countdown1_state.is_running() {
            "Running"
        } else {
            "Stopped"
        };

        let status4 = if countdown4_state.is_running() {
            "Counting up..."
        } else {
            "Stopped"
        };

        let status6 = if countdown6_state.is_completed() {
            "Time's up!"
        } else {
            "10 second countdown"
        };

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
                        .gap(px(40.0))
                        .child(
                            VStack::new()
                                .gap(px(8.0))
                                .child(
                                    div()
                                        .text_size(px(28.0))
                                        .font_weight(FontWeight::BOLD)
                                        .child("Countdown Component Demo"),
                                )
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child("A countdown timer with various display formats and sizes"),
                                ),
                        )
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(20.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("1. Default Countdown (1 hour, 1 minute, 1 second)"),
                                )
                                .child(
                                    div()
                                        .text_size(px(12.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child(format!("Status: {}", status1)),
                                )
                                .child(
                                    div()
                                        .p(px(24.0))
                                        .bg(theme.tokens.card)
                                        .rounded(theme.tokens.radius_lg)
                                        .border_1()
                                        .border_color(theme.tokens.border)
                                        .child(Countdown::new("countdown-1", self.countdown1.clone())),
                                ),
                        )
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(20.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("2. Large Size with Days"),
                                )
                                .child(
                                    div()
                                        .text_size(px(12.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child("2 days, 5 hours, 30 minutes, 45 seconds"),
                                )
                                .child(
                                    div()
                                        .p(px(24.0))
                                        .bg(theme.tokens.card)
                                        .rounded(theme.tokens.radius_lg)
                                        .border_1()
                                        .border_color(theme.tokens.border)
                                        .child(
                                            Countdown::new("countdown-2", self.countdown2.clone())
                                                .size(CountdownSize::Lg),
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
                                        .child("3. Small Size - Minutes and Seconds Only"),
                                )
                                .child(
                                    div()
                                        .text_size(px(12.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child("5 minutes, 30 seconds with minimal format"),
                                )
                                .child(
                                    div()
                                        .p(px(16.0))
                                        .bg(theme.tokens.card)
                                        .rounded(theme.tokens.radius_lg)
                                        .border_1()
                                        .border_color(theme.tokens.border)
                                        .child(
                                            Countdown::new("countdown-3", self.countdown3.clone())
                                                .size(CountdownSize::Sm)
                                                .format(CountdownFormat::minimal()),
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
                                        .child("4. Count Up Mode (Elapsed Time)"),
                                )
                                .child(
                                    div()
                                        .text_size(px(12.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child(format!("Status: {}", status4)),
                                )
                                .child(
                                    div()
                                        .p(px(24.0))
                                        .bg(theme.tokens.card)
                                        .rounded(theme.tokens.radius_lg)
                                        .border_1()
                                        .border_color(theme.tokens.border)
                                        .child(
                                            Countdown::new("countdown-4", self.countdown4.clone())
                                                .show_days(false)
                                                .separator(CountdownSeparator::Space),
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
                                        .child("5. Different Separators"),
                                )
                                .child(
                                    HStack::new()
                                        .gap(px(24.0))
                                        .child(
                                            VStack::new()
                                                .gap(px(8.0))
                                                .child(
                                                    div()
                                                        .text_size(px(12.0))
                                                        .text_color(theme.tokens.muted_foreground)
                                                        .child("Dot separator"),
                                                )
                                                .child(
                                                    div()
                                                        .p(px(16.0))
                                                        .bg(theme.tokens.card)
                                                        .rounded(theme.tokens.radius_lg)
                                                        .border_1()
                                                        .border_color(theme.tokens.border)
                                                        .child(
                                                            Countdown::new(
                                                                "countdown-5a",
                                                                self.countdown5.clone(),
                                                            )
                                                            .size(CountdownSize::Sm)
                                                            .separator(CountdownSeparator::Dot)
                                                            .format(CountdownFormat::time_only()),
                                                        ),
                                                ),
                                        )
                                        .child(
                                            VStack::new()
                                                .gap(px(8.0))
                                                .child(
                                                    div()
                                                        .text_size(px(12.0))
                                                        .text_color(theme.tokens.muted_foreground)
                                                        .child("Dash separator"),
                                                )
                                                .child(
                                                    div()
                                                        .p(px(16.0))
                                                        .bg(theme.tokens.card)
                                                        .rounded(theme.tokens.radius_lg)
                                                        .border_1()
                                                        .border_color(theme.tokens.border)
                                                        .child(
                                                            Countdown::new(
                                                                "countdown-5b",
                                                                self.countdown5.clone(),
                                                            )
                                                            .size(CountdownSize::Sm)
                                                            .separator(CountdownSeparator::Dash)
                                                            .format(CountdownFormat::time_only()),
                                                        ),
                                                ),
                                        )
                                        .child(
                                            VStack::new()
                                                .gap(px(8.0))
                                                .child(
                                                    div()
                                                        .text_size(px(12.0))
                                                        .text_color(theme.tokens.muted_foreground)
                                                        .child("No separator"),
                                                )
                                                .child(
                                                    div()
                                                        .p(px(16.0))
                                                        .bg(theme.tokens.card)
                                                        .rounded(theme.tokens.radius_lg)
                                                        .border_1()
                                                        .border_color(theme.tokens.border)
                                                        .child(
                                                            Countdown::new(
                                                                "countdown-5c",
                                                                self.countdown5.clone(),
                                                            )
                                                            .size(CountdownSize::Sm)
                                                            .separator(CountdownSeparator::None)
                                                            .format(CountdownFormat::time_only()),
                                                        ),
                                                ),
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
                                        .child("6. Short Countdown with Completion Callback"),
                                )
                                .child(
                                    div()
                                        .text_size(px(12.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child(format!("Status: {}", status6)),
                                )
                                .child(
                                    div()
                                        .p(px(24.0))
                                        .bg(theme.tokens.card)
                                        .rounded(theme.tokens.radius_lg)
                                        .border_1()
                                        .border_color(theme.tokens.border)
                                        .child(
                                            Countdown::new("countdown-6", self.countdown6.clone())
                                                .size(CountdownSize::Lg)
                                                .show_days(false)
                                                .show_hours(false)
                                                .on_complete(|_, _| {
                                                    println!("Countdown completed!");
                                                }),
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
                                        .child(
                                            "Features: Multiple sizes, configurable formats, separators, count up/down modes, completion callbacks",
                                        ),
                                )
                                .child(
                                    div()
                                        .mt(px(8.0))
                                        .text_size(px(12.0))
                                        .text_color(theme.tokens.accent_foreground)
                                        .child(
                                            "The countdown automatically updates every second using cx.spawn() with background executor timer",
                                        ),
                                ),
                        ),
                ),
            )
    }
}
