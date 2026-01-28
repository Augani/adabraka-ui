use adabraka_ui::{
    components::hotkey_input::{HotkeyInput, HotkeyInputState, HotkeyValue},
    layout::VStack,
    theme::{install_theme, use_theme, Theme},
};
use gpui::{prelude::FluentBuilder as _, *};

struct HotkeyInputDemo {
    basic_hotkey: Entity<HotkeyInputState>,
    preset_hotkey: Entity<HotkeyInputState>,
    save_hotkey: Entity<HotkeyInputState>,
    open_hotkey: Entity<HotkeyInputState>,
    disabled_hotkey: Entity<HotkeyInputState>,
    last_captured: Option<String>,
}

impl HotkeyInputDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        let basic_hotkey = cx.new(|cx| HotkeyInputState::new(cx));

        let preset_hotkey = cx.new(|cx| {
            HotkeyInputState::with_hotkey(
                cx,
                HotkeyValue::new(
                    "s",
                    Modifiers {
                        platform: true,
                        ..Default::default()
                    },
                ),
            )
        });

        let save_hotkey = cx.new(|cx| HotkeyInputState::new(cx));
        let open_hotkey = cx.new(|cx| HotkeyInputState::new(cx));

        let disabled_hotkey = cx.new(|cx| {
            HotkeyInputState::with_hotkey(
                cx,
                HotkeyValue::new(
                    "p",
                    Modifiers {
                        platform: true,
                        shift: true,
                        ..Default::default()
                    },
                ),
            )
        });

        Self {
            basic_hotkey,
            preset_hotkey,
            save_hotkey,
            open_hotkey,
            disabled_hotkey,
            last_captured: None,
        }
    }
}

impl Render for HotkeyInputDemo {
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
                                    .child("Hotkey Input Demo"),
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Click to start recording, press keys to capture. Escape cancels."),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(20.0))
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
                                            .child("Basic Hotkey Capture"),
                                    )
                                    .child(
                                        HotkeyInput::new(self.basic_hotkey.clone())
                                            .placeholder("Click to record hotkey...")
                                            .on_change(move |hotkey, _, cx| {
                                                entity.update(cx, |this, cx| {
                                                    this.last_captured = hotkey.map(|h| h.format_display());
                                                    cx.notify();
                                                });
                                            })
                                            .w(px(250.0)),
                                    )
                            })
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(14.0))
                                            .font_weight(FontWeight::MEDIUM)
                                            .child("With Initial Value"),
                                    )
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("Pre-configured with Cmd+S"),
                                    )
                                    .child(
                                        HotkeyInput::new(self.preset_hotkey.clone())
                                            .w(px(250.0)),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(14.0))
                                            .font_weight(FontWeight::MEDIUM)
                                            .child("Disabled State"),
                                    )
                                    .child(
                                        HotkeyInput::new(self.disabled_hotkey.clone())
                                            .disabled(true)
                                            .w(px(250.0)),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(12.0))
                                    .child(
                                        div()
                                            .text_size(px(14.0))
                                            .font_weight(FontWeight::MEDIUM)
                                            .child("Multiple Shortcut Configuration"),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .gap(px(16.0))
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .gap(px(4.0))
                                                    .child(
                                                        div()
                                                            .text_size(px(12.0))
                                                            .text_color(theme.tokens.muted_foreground)
                                                            .child("Save"),
                                                    )
                                                    .child(
                                                        HotkeyInput::new(self.save_hotkey.clone())
                                                            .placeholder("Set save shortcut")
                                                            .w(px(180.0)),
                                                    ),
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .gap(px(4.0))
                                                    .child(
                                                        div()
                                                            .text_size(px(12.0))
                                                            .text_color(theme.tokens.muted_foreground)
                                                            .child("Open"),
                                                    )
                                                    .child(
                                                        HotkeyInput::new(self.open_hotkey.clone())
                                                            .placeholder("Set open shortcut")
                                                            .w(px(180.0)),
                                                    ),
                                            ),
                                    ),
                            ),
                    )
                    .when(self.last_captured.is_some(), |d| {
                        d.child(
                            div()
                                .mt(px(16.0))
                                .p(px(12.0))
                                .bg(theme.tokens.muted)
                                .rounded(theme.tokens.radius_md)
                                .flex()
                                .items_center()
                                .gap(px(8.0))
                                .child(
                                    div()
                                        .text_size(px(12.0))
                                        .font_weight(FontWeight::MEDIUM)
                                        .child("Last captured:"),
                                )
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .font_family(theme.tokens.font_mono.clone())
                                        .text_color(theme.tokens.primary)
                                        .child(self.last_captured.clone().unwrap_or_default()),
                                ),
                        )
                    }),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        adabraka_ui::init(cx);
        install_theme(cx, Theme::dark());

        let bounds = Bounds::centered(None, size(px(600.0), px(550.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("Hotkey Input Demo".into()),
                    appears_transparent: false,
                    traffic_light_position: None,
                }),
                ..Default::default()
            },
            |_, cx| cx.new(|cx| HotkeyInputDemo::new(cx)),
        )
        .unwrap();

        cx.activate(true);
    });
}
