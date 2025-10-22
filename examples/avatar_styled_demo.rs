use adabraka_ui::{
    prelude::*,
    components::{
        scrollable::scrollable_vertical,
        avatar::{Avatar, AvatarSize},
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
                        title: Some("Avatar Styled Trait Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(900.0), px(800.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|_| AvatarStyledDemo::new()),
            )
            .unwrap();
        });
}

struct AvatarStyledDemo;

impl AvatarStyledDemo {
    fn new() -> Self {
        Self
    }
}

impl Render for AvatarStyledDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
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
                            .child("Avatar Styled Trait Customization Demo")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("Demonstrating full GPUI styling control via Styled trait")
                    )
            )
            // 1. Custom Border Examples
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("1. Custom Borders (via Styled trait)")
                    )
                    .child(
                        HStack::new()
                            .gap(px(12.0))
                            .items_center()
                            .child(
                                Avatar::new()
                                    .name("Alice Johnson")
                                    .size(AvatarSize::Lg)
                            )
                            .child(
                                Avatar::new()
                                    .name("Bob Smith")
                                    .size(AvatarSize::Lg)
                                    .border_4()  // Styled trait
                                    .border_color(rgb(0x3b82f6))
                            )
                            .child(
                                Avatar::new()
                                    .name("Carol White")
                                    .size(AvatarSize::Lg)
                                    .border_4()  // Styled trait
                                    .border_color(rgb(0xef4444))
                            )
                            .child(
                                Avatar::new()
                                    .name("David Brown")
                                    .size(AvatarSize::Lg)
                                    .border_4()  // Styled trait
                                    .border_color(rgb(0x10b981))
                            )
                    )
            )
            // 2. Custom Shadow Effects
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("2. Shadow Effects")
                    )
                    .child(
                        HStack::new()
                            .gap(px(12.0))
                            .items_center()
                            .child(
                                Avatar::new()
                                    .name("Eve Davis")
                                    .size(AvatarSize::Lg)
                                    .shadow_sm()  // Styled trait
                            )
                            .child(
                                Avatar::new()
                                    .name("Frank Miller")
                                    .size(AvatarSize::Lg)
                                    .shadow_md()  // Styled trait
                            )
                            .child(
                                Avatar::new()
                                    .name("Grace Lee")
                                    .size(AvatarSize::Lg)
                                    .shadow_lg()  // Styled trait
                            )
                    )
            )
            // 3. Custom Opacity
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("3. Opacity Control")
                    )
                    .child(
                        HStack::new()
                            .gap(px(12.0))
                            .items_center()
                            .child(
                                Avatar::new()
                                    .name("Henry Wilson")
                                    .size(AvatarSize::Lg)
                            )
                            .child(
                                Avatar::new()
                                    .name("Ivy Martinez")
                                    .size(AvatarSize::Lg)
                                    .opacity(0.7)  // Styled trait
                            )
                            .child(
                                Avatar::new()
                                    .name("Jack Anderson")
                                    .size(AvatarSize::Lg)
                                    .opacity(0.5)  // Styled trait
                            )
                            .child(
                                Avatar::new()
                                    .name("Kate Taylor")
                                    .size(AvatarSize::Lg)
                                    .opacity(0.3)  // Styled trait
                            )
                    )
            )
            // 4. Custom Size Override
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("4. Custom Size Override")
                    )
                    .child(
                        HStack::new()
                            .gap(px(12.0))
                            .items_center()
                            .child(
                                Avatar::new()
                                    .name("Leo Garcia")
                                    .size(AvatarSize::Md)
                                    .w(px(100.0))  // Styled trait - override default width
                                    .h(px(100.0))  // Styled trait - override default height
                            )
                            .child(
                                Avatar::new()
                                    .name("Maria Rodriguez")
                                    .size(AvatarSize::Md)
                                    .w(px(80.0))  // Styled trait
                                    .h(px(80.0))  // Styled trait
                            )
                    )
            )
            // 5. Custom Spacing
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("5. Custom Spacing (Margin)")
                    )
                    .child(
                        HStack::new()
                            .items_center()
                            .child(
                                Avatar::new()
                                    .name("Nathan Clark")
                                    .size(AvatarSize::Md)
                                    .mr(px(20.0))  // Styled trait
                            )
                            .child(
                                Avatar::new()
                                    .name("Olivia Hall")
                                    .size(AvatarSize::Md)
                                    .mx(px(32.0))  // Styled trait
                            )
                            .child(
                                Avatar::new()
                                    .name("Paul Allen")
                                    .size(AvatarSize::Md)
                                    .ml(px(20.0))  // Styled trait
                            )
                    )
            )
            // 6. Custom Border Radius Override
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("6. Border Radius Override")
                    )
                    .child(
                        HStack::new()
                            .gap(px(12.0))
                            .items_center()
                            .child(
                                Avatar::new()
                                    .name("Quinn Adams")
                                    .size(AvatarSize::Lg)
                                    .rounded(px(0.0))  // Styled trait - square
                            )
                            .child(
                                Avatar::new()
                                    .name("Rachel Baker")
                                    .size(AvatarSize::Lg)
                                    .rounded(px(8.0))  // Styled trait - rounded corners
                            )
                            .child(
                                Avatar::new()
                                    .name("Sam Collins")
                                    .size(AvatarSize::Lg)
                                    // Default is rounded_full
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
                        HStack::new()
                            .gap(px(12.0))
                            .items_center()
                            .child(
                                Avatar::new()
                                    .name("Tina Evans")
                                    .size(AvatarSize::Lg)
                                    .border_4()  // Styled trait
                                    .border_color(rgb(0x8b5cf6))
                                    .shadow_lg()  // Styled trait
                                    .mx(px(8.0))  // Styled trait
                            )
                            .child(
                                Avatar::new()
                                    .name("Uma Foster")
                                    .size(AvatarSize::Xl)
                                    .border_4()  // Styled trait
                                    .border_color(rgb(0xf59e0b))
                                    .shadow_md()  // Styled trait
                                    .rounded(px(12.0))  // Styled trait
                            )
                            .child(
                                Avatar::new()
                                    .name("Victor Green")
                                    .size(AvatarSize::Md)
                                    .w(px(120.0))  // Styled trait
                                    .h(px(120.0))  // Styled trait
                                    .border_4()  // Styled trait
                                    .border_color(rgb(0x06b6d4))
                                    .shadow_lg()  // Styled trait
                            )
                    )
            )
            // 8. All Sizes with Custom Styling
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("8. All Sizes with Custom Border & Shadow")
                    )
                    .child(
                        HStack::new()
                            .gap(px(12.0))
                            .items_center()
                            .child(
                                Avatar::new()
                                    .name("Wendy Hill")
                                    .size(AvatarSize::Xs)
                                    .border_2()  // Styled trait
                                    .border_color(rgb(0x3b82f6))
                                    .shadow_sm()  // Styled trait
                            )
                            .child(
                                Avatar::new()
                                    .name("Xander King")
                                    .size(AvatarSize::Sm)
                                    .border_2()  // Styled trait
                                    .border_color(rgb(0x8b5cf6))
                                    .shadow_sm()  // Styled trait
                            )
                            .child(
                                Avatar::new()
                                    .name("Yara Lopez")
                                    .size(AvatarSize::Md)
                                    .border_4()  // Styled trait
                                    .border_color(rgb(0x10b981))
                                    .shadow_md()  // Styled trait
                            )
                            .child(
                                Avatar::new()
                                    .name("Zack Moore")
                                    .size(AvatarSize::Lg)
                                    .border_4()  // Styled trait
                                    .border_color(rgb(0xef4444))
                                    .shadow_md()  // Styled trait
                            )
                            .child(
                                Avatar::new()
                                    .name("Amy Nelson")
                                    .size(AvatarSize::Xl)
                                    .border_4()  // Styled trait
                                    .border_color(rgb(0xf59e0b))
                                    .shadow_lg()  // Styled trait
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
                            .child("Methods used: .border_4(), .border_color(), .shadow_sm/md/lg(), .opacity(), .w(), .h(), .mr(), .mx(), .ml(), .rounded()")
                    )
            )
                )
            )
    }
}
