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
                        title: Some("DataTable Styled Trait Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(1200.0), px(1000.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|cx| DataTableStyledDemo::new(cx)),
            )
            .unwrap();
        });
}

#[derive(Clone, Debug)]
struct Product {
    id: u32,
    name: String,
    category: String,
    price: String,
    stock: String,
}

struct DataTableStyledDemo {
    table1: Entity<DataTable<Product>>,
    table2: Entity<DataTable<Product>>,
    table3: Entity<DataTable<Product>>,
    table4: Entity<DataTable<Product>>,
    table5: Entity<DataTable<Product>>,
    table6: Entity<DataTable<Product>>,
}

impl DataTableStyledDemo {
    fn new(cx: &mut App) -> Self {
        let data = Self::generate_data(20);

        // 1. Default DataTable
        let table1 = cx.new(|cx| {
            DataTable::new(data.clone(), Self::create_columns(), cx)
                .show_search(false)
        });

        // 2. Custom Border Styling
        let table2 = cx.new(|cx| {
            DataTable::new(data.clone(), Self::create_columns(), cx)
                .show_search(false)
                .border_4()
                .border_color(rgb(0x3b82f6))
                .rounded(px(20.0))
        });

        // 3. Custom Background and Shadow
        let table3 = cx.new(|cx| {
            DataTable::new(data.clone(), Self::create_columns(), cx)
                .show_search(false)
                .bg(rgb(0x1e293b))
                .shadow_xl()
                .rounded(px(16.0))
        });

        // 4. Custom Width and Height
        let table4 = cx.new(|cx| {
            DataTable::new(data.clone(), Self::create_columns(), cx)
                .show_search(false)
                .w(px(600.0))
                .h(px(300.0))
        });

        // 5. Custom Padding
        let table5 = cx.new(|cx| {
            DataTable::new(data.clone(), Self::create_columns(), cx)
                .show_search(false)
                .p(px(24.0))
                .bg(rgb(0x374151))
        });

        // 6. Combined Styling
        let table6 = cx.new(|cx| {
            DataTable::new(data.clone(), Self::create_columns(), cx)
                .show_search(false)
                .border_3()
                .border_color(rgb(0x8b5cf6))
                .rounded(px(24.0))
                .shadow_2xl()
                .bg(rgb(0x312e81))
                .p(px(16.0))
        });

        Self {
            table1,
            table2,
            table3,
            table4,
            table5,
            table6,
        }
    }

    fn generate_data(count: usize) -> Vec<Product> {
        let names = vec![
            "Wireless Mouse", "Mechanical Keyboard", "USB-C Hub", "Monitor Stand",
            "Laptop Sleeve", "Webcam", "Headphones", "Desk Lamp",
        ];
        let categories = vec!["Electronics", "Accessories", "Peripherals", "Office"];

        (0..count)
            .map(|i| {
                let name = names[i % names.len()];
                let category = categories[i % categories.len()];

                Product {
                    id: (i + 1) as u32,
                    name: format!("{} #{}", name, i + 1),
                    category: category.to_string(),
                    price: format!("${}.99", 19 + (i * 7) % 100),
                    stock: format!("{}", 50 + (i * 13) % 200),
                }
            })
            .collect()
    }

    fn create_columns() -> Vec<ColumnDef<Product>> {
        vec![
            ColumnDef::new("id", "ID", |product: &Product| product.id.to_string().into())
                .width(px(80.0))
                .sortable(true),
            ColumnDef::new("name", "Product Name", |product: &Product| product.name.clone().into())
                .width(px(250.0))
                .sortable(true),
            ColumnDef::new("category", "Category", |product: &Product| product.category.clone().into())
                .width(px(150.0))
                .sortable(true),
            ColumnDef::new("price", "Price", |product: &Product| product.price.clone().into())
                .width(px(120.0))
                .sortable(true),
            ColumnDef::new("stock", "Stock", |product: &Product| product.stock.clone().into())
                .width(px(100.0))
                .sortable(true),
        ]
    }
}

impl Render for DataTableStyledDemo {
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
                                        .text_size(px(28.0))
                                        .font_weight(FontWeight::BOLD)
                                        .child("DataTable Styled Trait Customization Demo")
                                )
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child("Demonstrating full GPUI styling control via Styled trait for DataTable")
                                )
                        )
                        // 1. Default DataTable
                        .child(
                            VStack::new()
                                .gap(px(12.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("1. Default DataTable (No Styling)")
                                )
                                .child(
                                    div()
                                        .text_size(px(13.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child("Standard appearance with default theme styling")
                                )
                                .child(self.table1.clone())
                        )
                        // 2. Custom Border Styling
                        .child(
                            VStack::new()
                                .gap(px(12.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("2. Custom Border Styling")
                                )
                                .child(
                                    div()
                                        .text_size(px(13.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child("Using .border_4(), .border_color(), .rounded() to customize appearance")
                                )
                                .child(self.table2.clone())
                        )
                        // 3. Custom Background and Shadow
                        .child(
                            VStack::new()
                                .gap(px(12.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("3. Custom Background and Shadow")
                                )
                                .child(
                                    div()
                                        .text_size(px(13.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child("Using .bg(), .shadow_xl(), .rounded() for darker theme with depth")
                                )
                                .child(self.table3.clone())
                        )
                        // 4. Custom Width and Height
                        .child(
                            VStack::new()
                                .gap(px(12.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("4. Custom Width and Height")
                                )
                                .child(
                                    div()
                                        .text_size(px(13.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child("Using .w() and .h() to control table dimensions")
                                )
                                .child(self.table4.clone())
                        )
                        // 5. Custom Padding
                        .child(
                            VStack::new()
                                .gap(px(12.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("5. Custom Padding")
                                )
                                .child(
                                    div()
                                        .text_size(px(13.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child("Using .p() and .bg() for custom spacing and background")
                                )
                                .child(self.table5.clone())
                        )
                        // 6. Combined Styling
                        .child(
                            VStack::new()
                                .gap(px(12.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("6. Combined Styling (Multiple Styled Trait Methods)")
                                )
                                .child(
                                    div()
                                        .text_size(px(13.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child("Combining .border_3(), .border_color(), .rounded(), .shadow_2xl(), .bg(), .p()")
                                )
                                .child(self.table6.clone())
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
                                        .child("Methods used: .border_4(), .border_color(), .rounded(), .bg(), .shadow_xl(), .shadow_2xl(), .w(), .h(), .p()")
                                )
                        )
                )
            )
    }
}
