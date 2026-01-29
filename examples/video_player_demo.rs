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

fn main() {
    Application::new()
        .with_assets(Assets {
            base: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
        })
        .run(|cx| {
            adabraka_ui::init(cx);
            adabraka_ui::set_icon_base_path("assets/icons");
            init_video_player(cx);
            install_theme(cx, Theme::dark());

            cx.open_window(
                WindowOptions {
                    titlebar: Some(TitlebarOptions {
                        title: Some("VideoPlayer Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(900.0), px(700.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|cx| VideoPlayerDemoApp::new(cx)),
            )
            .unwrap();
        });
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

        let state_for_timer = player_state.clone();
        cx.spawn(
            async move |_this, cx| {
                loop {
                    cx.background_executor().timer(Duration::from_millis(100)).await;

                    let should_continue = state_for_timer.update(cx, |state, cx| {
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
                    }).is_ok();

                    if !should_continue {
                        break;
                    }
                }
            }
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
            .flex()
            .flex_col()
            .p(px(32.0))
            .gap(px(24.0))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .child(
                        div()
                            .text_size(px(28.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(theme.tokens.foreground)
                            .child("VideoPlayer Component"),
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("A video player control overlay with full playback controls."),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(16.0))
                    .child(
                        VideoPlayer::new(self.player_state.clone())
                            .size(VideoPlayerSize::Md)
                            .on_play(|_, _| println!("Video: Play"))
                            .on_pause(|_, _| println!("Video: Pause"))
                            .on_seek(|time, _, _| println!("Video: Seek to {:.1}s", time))
                            .on_volume_change(|vol, _, _| println!("Video: Volume {:.0}%", vol * 100.0))
                            .on_fullscreen(|fs, _, _| println!("Video: Fullscreen {}", fs))
                            .on_playback_speed_change(|speed, _, _| {
                                println!("Video: Speed {}", speed.label())
                            }),
                    ),
            )
            .child(
                div()
                    .mt(px(16.0))
                    .p(px(16.0))
                    .bg(theme.tokens.accent)
                    .rounded(px(8.0))
                    .flex()
                    .flex_col()
                    .gap(px(12.0))
                    .child(
                        div()
                            .text_size(px(16.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(theme.tokens.accent_foreground)
                            .child("Integration Guide"),
                    )
                    .child(
                        div()
                            .text_size(px(13.0))
                            .text_color(theme.tokens.accent_foreground)
                            .child("The VideoPlayer provides UI controls. To play actual video:"),
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .font_family(theme.tokens.font_mono.clone())
                            .text_color(theme.tokens.accent_foreground)
                            .p(px(12.0))
                            .bg(theme.tokens.background.opacity(0.3))
                            .rounded(px(6.0))
                            .child(
r#"// 1. Create state
let video_state = cx.new(|cx| VideoPlayerState::new(cx));

// 2. Your video decoder updates our state
decoder.on_frame(|frame_path, time| {
    video_state.update(cx, |s, cx| {
        s.set_frame(frame_path, cx);  // We render the frame
        s.set_current_time(time, cx);
    });
});

// 3. Handle playback events
VideoPlayer::new(video_state)
    .on_play(|_, _| decoder.play())
    .on_pause(|_, _| decoder.pause())
    .on_seek(|time, _, _| decoder.seek(time))"#
                            ),
                    )
                    .child(
                        div()
                            .text_size(px(13.0))
                            .text_color(theme.tokens.accent_foreground)
                            .child("Or use .overlay_only() for just controls over your own video rendering."),
                    ),
            )
    }
}
