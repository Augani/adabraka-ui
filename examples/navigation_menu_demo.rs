use gpui::*;
use gpui::prelude::FluentBuilder;
use adabraka_ui::{
    prelude::*,
    components::{
        navigation_menu::{NavigationMenu, NavigationMenuItem, NavigationMenuOrientation},
        scrollable::scrollable_vertical,
    },
    layout::{VStack, HStack},
};
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

    fn list(&self, path: &str) -> Result<Vec<gpui::SharedString>> {
        std::fs::read_dir(self.base.join(path))
            .map(|entries| {
                entries
                    .filter_map(|entry| {
                        entry
                            .ok()
                            .and_then(|entry| entry.file_name().into_string().ok())
                            .map(gpui::SharedString::from)
                    })
                    .collect()
            })
            .map_err(|err| err.into())
    }
}

struct NavigationMenuDemo {
    // Vertical menu state
    selected_vertical_item: Option<String>,
    expanded_vertical_items: Vec<String>,

    // Horizontal menu state
    selected_horizontal_item: Option<String>,
    expanded_horizontal_items: Vec<String>,
}

impl NavigationMenuDemo {
    fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            selected_vertical_item: Some("home".to_string()),
            expanded_vertical_items: vec!["products".to_string()], // Products expanded by default
            selected_horizontal_item: Some("dashboard".to_string()),
            expanded_horizontal_items: vec![],
        }
    }
}

