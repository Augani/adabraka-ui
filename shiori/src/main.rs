mod app;
mod autosave;
mod search_bar;
mod status_bar;

use app::AppState;
use adabraka_ui::theme::{install_theme, Theme};
use gpui::*;
use std::borrow::Cow;
use std::path::PathBuf;

struct Assets {
    base: PathBuf,
}

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        std::fs::read(self.base.join(path))
            .map(|data| Some(Cow::Owned(data)))
            .map_err(|err| err.into())
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        std::fs::read_dir(self.base.join(path))
            .map(|entries| {
                entries
                    .filter_map(|entry| {
                        entry
                            .ok()
                            .and_then(|e| e.file_name().into_string().ok())
                            .map(SharedString::from)
                    })
                    .collect()
            })
            .map_err(|err| err.into())
    }
}

fn main() {
    let paths: Vec<PathBuf> = std::env::args().skip(1).map(PathBuf::from).collect();

    Application::new()
        .with_assets(Assets {
            base: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(".."),
        })
        .run(move |cx: &mut App| {
            adabraka_ui::init(cx);
            adabraka_ui::set_icon_base_path("assets/icons");
            install_theme(cx, Theme::dark());
            app::init(cx);

            let bounds = Bounds::centered(None, size(px(1200.0), px(800.0)), cx);
            let paths_for_window = paths.clone();
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    titlebar: Some(TitlebarOptions {
                        title: Some("Shiori".into()),
                        appears_transparent: false,
                        traffic_light_position: None,
                    }),
                    ..Default::default()
                },
                |_, cx| {
                    cx.new(|cx| {
                        let mut state = AppState::new(cx);
                        let mut file_paths = Vec::new();
                        let mut folder_path = None;
                        for path in paths_for_window {
                            if path.is_dir() {
                                folder_path = Some(path);
                            } else {
                                file_paths.push(path);
                            }
                        }
                        if let Some(folder) = folder_path {
                            state.open_folder(folder, cx);
                        }
                        if !file_paths.is_empty() {
                            state.open_paths(file_paths, cx);
                        }
                        state
                    })
                },
            )
            .unwrap();

            cx.activate(true);
        });
}
