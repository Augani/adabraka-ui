use gpui::*;
use adabraka_ui::{
    prelude::*,
    navigation::tree::{TreeList, TreeNode},
    components::icon::IconSource,
    layout::VStack,
};
use std::path::{Path, PathBuf};
use std::fs;
use std::time::Instant;

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

/// Read a directory recursively and build tree nodes
/// Returns (nodes, total_count) where total_count includes all descendants
fn read_directory(path: &Path, max_depth: usize, current_depth: usize) -> Result<(Vec<TreeNode<String>>, usize)> {
    if current_depth >= max_depth {
        return Ok((vec![], 0));
    }

    let entries = match fs::read_dir(path) {
        Ok(entries) => entries,
        Err(_e) => {
            return Ok((vec![], 0));
        }
    };

    let mut nodes = Vec::new();
    let mut total_count = 0;

    // Collect and sort entries (directories first, then files, alphabetically)
    let mut entries_vec: Vec<_> = entries
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_name()
                .to_string_lossy()
                .chars()
                .next()
                .map(|c| c != '.')
                .unwrap_or(true)
        })
        .collect();

    entries_vec.sort_by(|a, b| {
        let a_is_dir = a.file_type().map(|t| t.is_dir()).unwrap_or(false);
        let b_is_dir = b.file_type().map(|t| t.is_dir()).unwrap_or(false);

        match (a_is_dir, b_is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.file_name().cmp(&b.file_name()),
        }
    });

    for entry in entries_vec {
        let path = entry.path();
        let file_name = entry.file_name().to_string_lossy().to_string();
        let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);

        let id = path.to_string_lossy().to_string();

        let (children, child_count) = if is_dir {
            read_directory(&path, max_depth, current_depth + 1).unwrap_or((vec![], 0))
        } else {
            (vec![], 0)
        };

        let icon = if is_dir {
            "src/icons/folder.svg"
        } else {
            "src/icons/file.svg"
        };

        let mut node = TreeNode::new(id, file_name).with_icon(icon);

        if !children.is_empty() {
            node = node.with_children(children);
        }

        total_count += 1 + child_count;
        nodes.push(node);
    }

    Ok((nodes, total_count))
}

struct TreePerformanceDemo {
    selected_node: Option<String>,
    expanded_nodes: Vec<String>,
    tree_nodes: Vec<TreeNode<String>>,
    total_nodes: usize,
    current_path: Option<PathBuf>,
    max_depth: usize,
    load_time_ms: u128,
    status_message: String,
}

impl TreePerformanceDemo {
    fn new() -> Self {
        Self {
            selected_node: None,
            expanded_nodes: vec![],
            tree_nodes: vec![],
            total_nodes: 0,
            current_path: None,
            max_depth: 10, // Read up to 10 levels deep
            load_time_ms: 0,
            status_message: "Select a folder to browse with virtual scrolling performance".to_string(),
        }
    }

    fn load_folder(&mut self, path: PathBuf) {
        let start = Instant::now();

        self.status_message = format!("Loading {:?}...", path);

        match read_directory(&path, self.max_depth, 0) {
            Ok((nodes, count)) => {
                self.tree_nodes = nodes;
                self.total_nodes = count;
                self.current_path = Some(path.clone());
                self.load_time_ms = start.elapsed().as_millis();
                self.status_message = format!(
                    "✅ Loaded {} items from {:?} in {}ms",
                    count,
                    path.file_name().unwrap_or_default(),
                    self.load_time_ms
                );
                self.expanded_nodes.clear();
                self.selected_node = None;
            }
            Err(e) => {
                self.status_message = format!("❌ Failed to load folder: {}", e);
            }
        }
    }

    fn expand_all(&mut self, nodes: &[TreeNode<String>]) {
        for node in nodes {
            if !node.children.is_empty() {
                self.expanded_nodes.push(node.id.clone());
                self.expand_all(&node.children);
            }
        }
    }

    fn collapse_all(&mut self) {
        self.expanded_nodes.clear();
    }
}

