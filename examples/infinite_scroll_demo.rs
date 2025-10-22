use adabraka_ui::prelude::*;
use gpui::*;

fn main() {
    Application::new().run(|cx| {
        cx.open_window(
            WindowOptions {
                titlebar: Some(gpui::TitlebarOptions {
                    title: Some("Infinite Scroll DataTable Demo".into()),
                    ..Default::default()
                }),
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(1400.0), px(900.0)),
                    cx,
                ))),
                ..Default::default()
            },
            |window, cx| cx.new(|cx| InfiniteScrollApp::new(window, cx)),
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
}

struct InfiniteScrollApp {
    table: Entity<DataTable<User>>,
    next_id: u32,
    total_loaded: usize,
}

impl InfiniteScrollApp {
    fn new(_window: &mut Window, cx: &mut App) -> Self {
        let theme = Theme::dark();
        install_theme(cx, theme.clone());

        // Start with just 100 rows
        let initial_data = Self::generate_batch(1, 100);
        let columns = Self::create_columns();

        let table = cx.new(|cx| {
            DataTable::new(initial_data, columns, cx)
                .on_load_more(|_window, cx| {
                    println!("[Infinite Scroll] on_load_more triggered!");
                    cx.notify();
                })
                .load_more_threshold(0.7) // Trigger at 70%
        });

        Self {
            table,
            next_id: 101,
            total_loaded: 100,
        }
    }

    fn generate_batch(start_id: u32, count: usize) -> Vec<User> {
        let names = vec![
            "Ada Lovelace", "Grace Hopper", "Alan Turing", "John McCarthy", "Donald Knuth",
            "Barbara Liskov", "Edsger Dijkstra", "Ken Thompson", "Dennis Ritchie", "Brian Kernighan",
        ];

        let roles = vec!["Engineer", "Senior Engineer", "Lead", "Manager", "Director"];
        let statuses = vec!["Active", "Away", "Offline"];

        (0..count)
            .map(|i| {
                let id = start_id + i as u32;
                let name = names[i % names.len()];
                let role = roles[i % roles.len()];
                let status = statuses[i % statuses.len()];

                User {
                    id,
                    name: format!("{} #{}", name, id),
                    email: format!("user{}@company.com", id),
                    role: role.to_string(),
                    status: status.to_string(),
                }
            })
            .collect()
    }

    fn create_columns() -> Vec<ColumnDef<User>> {
        vec![
            ColumnDef::new("id", "ID", |user: &User| user.id.to_string().into())
                .width(px(100.0))
                .min_width(px(80.0)),
            ColumnDef::new("name", "Name", |user: &User| user.name.clone().into())
                .width(px(250.0))
                .min_width(px(200.0)),
            ColumnDef::new("email", "Email", |user: &User| user.email.clone().into())
                .width(px(300.0))
                .min_width(px(250.0)),
            ColumnDef::new("role", "Role", |user: &User| user.role.clone().into())
                .width(px(200.0))
                .min_width(px(150.0)),
            ColumnDef::new("status", "Status", |user: &User| user.status.clone().into())
                .width(px(150.0))
                .min_width(px(100.0)),
        ]
    }

    fn load_more_data(&mut self, cx: &mut Context<Self>) {
        println!("[Infinite Scroll] Loading more data... Currently loaded: {}", self.total_loaded);

        // In a real app, this would be an async API call
        // For demo, we'll just generate 50 more rows immediately
        let batch_size = 50;
        let new_data = Self::generate_batch(self.next_id, batch_size);

        self.table.update(cx, |table, cx| {
            table.append_data(new_data, cx);
        });

        self.next_id += batch_size as u32;
        self.total_loaded += batch_size;

        println!("[Infinite Scroll] Loaded {} more rows. Total: {}", batch_size, self.total_loaded);
        cx.notify();
    }
}

impl Render for InfiniteScrollApp {
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
                                    .child("Infinite Scroll DataTable")
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child(format!(
                                        "Started with 100 rows • Currently loaded: {} rows • Scroll down to load more",
                                        self.total_loaded
                                    ))
                            )
                    )
                    .child(
                        HStack::new()
                            .spacing(12.0)
                            .child(
                                Badge::new(format!("{} rows", self.total_loaded))
                                    .variant(BadgeVariant::Secondary)
                            )
                            .child(
                                Button::new("load-more-btn", "Load More")
                                    .size(ButtonSize::Sm)
                                    .variant(ButtonVariant::Default)
                                    .on_click(cx.listener(|view, _event, _window, cx| {
                                        view.load_more_data(cx);
                                    }))
                            )
                    )
            )
            .child(
                // Main content
                div()
                    .flex_1()
                    .w_full()
                    .overflow_hidden()
                    .child(
                        scrollable_vertical(
                            div()
                                .p(px(32.0))
                                .child(
                                    VStack::new()
                                        .spacing(24.0)
                                        .child(
                                            Card::new()
                                                .header(
                                                    div()
                                                        .text_size(px(18.0))
                                                        .font_weight(FontWeight::SEMIBOLD)
                                                        .child("How It Works")
                                                )
                                                .content(
                                                    VStack::new()
                                                        .spacing(12.0)
                                                        .child(
                                                            div()
                                                                .text_size(px(14.0))
                                                                .text_color(theme.tokens.foreground)
                                                                .child("1. Started with 100 rows")
                                                        )
                                                        .child(
                                                            div()
                                                                .text_size(px(14.0))
                                                                .text_color(theme.tokens.foreground)
                                                                .child("2. When you scroll past 70% of loaded data, 50 more rows load automatically")
                                                        )
                                                        .child(
                                                            div()
                                                                .text_size(px(14.0))
                                                                .text_color(theme.tokens.foreground)
                                                                .child("3. The table grows dynamically as you scroll")
                                                        )
                                                        .child(
                                                            div()
                                                                .text_size(px(14.0))
                                                                .text_color(theme.tokens.foreground)
                                                                .child("4. Virtual scrolling keeps performance smooth even with thousands of rows")
                                                        )
                                                )
                                        )
                                        .child(
                                            Card::new()
                                                .header(
                                                    HStack::new()
                                                        .justify(Justify::Between)
                                                        .align(Align::Center)
                                                        .child(
                                                            div()
                                                                .text_size(px(18.0))
                                                                .font_weight(FontWeight::SEMIBOLD)
                                                                .child("User Data")
                                                        )
                                                        .child(
                                                            Badge::new(format!("{} loaded", self.total_loaded))
                                                                .variant(BadgeVariant::Default)
                                                        )
                                                )
                                                .content(
                                                    div()
                                                        .w_full()
                                                        .child(self.table.clone())
                                                )
                                        )
                                )
                        )
                    )
            )
    }
}
