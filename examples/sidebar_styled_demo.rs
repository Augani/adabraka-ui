use adabraka_ui::{
    prelude::*,
    navigation::sidebar::{Sidebar, SidebarItem, SidebarVariant, SidebarPosition},
    components::{icon_source::IconSource, scrollable::scrollable_vertical},
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
                        title: Some("Sidebar Styled Trait Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(1200.0), px(900.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|_| SidebarStyledDemo::new()),
            )
            .unwrap();
        });
}

struct SidebarStyledDemo;

impl SidebarStyledDemo {
    fn new() -> Self {
        Self
    }

    fn create_sample_items() -> Vec<SidebarItem<String>> {
        vec![
            SidebarItem::new("dashboard".to_string(), "Dashboard")
                .with_icon(IconSource::Named("globe".to_string())),
            SidebarItem::new("analytics".to_string(), "Analytics")
                .with_icon(IconSource::Named("search".to_string()))
                .with_badge("3".to_string()),
            SidebarItem::new("separator1".to_string(), "").separator(true),
            SidebarItem::new("projects".to_string(), "Projects")
                .with_icon(IconSource::Named("globe".to_string())),
            SidebarItem::new("team".to_string(), "Team")
                .with_icon(IconSource::Named("search".to_string())),
        ]
    }
}

