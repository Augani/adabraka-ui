use adabraka_ui::prelude::*;
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

struct VideoPlayerDemoApp {
    player_state: Entity<VideoPlayerState>,
}

impl VideoPlayerDemoApp {
    fn new(cx: &mut Context<Self>) -> Self {
        let player_state = cx.new(|cx| {
            let mut state = VideoPlayerState::new(cx);
            state.set_duration(180.0, cx);
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
                            demo.player_state.update(cx, |state, cx| {
                                if state.is_playing() {
                                    let speed = state.playback_speed().multiplier() as f64;
                                    let new_time = state.current_time() + 0.1 * speed;
                                    if new_time >= state.duration() {
                                        state.set_current_time(0.0, cx);
                                        state.pause(cx);
                                    } else {
                                        state.set_current_time(new_time, cx);
                                    }
                                }
                                state.check_auto_hide(cx);
                            });
                        })
                        .is_ok();

                    if !should_continue {
                        break;
                    }
                }
            },
        )
        .detach();

        Self { player_state }
    }
}

impl Render for VideoPlayerDemoApp {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        div()
            .size_full()
            .bg(theme.tokens.background)
            .p(px(40.0))
            .flex()
            .flex_col()
            .gap(px(32.0))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .child(h1("Video Player Component"))
                    .child(muted(
                        "A full-featured video player UI with controls overlay, progress bar, volume, and playback speed.",
                    )),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(24.0))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(12.0))
                            .child(h3("Default Size with Poster"))
                            .child(
                                VideoPlayer::new(self.player_state.clone())
                                    .size(VideoPlayerSize::Md)
                                    .poster("assets/images/carousel_1.jpg")
                                    .on_play(|_, _| {
                                        println!("Video started playing");
                                    })
                                    .on_pause(|_, _| {
                                        println!("Video paused");
                                    })
                                    .on_seek(|time, _, _| {
                                        println!("Seeked to: {:.1}s", time);
                                    })
                                    .on_volume_change(|vol, _, _| {
                                        println!("Volume changed to: {:.0}%", vol * 100.0);
                                    })
                                    .on_fullscreen(|is_fs, _, _| {
                                        println!("Fullscreen: {}", is_fs);
                                    })
                                    .on_playback_speed_change(|speed, _, _| {
                                        println!("Speed changed to: {}", speed.label());
                                    }),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .gap(px(16.0))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.0))
                                    .child(label("Small Player"))
                                    .child(
                                        VideoPlayer::new(self.player_state.clone())
                                            .size(VideoPlayerSize::Sm)
                                            .poster("assets/images/carousel_2.jpg"),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(12.0))
                            .child(h3("Keyboard Shortcuts"))
                            .child(
                                div()
                                    .flex()
                                    .flex_wrap()
                                    .gap(px(12.0))
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap(px(4.0))
                                            .child(Badge::new("Space").variant(BadgeVariant::Secondary))
                                            .child(muted("Play/Pause")),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap(px(4.0))
                                            .child(Badge::new("M").variant(BadgeVariant::Secondary))
                                            .child(muted("Mute")),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap(px(4.0))
                                            .child(Badge::new("F").variant(BadgeVariant::Secondary))
                                            .child(muted("Fullscreen")),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap(px(4.0))
                                            .child(Badge::new("Left/Right").variant(BadgeVariant::Secondary))
                                            .child(muted("Seek 10s")),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap(px(4.0))
                                            .child(Badge::new("Up/Down").variant(BadgeVariant::Secondary))
                                            .child(muted("Volume")),
                                    ),
                            ),
                    ),
            )
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
            init_video_player(cx);

            let bounds = Bounds::centered(None, size(px(900.0), px(800.0)), cx);
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                |_, cx| cx.new(VideoPlayerDemoApp::new),
            )
            .unwrap();
        });
}
