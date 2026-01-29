use adabraka_ui::prelude::*;
use gpui::*;

struct FileUploadDemoApp {
    upload_state1: Entity<FileUploadState>,
    upload_state2: Entity<FileUploadState>,
}

impl FileUploadDemoApp {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            upload_state1: cx.new(|_| FileUploadState::new()),
            upload_state2: cx.new(|_| FileUploadState::new()),
        }
    }
}

impl Render for FileUploadDemoApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        let files1 = &self.upload_state1.read(cx).files;
        let files2 = &self.upload_state2.read(cx).files;

        div()
            .size_full()
            .bg(theme.tokens.background)
            .p(px(40.0))
            .flex()
            .flex_col()
            .gap(px(32.0))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .child(h2("File Upload Component"))
                    .child(muted("Drag and drop or click to upload files")),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(24.0))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(8.0))
                            .child(label("Default Upload"))
                            .child(FileUpload::new("upload1", self.upload_state1.clone()))
                            .child(muted(format!("Files: {}", files1.len()))),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(8.0))
                            .child(label("Multiple Files - Large Size"))
                            .child(
                                FileUpload::new("upload2", self.upload_state2.clone())
                                    .multiple(true)
                                    .size(FileUploadSize::Lg),
                            )
                            .child(muted(format!("Files: {}", files2.len()))),
                    ),
            )
    }
}

fn main() {
    Application::new().run(|cx| {
        adabraka_ui::init(cx);
        install_theme(cx, Theme::dark());

        let bounds = Bounds::centered(None, size(px(600.0), px(500.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(FileUploadDemoApp::new),
        )
        .unwrap();
    });
}
