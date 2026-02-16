use crate::components::icon::Icon;
use crate::components::icon_source::IconSource;
use crate::theme::use_theme;
use gpui::{prelude::FluentBuilder as _, *};
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FileNodeKind {
    File,
    Directory,
    Symlink,
}

#[derive(Clone, Debug)]
pub struct FileNode {
    pub path: PathBuf,
    pub name: String,
    pub kind: FileNodeKind,
    pub children: Vec<FileNode>,
    pub size: Option<u64>,
    pub modified: Option<String>,
    pub is_hidden: bool,
    pub has_unloaded_children: bool,
}

impl FileNode {
    pub fn file(path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        Self {
            path,
            name,
            kind: FileNodeKind::File,
            children: Vec::new(),
            size: None,
            modified: None,
            is_hidden: false,
            has_unloaded_children: false,
        }
    }

    pub fn directory(path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        Self {
            path,
            name,
            kind: FileNodeKind::Directory,
            children: Vec::new(),
            size: None,
            modified: None,
            is_hidden: false,
            has_unloaded_children: false,
        }
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn with_children(mut self, children: Vec<FileNode>) -> Self {
        self.children = children;
        self
    }

    pub fn with_size(mut self, size: u64) -> Self {
        self.size = Some(size);
        self
    }

    pub fn with_modified(mut self, modified: impl Into<String>) -> Self {
        self.modified = Some(modified.into());
        self
    }

    pub fn hidden(mut self, is_hidden: bool) -> Self {
        self.is_hidden = is_hidden;
        self
    }

    pub fn with_unloaded_children(mut self, has_unloaded: bool) -> Self {
        self.has_unloaded_children = has_unloaded;
        self
    }

    pub fn is_directory(&self) -> bool {
        self.kind == FileNodeKind::Directory
    }

    pub fn extension(&self) -> Option<&str> {
        self.path.extension().and_then(|e| e.to_str())
    }

    pub fn file_icon(&self, is_expanded: bool) -> IconSource {
        match self.kind {
            FileNodeKind::Directory => {
                if is_expanded {
                    IconSource::Named("folder-open".into())
                } else {
                    IconSource::Named("folder".into())
                }
            }
            FileNodeKind::Symlink => IconSource::Named("link".into()),
            FileNodeKind::File => match self.extension() {
                Some("json") | Some("yaml") | Some("yml") | Some("toml") | Some("xml") => {
                    IconSource::Named("file-json".into())
                }
                Some("md") | Some("txt") | Some("doc") | Some("docx") | Some("pdf") => {
                    IconSource::Named("file-text".into())
                }
                Some("sh") | Some("bash") | Some("zsh") => IconSource::Named("hash".into()),
                Some("png") | Some("jpg") | Some("jpeg") | Some("gif") | Some("svg")
                | Some("ico") | Some("webp") => IconSource::Named("image".into()),
                Some("mp3") | Some("wav") | Some("ogg") | Some("flac") => {
                    IconSource::Named("music".into())
                }
                Some("mp4") | Some("mov") | Some("avi") | Some("webm") => {
                    IconSource::Named("video".into())
                }
                Some("zip") | Some("tar") | Some("gz") | Some("rar") | Some("7z") => {
                    IconSource::Named("archive".into())
                }
                _ => IconSource::Named("file-code".into()),
            },
        }
    }

    pub fn file_icon_color(&self, theme: &crate::theme::Theme) -> Hsla {
        match self.kind {
            FileNodeKind::Directory => rgb(0x60a5fa).into(),
            FileNodeKind::Symlink => theme.tokens.muted_foreground,
            FileNodeKind::File => match self.extension() {
                Some("json") | Some("yaml") | Some("yml") | Some("toml") | Some("xml") => {
                    rgb(0xfbbf24).into()
                }
                Some("md") | Some("txt") | Some("doc") | Some("docx") | Some("pdf") => {
                    rgb(0xa78bfa).into()
                }
                Some("sh") | Some("bash") | Some("zsh") => rgb(0x4ade80).into(),
                Some("png") | Some("jpg") | Some("jpeg") | Some("gif") | Some("svg")
                | Some("ico") | Some("webp") => rgb(0x22c55e).into(),
                Some("mp3") | Some("wav") | Some("ogg") | Some("flac") => rgb(0xf472b6).into(),
                Some("mp4") | Some("mov") | Some("avi") | Some("webm") => rgb(0xf472b6).into(),
                Some("zip") | Some("tar") | Some("gz") | Some("rar") | Some("7z") => {
                    rgb(0xfbbf24).into()
                }
                _ => rgb(0x9ca3af).into(),
            },
        }
    }
}

#[derive(Clone)]
struct FlatFileNode {
    node: FileNode,
    level: usize,
}

fn sort_file_nodes(nodes: &mut Vec<FileNode>) {
    nodes.sort_by(|a, b| match (a.is_directory(), b.is_directory()) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
    });
    for node in nodes.iter_mut() {
        if !node.children.is_empty() {
            sort_file_nodes(&mut node.children);
        }
    }
}

