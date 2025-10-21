use adabraka_ui::navigation::tree::{TreeNode, TreeList};
use gpui::{prelude::*, *};
use adabraka_ui::theme::{Theme, setup_theme};

// Define a simple ID type for tree nodes
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct NodeId(String);

impl From<&str> for NodeId {
    fn from(s: &str) -> Self {
        NodeId(s.to_string())
    }
}

// Main window component
struct TreeFilterDemo {
    filter: String,
    expanded_ids: Vec<NodeId>,
    selected_id: Option<NodeId>,
}

impl TreeFilterDemo {
    fn new() -> Self {
        Self {
            filter: String::new(),
            expanded_ids: vec![
                NodeId::from("src"),
                NodeId::from("components"),
                NodeId::from("documents"),
            ],
            selected_id: Some(NodeId::from("app.rs")),
        }
    }

    fn build_sample_tree() -> Vec<TreeNode<NodeId>> {
        vec![
            TreeNode::new(NodeId::from("src"), "src")
                .with_icon("folder")
                .with_children(vec![
                    TreeNode::new(NodeId::from("components"), "components")
                        .with_icon("folder")
                        .with_children(vec![
                            TreeNode::new(NodeId::from("button.rs"), "button.rs")
                                .with_icon("file"),
                            TreeNode::new(NodeId::from("input.rs"), "input.rs")
                                .with_icon("file"),
                            TreeNode::new(NodeId::from("modal.rs"), "modal.rs")
                                .with_icon("file"),
                            TreeNode::new(NodeId::from("dropdown.rs"), "dropdown.rs")
                                .with_icon("file"),
                        ]),
                    TreeNode::new(NodeId::from("views"), "views")
                        .with_icon("folder")
                        .with_children(vec![
                            TreeNode::new(NodeId::from("home.rs"), "home.rs")
                                .with_icon("file"),
                            TreeNode::new(NodeId::from("dashboard.rs"), "dashboard.rs")
                                .with_icon("file"),
                            TreeNode::new(NodeId::from("settings.rs"), "settings.rs")
                                .with_icon("file"),
                        ]),
                    TreeNode::new(NodeId::from("app.rs"), "app.rs")
                        .with_icon("file"),
                    TreeNode::new(NodeId::from("main.rs"), "main.rs")
                        .with_icon("file"),
                ]),
            TreeNode::new(NodeId::from("tests"), "tests")
                .with_icon("folder")
                .with_children(vec![
                    TreeNode::new(NodeId::from("unit"), "unit")
                        .with_icon("folder")
                        .with_children(vec![
                            TreeNode::new(NodeId::from("test_button.rs"), "test_button.rs")
                                .with_icon("file"),
                            TreeNode::new(NodeId::from("test_input.rs"), "test_input.rs")
                                .with_icon("file"),
                        ]),
                    TreeNode::new(NodeId::from("integration"), "integration")
                        .with_icon("folder")
                        .with_children(vec![
                            TreeNode::new(NodeId::from("test_app.rs"), "test_app.rs")
                                .with_icon("file"),
                        ]),
                ]),
            TreeNode::new(NodeId::from("documents"), "documents")
                .with_icon("folder")
                .with_children(vec![
                    TreeNode::new(NodeId::from("readme.md"), "README.md")
                        .with_icon("file"),
                    TreeNode::new(NodeId::from("contributing.md"), "CONTRIBUTING.md")
                        .with_icon("file"),
                    TreeNode::new(NodeId::from("changelog.md"), "CHANGELOG.md")
                        .with_icon("file"),
                    TreeNode::new(NodeId::from("api"), "api")
                        .with_icon("folder")
                        .with_children(vec![
                            TreeNode::new(NodeId::from("api_reference.md"), "api_reference.md")
                                .with_icon("file"),
                            TreeNode::new(NodeId::from("examples.md"), "examples.md")
                                .with_icon("file"),
                        ]),
                ]),
            TreeNode::new(NodeId::from("config"), "config")
                .with_icon("folder")
                .with_children(vec![
                    TreeNode::new(NodeId::from("settings.json"), "settings.json")
                        .with_icon("file"),
                    TreeNode::new(NodeId::from("database.yml"), "database.yml")
                        .with_icon("file"),
                ]),
        ]
    }
}

