use adabraka_ui::prelude::*;
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

struct ImageViewerDemoApp {
    viewer: Entity<ImageViewer>,
}

impl ImageViewerDemoApp {
    fn new(cx: &mut Context<Self>) -> Self {
        let images = vec![
            ImageItem::new("assets/images/carousel_1.jpg"),
            ImageItem::new("assets/images/carousel_2.jpg"),
            ImageItem::new("assets/images/carousel_3.jpg"),
            ImageItem::new("assets/images/carousel_4.jpg"),
            ImageItem::new("assets/images/carousel_5.jpg"),
        ];

        let viewer_state = cx.new(|_| ImageViewerState::new(images));

        Self {
            viewer: cx.new(|cx| ImageViewer::new(viewer_state, cx).show_thumbnails(true)),
        }
    }
}

impl Render for ImageViewerDemoApp {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        div()
            .size_full()
            .bg(theme.tokens.background)
            .p(px(40.0))
            .flex()
            .flex_col()
            .gap(px(24.0))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .child(h2("Image Viewer Component"))
                    .child(muted("View images with navigation and thumbnails")),
            )
            .child(
                div()
                    .h(px(500.0))
                    .w_full()
                    .rounded(theme.tokens.radius_lg)
                    .overflow_hidden()
                    .child(self.viewer.clone()),
            )
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
            init_image_viewer(cx);

            let bounds = Bounds::centered(None, size(px(900.0), px(700.0)), cx);
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                |_, cx| cx.new(ImageViewerDemoApp::new),
            )
            .unwrap();
        });
}
