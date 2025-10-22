use adabraka_ui::prelude::*;
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
                        title: Some("Button Features Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(900.0), px(700.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|_| ButtonFeaturesDemo::new()),
            )
            .unwrap();
        });
}

struct ButtonFeaturesDemo {
    click_count: usize,
    is_loading: bool,
    selected_variant: usize,
}

impl ButtonFeaturesDemo {
    fn new() -> Self {
        Self {
            click_count: 0,
            is_loading: false,
            selected_variant: 0,
        }
    }
}

impl Render for ButtonFeaturesDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        VStack::new()
            .size_full()
            .bg(theme.tokens.background)
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
                            .child("Button Features Demo")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child(format!("Click count: {}", self.click_count))
                    )
            )
            // Basic Variants
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("1. Button Variants")
                    )
                    .child(
                        HStack::new()
                            .gap(px(12.0))
                            .child(
                                Button::new("default-btn", "Default")
                                    .variant(ButtonVariant::Default)
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                Button::new("secondary-btn", "Secondary")
                                    .variant(ButtonVariant::Secondary)
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                Button::new("destructive-btn", "Destructive")
                                    .variant(ButtonVariant::Destructive)
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                Button::new("outline-btn", "Outline")
                                    .variant(ButtonVariant::Outline)
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                Button::new("ghost-btn", "Ghost")
                                    .variant(ButtonVariant::Ghost)
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                Button::new("link-btn", "Link")
                                    .variant(ButtonVariant::Link)
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
                            )
                    )
            )
            // Sizes
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("2. Button Sizes")
                    )
                    .child(
                        HStack::new()
                            .gap(px(12.0))
                            .items_center()
                            .child(
                                Button::new("small-btn", "Small")
                                    .size(ButtonSize::Sm)
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                Button::new("medium-btn", "Medium")
                                    .size(ButtonSize::Md)
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                Button::new("large-btn", "Large")
                                    .size(ButtonSize::Lg)
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
                            )
                    )
            )
            // Icons
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("3. Buttons with Icons")
                    )
                    .child(
                        HStack::new()
                            .gap(px(12.0))
                            .child(
                                Button::new("save-btn", "Save")
                                    .icon(IconSource::Named("save".to_string()))
                                    .icon_position(IconPosition::Start)
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                Button::new("next-btn", "Next")
                                    .icon(IconSource::Named("chevron-right".to_string()))
                                    .icon_position(IconPosition::End)
                                    .variant(ButtonVariant::Secondary)
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                Button::new("delete-btn", "Delete")
                                    .icon(IconSource::Named("trash".to_string()))
                                    .variant(ButtonVariant::Destructive)
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
                            )
                    )
            )
            // Loading State
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("4. Loading State")
                    )
                    .child(
                        HStack::new()
                            .gap(px(12.0))
                            .child(
                                Button::new("toggle-loading-btn", if self.is_loading { "Stop Loading" } else { "Start Loading" })
                                    .variant(ButtonVariant::Secondary)
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.is_loading = !view.is_loading;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                Button::new("loading-btn", "Processing...")
                                    .loading(self.is_loading)
                                    .icon(IconSource::Named("loader".to_string()))
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
                            )
                    )
            )
            // Selected State
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("5. Selected State (Toggle Buttons)")
                    )
                    .child(
                        HStack::new()
                            .gap(px(12.0))
                            .child(
                                Button::new("variant-0", "Option 1")
                                    .variant(ButtonVariant::Outline)
                                    .selected(self.selected_variant == 0)
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.selected_variant = 0;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                Button::new("variant-1", "Option 2")
                                    .variant(ButtonVariant::Outline)
                                    .selected(self.selected_variant == 1)
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.selected_variant = 1;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                Button::new("variant-2", "Option 3")
                                    .variant(ButtonVariant::Outline)
                                    .selected(self.selected_variant == 2)
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.selected_variant = 2;
                                        cx.notify();
                                    }))
                            )
                    )
            )
            // Disabled State
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("6. Disabled State")
                    )
                    .child(
                        HStack::new()
                            .gap(px(12.0))
                            .child(
                                Button::new("disabled-default", "Disabled")
                                    .disabled(true)
                            )
                            .child(
                                Button::new("disabled-secondary", "Disabled")
                                    .variant(ButtonVariant::Secondary)
                                    .disabled(true)
                            )
                            .child(
                                Button::new("disabled-destructive", "Disabled")
                                    .variant(ButtonVariant::Destructive)
                                    .disabled(true)
                            )
                    )
            )
    }
}
