use adabraka_ui::{components::scrollable::scrollable_vertical, prelude::*};
use gpui::*;
use std::path::PathBuf;
use std::time::Duration;

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
                        title: Some("AudioPlayer Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(800.0), px(700.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|cx| AudioPlayerDemo::new(cx)),
            )
            .unwrap();
        });
}

struct AudioPlayerDemo {
    full_player: Entity<AudioPlayerState>,
    compact_player: Entity<AudioPlayerState>,
    disabled_player: Entity<AudioPlayerState>,
    custom_player: Entity<AudioPlayerState>,
}

impl AudioPlayerDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        let full_player = cx.new(|cx| {
            let mut state = AudioPlayerState::new(cx);
            state.set_duration(245.0, cx);
            state.set_current_time(0.0, cx);
            state
        });

        let compact_player = cx.new(|cx| {
            let mut state = AudioPlayerState::new(cx);
            state.set_duration(180.0, cx);
            state.set_current_time(0.0, cx);
            state
        });

        let disabled_player = cx.new(|cx| {
            let mut state = AudioPlayerState::new(cx);
            state.set_duration(120.0, cx);
            state.set_current_time(30.0, cx);
            state
        });

        let custom_player = cx.new(|cx| {
            let mut state = AudioPlayerState::new(cx);
            state.set_duration(360.0, cx);
            state.set_current_time(0.0, cx);
            state.set_volume(0.6, cx);
            state
        });

        cx.spawn(
            async | this,
            cx | {
                loop {
                    cx.background_executor()
                        .timer(Duration::from_millis(100))
                        .await;

                    let should_continue = this
                        .update(cx, |demo, cx| {
                            let players = [
                                demo.full_player.clone(),
                                demo.compact_player.clone(),
                                demo.custom_player.clone(),
                            ];
                            for player in &players {
                                player.update(cx, |state, cx| {
                                    if state.is_playing() {
                                        let speed = state.playback_speed().value();
                                        let new_time = state.current_time() + 0.1 * speed;
                                        if new_time >= state.duration() {
                                            state.set_current_time(0.0, cx);
                                            state.set_playing(false, cx);
                                        } else {
                                            state.set_current_time(new_time, cx);
                                        }
                                    }
                                });
                            }
                        })
                        .is_ok();

                    if !should_continue {
                        break;
                    }
                }
            },
        )
        .detach();

        Self {
            full_player,
            compact_player,
            disabled_player,
            custom_player,
        }
    }
}

impl Render for AudioPlayerDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

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
                                        .text_size(px(28.0))
                                        .font_weight(FontWeight::BOLD)
                                        .child("AudioPlayer Component"),
                                )
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child(
                                            "A versatile audio player component with playback controls, \
                                            progress bar, volume control, and playback speed options.",
                                        ),
                                ),
                        )
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("Full Size Player"),
                                )
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child(
                                            "Complete player with title, progress bar, volume control, \
                                            and playback speed.",
                                        ),
                                )
                                .child(
                                    div().max_w(px(500.0)).child(
                                        AudioPlayer::new(self.full_player.clone())
                                            .full()
                                            .title("Track Title - Artist Name")
                                            .on_play(|_, _| {
                                                println!("Play pressed");
                                            })
                                            .on_pause(|_, _| {
                                                println!("Pause pressed");
                                            })
                                            .on_seek(|time, _, _| {
                                                println!("Seek to: {:.1}s", time);
                                            })
                                            .on_volume_change(|volume, _, _| {
                                                println!("Volume: {:.0}%", volume * 100.0);
                                            })
                                            .on_speed_change(|speed, _, _| {
                                                println!("Speed: {}", speed.label());
                                            }),
                                    ),
                                ),
                        )
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("Compact Size Player"),
                                )
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child(
                                            "Minimal player with play button, progress bar, and time display.",
                                        ),
                                )
                                .child(
                                    div().max_w(px(400.0)).child(
                                        AudioPlayer::new(self.compact_player.clone())
                                            .compact()
                                            .on_play(|_, _| {
                                                println!("Compact: Play pressed");
                                            })
                                            .on_pause(|_, _| {
                                                println!("Compact: Pause pressed");
                                            }),
                                    ),
                                ),
                        )
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("Disabled State"),
                                )
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child("Player in disabled state with reduced opacity."),
                                )
                                .child(
                                    div().max_w(px(500.0)).child(
                                        AudioPlayer::new(self.disabled_player.clone())
                                            .full()
                                            .title("Disabled Track")
                                            .disabled(true),
                                    ),
                                ),
                        )
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("Custom Styled Player"),
                                )
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child(
                                            "Player with custom styling via Styled trait methods.",
                                        ),
                                )
                                .child(
                                    div().max_w(px(500.0)).child(
                                        AudioPlayer::new(self.custom_player.clone())
                                            .full()
                                            .title("Custom Styled Track")
                                            .bg(hsla(258.0 / 360.0, 0.90, 0.66, 0.15))
                                            .border_2()
                                            .border_color(hsla(258.0 / 360.0, 0.90, 0.66, 0.5))
                                            .rounded(px(16.0))
                                            .shadow_lg(),
                                    ),
                                ),
                        )
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("Multiple Compact Players"),
                                )
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child("Compact players work well in lists or playlists."),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .gap(px(8.0))
                                        .max_w(px(350.0))
                                        .child(
                                            AudioPlayer::new(self.compact_player.clone()).compact(),
                                        )
                                        .child(
                                            AudioPlayer::new(self.full_player.clone()).compact(),
                                        )
                                        .child(
                                            AudioPlayer::new(self.custom_player.clone()).compact(),
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
                                            "Note: This component provides UI controls for audio playback. \
                                            Actual audio playback requires integration with a system audio backend.",
                                        ),
                                )
                                .child(
                                    div()
                                        .mt(px(8.0))
                                        .text_size(px(12.0))
                                        .text_color(theme.tokens.accent_foreground)
                                        .child(
                                            "Features: Play/Pause, Progress seek, Volume control, \
                                            Mute toggle, Playback speed (0.5x, 1x, 1.5x, 2x)",
                                        ),
                                ),
                        ),
                ),
            )
    }
}
