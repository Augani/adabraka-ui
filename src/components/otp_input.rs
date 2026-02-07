use crate::theme::use_theme;
use gpui::{prelude::FluentBuilder as _, *};
use std::rc::Rc;

actions!(
    otp_input,
    [
        OTPBackspace,
        OTPDelete,
        OTPLeft,
        OTPRight,
        OTPHome,
        OTPEnd,
        OTPPaste,
        OTPEscape,
    ]
);

pub fn init(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("backspace", OTPBackspace, Some("OTPInput")),
        KeyBinding::new("delete", OTPDelete, Some("OTPInput")),
        KeyBinding::new("left", OTPLeft, Some("OTPInput")),
        KeyBinding::new("right", OTPRight, Some("OTPInput")),
        KeyBinding::new("home", OTPHome, Some("OTPInput")),
        KeyBinding::new("end", OTPEnd, Some("OTPInput")),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-v", OTPPaste, Some("OTPInput")),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-v", OTPPaste, Some("OTPInput")),
        KeyBinding::new("escape", OTPEscape, Some("OTPInput")),
    ]);
}

#[derive(Clone, Debug)]
pub enum OTPInputEvent {
    Change(String),
    Complete(String),
    Focus,
    Blur,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OTPInputSize {
    Sm,
    Md,
    Lg,
}

impl Default for OTPInputSize {
    fn default() -> Self {
        Self::Md
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OTPInputState {
    Default,
    Error,
    Success,
}

impl Default for OTPInputState {
    fn default() -> Self {
        Self::Default
    }
}

pub struct OTPState {
    focus_handles: Vec<FocusHandle>,
    digits: Vec<Option<char>>,
    focused_index: usize,
    digit_count: usize,
    masked: bool,
    disabled: bool,
    state: OTPInputState,
}

impl EventEmitter<OTPInputEvent> for OTPState {}

impl OTPState {
    pub fn new(cx: &mut Context<Self>, digit_count: usize) -> Self {
        let digit_count = digit_count.clamp(4, 8);
        let focus_handles: Vec<FocusHandle> = (0..digit_count).map(|_| cx.focus_handle()).collect();

        Self {
            focus_handles,
            digits: vec![None; digit_count],
            focused_index: 0,
            digit_count,
            masked: false,
            disabled: false,
            state: OTPInputState::Default,
        }
    }

    pub fn digit_count(mut self, count: usize) -> Self {
        let count = count.clamp(4, 8);
        self.digit_count = count;
        self.digits.resize(count, None);
        self
    }

    pub fn masked(mut self, masked: bool) -> Self {
        self.masked = masked;
        self
    }

    pub fn value(&self) -> String {
        self.digits.iter().filter_map(|d| *d).collect()
    }

    pub fn is_complete(&self) -> bool {
        self.digits.iter().all(|d| d.is_some())
    }

    pub fn set_value(&mut self, value: &str, cx: &mut Context<Self>) {
        self.digits.fill(None);

        for (i, ch) in value.chars().take(self.digit_count).enumerate() {
            if ch.is_ascii_digit() {
                self.digits[i] = Some(ch);
            }
        }

        cx.emit(OTPInputEvent::Change(self.value()));

        if self.is_complete() {
            cx.emit(OTPInputEvent::Complete(self.value()));
        }

        cx.notify();
    }

    pub fn clear(&mut self, cx: &mut Context<Self>) {
        self.digits.fill(None);
        self.focused_index = 0;
        cx.emit(OTPInputEvent::Change(String::new()));
        cx.notify();
    }

    pub fn set_state(&mut self, state: OTPInputState, cx: &mut Context<Self>) {
        self.state = state;
        cx.notify();
    }

    pub fn set_error(&mut self, cx: &mut Context<Self>) {
        self.state = OTPInputState::Error;
        cx.notify();
    }

    pub fn set_success(&mut self, cx: &mut Context<Self>) {
        self.state = OTPInputState::Success;
        cx.notify();
    }

    pub fn focus_first(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.focused_index = 0;
        if let Some(handle) = self.focus_handles.first() {
            window.focus(handle);
        }
        cx.notify();
    }

    fn set_digit(
        &mut self,
        index: usize,
        digit: char,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if index >= self.digit_count || !digit.is_ascii_digit() {
            return;
        }

        self.digits[index] = Some(digit);
        cx.emit(OTPInputEvent::Change(self.value()));

        if self.is_complete() {
            cx.emit(OTPInputEvent::Complete(self.value()));
        } else if index + 1 < self.digit_count {
            self.focused_index = index + 1;
            window.focus(&self.focus_handles[index + 1]);
        }

        cx.notify();
    }

    fn clear_digit(&mut self, index: usize, cx: &mut Context<Self>) {
        if index >= self.digit_count {
            return;
        }

        self.digits[index] = None;
        cx.emit(OTPInputEvent::Change(self.value()));
        cx.notify();
    }

    fn move_left(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.focused_index > 0 {
            self.focused_index -= 1;
            window.focus(&self.focus_handles[self.focused_index]);
            cx.notify();
        }
    }

    fn move_right(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.focused_index + 1 < self.digit_count {
            self.focused_index += 1;
            window.focus(&self.focus_handles[self.focused_index]);
            cx.notify();
        }
    }

    fn move_home(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.focused_index = 0;
        window.focus(&self.focus_handles[0]);
        cx.notify();
    }

    fn move_end(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.focused_index = self.digit_count - 1;
        window.focus(&self.focus_handles[self.focused_index]);
        cx.notify();
    }

    pub fn backspace(&mut self, _: &OTPBackspace, window: &mut Window, cx: &mut Context<Self>) {
        if self.disabled {
            return;
        }

        if self.digits[self.focused_index].is_some() {
            self.clear_digit(self.focused_index, cx);
        } else if self.focused_index > 0 {
            self.focused_index -= 1;
            self.clear_digit(self.focused_index, cx);
            window.focus(&self.focus_handles[self.focused_index]);
        }
    }

    pub fn delete(&mut self, _: &OTPDelete, _window: &mut Window, cx: &mut Context<Self>) {
        if self.disabled {
            return;
        }

        self.clear_digit(self.focused_index, cx);
    }

    pub fn left(&mut self, _: &OTPLeft, window: &mut Window, cx: &mut Context<Self>) {
        self.move_left(window, cx);
    }

    pub fn right(&mut self, _: &OTPRight, window: &mut Window, cx: &mut Context<Self>) {
        self.move_right(window, cx);
    }

    pub fn home(&mut self, _: &OTPHome, window: &mut Window, cx: &mut Context<Self>) {
        self.move_home(window, cx);
    }

    pub fn end(&mut self, _: &OTPEnd, window: &mut Window, cx: &mut Context<Self>) {
        self.move_end(window, cx);
    }

    pub fn paste(&mut self, _: &OTPPaste, _window: &mut Window, cx: &mut Context<Self>) {
        if self.disabled {
            return;
        }

        if let Some(text) = cx.read_from_clipboard().and_then(|item| item.text()) {
            let digits: String = text.chars().filter(|c| c.is_ascii_digit()).collect();
            self.set_value(&digits, cx);
        }
    }

    pub fn escape(&mut self, _: &OTPEscape, window: &mut Window, cx: &mut Context<Self>) {
        window.blur();
        cx.emit(OTPInputEvent::Blur);
        cx.notify();
    }

    pub fn focus_handle(&self, index: usize, _: &App) -> Option<FocusHandle> {
        self.focus_handles.get(index).cloned()
    }
}

impl Focusable for OTPState {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handles
            .first()
            .cloned()
            .expect("OTPState must have at least one focus handle")
    }
}

impl Render for OTPState {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}

#[derive(IntoElement)]
pub struct OTPInput {
    state: Entity<OTPState>,
    size: OTPInputSize,
    disabled: bool,
    masked: bool,
    separator: Option<SharedString>,
    separator_position: Option<usize>,
    on_change: Option<Rc<dyn Fn(String, &mut App)>>,
    on_complete: Option<Rc<dyn Fn(String, &mut App)>>,
    style: StyleRefinement,
}

impl OTPInput {
    pub fn new(state: &Entity<OTPState>) -> Self {
        Self {
            state: state.clone(),
            size: OTPInputSize::default(),
            disabled: false,
            masked: false,
            separator: None,
            separator_position: None,
            on_change: None,
            on_complete: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn size(mut self, size: OTPInputSize) -> Self {
        self.size = size;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn masked(mut self, masked: bool) -> Self {
        self.masked = masked;
        self
    }

    pub fn separator(mut self, separator: impl Into<SharedString>) -> Self {
        self.separator = Some(separator.into());
        self
    }

    pub fn separator_position(mut self, position: usize) -> Self {
        self.separator_position = Some(position);
        self
    }

    pub fn on_change<F>(mut self, callback: F) -> Self
    where
        F: Fn(String, &mut App) + 'static,
    {
        self.on_change = Some(Rc::new(callback));
        self
    }

    pub fn on_complete<F>(mut self, callback: F) -> Self
    where
        F: Fn(String, &mut App) + 'static,
    {
        self.on_complete = Some(Rc::new(callback));
        self
    }

    fn box_size(&self) -> Pixels {
        match self.size {
            OTPInputSize::Sm => px(36.0),
            OTPInputSize::Md => px(44.0),
            OTPInputSize::Lg => px(52.0),
        }
    }

    fn font_size(&self) -> Pixels {
        match self.size {
            OTPInputSize::Sm => px(16.0),
            OTPInputSize::Md => px(20.0),
            OTPInputSize::Lg => px(24.0),
        }
    }

    fn input_gap(&self) -> Pixels {
        match self.size {
            OTPInputSize::Sm => px(6.0),
            OTPInputSize::Md => px(8.0),
            OTPInputSize::Lg => px(10.0),
        }
    }
}

impl Styled for OTPInput {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for OTPInput {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let box_size = self.box_size();
        let font_size = self.font_size();
        let input_gap = self.input_gap();

        let otp_state = self.state.read(cx);
        let digit_count = otp_state.digit_count;
        let digits = otp_state.digits.clone();
        let _focused_index = otp_state.focused_index;
        let state = otp_state.state;
        let masked = self.masked || otp_state.masked;
        let disabled = self.disabled || otp_state.disabled;
        let focus_handles: Vec<FocusHandle> = otp_state.focus_handles.iter().cloned().collect();

        self.state.update(cx, |state, _| {
            state.disabled = disabled;
            state.masked = masked;
        });

        let on_change_callback = self.on_change.clone();
        let on_complete_callback = self.on_complete.clone();

        if on_change_callback.is_some() || on_complete_callback.is_some() {
            let state_entity = self.state.clone();
            cx.subscribe(
                &state_entity,
                move |_emitter: Entity<OTPState>, event: &OTPInputEvent, cx: &mut App| match event {
                    OTPInputEvent::Change(value) => {
                        if let Some(callback) = on_change_callback.as_ref() {
                            callback(value.clone(), cx);
                        }
                    }
                    OTPInputEvent::Complete(value) => {
                        if let Some(callback) = on_complete_callback.as_ref() {
                            callback(value.clone(), cx);
                        }
                    }
                    _ => {}
                },
            )
            .detach();
        }

        let (border_color, focus_border_color) = match state {
            OTPInputState::Default => (theme.tokens.border, theme.tokens.ring),
            OTPInputState::Error => (theme.tokens.destructive, theme.tokens.destructive),
            OTPInputState::Success => (theme.tokens.primary, theme.tokens.primary),
        };

        let user_style = self.style;
        let separator = self.separator.clone();
        let separator_position = self.separator_position.unwrap_or(digit_count / 2);

        div()
            .id(("otp-input", self.state.entity_id()))
            .key_context("OTPInput")
            .flex()
            .items_center()
            .gap(input_gap)
            .when(!disabled, |this| {
                this.on_action(window.listener_for(&self.state, OTPState::backspace))
                    .on_action(window.listener_for(&self.state, OTPState::delete))
                    .on_action(window.listener_for(&self.state, OTPState::left))
                    .on_action(window.listener_for(&self.state, OTPState::right))
                    .on_action(window.listener_for(&self.state, OTPState::home))
                    .on_action(window.listener_for(&self.state, OTPState::end))
                    .on_action(window.listener_for(&self.state, OTPState::paste))
                    .on_action(window.listener_for(&self.state, OTPState::escape))
            })
            .children((0..digit_count).flat_map(|i| {
                let state_clone = self.state.clone();
                let focus_handle = focus_handles[i].clone();
                let focus_handle_for_track = focus_handle.clone();
                let focus_handle_for_click = focus_handle.clone();
                let is_focused = focus_handle.is_focused(window);
                let digit = digits[i];

                let display_char = if let Some(d) = digit {
                    if masked {
                        SharedString::from("●")
                    } else {
                        SharedString::from(d.to_string())
                    }
                } else {
                    SharedString::from("")
                };

                let digit_box = div()
                    .id(ElementId::NamedInteger("otp-digit".into(), i as u64))
                    .track_focus(&focus_handle_for_track.tab_index(0).tab_stop(true))
                    .size(box_size)
                    .flex()
                    .items_center()
                    .justify_center()
                    .bg(if disabled {
                        theme.tokens.muted.opacity(0.5)
                    } else {
                        theme.tokens.background
                    })
                    .border_1()
                    .border_color(if is_focused && !disabled {
                        focus_border_color
                    } else {
                        border_color
                    })
                    .rounded(theme.tokens.radius_md)
                    .text_size(font_size)
                    .font_weight(FontWeight::SEMIBOLD)
                    .font_family(theme.tokens.font_mono.clone())
                    .text_color(if disabled {
                        theme.tokens.muted_foreground
                    } else {
                        theme.tokens.foreground
                    })
                    .when(is_focused && !disabled, |this| {
                        this.shadow(smallvec::smallvec![theme.tokens.focus_ring_light()])
                    })
                    .when(!disabled, |this| {
                        this.cursor(CursorStyle::IBeam)
                            .hover(|style| style.border_color(focus_border_color))
                    })
                    .on_mouse_down(MouseButton::Left, {
                        let state = state_clone.clone();
                        move |_, window, cx| {
                            window.focus(&focus_handle_for_click);
                            state.update(cx, |s, cx| {
                                s.focused_index = i;
                                cx.notify();
                            });
                        }
                    })
                    .on_key_down({
                        let state = state_clone.clone();
                        move |event, window, cx| {
                            if disabled {
                                return;
                            }

                            let key = &event.keystroke.key;
                            if key.len() == 1 {
                                if let Some(ch) = key.chars().next() {
                                    if ch.is_ascii_digit() {
                                        state.update(cx, |s, cx| {
                                            s.set_digit(i, ch, window, cx);
                                        });
                                        cx.stop_propagation();
                                    }
                                }
                            }
                        }
                    })
                    .child(display_char)
                    .into_any_element();

                let should_show_separator =
                    separator.is_some() && i == separator_position - 1 && i + 1 < digit_count;

                if should_show_separator {
                    vec![
                        digit_box,
                        div()
                            .text_size(font_size)
                            .text_color(theme.tokens.muted_foreground)
                            .child(separator.clone().unwrap())
                            .into_any_element(),
                    ]
                } else {
                    vec![digit_box]
                }
            }))
            .map(|this| {
                let mut container = this;
                container.style().refine(&user_style);
                container
            })
    }
}
