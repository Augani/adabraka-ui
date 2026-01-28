use adabraka_ui::{
    components::time_picker::{TimeFormat, TimePicker, TimePickerState, TimeValue},
    layout::VStack,
    theme::{install_theme, use_theme, Theme},
};
use gpui::{prelude::FluentBuilder as _, *};

struct TimePickerDemo {
    time_12h: Entity<TimePickerState>,
    time_24h: Entity<TimePickerState>,
    time_with_seconds: Entity<TimePickerState>,
    selected_times: [Option<TimeValue>; 3],
}

impl TimePickerDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        let time_12h = cx.new(|cx| {
            let mut state = TimePickerState::new(cx);
            state.set_format(TimeFormat::Hour12, cx);
            state
        });
        let time_24h = cx.new(|cx| TimePickerState::new(cx));
        let time_with_seconds = cx.new(|cx| {
            let mut state = TimePickerState::new(cx);
            state.set_show_seconds(true, cx);
            state
        });
        Self {
            time_12h,
            time_24h,
            time_with_seconds,
            selected_times: [None, None, None],
        }
    }
}

impl Render for TimePickerDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();
        let entity = cx.entity().clone();

        div()
            .size_full()
            .bg(theme.tokens.background)
            .text_color(theme.tokens.foreground)
            .font_family(theme.tokens.font_family.clone())
            .child(
                VStack::new()
                    .p(px(24.0))
                    .gap(px(24.0))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .text_size(px(32.0))
                                    .font_weight(FontWeight::BOLD)
                                    .child("Time Picker Demo"),
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Time selection with 12/24 hour formats and optional seconds"),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(24.0))
                            .child({
                                let entity = entity.clone();
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(14.0))
                                            .font_weight(FontWeight::MEDIUM)
                                            .child("12-Hour Format"),
                                    )
                                    .child(
                                        TimePicker::new(self.time_12h.clone())
                                            .on_change(move |value, _, cx| {
                                                entity.update(cx, |this, cx| {
                                                    this.selected_times[0] = Some(*value);
                                                    cx.notify();
                                                });
                                            })
                                    )
                                    .when_some(self.selected_times[0], |d, time| {
                                        d.child(
                                            div()
                                                .text_size(px(12.0))
                                                .text_color(theme.tokens.muted_foreground)
                                                .child(time.format_string(TimeFormat::Hour12)),
                                        )
                                    })
                            })
                            .child({
                                let entity = entity.clone();
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(14.0))
                                            .font_weight(FontWeight::MEDIUM)
                                            .child("24-Hour Format"),
                                    )
                                    .child(
                                        TimePicker::new(self.time_24h.clone())
                                            .on_change(move |value, _, cx| {
                                                entity.update(cx, |this, cx| {
                                                    this.selected_times[1] = Some(*value);
                                                    cx.notify();
                                                });
                                            })
                                    )
                                    .when_some(self.selected_times[1], |d, time| {
                                        d.child(
                                            div()
                                                .text_size(px(12.0))
                                                .text_color(theme.tokens.muted_foreground)
                                                .child(time.format_string(TimeFormat::Hour24)),
                                        )
                                    })
                            })
                            .child({
                                let entity = entity.clone();
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(14.0))
                                            .font_weight(FontWeight::MEDIUM)
                                            .child("With Seconds"),
                                    )
                                    .child(
                                        TimePicker::new(self.time_with_seconds.clone())
                                            .on_change(move |value, _, cx| {
                                                entity.update(cx, |this, cx| {
                                                    this.selected_times[2] = Some(*value);
                                                    cx.notify();
                                                });
                                            })
                                    )
                                    .when_some(self.selected_times[2], |d, time| {
                                        d.child(
                                            div()
                                                .text_size(px(12.0))
                                                .text_color(theme.tokens.muted_foreground)
                                                .child(time.format_string(TimeFormat::Hour12)),
                                        )
                                    })
                            }),
                    ),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        adabraka_ui::init(cx);
        install_theme(cx, Theme::dark());

        let bounds = Bounds::centered(None, size(px(500.0), px(500.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("Time Picker Demo".into()),
                    appears_transparent: false,
                    traffic_light_position: None,
                }),
                ..Default::default()
            },
            |_, cx| cx.new(|cx| TimePickerDemo::new(cx)),
        )
        .unwrap();

        cx.activate(true);
    });
}
