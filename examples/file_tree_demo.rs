use adabraka_ui::{
    layout::VStack,
    navigation::file_tree::{FileNode, FileTree},
    theme::{install_theme, use_theme, Theme},
};
use gpui::{prelude::FluentBuilder as _, *};
use std::path::PathBuf;

struct FileTreeDemo {
    selected_path: Option<PathBuf>,
    expanded_paths: Vec<PathBuf>,
    show_hidden: bool,
}

impl FileTreeDemo {
    fn new() -> Self {
        Self {
            selected_path: None,
            expanded_paths: vec![PathBuf::from("/project"), PathBuf::from("/project/src")],
            show_hidden: false,
        }
    }

    fn sample_tree() -> Vec<FileNode> {
        vec![FileNode::directory("/project")
            .with_name("my-project")
            .with_children(vec![
                FileNode::directory("/project/src")
                    .with_name("src")
                    .with_children(vec![
                        FileNode::file("/project/src/main.rs")
                            .with_name("main.rs")
                            .with_size(1024),
                        FileNode::file("/project/src/lib.rs")
                            .with_name("lib.rs")
                            .with_size(2048),
                        FileNode::directory("/project/src/components")
                            .with_name("components")
                            .with_children(vec![
                                FileNode::file("/project/src/components/button.rs")
                                    .with_name("button.rs")
                                    .with_size(3456),
                                FileNode::file("/project/src/components/input.rs")
                                    .with_name("input.rs")
                                    .with_size(4567),
                                FileNode::file("/project/src/components/mod.rs")
                                    .with_name("mod.rs")
                                    .with_size(256),
                            ]),
                        FileNode::directory("/project/src/utils")
                            .with_name("utils")
                            .with_children(vec![FileNode::file("/project/src/utils/helpers.rs")
                                .with_name("helpers.rs")
                                .with_size(1234)]),
                    ]),
                FileNode::directory("/project/tests")
                    .with_name("tests")
                    .with_children(vec![FileNode::file("/project/tests/integration.rs")
                        .with_name("integration.rs")
                        .with_size(5678)]),
                FileNode::directory("/project/assets")
                    .with_name("assets")
                    .with_children(vec![
                        FileNode::file("/project/assets/logo.png")
                            .with_name("logo.png")
                            .with_size(45678),
                        FileNode::file("/project/assets/style.css")
                            .with_name("style.css")
                            .with_size(2345),
                    ]),
                FileNode::file("/project/Cargo.toml")
                    .with_name("Cargo.toml")
                    .with_size(512),
                FileNode::file("/project/README.md")
                    .with_name("README.md")
                    .with_size(4096),
                FileNode::file("/project/.gitignore")
                    .with_name(".gitignore")
                    .with_size(128)
                    .hidden(true),
                FileNode::file("/project/.env")
                    .with_name(".env")
                    .with_size(64)
                    .hidden(true),
            ])]
    }
}

impl Render for FileTreeDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();
        let entity = cx.entity().clone();

        div()
            .size_full()
            .bg(theme.tokens.background)
            .text_color(theme.tokens.foreground)
            .font_family(theme.tokens.font_family.clone())
            .child(
                VStack::new()
                    .p(px(24.0))
                    .gap(px(24.0))
                    .size_full()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .text_size(px(32.0))
                                    .font_weight(FontWeight::BOLD)
                                    .child("File Tree Demo"),
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("File browser component with icons, expand/collapse, and selection"),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .gap(px(16.0))
                            .child(
                                div()
                                    .id("toggle-hidden")
                                    .px(px(12.0))
                                    .py(px(8.0))
                                    .bg(if self.show_hidden {
                                        theme.tokens.primary
                                    } else {
                                        theme.tokens.muted
                                    })
                                    .text_color(if self.show_hidden {
                                        theme.tokens.primary_foreground
                                    } else {
                                        theme.tokens.foreground
                                    })
                                    .rounded(theme.tokens.radius_md)
                                    .cursor_pointer()
                                    .text_size(px(14.0))
                                    .on_click({
                                        let entity = entity.clone();
                                        move |_, _, cx| {
                                            entity.update(cx, |this, cx| {
                                                this.show_hidden = !this.show_hidden;
                                                cx.notify();
                                            });
                                        }
                                    })
                                    .child(if self.show_hidden {
                                        "Hide Hidden Files"
                                    } else {
                                        "Show Hidden Files"
                                    }),
                            )
                            .when_some(self.selected_path.as_ref(), |d, path| {
                                d.child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child(format!("Selected: {}", path.display())),
                                )
                            }),
                    )
                    .child(
                        div()
                            .flex_1()
                            .border_1()
                            .border_color(theme.tokens.border)
                            .rounded(theme.tokens.radius_md)
                            .overflow_hidden()
                            .child({
                                let entity = entity.clone();
                                FileTree::new()
                                    .nodes(Self::sample_tree())
                                    .expanded_paths(self.expanded_paths.clone())
                                    .show_hidden(self.show_hidden)
                                    .show_file_size(true)
                                    .when_some(self.selected_path.clone(), |ft, path| {
                                        ft.selected_path(path)
                                    })
                                    .on_select({
                                        let entity = entity.clone();
                                        move |path, _, cx| {
                                            entity.update(cx, |this, cx| {
                                                this.selected_path = Some(path.clone());
                                                cx.notify();
                                            });
                                        }
                                    })
                                    .on_toggle({
                                        let entity = entity.clone();
                                        move |path, expanded, _, cx| {
                                            entity.update(cx, |this, cx| {
                                                if expanded {
                                                    if !this.expanded_paths.contains(path) {
                                                        this.expanded_paths.push(path.clone());
                                                    }
                                                } else {
                                                    this.expanded_paths.retain(|p| p != path);
                                                }
                                                cx.notify();
                                            });
                                        }
                                    })
                                    .on_open(move |path, _, _cx| {
                                        println!("Opening file: {}", path.display());
                                    })
                            }),
                    ),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        adabraka_ui::init(cx);
        install_theme(cx, Theme::dark());

        let bounds = Bounds::centered(None, size(px(600.0), px(700.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("File Tree Demo".into()),
                    appears_transparent: false,
                    traffic_light_position: None,
                }),
                ..Default::default()
            },
            |_, cx| cx.new(|_| FileTreeDemo::new()),
        )
        .unwrap();

        cx.activate(true);
    });
}
