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
                        title: Some("Checkbox Styled Trait Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(900.0), px(800.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|_| CheckboxStyledDemo::new()),
            )
            .unwrap();
        });
}

struct CheckboxStyledDemo {
    checkbox1: bool,
    checkbox2: bool,
    checkbox3: bool,
    checkbox4: bool,
    checkbox5: bool,
    checkbox6: bool,
    checkbox7: bool,
    checkbox8: bool,
    checkbox9: bool,
    checkbox10: bool,
    checkbox11: bool,
}

impl CheckboxStyledDemo {
    fn new() -> Self {
        Self {
            checkbox1: false,
            checkbox2: false,
            checkbox3: false,
            checkbox4: false,
            checkbox5: true,
            checkbox6: false,
            checkbox7: true,
            checkbox8: false,
            checkbox9: false,
            checkbox10: true,
            checkbox11: false,
        }
    }
}

impl Render for CheckboxStyledDemo {
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
                            .child("Checkbox Styled Trait Customization Demo")
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
                                Checkbox::new("default-padding")
                                    .label("Default Padding")
                                    .checked(self.checkbox1)
                                    .on_click(cx.listener(|view, checked, _, cx| {
                                        view.checkbox1 = *checked;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                Checkbox::new("custom-p4")
                                    .label("Custom p_4()")
                                    .checked(self.checkbox2)
                                    .p_4()  // <- Styled trait method
                                    .on_click(cx.listener(|view, checked, _, cx| {
                                        view.checkbox2 = *checked;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                Checkbox::new("custom-p8")
                                    .label("Custom p_8()")
                                    .checked(self.checkbox3)
                                    .p_8()  // <- Styled trait method
                                    .on_click(cx.listener(|view, checked, _, cx| {
                                        view.checkbox3 = *checked;
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
                                Checkbox::new("blue-bg")
                                    .label("Blue Background")
                                    .checked(self.checkbox4)
                                    .bg(hsla(217.0 / 360.0, 0.91, 0.60, 0.1))  // <- Styled trait (blue with opacity)
                                    .rounded(px(8.0))
                                    .on_click(cx.listener(|view, checked, _, cx| {
                                        view.checkbox4 = *checked;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                Checkbox::new("purple-bg")
                                    .label("Purple Background")
                                    .checked(self.checkbox5)
                                    .bg(hsla(271.0 / 360.0, 0.81, 0.66, 0.1))  // <- Styled trait (purple with opacity)
                                    .rounded(px(8.0))
                                    .on_click(cx.listener(|view, checked, _, cx| {
                                        view.checkbox5 = *checked;
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
                                Checkbox::new("border-2")
                                    .label("Border 2px")
                                    .checked(self.checkbox6)
                                    .border_2()  // <- Styled trait
                                    .border_color(rgb(0x3b82f6))
                                    .rounded(px(8.0))
                                    .p_2()
                                    .on_click(cx.listener(|view, checked, _, cx| {
                                        view.checkbox6 = *checked;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                Checkbox::new("border-red")
                                    .label("Red Border")
                                    .checked(self.checkbox7)
                                    .border_2()  // <- Styled trait
                                    .border_color(rgb(0xef4444))
                                    .rounded(px(8.0))
                                    .p_2()
                                    .on_click(cx.listener(|view, checked, _, cx| {
                                        view.checkbox7 = *checked;
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
                                Checkbox::new("no-radius")
                                    .label("No Radius")
                                    .checked(self.checkbox8)
                                    .rounded(px(0.0))  // <- Styled trait
                                    .on_click(cx.listener(|view, checked, _, cx| {
                                        view.checkbox8 = *checked;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                Checkbox::new("rounded-lg")
                                    .label("Large Radius")
                                    .checked(self.checkbox9)
                                    .rounded(px(16.0))  // <- Styled trait
                                    .on_click(cx.listener(|view, checked, _, cx| {
                                        view.checkbox9 = *checked;
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
                                Checkbox::new("full-width")
                                    .label("Full Width Checkbox")
                                    .checked(self.checkbox10)
                                    .w_full()  // <- Styled trait
                                    .bg(hsla(220.0 / 360.0, 0.70, 0.50, 0.1))
                                    .p_4()
                                    .rounded(px(8.0))
                                    .on_click(cx.listener(|view, checked, _, cx| {
                                        view.checkbox10 = *checked;
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
                                Checkbox::new("combined-1")
                                    .label("Purple Box with Shadow and Large Padding")
                                    .checked(self.checkbox11)
                                    .p_8()  // <- Styled trait
                                    .rounded(px(12.0))  // <- Styled trait
                                    .bg(hsla(271.0 / 360.0, 0.81, 0.66, 0.1))  // <- Styled trait (purple with opacity)
                                    .border_2()  // <- Styled trait
                                    .border_color(hsla(271.0 / 360.0, 0.81, 0.66, 1.0))
                                    .shadow_lg()  // <- Styled trait
                                    .on_click(cx.listener(|view, checked, _, cx| {
                                        view.checkbox11 = *checked;
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
