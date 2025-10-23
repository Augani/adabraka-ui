use adabraka_ui::{
    prelude::*,
    components::scrollable::scrollable_vertical,
    overlays::toast::{ToastManager, ToastItem, ToastVariant},
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
                        title: Some("Toast Styled Trait Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(900.0), px(800.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|cx| ToastStyledDemo::new(cx)),
            )
            .unwrap();
        });
}

struct ToastStyledDemo {
    toast_manager: Entity<ToastManager>,
    next_id: u64,
}

impl ToastStyledDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            toast_manager: cx.new(|cx| ToastManager::new(cx)),
            next_id: 0,
        }
    }

    fn get_next_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    fn show_default_toast(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let id = self.get_next_id();
        let toast = ToastItem::new(id, "Default Toast")
            .description("This is a standard toast notification");

        self.toast_manager.update(cx, |manager, cx| {
            manager.add_toast(toast, window, cx);
        });
    }

    fn show_custom_padding_toast(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let id = self.get_next_id();
        let toast = ToastItem::new(id, "Custom Padding")
            .description("Toast with extra padding applied via Styled trait")
            .p(px(24.0));

        self.toast_manager.update(cx, |manager, cx| {
            manager.add_toast(toast, window, cx);
        });
    }

    fn show_custom_background_toast(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let id = self.get_next_id();
        let toast = ToastItem::new(id, "Custom Background")
            .description("Purple background with custom styling")
            .variant(ToastVariant::Default)
            .bg(rgb(0x8b5cf6))
            .border_color(rgb(0xa78bfa));

        self.toast_manager.update(cx, |manager, cx| {
            manager.add_toast(toast, window, cx);
        });
    }

    fn show_custom_border_toast(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let id = self.get_next_id();
        let toast = ToastItem::new(id, "Custom Border")
            .description("Toast with thick colored border")
            .variant(ToastVariant::Success)
            .border_4()
            .border_color(rgb(0x10b981));

        self.toast_manager.update(cx, |manager, cx| {
            manager.add_toast(toast, window, cx);
        });
    }

    fn show_custom_radius_toast(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let id = self.get_next_id();
        let toast = ToastItem::new(id, "Sharp Corners")
            .description("Toast with no border radius")
            .variant(ToastVariant::Warning)
            .rounded(px(0.0));

        self.toast_manager.update(cx, |manager, cx| {
            manager.add_toast(toast, window, cx);
        });
    }

    fn show_large_toast(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let id = self.get_next_id();
        let toast = ToastItem::new(id, "Large Toast")
            .description("Toast with increased minimum width")
            .variant(ToastVariant::Default)
            .min_w(px(400.0))
            .p(px(20.0));

        self.toast_manager.update(cx, |manager, cx| {
            manager.add_toast(toast, window, cx);
        });
    }

    fn show_gradient_toast(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let id = self.get_next_id();
        let toast = ToastItem::new(id, "Gradient Style")
            .description("Toast with custom gradient-like appearance")
            .bg(gpui::hsla(260.0 / 360.0, 0.6, 0.5, 1.0))
            .border_2()
            .border_color(gpui::hsla(260.0 / 360.0, 0.7, 0.6, 1.0))
            .rounded(px(12.0))
            .shadow_xl();

        self.toast_manager.update(cx, |manager, cx| {
            manager.add_toast(toast, window, cx);
        });
    }

    fn show_premium_toast(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let id = self.get_next_id();
        let toast = ToastItem::new(id, "Premium Notification")
            .description("Fully customized premium toast with gold styling")
            .bg(rgb(0xfbbf24))
            .border_3()
            .border_color(rgb(0xf59e0b))
            .rounded(px(16.0))
            .p(px(20.0))
            .shadow_2xl()
            .min_w(px(380.0));

        self.toast_manager.update(cx, |manager, cx| {
            manager.add_toast(toast, window, cx);
        });
    }

    fn clear_all_toasts(&mut self, cx: &mut Context<Self>) {
        self.toast_manager.update(cx, |manager, cx| {
            manager.clear_all(cx);
        });
    }
}

