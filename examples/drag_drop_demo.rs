use gpui::{prelude::FluentBuilder as _, *};
use adabraka_ui::{
    components::drag_drop::{DragData, Draggable, DropZone, DropZoneStyle},
    theme::{install_theme, Theme},
};

/// File item that can be dragged
#[derive(Clone, Debug)]
struct FileItem {
    name: String,
    size: String,
    file_type: FileType,
}

#[derive(Clone, Debug)]
enum FileType {
    Document,
    Image,
    Video,
}

impl FileType {
    fn icon(&self) -> &'static str {
        match self {
            FileType::Document => "📄",
            FileType::Image => "🖼️",
            FileType::Video => "🎬",
        }
    }

    fn color(&self) -> Hsla {
        match self {
            FileType::Document => rgb(0x3b82f6).into(), // blue
            FileType::Image => rgb(0x10b981).into(),    // green
            FileType::Video => rgb(0xf59e0b).into(),    // amber
        }
    }
}

struct DragDropDemo {
    uploaded_files: Vec<FileItem>,
    trash_files: Vec<FileItem>,
}

impl DragDropDemo {
    fn new() -> Self {
        Self {
            uploaded_files: Vec::new(),
            trash_files: Vec::new(),
        }
    }

    fn sample_files() -> Vec<FileItem> {
        vec![
            FileItem {
                name: "presentation.pdf".to_string(),
                size: "2.4 MB".to_string(),
                file_type: FileType::Document,
            },
            FileItem {
                name: "vacation.jpg".to_string(),
                size: "1.8 MB".to_string(),
                file_type: FileType::Image,
            },
            FileItem {
                name: "tutorial.mp4".to_string(),
                size: "45.2 MB".to_string(),
                file_type: FileType::Video,
            },
            FileItem {
                name: "report.docx".to_string(),
                size: "856 KB".to_string(),
                file_type: FileType::Document,
            },
            FileItem {
                name: "screenshot.png".to_string(),
                size: "432 KB".to_string(),
                file_type: FileType::Image,
            },
        ]
    }
}

