use adabraka_ui::{
    prelude::*,
    navigation::breadcrumbs::{Breadcrumbs, BreadcrumbItem},
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
                        title: Some("Breadcrumbs Styled Trait Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(900.0), px(800.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|_| BreadcrumbsStyledDemo::new()),
            )
            .unwrap();
        });
}

struct BreadcrumbsStyledDemo;

impl BreadcrumbsStyledDemo {
    fn new() -> Self {
        Self
    }

    fn create_sample_items() -> Vec<BreadcrumbItem<String>> {
        vec![
            BreadcrumbItem {
                id: "home".to_string(),
                label: "Home".into(),
                icon: None,
            },
            BreadcrumbItem {
                id: "projects".to_string(),
                label: "Projects".into(),
                icon: None,
            },
            BreadcrumbItem {
                id: "ui-library".to_string(),
                label: "UI Library".into(),
                icon: None,
            },
            BreadcrumbItem {
                id: "components".to_string(),
                label: "Components".into(),
                icon: None,
            },
        ]
    }
}

impl Render for BreadcrumbsStyledDemo {
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
                            .child("Breadcrumbs Styled Trait Customization Demo")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("Demonstrating full GPUI styling control via Styled trait for Breadcrumbs")
                    )
            )
            // 1. Default Breadcrumbs
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("1. Default Breadcrumbs (No Custom Styling)")
                    )
                    .child(
                        Breadcrumbs::new(cx)
                            .items(Self::create_sample_items())
                            .on_click(|id, _, _| {
                                println!("Clicked: {}", id);
                            })
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
                        Breadcrumbs::new(cx)
                            .items(Self::create_sample_items())
                            .on_click(|id, _, _| {
                                println!("Clicked: {}", id);
                            })
                            .p_4()  // Styled trait method
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("Applied: .p_4()")
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
                        Breadcrumbs::new(cx)
                            .items(Self::create_sample_items())
                            .on_click(|id, _, _| {
                                println!("Clicked: {}", id);
                            })
                            .bg(theme.tokens.accent.opacity(0.2))  // Styled trait
                            .p_4()
                            .rounded(px(8.0))
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("Applied: .bg(accent.opacity(0.2)), .p_4(), .rounded(px(8.0))")
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
                            .child("4. Custom Border")
                    )
                    .child(
                        Breadcrumbs::new(cx)
                            .items(Self::create_sample_items())
                            .on_click(|id, _, _| {
                                println!("Clicked: {}", id);
                            })
                            .border_2()  // Styled trait
                            .border_color(rgb(0x3b82f6))
                            .p_4()
                            .rounded(px(8.0))
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("Applied: .border_2(), .border_color(blue), .p_4(), .rounded(px(8.0))")
                    )
            )
            // 5. Shadow Effect
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("5. Shadow Effect")
                    )
                    .child(
                        Breadcrumbs::new(cx)
                            .items(Self::create_sample_items())
                            .on_click(|id, _, _| {
                                println!("Clicked: {}", id);
                            })
                            .bg(theme.tokens.background)
                            .p_4()
                            .rounded(px(8.0))
                            .shadow_lg()  // Styled trait
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("Applied: .bg(background), .p_4(), .rounded(px(8.0)), .shadow_lg()")
                    )
            )
            // 6. Full Width
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("6. Full Width with Background")
                    )
                    .child(
                        Breadcrumbs::new(cx)
                            .items(Self::create_sample_items())
                            .on_click(|id, _, _| {
                                println!("Clicked: {}", id);
                            })
                            .w_full()  // Styled trait
                            .bg(theme.tokens.secondary)
                            .p_4()
                            .rounded(px(8.0))
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("Applied: .w_full(), .bg(secondary), .p_4(), .rounded(px(8.0))")
                    )
            )
            // 7. Card Style with Combined Effects
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("7. Card Style (Combined Effects)")
                    )
                    .child(
                        Breadcrumbs::new(cx)
                            .items(Self::create_sample_items())
                            .on_click(|id, _, _| {
                                println!("Clicked: {}", id);
                            })
                            .bg(rgb(0x8b5cf6))  // Styled trait
                            .border_2()  // Styled trait
                            .border_color(rgb(0x8b5cf6))
                            .p_8()  // Styled trait
                            .rounded(px(12.0))  // Styled trait
                            .shadow_md()  // Styled trait
                            .w_full()  // Styled trait
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("Applied: .bg(purple), .border_2(), .p_8(), .rounded(px(12.0)), .shadow_md(), .w_full()")
                    )
            )
            // 8. Custom Sizing and Positioning
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("8. Custom Sizing with Margin")
                    )
                    .child(
                        Breadcrumbs::new(cx)
                            .items(Self::create_sample_items())
                            .on_click(|id, _, _| {
                                println!("Clicked: {}", id);
                            })
                            .w(px(600.0))  // Styled trait
                            .bg(theme.tokens.accent.opacity(0.15))
                            .px(px(24.0))  // Styled trait
                            .py(px(12.0))  // Styled trait
                            .rounded(px(8.0))  // Styled trait
                            .m_4()  // Styled trait
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("Applied: .w(px(600.0)), .px(px(24.0)), .py(px(12.0)), .rounded(px(8.0)), .m_4()")
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
                            .child("Methods used: .p_4(), .p_8(), .px(), .py(), .bg(), .border_2(), .border_color(), .rounded(), .w_full(), .w(), .shadow_md/lg(), .m_4()")
                    )
            )
                )
            )
    }
}
