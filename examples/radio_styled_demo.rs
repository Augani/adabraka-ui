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
                        title: Some("Radio Styled Trait Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(900.0), px(800.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|_| RadioStyledDemo::new()),
            )
            .unwrap();
        });
}

struct RadioStyledDemo {
    selection1: Option<usize>,
    selection2: Option<usize>,
    selection3: Option<usize>,
    selection4: Option<usize>,
    selection5: Option<usize>,
    selection6: Option<usize>,
}

impl RadioStyledDemo {
    fn new() -> Self {
        Self {
            selection1: Some(0),
            selection2: Some(1),
            selection3: Some(0),
            selection4: Some(2),
            selection5: Some(1),
            selection6: Some(0),
        }
    }
}

impl Render for RadioStyledDemo {
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
                            .child("Radio Styled Trait Customization Demo")
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
                        RadioGroup::new("padding-group")
                            .selected_index(self.selection1)
                            .on_change(cx.listener(|view, index, _, cx| {
                                view.selection1 = Some(*index);
                                cx.notify();
                            }))
                            .child(Radio::new("default").label("Default Padding"))
                            .child(
                                Radio::new("p4")
                                    .label("Custom p_4()")
                                    .p_4()  // <- Styled trait method
                            )
                            .child(
                                Radio::new("p8")
                                    .label("Custom p_8()")
                                    .p_8()  // <- Styled trait method
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
                        RadioGroup::new("bg-group")
                            .selected_index(self.selection2)
                            .on_change(cx.listener(|view, index, _, cx| {
                                view.selection2 = Some(*index);
                                cx.notify();
                            }))
                            .child(
                                Radio::new("blue-bg")
                                    .label("Blue Background")
                                    .bg(hsla(217.0 / 360.0, 0.91, 0.60, 0.1))  // <- Styled trait (blue with opacity)
                                    .rounded(px(8.0))
                            )
                            .child(
                                Radio::new("purple-bg")
                                    .label("Purple Background")
                                    .bg(hsla(271.0 / 360.0, 0.81, 0.66, 0.1))  // <- Styled trait (purple with opacity)
                                    .rounded(px(8.0))
                            )
                            .child(
                                Radio::new("green-bg")
                                    .label("Green Background")
                                    .bg(hsla(142.0 / 360.0, 0.76, 0.45, 0.1))  // <- Styled trait (green with opacity)
                                    .rounded(px(8.0))
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
                        RadioGroup::new("border-group")
                            .selected_index(self.selection3)
                            .on_change(cx.listener(|view, index, _, cx| {
                                view.selection3 = Some(*index);
                                cx.notify();
                            }))
                            .child(
                                Radio::new("border-blue")
                                    .label("Blue Border 2px")
                                    .border_2()  // <- Styled trait
                                    .border_color(rgb(0x3b82f6))
                                    .rounded(px(8.0))
                                    .p_2()
                            )
                            .child(
                                Radio::new("border-red")
                                    .label("Red Border 2px")
                                    .border_2()  // <- Styled trait
                                    .border_color(rgb(0xef4444))
                                    .rounded(px(8.0))
                                    .p_2()
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
                        RadioGroup::new("radius-group")
                            .selected_index(self.selection4)
                            .on_change(cx.listener(|view, index, _, cx| {
                                view.selection4 = Some(*index);
                                cx.notify();
                            }))
                            .child(
                                Radio::new("no-radius")
                                    .label("No Radius")
                                    .rounded(px(0.0))  // <- Styled trait
                            )
                            .child(
                                Radio::new("medium-radius")
                                    .label("Medium Radius")
                                    .rounded(px(8.0))  // <- Styled trait
                            )
                            .child(
                                Radio::new("large-radius")
                                    .label("Large Radius")
                                    .rounded(px(16.0))  // <- Styled trait
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
                        RadioGroup::new("width-group")
                            .selected_index(self.selection5)
                            .on_change(cx.listener(|view, index, _, cx| {
                                view.selection5 = Some(*index);
                                cx.notify();
                            }))
                            .child(
                                Radio::new("full-width-1")
                                    .label("Full Width Radio 1")
                                    .w_full()  // <- Styled trait
                                    .bg(hsla(220.0 / 360.0, 0.70, 0.50, 0.1))
                                    .p_4()
                                    .rounded(px(8.0))
                            )
                            .child(
                                Radio::new("full-width-2")
                                    .label("Full Width Radio 2")
                                    .w_full()  // <- Styled trait
                                    .bg(hsla(220.0 / 360.0, 0.70, 0.50, 0.1))
                                    .p_4()
                                    .rounded(px(8.0))
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
                        RadioGroup::new("combined-group")
                            .selected_index(self.selection6)
                            .on_change(cx.listener(|view, index, _, cx| {
                                view.selection6 = Some(*index);
                                cx.notify();
                            }))
                            .child(
                                Radio::new("combined-1")
                                    .label("Purple Box with Shadow and Large Padding")
                                    .p_8()  // <- Styled trait
                                    .rounded(px(12.0))  // <- Styled trait
                                    .bg(hsla(271.0 / 360.0, 0.81, 0.66, 0.1))  // <- Styled trait (purple with opacity)
                                    .border_2()  // <- Styled trait
                                    .border_color(hsla(271.0 / 360.0, 0.81, 0.66, 1.0))
                                    .shadow_lg()  // <- Styled trait
                            )
                            .child(
                                Radio::new("combined-2")
                                    .label("Blue Box with Different Styling")
                                    .p_6()  // <- Styled trait
                                    .rounded(px(8.0))  // <- Styled trait
                                    .bg(hsla(217.0 / 360.0, 0.91, 0.60, 0.1))  // <- Styled trait (blue with opacity)
                                    .border_2()  // <- Styled trait
                                    .border_color(hsla(217.0 / 360.0, 0.91, 0.60, 1.0))
                                    .shadow_md()  // <- Styled trait
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
                            .child("Methods used: .p_2(), .p_4(), .p_6(), .p_8(), .bg(), .border_2(), .border_color(), .rounded(), .w_full(), .shadow_lg(), .shadow_md()")
                    )
            )
                )
            )
    }
}