fn flatten_file_tree(
    nodes: &[FileNode],
    expanded_paths: &HashSet<PathBuf>,
    level: usize,
    show_hidden: bool,
) -> Vec<FlatFileNode> {
    let mut flat = Vec::new();

    for node in nodes {
        if node.is_hidden && !show_hidden {
            continue;
        }

        flat.push(FlatFileNode {
            node: node.clone(),
            level,
        });

        let has_children = !node.children.is_empty() || node.has_unloaded_children;
        if has_children && expanded_paths.contains(&node.path) {
            let children =
                flatten_file_tree(&node.children, expanded_paths, level + 1, show_hidden);
            flat.extend(children);
        }
    }

    flat
}

fn format_size(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if size >= GB {
        format!("{:.1} GB", size as f64 / GB as f64)
    } else if size >= MB {
        format!("{:.1} MB", size as f64 / MB as f64)
    } else if size >= KB {
        format!("{:.1} KB", size as f64 / KB as f64)
    } else {
        format!("{} B", size)
    }
}

const ROW_HEIGHT: f32 = 28.0;

#[derive(IntoElement)]
pub struct FileTree {
    nodes: Vec<FileNode>,
    selected_path: Option<PathBuf>,
    expanded_paths: Vec<PathBuf>,
    show_hidden: bool,
    show_file_size: bool,
    on_select: Option<Arc<dyn Fn(&PathBuf, &mut Window, &mut App) + Send + Sync>>,
    on_open: Option<Arc<dyn Fn(&PathBuf, &mut Window, &mut App) + Send + Sync>>,
    on_toggle: Option<Arc<dyn Fn(&PathBuf, bool, &mut Window, &mut App) + Send + Sync>>,
    on_context_menu:
        Option<Arc<dyn Fn(&PathBuf, Point<Pixels>, &mut Window, &mut App) + Send + Sync>>,
    style: StyleRefinement,
}

impl FileTree {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            selected_path: None,
            expanded_paths: Vec::new(),
            show_hidden: false,
            show_file_size: false,
            on_select: None,
            on_open: None,
            on_toggle: None,
            on_context_menu: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn nodes(mut self, mut nodes: Vec<FileNode>) -> Self {
        sort_file_nodes(&mut nodes);
        self.nodes = nodes;
        self
    }

    pub fn selected_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.selected_path = Some(path.into());
        self
    }

    pub fn expanded_paths(mut self, paths: Vec<PathBuf>) -> Self {
        self.expanded_paths = paths;
        self
    }

    pub fn show_hidden(mut self, show: bool) -> Self {
        self.show_hidden = show;
        self
    }

    pub fn show_file_size(mut self, show: bool) -> Self {
        self.show_file_size = show;
        self
    }

    pub fn on_select<F>(mut self, handler: F) -> Self
    where
        F: Fn(&PathBuf, &mut Window, &mut App) + Send + Sync + 'static,
    {
        self.on_select = Some(Arc::new(handler));
        self
    }

    pub fn on_open<F>(mut self, handler: F) -> Self
    where
        F: Fn(&PathBuf, &mut Window, &mut App) + Send + Sync + 'static,
    {
        self.on_open = Some(Arc::new(handler));
        self
    }

    pub fn on_toggle<F>(mut self, handler: F) -> Self
    where
        F: Fn(&PathBuf, bool, &mut Window, &mut App) + Send + Sync + 'static,
    {
        self.on_toggle = Some(Arc::new(handler));
        self
    }

    pub fn on_context_menu<F>(mut self, handler: F) -> Self
    where
        F: Fn(&PathBuf, Point<Pixels>, &mut Window, &mut App) + Send + Sync + 'static,
    {
        self.on_context_menu = Some(Arc::new(handler));
        self
    }
}

