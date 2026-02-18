use crate::icon_config::resolve_icon_path;
use crate::theme::use_theme;
use gpui::{prelude::*, *};
#[cfg(feature = "audio")]
use std::io::BufReader;
use std::rc::Rc;
#[cfg(feature = "audio")]
use std::sync::Mutex;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum AudioPlayerSize {
    Compact,
    #[default]
    Full,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PlaybackSpeed {
    Half,
    Normal,
    OneAndHalf,
    Double,
}

impl Default for PlaybackSpeed {
    fn default() -> Self {
        Self::Normal
    }
}

impl PlaybackSpeed {
    pub fn value(&self) -> f32 {
        match self {
            PlaybackSpeed::Half => 0.5,
            PlaybackSpeed::Normal => 1.0,
            PlaybackSpeed::OneAndHalf => 1.5,
            PlaybackSpeed::Double => 2.0,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            PlaybackSpeed::Half => "0.5x",
            PlaybackSpeed::Normal => "1x",
            PlaybackSpeed::OneAndHalf => "1.5x",
            PlaybackSpeed::Double => "2x",
        }
    }

    pub fn next(&self) -> Self {
        match self {
            PlaybackSpeed::Half => PlaybackSpeed::Normal,
            PlaybackSpeed::Normal => PlaybackSpeed::OneAndHalf,
            PlaybackSpeed::OneAndHalf => PlaybackSpeed::Double,
            PlaybackSpeed::Double => PlaybackSpeed::Half,
        }
    }
}

#[cfg(feature = "audio")]
pub struct AudioBackend {
    sink: rodio::Sink,
    _stream: rodio::OutputStream,
    stream_handle: rodio::OutputStreamHandle,
    file_path: Option<String>,
}

#[cfg(feature = "audio")]
impl AudioBackend {
    pub fn new() -> Option<Self> {
        let (stream, stream_handle) = rodio::OutputStream::try_default().ok()?;
        let sink = rodio::Sink::try_new(&stream_handle).ok()?;
        sink.pause();
        Some(Self {
            sink,
            _stream: stream,
            stream_handle,
            file_path: None,
        })
    }

    pub fn load(&mut self, path: &str) -> Result<std::time::Duration, String> {
        use rodio::Source;

        let file = std::fs::File::open(path).map_err(|e| e.to_string())?;
        let reader = BufReader::new(file);
        let source = rodio::Decoder::new(reader).map_err(|e| e.to_string())?;
        let duration = source.total_duration().unwrap_or(std::time::Duration::ZERO);

        self.sink.stop();
        self.sink = rodio::Sink::try_new(&self.stream_handle).map_err(|e| e.to_string())?;
        self.sink.append(source);
        self.sink.pause();
        self.file_path = Some(path.to_string());

        Ok(duration)
    }

    pub fn play(&self) {
        self.sink.play();
    }

    pub fn pause(&self) {
        self.sink.pause();
    }

    pub fn stop(&self) {
        self.sink.stop();
    }

    pub fn set_volume(&self, volume: f32) {
        self.sink.set_volume(volume);
    }

    pub fn set_speed(&self, speed: f32) {
        self.sink.set_speed(speed);
    }

    pub fn is_empty(&self) -> bool {
        self.sink.empty()
    }
}

pub struct AudioPlayerState {
    is_playing: bool,
    is_muted: bool,
    current_time: f32,
    duration: f32,
    volume: f32,
    playback_speed: PlaybackSpeed,
    focus_handle: FocusHandle,
    progress_dragging: bool,
    volume_dragging: bool,
    progress_bounds: Bounds<Pixels>,
    volume_bounds: Bounds<Pixels>,
    source_path: Option<String>,
    #[cfg(feature = "audio")]
    backend: Option<Arc<Mutex<AudioBackend>>>,
}

impl AudioPlayerState {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            is_playing: false,
            is_muted: false,
            current_time: 0.0,
            duration: 0.0,
            volume: 0.8,
            playback_speed: PlaybackSpeed::Normal,
            focus_handle: cx.focus_handle(),
            progress_dragging: false,
            volume_dragging: false,
            progress_bounds: Bounds::default(),
            volume_bounds: Bounds::default(),
            source_path: None,
            #[cfg(feature = "audio")]
            backend: AudioBackend::new().map(|b| Arc::new(Mutex::new(b))),
        }
    }

    #[cfg(feature = "audio")]
    pub fn load_file(&mut self, path: impl Into<String>, cx: &mut Context<Self>) -> bool {
        let path_str = path.into();
        self.source_path = Some(path_str.clone());

        if let Some(ref backend) = self.backend {
            if let Ok(mut backend) = backend.lock() {
                match backend.load(&path_str) {
                    Ok(duration) => {
                        self.duration = duration.as_secs_f32();
                        self.current_time = 0.0;
                        self.is_playing = false;
                        backend.set_volume(self.volume);
                        backend.set_speed(self.playback_speed.value());
                        cx.notify();
                        return true;
                    }
                    Err(e) => {
                        eprintln!("Failed to load audio: {}", e);
                        return false;
                    }
                }
            }
        }
        false
    }

    #[cfg(not(feature = "audio"))]
    pub fn load_file(&mut self, path: impl Into<String>, cx: &mut Context<Self>) -> bool {
        self.source_path = Some(path.into());
        cx.notify();
        false
    }

    pub fn is_playing(&self) -> bool {
        self.is_playing
    }

    pub fn set_playing(&mut self, playing: bool, cx: &mut Context<Self>) {
        self.is_playing = playing;
        #[cfg(feature = "audio")]
        if let Some(ref backend) = self.backend {
            if let Ok(backend) = backend.lock() {
                if playing {
                    backend.play();
                } else {
                    backend.pause();
                }
            }
        }
        cx.notify();
    }

    pub fn toggle_playing(&mut self, cx: &mut Context<Self>) {
        self.is_playing = !self.is_playing;
        #[cfg(feature = "audio")]
        if let Some(ref backend) = self.backend {
            if let Ok(backend) = backend.lock() {
                if self.is_playing {
                    backend.play();
                } else {
                    backend.pause();
                }
            }
        }
        cx.notify();
    }

    pub fn is_muted(&self) -> bool {
        self.is_muted
    }

    pub fn set_muted(&mut self, muted: bool, cx: &mut Context<Self>) {
        self.is_muted = muted;
        #[cfg(feature = "audio")]
        self.apply_volume();
        cx.notify();
    }

    pub fn toggle_muted(&mut self, cx: &mut Context<Self>) {
        self.is_muted = !self.is_muted;
        #[cfg(feature = "audio")]
        self.apply_volume();
        cx.notify();
    }

    pub fn current_time(&self) -> f32 {
        self.current_time
    }

    pub fn set_current_time(&mut self, time: f32, cx: &mut Context<Self>) {
        self.current_time = time.clamp(0.0, self.duration);
        cx.notify();
    }

    pub fn duration(&self) -> f32 {
        self.duration
    }

    pub fn set_duration(&mut self, duration: f32, cx: &mut Context<Self>) {
        if duration < 0.0 {
            return;
        }
        self.duration = duration;
        self.current_time = self.current_time.min(duration);
        cx.notify();
    }

    pub fn volume(&self) -> f32 {
        self.volume
    }

    pub fn set_volume(&mut self, volume: f32, cx: &mut Context<Self>) {
        self.volume = volume.clamp(0.0, 1.0);
        if self.volume > 0.0 {
            self.is_muted = false;
        }
        #[cfg(feature = "audio")]
        self.apply_volume();
        cx.notify();
    }

    #[cfg(feature = "audio")]
    fn apply_volume(&self) {
        if let Some(ref backend) = self.backend {
            if let Ok(backend) = backend.lock() {
                let effective_vol = if self.is_muted { 0.0 } else { self.volume };
                backend.set_volume(effective_vol);
            }
        }
    }

    pub fn effective_volume(&self) -> f32 {
        if self.is_muted {
            0.0
        } else {
            self.volume
        }
    }

    pub fn playback_speed(&self) -> PlaybackSpeed {
        self.playback_speed
    }

    pub fn set_playback_speed(&mut self, speed: PlaybackSpeed, cx: &mut Context<Self>) {
        self.playback_speed = speed;
        #[cfg(feature = "audio")]
        if let Some(ref backend) = self.backend {
            if let Ok(backend) = backend.lock() {
                backend.set_speed(speed.value());
            }
        }
        cx.notify();
    }

    pub fn cycle_playback_speed(&mut self, cx: &mut Context<Self>) {
        self.playback_speed = self.playback_speed.next();
        #[cfg(feature = "audio")]
        if let Some(ref backend) = self.backend {
            if let Ok(backend) = backend.lock() {
                backend.set_speed(self.playback_speed.value());
            }
        }
        cx.notify();
    }

    pub fn stop(&mut self, cx: &mut Context<Self>) {
        self.is_playing = false;
        self.current_time = 0.0;
        #[cfg(feature = "audio")]
        if let Some(ref backend) = self.backend {
            if let Ok(backend) = backend.lock() {
                backend.stop();
            }
        }
        cx.notify();
    }

    #[cfg(feature = "audio")]
    pub fn is_finished(&self) -> bool {
        if let Some(ref backend) = self.backend {
            if let Ok(backend) = backend.lock() {
                return backend.is_empty();
            }
        }
        false
    }

    #[cfg(not(feature = "audio"))]
    pub fn is_finished(&self) -> bool {
        self.current_time >= self.duration
    }

    fn progress_percentage(&self) -> f32 {
        if self.duration <= 0.0 {
            return 0.0;
        }
        (self.current_time / self.duration).clamp(0.0, 1.0)
    }

    fn update_progress_from_position(&mut self, position: Point<Pixels>, cx: &mut Context<Self>) {
        let track_width = self.progress_bounds.size.width;
        if track_width <= px(0.0) {
            return;
        }
        let relative_x = (position.x - self.progress_bounds.left()).clamp(px(0.0), track_width);
        let percentage = (relative_x / track_width).clamp(0.0, 1.0);
        self.current_time = percentage * self.duration;
        cx.notify();
    }

    fn update_volume_from_position(&mut self, position: Point<Pixels>, cx: &mut Context<Self>) {
        let track_width = self.volume_bounds.size.width;
        if track_width <= px(0.0) {
            return;
        }
        let relative_x = (position.x - self.volume_bounds.left()).clamp(px(0.0), track_width);
        let volume = (relative_x / track_width).clamp(0.0, 1.0);
        self.set_volume(volume, cx);
    }

    #[cfg(feature = "audio")]
    pub fn is_audio_loaded(&self) -> bool {
        self.backend.is_some() && self.source_path.is_some()
    }

    #[cfg(not(feature = "audio"))]
    pub fn is_audio_loaded(&self) -> bool {
        false
    }
}

