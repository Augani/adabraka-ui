use adabraka_ui::{
    prelude::*,
    components::icon::Icon,
};
use gpui::*;
use std::path::PathBuf;

struct Assets {
    base: PathBuf,
}

impl gpui::AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<std::borrow::Cow<'static, [u8]>>> {
        std::fs::read(self.base.join(path))
            .map(|data| Some(std::borrow::Cow::Owned(data)))
            .map_err(|err| err.into())
    }

    fn list(&self, path: &str) -> Result<Vec<gpui::SharedString>> {
        std::fs::read_dir(self.base.join(path))
            .map(|entries| {
                entries
                    .filter_map(|entry| {
                        entry
                            .ok()
                            .and_then(|entry| entry.file_name().into_string().ok())
                            .map(gpui::SharedString::from)
                    })
                    .collect()
            })
            .map_err(|err| err.into())
    }
}

fn main() {
    Application::new()
        .with_assets(Assets { base: PathBuf::from(env!("CARGO_MANIFEST_DIR")) })
        .run(|cx| {
        adabraka_ui::init(cx);
        adabraka_ui::set_icon_base_path("assets/icons");

        cx.open_window(
            WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some("Icon Showcase - adabraka-ui".into()),
                    ..Default::default()
                }),
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(1200.0), px(800.0)),
                    cx,
                ))),
                ..Default::default()
            },
            |window, cx| cx.new(|cx| IconShowcaseApp::new(window, cx)),
        )
        .unwrap();
    });
}

struct IconShowcaseApp {}

impl IconShowcaseApp {
    fn new(_window: &mut Window, cx: &mut App) -> Self {
        let theme = Theme::light();
        install_theme(cx, theme.clone());

        Self {}
    }

    fn render_icon_grid(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        let icons = vec![
            ("arrow-up", "Arrow Up"),
            ("arrow-down", "Arrow Down"),
            ("arrow-left", "Arrow Left"),
            ("arrow-right", "Arrow Right"),
            ("chevron-up", "Chevron Up"),
            ("chevron-down", "Chevron Down"),
            ("chevron-left", "Chevron Left"),
            ("chevron-right", "Chevron Right"),
            ("x", "Close"),
            ("check", "Check"),
            ("plus", "Plus"),
            ("minus", "Minus"),
            ("search", "Search"),
            ("menu", "Menu"),
            ("settings", "Settings"),
            ("user", "User"),
            ("heart", "Heart"),
            ("star", "Star"),
            ("home", "Home"),
            ("folder", "Folder"),
            ("file", "File"),
            ("trash", "Trash"),
            ("edit", "Edit"),
            ("copy", "Copy"),
            ("download", "Download"),
            ("upload", "Upload"),
            ("globe", "Globe"),
            ("mail", "Mail"),
            ("bell", "Bell"),
            ("calendar", "Calendar"),
            ("clock", "Clock"),
            ("lock", "Lock"),
            ("unlock", "Unlock"),
            ("eye", "Eye"),
            ("eye-off", "Eye Off"),
            ("sun", "Sun"),
            ("moon", "Moon"),
            ("palette", "Palette"),
            ("camera", "Camera"),
            ("video", "Video"),
            ("music", "Music"),
            ("play", "Play"),
            ("pause", "Pause"),
            ("volume-2", "Volume"),
            ("bluetooth", "Bluetooth"),
            ("wifi", "WiFi"),
            ("battery", "Battery"),
        ];

        div()
            .flex()
            .flex_col()
            .size_full()
            .p_6()
            .bg(theme.tokens.background)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(
                                div()
                                    .text_2xl()
                                    .text_color(theme.tokens.foreground)
                                    .child("Icon Showcase")
                            )
                            .child(
                                div()
                                    .text_lg()
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Browse through available icons from the Lucide icon set")
                            )
                    )
                    .child(
                        div()
                            .flex()
                            .flex_wrap()
                            .gap_4()
                            .children(icons.into_iter().map(|(icon_name, label)| {
                                self.render_icon_card(icon_name, label, cx)
                            }))
                    )
            )
    }

    fn render_icon_card(&self, icon_name: &str, label: &str, _cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();
        let icon_name = icon_name.to_string();
        let label = label.to_string();

        div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_2()
            .p_4()
            .rounded_md()
            .bg(theme.tokens.card)
            .border_1()
            .border_color(theme.tokens.border)
            .hover(|style| {
                style.bg(theme.tokens.muted)
            })
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .size(px(48.0))
                    .child(
                        Icon::new(icon_name.clone())
                            .size(px(32.0))
                            .color(theme.tokens.foreground)
                    )
            )
            .child(
                div()
                    .text_sm()
                    .text_color(theme.tokens.foreground)
                    .child(label.clone())
            )
            .child(
                div()
                    .text_xs()
                    .text_color(theme.tokens.muted_foreground)
                    .child(icon_name.clone())
            )
    }
}

impl Render for IconShowcaseApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.render_icon_grid(cx)
    }
}
