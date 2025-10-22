use gpui::*;
use adabraka_ui::{
    navigation::sidebar::{Sidebar, SidebarItem, SidebarVariant, SidebarPosition},
    components::{icon_source::IconSource, resizable::{ResizableState, h_resizable, resizable_panel}},
    layout::VStack,
    theme::use_theme,
};

struct SidebarDemo {
    selected_sidebar_item: Option<String>,
    sidebar_expanded: bool,
    sidebar_resizable_state: Entity<ResizableState>,
}

impl SidebarDemo {
    fn new(cx: &mut App) -> Self {
        Self {
            selected_sidebar_item: Some("dashboard".to_string()),
            sidebar_expanded: true,
            sidebar_resizable_state: ResizableState::new(cx),
        }
    }
}

impl Render for SidebarDemo {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        h_resizable("sidebar-layout", self.sidebar_resizable_state.clone())
            .child({
                let app_entity = cx.entity().downgrade();
                let app_entity_for_select = app_entity.clone();
                let app_entity_for_toggle = app_entity.clone();

                resizable_panel()
                    .child(
                        Sidebar::new(cx)
                            .variant(SidebarVariant::Collapsible)
                            .position(SidebarPosition::Left)
                            .expanded_width(px(280.0))
                            .collapsed_width(px(64.0))
                            .items({
                                let mut items = Vec::new();
                                items.push(SidebarItem::new("dashboard".to_string(), "Dashboard".to_string())
                                    .with_icon(IconSource::Named("globe".to_string())));
                                items.push(SidebarItem::new("analytics".to_string(), "Analytics".to_string())
                                    .with_icon(IconSource::Named("search".to_string()))
                                    .with_badge("3".to_string()));
                                items.push(SidebarItem::new("separator1".to_string(), "".to_string()).separator(true));
                                items.push(SidebarItem::new("projects".to_string(), "Projects".to_string())
                                    .with_icon(IconSource::Named("globe".to_string())));
                                items.push(SidebarItem::new("team".to_string(), "Team".to_string())
                                    .with_icon(IconSource::Named("search".to_string())));
                                items.push(SidebarItem::new("settings".to_string(), "Settings".to_string())
                                    .with_icon(IconSource::Named("palette".to_string())));
                                items
                            })
                            .selected_id(self.selected_sidebar_item.clone().unwrap_or("dashboard".to_string()))
                            .show_toggle_button(true)
                            .on_select(move |id, _window, cx| {
                                if let Some(app) = app_entity_for_select.upgrade() {
                                    app.update(cx, |demo, _cx| {
                                        demo.selected_sidebar_item = Some(id.clone());
                                    });
                                }
                            })
                            .on_toggle(move |expanded, _window, cx| {
                                if let Some(app) = app_entity_for_toggle.upgrade() {
                                    app.update(cx, |demo, _cx| {
                                        demo.sidebar_expanded = expanded;
                                    });
                                }
                            })
                    )
            })
            .child(
                resizable_panel()
                    .child(
                        VStack::new()
                            .flex_1()
                            .p(px(32.0))
                            .gap(px(16.0))
                            .child(
                                div()
                                    .text_size(px(24.0))
                                    .font_weight(FontWeight::BOLD)
                                    .child("Sidebar Component Demo"),
                            )
                            .child(
                                div()
                                    .text_size(px(16.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("A collapsible sidebar with navigation items, icons, and badges."),
                            )
                            .child(
                                VStack::new()
                                    .w_full()
                                    .p(px(24.0))
                                    .bg(theme.tokens.card)
                                    .border_1()
                                    .border_color(theme.tokens.border)
                                    .border_r(theme.tokens.radius_md)
                                    .gap(px(16.0))
                                    .child(
                                        div()
                                            .text_size(px(18.0))
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .child("Current Selection"),
                                    )
                                    .child(
                                        div()
                                            .text_size(px(16.0))
                                            .child(format!(
                                                "Selected: {}",
                                                self.selected_sidebar_item.as_deref().unwrap_or("None")
                                            )),
                                    )
                                    .child(
                                        div()
                                            .text_size(px(16.0))
                                            .child(format!(
                                                "Sidebar Expanded: {}",
                                                self.sidebar_expanded
                                            )),
                                    )
                            )
                            .child(
                                VStack::new()
                                    .w_full()
                                    .gap(px(12.0))
                                    .child(
                                        div()
                                            .text_size(px(16.0))
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .child("Features"),
                                    )
                                    .children(vec![
                                        div().child("• Collapsible/expandable sidebar"),
                                        div().child("• Multiple variants (fixed, collapsible, overlay)"),
                                        div().child("• Icon support for navigation items"),
                                        div().child("• Badge support for notifications"),
                                        div().child("• Separator items"),
                                        div().child("• Keyboard navigation (Escape to toggle)"),
                                        div().child("• Position control (left/right)"),
                                        div().child("• Customizable widths"),
                                        div().child("• Selection state management"),
                                        div().child("• Hover effects and focus management"),
                                    ])
                            )
                    )
            )
    }
}

fn main() {
    Application::new()
        .with_assets(Assets { base: PathBuf::from(env!("CARGO_MANIFEST_DIR")) })
        .run(move |cx: &mut App| {
            adabraka_ui::theme::install_theme(cx, adabraka_ui::theme::Theme::dark());
            adabraka_ui::init(cx);
            adabraka_ui::set_icon_base_path("assets/icons");

            let options = WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(1200.), px(800.)),
                    cx,
                ))),
                ..Default::default()
            };

            cx.open_window(options, |_, cx| {
                cx.activate(false);
                cx.new(|cx| SidebarDemo::new(cx))
            })
            .unwrap();
        });
}

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
                        Some(SharedString::from(
                            entry.ok()?.path().to_string_lossy().into_owned(),
                        ))
                    })
                    .collect::<Vec<_>>()
            })
            .map_err(|err| err.into())
    }
}
