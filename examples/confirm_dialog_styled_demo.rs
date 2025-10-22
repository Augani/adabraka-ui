use adabraka_ui::{
    prelude::*,
    components::{
        scrollable::scrollable_vertical,
        confirm_dialog::Dialog,
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
                        title: Some("Dialog Styled Trait Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(900.0), px(800.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|_| DialogStyledDemo::new()),
            )
            .unwrap();
        });
}

struct DialogStyledDemo {
    show_default: bool,
    show_custom_size: bool,
    show_custom_colors: bool,
    show_custom_radius: bool,
    show_custom_shadow: bool,
    show_custom_border: bool,
    show_combined: bool,
}

impl DialogStyledDemo {
    fn new() -> Self {
        Self {
            show_default: false,
            show_custom_size: false,
            show_custom_colors: false,
            show_custom_radius: false,
            show_custom_shadow: false,
            show_custom_border: false,
            show_combined: false,
        }
    }
}

impl Render for DialogStyledDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        let mut root = div()
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
                                        .child("Dialog Styled Trait Customization Demo")
                                )
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child("Demonstrating full GPUI styling control via Styled trait for Dialog component")
                                )
                        )
                        // 1. Default Dialog
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("1. Default Dialog")
                                )
                                .child(
                                    Button::new("show-default", "Show Default Dialog")
                                        .variant(ButtonVariant::Default)
                                        .on_click(cx.listener(|view, _, _, cx| {
                                            view.show_default = true;
                                            cx.notify();
                                        }))
                                )
                        )
                        // 2. Custom Size
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("2. Custom Size (via Styled trait)")
                                )
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child("Using .w(), .min_h() methods")
                                )
                                .child(
                                    Button::new("show-custom-size", "Show Custom Size Dialog")
                                        .variant(ButtonVariant::Default)
                                        .on_click(cx.listener(|view, _, _, cx| {
                                            view.show_custom_size = true;
                                            cx.notify();
                                        }))
                                )
                        )
                        // 3. Custom Colors
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("3. Custom Colors (via Styled trait)")
                                )
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child("Using .bg() and .border_color() methods")
                                )
                                .child(
                                    Button::new("show-custom-colors", "Show Custom Colors Dialog")
                                        .variant(ButtonVariant::Default)
                                        .on_click(cx.listener(|view, _, _, cx| {
                                            view.show_custom_colors = true;
                                            cx.notify();
                                        }))
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
                                        .child("4. Custom Border Radius (via Styled trait)")
                                )
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child("Using .rounded() method")
                                )
                                .child(
                                    Button::new("show-custom-radius", "Show Custom Radius Dialog")
                                        .variant(ButtonVariant::Default)
                                        .on_click(cx.listener(|view, _, _, cx| {
                                            view.show_custom_radius = true;
                                            cx.notify();
                                        }))
                                )
                        )
                        // 5. Custom Shadow
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("5. Custom Shadow (via Styled trait)")
                                )
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child("Using .shadow_2xl() method for larger shadow")
                                )
                                .child(
                                    Button::new("show-custom-shadow", "Show Custom Shadow Dialog")
                                        .variant(ButtonVariant::Default)
                                        .on_click(cx.listener(|view, _, _, cx| {
                                            view.show_custom_shadow = true;
                                            cx.notify();
                                        }))
                                )
                        )
                        // 6. Custom Border
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("6. Custom Border (via Styled trait)")
                                )
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child("Using .border_4() and .border_color() methods")
                                )
                                .child(
                                    Button::new("show-custom-border", "Show Custom Border Dialog")
                                        .variant(ButtonVariant::Default)
                                        .on_click(cx.listener(|view, _, _, cx| {
                                            view.show_custom_border = true;
                                            cx.notify();
                                        }))
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
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child("Combining multiple style methods for complete customization")
                                )
                                .child(
                                    Button::new("show-combined", "Show Combined Styling Dialog")
                                        .variant(ButtonVariant::Default)
                                        .on_click(cx.listener(|view, _, _, cx| {
                                            view.show_combined = true;
                                            cx.notify();
                                        }))
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
                                        .child("All dialog customizations use the Styled trait for full GPUI styling control!")
                                )
                                .child(
                                    div()
                                        .mt(px(8.0))
                                        .text_size(px(12.0))
                                        .text_color(theme.tokens.accent_foreground)
                                        .child("Methods used: .w(), .min_h(), .bg(), .border_color(), .border_4(), .rounded(), .shadow_2xl(), .p()")
                                )
                        )
                )
            );

        // Render dialogs conditionally
        if self.show_default {
            root = root.child(
                Dialog::new()
                    .header(
                        div()
                            .p(px(24.0))
                            .border_b_1()
                            .border_color(theme.tokens.border)
                            .child(
                                div()
                                    .text_size(px(18.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("Default Dialog")
                            )
                    )
                    .content(
                        div()
                            .p(px(24.0))
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .child("This is a default dialog with no custom styling.")
                            )
                    )
                    .footer(
                        div()
                            .p(px(24.0))
                            .border_t_1()
                            .border_color(theme.tokens.border)
                            .flex()
                            .justify_end()
                            .gap(px(12.0))
                            .child(
                                Button::new("default-close", "Close")
                                    .variant(ButtonVariant::Outline)
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.show_default = false;
                                        cx.notify();
                                    }))
                            )
                    )
            );
        }

        if self.show_custom_size {
            root = root.child(
                Dialog::new()
                    .width(px(700.0))
                    .w(px(700.0))  // Custom width via Styled trait
                    .min_h(px(400.0))  // Custom min height via Styled trait
                    .header(
                        div()
                            .p(px(24.0))
                            .border_b_1()
                            .border_color(theme.tokens.border)
                            .child(
                                div()
                                    .text_size(px(18.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("Custom Size Dialog")
                            )
                    )
                    .content(
                        div()
                            .p(px(24.0))
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .child("This dialog has custom dimensions set via Styled trait methods: .w(px(700.0)) and .min_h(px(400.0))")
                            )
                    )
                    .footer(
                        div()
                            .p(px(24.0))
                            .border_t_1()
                            .border_color(theme.tokens.border)
                            .flex()
                            .justify_end()
                            .gap(px(12.0))
                            .child(
                                Button::new("size-close", "Close")
                                    .variant(ButtonVariant::Outline)
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.show_custom_size = false;
                                        cx.notify();
                                    }))
                            )
                    )
            );
        }

        if self.show_custom_colors {
            root = root.child(
                Dialog::new()
                    .bg(rgb(0x1e293b))  // Custom dark blue background via Styled trait
                    .border_color(rgb(0x3b82f6))  // Custom blue border via Styled trait
                    .header(
                        div()
                            .p(px(24.0))
                            .border_b_1()
                            .border_color(rgb(0x3b82f6))
                            .child(
                                div()
                                    .text_size(px(18.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(0x60a5fa))
                                    .child("Custom Colors Dialog")
                            )
                    )
                    .content(
                        div()
                            .p(px(24.0))
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(rgb(0xcbd5e1))
                                    .child("This dialog has custom colors: .bg(rgb(0x1e293b)) and .border_color(rgb(0x3b82f6))")
                            )
                    )
                    .footer(
                        div()
                            .p(px(24.0))
                            .border_t_1()
                            .border_color(rgb(0x3b82f6))
                            .flex()
                            .justify_end()
                            .gap(px(12.0))
                            .child(
                                Button::new("colors-close", "Close")
                                    .variant(ButtonVariant::Outline)
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.show_custom_colors = false;
                                        cx.notify();
                                    }))
                            )
                    )
            );
        }

        if self.show_custom_radius {
            root = root.child(
                Dialog::new()
                    .rounded(px(0.0))  // No border radius via Styled trait
                    .header(
                        div()
                            .p(px(24.0))
                            .border_b_1()
                            .border_color(theme.tokens.border)
                            .child(
                                div()
                                    .text_size(px(18.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("Custom Border Radius Dialog")
                            )
                    )
                    .content(
                        div()
                            .p(px(24.0))
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .child("This dialog has sharp corners with .rounded(px(0.0)) - no border radius!")
                            )
                    )
                    .footer(
                        div()
                            .p(px(24.0))
                            .border_t_1()
                            .border_color(theme.tokens.border)
                            .flex()
                            .justify_end()
                            .gap(px(12.0))
                            .child(
                                Button::new("radius-close", "Close")
                                    .variant(ButtonVariant::Outline)
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.show_custom_radius = false;
                                        cx.notify();
                                    }))
                            )
                    )
            );
        }

        if self.show_custom_shadow {
            root = root.child(
                Dialog::new()
                    .shadow_2xl()  // Extra large shadow via Styled trait
                    .header(
                        div()
                            .p(px(24.0))
                            .border_b_1()
                            .border_color(theme.tokens.border)
                            .child(
                                div()
                                    .text_size(px(18.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("Custom Shadow Dialog")
                            )
                    )
                    .content(
                        div()
                            .p(px(24.0))
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .child("This dialog has an extra large shadow with .shadow_2xl() for enhanced depth!")
                            )
                    )
                    .footer(
                        div()
                            .p(px(24.0))
                            .border_t_1()
                            .border_color(theme.tokens.border)
                            .flex()
                            .justify_end()
                            .gap(px(12.0))
                            .child(
                                Button::new("shadow-close", "Close")
                                    .variant(ButtonVariant::Outline)
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.show_custom_shadow = false;
                                        cx.notify();
                                    }))
                            )
                    )
            );
        }

        if self.show_custom_border {
            root = root.child(
                Dialog::new()
                    .border_4()  // Thick border via Styled trait
                    .border_color(rgb(0x8b5cf6))  // Purple border via Styled trait
                    .header(
                        div()
                            .p(px(24.0))
                            .border_b_1()
                            .border_color(rgb(0x8b5cf6))
                            .child(
                                div()
                                    .text_size(px(18.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(0x8b5cf6))
                                    .child("Custom Border Dialog")
                            )
                    )
                    .content(
                        div()
                            .p(px(24.0))
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .child("This dialog has a thick purple border with .border_4() and .border_color(rgb(0x8b5cf6))")
                            )
                    )
                    .footer(
                        div()
                            .p(px(24.0))
                            .border_t_1()
                            .border_color(rgb(0x8b5cf6))
                            .flex()
                            .justify_end()
                            .gap(px(12.0))
                            .child(
                                Button::new("border-close", "Close")
                                    .variant(ButtonVariant::Outline)
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.show_custom_border = false;
                                        cx.notify();
                                    }))
                            )
                    )
            );
        }

        if self.show_combined {
            root = root.child(
                Dialog::new()
                    .w(px(600.0))  // Custom width
                    .min_h(px(350.0))  // Custom min height
                    .bg(rgb(0x0f172a))  // Dark background
                    .border_4()  // Thick border
                    .border_color(rgb(0x10b981))  // Green border
                    .rounded(px(24.0))  // Large border radius
                    .shadow_2xl()  // Extra large shadow
                    .p(px(4.0))  // Internal padding
                    .header(
                        div()
                            .p(px(24.0))
                            .border_b_1()
                            .border_color(rgb(0x10b981))
                            .child(
                                div()
                                    .text_size(px(18.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(0x10b981))
                                    .child("Combined Styling Dialog")
                            )
                    )
                    .content(
                        div()
                            .p(px(24.0))
                            .child(
                                VStack::new()
                                    .gap(px(12.0))
                                    .child(
                                        div()
                                            .text_size(px(14.0))
                                            .text_color(rgb(0xe2e8f0))
                                            .child("This dialog combines multiple Styled trait methods:")
                                    )
                                    .child(
                                        div()
                                            .text_size(px(13.0))
                                            .text_color(rgb(0x94a3b8))
                                            .child("- .w(px(600.0)) - Custom width")
                                    )
                                    .child(
                                        div()
                                            .text_size(px(13.0))
                                            .text_color(rgb(0x94a3b8))
                                            .child("- .min_h(px(350.0)) - Minimum height")
                                    )
                                    .child(
                                        div()
                                            .text_size(px(13.0))
                                            .text_color(rgb(0x94a3b8))
                                            .child("- .bg(rgb(0x0f172a)) - Dark background")
                                    )
                                    .child(
                                        div()
                                            .text_size(px(13.0))
                                            .text_color(rgb(0x94a3b8))
                                            .child("- .border_4() - Thick border")
                                    )
                                    .child(
                                        div()
                                            .text_size(px(13.0))
                                            .text_color(rgb(0x94a3b8))
                                            .child("- .border_color(rgb(0x10b981)) - Green border")
                                    )
                                    .child(
                                        div()
                                            .text_size(px(13.0))
                                            .text_color(rgb(0x94a3b8))
                                            .child("- .rounded(px(24.0)) - Large border radius")
                                    )
                                    .child(
                                        div()
                                            .text_size(px(13.0))
                                            .text_color(rgb(0x94a3b8))
                                            .child("- .shadow_2xl() - Extra large shadow")
                                    )
                                    .child(
                                        div()
                                            .text_size(px(13.0))
                                            .text_color(rgb(0x94a3b8))
                                            .child("- .p(px(4.0)) - Internal padding")
                                    )
                            )
                    )
                    .footer(
                        div()
                            .p(px(24.0))
                            .border_t_1()
                            .border_color(rgb(0x10b981))
                            .flex()
                            .justify_end()
                            .gap(px(12.0))
                            .child(
                                Button::new("combined-close", "Close")
                                    .variant(ButtonVariant::Outline)
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.show_combined = false;
                                        cx.notify();
                                    }))
                            )
                    )
            );
        }

        root
    }
}
