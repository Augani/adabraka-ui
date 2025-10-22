use gpui::*;
use gpui::prelude::FluentBuilder;
use adabraka_ui::{
    prelude::*,
    navigation::tree::{TreeList, TreeNode, List, ListItem},
    components::{
        scrollable::scrollable_vertical,
        input::{Input, InputState, InputVariant, InputSize, init as init_input},
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

struct TreeListDemo {
    // TreeList state
    selected_tree_node: Option<String>,
    expanded_tree_nodes: Vec<String>,
    tree_filter: String,
    tree_search_input: Entity<InputState>,
    auto_expand_matches: bool,

    // List state
    selected_list_item: Option<String>,
    list_filter: String,
    list_search_input: Entity<InputState>,
}

impl TreeListDemo {
    fn new(cx: &mut Context<TreeListDemo>) -> Self {
        Self {
            selected_tree_node: Some("home".to_string()),
            expanded_tree_nodes: vec![], // Start with no nodes expanded
            tree_filter: String::new(),
            tree_search_input: cx.new(|cx| InputState::new(cx)),
            auto_expand_matches: true,
            selected_list_item: Some("dashboard".to_string()),
            list_filter: String::new(),
            list_search_input: cx.new(|cx| InputState::new(cx)),
        }
    }
}

impl Render for TreeListDemo {
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
                                    .child("TreeList & List Component Demo"),
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Professional navigation components for hierarchical and flat lists"),
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
                                // TreeList Section
                                .child(
                                    VStack::new()
                                        .w_full()
                                        .gap(px(16.0))
                                        .child(
                                            div()
                                                .text_size(px(18.0))
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .child("TreeList - Hierarchical Navigation with Search"),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(14.0))
                                                .text_color(theme.tokens.muted_foreground)
                                                .child("Expandable tree structure with fast filtering. Perfect for file explorers, category navigation, and nested menus."),
                                        )
                                        // Search input for tree
                                        .child(
                                            div()
                                                .w_full()
                                                .max_w(px(400.0))
                                                .child({
                                                    let app_entity = cx.entity().downgrade();
                                                    let search_state = self.tree_search_input.clone();

                                                    VStack::new()
                                                        .gap(px(8.0))
                                                        .child(
                                                            Input::new(&search_state)
                                                                .placeholder("Search tree nodes...")
                                                                .variant(InputVariant::Default)
                                                                .size(InputSize::Md)
                                                                .clearable(true)
                                                                .prefix(
                                                                    div()
                                                                        .flex()
                                                                        .items_center()
                                                                        .child(
                                                                            svg()
                                                                                .path("assets/icons/search.svg")
                                                                                .size(px(16.0))
                                                                                .text_color(theme.tokens.muted_foreground)
                                                                        )
                                                                )
                                                                .on_change({
                                                                    let app_entity = app_entity.clone();
                                                                    move |value, cx| {
                                                                        if let Some(app) = app_entity.upgrade() {
                                                                            app.update(cx, |demo, cx| {
                                                                                demo.tree_filter = value.to_string();
                                                                                cx.notify();
                                                                            });
                                                                        }
                                                                    }
                                                                })
                                                        )
                                                        .child(
                                                            HStack::new()
                                                                .gap(px(8.0))
                                                                .child(
                                                                    div()
                                                                        .px(px(12.0))
                                                                        .py(px(6.0))
                                                                        .rounded(px(6.0))
                                                                        .bg(if self.auto_expand_matches {
                                                                            theme.tokens.accent.opacity(0.2)
                                                                        } else {
                                                                            theme.tokens.muted.opacity(0.2)
                                                                        })
                                                                        .border_1()
                                                                        .border_color(if self.auto_expand_matches {
                                                                            theme.tokens.accent.opacity(0.3)
                                                                        } else {
                                                                            theme.tokens.border
                                                                        })
                                                                        .cursor(CursorStyle::PointingHand)
                                                                        .on_mouse_down(MouseButton::Left, {
                                                                            let app_entity = app_entity.clone();
                                                                            move |_, _window, cx| {
                                                                                if let Some(app) = app_entity.upgrade() {
                                                                                    app.update(cx, |demo, cx| {
                                                                                        demo.auto_expand_matches = !demo.auto_expand_matches;
                                                                                        cx.notify();
                                                                                    });
                                                                                }
                                                                            }
                                                                        })
                                                                        .hover(|mut style| {
                                                                            style.opacity = Some(0.8);
                                                                            style
                                                                        })
                                                                        .child(
                                                                            div()
                                                                                .flex()
                                                                                .items_center()
                                                                                .gap(px(6.0))
                                                                                .text_size(px(12.0))
                                                                                .text_color(if self.auto_expand_matches {
                                                                                    theme.tokens.accent
                                                                                } else {
                                                                                    theme.tokens.muted_foreground
                                                                                })
                                                                                .font_weight(FontWeight::MEDIUM)
                                                                                .child("Auto-expand matches")
                                                                        )
                                                                )
                                                                .when(!self.tree_filter.is_empty(), |stack| {
                                                                    stack.child(
                                                                        div()
                                                                            .px(px(8.0))
                                                                            .py(px(4.0))
                                                                            .rounded(px(4.0))
                                                                            .bg(theme.tokens.muted.opacity(0.3))
                                                                            .text_size(px(11.0))
                                                                            .text_color(theme.tokens.muted_foreground)
                                                                            .child(format!("Filtering: '{}'", self.tree_filter))
                                                                    )
                                                                })
                                                        )
                                                })
                                        )
                                        .child(
                                            div()
                                                .w_full()
                                                .max_w(px(400.0))
                                                .border_1()
                                                .border_color(theme.tokens.border)
                                                .rounded(theme.tokens.radius_md)
                                                .overflow_hidden()
                .child({
                    let selected_node = self.selected_tree_node.clone();
                    let expanded_nodes = self.expanded_tree_nodes.clone();
                    let app_entity = cx.entity().downgrade();

                    TreeList::new()
                        .nodes(vec![
                                                            TreeNode::new("home".to_string(), "Home")
                                                                .with_icon("assets/icons/folder.svg")
                                                                .with_children(vec![
                                                                    TreeNode::new("favorites".to_string(), "Favorites")
                                                                        .with_icon("assets/icons/folder.svg"),
                                                                    TreeNode::new("recent".to_string(), "Recent Files")
                                                                        .with_icon("assets/icons/folder.svg"),
                                                                ]),
                                                            TreeNode::new("documents".to_string(), "Documents")
                                                                .with_icon("assets/icons/folder.svg")
                                                                .with_children(vec![
                                                                    TreeNode::new("work".to_string(), "Work")
                                                                        .with_icon("assets/icons/folder.svg")
                                                                        .with_children(vec![
                                                                            TreeNode::new("projects".to_string(), "Projects")
                                                                                .with_icon("assets/icons/file.svg"),
                                                                            TreeNode::new("reports".to_string(), "Reports")
                                                                                .with_icon("assets/icons/file.svg"),
                                                                        ]),
                                                                    TreeNode::new("personal".to_string(), "Personal")
                                                                        .with_icon("assets/icons/file.svg"),
                                                                ]),
                                                            TreeNode::new("downloads".to_string(), "Downloads")
                                                                .with_icon("assets/icons/folder.svg")
                                                                .with_children(vec![
                                                                    TreeNode::new("images".to_string(), "Images")
                                                                        .with_icon("assets/icons/folder.svg"),
                                                                    TreeNode::new("videos".to_string(), "Videos")
                                                                        .with_icon("assets/icons/file.svg"),
                                                                ]),
                                                            TreeNode::new("settings".to_string(), "Settings")
                                                                .with_icon("assets/icons/file.svg"),
                                                            TreeNode::new("disabled".to_string(), "Disabled Item")
                                                                .with_icon("assets/icons/file.svg")
                                                                .disabled(true),
                        ])
                        .selected_id(selected_node.unwrap_or_default())
                        .expanded_ids(expanded_nodes.clone())
                        .filter(&self.tree_filter)
                        .auto_expand_matches(self.auto_expand_matches)
                        .highlight_matches(true)
                                                        .on_select({
                                                            let app_entity = app_entity.clone();
                                                            move |id, _window, cx| {
                                                                if let Some(app) = app_entity.upgrade() {
                                                                    app.update(cx, |demo, cx| {
                                                                        demo.selected_tree_node = Some(id.clone());
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
                                                                            if !demo.expanded_tree_nodes.contains(id) {
                                                                                demo.expanded_tree_nodes.push(id.clone());
                                                                            }
                                                                        } else {
                                                                            demo.expanded_tree_nodes.retain(|node_id| node_id != id);
                                                                        }
                                                                        cx.notify();
                                                                    });
                                                                }
                                                            }
                                                        })
                                                })
                                        )
                                        .child(
                                            div()
                                                .p(px(16.0))
                                                .bg(theme.tokens.muted.opacity(0.3))
                                                .rounded(theme.tokens.radius_md)
                                                .border_1()
                                                .border_color(theme.tokens.border)
                                                .text_size(px(13.0))
                                                .child(format!(
                                                    "Selected: {} | Expanded: {} | Filter: {}",
                                                    self.selected_tree_node.as_ref().map(|s| s.as_str()).unwrap_or("None"),
                                                    self.expanded_tree_nodes.len(),
                                                    if self.tree_filter.is_empty() { "None" } else { &self.tree_filter }
                                                ))
                                        ),
                                )
                                // List Section
                                .child(
                                    VStack::new()
                                        .w_full()
                                        .gap(px(16.0))
                                        .child(
                                            div()
                                                .text_size(px(18.0))
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .child("List - Flat Navigation"),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(14.0))
                                                .text_color(theme.tokens.muted_foreground)
                                                .child("Simple flat list perfect for sidebar navigation, menus, and settings panels."),
                                        )
                                        .child(
                                            div()
                                                .w_full()
                                                .max_w(px(400.0))
                                                .border_1()
                                                .border_color(theme.tokens.border)
                                                .rounded(theme.tokens.radius_md)
                                                .overflow_hidden()
                                                .child({
                                                    let selected_item = self.selected_list_item.clone();
                                                    let app_entity = cx.entity().downgrade();

                                                    List::new()
                                                        .items(vec![
                                                            ListItem::new("dashboard".to_string(), "Dashboard")
                                                                .with_icon("assets/icons/folder.svg"),
                                                            ListItem::new("analytics".to_string(), "Analytics")
                                                                .with_icon("assets/icons/file.svg")
                                                                .with_badge("New"),
                                                            ListItem::new("projects".to_string(), "Projects")
                                                                .with_icon("assets/icons/file.svg")
                                                                .with_badge("12"),
                                                            ListItem::new("team".to_string(), "Team")
                                                                .with_icon("assets/icons/file.svg"),
                                                            ListItem::new("settings".to_string(), "Settings")
                                                                .with_icon("assets/icons/file.svg"),
                                                            ListItem::new("notifications".to_string(), "Notifications")
                                                                .with_icon("assets/icons/file.svg")
                                                                .with_badge("5"),
                                                            ListItem::new("help".to_string(), "Help & Support")
                                                                .with_icon("assets/icons/file.svg"),
                                                            ListItem::new("disabled".to_string(), "Disabled Item")
                                                                .with_icon("assets/icons/file.svg")
                                                                .disabled(true),
                                                        ])
                                                        .selected_id(selected_item.unwrap_or_default())
                                                        .on_select(move |id, _window, cx| {
                                                            if let Some(app) = app_entity.upgrade() {
                                                                app.update(cx, |demo, cx| {
                                                                    demo.selected_list_item = Some(id.clone());
                                                                    cx.notify();
                                                                });
                                                            }
                                                        })
                                                })
                                        )
                                        .child(
                                            div()
                                                .p(px(16.0))
                                                .bg(theme.tokens.muted.opacity(0.3))
                                                .rounded(theme.tokens.radius_md)
                                                .border_1()
                                                .border_color(theme.tokens.border)
                                                .text_size(px(13.0))
                                                .child(format!(
                                                    "Selected: {}",
                                                    self.selected_list_item.as_ref().map(|s| s.as_str()).unwrap_or("None")
                                                ))
                                        ),
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
                                                .child("TreeList with Filtering:")
                                                .child("  • Fast search with fuzzy matching")
                                                .child("  • Auto-expand to show matches")
                                                .child("  • Highlighted search results")
                                                .child("  • Parent visibility preserved")
                                                .child("  • Expandable/collapsible hierarchical structure")
                                                .child("  • Icon support for each node")
                                                .child("  • Proper indentation for nested levels")
                                                .child("  • Selection and disabled states")
                                                .child("  • Smooth hover effects")
                                                .child("")
                                                .child("List:")
                                                .child("  • Simple flat navigation")
                                                .child("  • Icon and badge support")
                                                .child("  • Selection state with visual feedback")
                                                .child("  • Disabled items")
                                                .child("  • Perfect for sidebars and menus"),
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
        init_input(cx);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(1200.0), px(900.0)),
                    cx,
                ))),
                titlebar: Some(TitlebarOptions {
                    title: Some("TreeList & List Demo".into()),
                    appears_transparent: false,
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_window, cx| cx.new(|cx| TreeListDemo::new(cx)),
        )
        .unwrap();
    });
}
