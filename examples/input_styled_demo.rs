use adabraka_ui::{
    prelude::*,
    components::scrollable::scrollable_vertical,
    components::input::{Input, InputState, InputVariant},
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
                        title: Some("Input Styled Trait Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(900.0), px(800.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|cx| InputStyledDemo::new(cx)),
            )
            .unwrap();
        });
}

struct InputStyledDemo {
    input1: Entity<InputState>,
    input2: Entity<InputState>,
    input3: Entity<InputState>,
    input4: Entity<InputState>,
    input5: Entity<InputState>,
    input6: Entity<InputState>,
    input7: Entity<InputState>,
    input8: Entity<InputState>,
    input9: Entity<InputState>,
    input10: Entity<InputState>,
    input11: Entity<InputState>,
    input12: Entity<InputState>,
}

impl InputStyledDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            input1: cx.new(|cx| InputState::new(cx)),
            input2: cx.new(|cx| InputState::new(cx)),
            input3: cx.new(|cx| InputState::new(cx)),
            input4: cx.new(|cx| InputState::new(cx)),
            input5: cx.new(|cx| InputState::new(cx)),
            input6: cx.new(|cx| InputState::new(cx)),
            input7: cx.new(|cx| InputState::new(cx)),
            input8: cx.new(|cx| InputState::new(cx)),
            input9: cx.new(|cx| InputState::new(cx)),
            input10: cx.new(|cx| InputState::new(cx)),
            input11: cx.new(|cx| InputState::new(cx)),
            input12: cx.new(|cx| InputState::new(cx)),
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
                        .text_color(theme.tokens.foreground)
                        .p(px(32.0))
                        .gap(px(32.0))
            .child(
                VStack::new()
                    .gap(px(8.0))
                    .child(
                        div()
                            .text_size(px(24.0))
                            .font_weight(FontWeight::BOLD)
                            .child("Input Styled Trait Customization Demo")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("Demonstrating full GPUI styling control via Styled trait")
                    )
            )
            // 1. Custom Width Examples
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("1. Custom Width (via Styled trait)")
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(
                                Input::new(&self.input1)
                                    .placeholder("Default width")
                                    .variant(InputVariant::Default)
                            )
                            .child(
                                Input::new(&self.input2)
                                    .placeholder("Full width input")
                                    .variant(InputVariant::Default)
                                    .w_full()  // Styled trait method
                            )
                            .child(
                                Input::new(&self.input3)
                                    .placeholder("Custom width (600px)")
                                    .variant(InputVariant::Default)
                                    .w(px(600.0))  // Styled trait method
                            )
                    )
            )
            // 2. Custom Padding
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("2. Custom Padding")
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(
                                Input::new(&self.input4)
                                    .placeholder("Default padding")
                                    .variant(InputVariant::Outline)
                            )
                            .child(
                                Input::new(&self.input5)
                                    .placeholder("Extra padding - p_4()")
                                    .variant(InputVariant::Outline)
                                    .p_4()  // Styled trait method
                            )
                            .child(
                                Input::new(&self.input6)
                                    .placeholder("More padding - p_8()")
                                    .variant(InputVariant::Outline)
                                    .p_8()  // Styled trait method
                            )
                    )
            )
            // 3. Custom Background Colors
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("3. Custom Background Colors")
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(
                                Input::new(&self.input7)
                                    .placeholder("Blue background")
                                    .variant(InputVariant::Ghost)
                                    .bg(rgb(0x1e3a8a))  // Styled trait
                                    .text_color(gpui::white())
                            )
                            .child(
                                Input::new(&self.input8)
                                    .placeholder("Purple background")
                                    .variant(InputVariant::Ghost)
                                    .bg(rgb(0x581c87))  // Styled trait
                                    .text_color(gpui::white())
                            )
                    )
            )
            // 4. Custom Border Radius
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("4. Custom Border Radius")
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(
                                Input::new(&self.input9)
                                    .placeholder("No radius (sharp corners)")
                                    .variant(InputVariant::Default)
                                    .rounded(px(0.0))  // Styled trait
                            )
                            .child(
                                Input::new(&self.input10)
                                    .placeholder("Large radius (20px)")
                                    .variant(InputVariant::Default)
                                    .rounded(px(20.0))  // Styled trait
                            )
                    )
            )
            // 5. Combined Styling
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("5. Combined Styling (Multiple Styled Trait Methods)")
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(
                                Input::new(&self.input11)
                                    .placeholder("Fully customized input")
                                    .variant(InputVariant::Ghost)
                                    .w_full()  // Styled trait
                                    .p(px(20.0))  // Styled trait
                                    .bg(rgb(0x047857))  // Styled trait
                                    .text_color(gpui::white())
                                    .rounded(px(12.0))  // Styled trait
                                    .shadow_lg()  // Styled trait
                            )
                            .child(
                                Input::new(&self.input12)
                                    .placeholder("Ultra custom with border")
                                    .variant(InputVariant::Outline)
                                    .w_full()  // Styled trait
                                    .px(px(24.0))  // Styled trait
                                    .py(px(16.0))  // Styled trait
                                    .bg(hsla(43.0 / 360.0, 0.96, 0.56, 0.2))  // Styled trait
                                    .rounded(px(16.0))  // Styled trait
                                    .border_2()  // Styled trait
                                    .border_color(rgb(0xfbbf24))
                            )
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
                            .child("All customizations above use the Styled trait for full GPUI styling control!")
                    )
                    .child(
                        div()
                            .mt(px(8.0))
                            .text_size(px(12.0))
                            .text_color(theme.tokens.accent_foreground)
                            .child("Methods used: .w_full(), .w(), .p_4(), .p_8(), .p(), .px(), .py(), .bg(), .rounded(), .border_2(), .shadow_lg()")
                    )
            )
                )
            )
    }
}