impl Render for TreePerformanceDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        VStack::new()
            .fill() // Fill available space
            .bg(theme.tokens.background)
            .text_color(theme.tokens.foreground)
            .child(
                VStack::new()
                    .fill_width() // Fill width
                    .padding(px(24.0)) // Use padding utility
                    .gap(px(16.0))
                    .border_b_1()
                    .border_color(theme.tokens.border)
                    .child(
                        div()
                            .text_size(px(24.0))
                            .font_weight(FontWeight::BOLD)
                            .child("Real File System Browser"),
                    )
                    .child(
                        div()
                            .flex()
                            .gap(px(32.0))
                            .text_size(px(14.0))
                            .child(format!("Total Items: {}", self.total_nodes))
                            .child(format!("Expanded: {}", self.expanded_nodes.len()))
                            .child(format!("Max Depth: {}", self.max_depth))
                            .children(if self.load_time_ms > 0 {
                                Some(format!("Load Time: {}ms", self.load_time_ms))
                            } else {
                                None
                            })
                    )
                    .child(
                        div()
                            .flex()
                            .gap(px(8.0))
                            .child({
                                let app_entity = cx.entity().downgrade();
                                Button::new("load-downloads-btn", "Load ~/Downloads")
                                    .on_click(move |_, _, cx| {
                                        if let Some(app) = app_entity.upgrade() {
                                            app.update(cx, |demo, cx| {
                                                if let Ok(home) = std::env::var("HOME") {
                                                    let downloads = PathBuf::from(home).join("Downloads");
                                                    demo.load_folder(downloads);
                                                } else {
                                                    demo.status_message = "❌ Could not find HOME env var".to_string();
                                                }
                                                cx.notify();
                                            });
                                        }
                                    })
                            })
                            .child({
                                let app_entity = cx.entity().downgrade();
                                Button::new("load-desktop-btn", "Load ~/Desktop")
                                    .on_click(move |_, _, cx| {
                                        if let Some(app) = app_entity.upgrade() {
                                            app.update(cx, |demo, cx| {
                                                if let Ok(home) = std::env::var("HOME") {
                                                    let desktop = PathBuf::from(home).join("Desktop");
                                                    demo.load_folder(desktop);
                                                } else {
                                                    demo.status_message = "❌ Could not find HOME env var".to_string();
                                                }
                                                cx.notify();
                                            });
                                        }
                                    })
                            })
                            .child({
                                let app_entity = cx.entity().downgrade();
                                Button::new("load-project-btn", "Load Project Folder")
                                    .on_click(move |_, _, cx| {
                                        if let Some(app) = app_entity.upgrade() {
                                            app.update(cx, |demo, cx| {
                                                // Load the current project directory
                                                let project = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                                                    .parent()
                                                    .and_then(|p| p.parent())
                                                    .map(|p| p.to_path_buf())
                                                    .unwrap_or_else(|| PathBuf::from("."));
                                                demo.load_folder(project);
                                                cx.notify();
                                            });
                                        }
                                    })
                            })
                            .child({
                                let app_entity = cx.entity().downgrade();
                                let tree_nodes_clone = self.tree_nodes.clone();
                                Button::new("expand-all-btn", "Expand All")
                                    .disabled(self.tree_nodes.is_empty())
                                    .on_click(move |_, _, cx| {
                                        if let Some(app) = app_entity.upgrade() {
                                            let start = Instant::now();
                                            app.update(cx, |demo, cx| {
                                                demo.expand_all(&tree_nodes_clone);
                                                let elapsed = start.elapsed().as_millis();
                                                demo.status_message = format!("✅ Expanded all in {}ms", elapsed);
                                                cx.notify();
                                            });
                                        }
                                    })
                            })
                            .child({
                                let app_entity = cx.entity().downgrade();
                                Button::new("collapse-all-btn", "Collapse All")
                                    .disabled(self.tree_nodes.is_empty())
                                    .on_click(move |_, _, cx| {
                                        if let Some(app) = app_entity.upgrade() {
                                            app.update(cx, |demo, cx| {
                                                demo.collapse_all();
                                                demo.status_message = "Collapsed all folders".to_string();
                                                cx.notify();
                                            });
                                        }
                                    })
                            })
                    )
                    .child(
                        div()
                            .p(px(12.0))
                            .bg(theme.tokens.muted.opacity(0.5))
                            .rounded(theme.tokens.radius_md)
                            .text_size(px(13.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child(self.status_message.clone())
                    ),
            )
            // Tree view - grows to fill remaining space
            .child({
                VStack::new()
                    .grow() // Take remaining vertical space
                    .fill_width() // Fill width
                    .overflow_hidden() // Hide overflow
                    .child(
                        if self.tree_nodes.is_empty() {
                            // Show placeholder when no folder loaded
                            div()
                                .size_full()
                                .flex()
                                .items_center()
                                .justify_center()
                                .child(
                                    VStack::new()
                                        .gap(px(16.0))
                                        .items_center()
                                        .child(
                                            div()
                                                .text_size(px(48.0))
                                                .child("📁")
                                        )
                                        .child(
                                            div()
                                                .text_size(px(18.0))
                                                .text_color(theme.tokens.muted_foreground)
                                                .child("No folder selected")
                                        )
                                        .child(
                                            div()
                                                .text_size(px(14.0))
                                                .text_color(theme.tokens.muted_foreground)
                                                .child("Click any folder button above to browse")
                                        )
                                )
                        } else {
                            let selected_node = self.selected_node.clone();
                            let expanded_nodes = self.expanded_nodes.clone();
                            let app_entity = cx.entity().downgrade();

                            div()
                                .size_full()
                                .child(
                                    TreeList::new()
                                        .nodes(self.tree_nodes.clone())
                                        .selected_id(selected_node.unwrap_or_default())
                                        .expanded_ids(expanded_nodes.clone())
                                        .on_select({
                                            let app_entity = app_entity.clone();
                                            move |id, _window, cx| {
                                                if let Some(app) = app_entity.upgrade() {
                                                    app.update(cx, |demo, cx| {
                                                        demo.selected_node = Some(id.clone());
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
                                                            if !demo.expanded_nodes.contains(id) {
                                                                demo.expanded_nodes.push(id.clone());
                                                            }
                                                        } else {
                                                            demo.expanded_nodes.retain(|node_id| node_id != id);
                                                        }
                                                        cx.notify();
                                                    });
                                                }
                                            }
                                        })
                                )
                        }
                    )
            })
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
                        size(px(1000.0), px(800.0)),
                        cx,
                    ))),
                    titlebar: Some(TitlebarOptions {
                        title: Some("Real File System Browser".into()),
                        appears_transparent: false,
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                |_window, cx| cx.new(|_cx| TreePerformanceDemo::new()),
            )
            .unwrap();
        });
}
