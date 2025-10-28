use adabraka_ui::{
    components::{
        input::{Input, InputType, InputVariant},
        input_state::InputState,
    },
    layout::VStack,
    theme::{install_theme, Theme},
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

struct PasswordTestApp {
    password_input: Entity<InputState>,
}

impl PasswordTestApp {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            password_input: cx.new(|cx| InputState::new(cx)
                .input_type(InputType::Password)
                .placeholder("Enter password")),
        }
    }
}

impl Render for PasswordTestApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = adabraka_ui::theme::use_theme();

        VStack::new()
            .size_full()
            .bg(theme.tokens.background)
            .p(px(32.0))
            .gap(px(16.0))
            .child(
                div()
                    .text_size(px(24.0))
                    .text_color(theme.tokens.foreground)
                    .child("Password Toggle Test")
            )
            .child(
                Input::new(&self.password_input)
                    .input_type(InputType::Password)
                    .password(true)
                    .variant(InputVariant::Outline)
                    .placeholder("Type a password...")
                    .on_change({
                        let entity = cx.entity();
                        move |_value, cx| {
                            entity.update(cx, |_app, cx| {
                                cx.notify();
                            });
                        }
                    })
            )
            .child(
                div()
                    .text_size(px(14.0))
                    .text_color(theme.tokens.muted_foreground)
                    .child("Click the eye icon to toggle password visibility")
            )
    }
}

fn main() {
    Application::new()
        .with_assets(Assets { base: PathBuf::from(env!("CARGO_MANIFEST_DIR")) })
        .run(move |cx| {
            install_theme(cx, Theme::dark());
            adabraka_ui::init(cx);
            adabraka_ui::set_icon_base_path("assets/icons");

            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                        None,
                        size(px(600.0), px(400.0)),
                        cx,
                    ))),
                    titlebar: Some(TitlebarOptions {
                        title: Some("Password Toggle Test".into()),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                |_window, cx| cx.new(|cx| PasswordTestApp::new(cx)),
            )
            .unwrap();
        });
}