impl Default for FileTree {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for FileTree {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for FileTree {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let user_style = self.style;

        let expanded_set: HashSet<PathBuf> = self.expanded_paths.into_iter().collect();
        let flat_nodes = flatten_file_tree(&self.nodes, &expanded_set, 0, self.show_hidden);

        let selected_path = self.selected_path;
        let on_select = self.on_select;
        let on_open = self.on_open;
        let on_toggle = self.on_toggle;
        let on_context_menu = self.on_context_menu;
        let show_file_size = self.show_file_size;

        div()
            .flex()
            .flex_col()
            .w_full()
            .bg(gpui::transparent_black())
            .map(|mut this| {
                this.style().refine(&user_style);
                this
            })
            .children(flat_nodes.into_iter().map(|flat_node| {
                let is_selected = selected_path.as_ref() == Some(&flat_node.node.path);
                let is_expanded = expanded_set.contains(&flat_node.node.path);
                let has_children =
                    !flat_node.node.children.is_empty() || flat_node.node.has_unloaded_children;
                let indent = px((flat_node.level as f32) * 16.0);
                let node = flat_node.node;
                let path = node.path.clone();

                let icon_color = node.file_icon_color(&theme);
                let node_icon = node.file_icon(is_expanded);

                div()
                    .id(SharedString::from(path.to_string_lossy().to_string()))
                    .w_full()
                    .h(px(ROW_HEIGHT))
                    .flex()
                    .items_center()
                    .mx(px(8.0))
                    .px(px(8.0))
                    .pl(indent + px(8.0))
                    .rounded(px(8.0))
                    .cursor_pointer()
                    .bg(if is_selected {
                        theme.tokens.accent
                    } else {
                        gpui::transparent_black()
                    })
                    .text_color(if is_selected {
                        theme.tokens.accent_foreground
                    } else if node.is_hidden {
                        theme.tokens.muted_foreground
                    } else {
                        theme.tokens.foreground
                    })
                    .when(!is_selected, |d| {
                        d.hover(|s| s.bg(theme.tokens.accent.opacity(0.5)))
                    })
                    .on_click({
                        let path = path.clone();
                        let on_select = on_select.clone();
                        let on_toggle = on_toggle.clone();
                        let on_open = on_open.clone();
                        let is_dir = node.is_directory();

                        move |event, window, cx| {
                            if let Some(ref handler) = on_select {
                                handler(&path, window, cx);
                            }

                            if is_dir {
                                if let Some(ref handler) = on_toggle {
                                    handler(&path, !is_expanded, window, cx);
                                }
                            } else if event.click_count() == 2 {
                                if let Some(ref handler) = on_open {
                                    handler(&path, window, cx);
                                }
                            }
                        }
                    })
                    .on_mouse_down(MouseButton::Right, {
                        let path = path.clone();
                        let on_context_menu = on_context_menu.clone();

                        move |event, window, cx| {
                            if let Some(ref handler) = on_context_menu {
                                handler(&path, event.position, window, cx);
                            }
                        }
                    })
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(6.0))
                            .flex_1()
                            .child(
                                div()
                                    .w(px(16.0))
                                    .h(px(16.0))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .when(has_children, |d| {
                                        d.child(
                                            Icon::new(if is_expanded {
                                                "chevron-down"
                                            } else {
                                                "chevron-right"
                                            })
                                            .size(px(12.0))
                                            .color(theme.tokens.muted_foreground),
                                        )
                                    }),
                            )
                            .child(Icon::new(node_icon).size(px(16.0)).color(if is_selected {
                                theme.tokens.accent_foreground
                            } else {
                                icon_color
                            }))
                            .child(
                                div()
                                    .flex_1()
                                    .text_size(px(13.0))
                                    .font_family(theme.tokens.font_family.clone())
                                    .when(node.is_hidden, |d| d.opacity(0.6))
                                    .child(node.name.clone()),
                            )
                            .when(
                                show_file_size && node.size.is_some() && !node.is_directory(),
                                |d| {
                                    d.child(
                                        div()
                                            .text_size(px(11.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child(format_size(node.size.unwrap())),
                                    )
                                },
                            ),
                    )
            }))
    }
}
