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
                        title: Some("MasonryGrid Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(1200.0), px(900.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|_| MasonryGridDemoView),
            )
            .unwrap();
        });
}

struct MasonryGridDemoView;

fn card(title: &str, height: f32, color: u32) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_2()
        .p_4()
        .bg(rgb(color))
        .rounded(px(8.0))
        .border_1()
        .border_color(rgb(0x333333))
        .h(px(height))
        .child(
            div()
                .text_base()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgb(0xffffff))
                .child(title.to_string()),
        )
        .child(
            div()
                .text_sm()
                .text_color(rgb(0xcccccc))
                .child(format!("Height: {}px", height)),
        )
}

impl Render for MasonryGridDemoView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let cards = vec![
            ("Photo Gallery", 180.0, 0x2d3748),
            ("Recent Posts", 280.0, 0x4a5568),
            ("Weather", 120.0, 0x38a169),
            ("Calendar", 220.0, 0x3182ce),
            ("Tasks", 160.0, 0x805ad5),
            ("Music Player", 240.0, 0xd53f8c),
            ("Notes", 140.0, 0xdd6b20),
            ("Stats", 200.0, 0x319795),
            ("Contacts", 260.0, 0x718096),
            ("Messages", 180.0, 0xe53e3e),
            ("Bookmarks", 150.0, 0x667eea),
            ("Settings", 190.0, 0x48bb78),
        ];

        ScrollContainer::vertical()
            .size_full()
            .child(
                VStack::new()
                    .fill_width()
                    .padding(px(24.0))
                    .gap(px(24.0))
                    .child(
                        VStack::new()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .text_2xl()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0xffffff))
                                    .child("MasonryGrid Layout"),
                            )
                            .child(
                                div()
                                    .text_base()
                                    .text_color(rgb(0xa0aec0))
                                    .child("Pinterest-style masonry layout with items flowing into the shortest column"),
                            ),
                    )
                    .child(
                        VStack::new()
                            .gap(px(16.0))
                            .fill_width()
                            .child(
                                div()
                                    .text_lg()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(0xe2e8f0))
                                    .child("3 Columns (Default)"),
                            )
                            .child(
                                MasonryGrid::new()
                                    .columns(3)
                                    .gap(px(16.0))
                                    .fill_width()
                                    .items(cards.iter().map(|(title, height, color)| {
                                        (card(title, *height, *color), *height)
                                    })),
                            ),
                    )
                    .child(
                        VStack::new()
                            .gap(px(16.0))
                            .fill_width()
                            .child(
                                div()
                                    .text_lg()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(0xe2e8f0))
                                    .child("4 Columns"),
                            )
                            .child(
                                MasonryGrid::new()
                                    .columns(4)
                                    .gap(px(12.0))
                                    .fill_width()
                                    .items(cards.iter().map(|(title, height, color)| {
                                        (card(title, *height, *color), *height)
                                    })),
                            ),
                    )
                    .child(
                        VStack::new()
                            .gap(px(16.0))
                            .fill_width()
                            .child(
                                div()
                                    .text_lg()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(0xe2e8f0))
                                    .child("2 Columns with Larger Gap"),
                            )
                            .child(
                                MasonryGrid::new()
                                    .columns(2)
                                    .gap(px(24.0))
                                    .fill_width()
                                    .items(cards.iter().take(6).map(|(title, height, color)| {
                                        (card(title, *height, *color), *height)
                                    })),
                            ),
                    ),
            )
            .bg(rgb(0x1a202c))
    }
}