impl Focusable for AudioPlayerState {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for AudioPlayerState {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}

fn format_time(seconds: f32) -> String {
    let total_seconds = seconds.max(0.0) as u32;
    let minutes = total_seconds / 60;
    let secs = total_seconds % 60;
    format!("{:02}:{:02}", minutes, secs)
}

#[derive(IntoElement)]
pub struct AudioPlayer {
    state: Entity<AudioPlayerState>,
    size: AudioPlayerSize,
    disabled: bool,
    title: Option<SharedString>,
    on_play: Option<Rc<dyn Fn(&mut Window, &mut App) + 'static>>,
    on_pause: Option<Rc<dyn Fn(&mut Window, &mut App) + 'static>>,
    on_seek: Option<Rc<dyn Fn(f32, &mut Window, &mut App) + 'static>>,
    on_volume_change: Option<Rc<dyn Fn(f32, &mut Window, &mut App) + 'static>>,
    on_speed_change: Option<Rc<dyn Fn(PlaybackSpeed, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
}

impl AudioPlayer {
    pub fn new(state: Entity<AudioPlayerState>) -> Self {
        Self {
            state,
            size: AudioPlayerSize::Full,
            disabled: false,
            title: None,
            on_play: None,
            on_pause: None,
            on_seek: None,
            on_volume_change: None,
            on_speed_change: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn size(mut self, size: AudioPlayerSize) -> Self {
        self.size = size;
        self
    }

    pub fn compact(mut self) -> Self {
        self.size = AudioPlayerSize::Compact;
        self
    }

    pub fn full(mut self) -> Self {
        self.size = AudioPlayerSize::Full;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
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

    pub fn on_seek(mut self, handler: impl Fn(f32, &mut Window, &mut App) + 'static) -> Self {
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

    pub fn on_speed_change(
        mut self,
        handler: impl Fn(PlaybackSpeed, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_speed_change = Some(Rc::new(handler));
        self
    }
}

impl Styled for AudioPlayer {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for AudioPlayer {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let state = self.state.read(cx);
        let is_playing = state.is_playing;
        let is_muted = state.is_muted;
        let current_time = state.current_time;
        let duration = state.duration;
        let volume = state.volume;
        let progress_percentage = state.progress_percentage();
        let playback_speed = state.playback_speed;
        let user_style = self.style.clone();

        let (padding, gap, button_size, icon_size, track_height, thumb_size) = match self.size {
            AudioPlayerSize::Compact => (px(8.0), px(8.0), px(32.0), px(16.0), px(4.0), px(12.0)),
            AudioPlayerSize::Full => (px(16.0), px(12.0), px(40.0), px(20.0), px(6.0), px(16.0)),
        };

        let play_icon = if is_playing { "pause" } else { "play" };
        let volume_icon = if is_muted || volume == 0.0 {
            "volume-x"
        } else if volume < 0.5 {
            "volume-1"
        } else {
            "volume-2"
        };

        let base = div()
            .flex()
            .items_center()
            .gap(gap)
            .p(padding)
            .bg(theme.tokens.card)
            .border_1()
            .border_color(theme.tokens.border)
            .rounded(theme.tokens.radius_lg)
            .when(self.disabled, |this| this.opacity(0.5));

        match self.size {
            AudioPlayerSize::Compact => base
                .map(|this| {
                    let mut div = this;
                    div.style().refine(&user_style);
                    div
                })
                .child(self.render_play_button(
                    window,
                    &theme,
                    play_icon,
                    is_playing,
                    button_size,
                    icon_size,
                ))
                .child(self.render_progress_bar(
                    window,
                    &theme,
                    progress_percentage,
                    track_height,
                    thumb_size,
                ))
                .child(
                    div()
                        .text_size(px(11.0))
                        .text_color(theme.tokens.muted_foreground)
                        .font_family(theme.tokens.font_mono.clone())
                        .child(format!(
                            "{} / {}",
                            format_time(current_time),
                            format_time(duration)
                        )),
                ),

            AudioPlayerSize::Full => base
                .flex_col()
                .gap(px(12.0))
                .map(|this| {
                    let mut div = this;
                    div.style().refine(&user_style);
                    div
                })
                .when_some(self.title.clone(), |this, title| {
                    this.child(
                        div()
                            .w_full()
                            .text_size(px(14.0))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(theme.tokens.foreground)
                            .overflow_hidden()
                            .text_ellipsis()
                            .child(title),
                    )
                })
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(4.0))
                        .w_full()
                        .child(self.render_progress_bar(
                            window,
                            &theme,
                            progress_percentage,
                            track_height,
                            thumb_size,
                        ))
                        .child(
                            div()
                                .flex()
                                .justify_between()
                                .w_full()
                                .text_size(px(12.0))
                                .font_family(theme.tokens.font_mono.clone())
                                .text_color(theme.tokens.muted_foreground)
                                .child(format_time(current_time))
                                .child(format_time(duration)),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .w_full()
                        .child(div().flex().items_center().gap(px(8.0)).child(
                            self.render_play_button(
                                window,
                                &theme,
                                play_icon,
                                is_playing,
                                button_size,
                                icon_size,
                            ),
                        ))
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap(px(8.0))
                                .child(self.render_speed_button(window, &theme, playback_speed))
                                .child(self.render_mute_button(
                                    window,
                                    &theme,
                                    volume_icon,
                                    px(28.0),
                                    px(14.0),
                                ))
                                .child(self.render_volume_slider(
                                    window,
                                    &theme,
                                    volume,
                                    px(80.0),
                                    px(4.0),
                                    px(12.0),
                                )),
                        ),
                ),
        }
    }
}

impl AudioPlayer {
    fn render_play_button(
        &self,
        window: &mut Window,
        theme: &crate::theme::Theme,
        icon_name: &str,
        _is_playing: bool,
        button_size: Pixels,
        icon_size: Pixels,
    ) -> impl IntoElement {
        let state = self.state.clone();
        let on_play = self.on_play.clone();
        let on_pause = self.on_pause.clone();
        let disabled = self.disabled;

        div()
            .id("audio-play-button")
            .flex()
            .items_center()
            .justify_center()
            .size(button_size)
            .rounded_full()
            .bg(theme.tokens.primary)
            .when(!disabled, |this| {
                this.cursor(CursorStyle::PointingHand)
                    .hover(|style| style.bg(theme.tokens.primary.opacity(0.9)))
            })
            .child(
                svg()
                    .path(resolve_icon_path(icon_name))
                    .size(icon_size)
                    .text_color(theme.tokens.primary_foreground),
            )
            .when(!disabled, |this| {
                this.on_click(window.listener_for(&state, move |state, _, window, cx| {
                    state.toggle_playing(cx);
                    if state.is_playing {
                        if let Some(ref handler) = on_play {
                            handler(window, cx);
                        }
                    } else if let Some(ref handler) = on_pause {
                        handler(window, cx);
                    }
                }))
            })
    }

    fn render_progress_bar(
        &self,
        window: &mut Window,
        theme: &crate::theme::Theme,
        percentage: f32,
        track_height: Pixels,
        thumb_size: Pixels,
    ) -> impl IntoElement {
        let state = self.state.clone();
        let on_seek = self.on_seek.clone();
        let disabled = self.disabled;

        div()
            .flex_1()
            .h(thumb_size)
            .flex()
            .items_center()
            .relative()
            .child(
                canvas(
                    {
                        let state = state.clone();
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
                    .w_full()
                    .h(track_height)
                    .rounded_full()
                    .bg(theme.tokens.muted)
                    .overflow_hidden()
                    .child(
                        div()
                            .h_full()
                            .w(relative(percentage))
                            .bg(theme.tokens.primary),
                    ),
            )
            .child(
                div()
                    .absolute()
                    .left(relative(percentage))
                    .top(px(0.0))
                    .ml(-(thumb_size / 2.0))
                    .size(thumb_size)
                    .rounded_full()
                    .bg(theme.tokens.primary)
                    .border_2()
                    .border_color(theme.tokens.background)
                    .shadow_sm()
                    .when(!disabled, |this| this.cursor(CursorStyle::PointingHand)),
            )
            .when(!disabled, |this| {
                let state_down = state.clone();
                let on_seek_down = on_seek.clone();
                let state_move = state.clone();
                let on_seek_move = on_seek.clone();
                let state_up = state.clone();

                this.on_mouse_down(
                    MouseButton::Left,
                    window.listener_for(
                        &state_down,
                        move |state, e: &MouseDownEvent, window, cx| {
                            state.progress_dragging = true;
                            state.update_progress_from_position(e.position, cx);
                            if let Some(ref handler) = on_seek_down {
                                handler(state.current_time, window, cx);
                            }
                        },
                    ),
                )
                .on_mouse_move(window.listener_for(
                    &state_move,
                    move |state, e: &MouseMoveEvent, window, cx| {
                        if state.progress_dragging {
                            state.update_progress_from_position(e.position, cx);
                            if let Some(ref handler) = on_seek_move {
                                handler(state.current_time, window, cx);
                            }
                        }
                    },
                ))
                .on_mouse_up(
                    MouseButton::Left,
                    window.listener_for(&state_up, move |state, _: &MouseUpEvent, _, _| {
                        state.progress_dragging = false;
                    }),
                )
            })
    }

    fn render_mute_button(
        &self,
        window: &mut Window,
        theme: &crate::theme::Theme,
        icon_name: &str,
        button_size: Pixels,
        icon_size: Pixels,
    ) -> impl IntoElement {
        let state = self.state.clone();
        let disabled = self.disabled;

        div()
            .id("audio-mute-button")
            .flex()
            .items_center()
            .justify_center()
            .size(button_size)
            .rounded(theme.tokens.radius_md)
            .when(!disabled, |this| {
                this.cursor(CursorStyle::PointingHand)
                    .hover(|style| style.bg(theme.tokens.accent))
            })
            .child(
                svg()
                    .path(resolve_icon_path(icon_name))
                    .size(icon_size)
                    .text_color(theme.tokens.muted_foreground),
            )
            .when(!disabled, |this| {
                this.on_click(window.listener_for(&state, move |state, _, _, cx| {
                    state.toggle_muted(cx);
                }))
            })
    }

    fn render_volume_slider(
        &self,
        window: &mut Window,
        theme: &crate::theme::Theme,
        volume: f32,
        width: Pixels,
        track_height: Pixels,
        thumb_size: Pixels,
    ) -> impl IntoElement {
        let state = self.state.clone();
        let on_volume_change = self.on_volume_change.clone();
        let disabled = self.disabled;

        div()
            .w(width)
            .h(thumb_size)
            .flex()
            .items_center()
            .relative()
            .child(
                canvas(
                    {
                        let state = state.clone();
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
                    .w_full()
                    .h(track_height)
                    .rounded_full()
                    .bg(theme.tokens.muted)
                    .overflow_hidden()
                    .child(
                        div()
                            .h_full()
                            .w(relative(volume))
                            .bg(theme.tokens.muted_foreground),
                    ),
            )
            .child(
                div()
                    .absolute()
                    .left(relative(volume))
                    .top(px(0.0))
                    .ml(-(thumb_size / 2.0))
                    .size(thumb_size)
                    .rounded_full()
                    .bg(theme.tokens.foreground)
                    .border_2()
                    .border_color(theme.tokens.background)
                    .shadow_sm()
                    .when(!disabled, |this| this.cursor(CursorStyle::PointingHand)),
            )
            .when(!disabled, |this| {
                let state_down = state.clone();
                let on_vol_down = on_volume_change.clone();
                let state_move = state.clone();
                let on_vol_move = on_volume_change.clone();
                let state_up = state.clone();

                this.on_mouse_down(
                    MouseButton::Left,
                    window.listener_for(
                        &state_down,
                        move |state, e: &MouseDownEvent, window, cx| {
                            state.volume_dragging = true;
                            state.update_volume_from_position(e.position, cx);
                            if let Some(ref handler) = on_vol_down {
                                handler(state.volume, window, cx);
                            }
                        },
                    ),
                )
                .on_mouse_move(window.listener_for(
                    &state_move,
                    move |state, e: &MouseMoveEvent, window, cx| {
                        if state.volume_dragging {
                            state.update_volume_from_position(e.position, cx);
                            if let Some(ref handler) = on_vol_move {
                                handler(state.volume, window, cx);
                            }
                        }
                    },
                ))
                .on_mouse_up(
                    MouseButton::Left,
                    window.listener_for(&state_up, move |state, _: &MouseUpEvent, _, _| {
                        state.volume_dragging = false;
                    }),
                )
            })
    }

    fn render_speed_button(
        &self,
        window: &mut Window,
        theme: &crate::theme::Theme,
        speed: PlaybackSpeed,
    ) -> impl IntoElement {
        let state = self.state.clone();
        let on_speed_change = self.on_speed_change.clone();
        let disabled = self.disabled;

        div()
            .id("audio-speed-button")
            .flex()
            .items_center()
            .justify_center()
            .px(px(8.0))
            .h(px(28.0))
            .rounded(theme.tokens.radius_md)
            .text_size(px(12.0))
            .font_weight(FontWeight::MEDIUM)
            .text_color(theme.tokens.muted_foreground)
            .when(!disabled, |this| {
                this.cursor(CursorStyle::PointingHand)
                    .hover(|style| style.bg(theme.tokens.accent))
            })
            .child(speed.label())
            .when(!disabled, |this| {
                this.on_click(window.listener_for(&state, move |state, _, window, cx| {
                    state.cycle_playback_speed(cx);
                    if let Some(ref handler) = on_speed_change {
                        handler(state.playback_speed, window, cx);
                    }
                }))
            })
    }
}
