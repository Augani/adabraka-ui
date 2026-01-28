use crate::theme::use_theme;
use gpui::{prelude::FluentBuilder as _, *};
use std::rc::Rc;

#[derive(Clone)]
pub struct HotkeyValue {
    pub key: String,
    pub modifiers: Modifiers,
}

impl HotkeyValue {
    pub fn new(key: impl Into<String>, modifiers: Modifiers) -> Self {
        Self {
            key: key.into(),
            modifiers,
        }
    }

    pub fn from_keystroke(keystroke: &Keystroke) -> Option<Self> {
        let key = keystroke.key.as_str();
        if Self::is_modifier_only(key) {
            return None;
        }
        Some(Self {
            key: key.to_string(),
            modifiers: keystroke.modifiers,
        })
    }

    fn is_modifier_only(key: &str) -> bool {
        matches!(
            key.to_lowercase().as_str(),
            "shift" | "control" | "alt" | "meta" | "cmd" | "command" | "ctrl" | "option"
        )
    }

    #[cfg(target_os = "macos")]
    pub fn format_display(&self) -> String {
        let mut result = String::new();
        if self.modifiers.control {
            result.push_str("⌃");
        }
        if self.modifiers.alt {
            result.push_str("⌥");
        }
        if self.modifiers.shift {
            result.push_str("⇧");
        }
        if self.modifiers.platform {
            result.push_str("⌘");
        }
        result.push_str(&self.format_key());
        result
    }

    #[cfg(not(target_os = "macos"))]
    pub fn format_display(&self) -> String {
        let mut parts = Vec::new();
        if self.modifiers.control {
            parts.push("Ctrl".to_string());
        }
        if self.modifiers.alt {
            parts.push("Alt".to_string());
        }
        if self.modifiers.shift {
            parts.push("Shift".to_string());
        }
        if self.modifiers.platform {
            parts.push("Win".to_string());
        }
        parts.push(self.format_key());
        parts.join("+")
    }

    fn format_key(&self) -> String {
        match self.key.as_str() {
            "space" => "Space".to_string(),
            "enter" => "Enter".to_string(),
            "escape" => "Esc".to_string(),
            "tab" => "Tab".to_string(),
            "backspace" => "Backspace".to_string(),
            "delete" => "Del".to_string(),
            "up" => "Up".to_string(),
            "down" => "Down".to_string(),
            "left" => "Left".to_string(),
            "right" => "Right".to_string(),
            "home" => "Home".to_string(),
            "end" => "End".to_string(),
            "pageup" => "PgUp".to_string(),
            "pagedown" => "PgDn".to_string(),
            k if k.starts_with('f') && k.len() <= 3 => k.to_uppercase(),
            k => k.to_uppercase(),
        }
    }
}

pub struct HotkeyInputState {
    hotkey: Option<HotkeyValue>,
    recording: bool,
    focus_handle: FocusHandle,
}

