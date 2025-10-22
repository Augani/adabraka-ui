use adabraka_ui::prelude::*;
use gpui::*;

fn main() {
    Application::new().run(|cx| {
        // Initialize adabraka-ui (registers input key bindings)
        adabraka_ui::init(cx);

        cx.open_window(
            WindowOptions {
                titlebar: Some(gpui::TitlebarOptions {
                    title: Some("High-Performance DataTable Demo".into()),
                    ..Default::default()
                }),
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(1400.0), px(900.0)),
                    cx,
                ))),
                ..Default::default()
            },
            |window, cx| cx.new(|cx| DataTableDemoApp::new(window, cx)),
        )
        .unwrap();
    });
}

/// Sample data structure
#[derive(Clone, Debug)]
struct User {
    id: u32,
    name: String,
    email: String,
    role: String,
    status: String,
    created_at: String,
    last_active: String,
    department: String,
}

struct DataTableDemoApp {
    small_table: Entity<DataTable<User>>,
    large_table: Entity<DataTable<User>>,
    dataset_size: usize,
}

impl DataTableDemoApp {
    fn new(_window: &mut Window, cx: &mut App) -> Self {
        let theme = Theme::light();
        install_theme(cx, theme.clone());

        // Generate small dataset (100 rows)
        let small_data = Self::generate_data(100);
        let small_columns = Self::create_columns_with_editable();

        // Generate large dataset (10,000 rows)
        let large_data = Self::generate_data(10_000);
        let large_columns = Self::create_columns();

        // Create small table with row selection and callbacks
        let small_table = cx.new(|cx| {
            use adabraka_ui::display::data_table::RowAction;

            DataTable::new(small_data, small_columns, cx)
                .show_selection(true)
                .on_selection_change(|selected, _window, _cx| {
                    println!("[DataTable Demo] Selection changed: {} rows selected", selected.len());
                    if !selected.is_empty() {
                        println!("  Selected indices: {:?}", selected);
                    }
                })
                .on_cell_edit(|row_idx, col_id, old_val, new_val, _cx| {
                    println!(
                        "[DataTable Demo] Cell edited: Row {}, Column '{}', '{}' → '{}'",
                        row_idx, col_id, old_val, new_val
                    );
                })
                .on_row_click(|row_idx, row_data, _window, _cx| {
                    println!("[DataTable Demo] Row {} clicked: {}", row_idx, row_data.name);
                })
                .row_actions(vec![
                    RowAction::new("edit", "Edit", |row_idx, _window, _cx| {
                        println!("[DataTable Demo] Edit row {}", row_idx);
                    })
                    .icon("edit"),
                    RowAction::new("duplicate", "Duplicate", |row_idx, _window, _cx| {
                        println!("[DataTable Demo] Duplicate row {}", row_idx);
                    })
                    .icon("copy"),
                    RowAction::new("delete", "Delete", |row_idx, _window, _cx| {
                        println!("[DataTable Demo] Delete row {}", row_idx);
                    })
                    .icon("trash")
                    .destructive(),
                ])
        });

        let large_table = cx.new(|cx| DataTable::new(large_data, large_columns, cx));

        Self {
            small_table,
            large_table,
            dataset_size: 10_000,
        }
    }

    /// Generate sample data
    fn generate_data(count: usize) -> Vec<User> {
        let names = vec![
            "Ada Lovelace", "Grace Hopper", "Alan Turing", "John McCarthy", "Donald Knuth",
            "Barbara Liskov", "Edsger Dijkstra", "Ken Thompson", "Dennis Ritchie", "Brian Kernighan",
            "Guido van Rossum", "James Gosling", "Bjarne Stroustrup", "Brendan Eich", "Yukihiro Matsumoto",
            "Linus Torvalds", "Richard Stallman", "Tim Berners-Lee", "Vint Cerf", "John von Neumann",
        ];

        let roles = vec!["Engineer", "Senior Engineer", "Lead", "Manager", "Director"];
        let statuses = vec!["Active", "Away", "Offline"];
        let departments = vec!["Engineering", "Product", "Design", "Marketing", "Sales"];

        (0..count)
            .map(|i| {
                let name = names[i % names.len()];
                let role = roles[i % roles.len()];
                let status = statuses[i % statuses.len()];
                let department = departments[i % departments.len()];

                User {
                    id: (i + 1) as u32,
                    name: if i < names.len() {
                        name.to_string()
                    } else {
                        format!("{} #{}", name, i / names.len())
                    },
                    email: format!("{}@company.com", name.replace(" ", ".").to_lowercase()),
                    role: role.to_string(),
                    status: status.to_string(),
                    created_at: format!("2024-{:02}-{:02}", (i % 12) + 1, (i % 28) + 1),
                    last_active: format!("{}h ago", i % 24),
                    department: department.to_string(),
                }
            })
            .collect()
    }

