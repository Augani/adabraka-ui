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
                        title: Some("Button Styled Trait Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(900.0), px(800.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|_| ButtonStyledDemo::new()),
            )
            .unwrap();
        });
}

struct ButtonStyledDemo {
    click_count: usize,
}

impl ButtonStyledDemo {
    fn new() -> Self {
        Self {
            click_count: 0,
        }
    }
}

impl Render for ButtonStyledDemo {
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
                            .child("Button Styled Trait Customization Demo")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("Demonstrating full GPUI styling control via Styled trait")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.accent)
                            .child(format!("Total clicks: {}", self.click_count))
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
                        HStack::new()
                            .gap(px(12.0))
                            .items_center()
                            .child(
                                Button::new("default-padding", "Default")
                                    .variant(ButtonVariant::Default)
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                Button::new("custom-p4", "Custom p_4()")
                                    .variant(ButtonVariant::Default)
                                    .p_4()  // ← Styled trait method
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                Button::new("custom-p8", "Custom p_8()")
                                    .variant(ButtonVariant::Default)
                                    .p_8()  // ← Styled trait method
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                Button::new("custom-px", "Custom px(32)")
                                    .variant(ButtonVariant::Default)
                                    .px(px(32.0))  // ← Styled trait method
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
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
                        HStack::new()
                            .gap(px(12.0))
                            .child(
                                Button::new("blue-bg", "Blue Background")
                                    .variant(ButtonVariant::Ghost)
                                    .bg(rgb(0x3b82f6))  // ← Styled trait
                                    .text_color(gpui::white())
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                Button::new("purple-bg", "Purple Background")
                                    .variant(ButtonVariant::Ghost)
                                    .bg(rgb(0x8b5cf6))  // ← Styled trait
                                    .text_color(gpui::white())
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                Button::new("green-bg", "Green Background")
                                    .variant(ButtonVariant::Ghost)
                                    .bg(rgb(0x10b981))  // ← Styled trait
                                    .text_color(gpui::white())
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
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
                        HStack::new()
                            .gap(px(12.0))
                            .child(
                                Button::new("border-2", "Border 2px")
                                    .variant(ButtonVariant::Outline)
                                    .border_2()  // ← Styled trait
                                    .border_color(rgb(0x3b82f6))
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                Button::new("border-red", "Red Border")
                                    .variant(ButtonVariant::Outline)
                                    .border_2()  // ← Styled trait
                                    .border_color(rgb(0xef4444))
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                Button::new("border-dashed", "Dashed Border")
                                    .variant(ButtonVariant::Outline)
                                    .border_2()  // ← Styled trait
                                    .border_color(rgb(0x8b5cf6))
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
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
                        HStack::new()
                            .gap(px(12.0))
                            .child(
                                Button::new("no-radius", "No Radius")
                                    .variant(ButtonVariant::Default)
                                    .rounded(px(0.0))  // ← Styled trait
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                Button::new("rounded-lg", "Large Radius")
                                    .variant(ButtonVariant::Default)
                                    .rounded(px(16.0))  // ← Styled trait
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                Button::new("pill", "Pill Shape")
                                    .variant(ButtonVariant::Default)
                                    .rounded(px(999.0))  // ← Styled trait
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
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
                                Button::new("full-width", "Full Width Button")
                                    .variant(ButtonVariant::Default)
                                    .w_full()  // ← Styled trait
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                Button::new("custom-width", "Custom Width (300px)")
                                    .variant(ButtonVariant::Secondary)
                                    .w(px(300.0))  // ← Styled trait
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
                            )
                    )
            )
            // 6. Shadow Effects
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("6. Shadow Effects")
                    )
                    .child(
                        HStack::new()
                            .gap(px(12.0))
                            .child(
                                Button::new("shadow-sm", "Shadow Small")
                                    .variant(ButtonVariant::Default)
                                    .shadow_sm()  // ← Styled trait
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                Button::new("shadow-md", "Shadow Medium")
                                    .variant(ButtonVariant::Default)
                                    .shadow_md()  // ← Styled trait
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                Button::new("shadow-lg", "Shadow Large")
                                    .variant(ButtonVariant::Default)
                                    .shadow_lg()  // ← Styled trait
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
                            )
                    )
            )
            // 7. Combined Styling
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("7. Combined Styling (Multiple Styled Trait Methods)")
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(
                                Button::new("combined-1", "Purple Pill with Shadow")
                                    .variant(ButtonVariant::Ghost)
                                    .p_8()  // ← Styled trait
                                    .rounded(px(999.0))  // ← Styled trait
                                    .bg(rgb(0x8b5cf6))  // ← Styled trait
                                    .text_color(gpui::white())
                                    .shadow_lg()  // ← Styled trait
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                Button::new("combined-2", "Full Width Custom Button")
                                    .variant(ButtonVariant::Outline)
                                    .w_full()  // ← Styled trait
                                    .p(px(20.0))  // ← Styled trait
                                    .border_2()  // ← Styled trait
                                    .border_color(rgb(0x10b981))
                                    .rounded(px(12.0))  // ← Styled trait
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                Button::new("combined-3", "Ultra Custom Button")
                                    .variant(ButtonVariant::Ghost)
                                    .px(px(40.0))  // ← Styled trait
                                    .py(px(16.0))  // ← Styled trait
                                    .bg(rgb(0xf59e0b))  // ← Styled trait
                                    .text_color(gpui::white())
                                    .rounded(px(8.0))  // ← Styled trait
                                    .shadow_md()  // ← Styled trait
                                    .w(px(400.0))  // ← Styled trait
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
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
                            .child("✅ All customizations above use the Styled trait for full GPUI styling control!")
                    )
                    .child(
                        div()
                            .mt(px(8.0))
                            .text_size(px(12.0))
                            .text_color(theme.tokens.accent_foreground)
                            .child("Methods used: .p_4(), .p_8(), .px(), .bg(), .border_2(), .rounded(), .w_full(), .w(), .shadow_sm/md/lg()")
                    )
            )
                )
            )
    }
}
