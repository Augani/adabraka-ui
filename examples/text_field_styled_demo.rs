use adabraka_ui::{
    prelude::*,
    components::{
        scrollable::scrollable_vertical,
        input::{Input, InputState},
    },
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
                        title: Some("Input Styled Trait Demo (TextField Component)".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(900.0), px(800.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| {
                    // Create Input states 
                    let states: Vec<Entity<InputState>> = (0..10)
                        .map(|_| cx.new(|cx| InputState::new(cx)))
                        .collect();
                    
                    cx.new(|_| InputStyledDemo::new(states))
                },
            )
            .unwrap();
        });
}

struct InputStyledDemo {
    input_states: Vec<Entity<InputState>>,
}

impl InputStyledDemo {
    fn new(states: Vec<Entity<InputState>>) -> Self {
        Self {
            input_states: states,
        }
    }
}

impl Render for InputStyledDemo {
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
                        .p(px(32.0))
                        .gap(px(32.0))
                        // Header
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap(px(8.0))
                                .child(
                                    div()
                                        .text_size(px(24.0))
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(theme.tokens.foreground)
                                        .child("Input/TextField Styled Trait Demo")
                                )
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child("Demonstrating full GPUI styling control via Styled trait")
                                )
                        )
                        // Section 1: Custom Width
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(16.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(theme.tokens.foreground)
                                        .child("1. Custom Width")
                                )
                                .child(
                                    Input::new(&self.input_states[0])
                                        .placeholder("Full width input field")
                                        .w_full()
                                )
                                .child(
                                    Input::new(&self.input_states[1])
                                        .placeholder("Custom width 300px")
                                        .w(px(300.0))
                                )
                        )
                        // Section 2: Custom Padding
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(16.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(theme.tokens.foreground)
                                        .child("2. Custom Padding")
                                )
                                .child(
                                    Input::new(&self.input_states[2])
                                        .placeholder("Large padding with .p_8()")
                                        .p_8()
                                        .w(px(400.0))
                                )
                        )
                        // Section 3: Custom Background
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(16.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(theme.tokens.foreground)
                                        .child("3. Custom Background")
                                )
                                .child(
                                    Input::new(&self.input_states[3])
                                        .placeholder("Dark blue background")
                                        .bg(rgb(0x1e3a8a))
                                        .w(px(400.0))
                                )
                        )
                        // Section 4: Custom Borders
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(16.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(theme.tokens.foreground)
                                        .child("4. Custom Borders")
                                )
                                .child(
                                    Input::new(&self.input_states[4])
                                        .placeholder("Thick blue border")
                                        .border_4()
                                        .border_color(rgb(0x3b82f6))
                                        .w(px(400.0))
                                )
                        )
                        // Section 5: Custom Border Radius
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(16.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(theme.tokens.foreground)
                                        .child("5. Custom Border Radius")
                                )
                                .child(
                                    Input::new(&self.input_states[5])
                                        .placeholder("Large rounded corners")
                                        .rounded(px(16.0))
                                        .w(px(400.0))
                                )
                        )
                        // Section 6: Shadow Effects
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(16.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(theme.tokens.foreground)
                                        .child("6. Shadow Effects")
                                )
                                .child(
                                    Input::new(&self.input_states[6])
                                        .placeholder("Large shadow effect")
                                        .shadow_lg()
                                        .w(px(400.0))
                                )
                        )
                        // Section 7: Combined Styling
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(16.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(theme.tokens.foreground)
                                        .child("7. Combined Custom Styling")
                                )
                                .child(
                                    Input::new(&self.input_states[7])
                                        .placeholder("Ultra styled input field")
                                        .p_8()
                                        .bg(rgb(0x581c87))
                                        .border_2()
                                        .border_color(rgb(0xa855f7))
                                        .rounded(px(12.0))
                                        .shadow_md()
                                        .w(px(500.0))
                                )
                        )
                        // Info Box
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
                                        .child("All customizations use the Styled trait for full GPUI styling control!")
                                )
                                .child(
                                    div()
                                        .mt(px(8.0))
                                        .text_size(px(12.0))
                                        .text_color(theme.tokens.accent_foreground)
                                        .child("Note: Using Input component (TextField uses same Styled trait pattern)")
                                )
                        )
                )
            )
    }
}