impl Render for SidebarStyledDemo {
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
                                        .text_size(px(28.0))
                                        .font_weight(FontWeight::BOLD)
                                        .child("Sidebar Styled Trait Customization Demo")
                                )
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child("Demonstrating full GPUI styling control via Styled trait")
                                )
                        )
                        // 1. Custom Shadow Effects
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(20.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("1. Custom Shadow Effects")
                                )
                                .child(
                                    div()
                                        .flex()
                                        .gap(px(16.0))
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .gap(px(8.0))
                                                .child(
                                                    div()
                                                        .text_size(px(14.0))
                                                        .font_weight(FontWeight::MEDIUM)
                                                        .child("Small Shadow")
                                                )
                                                .child(
                                                    Sidebar::new(cx)
                                                        .items(Self::create_sample_items())
                                                        .selected_id("dashboard".to_string())
                                                        .variant(SidebarVariant::Fixed)
                                                        .expanded_width(px(240.0))
                                                        .shadow_sm()  // Styled trait
                                                )
                                        )
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .gap(px(8.0))
                                                .child(
                                                    div()
                                                        .text_size(px(14.0))
                                                        .font_weight(FontWeight::MEDIUM)
                                                        .child("Large Shadow + Rounded")
                                                )
                                                .child(
                                                    Sidebar::new(cx)
                                                        .items(Self::create_sample_items())
                                                        .selected_id("analytics".to_string())
                                                        .variant(SidebarVariant::Fixed)
                                                        .expanded_width(px(240.0))
                                                        .shadow_lg()  // Styled trait
                                                        .rounded(px(12.0))  // Styled trait
                                                )
                                        )
                                )
                        )
                        // 2. Custom Background Colors
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(20.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("2. Custom Background Colors")
                                )
                                .child(
                                    div()
                                        .flex()
                                        .gap(px(16.0))
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .gap(px(8.0))
                                                .child(
                                                    div()
                                                        .text_size(px(14.0))
                                                        .font_weight(FontWeight::MEDIUM)
                                                        .child("Purple Background")
                                                )
                                                .child(
                                                    Sidebar::new(cx)
                                                        .items(Self::create_sample_items())
                                                        .selected_id("projects".to_string())
                                                        .variant(SidebarVariant::Fixed)
                                                        .expanded_width(px(240.0))
                                                        .bg(rgb(0x6b21a8))  // Styled trait - deep purple
                                                        .rounded(px(8.0))
                                                )
                                        )
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .gap(px(8.0))
                                                .child(
                                                    div()
                                                        .text_size(px(14.0))
                                                        .font_weight(FontWeight::MEDIUM)
                                                        .child("Gradient-like Dark Blue")
                                                )
                                                .child(
                                                    Sidebar::new(cx)
                                                        .items(Self::create_sample_items())
                                                        .selected_id("team".to_string())
                                                        .variant(SidebarVariant::Fixed)
                                                        .expanded_width(px(240.0))
                                                        .bg(rgb(0x1e3a8a))  // Styled trait - dark blue
                                                        .rounded(px(8.0))
                                                )
                                        )
                                )
                        )
                        // 3. Custom Borders
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(20.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("3. Custom Borders")
                                )
                                .child(
                                    div()
                                        .flex()
                                        .gap(px(16.0))
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .gap(px(8.0))
                                                .child(
                                                    div()
                                                        .text_size(px(14.0))
                                                        .font_weight(FontWeight::MEDIUM)
                                                        .child("Blue Border (2px)")
                                                )
                                                .child(
                                                    Sidebar::new(cx)
                                                        .items(Self::create_sample_items())
                                                        .selected_id("dashboard".to_string())
                                                        .variant(SidebarVariant::Fixed)
                                                        .expanded_width(px(240.0))
                                                        .border_2()  // Styled trait
                                                        .border_color(rgb(0x3b82f6))
                                                        .rounded(px(8.0))
                                                )
                                        )
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .gap(px(8.0))
                                                .child(
                                                    div()
                                                        .text_size(px(14.0))
                                                        .font_weight(FontWeight::MEDIUM)
                                                        .child("Green Border (4px)")
                                                )
                                                .child(
                                                    Sidebar::new(cx)
                                                        .items(Self::create_sample_items())
                                                        .selected_id("analytics".to_string())
                                                        .variant(SidebarVariant::Fixed)
                                                        .expanded_width(px(240.0))
                                                        .border_4()  // Styled trait
                                                        .border_color(rgb(0x10b981))
                                                        .rounded(px(8.0))
                                                )
                                        )
                                )
                        )
                        // 4. Custom Border Radius
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(20.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("4. Custom Border Radius")
                                )
                                .child(
                                    div()
                                        .flex()
                                        .gap(px(16.0))
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .gap(px(8.0))
                                                .child(
                                                    div()
                                                        .text_size(px(14.0))
                                                        .font_weight(FontWeight::MEDIUM)
                                                        .child("No Radius (Sharp)")
                                                )
                                                .child(
                                                    Sidebar::new(cx)
                                                        .items(Self::create_sample_items())
                                                        .selected_id("projects".to_string())
                                                        .variant(SidebarVariant::Fixed)
                                                        .expanded_width(px(240.0))
                                                        .rounded(px(0.0))  // Styled trait
                                                        .border_1()
                                                        .border_color(theme.tokens.border)
                                                )
                                        )
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .gap(px(8.0))
                                                .child(
                                                    div()
                                                        .text_size(px(14.0))
                                                        .font_weight(FontWeight::MEDIUM)
                                                        .child("Large Radius (24px)")
                                                )
                                                .child(
                                                    Sidebar::new(cx)
                                                        .items(Self::create_sample_items())
                                                        .selected_id("team".to_string())
                                                        .variant(SidebarVariant::Fixed)
                                                        .expanded_width(px(240.0))
                                                        .rounded(px(24.0))  // Styled trait
                                                        .border_1()
                                                        .border_color(theme.tokens.border)
                                                )
                                        )
                                )
                        )
                        // 5. Custom Padding & Margins
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(20.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("5. Custom Padding & Margins")
                                )
                                .child(
                                    div()
                                        .flex()
                                        .gap(px(16.0))
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .gap(px(8.0))
                                                .child(
                                                    div()
                                                        .text_size(px(14.0))
                                                        .font_weight(FontWeight::MEDIUM)
                                                        .child("Extra Padding (p_8)")
                                                )
                                                .child(
                                                    Sidebar::new(cx)
                                                        .items(Self::create_sample_items())
                                                        .selected_id("dashboard".to_string())
                                                        .variant(SidebarVariant::Fixed)
                                                        .expanded_width(px(240.0))
                                                        .p_8()  // Styled trait
                                                        .bg(theme.tokens.accent)
                                                        .rounded(px(8.0))
                                                )
                                        )
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .gap(px(8.0))
                                                .child(
                                                    div()
                                                        .text_size(px(14.0))
                                                        .font_weight(FontWeight::MEDIUM)
                                                        .child("Margin + Padding")
                                                )
                                                .child(
                                                    Sidebar::new(cx)
                                                        .items(Self::create_sample_items())
                                                        .selected_id("analytics".to_string())
                                                        .variant(SidebarVariant::Fixed)
                                                        .expanded_width(px(240.0))
                                                        .m_4()  // Styled trait
                                                        .p_4()  // Styled trait
                                                        .bg(theme.tokens.secondary)
                                                        .rounded(px(12.0))
                                                )
                                        )
                                )
                        )
                        // 6. Combined Styling (Advanced)
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(20.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("6. Combined Styling (Multiple Styled Trait Methods)")
                                )
                                .child(
                                    div()
                                        .flex()
                                        .gap(px(16.0))
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .gap(px(8.0))
                                                .child(
                                                    div()
                                                        .text_size(px(14.0))
                                                        .font_weight(FontWeight::MEDIUM)
                                                        .child("Card Style Sidebar")
                                                )
                                                .child(
                                                    Sidebar::new(cx)
                                                        .items(Self::create_sample_items())
                                                        .selected_id("projects".to_string())
                                                        .variant(SidebarVariant::Fixed)
                                                        .expanded_width(px(240.0))
                                                        .p_6()  // Styled trait
                                                        .bg(rgb(0x7c3aed))  // Styled trait
                                                        .rounded(px(16.0))  // Styled trait
                                                        .shadow_lg()  // Styled trait
                                                        .border_1()  // Styled trait
                                                        .border_color(rgb(0xa855f7))
                                                )
                                        )
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .gap(px(8.0))
                                                .child(
                                                    div()
                                                        .text_size(px(14.0))
                                                        .font_weight(FontWeight::MEDIUM)
                                                        .child("Professional Sidebar")
                                                )
                                                .child(
                                                    Sidebar::new(cx)
                                                        .items(Self::create_sample_items())
                                                        .selected_id("team".to_string())
                                                        .variant(SidebarVariant::Fixed)
                                                        .expanded_width(px(240.0))
                                                        .p_4()  // Styled trait
                                                        .m_2()  // Styled trait
                                                        .bg(rgb(0x0f172a))  // Styled trait
                                                        .rounded(px(12.0))  // Styled trait
                                                        .shadow_md()  // Styled trait
                                                        .border_2()  // Styled trait
                                                        .border_color(rgb(0x334155))
                                                )
                                        )
                                )
                        )
                        // 7. Opacity & Transparency
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(20.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("7. Opacity & Transparency")
                                )
                                .child(
                                    div()
                                        .flex()
                                        .gap(px(16.0))
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .gap(px(8.0))
                                                .child(
                                                    div()
                                                        .text_size(px(14.0))
                                                        .font_weight(FontWeight::MEDIUM)
                                                        .child("Semi-Transparent")
                                                )
                                                .child(
                                                    Sidebar::new(cx)
                                                        .items(Self::create_sample_items())
                                                        .selected_id("dashboard".to_string())
                                                        .variant(SidebarVariant::Fixed)
                                                        .expanded_width(px(240.0))
                                                          // Styled trait
                                                        .bg(theme.tokens.primary)
                                                        .rounded(px(8.0))
                                                )
                                        )
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .gap(px(8.0))
                                                .child(
                                                    div()
                                                        .text_size(px(14.0))
                                                        .font_weight(FontWeight::MEDIUM)
                                                        .child("Glass Effect")
                                                )
                                                .child(
                                                    Sidebar::new(cx)
                                                        .items(Self::create_sample_items())
                                                        .selected_id("analytics".to_string())
                                                        .variant(SidebarVariant::Fixed)
                                                        .expanded_width(px(240.0))
                                                          // Styled trait
                                                        .bg(theme.tokens.card)
                                                        .rounded(px(12.0))
                                                        .border_1()
                                                        .border_color(theme.tokens.border)
                                                )
                                        )
                                )
                        )
                        // 8. Width Customization
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(20.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("8. Width Customization")
                                )
                                .child(
                                    div()
                                        .flex()
                                        .gap(px(16.0))
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .gap(px(8.0))
                                                .child(
                                                    div()
                                                        .text_size(px(14.0))
                                                        .font_weight(FontWeight::MEDIUM)
                                                        .child("Narrow Sidebar (w: 200px)")
                                                )
                                                .child(
                                                    Sidebar::new(cx)
                                                        .items(Self::create_sample_items())
                                                        .selected_id("projects".to_string())
                                                        .variant(SidebarVariant::Fixed)
                                                        .expanded_width(px(200.0))
                                                        .w(px(200.0))  // Styled trait override
                                                        .border_1()
                                                        .border_color(theme.tokens.border)
                                                        .rounded(px(8.0))
                                                )
                                        )
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .gap(px(8.0))
                                                .child(
                                                    div()
                                                        .text_size(px(14.0))
                                                        .font_weight(FontWeight::MEDIUM)
                                                        .child("Wide Sidebar (w: 320px)")
                                                )
                                                .child(
                                                    Sidebar::new(cx)
                                                        .items(Self::create_sample_items())
                                                        .selected_id("team".to_string())
                                                        .variant(SidebarVariant::Fixed)
                                                        .expanded_width(px(320.0))
                                                        .w(px(320.0))  // Styled trait override
                                                        .border_1()
                                                        .border_color(theme.tokens.border)
                                                        .rounded(px(8.0))
                                                )
                                        )
                                )
                        )
                        // Info Box
                        .child(
                            div()
                                .mt(px(16.0))
                                .p(px(20.0))
                                .bg(theme.tokens.accent)
                                .rounded(px(12.0))
                                .shadow_md()
                                .child(
                                    VStack::new()
                                        .gap(px(8.0))
                                        .child(
                                            div()
                                                .text_size(px(16.0))
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .text_color(theme.tokens.accent_foreground)
                                                .child("Styled Trait Implementation Complete!")
                                        )
                                        .child(
                                            div()
                                                .text_size(px(14.0))
                                                .text_color(theme.tokens.accent_foreground)
                                                .child("All customizations above use the Styled trait for full GPUI styling control.")
                                        )
                                        .child(
                                            div()
                                                .mt(px(8.0))
                                                .text_size(px(12.0))
                                                .text_color(theme.tokens.accent_foreground)
                                                .child("Methods used: .shadow_sm/md/lg(), .bg(), .border_1/2/4(), .rounded(), .p_4/6/8(), .m_2/4(), .w()")
                                        )
                                )
                        )
                )
            )
    }
}
