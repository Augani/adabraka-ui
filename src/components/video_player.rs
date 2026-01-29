use crate::theme::use_theme;
use gpui::{prelude::FluentBuilder as _, *};
use std::rc::Rc;
use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VideoPlaybackState {
    Stopped,
    Playing,
    Paused,
    Buffering,
}

impl Default for VideoPlaybackState {
    fn default() -> Self {
        Self::Stopped
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VideoPlayerSize {
    Sm,
    Md,
    Lg,
    Full,
}

impl Default for VideoPlayerSize {
    fn default() -> Self {
        Self::Md
    }
}

impl VideoPlayerSize {
    pub fn dimensions(&self) -> (Pixels, Pixels) {
        match self {
            Self::Sm => (px(400.0), px(225.0)),
            Self::Md => (px(640.0), px(360.0)),
            Self::Lg => (px(854.0), px(480.0)),
            Self::Full => (px(1280.0), px(720.0)),
        }
    }

    pub fn controls_height(&self) -> Pixels {
        match self {
            Self::Sm => px(36.0),
            Self::Md => px(44.0),
            Self::Lg => px(52.0),
            Self::Full => px(56.0),
        }
    }

    pub fn icon_size(&self) -> Pixels {
        match self {
            Self::Sm => px(16.0),
            Self::Md => px(20.0),
            Self::Lg => px(24.0),
            Self::Full => px(28.0),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VideoPlaybackSpeed {
    Quarter,
    Half,
    ThreeQuarter,
    Normal,
    OneAndQuarter,
    OneAndHalf,
    Double,
}

impl Default for VideoPlaybackSpeed {
    fn default() -> Self {
        Self::Normal
    }
}

impl VideoPlaybackSpeed {
    pub fn multiplier(&self) -> f32 {
        match self {
            Self::Quarter => 0.25,
            Self::Half => 0.5,
            Self::ThreeQuarter => 0.75,
            Self::Normal => 1.0,
            Self::OneAndQuarter => 1.25,
            Self::OneAndHalf => 1.5,
            Self::Double => 2.0,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Quarter => "0.25x",
            Self::Half => "0.5x",
            Self::ThreeQuarter => "0.75x",
            Self::Normal => "1x",
            Self::OneAndQuarter => "1.25x",
            Self::OneAndHalf => "1.5x",
            Self::Double => "2x",
        }
    }

    pub fn next(&self) -> Self {
        match self {
            Self::Quarter => Self::Half,
            Self::Half => Self::ThreeQuarter,
            Self::ThreeQuarter => Self::Normal,
            Self::Normal => Self::OneAndQuarter,
            Self::OneAndQuarter => Self::OneAndHalf,
            Self::OneAndHalf => Self::Double,
            Self::Double => Self::Quarter,
        }
    }

    pub fn all() -> &'static [VideoPlaybackSpeed] {
        &[
            Self::Quarter,
            Self::Half,
            Self::ThreeQuarter,
            Self::Normal,
            Self::OneAndQuarter,
            Self::OneAndHalf,
            Self::Double,
        ]
    }
}

pub struct VideoPlayerState {
    playback_state: VideoPlaybackState,
    current_time: f64,
    duration: f64,
    volume: f32,
    is_muted: bool,
    previous_volume: f32,
    playback_speed: VideoPlaybackSpeed,
    is_fullscreen: bool,
    show_controls: bool,
    last_interaction: Instant,
    controls_timeout: Duration,
    is_seeking: bool,
    progress_bounds: Bounds<Pixels>,
    volume_bounds: Bounds<Pixels>,
    is_volume_dragging: bool,
    show_speed_menu: bool,
    focus_handle: FocusHandle,
    current_frame: Option<SharedString>,
    video_title: Option<SharedString>,
}

impl VideoPlayerState {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            playback_state: VideoPlaybackState::Stopped,
            current_time: 0.0,
            duration: 0.0,
            volume: 1.0,
            is_muted: false,
            previous_volume: 1.0,
            playback_speed: VideoPlaybackSpeed::Normal,
            is_fullscreen: false,
            show_controls: true,
            last_interaction: Instant::now(),
            controls_timeout: Duration::from_secs(3),
            is_seeking: false,
            progress_bounds: Bounds::default(),
            volume_bounds: Bounds::default(),
            is_volume_dragging: false,
            show_speed_menu: false,
            focus_handle: cx.focus_handle(),
            current_frame: None,
            video_title: None,
        }
    }

    pub fn set_frame(&mut self, frame_path: impl Into<SharedString>, cx: &mut Context<Self>) {
        self.current_frame = Some(frame_path.into());
        cx.notify();
    }

    pub fn clear_frame(&mut self, cx: &mut Context<Self>) {
        self.current_frame = None;
        cx.notify();
    }

    pub fn current_frame(&self) -> Option<&SharedString> {
        self.current_frame.as_ref()
    }

    pub fn set_title(&mut self, title: impl Into<SharedString>, cx: &mut Context<Self>) {
        self.video_title = Some(title.into());
        cx.notify();
    }

    pub fn title(&self) -> Option<&SharedString> {
        self.video_title.as_ref()
    }

    pub fn playback_state(&self) -> VideoPlaybackState {
        self.playback_state
    }

    pub fn is_playing(&self) -> bool {
        self.playback_state == VideoPlaybackState::Playing
    }

    pub fn play(&mut self, cx: &mut Context<Self>) {
        self.playback_state = VideoPlaybackState::Playing;
        self.touch_controls(cx);
        cx.notify();
    }

    pub fn pause(&mut self, cx: &mut Context<Self>) {
        self.playback_state = VideoPlaybackState::Paused;
        self.show_controls = true;
        cx.notify();
    }

    pub fn stop(&mut self, cx: &mut Context<Self>) {
        self.playback_state = VideoPlaybackState::Stopped;
        self.current_time = 0.0;
        self.show_controls = true;
        cx.notify();
    }

    pub fn toggle_play(&mut self, cx: &mut Context<Self>) {
        match self.playback_state {
            VideoPlaybackState::Playing => self.pause(cx),
            VideoPlaybackState::Paused | VideoPlaybackState::Stopped => self.play(cx),
            VideoPlaybackState::Buffering => {}
        }
    }

    pub fn current_time(&self) -> f64 {
        self.current_time
    }

    pub fn set_current_time(&mut self, time: f64, cx: &mut Context<Self>) {
        self.current_time = time.clamp(0.0, self.duration);
        cx.notify();
    }

    pub fn duration(&self) -> f64 {
        self.duration
    }

    pub fn set_duration(&mut self, duration: f64, cx: &mut Context<Self>) {
        self.duration = duration.max(0.0);
        cx.notify();
    }

    pub fn progress(&self) -> f64 {
        if self.duration <= 0.0 {
            return 0.0;
        }
        (self.current_time / self.duration).clamp(0.0, 1.0)
    }

    pub fn seek(&mut self, position: f64, cx: &mut Context<Self>) {
        let clamped = position.clamp(0.0, 1.0);
        self.current_time = clamped * self.duration;
        self.touch_controls(cx);
        cx.notify();
    }

    pub fn seek_relative(&mut self, delta: f64, cx: &mut Context<Self>) {
        let new_time = (self.current_time + delta).clamp(0.0, self.duration);
        self.current_time = new_time;
        self.touch_controls(cx);
        cx.notify();
    }

    pub fn volume(&self) -> f32 {
        self.volume
    }

    pub fn effective_volume(&self) -> f32 {
        if self.is_muted {
            0.0
        } else {
            self.volume
        }
    }

    pub fn set_volume(&mut self, volume: f32, cx: &mut Context<Self>) {
        self.volume = volume.clamp(0.0, 1.0);
        if self.volume > 0.0 {
            self.is_muted = false;
        }
        self.touch_controls(cx);
        cx.notify();
    }

    pub fn is_muted(&self) -> bool {
        self.is_muted
    }

    pub fn toggle_mute(&mut self, cx: &mut Context<Self>) {
        if self.is_muted {
            self.is_muted = false;
            if self.volume == 0.0 {
                self.volume = self.previous_volume;
            }
        } else {
            self.previous_volume = self.volume;
            self.is_muted = true;
        }
        self.touch_controls(cx);
        cx.notify();
    }

    pub fn playback_speed(&self) -> VideoPlaybackSpeed {
        self.playback_speed
    }

    pub fn set_playback_speed(&mut self, speed: VideoPlaybackSpeed, cx: &mut Context<Self>) {
        self.playback_speed = speed;
        self.show_speed_menu = false;
        self.touch_controls(cx);
        cx.notify();
    }

    pub fn cycle_playback_speed(&mut self, cx: &mut Context<Self>) {
        self.playback_speed = self.playback_speed.next();
        self.touch_controls(cx);
        cx.notify();
    }

    pub fn is_fullscreen(&self) -> bool {
        self.is_fullscreen
    }

    pub fn toggle_fullscreen(&mut self, cx: &mut Context<Self>) {
        self.is_fullscreen = !self.is_fullscreen;
        self.touch_controls(cx);
        cx.notify();
    }

    pub fn show_controls(&self) -> bool {
        self.show_controls
    }

    pub fn touch_controls(&mut self, cx: &mut Context<Self>) {
        self.show_controls = true;
        self.last_interaction = Instant::now();
        cx.notify();
    }

    pub fn hide_controls(&mut self, cx: &mut Context<Self>) {
        if self.playback_state == VideoPlaybackState::Playing
            && !self.is_seeking
            && !self.is_volume_dragging
        {
            self.show_controls = false;
            cx.notify();
        }
    }

    pub fn check_auto_hide(&mut self, cx: &mut Context<Self>) {
        if self.last_interaction.elapsed() > self.controls_timeout {
            self.hide_controls(cx);
        }
    }

    pub fn toggle_speed_menu(&mut self, cx: &mut Context<Self>) {
        self.show_speed_menu = !self.show_speed_menu;
        self.touch_controls(cx);
        cx.notify();
    }

    fn update_seek_from_position(&mut self, position: Point<Pixels>, cx: &mut Context<Self>) {
        let track_width = self.progress_bounds.size.width;
        if track_width <= px(0.0) {
            return;
        }

        let relative_x = (position.x - self.progress_bounds.left()).clamp(px(0.0), track_width);
        let percentage = (relative_x / track_width).clamp(0.0, 1.0);
        self.seek(percentage as f64, cx);
    }

    fn update_volume_from_position(&mut self, position: Point<Pixels>, cx: &mut Context<Self>) {
        let track_width = self.volume_bounds.size.width;
        if track_width <= px(0.0) {
            return;
        }

        let relative_x = (position.x - self.volume_bounds.left()).clamp(px(0.0), track_width);
        let percentage = (relative_x / track_width).clamp(0.0, 1.0);
        self.set_volume(percentage as f32, cx);
    }
}

impl Focusable for VideoPlayerState {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for VideoPlayerState {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}

actions!(
    video_player,
    [
        VideoPlayerTogglePlay,
        VideoPlayerMute,
        VideoPlayerFullscreen,
        VideoPlayerSeekForward,
        VideoPlayerSeekBackward,
        VideoPlayerVolumeUp,
        VideoPlayerVolumeDown,
    ]
);

pub fn init_video_player(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("space", VideoPlayerTogglePlay, Some("VideoPlayer")),
        KeyBinding::new("m", VideoPlayerMute, Some("VideoPlayer")),
        KeyBinding::new("f", VideoPlayerFullscreen, Some("VideoPlayer")),
        KeyBinding::new("right", VideoPlayerSeekForward, Some("VideoPlayer")),
        KeyBinding::new("left", VideoPlayerSeekBackward, Some("VideoPlayer")),
        KeyBinding::new("up", VideoPlayerVolumeUp, Some("VideoPlayer")),
        KeyBinding::new("down", VideoPlayerVolumeDown, Some("VideoPlayer")),
    ]);
}

fn format_time(seconds: f64) -> String {
    let total_seconds = seconds.floor() as i64;
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let secs = total_seconds % 60;

    if hours > 0 {
        format!("{}:{:02}:{:02}", hours, minutes, secs)
    } else {
        format!("{}:{:02}", minutes, secs)
    }
}

#[derive(IntoElement)]
pub struct VideoPlayer {
    state: Entity<VideoPlayerState>,
    size: VideoPlayerSize,
    poster: Option<SharedString>,
    show_poster: bool,
    overlay_only: bool,
    on_play: Option<Rc<dyn Fn(&mut Window, &mut App)>>,
    on_pause: Option<Rc<dyn Fn(&mut Window, &mut App)>>,
    on_seek: Option<Rc<dyn Fn(f64, &mut Window, &mut App)>>,
    on_volume_change: Option<Rc<dyn Fn(f32, &mut Window, &mut App)>>,
    on_fullscreen: Option<Rc<dyn Fn(bool, &mut Window, &mut App)>>,
    on_playback_speed_change: Option<Rc<dyn Fn(VideoPlaybackSpeed, &mut Window, &mut App)>>,
    style: StyleRefinement,
}

impl VideoPlayer {
    pub fn new(state: Entity<VideoPlayerState>) -> Self {
        Self {
            state,
            size: VideoPlayerSize::default(),
            poster: None,
            show_poster: true,
            overlay_only: false,
            on_play: None,
            on_pause: None,
            on_seek: None,
            on_volume_change: None,
            on_fullscreen: None,
            on_playback_speed_change: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn size(mut self, size: VideoPlayerSize) -> Self {
        self.size = size;
        self
    }

    pub fn poster(mut self, poster: impl Into<SharedString>) -> Self {
        self.poster = Some(poster.into());
        self
    }

    pub fn show_poster(mut self, show: bool) -> Self {
        self.show_poster = show;
        self
    }

    pub fn overlay_only(mut self) -> Self {
        self.overlay_only = true;
        self
    }

    pub fn on_play(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_play = Some(Rc::new(handler));
        self
    }

    pub fn on_pause(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_pause = Some(Rc::new(handler));
        self
    }

    pub fn on_seek(mut self, handler: impl Fn(f64, &mut Window, &mut App) + 'static) -> Self {
        self.on_seek = Some(Rc::new(handler));
        self
    }

    pub fn on_volume_change(
        mut self,
        handler: impl Fn(f32, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_volume_change = Some(Rc::new(handler));
        self
    }

    pub fn on_fullscreen(
        mut self,
        handler: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_fullscreen = Some(Rc::new(handler));
        self
    }

    pub fn on_playback_speed_change(
        mut self,
        handler: impl Fn(VideoPlaybackSpeed, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_playback_speed_change = Some(Rc::new(handler));
        self
    }
}

impl Styled for VideoPlayer {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for VideoPlayer {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let state = self.state.read(cx);
        let focus_handle = state.focus_handle(cx);

        let playback_state = state.playback_state();
        let is_playing = state.is_playing();
        let current_time = state.current_time();
        let duration = state.duration();
        let progress = state.progress();
        let volume = state.volume();
        let is_muted = state.is_muted();
        let playback_speed = state.playback_speed();
        let is_fullscreen = state.is_fullscreen();
        let show_controls = state.show_controls();
        let show_speed_menu = state.show_speed_menu;

        let (width, height) = self.size.dimensions();
        let controls_height = self.size.controls_height();
        let icon_size = self.size.icon_size();

        let show_poster = self.show_poster
            && self.poster.is_some()
            && playback_state == VideoPlaybackState::Stopped;

        let current_frame = state.current_frame().cloned();
        let overlay_only = self.overlay_only;

        let user_style = self.style;

        let volume_icon = if is_muted || volume == 0.0 {
            "volume-x"
        } else if volume < 0.5 {
            "volume-1"
        } else {
            "volume-2"
        };

        let play_icon = if is_playing { "pause" } else { "play" };

        let state_entity = self.state.clone();
        let state_for_actions = self.state.clone();
        let state_for_mouse = self.state.clone();

        let on_play = self.on_play.clone();
        let on_pause = self.on_pause.clone();
        let on_seek = self.on_seek.clone();
        let on_volume_change = self.on_volume_change.clone();
        let on_fullscreen = self.on_fullscreen.clone();
        let on_playback_speed_change = self.on_playback_speed_change.clone();

        div()
            .id("video-player")
            .key_context("VideoPlayer")
            .track_focus(&focus_handle)
            .relative()
            .w(width)
            .h(height)
            .bg(gpui::black())
            .rounded(theme.tokens.radius_lg)
            .overflow_hidden()
            .cursor(CursorStyle::Arrow)
            .map(|this| {
                let mut div = this;
                div.style().refine(&user_style);
                div
            })
            .on_action({
                let state = state_for_actions.clone();
                let on_play = on_play.clone();
                let on_pause = on_pause.clone();
                move |_: &VideoPlayerTogglePlay, window, cx| {
                    let is_playing = state.read(cx).is_playing();
                    cx.update_entity(&state, |state, cx| state.toggle_play(cx));
                    if is_playing {
                        if let Some(handler) = &on_pause {
                            handler(window, cx);
                        }
                    } else if let Some(handler) = &on_play {
                        handler(window, cx);
                    }
                }
            })
            .on_action({
                let state = state_for_actions.clone();
                move |_: &VideoPlayerMute, _, cx| {
                    cx.update_entity(&state, |state, cx| state.toggle_mute(cx));
                }
            })
            .on_action({
                let state = state_for_actions.clone();
                let on_fullscreen = on_fullscreen.clone();
                move |_: &VideoPlayerFullscreen, window, cx| {
                    cx.update_entity(&state, |state, cx| state.toggle_fullscreen(cx));
                    let is_fullscreen = state.read(cx).is_fullscreen();
                    if let Some(handler) = &on_fullscreen {
                        handler(is_fullscreen, window, cx);
                    }
                }
            })
            .on_action({
                let state = state_for_actions.clone();
                let on_seek = on_seek.clone();
                move |_: &VideoPlayerSeekForward, window, cx| {
                    cx.update_entity(&state, |state, cx| state.seek_relative(10.0, cx));
                    if let Some(handler) = &on_seek {
                        handler(state.read(cx).current_time(), window, cx);
                    }
                }
            })
            .on_action({
                let state = state_for_actions.clone();
                let on_seek = on_seek.clone();
                move |_: &VideoPlayerSeekBackward, window, cx| {
                    cx.update_entity(&state, |state, cx| state.seek_relative(-10.0, cx));
                    if let Some(handler) = &on_seek {
                        handler(state.read(cx).current_time(), window, cx);
                    }
                }
            })
            .on_action({
                let state = state_for_actions.clone();
                let on_volume_change = on_volume_change.clone();
                move |_: &VideoPlayerVolumeUp, window, cx| {
                    let current = state.read(cx).volume();
                    let new_volume = (current + 0.1).min(1.0);
                    cx.update_entity(&state, |state, cx| state.set_volume(new_volume, cx));
                    if let Some(handler) = &on_volume_change {
                        handler(new_volume, window, cx);
                    }
                }
            })
            .on_action({
                let state = state_for_actions.clone();
                let on_volume_change = on_volume_change.clone();
                move |_: &VideoPlayerVolumeDown, window, cx| {
                    let current = state.read(cx).volume();
                    let new_volume = (current - 0.1).max(0.0);
                    cx.update_entity(&state, |state, cx| state.set_volume(new_volume, cx));
                    if let Some(handler) = &on_volume_change {
                        handler(new_volume, window, cx);
                    }
                }
            })
            .on_mouse_move({
                let state = state_for_mouse.clone();
                window.listener_for(&state, move |state, _: &MouseMoveEvent, _, cx| {
                    state.touch_controls(cx);
                })
            })
            .when(show_poster, {
                let poster = self.poster.clone();
                move |this| {
                    this.child(
                        div()
                            .absolute()
                            .inset_0()
                            .flex()
                            .items_center()
                            .justify_center()
                            .when_some(poster, |this, poster_src| {
                                this.child(
                                    img(poster_src)
                                        .size_full()
                                        .object_fit(ObjectFit::Cover)
                                )
                            })
                    )
                }
            })
            .when(!show_poster && !overlay_only, |this| {
                if let Some(ref frame) = current_frame {
                    this.child(
                        div()
                            .absolute()
                            .inset_0()
                            .child(
                                img(frame.clone())
                                    .size_full()
                                    .object_fit(ObjectFit::Contain)
                            )
                    )
                } else {
                    this.child(
                        div()
                            .absolute()
                            .inset_0()
                            .bg(gpui::black())
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_color(gpui::white().opacity(0.5))
                            .text_sm()
                            .font_family(theme.tokens.font_family.clone())
                            .child("Video content area")
                    )
                }
            })
            .child({
                let state_play = state_entity.clone();
                let on_play_center = on_play.clone();
                let on_pause_center = on_pause.clone();

                div()
                    .id("center-play-button")
                    .absolute()
                    .inset_0()
                    .flex()
                    .items_center()
                    .justify_center()
                    .when(show_controls || !is_playing, |this| {
                        this.child(
                            div()
                                .id("play-overlay")
                                .size(px(72.0))
                                .rounded_full()
                                .bg(gpui::black().opacity(0.6))
                                .border_2()
                                .border_color(gpui::white().opacity(0.3))
                                .flex()
                                .items_center()
                                .justify_center()
                                .cursor(CursorStyle::PointingHand)
                                .hover(|style| style.bg(gpui::black().opacity(0.8)))
                                .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                                    let is_playing_now = state_play.read(cx).is_playing();
                                    cx.update_entity(&state_play, |state, cx| state.toggle_play(cx));
                                    if is_playing_now {
                                        if let Some(handler) = &on_pause_center {
                                            handler(window, cx);
                                        }
                                    } else if let Some(handler) = &on_play_center {
                                        handler(window, cx);
                                    }
                                })
                                .child(
                                    svg()
                                        .path(format!("icons/{}.svg", play_icon))
                                        .size(px(32.0))
                                        .text_color(gpui::white())
                                )
                        )
                    })
            })
            .when(show_controls, |this| {
                let state_progress = state_entity.clone();
                let state_progress_drag = state_entity.clone();
                let state_progress_move = state_entity.clone();
                let state_progress_up = state_entity.clone();
                let on_seek_progress = on_seek.clone();
                let on_seek_drag = on_seek.clone();

                let state_volume = state_entity.clone();
                let state_volume_icon = state_entity.clone();
                let state_volume_drag = state_entity.clone();
                let state_volume_move = state_entity.clone();
                let state_volume_up = state_entity.clone();
                let on_volume_slider = on_volume_change.clone();
                let on_volume_drag_change = on_volume_change.clone();

                let state_play_btn = state_entity.clone();
                let on_play_btn = on_play.clone();
                let on_pause_btn = on_pause.clone();

                let state_skip_back = state_entity.clone();
                let on_seek_back = on_seek.clone();

                let state_skip_forward = state_entity.clone();
                let on_seek_forward = on_seek.clone();

                let state_speed = state_entity.clone();
                let state_speed_item = state_entity.clone();
                let on_speed_change = on_playback_speed_change.clone();

                let state_fullscreen = state_entity.clone();
                let on_fullscreen_btn = on_fullscreen.clone();

                this.child(
                    div()
                        .id("controls-overlay")
                        .absolute()
                        .bottom_0()
                        .left_0()
                        .right_0()
                        .h(controls_height + px(40.0))
                        .bg(Hsla::from(gpui::black()).opacity(0.7))
                        .flex()
                        .flex_col()
                        .justify_end()
                        .child(
                            div()
                                .id("progress-bar-container")
                                .px(px(12.0))
                                .pb(px(4.0))
                                .child(
                                    div()
                                        .id("progress-bar")
                                        .relative()
                                        .h(px(6.0))
                                        .w_full()
                                        .bg(gpui::white().opacity(0.3))
                                        .rounded_full()
                                        .cursor(CursorStyle::PointingHand)
                                        .child(
                                            canvas(
                                                {
                                                    let state = state_progress.clone();
                                                    move |bounds, _, cx| {
                                                        state.update(cx, |state, _| {
                                                            state.progress_bounds = bounds;
                                                        });
                                                    }
                                                },
                                                |_, _, _, _| {},
                                            )
                                            .absolute()
                                            .size_full(),
                                        )
                                        .child(
                                            div()
                                                .absolute()
                                                .left_0()
                                                .top_0()
                                                .h_full()
                                                .w(relative(progress as f32))
                                                .bg(theme.tokens.primary)
                                                .rounded_full()
                                        )
                                        .child(
                                            div()
                                                .absolute()
                                                .left(relative(progress as f32))
                                                .top(px(-3.0))
                                                .ml(px(-6.0))
                                                .size(px(12.0))
                                                .rounded_full()
                                                .bg(theme.tokens.primary)
                                                .border_2()
                                                .border_color(gpui::white())
                                        )
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            window.listener_for(
                                                &state_progress_drag,
                                                {
                                                    let on_seek = on_seek_progress.clone();
                                                    move |state, e: &MouseDownEvent, window, cx| {
                                                        state.is_seeking = true;
                                                        state.update_seek_from_position(e.position, cx);
                                                        if let Some(handler) = &on_seek {
                                                            handler(state.current_time, window, cx);
                                                        }
                                                    }
                                                },
                                            ),
                                        )
                                        .on_mouse_move(
                                            window.listener_for(
                                                &state_progress_move,
                                                {
                                                    let on_seek = on_seek_drag.clone();
                                                    move |state, e: &MouseMoveEvent, window, cx| {
                                                        if state.is_seeking {
                                                            state.update_seek_from_position(e.position, cx);
                                                            if let Some(handler) = &on_seek {
                                                                handler(state.current_time, window, cx);
                                                            }
                                                        }
                                                    }
                                                },
                                            ),
                                        )
                                        .on_mouse_up(
                                            MouseButton::Left,
                                            window.listener_for(
                                                &state_progress_up,
                                                move |state, _: &MouseUpEvent, _, _| {
                                                    state.is_seeking = false;
                                                },
                                            ),
                                        )
                                )
                        )
                        .child(
                            div()
                                .id("controls-bar")
                                .h(controls_height)
                                .px(px(12.0))
                                .flex()
                                .items_center()
                                .justify_between()
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap(px(8.0))
                                        .child(
                                            div()
                                                .id("skip-back-btn")
                                                .size(px(32.0))
                                                .rounded(theme.tokens.radius_md)
                                                .flex()
                                                .items_center()
                                                .justify_center()
                                                .cursor(CursorStyle::PointingHand)
                                                .hover(|style| style.bg(gpui::white().opacity(0.2)))
                                                .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                                                    cx.update_entity(&state_skip_back, |state, cx| {
                                                        state.seek_relative(-10.0, cx);
                                                    });
                                                    if let Some(handler) = &on_seek_back {
                                                        handler(state_skip_back.read(cx).current_time(), window, cx);
                                                    }
                                                })
                                                .child(
                                                    svg()
                                                        .path("icons/rewind.svg")
                                                        .size(icon_size)
                                                        .text_color(gpui::white())
                                                )
                                        )
                                        .child(
                                            div()
                                                .id("play-pause-btn")
                                                .size(px(40.0))
                                                .rounded_full()
                                                .bg(theme.tokens.primary)
                                                .flex()
                                                .items_center()
                                                .justify_center()
                                                .cursor(CursorStyle::PointingHand)
                                                .hover(|style| style.opacity(0.9))
                                                .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                                                    let is_playing_now = state_play_btn.read(cx).is_playing();
                                                    cx.update_entity(&state_play_btn, |state, cx| state.toggle_play(cx));
                                                    if is_playing_now {
                                                        if let Some(handler) = &on_pause_btn {
                                                            handler(window, cx);
                                                        }
                                                    } else if let Some(handler) = &on_play_btn {
                                                        handler(window, cx);
                                                    }
                                                })
                                                .child(
                                                    svg()
                                                        .path(format!("icons/{}.svg", play_icon))
                                                        .size(icon_size)
                                                        .text_color(theme.tokens.primary_foreground)
                                                )
                                        )
                                        .child(
                                            div()
                                                .id("skip-forward-btn")
                                                .size(px(32.0))
                                                .rounded(theme.tokens.radius_md)
                                                .flex()
                                                .items_center()
                                                .justify_center()
                                                .cursor(CursorStyle::PointingHand)
                                                .hover(|style| style.bg(gpui::white().opacity(0.2)))
                                                .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                                                    cx.update_entity(&state_skip_forward, |state, cx| {
                                                        state.seek_relative(10.0, cx);
                                                    });
                                                    if let Some(handler) = &on_seek_forward {
                                                        handler(state_skip_forward.read(cx).current_time(), window, cx);
                                                    }
                                                })
                                                .child(
                                                    svg()
                                                        .path("icons/fast-forward.svg")
                                                        .size(icon_size)
                                                        .text_color(gpui::white())
                                                )
                                        )
                                        .child(
                                            div()
                                                .ml(px(8.0))
                                                .text_sm()
                                                .text_color(gpui::white())
                                                .font_family(theme.tokens.font_family.clone())
                                                .child(format!("{} / {}", format_time(current_time), format_time(duration)))
                                        )
                                )
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap(px(8.0))
                                        .child(
                                            div()
                                                .flex()
                                                .items_center()
                                                .gap(px(4.0))
                                                .child(
                                                    div()
                                                        .id("volume-btn")
                                                        .size(px(32.0))
                                                        .rounded(theme.tokens.radius_md)
                                                        .flex()
                                                        .items_center()
                                                        .justify_center()
                                                        .cursor(CursorStyle::PointingHand)
                                                        .hover(|style| style.bg(gpui::white().opacity(0.2)))
                                                        .on_mouse_down(MouseButton::Left, move |_, _, cx| {
                                                            cx.update_entity(&state_volume_icon, |state, cx| state.toggle_mute(cx));
                                                        })
                                                        .child(
                                                            svg()
                                                                .path(format!("icons/{}.svg", volume_icon))
                                                                .size(icon_size)
                                                                .text_color(gpui::white())
                                                        )
                                                )
                                                .child(
                                                    div()
                                                        .id("volume-slider")
                                                        .relative()
                                                        .w(px(80.0))
                                                        .h(px(4.0))
                                                        .bg(gpui::white().opacity(0.3))
                                                        .rounded_full()
                                                        .cursor(CursorStyle::PointingHand)
                                                        .child(
                                                            canvas(
                                                                {
                                                                    let state = state_volume.clone();
                                                                    move |bounds, _, cx| {
                                                                        state.update(cx, |state, _| {
                                                                            state.volume_bounds = bounds;
                                                                        });
                                                                    }
                                                                },
                                                                |_, _, _, _| {},
                                                            )
                                                            .absolute()
                                                            .size_full(),
                                                        )
                                                        .child(
                                                            div()
                                                                .absolute()
                                                                .left_0()
                                                                .top_0()
                                                                .h_full()
                                                                .w(relative(volume))
                                                                .bg(gpui::white())
                                                                .rounded_full()
                                                        )
                                                        .child(
                                                            div()
                                                                .absolute()
                                                                .left(relative(volume))
                                                                .top(px(-4.0))
                                                                .ml(px(-6.0))
                                                                .size(px(12.0))
                                                                .rounded_full()
                                                                .bg(gpui::white())
                                                        )
                                                        .on_mouse_down(
                                                            MouseButton::Left,
                                                            window.listener_for(
                                                                &state_volume_drag,
                                                                {
                                                                    let on_volume = on_volume_slider.clone();
                                                                    move |state, e: &MouseDownEvent, window, cx| {
                                                                        state.is_volume_dragging = true;
                                                                        state.update_volume_from_position(e.position, cx);
                                                                        if let Some(handler) = &on_volume {
                                                                            handler(state.volume, window, cx);
                                                                        }
                                                                    }
                                                                },
                                                            ),
                                                        )
                                                        .on_mouse_move(
                                                            window.listener_for(
                                                                &state_volume_move,
                                                                {
                                                                    let on_volume = on_volume_drag_change.clone();
                                                                    move |state, e: &MouseMoveEvent, window, cx| {
                                                                        if state.is_volume_dragging {
                                                                            state.update_volume_from_position(e.position, cx);
                                                                            if let Some(handler) = &on_volume {
                                                                                handler(state.volume, window, cx);
                                                                            }
                                                                        }
                                                                    }
                                                                },
                                                            ),
                                                        )
                                                        .on_mouse_up(
                                                            MouseButton::Left,
                                                            window.listener_for(
                                                                &state_volume_up,
                                                                move |state, _: &MouseUpEvent, _, _| {
                                                                    state.is_volume_dragging = false;
                                                                },
                                                            ),
                                                        )
                                                )
                                        )
                                        .child(
                                            div()
                                                .id("speed-btn")
                                                .relative()
                                                .child(
                                                    div()
                                                        .px(px(8.0))
                                                        .py(px(4.0))
                                                        .rounded(theme.tokens.radius_md)
                                                        .text_xs()
                                                        .text_color(gpui::white())
                                                        .font_family(theme.tokens.font_family.clone())
                                                        .cursor(CursorStyle::PointingHand)
                                                        .hover(|style| style.bg(gpui::white().opacity(0.2)))
                                                        .on_mouse_down(MouseButton::Left, move |_, _, cx| {
                                                            cx.update_entity(&state_speed, |state, cx| state.toggle_speed_menu(cx));
                                                        })
                                                        .child(playback_speed.label())
                                                )
                                                .when(show_speed_menu, {
                                                    let theme = theme.clone();
                                                    move |this| {
                                                        this.child(
                                                            div()
                                                                .absolute()
                                                                .bottom(px(36.0))
                                                                .right_0()
                                                                .w(px(80.0))
                                                                .bg(theme.tokens.popover)
                                                                .rounded(theme.tokens.radius_md)
                                                                .border_1()
                                                                .border_color(theme.tokens.border)
                                                                .shadow(vec![theme.tokens.shadow_lg])
                                                                .py(px(4.0))
                                                                .children(
                                                                    VideoPlaybackSpeed::all().iter().map(|speed| {
                                                                        let state_item = state_speed_item.clone();
                                                                        let on_speed = on_speed_change.clone();
                                                                        let speed_val = *speed;
                                                                        let is_selected = speed_val == playback_speed;

                                                                        div()
                                                                            .id(ElementId::Name(format!("speed-{}", speed_val.label()).into()))
                                                                            .px(px(12.0))
                                                                            .py(px(6.0))
                                                                            .text_xs()
                                                                            .text_color(if is_selected {
                                                                                theme.tokens.primary
                                                                            } else {
                                                                                theme.tokens.popover_foreground
                                                                            })
                                                                            .font_family(theme.tokens.font_family.clone())
                                                                            .cursor(CursorStyle::PointingHand)
                                                                            .hover(|style| style.bg(theme.tokens.accent))
                                                                            .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                                                                                cx.update_entity(&state_item, |state, cx| {
                                                                                    state.set_playback_speed(speed_val, cx);
                                                                                });
                                                                                if let Some(handler) = &on_speed {
                                                                                    handler(speed_val, window, cx);
                                                                                }
                                                                            })
                                                                            .child(speed_val.label())
                                                                    })
                                                                )
                                                        )
                                                    }
                                                })
                                        )
                                        .child(
                                            div()
                                                .id("fullscreen-btn")
                                                .size(px(32.0))
                                                .rounded(theme.tokens.radius_md)
                                                .flex()
                                                .items_center()
                                                .justify_center()
                                                .cursor(CursorStyle::PointingHand)
                                                .hover(|style| style.bg(gpui::white().opacity(0.2)))
                                                .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                                                    cx.update_entity(&state_fullscreen, |state, cx| state.toggle_fullscreen(cx));
                                                    let is_fs = state_fullscreen.read(cx).is_fullscreen();
                                                    if let Some(handler) = &on_fullscreen_btn {
                                                        handler(is_fs, window, cx);
                                                    }
                                                })
                                                .child(
                                                    svg()
                                                        .path(if is_fullscreen { "icons/minimize.svg" } else { "icons/maximize.svg" })
                                                        .size(icon_size)
                                                        .text_color(gpui::white())
                                                )
                                        )
                                )
                        )
                )
            })
    }
}
