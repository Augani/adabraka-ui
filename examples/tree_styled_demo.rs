use adabraka_ui::{
    prelude::*,
    components::scrollable::scrollable_vertical,
    navigation::tree::{TreeList, TreeNode},
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
                        title: Some("TreeList Styled Trait Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(1000.0), px(800.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|_| TreeStyledDemo::new()),
            )
            .unwrap();
        });
}

struct TreeStyledDemo {
    selected_id: Option<String>,
    expanded_ids: Vec<String>,
}

impl TreeStyledDemo {
    fn new() -> Self {
        Self {
            selected_id: Some("file1".to_string()),
            expanded_ids: vec!["root".to_string(), "folder1".to_string()],
        }
    }

    fn create_sample_tree() -> Vec<TreeNode<String>> {
        vec![
            TreeNode::new("root".to_string(), "Project Root")
                .with_icon("folder")
                .with_children(vec![
                    TreeNode::new("folder1".to_string(), "src")
                        .with_icon("folder")
                        .with_children(vec![
                            TreeNode::new("file1".to_string(), "main.rs")
                                .with_icon("file"),
                            TreeNode::new("file2".to_string(), "lib.rs")
                                .with_icon("file"),
                        ]),
                    TreeNode::new("folder2".to_string(), "tests")
                        .with_icon("folder")
                        .with_children(vec![
                            TreeNode::new("test1".to_string(), "integration_test.rs")
                                .with_icon("file"),
                        ]),
                    TreeNode::new("file3".to_string(), "Cargo.toml")
                        .with_icon("file"),
                ]),
        ]
    }
}

impl Render for TreeStyledDemo {
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
                            .child("TreeList Styled Trait Customization Demo")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("Demonstrating full GPUI styling control via Styled trait on TreeList")
                    )
            )
            // 1. Default TreeList
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("1. Default TreeList")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("No custom styling applied")
                    )
                    .child(
                        TreeList::new()
                            .nodes(Self::create_sample_tree())
                            .selected_id("file1".to_string())
                            .expanded_ids(vec!["root".to_string(), "folder1".to_string()])
                    )
            )
            // 2. Custom Padding
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("2. Custom Padding (via Styled trait)")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("TreeList with p(px(24.0)) padding")
                    )
                    .child(
                        TreeList::new()
                            .nodes(Self::create_sample_tree())
                            .selected_id("file2".to_string())
                            .expanded_ids(vec!["root".to_string(), "folder1".to_string()])
                            .p(px(24.0))  // Styled trait method
                    )
            )
            // 3. Custom Background & Border
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("3. Custom Background & Border")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("TreeList with custom blue background and border")
                    )
                    .child(
                        TreeList::new()
                            .nodes(Self::create_sample_tree())
                            .selected_id("test1".to_string())
                            .expanded_ids(vec!["root".to_string(), "folder2".to_string()])
                            .bg(rgb(0x1e3a5f))  // Styled trait
                            .border_2()  // Styled trait
                            .border_color(rgb(0x3b82f6))
                            .rounded(px(8.0))  // Styled trait
                            .p(px(12.0))  // Styled trait
                    )
            )
            // 4. Custom Border Radius & Shadow
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("4. Custom Border Radius & Shadow")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("TreeList with large border radius and shadow")
                    )
                    .child(
                        TreeList::new()
                            .nodes(Self::create_sample_tree())
                            .selected_id("file3".to_string())
                            .expanded_ids(vec!["root".to_string()])
                            .rounded(px(16.0))  // Styled trait
                            .shadow_lg()  // Styled trait
                            .p(px(16.0))  // Styled trait
                    )
            )
            // 5. Width Control
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("5. Width Control")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("TreeList with custom width (600px)")
                    )
                    .child(
                        TreeList::new()
                            .nodes(Self::create_sample_tree())
                            .selected_id("file1".to_string())
                            .expanded_ids(vec!["root".to_string(), "folder1".to_string()])
                            .w(px(600.0))  // Styled trait
                            .border_1()
                            .border_color(theme.tokens.border)
                            .rounded(px(8.0))
                            .p(px(12.0))
                    )
            )
            // 6. Combined Styling - Card Style
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("6. Combined Styling - Card Style")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("TreeList with multiple Styled trait methods combined")
                    )
                    .child(
                        TreeList::new()
                            .nodes(Self::create_sample_tree())
                            .selected_id("file2".to_string())
                            .expanded_ids(vec!["root".to_string(), "folder1".to_string(), "folder2".to_string()])
                            .bg(rgb(0x1a1a2e))  // Styled trait
                            .border_2()  // Styled trait
                            .border_color(rgb(0x8b5cf6))
                            .rounded(px(12.0))  // Styled trait
                            .p(px(20.0))  // Styled trait
                            .shadow_md()  // Styled trait
                    )
            )
            // 7. Custom Height Control
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("7. Custom Height Control")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("TreeList with max height and scrolling")
                    )
                    .child(
                        TreeList::new()
                            .nodes(Self::create_sample_tree())
                            .selected_id("test1".to_string())
                            .expanded_ids(vec!["root".to_string(), "folder1".to_string(), "folder2".to_string()])
                            .h(px(200.0))  // Styled trait
                            .border_1()
                            .border_color(theme.tokens.border)
                            .rounded(px(8.0))
                            .overflow_hidden()
                    )
            )
            // 8. Full Custom Styling
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("8. Full Custom Styling")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("TreeList with all styling options combined")
                    )
                    .child(
                        TreeList::new()
                            .nodes(Self::create_sample_tree())
                            .selected_id("file3".to_string())
                            .expanded_ids(vec!["root".to_string()])
                            .w(px(700.0))  // Styled trait
                            .h(px(250.0))  // Styled trait
                            .bg(rgb(0x0f172a))  // Styled trait
                            .border_2()  // Styled trait
                            .border_color(rgb(0x10b981))
                            .rounded(px(16.0))  // Styled trait
                            .px(px(24.0))  // Styled trait
                            .py(px(20.0))  // Styled trait
                            .shadow_lg()  // Styled trait
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
                            .child("All customizations above use the Styled trait for full GPUI styling control!")
                    )
                    .child(
                        div()
                            .mt(px(8.0))
                            .text_size(px(12.0))
                            .text_color(theme.tokens.accent_foreground)
                            .child("Methods used: .p(), .px(), .py(), .bg(), .border_1/2(), .rounded(), .w(), .h(), .shadow_md/lg(), .overflow_hidden()")
                    )
            )
                )
            )
    }
}
