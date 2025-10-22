use adabraka_ui::{
    prelude::*,
    components::scrollable::scrollable_vertical,
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
                        title: Some("Toggle Styled Trait Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(900.0), px(800.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|_| ToggleStyledDemo::new()),
            )
            .unwrap();
        });
}

struct ToggleStyledDemo {
    toggle1: bool,
    toggle2: bool,
    toggle3: bool,
    toggle4: bool,
    toggle5: bool,
    toggle6: bool,
    toggle7: bool,
    toggle8: bool,
    toggle9: bool,
    toggle10: bool,
    toggle11: bool,
}

impl ToggleStyledDemo {
    fn new() -> Self {
        Self {
            toggle1: false,
            toggle2: false,
            toggle3: false,
            toggle4: false,
            toggle5: true,
            toggle6: false,
            toggle7: true,
            toggle8: false,
            toggle9: false,
            toggle10: true,
            toggle11: false,
        }
    }
}

impl Render for ToggleStyledDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
                            .child("Toggle Styled Trait Customization Demo")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("Demonstrating full GPUI styling control via Styled trait")
                    )
            )
            // 1. Custom Padding Examples
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("1. Custom Padding (via Styled trait)")
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(
                                Toggle::new("default-padding")
                                    .label("Default Padding")
                                    .checked(self.toggle1)
                                    .on_click(cx.listener(|view, checked, _, cx| {
                                        view.toggle1 = *checked;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                Toggle::new("custom-p4")
                                    .label("Custom p_4()")
                                    .checked(self.toggle2)
                                    .p_4()  // <- Styled trait method
                                    .on_click(cx.listener(|view, checked, _, cx| {
                                        view.toggle2 = *checked;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                Toggle::new("custom-p8")
                                    .label("Custom p_8()")
                                    .checked(self.toggle3)
                                    .p_8()  // <- Styled trait method
                                    .on_click(cx.listener(|view, checked, _, cx| {
                                        view.toggle3 = *checked;
                                        cx.notify();
                                    }))
                            )
                    )
            )
            // 2. Custom Background Colors
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("2. Custom Background Colors")
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(
                                Toggle::new("blue-bg")
                                    .label("Blue Background")
                                    .checked(self.toggle4)
                                    .bg(hsla(217.0 / 360.0, 0.91, 0.60, 0.1))  // <- Styled trait (blue with opacity)
                                    .rounded(px(8.0))
                                    .on_click(cx.listener(|view, checked, _, cx| {
                                        view.toggle4 = *checked;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                Toggle::new("purple-bg")
                                    .label("Purple Background")
                                    .checked(self.toggle5)
                                    .bg(hsla(271.0 / 360.0, 0.81, 0.66, 0.1))  // <- Styled trait (purple with opacity)
                                    .rounded(px(8.0))
                                    .on_click(cx.listener(|view, checked, _, cx| {
                                        view.toggle5 = *checked;
                                        cx.notify();
                                    }))
                            )
                    )
            )
            // 3. Custom Borders
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("3. Custom Borders")
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(
                                Toggle::new("border-2")
                                    .label("Border 2px")
                                    .checked(self.toggle6)
                                    .border_2()  // <- Styled trait
                                    .border_color(rgb(0x3b82f6))
                                    .rounded(px(8.0))
                                    .p_2()
                                    .on_click(cx.listener(|view, checked, _, cx| {
                                        view.toggle6 = *checked;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                Toggle::new("border-red")
                                    .label("Red Border")
                                    .checked(self.toggle7)
                                    .border_2()  // <- Styled trait
                                    .border_color(rgb(0xef4444))
                                    .rounded(px(8.0))
                                    .p_2()
                                    .on_click(cx.listener(|view, checked, _, cx| {
                                        view.toggle7 = *checked;
                                        cx.notify();
                                    }))
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
                                Toggle::new("no-radius")
                                    .label("No Radius")
                                    .checked(self.toggle8)
                                    .rounded(px(0.0))  // <- Styled trait
                                    .on_click(cx.listener(|view, checked, _, cx| {
                                        view.toggle8 = *checked;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                Toggle::new("rounded-lg")
                                    .label("Large Radius")
                                    .checked(self.toggle9)
                                    .rounded(px(16.0))  // <- Styled trait
                                    .on_click(cx.listener(|view, checked, _, cx| {
                                        view.toggle9 = *checked;
                                        cx.notify();
                                    }))
                            )
                    )
            )
            // 5. Width Control
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("5. Width Control")
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(
                                Toggle::new("full-width")
                                    .label("Full Width Toggle")
                                    .checked(self.toggle10)
                                    .w_full()  // <- Styled trait
                                    .bg(hsla(220.0 / 360.0, 0.70, 0.50, 0.1))
                                    .p_4()
                                    .rounded(px(8.0))
                                    .on_click(cx.listener(|view, checked, _, cx| {
                                        view.toggle10 = *checked;
                                        cx.notify();
                                    }))
                            )
                    )
            )
            // 6. Combined Styling
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("6. Combined Styling (Multiple Styled Trait Methods)")
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(
                                Toggle::new("combined-1")
                                    .label("Purple Box with Shadow and Large Padding")
                                    .checked(self.toggle11)
                                    .p_8()  // <- Styled trait
                                    .rounded(px(12.0))  // <- Styled trait
                                    .bg(hsla(271.0 / 360.0, 0.81, 0.66, 0.1))  // <- Styled trait (purple with opacity)
                                    .border_2()  // <- Styled trait
                                    .border_color(hsla(271.0 / 360.0, 0.81, 0.66, 1.0))
                                    .shadow_lg()  // <- Styled trait
                                    .on_click(cx.listener(|view, checked, _, cx| {
                                        view.toggle11 = *checked;
                                        cx.notify();
                                    }))
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
                            .child("Methods used: .p_2(), .p_4(), .p_8(), .bg(), .border_2(), .border_color(), .rounded(), .w_full(), .shadow_lg()")
                    )
            )
                )
            )
    }
}
