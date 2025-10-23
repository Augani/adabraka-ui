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
                        title: Some("Table Styled Trait Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(1200.0), px(900.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|_| TableStyledDemo::new()),
            )
            .unwrap();
        });
}

struct TableStyledDemo;

impl TableStyledDemo {
    fn new() -> Self {
        Self
    }
}

impl Render for TableStyledDemo {
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
                            .child("Table Styled Trait Customization Demo")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("Demonstrating full GPUI styling control on Table component via Styled trait")
                    )
            )
            // 1. Default Table
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("1. Default Table (No Custom Styling)")
                    )
                    .child(
                        Table::new()
                            .columns(vec![
                                TableColumn::new("Name").width(px(150.0)),
                                TableColumn::new("Email").width(px(200.0)),
                                TableColumn::new("Status").width(px(120.0)),
                            ])
                            .rows(vec![
                                TableRow::new(vec![
                                    "Alice Johnson".into(),
                                    "alice@example.com".into(),
                                    "Active".into(),
                                ]),
                                TableRow::new(vec![
                                    "Bob Smith".into(),
                                    "bob@example.com".into(),
                                    "Active".into(),
                                ]),
                                TableRow::new(vec![
                                    "Carol White".into(),
                                    "carol@example.com".into(),
                                    "Inactive".into(),
                                ]),
                            ])
                    )
            )
            // 2. Custom Background Color
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("2. Custom Background Color")
                    )
                    .child(
                        div()
                            .text_size(px(13.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("Using .bg() to add a subtle blue background")
                    )
                    .child(
                        Table::new()
                            .columns(vec![
                                TableColumn::new("Product").width(px(150.0)),
                                TableColumn::new("Price").width(px(100.0)),
                                TableColumn::new("Stock").width(px(100.0)),
                            ])
                            .rows(vec![
                                TableRow::new(vec![
                                    "Laptop".into(),
                                    "$999".into(),
                                    "45".into(),
                                ]),
                                TableRow::new(vec![
                                    "Mouse".into(),
                                    "$25".into(),
                                    "120".into(),
                                ]),
                                TableRow::new(vec![
                                    "Keyboard".into(),
                                    "$75".into(),
                                    "89".into(),
                                ]),
                            ])
                            .bg(rgb(0x1e3a8a))
                    )
            )
            // 3. Custom Padding and Border
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("3. Custom Padding and Border")
                    )
                    .child(
                        div()
                            .text_size(px(13.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("Using .p_8() and .border_2() with custom border color")
                    )
                    .child(
                        Table::new()
                            .columns(vec![
                                TableColumn::new("ID").width(px(80.0)),
                                TableColumn::new("Task").width(px(200.0)),
                                TableColumn::new("Priority").width(px(100.0)),
                            ])
                            .rows(vec![
                                TableRow::new(vec![
                                    "1".into(),
                                    "Review pull requests".into(),
                                    "High".into(),
                                ]),
                                TableRow::new(vec![
                                    "2".into(),
                                    "Update documentation".into(),
                                    "Medium".into(),
                                ]),
                                TableRow::new(vec![
                                    "3".into(),
                                    "Fix bug #234".into(),
                                    "Critical".into(),
                                ]).selected(true),
                            ])
                            .p_8()
                            .border_2()
                            .border_color(rgb(0x10b981))
                    )
            )
            // 4. Large Rounded Corners with Shadow
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("4. Large Rounded Corners with Shadow")
                    )
                    .child(
                        div()
                            .text_size(px(13.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("Using .rounded(px(20.0)) and .shadow_lg()")
                    )
                    .child(
                        Table::new()
                            .columns(vec![
                                TableColumn::new("Date").width(px(120.0)),
                                TableColumn::new("Event").width(px(180.0)),
                                TableColumn::new("Location").width(px(140.0)),
                            ])
                            .rows(vec![
                                TableRow::new(vec![
                                    "2025-10-25".into(),
                                    "Team Meeting".into(),
                                    "Room A".into(),
                                ]),
                                TableRow::new(vec![
                                    "2025-10-26".into(),
                                    "Conference".into(),
                                    "Downtown".into(),
                                ]),
                                TableRow::new(vec![
                                    "2025-10-27".into(),
                                    "Workshop".into(),
                                    "Remote".into(),
                                ]),
                            ])
                            .rounded(px(20.0))
                            .shadow_lg()
                    )
            )
            // 5. Full Width Table
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("5. Full Width Table")
                    )
                    .child(
                        div()
                            .text_size(px(13.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("Using .w_full() to expand to container width")
                    )
                    .child(
                        Table::new()
                            .columns(vec![
                                TableColumn::new("Name").width(px(200.0)),
                                TableColumn::new("Department").width(px(150.0)),
                                TableColumn::new("Role").width(px(150.0)),
                                TableColumn::new("Years").width(px(100.0)),
                            ])
                            .rows(vec![
                                TableRow::new(vec![
                                    "John Doe".into(),
                                    "Engineering".into(),
                                    "Senior Dev".into(),
                                    "5".into(),
                                ]),
                                TableRow::new(vec![
                                    "Jane Smith".into(),
                                    "Design".into(),
                                    "Lead Designer".into(),
                                    "3".into(),
                                ]),
                            ])
                            .w_full()
                    )
            )
            // 6. Purple Theme with Custom Styling
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("6. Purple Theme with Custom Styling")
                    )
                    .child(
                        div()
                            .text_size(px(13.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("Combined: .bg(), .border_2(), .border_color(), .shadow_md()")
                    )
                    .child(
                        Table::new()
                            .columns(vec![
                                TableColumn::new("Code").width(px(100.0)),
                                TableColumn::new("Language").width(px(150.0)),
                                TableColumn::new("Lines").width(px(100.0)),
                            ])
                            .rows(vec![
                                TableRow::new(vec![
                                    "app.rs".into(),
                                    "Rust".into(),
                                    "342".into(),
                                ]),
                                TableRow::new(vec![
                                    "main.ts".into(),
                                    "TypeScript".into(),
                                    "567".into(),
                                ]),
                                TableRow::new(vec![
                                    "index.py".into(),
                                    "Python".into(),
                                    "234".into(),
                                ]).selected(true),
                            ])
                            .bg(rgb(0x8b5cf6))
                            .border_2()
                            .border_color(rgb(0x8b5cf6))
                            .shadow_md()
                    )
            )
            // 7. Compact Table with Custom Width
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("7. Compact Table with Custom Width")
                    )
                    .child(
                        div()
                            .text_size(px(13.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("Using .w(px(500.0)), .p_4(), .rounded(px(12.0))")
                    )
                    .child(
                        Table::new()
                            .columns(vec![
                                TableColumn::new("Icon").width(px(80.0)),
                                TableColumn::new("Name").width(px(120.0)),
                                TableColumn::new("Size").width(px(80.0)),
                            ])
                            .rows(vec![
                                TableRow::new(vec![
                                    "📁".into(),
                                    "Documents".into(),
                                    "12 MB".into(),
                                ]),
                                TableRow::new(vec![
                                    "📄".into(),
                                    "Resume.pdf".into(),
                                    "245 KB".into(),
                                ]),
                                TableRow::new(vec![
                                    "🖼️".into(),
                                    "Photo.jpg".into(),
                                    "3.2 MB".into(),
                                ]),
                            ])
                            .w(px(500.0))
                            .p_4()
                            .rounded(px(12.0))
                    )
            )
            // 8. Ultra Custom Styling Showcase
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("8. Ultra Custom Styling Showcase")
                    )
                    .child(
                        div()
                            .text_size(px(13.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("Combined: .p(px(20.0)), .bg(), .border_2(), .rounded(px(16.0)), .shadow_lg(), .w_full()")
                    )
                    .child(
                        Table::new()
                            .columns(vec![
                                TableColumn::new("Metric").width(px(180.0)),
                                TableColumn::new("Value").width(px(120.0)),
                                TableColumn::new("Change").width(px(100.0)),
                                TableColumn::new("Trend").width(px(100.0)),
                            ])
                            .rows(vec![
                                TableRow::new(vec![
                                    "Revenue".into(),
                                    "$45,231".into(),
                                    "+12.5%".into(),
                                    "↑".into(),
                                ]),
                                TableRow::new(vec![
                                    "Users".into(),
                                    "3,456".into(),
                                    "+8.2%".into(),
                                    "↑".into(),
                                ]).selected(true),
                                TableRow::new(vec![
                                    "Conversion".into(),
                                    "2.4%".into(),
                                    "-0.3%".into(),
                                    "↓".into(),
                                ]),
                                TableRow::new(vec![
                                    "Satisfaction".into(),
                                    "4.8/5.0".into(),
                                    "+0.2".into(),
                                    "↑".into(),
                                ]),
                            ])
                            .p(px(20.0))
                            .bg(rgb(0xf59e0b))
                            .border_2()
                            .border_color(rgb(0xf59e0b))
                            .rounded(px(16.0))
                            .shadow_lg()
                            .w_full()
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
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("All customizations above use the Styled trait for full GPUI styling control!")
                    )
                    .child(
                        div()
                            .mt(px(8.0))
                            .text_size(px(12.0))
                            .text_color(theme.tokens.accent_foreground)
                            .child("Methods used: .bg(), .p_4(), .p_8(), .p(), .border_2(), .border_color(), .rounded(), .shadow_md(), .shadow_lg(), .w_full(), .w()")
                    )
                    .child(
                        div()
                            .mt(px(4.0))
                            .text_size(px(12.0))
                            .text_color(theme.tokens.accent_foreground)
                            .child("The Styled trait enables direct access to GPUI's full styling system without component-specific constraints.")
                    )
            )
                )
            )
    }
}