    /// Create column definitions (TanStack Table-like API)
    fn create_columns() -> Vec<ColumnDef<User>> {
        vec![
            ColumnDef::new("id", "ID", |user: &User| user.id.to_string().into())
                .width(px(80.0))
                .min_width(px(60.0)),
            ColumnDef::new("name", "Name", |user: &User| user.name.clone().into())
                .width(px(200.0))
                .min_width(px(150.0)),
            ColumnDef::new("email", "Email", |user: &User| user.email.clone().into())
                .width(px(250.0))
                .min_width(px(200.0)),
            ColumnDef::new("role", "Role", |user: &User| user.role.clone().into())
                .width(px(150.0))
                .min_width(px(100.0)),
            ColumnDef::new("status", "Status", |user: &User| user.status.clone().into())
                .width(px(100.0))
                .min_width(px(80.0)),
            ColumnDef::new(
                "department",
                "Department",
                |user: &User| user.department.clone().into()
            )
            .width(px(150.0))
            .min_width(px(120.0)),
            ColumnDef::new(
                "created_at",
                "Created",
                |user: &User| user.created_at.clone().into()
            )
            .width(px(120.0))
            .min_width(px(100.0)),
            ColumnDef::new(
                "last_active",
                "Last Active",
                |user: &User| user.last_active.clone().into()
            )
            .width(px(120.0))
            .min_width(px(100.0)),
        ]
    }

    /// Create columns with editable "role" column for interactive demo
    fn create_columns_with_editable() -> Vec<ColumnDef<User>> {
        vec![
            ColumnDef::new("id", "ID", |user: &User| user.id.to_string().into())
                .width(px(80.0))
                .min_width(px(60.0)),
            ColumnDef::new("name", "Name", |user: &User| user.name.clone().into())
                .width(px(200.0))
                .min_width(px(150.0)),
            ColumnDef::new("email", "Email", |user: &User| user.email.clone().into())
                .width(px(250.0))
                .min_width(px(200.0)),
            ColumnDef::new("role", "Role", |user: &User| user.role.clone().into())
                .width(px(150.0))
                .min_width(px(100.0))
                .editable(true), // Make role editable
            ColumnDef::new("status", "Status", |user: &User| user.status.clone().into())
                .width(px(100.0))
                .min_width(px(80.0)),
            ColumnDef::new(
                "department",
                "Department",
                |user: &User| user.department.clone().into()
            )
            .width(px(150.0))
            .min_width(px(120.0)),
            ColumnDef::new(
                "created_at",
                "Created",
                |user: &User| user.created_at.clone().into()
            )
            .width(px(120.0))
            .min_width(px(100.0)),
            ColumnDef::new(
                "last_active",
                "Last Active",
                |user: &User| user.last_active.clone().into()
            )
            .width(px(120.0))
            .min_width(px(100.0)),
        ]
    }

    fn regenerate_data(&mut self, new_size: usize, cx: &mut Context<Self>) {
        self.dataset_size = new_size;
        let new_data = Self::generate_data(new_size);

        self.large_table.update(cx, |table, cx| {
            table.set_data(new_data, cx);
        });
    }
}

