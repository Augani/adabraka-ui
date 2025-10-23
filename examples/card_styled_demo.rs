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
                        title: Some("Card Styled Trait Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(1200.0), px(900.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|_| CardStyledDemoView),
            )
            .unwrap();
        });
}

struct CardStyledDemoView;

impl Render for CardStyledDemoView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .gap_6()
            .p_8()
            .bg(rgb(0xf5f5f5))
            .child(scrollable_vertical(
                div()
                    .flex()
                    .flex_col()
                    .gap_6()
                    .child(
                        div()
                            .text_xl()
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x1a1a1a))
                            .mb_4()
                            .child("Card Styled Trait Demo")
                    )
                    .child(
                        div()
                            .flex()
                            .flex_wrap()
                            .gap_6()
                            .child(
                                // 1. Default Card
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(rgb(0x666666))
                                            .child("Default Card")
                                    )
                                    .child(
                                        Card::new()
                                            .header(div().text_base().font_weight(FontWeight::SEMIBOLD).child("Default Styling"))
                                            .content(div().child("This card uses the default theme styling with no custom modifications."))
                                            .footer(div().text_sm().text_color(rgb(0x888888)).child("Footer content"))
                                            .w(px(300.0))
                                    )
                            )
                            .child(
                                // 2. Custom Background Color
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(rgb(0x666666))
                                            .child("Custom Background")
                                    )
                                    .child(
                                        Card::new()
                                            .header(div().text_base().font_weight(FontWeight::SEMIBOLD).child("Blue Background"))
                                            .content(div().child("This card demonstrates custom background color using the Styled trait."))
                                            .bg(rgb(0xe3f2fd))
                                            .w(px(300.0))
                                    )
                            )
                            .child(
                                // 3. Custom Border & Rounded Corners
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(rgb(0x666666))
                                            .child("Custom Border")
                                    )
                                    .child(
                                        Card::new()
                                            .header(div().text_base().font_weight(FontWeight::SEMIBOLD).child("Thick Border"))
                                            .content(div().child("This card has a custom border width, color, and rounded corners."))
                                            .border_3()
                                            .border_color(rgb(0x9c27b0))
                                            .rounded(px(16.0))
                                            .w(px(300.0))
                                    )
                            )
                            .child(
                                // 4. Custom Padding & Size
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(rgb(0x666666))
                                            .child("Custom Size & Padding")
                                    )
                                    .child(
                                        Card::new()
                                            .header(div().text_base().font_weight(FontWeight::SEMIBOLD).child("Wide Card"))
                                            .content(div().child("This card demonstrates custom width, padding, and margin."))
                                            .w(px(400.0))
                                            .p_6()
                                            .m_2()
                                    )
                            )
                            .child(
                                // 5. No Shadow with Gradient
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(rgb(0x666666))
                                            .child("Gradient Background")
                                    )
                                    .child(
                                        Card::new()
                                            .header(div().text_base().font_weight(FontWeight::SEMIBOLD).text_color(white()).child("Gradient Card"))
                                            .content(div().text_color(white()).child("This card uses a gradient background color."))
                                            .bg(rgb(0x667eea))
                                            .border_color(rgb(0x764ba2))
                                            .w(px(300.0))
                                    )
                            )
                            .child(
                                // 6. Compact Card
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(rgb(0x666666))
                                            .child("Compact Card")
                                    )
                                    .child(
                                        Card::new()
                                            .content(div().text_sm().child("A compact card with minimal content and tight spacing."))
                                            .w(px(250.0))
                                            .p_2()
                                            .rounded(px(6.0))
                                    )
                            )
                            .child(
                                // 7. Max Width with Centered Content
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(rgb(0x666666))
                                            .child("Max Width")
                                    )
                                    .child(
                                        Card::new()
                                            .header(div().text_base().font_weight(FontWeight::SEMIBOLD).child("Full Width Card"))
                                            .content(div().child("This card demonstrates max-width constraint with auto margins for centering."))
                                            .footer(div().text_sm().text_color(rgb(0x888888)).child("Responsive design"))
                                            .w(px(500.0))
                                            .max_w(px(600.0))
                                    )
                            )
                            .child(
                                // 8. Elevated Card (Custom Shadow)
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(rgb(0x666666))
                                            .child("Elevated Card")
                                    )
                                    .child(
                                        Card::new()
                                            .header(div().text_base().font_weight(FontWeight::SEMIBOLD).child("Elevated"))
                                            .content(div().child("This card appears elevated with a larger shadow effect."))
                                            .shadow(vec![
                                                BoxShadow {
                                                    offset: point(px(0.0), px(4.0)),
                                                    blur_radius: px(12.0),
                                                    spread_radius: px(0.0),
                                                    color: hsla(0.0, 0.0, 0.0, 0.25),
                                                }
                                            ])
                                            .w(px(300.0))
                                    )
                            )
                    )
            ))
    }
}