impl Render for TreeFilterDemo {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>().clone();

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(theme.tokens.background)
            .child(
                // Filter input at the top
                div()
                    .flex()
                    .items_center()
                    .px(px(16.0))
                    .py(px(12.0))
                    .bg(theme.tokens.card)
                    .border_b()
                    .border_color(theme.tokens.border)
                    .child(
                        div()
                            .flex_1()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .px(px(12.0))
                                    .py(px(8.0))
                                    .rounded(px(8.0))
                                    .bg(theme.tokens.input)
                                    .border()
                                    .border_color(theme.tokens.border)
                                    .child(
                                        // Search icon
                                        svg()
                                            .path("src/icons/regular/search.svg")
                                            .size(px(16.0))
                                            .mr(px(8.0))
                                            .text_color(theme.tokens.muted_foreground)
                                    )
                                    .child(
                                        div()
                                            .flex_1()
                                            .child(
                                                text_input()
                                                    .id("filter-input")
                                                    .placeholder("Filter tree nodes...")
                                                    .value(self.filter.clone())
                                                    .on_input(cx.listener(|this, value: &String, _cx| {
                                                        this.filter = value.clone();
                                                    }))
                                                    .text_size(px(14.0))
                                                    .text_color(theme.tokens.foreground)
                                                    .placeholder_text_color(theme.tokens.muted_foreground)
                                                    .bg(transparent_black())
                                                    .border_0()
                                            )
                                    )
                                    .when(!self.filter.is_empty(), |div| {
                                        div.child(
                                            // Clear button
                                            div()
                                                .ml(px(8.0))
                                                .cursor(CursorStyle::PointingHand)
                                                .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, cx| {
                                                    this.filter.clear();
                                                    cx.notify();
                                                }))
                                                .hover(|mut style| {
                                                    style.opacity = Some(0.7);
                                                    style
                                                })
                                                .child(
                                                    svg()
                                                        .path("src/icons/regular/x.svg")
                                                        .size(px(16.0))
                                                        .text_color(theme.tokens.muted_foreground)
                                                )
                                        )
                                    })
                            )
                    )
                    .child(
                        // Auto-expand toggle
                        div()
                            .ml(px(12.0))
                            .px(px(12.0))
                            .py(px(6.0))
                            .rounded(px(6.0))
                            .bg(theme.tokens.accent.opacity(0.1))
                            .border()
                            .border_color(theme.tokens.accent.opacity(0.2))
                            .cursor(CursorStyle::PointingHand)
                            .hover(|mut style| {
                                style.background = Some(theme.tokens.accent.opacity(0.2).into());
                                style
                            })
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(6.0))
                                    .text_size(px(12.0))
                                    .text_color(theme.tokens.accent)
                                    .font_weight(FontWeight::MEDIUM)
                                    .child("Auto-expand")
                            )
                    )
            )
            .child(
                // Tree component with filter
                div()
                    .flex_1()
                    .overflow_hidden()
                    .child(
                        TreeList::new()
                            .nodes(Self::build_sample_tree())
                            .selected_id(self.selected_id.clone())
                            .expanded_ids(self.expanded_ids.clone())
                            .filter(&self.filter)
                            .auto_expand_matches(true)
                            .highlight_matches(true)
                            .on_select(cx.listener(|this, id: &NodeId, cx| {
                                this.selected_id = Some(id.clone());
                                cx.notify();
                                println!("Selected: {:?}", id);
                            }))
                            .on_toggle(cx.listener(|this, id: &NodeId, expanded: bool, cx| {
                                if expanded {
                                    if !this.expanded_ids.contains(id) {
                                        this.expanded_ids.push(id.clone());
                                    }
                                } else {
                                    this.expanded_ids.retain(|i| i != id);
                                }
                                cx.notify();
                            }))
                    )
            )
            .child(
                // Status bar showing selected item
                div()
                    .flex()
                    .items_center()
                    .px(px(16.0))
                    .py(px(8.0))
                    .bg(theme.tokens.card)
                    .border_t()
                    .border_color(theme.tokens.border)
                    .text_size(px(12.0))
                    .text_color(theme.tokens.muted_foreground)
                    .child(
                        if let Some(ref selected) = self.selected_id {
                            format!("Selected: {}", selected.0)
                        } else {
                            "No selection".to_string()
                        }
                    )
                    .child(
                        div()
                            .flex_1()
                            .text_right()
                            .child(
                                if !self.filter.is_empty() {
                                    format!("Filter: '{}'", self.filter)
                                } else {
                                    "No filter applied".to_string()
                                }
                            )
                    )
            )
    }
}

fn main() {
    App::new().run(move |cx: &mut AppContext| {
        setup_theme(cx);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    size: Size {
                        width: px(600.0),
                        height: px(700.0),
                    },
                    origin: Point::default(),
                })),
                titlebar: Some(TitlebarOptions {
                    title: Some("Tree with Filter Demo".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            move |cx| cx.new_view(|_cx| TreeFilterDemo::new()),
        );
    });
}