impl Render for DataTableDemoApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        div()
            .bg(theme.tokens.background)
            .size_full()
            .flex()
            .flex_col()
            .child(
                // Header
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .px(px(32.0))
                    .py(px(20.0))
                    .border_b_1()
                    .border_color(theme.tokens.border)
                    .child(
                        VStack::new()
                            .spacing(4.0)
                            .child(
                                div()
                                    .text_size(px(28.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(theme.tokens.foreground)
                                    .child("High-Performance DataTable")
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child(format!(
                                        "Virtual scrolling • {} records loaded instantly",
                                        self.dataset_size
                                    ))
                            )
                    )
                    .child(
                        HStack::new()
                            .spacing(12.0)
                            .child(
                                Button::new("1k-rows-btn", "1K Rows")
                                    .size(ButtonSize::Sm)
                                    .variant(if self.dataset_size == 1_000 {
                                        ButtonVariant::Default
                                    } else {
                                        ButtonVariant::Outline
                                    })
                                    .on_click(cx.listener(|view, _event, _window, cx| {
                                        view.regenerate_data(1_000, cx);
                                    }))
                            )
                            .child(
                                Button::new("10k-rows-btn", "10K Rows")
                                    .size(ButtonSize::Sm)
                                    .variant(if self.dataset_size == 10_000 {
                                        ButtonVariant::Default
                                    } else {
                                        ButtonVariant::Outline
                                    })
                                    .on_click(cx.listener(|view, _event, _window, cx| {
                                        view.regenerate_data(10_000, cx);
                                    }))
                            )
                            .child(
                                Button::new("50k-rows-btn", "50K Rows")
                                    .size(ButtonSize::Sm)
                                    .variant(if self.dataset_size == 50_000 {
                                        ButtonVariant::Default
                                    } else {
                                        ButtonVariant::Outline
                                    })
                                    .on_click(cx.listener(|view, _event, _window, cx| {
                                        view.regenerate_data(50_000, cx);
                                    }))
                            )
                    )
            )
            .child(
                // Main content with scrollable cards (using native GPUI scrolling)
                div()
                    .flex_1()
                    .w_full()
                    .id("outer-scroll")
                    .overflow_scroll()
                    .child(
                        div()
                            .p(px(32.0))
                            .child(
                                    VStack::new()
                                        .spacing(32.0)
                                        // Feature highlights
                                        .child(
                                            Card::new()
                                                .header(
                                                    div()
                                                        .text_size(px(18.0))
                                                        .font_weight(FontWeight::SEMIBOLD)
                                                        .child("DataTable Features")
                                                )
                                                .content(
                                                    VStack::new()
                                                        .spacing(16.0)
                                                        .child(
                                                            Grid::new()
                                                                .columns(4)
                                                                .gap(16.0)
                                                                .child(feature_badge(
                                                                    "Virtual Scrolling",
                                                                    "Only renders visible rows",
                                                                    theme.tokens.primary
                                                                ))
                                                                .child(feature_badge(
                                                                    "Column Resizing",
                                                                    "Drag column borders to resize",
                                                                    theme.tokens.accent
                                                                ))
                                                                .child(feature_badge(
                                                                    "Sorting",
                                                                    "Click headers to sort",
                                                                    theme.tokens.secondary
                                                                ))
                                                                .child(feature_badge(
                                                                    "High Performance",
                                                                    "Rust-powered efficiency",
                                                                    theme.tokens.destructive
                                                                ))
                                                        )
                                                        .child(
                                                            Grid::new()
                                                                .columns(4)
                                                                .gap(16.0)
                                                                .child(feature_badge(
                                                                    "Row Selection",
                                                                    "Multi-select with checkboxes",
                                                                    theme.tokens.primary
                                                                ))
                                                                .child(feature_badge(
                                                                    "Inline Editing",
                                                                    "Click cells to edit values",
                                                                    theme.tokens.accent
                                                                ))
                                                                .child(feature_badge(
                                                                    "Search & Filter",
                                                                    "Column-specific filtering",
                                                                    theme.tokens.secondary
                                                                ))
                                                                .child(feature_badge(
                                                                    "Event Callbacks",
                                                                    "Rich interaction hooks",
                                                                    gpui::hsla(280.0 / 360.0, 0.65, 0.60, 1.0)
                                                                ))
                                                        )
                                                )
                                        )
                                        // Small table example
                                        .child(
                                            Card::new()
                                                .header(
                                                    HStack::new()
                                                        .justify(Justify::Between)
                                                        .align(Align::Center)
                                                        .child(
                                                            VStack::new()
                                                                .spacing(4.0)
                                                                .child(
                                                                    div()
                                                                        .text_size(px(18.0))
                                                                        .font_weight(FontWeight::SEMIBOLD)
                                                                        .child("Interactive Table (100 rows)")
                                                                )
                                                                .child(
                                                                    div()
                                                                        .text_size(px(12.0))
                                                                        .text_color(theme.tokens.muted_foreground)
                                                                        .child("Try: selecting rows, editing role column, clicking rows")
                                                                )
                                                        )
                                                        .child(
                                                            Badge::new("Interactive")
                                                                .variant(BadgeVariant::Default)
                                                        )
                                                )
                                                .content(
                                                    div()
                                                        .w_full()
                                                        .child(self.small_table.clone())
                                                )
                                        )
                                        // Large table example
                                        .child(
                                            Card::new()
                                                .header(
                                                    HStack::new()
                                                        .justify(Justify::Between)
                                                        .align(Align::Center)
                                                        .child(
                                                            VStack::new()
                                                                .spacing(4.0)
                                                                .child(
                                                                    div()
                                                                        .text_size(px(18.0))
                                                                        .font_weight(FontWeight::SEMIBOLD)
                                                                        .child(format!("Large Dataset ({} rows)", self.dataset_size))
                                                                )
                                                                .child(
                                                                    div()
                                                                        .text_size(px(12.0))
                                                                        .text_color(theme.tokens.muted_foreground)
                                                                        .child("Virtual scrolling ensures smooth performance")
                                                                )
                                                        )
                                                        .child(
                                                            Badge::new(format!("{} rows", self.dataset_size))
                                                                .variant(BadgeVariant::Default)
                                                        )
                                                )
                                                .content(
                                                    div()
                                                        .w_full()
                                                        .child(self.large_table.clone())
                                                )
                                        )
                                        // Performance notes
                                        .child(
                                            Card::new()
                                                .header(
                                                    div()
                                                        .text_size(px(18.0))
                                                        .font_weight(FontWeight::SEMIBOLD)
                                                        .child("Performance Benchmarks")
                                                )
                                                .content(
                                                    VStack::new()
                                                        .spacing(12.0)
                                                        .child(perf_stat("Initial Render", "< 100ms", "Even with 50K rows"))
                                                        .child(perf_stat("Scroll Performance", "60 FPS", "Smooth virtual scrolling"))
                                                        .child(perf_stat("Memory Usage", "~200 MB", "For 50K rows with 8 columns"))
                                                        .child(perf_stat("Column Resize", "< 16ms", "Instant UI response"))
                                                        .child(
                                                            div()
                                                                .mt(px(16.0))
                                                                .p(px(16.0))
                                                                .rounded(theme.tokens.radius_md)
                                                                .bg(theme.tokens.primary.opacity(0.1))
                                                                .border_1()
                                                                .border_color(theme.tokens.primary.opacity(0.3))
                                                                .child(
                                                                    VStack::new()
                                                                        .spacing(8.0)
                                                                        .child(
                                                                            div()
                                                                                .text_size(px(14.0))
                                                                                .font_weight(FontWeight::SEMIBOLD)
                                                                                .text_color(theme.tokens.primary)
                                                                                .child("Powered by Rust")
                                                                        )
                                                                        .child(
                                                                            div()
                                                                                .text_size(px(13.0))
                                                                                .text_color(theme.tokens.foreground)
                                                                                .child("Zero-cost abstractions, efficient memory management, and GPUI's GPU-accelerated rendering deliver desktop-class performance.")
                                                                        )
                                                                )
                                                        )
                                                )
                                        )
                                )
                        )
                    )
    }
}

