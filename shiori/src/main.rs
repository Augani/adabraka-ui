mod ansi_parser;
mod app;
mod autosave;
mod completion;
mod diff_highlighter;
mod git_service;
mod git_state;
mod git_view;
pub mod ide_theme;
mod pty_service;
mod search_bar;
mod terminal_state;
mod terminal_view;

use adabraka_ui::navigation::app_menu::{
    edit_menu, file_menu, view_menu, window_menu, StandardMacMenuBar,
};
use adabraka_ui::theme::{install_theme, Theme};
use app::{AppState, NewFile, OpenFile, OpenFolder, SaveFile};
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
            crate::ide_theme::sync_adabraka_theme_from_ide(cx);

            cx.set_menus(
                StandardMacMenuBar::new("Shiori")
                    .file_menu(
                        file_menu()
                            .action("New File", NewFile)
                            .action("Open File", OpenFile)
                            .action("Open Folder", OpenFolder)
                            .separator()
                            .action("Save", SaveFile),
                    )
                    .edit_menu(edit_menu())
                    .view_menu(view_menu())
                    .window_menu(window_menu())
                    .build(),
            );

            let bounds = Bounds::centered(None, size(px(1200.0), px(800.0)), cx);
            let paths_for_window = paths.clone();
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    titlebar: Some(TitlebarOptions {
                        title: Some("Shiori".into()),
                        appears_transparent: true,
                        traffic_light_position: Some(point(px(16.0), px(14.0))),
                    }),
                    window_background: WindowBackgroundAppearance::Opaque,
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