impl Render for DragDropDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = adabraka_ui::theme::use_theme();
        let files = Self::sample_files();

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(theme.tokens.background)
            .text_color(theme.tokens.foreground)
            .font_family(theme.tokens.font_family.clone())
            .p(px(24.0))
            .gap(px(24.0))
            .child(
                // Header
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .child(
                        div()
                            .text_size(px(32.0))
                            .font_weight(FontWeight::BOLD)
                            .child("Drag & Drop Demo")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("Drag files between zones to organize them")
                    )
            )
            .child(
                div()
                    .flex()
                    .flex_1()
                    .gap(px(24.0))
                    .child(
                        // Left panel - Available files
                        div()
                            .flex()
                            .flex_col()
                            .flex_1()
                            .gap(px(12.0))
                            .child(
                                div()
                                    .text_size(px(16.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("Available Files")
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.0))
                                    .children(files.iter().enumerate().map(|(ix, file)| {
                                        let file_clone = file.clone();
                                        let drag_data = DragData::new(file_clone.clone())
                                            .with_label(file.name.clone());

                                        Draggable::new(("file", ix), drag_data)
                                            .hover_bg(theme.tokens.muted.opacity(0.3))
                                            .child(
                                                div()
                                                    .flex()
                                                    .items_center()
                                                    .gap(px(12.0))
                                                    .px(px(12.0))
                                                    .py(px(10.0))
                                                    .rounded(theme.tokens.radius_md)
                                                    .border_1()
                                                    .border_color(file.file_type.color().opacity(0.3))
                                                    .bg(theme.tokens.card)
                                                    .child(
                                                        div()
                                                            .text_size(px(24.0))
                                                            .child(file.file_type.icon())
                                                    )
                                                    .child(
                                                        div()
                                                            .flex()
                                                            .flex_col()
                                                            .gap(px(2.0))
                                                            .flex_1()
                                                            .child(
                                                                div()
                                                                    .text_size(px(14.0))
                                                                    .font_weight(FontWeight::MEDIUM)
                                                                    .child(file.name.clone())
                                                            )
                                                            .child(
                                                                div()
                                                                    .text_size(px(12.0))
                                                                    .text_color(theme.tokens.muted_foreground)
                                                                    .child(file.size.clone())
                                                            )
                                                    )
                                            )
                                    }))
                            )
                    )
                    .child(
                        // Right panel - Drop zones
                        div()
                            .flex()
                            .flex_col()
                            .flex_1()
                            .gap(px(16.0))
                            .child(
                                // Upload zone
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.0))
                                    .flex_1()
                                    .child(
                                        div()
                                            .text_size(px(16.0))
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .child("Upload Zone")
                                    )
                                    .child(
                                        DropZone::<FileItem>::new("upload-zone")
                                            .style(DropZoneStyle::Dashed)
                                            .min_h(px(200.0))
                                            .on_drop(cx.listener(|this, data: &DragData<FileItem>, _, cx| {
                                                this.uploaded_files.push(data.data.clone());
                                                cx.notify();
                                            }))
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .gap(px(12.0))
                                                    .items_center()
                                                    .child(
                                                        div()
                                                            .text_size(px(48.0))
                                                            .child("📁")
                                                    )
                                                    .child(
                                                        div()
                                                            .text_size(px(14.0))
                                                            .text_color(theme.tokens.muted_foreground)
                                                            .child("Drop files here to upload")
                                                    )
                                                    .when(!self.uploaded_files.is_empty(), |this| {
                                                        this.child(
                                                            div()
                                                                .mt(px(16.0))
                                                                .flex()
                                                                .flex_col()
                                                                .gap(px(4.0))
                                                                .w_full()
                                                                .children(self.uploaded_files.iter().map(|file| {
                                                                    div()
                                                                        .flex()
                                                                        .items_center()
                                                                        .gap(px(8.0))
                                                                        .px(px(12.0))
                                                                        .py(px(6.0))
                                                                        .rounded(px(4.0))
                                                                        .bg(theme.tokens.muted.opacity(0.5))
                                                                        .child(div().text_size(px(16.0)).child(file.file_type.icon()))
                                                                        .child(
                                                                            div()
                                                                                .text_size(px(13.0))
                                                                                .child(file.name.clone())
                                                                        )
                                                                }))
                                                        )
                                                    })
                                            )
                                    )
                            )
                            .child(
                                // Trash zone
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.0))
                                    .flex_1()
                                    .child(
                                        div()
                                            .text_size(px(16.0))
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .child("Trash")
                                    )
                                    .child(
                                        DropZone::<FileItem>::new("trash-zone")
                                            .style(DropZoneStyle::Filled)
                                            .min_h(px(200.0))
                                            .on_drop(cx.listener(|this, data: &DragData<FileItem>, _, cx| {
                                                this.trash_files.push(data.data.clone());
                                                cx.notify();
                                            }))
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .gap(px(12.0))
                                                    .items_center()
                                                    .child(
                                                        div()
                                                            .text_size(px(48.0))
                                                            .child("🗑️")
                                                    )
                                                    .child(
                                                        div()
                                                            .text_size(px(14.0))
                                                            .text_color(theme.tokens.muted_foreground)
                                                            .child("Drop files here to delete")
                                                    )
                                                    .when(!self.trash_files.is_empty(), |this| {
                                                        this.child(
                                                            div()
                                                                .mt(px(16.0))
                                                                .flex()
                                                                .flex_col()
                                                                .gap(px(4.0))
                                                                .w_full()
                                                                .children(self.trash_files.iter().map(|file| {
                                                                    div()
                                                                        .flex()
                                                                        .items_center()
                                                                        .gap(px(8.0))
                                                                        .px(px(12.0))
                                                                        .py(px(6.0))
                                                                        .rounded(px(4.0))
                                                                        .bg(theme.tokens.destructive.opacity(0.1))
                                                                        .text_color(theme.tokens.destructive)
                                                                        .child(div().text_size(px(16.0)).child(file.file_type.icon()))
                                                                        .child(
                                                                            div()
                                                                                .text_size(px(13.0))
                                                                                .child(file.name.clone())
                                                                        )
                                                                }))
                                                        )
                                                    })
                                            )
                                    )
                            )
                    )
            )
            .child(
                // Footer
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .child(
                        div()
                            .text_size(px(13.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child(format!(
                                "{} uploaded • {} deleted",
                                self.uploaded_files.len(),
                                self.trash_files.len()
                            ))
                    )
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        // Initialize the UI library
        adabraka_ui::init(cx);

        // Install dark theme
        install_theme(cx, Theme::dark());

        let bounds = Bounds::centered(None, size(px(1000.0), px(700.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("Drag & Drop Demo".into()),
                    appears_transparent: false,
                    traffic_light_position: None,
                }),
                ..Default::default()
            },
            |_, cx| cx.new(|_| DragDropDemo::new()),
        )
        .unwrap();

        cx.activate(true);
    });
}