impl Render for ToastStyledDemo {
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
                                        .child("Toast Styled Trait Customization Demo")
                                )
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child("Demonstrating full GPUI styling control via Styled trait for Toast component")
                                )
                        )
                        // 1. Default Toast
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("1. Default Toast")
                                )
                                .child(
                                    Button::new("btn-default", "Show Default Toast")
                                        .on_click(cx.listener(|view: &mut ToastStyledDemo, _, window, cx| {
                                            view.show_default_toast(window, cx);
                                        }))
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
                                        .child("2. Custom Padding (via Styled trait)")
                                )
                                .child(
                                    Button::new("btn-padding", "Show Toast with Extra Padding")
                                        .on_click(cx.listener(|view: &mut ToastStyledDemo, _, window, cx| {
                                            view.show_custom_padding_toast(window, cx);
                                        }))
                                )
                        )
                        // 3. Custom Background
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("3. Custom Background Color")
                                )
                                .child(
                                    Button::new("btn-bg", "Show Purple Toast")
                                        .on_click(cx.listener(|view: &mut ToastStyledDemo, _, window, cx| {
                                            view.show_custom_background_toast(window, cx);
                                        }))
                                )
                        )
                        // 4. Custom Border
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("4. Custom Borders")
                                )
                                .child(
                                    Button::new("btn-border", "Show Thick Border Toast")
                                        .on_click(cx.listener(|view: &mut ToastStyledDemo, _, window, cx| {
                                            view.show_custom_border_toast(window, cx);
                                        }))
                                )
                        )
                        // 5. Custom Border Radius
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("5. Custom Border Radius")
                                )
                                .child(
                                    Button::new("btn-radius", "Show Sharp Corners Toast")
                                        .on_click(cx.listener(|view: &mut ToastStyledDemo, _, window, cx| {
                                            view.show_custom_radius_toast(window, cx);
                                        }))
                                )
                        )
                        // 6. Custom Size
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("6. Custom Size (Width & Padding)")
                                )
                                .child(
                                    Button::new("btn-size", "Show Large Toast")
                                        .on_click(cx.listener(|view: &mut ToastStyledDemo, _, window, cx| {
                                            view.show_large_toast(window, cx);
                                        }))
                                )
                        )
                        // 7. Gradient Effect
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("7. Gradient-like Styling")
                                )
                                .child(
                                    Button::new("btn-gradient", "Show Gradient Toast")
                                        .on_click(cx.listener(|view: &mut ToastStyledDemo, _, window, cx| {
                                            view.show_gradient_toast(window, cx);
                                        }))
                                )
                        )
                        // 8. Premium Combined Styling
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("8. Premium Combined Styling")
                                )
                                .child(
                                    Button::new("btn-premium", "Show Premium Toast")
                                        .variant(ButtonVariant::Destructive)
                                        .on_click(cx.listener(|view: &mut ToastStyledDemo, _, window, cx| {
                                            view.show_premium_toast(window, cx);
                                        }))
                                )
                        )
                        // Clear All
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("Clear All Toasts")
                                )
                                .child(
                                    Button::new("btn-clear", "Clear All")
                                        .variant(ButtonVariant::Outline)
                                        .on_click(cx.listener(|view: &mut ToastStyledDemo, _, _, cx| {
                                            view.clear_all_toasts(cx);
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
                                        .child("All customizations above use the Styled trait for full GPUI styling control!")
                                )
                                .child(
                                    div()
                                        .mt(px(8.0))
                                        .text_size(px(12.0))
                                        .text_color(theme.tokens.accent_foreground)
                                        .child("Methods used: .p(), .px(), .bg(), .border_1/2/3/4(), .border_color(), .rounded(), .min_w(), .shadow_lg/xl/2xl()")
                                )
                        )
                )
            )
            .child(self.toast_manager.clone())
    }
}