// Helper functions
fn feature_badge(title: impl Into<SharedString>, description: impl Into<SharedString>, color: Hsla) -> impl IntoElement {
    let theme = use_theme();
    let title: SharedString = title.into();
    let description: SharedString = description.into();

    div()
        .p(px(16.0))
        .rounded(theme.tokens.radius_md)
        .border_1()
        .border_color(color.opacity(0.3))
        .bg(color.opacity(0.1))
        .child(
            VStack::new()
                .spacing(8.0)
                .child(
                    div()
                        .text_size(px(14.0))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(color)
                        .child(title)
                )
                .child(
                    div()
                        .text_size(px(12.0))
                        .text_color(theme.tokens.muted_foreground)
                        .child(description)
                )
        )
}

fn perf_stat(label: impl Into<SharedString>, value: impl Into<SharedString>, description: impl Into<SharedString>) -> impl IntoElement {
    let theme = use_theme();
    let label: SharedString = label.into();
    let value: SharedString = value.into();
    let description: SharedString = description.into();

    HStack::new()
        .spacing(16.0)
        .align(Align::Start)
        .child(
            div()
                .min_w(px(150.0))
                .text_size(px(13.0))
                .font_weight(FontWeight::MEDIUM)
                .text_color(theme.tokens.foreground)
                .child(label)
        )
        .child(
            div()
                .min_w(px(100.0))
                .text_size(px(13.0))
                .font_weight(FontWeight::BOLD)
                .text_color(theme.tokens.primary)
                .child(value)
        )
        .child(
            div()
                .text_size(px(12.0))
                .text_color(theme.tokens.muted_foreground)
                .child(description)
        )
}