impl HotkeyInputState {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            hotkey: None,
            recording: false,
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn with_hotkey(cx: &mut Context<Self>, hotkey: HotkeyValue) -> Self {
        Self {
            hotkey: Some(hotkey),
            recording: false,
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn hotkey(&self) -> Option<&HotkeyValue> {
        self.hotkey.as_ref()
    }

    pub fn set_hotkey(&mut self, hotkey: Option<HotkeyValue>, cx: &mut Context<Self>) {
        self.hotkey = hotkey;
        self.recording = false;
        cx.notify();
    }

    pub fn is_recording(&self) -> bool {
        self.recording
    }

    pub fn start_recording(&mut self, cx: &mut Context<Self>) {
        self.recording = true;
        cx.notify();
    }

    pub fn stop_recording(&mut self, cx: &mut Context<Self>) {
        self.recording = false;
        cx.notify();
    }

    pub fn clear(&mut self, cx: &mut Context<Self>) {
        self.hotkey = None;
        self.recording = false;
        cx.notify();
    }

    pub fn capture_keystroke(&mut self, keystroke: &Keystroke, cx: &mut Context<Self>) -> bool {
        if !self.recording {
            return false;
        }

        if keystroke.key.as_str() == "escape" {
            self.stop_recording(cx);
            return true;
        }

        if let Some(hotkey) = HotkeyValue::from_keystroke(keystroke) {
            self.hotkey = Some(hotkey);
            self.recording = false;
            cx.notify();
            return true;
        }

        false
    }
}

impl Focusable for HotkeyInputState {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for HotkeyInputState {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}

#[derive(IntoElement)]
pub struct HotkeyInput {
    state: Entity<HotkeyInputState>,
    placeholder: SharedString,
    disabled: bool,
    on_change: Option<Rc<dyn Fn(Option<&HotkeyValue>, &mut Window, &mut App)>>,
    style: StyleRefinement,
}

impl HotkeyInput {
    pub fn new(state: Entity<HotkeyInputState>) -> Self {
        Self {
            state,
            placeholder: "Click to record".into(),
            disabled: false,
            on_change: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_change(
        mut self,
        handler: impl Fn(Option<&HotkeyValue>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }
}

impl Styled for HotkeyInput {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for HotkeyInput {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let user_style = self.style;

        let state_data = self.state.read(cx);
        let hotkey = state_data.hotkey.clone();
        let recording = state_data.recording;
        let focus_handle = state_data.focus_handle(cx);
        let is_focused = focus_handle.is_focused(window);

        let display_text: SharedString = if recording {
            "Press a key...".into()
        } else if let Some(ref hk) = hotkey {
            hk.format_display().into()
        } else {
            self.placeholder.clone()
        };

        let has_value = hotkey.is_some();
        let show_clear = has_value && !self.disabled && !recording;

        let state_for_click = self.state.clone();
        let state_for_keydown = self.state.clone();
        let state_for_clear = self.state.clone();

        let on_change_for_keydown = self.on_change.clone();
        let on_change_for_clear = self.on_change.clone();

        let border_color = if recording {
            theme.tokens.primary
        } else if is_focused {
            theme.tokens.ring
        } else {
            theme.tokens.input
        };

        let focus_ring = theme.tokens.focus_ring_light();
        let recording_ring = BoxShadow {
            offset: point(px(0.0), px(0.0)),
            blur_radius: px(0.0),
            spread_radius: px(3.0),
            color: theme.tokens.primary.opacity(0.3),
        };

        let text_color = if hotkey.is_some() && !recording {
            theme.tokens.foreground
        } else {
            theme.tokens.muted_foreground
        };

        let clear_button = if show_clear {
            Some(
                div()
                    .id("hotkey-clear")
                    .ml(px(8.0))
                    .px(px(6.0))
                    .py(px(4.0))
                    .rounded(px(4.0))
                    .text_color(theme.tokens.muted_foreground)
                    .hover(|s| s.bg(theme.tokens.muted).text_color(theme.tokens.foreground))
                    .on_click(move |_, window, cx| {
                        state_for_clear.update(cx, |state, cx| {
                            state.clear(cx);
                        });
                        if let Some(ref handler) = on_change_for_clear {
                            handler(None, window, cx);
                        }
                        cx.stop_propagation();
                    })
                    .child("×"),
            )
        } else {
            None
        };

        div()
            .map(|this| {
                let mut d = this;
                d.style().refine(&user_style);
                d
            })
            .child(
                div()
                    .id(("hotkey-input", self.state.entity_id()))
                    .track_focus(&focus_handle.tab_index(0).tab_stop(true))
                    .h(px(40.0))
                    .px(px(12.0))
                    .flex()
                    .items_center()
                    .justify_between()
                    .bg(theme.tokens.background)
                    .border_1()
                    .border_color(border_color)
                    .rounded(theme.tokens.radius_md)
                    .font_family(theme.tokens.font_mono.clone())
                    .text_size(px(14.0))
                    .when(self.disabled, |d| d.opacity(0.5).cursor_not_allowed())
                    .when(!self.disabled, |d| d.cursor_pointer())
                    .when(is_focused && !recording, |d| d.shadow(vec![focus_ring]))
                    .when(recording, |d| {
                        d.shadow(vec![recording_ring])
                            .border_color(theme.tokens.primary)
                    })
                    .when(!self.disabled, |d| {
                        d.on_click(move |_, window, cx| {
                            state_for_click.update(cx, |state, cx| {
                                if !state.recording {
                                    state.start_recording(cx);
                                }
                            });
                            window.refresh();
                        })
                    })
                    .when(!self.disabled, |d| {
                        d.on_key_down(move |event, window, cx| {
                            let (captured, hotkey) = state_for_keydown.update(cx, |state, cx| {
                                let captured = state.capture_keystroke(&event.keystroke, cx);
                                (captured, state.hotkey.clone())
                            });
                            if captured {
                                if let Some(ref handler) = on_change_for_keydown {
                                    handler(hotkey.as_ref(), window, cx);
                                }
                                cx.stop_propagation();
                            }
                        })
                    })
                    .child(
                        div()
                            .flex_1()
                            .text_color(text_color)
                            .when(recording, |d| d.opacity(0.7))
                            .child(display_text),
                    )
                    .children(clear_button),
            )
    }
}
