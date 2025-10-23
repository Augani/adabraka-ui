use adabraka_ui::{
    prelude::*,
    overlays::hover_card::{HoverCard, HoverCardPosition, HoverCardAlignment},
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
                        title: Some("HoverCard Styled Trait Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(900.0), px(800.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|_| HoverCardStyledDemo::new()),
            )
            .unwrap();
        });
}

struct HoverCardStyledDemo {}

impl HoverCardStyledDemo {
    fn new() -> Self {
        Self {}
    }
}

impl Render for HoverCardStyledDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        div()
            .size_full()
            .bg(theme.tokens.background)
            .overflow_scroll()
            .child(
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
                                    .child("HoverCard Styled Trait Customization Demo")
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Demonstrating full GPUI styling control via Styled trait on HoverCards")
                            )
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .text_color(theme.tokens.accent)
                                    .child("Note: This demo shows HoverCards with is_open set to true for visibility")
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
                                    .gap(px(80.0))
                                    .items_start()
                                    .child(
                                        HoverCard::new()
                                            .is_open(true)
                                            .trigger(
                                                div()
                                                    .p(px(12.0))
                                                    .bg(theme.tokens.secondary)
                                                    .rounded(px(8.0))
                                                    .child("Default Padding")
                                            )
                                            .content(
                                                div()
                                                    .p(px(12.0))
                                                    .child("Default padding hover card content")
                                            )
                                    )
                                    .child(
                                        HoverCard::new()
                                            .is_open(true)
                                            .p_8()  // <- Styled trait method
                                            .trigger(
                                                div()
                                                    .p(px(12.0))
                                                    .bg(theme.tokens.secondary)
                                                    .rounded(px(8.0))
                                                    .child("p_8() Padding")
                                            )
                                            .content(
                                                div()
                                                    .p(px(12.0))
                                                    .child("Extra padding via p_8() styled trait")
                                            )
                                    )
                                    .child(
                                        HoverCard::new()
                                            .is_open(true)
                                            .px(px(32.0))  // <- Styled trait method
                                            .py(px(20.0))  // <- Styled trait method
                                            .trigger(
                                                div()
                                                    .p(px(12.0))
                                                    .bg(theme.tokens.secondary)
                                                    .rounded(px(8.0))
                                                    .child("Custom px/py")
                                            )
                                            .content(
                                                div()
                                                    .p(px(12.0))
                                                    .child("Large custom padding: px(32) py(20)")
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
                                    .text_size(px(18.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("2. Custom Background Colors")
                            )
                            .child(
                                HStack::new()
                                    .gap(px(80.0))
                                    .items_start()
                                    .child(
                                        HoverCard::new()
                                            .is_open(true)
                                            .bg(rgb(0x3b82f6))  // <- Styled trait
                                            .trigger(
                                                div()
                                                    .p(px(12.0))
                                                    .bg(theme.tokens.secondary)
                                                    .rounded(px(8.0))
                                                    .child("Blue Theme")
                                            )
                                            .content(
                                                div()
                                                    .p(px(12.0))
                                                    .text_color(gpui::white())
                                                    .child("Blue background hover card")
                                            )
                                    )
                                    .child(
                                        HoverCard::new()
                                            .is_open(true)
                                            .bg(rgb(0x8b5cf6))  // <- Styled trait
                                            .trigger(
                                                div()
                                                    .p(px(12.0))
                                                    .bg(theme.tokens.secondary)
                                                    .rounded(px(8.0))
                                                    .child("Purple Theme")
                                            )
                                            .content(
                                                div()
                                                    .p(px(12.0))
                                                    .text_color(gpui::white())
                                                    .child("Purple background hover card")
                                            )
                                    )
                                    .child(
                                        HoverCard::new()
                                            .is_open(true)
                                            .bg(rgb(0x10b981))  // <- Styled trait
                                            .trigger(
                                                div()
                                                    .p(px(12.0))
                                                    .bg(theme.tokens.secondary)
                                                    .rounded(px(8.0))
                                                    .child("Green Theme")
                                            )
                                            .content(
                                                div()
                                                    .p(px(12.0))
                                                    .text_color(gpui::white())
                                                    .child("Green background hover card")
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
                                    .text_size(px(18.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("3. Custom Borders")
                            )
                            .child(
                                HStack::new()
                                    .gap(px(80.0))
                                    .items_start()
                                    .child(
                                        HoverCard::new()
                                            .is_open(true)
                                            .border_2()  // <- Styled trait
                                            .border_color(rgb(0x3b82f6))
                                            .trigger(
                                                div()
                                                    .p(px(12.0))
                                                    .bg(theme.tokens.secondary)
                                                    .rounded(px(8.0))
                                                    .child("2px Blue Border")
                                            )
                                            .content(
                                                div()
                                                    .p(px(12.0))
                                                    .child("Thick blue border hover card")
                                            )
                                    )
                                    .child(
                                        HoverCard::new()
                                            .is_open(true)
                                            .border_2()  // <- Styled trait
                                            .border_color(rgb(0xef4444))
                                            .trigger(
                                                div()
                                                    .p(px(12.0))
                                                    .bg(theme.tokens.secondary)
                                                    .rounded(px(8.0))
                                                    .child("2px Red Border")
                                            )
                                            .content(
                                                div()
                                                    .p(px(12.0))
                                                    .child("Thick red border hover card")
                                            )
                                    )
                                    .child(
                                        HoverCard::new()
                                            .is_open(true)
                                            .border(px(4.0))  // <- Styled trait
                                            .border_color(rgb(0x8b5cf6))
                                            .trigger(
                                                div()
                                                    .p(px(12.0))
                                                    .bg(theme.tokens.secondary)
                                                    .rounded(px(8.0))
                                                    .child("4px Purple Border")
                                            )
                                            .content(
                                                div()
                                                    .p(px(12.0))
                                                    .child("Ultra thick purple border")
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
                                    .text_size(px(18.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("4. Custom Border Radius")
                            )
                            .child(
                                HStack::new()
                                    .gap(px(80.0))
                                    .items_start()
                                    .child(
                                        HoverCard::new()
                                            .is_open(true)
                                            .rounded(px(0.0))  // <- Styled trait
                                            .trigger(
                                                div()
                                                    .p(px(12.0))
                                                    .bg(theme.tokens.secondary)
                                                    .rounded(px(8.0))
                                                    .child("No Radius")
                                            )
                                            .content(
                                                div()
                                                    .p(px(12.0))
                                                    .child("Sharp corners - no border radius")
                                            )
                                    )
                                    .child(
                                        HoverCard::new()
                                            .is_open(true)
                                            .rounded(px(20.0))  // <- Styled trait
                                            .trigger(
                                                div()
                                                    .p(px(12.0))
                                                    .bg(theme.tokens.secondary)
                                                    .rounded(px(8.0))
                                                    .child("Large Radius")
                                            )
                                            .content(
                                                div()
                                                    .p(px(12.0))
                                                    .child("Large border radius: 20px")
                                            )
                                    )
                                    .child(
                                        HoverCard::new()
                                            .is_open(true)
                                            .rounded(px(999.0))  // <- Styled trait
                                            .trigger(
                                                div()
                                                    .p(px(12.0))
                                                    .bg(theme.tokens.secondary)
                                                    .rounded(px(8.0))
                                                    .child("Pill Shape")
                                            )
                                            .content(
                                                div()
                                                    .p(px(12.0))
                                                    .child("Pill-shaped hover card")
                                            )
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
                                    .gap(px(40.0))
                                    .items_start()
                                    .child(
                                        HoverCard::new()
                                            .is_open(true)
                                            .w(px(300.0))  // <- Styled trait
                                            .trigger(
                                                div()
                                                    .p(px(12.0))
                                                    .bg(theme.tokens.secondary)
                                                    .rounded(px(8.0))
                                                    .child("w(300px)")
                                            )
                                            .content(
                                                div()
                                                    .p(px(12.0))
                                                    .child("Custom width 300px hover card with some longer text to demonstrate")
                                            )
                                    )
                                    .child(
                                        HoverCard::new()
                                            .is_open(true)
                                            .w(px(500.0))  // <- Styled trait
                                            .trigger(
                                                div()
                                                    .p(px(12.0))
                                                    .bg(theme.tokens.secondary)
                                                    .rounded(px(8.0))
                                                    .child("w(500px)")
                                            )
                                            .content(
                                                div()
                                                    .p(px(12.0))
                                                    .child("Extra wide 500px hover card - perfect for displaying more detailed information")
                                            )
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
                                    .gap(px(80.0))
                                    .items_start()
                                    .child(
                                        HoverCard::new()
                                            .is_open(true)
                                            .shadow_sm()  // <- Styled trait
                                            .trigger(
                                                div()
                                                    .p(px(12.0))
                                                    .bg(theme.tokens.secondary)
                                                    .rounded(px(8.0))
                                                    .child("Small Shadow")
                                            )
                                            .content(
                                                div()
                                                    .p(px(12.0))
                                                    .child("Subtle shadow effect")
                                            )
                                    )
                                    .child(
                                        HoverCard::new()
                                            .is_open(true)
                                            .shadow_md()  // <- Styled trait
                                            .trigger(
                                                div()
                                                    .p(px(12.0))
                                                    .bg(theme.tokens.secondary)
                                                    .rounded(px(8.0))
                                                    .child("Medium Shadow")
                                            )
                                            .content(
                                                div()
                                                    .p(px(12.0))
                                                    .child("Medium shadow effect")
                                            )
                                    )
                                    .child(
                                        HoverCard::new()
                                            .is_open(true)
                                            .shadow_lg()  // <- Styled trait
                                            .trigger(
                                                div()
                                                    .p(px(12.0))
                                                    .bg(theme.tokens.secondary)
                                                    .rounded(px(8.0))
                                                    .child("Large Shadow")
                                            )
                                            .content(
                                                div()
                                                    .p(px(12.0))
                                                    .child("Prominent large shadow")
                                            )
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
                                    .gap(px(40.0))
                                    .items_start()
                                    .child(
                                        HoverCard::new()
                                            .is_open(true)
                                            .p_8()  // <- Styled trait
                                            .rounded(px(999.0))  // <- Styled trait
                                            .bg(rgb(0x8b5cf6))  // <- Styled trait
                                            .shadow_lg()  // <- Styled trait
                                            .trigger(
                                                div()
                                                    .p(px(12.0))
                                                    .bg(theme.tokens.secondary)
                                                    .rounded(px(8.0))
                                                    .child("Purple Pill + Shadow")
                                            )
                                            .content(
                                                div()
                                                    .p(px(12.0))
                                                    .text_color(gpui::white())
                                                    .child("Purple pill-shaped hover card with large shadow")
                                            )
                                    )
                                    .child(
                                        HoverCard::new()
                                            .is_open(true)
                                            .p(px(24.0))  // <- Styled trait
                                            .border_2()  // <- Styled trait
                                            .border_color(rgb(0x10b981))
                                            .rounded(px(16.0))  // <- Styled trait
                                            .bg(rgb(0x059669))  // <- Styled trait
                                            .shadow_md()  // <- Styled trait
                                            .w(px(400.0))  // <- Styled trait
                                            .trigger(
                                                div()
                                                    .p(px(12.0))
                                                    .bg(theme.tokens.secondary)
                                                    .rounded(px(8.0))
                                                    .child("Green Ultra Custom")
                                            )
                                            .content(
                                                div()
                                                    .p(px(12.0))
                                                    .text_color(gpui::white())
                                                    .child("Custom bordered green hover card with all the styling bells and whistles")
                                            )
                                    )
                            )
                    )
                    // 8. Different Positions with Custom Styling
                    .child(
                        VStack::new()
                            .gap(px(16.0))
                            .child(
                                div()
                                    .text_size(px(18.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("8. Different Positions with Custom Styling")
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Each position has custom styling applied via Styled trait")
                            )
                            .child(
                                HStack::new()
                                    .gap(px(100.0))
                                    .items_start()
                                    .child(
                                        HoverCard::new()
                                            .is_open(true)
                                            .position(HoverCardPosition::Top)
                                            .bg(rgb(0x3b82f6))
                                            .p_4()
                                            .rounded(px(12.0))
                                            .shadow_lg()
                                            .trigger(
                                                div()
                                                    .p(px(12.0))
                                                    .bg(theme.tokens.secondary)
                                                    .rounded(px(8.0))
                                                    .child("Top Position")
                                            )
                                            .content(
                                                div()
                                                    .p(px(12.0))
                                                    .text_color(gpui::white())
                                                    .child("Top positioned - Blue styled")
                                            )
                                    )
                                    .child(
                                        HoverCard::new()
                                            .is_open(true)
                                            .position(HoverCardPosition::Bottom)
                                            .bg(rgb(0x10b981))
                                            .p_4()
                                            .rounded(px(12.0))
                                            .shadow_lg()
                                            .trigger(
                                                div()
                                                    .p(px(12.0))
                                                    .bg(theme.tokens.secondary)
                                                    .rounded(px(8.0))
                                                    .child("Bottom Position")
                                            )
                                            .content(
                                                div()
                                                    .p(px(12.0))
                                                    .text_color(gpui::white())
                                                    .child("Bottom positioned - Green styled")
                                            )
                                    )
                            )
                            .child(
                                HStack::new()
                                    .gap(px(100.0))
                                    .items_start()
                                    .child(
                                        HoverCard::new()
                                            .is_open(true)
                                            .position(HoverCardPosition::Left)
                                            .bg(rgb(0x8b5cf6))
                                            .p_4()
                                            .rounded(px(12.0))
                                            .shadow_lg()
                                            .trigger(
                                                div()
                                                    .p(px(12.0))
                                                    .bg(theme.tokens.secondary)
                                                    .rounded(px(8.0))
                                                    .child("Left Position")
                                            )
                                            .content(
                                                div()
                                                    .p(px(12.0))
                                                    .text_color(gpui::white())
                                                    .child("Left positioned - Purple styled")
                                            )
                                    )
                                    .child(
                                        HoverCard::new()
                                            .is_open(true)
                                            .position(HoverCardPosition::Right)
                                            .bg(rgb(0xf59e0b))
                                            .p_4()
                                            .rounded(px(12.0))
                                            .shadow_lg()
                                            .trigger(
                                                div()
                                                    .p(px(12.0))
                                                    .bg(theme.tokens.secondary)
                                                    .rounded(px(8.0))
                                                    .child("Right Position")
                                            )
                                            .content(
                                                div()
                                                    .p(px(12.0))
                                                    .text_color(gpui::white())
                                                    .child("Right positioned - Orange styled")
                                            )
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
                                    .child("All hover card customizations above use the Styled trait for full GPUI styling control!")
                            )
                            .child(
                                div()
                                    .mt(px(8.0))
                                    .text_size(px(12.0))
                                    .text_color(theme.tokens.accent_foreground)
                                    .child("Methods used: .p_4(), .p_8(), .px(), .py(), .bg(), .border_2(), .border(), .border_color(), .rounded(), .w(), .shadow_sm/md/lg()")
                            )
                    )
            )
    }
}