impl Render for NavigationMenuDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        VStack::new()
            .size_full()
            .bg(theme.tokens.background)
            .text_color(theme.tokens.foreground)
            // Header
            .child(
                VStack::new()
                    .w_full()
                    .h(px(80.0))
                    .justify_center()
                    .items_center()
                    .border_b_1()
                    .border_color(theme.tokens.border)
                    .child(
                        VStack::new()
                            .items_center()
                            .gap(px(4.0))
                            .child(
                                div()
                                    .text_size(px(24.0))
                                    .font_weight(FontWeight::BOLD)
                                    .child("NavigationMenu Component Demo"),
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Professional hierarchical navigation with state management"),
                            ),
                    ),
            )
            // Main content
            .child(
                div()
                    .flex_1()
                    .w_full()
                    .overflow_hidden()
                    .child(
                        scrollable_vertical(
                            div()
                                .flex()
                                .flex_col()
                                .w_full()
                                .p(px(32.0))
                                .gap(px(48.0))
                                // Vertical Menu Section
                                .child(
                                    VStack::new()
                                        .w_full()
                                        .gap(px(16.0))
                                        .child(
                                            div()
                                                .text_size(px(18.0))
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .child("Vertical Navigation Menu"),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(14.0))
                                                .text_color(theme.tokens.muted_foreground)
                                                .child("Perfect for sidebars with hierarchical structure. Click items to select, click chevron to expand/collapse."),
                                        )
                                        .child(
                                            HStack::new()
                                                .w_full()
                                                .gap(px(24.0))
                                                .items_start()
                                                // Menu
                                                .child(
                                                    div()
                                                        .w(px(280.0))
                                                        .border_1()
                                                        .border_color(theme.tokens.border)
                                                        .rounded(theme.tokens.radius_md)
                                                        .overflow_hidden()
                                                        .bg(theme.tokens.card)
                                                        .child({
                                                            let selected = self.selected_vertical_item.clone();
                                                            let expanded = self.expanded_vertical_items.clone();
                                                            let app_entity = cx.entity().downgrade();

                                                            NavigationMenu::new()
                                                                .orientation(NavigationMenuOrientation::Vertical)
                                                                .items(vec![
                                                                    NavigationMenuItem::new("home".to_string(), "Home")
                                                                        .with_icon("assets/icons/folder.svg"),
                                                                    NavigationMenuItem::new("dashboard".to_string(), "Dashboard")
                                                                        .with_icon("assets/icons/folder.svg"),
                                                                    NavigationMenuItem::new("products".to_string(), "Products")
                                                                        .with_icon("assets/icons/folder.svg")
                                                                        .with_children(vec![
                                                                            NavigationMenuItem::new("electronics".to_string(), "Electronics")
                                                                                .with_icon("assets/icons/file.svg"),
                                                                            NavigationMenuItem::new("clothing".to_string(), "Clothing")
                                                                                .with_icon("assets/icons/file.svg")
                                                                                .with_children(vec![
                                                                                    NavigationMenuItem::new("mens".to_string(), "Men's"),
                                                                                    NavigationMenuItem::new("womens".to_string(), "Women's"),
                                                                                    NavigationMenuItem::new("kids".to_string(), "Kids"),
                                                                                ]),
                                                                            NavigationMenuItem::new("books".to_string(), "Books")
                                                                                .with_icon("assets/icons/file.svg"),
                                                                        ]),
                                                                    NavigationMenuItem::new("orders".to_string(), "Orders")
                                                                        .with_icon("assets/icons/folder.svg")
                                                                        .with_children(vec![
                                                                            NavigationMenuItem::new("pending".to_string(), "Pending"),
                                                                            NavigationMenuItem::new("completed".to_string(), "Completed"),
                                                                            NavigationMenuItem::new("cancelled".to_string(), "Cancelled"),
                                                                        ]),
                                                                    NavigationMenuItem::new("customers".to_string(), "Customers")
                                                                        .with_icon("assets/icons/folder.svg"),
                                                                    NavigationMenuItem::new("analytics".to_string(), "Analytics")
                                                                        .with_icon("assets/icons/folder.svg")
                                                                        .with_children(vec![
                                                                            NavigationMenuItem::new("sales".to_string(), "Sales Reports"),
                                                                            NavigationMenuItem::new("traffic".to_string(), "Traffic"),
                                                                        ]),
                                                                    NavigationMenuItem::new("settings".to_string(), "Settings")
                                                                        .with_icon("assets/icons/file.svg"),
                                                                    NavigationMenuItem::new("disabled".to_string(), "Disabled Item")
                                                                        .with_icon("assets/icons/file.svg")
                                                                        .disabled(true),
                                                                ])
                                                                .selected_id(selected.unwrap_or_default())
                                                                .expanded_ids(expanded)
                                                                .on_select({
                                                                    let app_entity = app_entity.clone();
                                                                    move |id, _window, cx| {
                                                                        if let Some(app) = app_entity.upgrade() {
                                                                            app.update(cx, |demo, cx| {
                                                                                demo.selected_vertical_item = Some(id.clone());
                                                                                cx.notify();
                                                                            });
                                                                        }
                                                                    }
                                                                })
                                                                .on_toggle({
                                                                    let app_entity = app_entity.clone();
                                                                    move |id, is_expanded, _window, cx| {
                                                                        if let Some(app) = app_entity.upgrade() {
                                                                            app.update(cx, |demo, cx| {
                                                                                if is_expanded {
                                                                                    if !demo.expanded_vertical_items.contains(id) {
                                                                                        demo.expanded_vertical_items.push(id.clone());
                                                                                    }
                                                                                } else {
                                                                                    demo.expanded_vertical_items.retain(|item_id| item_id != id);
                                                                                }
                                                                                cx.notify();
                                                                            });
                                                                        }
                                                                    }
                                                                })
                                                        })
                                                )
                                                // State display
                                                .child(
                                                    div()
                                                        .flex_1()
                                                        .child(
                                                            VStack::new()
                                                                .gap(px(12.0))
                                                                .child(
                                                                    div()
                                                                        .p(px(16.0))
                                                                        .bg(theme.tokens.muted.opacity(0.3))
                                                                        .rounded(theme.tokens.radius_md)
                                                                        .border_1()
                                                                        .border_color(theme.tokens.border)
                                                                        .child(
                                                                            VStack::new()
                                                                                .gap(px(8.0))
                                                                                .child(
                                                                                    div()
                                                                                        .text_size(px(14.0))
                                                                                        .font_weight(FontWeight::SEMIBOLD)
                                                                                        .child("Current State")
                                                                                )
                                                                                .child(
                                                                                    div()
                                                                                        .text_size(px(13.0))
                                                                                        .child(format!(
                                                                                            "Selected: {}",
                                                                                            self.selected_vertical_item.as_ref().map(|s| s.as_str()).unwrap_or("None")
                                                                                        ))
                                                                                )
                                                                                .child(
                                                                                    div()
                                                                                        .text_size(px(13.0))
                                                                                        .child(format!(
                                                                                            "Expanded Items: {}",
                                                                                            if self.expanded_vertical_items.is_empty() {
                                                                                                "None".to_string()
                                                                                            } else {
                                                                                                self.expanded_vertical_items.join(", ")
                                                                                            }
                                                                                        ))
                                                                                )
                                                                        )
                                                                )
                                                        )
                                                )
                                        )
                                )
                                // Horizontal Menu Section
                                .child(
                                    VStack::new()
                                        .w_full()
                                        .gap(px(16.0))
                                        .child(
                                            div()
                                                .text_size(px(18.0))
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .child("Horizontal Navigation Menu"),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(14.0))
                                                .text_color(theme.tokens.muted_foreground)
                                                .child("Perfect for top navigation bars. Dropdowns appear below items when expanded."),
                                        )
                                        .child(
                                            VStack::new()
                                                .w_full()
                                                .gap(px(16.0))
                                                // Menu
                                                .child(
                                                    div()
                                                        .w_full()
                                                        .border_1()
                                                        .border_color(theme.tokens.border)
                                                        .rounded(theme.tokens.radius_md)
                                                        .bg(theme.tokens.card)
                                                        .p(px(8.0))
                                                        .child({
                                                            let selected = self.selected_horizontal_item.clone();
                                                            let expanded = self.expanded_horizontal_items.clone();
                                                            let app_entity = cx.entity().downgrade();

                                                            NavigationMenu::new()
                                                                .orientation(NavigationMenuOrientation::Horizontal)
                                                                .items(vec![
                                                                    NavigationMenuItem::new("dashboard".to_string(), "Dashboard")
                                                                        .with_icon("assets/icons/folder.svg"),
                                                                    NavigationMenuItem::new("projects".to_string(), "Projects")
                                                                        .with_icon("assets/icons/folder.svg")
                                                                        .with_children(vec![
                                                                            NavigationMenuItem::new("active".to_string(), "Active Projects"),
                                                                            NavigationMenuItem::new("archived".to_string(), "Archived"),
                                                                            NavigationMenuItem::new("templates".to_string(), "Templates"),
                                                                        ]),
                                                                    NavigationMenuItem::new("team".to_string(), "Team")
                                                                        .with_icon("assets/icons/folder.svg")
                                                                        .with_children(vec![
                                                                            NavigationMenuItem::new("members".to_string(), "Members"),
                                                                            NavigationMenuItem::new("roles".to_string(), "Roles"),
                                                                        ]),
                                                                    NavigationMenuItem::new("settings".to_string(), "Settings")
                                                                        .with_icon("assets/icons/file.svg"),
                                                                ])
                                                                .selected_id(selected.unwrap_or_default())
                                                                .expanded_ids(expanded)
                                                                .on_select({
                                                                    let app_entity = app_entity.clone();
                                                                    move |id, _window, cx| {
                                                                        if let Some(app) = app_entity.upgrade() {
                                                                            app.update(cx, |demo, cx| {
                                                                                demo.selected_horizontal_item = Some(id.clone());
                                                                                cx.notify();
                                                                            });
                                                                        }
                                                                    }
                                                                })
                                                                .on_toggle({
                                                                    let app_entity = app_entity.clone();
                                                                    move |id, is_expanded, _window, cx| {
                                                                        if let Some(app) = app_entity.upgrade() {
                                                                            app.update(cx, |demo, cx| {
                                                                                if is_expanded {
                                                                                    if !demo.expanded_horizontal_items.contains(id) {
                                                                                        demo.expanded_horizontal_items.push(id.clone());
                                                                                    }
                                                                                } else {
                                                                                    demo.expanded_horizontal_items.retain(|item_id| item_id != id);
                                                                                }
                                                                                cx.notify();
                                                                            });
                                                                        }
                                                                    }
                                                                })
                                                        })
                                                )
                                                // State display
                                                .child(
                                                    div()
                                                        .p(px(16.0))
                                                        .bg(theme.tokens.muted.opacity(0.3))
                                                        .rounded(theme.tokens.radius_md)
                                                        .border_1()
                                                        .border_color(theme.tokens.border)
                                                        .child(
                                                            VStack::new()
                                                                .gap(px(8.0))
                                                                .child(
                                                                    div()
                                                                        .text_size(px(14.0))
                                                                        .font_weight(FontWeight::SEMIBOLD)
                                                                        .child("Current State")
                                                                )
                                                                .child(
                                                                    div()
                                                                        .text_size(px(13.0))
                                                                        .child(format!(
                                                                            "Selected: {}",
                                                                            self.selected_horizontal_item.as_ref().map(|s| s.as_str()).unwrap_or("None")
                                                                        ))
                                                                )
                                                                .child(
                                                                    div()
                                                                        .text_size(px(13.0))
                                                                        .child(format!(
                                                                            "Expanded Items: {}",
                                                                            if self.expanded_horizontal_items.is_empty() {
                                                                                "None".to_string()
                                                                            } else {
                                                                                self.expanded_horizontal_items.join(", ")
                                                                            }
                                                                        ))
                                                                )
                                                        )
                                                )
                                        )
                                )
                                // Features Section
                                .child(
                                    VStack::new()
                                        .w_full()
                                        .gap(px(16.0))
                                        .p(px(24.0))
                                        .bg(theme.tokens.muted.opacity(0.5))
                                        .rounded(theme.tokens.radius_md)
                                        .border_1()
                                        .border_color(theme.tokens.border)
                                        .child(
                                            div()
                                                .text_size(px(16.0))
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .child("✨ Features"),
                                        )
                                        .child(
                                            VStack::new()
                                                .gap(px(8.0))
                                                .text_size(px(14.0))
                                                .child("Hierarchical Navigation:")
                                                .child("  • Unlimited nesting depth with recursive children")
                                                .child("  • Parent-managed state (selected_id, expanded_ids)")
                                                .child("  • Generic ID type support (String, u32, enums, etc.)")
                                                .child("  • Efficient HashSet lookups for O(1) expanded state checks")
                                                .child("")
                                                .child("Interactive States:")
                                                .child("  • Selected state with visual feedback")
                                                .child("  • Expand/collapse with chevron icons")
                                                .child("  • Hover effects on interactive elements")
                                                .child("  • Disabled state support")
                                                .child("")
                                                .child("Flexible Layout:")
                                                .child("  • Vertical orientation for sidebars")
                                                .child("  • Horizontal orientation for top navigation")
                                                .child("  • Automatic indentation for nested items")
                                                .child("  • Icon support for visual hierarchy")
                                                .child("")
                                                .child("Callbacks:")
                                                .child("  • on_select: Notifies parent when item is clicked")
                                                .child("  • on_toggle: Notifies parent when expand/collapse state changes")
                                                .child("  • Separate handlers allow independent state management"),
                                        ),
                                ),
                        ),
                    ),
            )
    }
}

fn main() {
    Application::new()
        .with_assets(Assets { base: PathBuf::from(env!("CARGO_MANIFEST_DIR")) })
        .run(move |cx: &mut App| {
        adabraka_ui::theme::install_theme(cx, adabraka_ui::theme::Theme::dark());
        adabraka_ui::init(cx);
        adabraka_ui::set_icon_base_path("assets/icons");

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(1200.0), px(900.0)),
                    cx,
                ))),
                titlebar: Some(TitlebarOptions {
                    title: Some("NavigationMenu Demo".into()),
                    appears_transparent: false,
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_window, cx| cx.new(|cx| NavigationMenuDemo::new(cx)),
        )
        .unwrap();
    });
}
