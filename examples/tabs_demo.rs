use gpui::*;
use adabraka_ui::{
    prelude::*,
    navigation::tabs::{Tabs, TabItem, TabPanel, TabVariant, init_tabs},
    components::{icon::IconSource, scrollable::scrollable_vertical},
    layout::{VStack, HStack},
    navigation::breadcrumbs::{Breadcrumbs, BreadcrumbItem},
};
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

struct TabsDemo {
    selected_underline_tab: usize,
    selected_enclosed_tab: usize,
    selected_pills_tab: usize,
    closeable_tabs: Vec<String>,
    breadcrumb_path: Vec<String>,
}

impl TabsDemo {
    fn new() -> Self {
        Self {
            selected_underline_tab: 0, // "home" tab index
            selected_enclosed_tab: 0, // "profile" tab index
            selected_pills_tab: 0, // "dashboard" tab index
            closeable_tabs: vec![
                "Tab 1".to_string(),
                "Tab 2".to_string(),
                "Tab 3".to_string(),
                "Tab 4".to_string(),
            ],
            breadcrumb_path: vec![
                "Home".to_string(),
                "Settings".to_string(),
                "Profile".to_string(),
            ],
        }
    }
}

impl Render for TabsDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        VStack::new()
            .size_full()
            .bg(theme.tokens.background)
            .text_color(theme.tokens.foreground)
            // Header
            .child(
                VStack::new()
                    .w_full()
                    .h(px(80.0))
                    .justify_center()
                    .items_center()
                    .border_b_1()
                    .border_color(theme.tokens.border)
                    .child(
                        VStack::new()
                            .items_center()
                            .gap(px(4.0))
                            .child(
                                div()
                                    .text_size(px(24.0))
                                    .font_weight(FontWeight::BOLD)
                                    .child("Tabs Component Demo"),
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Professional tabs with variants, icons, badges, and keyboard navigation"),
                            ),
                    ),
            )
            // Main content
            .child(
                div()
                    .flex_1()
                    .w_full()
                    .overflow_hidden()
                    .child(
                        scrollable_vertical(
                            div()
                                .flex()
                                .flex_col()
                                .w_full()
                                .p(px(32.0))
                                .gap(px(48.0))
                                // Underline Variant
                                .child(
                                    VStack::new()
                                        .w_full()
                                        .gap(px(16.0))
                                        .child(
                                            div()
                                                .text_size(px(18.0))
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .child("Underline Variant (Default)"),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(14.0))
                                                .text_color(theme.tokens.muted_foreground)
                                                .child("Clean, minimal design with an animated underline indicator. Perfect for navigation and content organization."),
                                        )
                                        .child({
                                            let selected_tab = self.selected_underline_tab;
                                            let app_entity = cx.entity().downgrade();
                                            Tabs::new()
                                                .variant(TabVariant::Underline)
                                                .tabs(vec![
                                                    TabItem::new("home", "Home")
                                                        .icon(IconSource::Named("home".to_string())),
                                                    TabItem::new("search", "Search")
                                                        .icon(IconSource::Named("search".to_string())),
                                                    TabItem::new("notifications", "Notifications")
                                                        .icon(IconSource::Named("bell".to_string()))
                                                        .badge("12"),
                                                    TabItem::new("settings", "Settings")
                                                        .icon(IconSource::Named("settings".to_string())),
                                                    TabItem::new("disabled", "Disabled")
                                                        .disabled(true),
                                                ])
                                                .selected_index(selected_tab)
                                                .panels(vec![
                                                    TabPanel::new(|| {
                                                        div()
                                                            .p(px(24.0))
                                                            .child(
                                                                div()
                                                                    .text_size(px(16.0))
                                                                    .child("Welcome to the Home panel! This is where your main content would live."),
                                                            )
                                                    }),
                                                    TabPanel::new(|| {
                                                        div()
                                                            .p(px(24.0))
                                                            .child(
                                                                div()
                                                                    .text_size(px(16.0))
                                                                    .child("Search panel content. Add your search interface here."),
                                                            )
                                                    }),
                                                    TabPanel::new(|| {
                                                        div()
                                                            .p(px(24.0))
                                                            .child(
                                                                div()
                                                                    .text_size(px(16.0))
                                                                    .child("Notifications panel. You have 12 unread notifications!"),
                                                            )
                                                    }),
                                                    TabPanel::new(|| {
                                                        div()
                                                            .p(px(24.0))
                                                            .child(
                                                                div()
                                                                    .text_size(px(16.0))
                                                                    .child("Settings panel. Configure your preferences here."),
                                                            )
                                                    }),
                                                    TabPanel::new(|| {
                                                        div()
                                                            .p(px(24.0))
                                                            .child(
                                                                div()
                                                                    .text_size(px(16.0))
                                                                    .child("This tab is disabled and cannot be selected."),
                                                            )
                                                    }),
                                                ])
                                                .on_change(move |index, _window, cx| {
                                                    if let Some(app) = app_entity.upgrade() {
                                                        app.update(cx, |demo, _cx| {
                                                            demo.selected_underline_tab = *index;
                                                        });
                                                    }
                                                })
                                        }),
                                )
                                // Enclosed Variant
                                .child(
                                    VStack::new()
                                        .w_full()
                                        .gap(px(16.0))
                                        .child(
                                            div()
                                                .text_size(px(18.0))
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .child("Enclosed Variant"),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(14.0))
                                                .text_color(theme.tokens.muted_foreground)
                                                .child("Traditional tab style with borders. Ideal for clear content separation and dashboard layouts."),
                                        )
                                        .child({
                                            let selected_tab = self.selected_enclosed_tab;
                                            let app_entity = cx.entity().downgrade();
                                            Tabs::new()
                                                .variant(TabVariant::Enclosed)
                                                .tabs(vec![
                                                    TabItem::new("profile", "Profile")
                                                        .icon(IconSource::Named("user".to_string())),
                                                    TabItem::new("account", "Account")
                                                        .icon(IconSource::Named("credit-card".to_string())),
                                                    TabItem::new("security", "Security")
                                                        .icon(IconSource::Named("lock".to_string()))
                                                        .badge("!"),
                                                    TabItem::new("integrations", "Integrations")
                                                        .icon(IconSource::Named("plugin".to_string())),
                                                ])
                                                .selected_index(selected_tab)
                                                .panels(vec![
                                                    TabPanel::new(|| {
                                                        div()
                                                            .p(px(24.0))
                                                            .child(
                                                                VStack::new()
                                                                    .gap(px(12.0))
                                                                    .child(
                                                                        div()
                                                                            .text_size(px(16.0))
                                                                            .font_weight(FontWeight::SEMIBOLD)
                                                                            .child("Profile Information"),
                                                                    )
                                                                    .child(
                                                                        div()
                                                                            .text_size(px(14.0))
                                                                            .child("Manage your profile details and preferences."),
                                                                    ),
                                                            )
                                                    }),
                                                    TabPanel::new(|| {
                                                        div()
                                                            .p(px(24.0))
                                                            .child(
                                                                VStack::new()
                                                                    .gap(px(12.0))
                                                                    .child(
                                                                        div()
                                                                            .text_size(px(16.0))
                                                                            .font_weight(FontWeight::SEMIBOLD)
                                                                            .child("Account Settings"),
                                                                    )
                                                                    .child(
                                                                        div()
                                                                            .text_size(px(14.0))
                                                                            .child("Manage your account and billing information."),
                                                                    ),
                                                            )
                                                    }),
                                                    TabPanel::new(|| {
                                                        div()
                                                            .p(px(24.0))
                                                            .child(
                                                                VStack::new()
                                                                    .gap(px(12.0))
                                                                    .child(
                                                                        div()
                                                                            .text_size(px(16.0))
                                                                            .font_weight(FontWeight::SEMIBOLD)
                                                                            .child("Security Settings"),
                                                                    )
                                                                    .child(
                                                                        div()
                                                                            .text_size(px(14.0))
                                                                            .child("⚠️ Action required: Update your security settings."),
                                                                    ),
                                                            )
                                                    }),
                                                    TabPanel::new(|| {
                                                        div()
                                                            .p(px(24.0))
                                                            .child(
                                                                VStack::new()
                                                                    .gap(px(12.0))
                                                                    .child(
                                                                        div()
                                                                            .text_size(px(16.0))
                                                                            .font_weight(FontWeight::SEMIBOLD)
                                                                            .child("Integrations"),
                                                                    )
                                                                    .child(
                                                                        div()
                                                                            .text_size(px(14.0))
                                                                            .child("Connect your favorite tools and services."),
                                                                    ),
                                                            )
                                                    }),
                                                ])
                                                .on_change(move |index, _window, cx| {
                                                    if let Some(app) = app_entity.upgrade() {
                                                        app.update(cx, |demo, _cx| {
                                                            demo.selected_enclosed_tab = *index;
                                                        });
                                                    }
                                                })
                                        }),
                                )
                                // Pills Variant
                                .child(
                                    VStack::new()
                                        .w_full()
                                        .gap(px(16.0))
                                        .child(
                                            div()
                                                .text_size(px(18.0))
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .child("Pills Variant"),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(14.0))
                                                .text_color(theme.tokens.muted_foreground)
                                                .child("Modern, rounded design with filled backgrounds. Great for compact interfaces and mobile-first designs."),
                                        )
                                        .child({
                                            let selected_tab = self.selected_pills_tab;
                                            let app_entity = cx.entity().downgrade();
                                            Tabs::new()
                                                .variant(TabVariant::Pills)
                                                .tabs(vec![
                                                    TabItem::new("dashboard", "Dashboard")
                                                        .icon(IconSource::Named("dashboard".to_string())),
                                                    TabItem::new("analytics", "Analytics")
                                                        .icon(IconSource::Named("chart".to_string())),
                                                    TabItem::new("reports", "Reports")
                                                        .icon(IconSource::Named("file-text".to_string()))
                                                        .badge("3"),
                                                    TabItem::new("export", "Export")
                                                        .icon(IconSource::Named("download".to_string())),
                                                ])
                                                .selected_index(selected_tab)
                                                .panels(vec![
                                                    TabPanel::new(|| {
                                                        div()
                                                            .p(px(24.0))
                                                            .child(
                                                                div()
                                                                    .text_size(px(16.0))
                                                                    .child("Dashboard overview with key metrics and insights."),
                                                            )
                                                    }),
                                                    TabPanel::new(|| {
                                                        div()
                                                            .p(px(24.0))
                                                            .child(
                                                                div()
                                                                    .text_size(px(16.0))
                                                                    .child("Analytics panel with charts and data visualization."),
                                                            )
                                                    }),
                                                    TabPanel::new(|| {
                                                        div()
                                                            .p(px(24.0))
                                                            .child(
                                                                div()
                                                                    .text_size(px(16.0))
                                                                    .child("Reports panel. You have 3 new reports available."),
                                                            )
                                                    }),
                                                    TabPanel::new(|| {
                                                        div()
                                                            .p(px(24.0))
                                                            .child(
                                                                div()
                                                                    .text_size(px(16.0))
                                                                    .child("Export your data in various formats."),
                                                            )
                                                    }),
                                                ])
                                                .on_change(move |index, _window, cx| {
                                                    if let Some(app) = app_entity.upgrade() {
                                                        app.update(cx, |demo, _cx| {
                                                            demo.selected_pills_tab = *index;
                                                        });
                                                    }
                                                })
                                        }),
                                )
                                // Closeable Tabs
                                .child(
                                    VStack::new()
                                        .w_full()
                                        .gap(px(16.0))
                                        .child(
                                            div()
                                                .text_size(px(18.0))
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .child("Closeable Tabs"),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(14.0))
                                                .text_color(theme.tokens.muted_foreground)
                                                .child("Tabs with close buttons. Perfect for document editors, multi-file views, and dynamic content. Use Cmd+W to close the active tab."),
                                        )
                                        .child({
                                            let tabs: Vec<TabItem<String>> = self.closeable_tabs
                                                .iter()
                                                .enumerate()
                                                .map(|(index, label)| {
                                                    TabItem::new(index.to_string(), label.clone())
                                                        .icon(IconSource::Named("file-text".to_string()))
                                                        .closeable(true)
                                                })
                                                .collect();

                                            let panels: Vec<TabPanel> = self.closeable_tabs
                                                .iter()
                                                .map(|label| {
                                                    let label = label.clone();
                                                    TabPanel::new(move || {
                                                        div()
                                                            .p(px(24.0))
                                                            .child(
                                                                div()
                                                                    .text_size(px(16.0))
                                                                    .child(format!("Content for {}. Click the X to close this tab.", label)),
                                                            )
                                                    })
                                                })
                                                .collect();

                                            let app_entity = cx.entity().downgrade();
                                            Tabs::new()
                                                .variant(TabVariant::Underline)
                                                .tabs(tabs)
                                                .panels(panels)
                                                .on_close(move |id, _window, cx| {
                                                    // Close functionality - remove tab from list
                                                    if let Some(app) = app_entity.upgrade() {
                                                        app.update(cx, |demo, _cx| {
                                                            if let Ok(index) = id.parse::<usize>() {
                                                                if index < demo.closeable_tabs.len() {
                                                                    demo.closeable_tabs.remove(index);
                                                                }
                                                            }
                                                        });
                                                    }
                                                })
                                        }),
                                )
                                // Breadcrumbs
                                .child(
                                    VStack::new()
                                        .w_full()
                                        .gap(px(16.0))
                                        .child(
                                            div()
                                                .text_size(px(18.0))
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .child("Breadcrumbs"),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(14.0))
                                                .text_color(theme.tokens.muted_foreground)
                                                .child("Navigation breadcrumbs with clickable path items. Perfect for hierarchical navigation and showing current location."),
                                        )
                                        .child({
                                            let breadcrumb_items: Vec<BreadcrumbItem<String>> = self.breadcrumb_path
                                                .iter()
                                                .enumerate()
                                                .map(|(index, label)| {
                                                    BreadcrumbItem {
                                                        id: index.to_string(),
                                                        label: label.clone().into(),
                                                        icon: if index == 0 {
                                                            Some(IconSource::Named("globe".to_string()))
                                                        } else {
                                                            None
                                                        },
                                                    }
                                                })
                                                .collect();

                                            let app_entity = cx.entity().downgrade();
                                            Breadcrumbs::new(cx)
                                                .items(breadcrumb_items)
                                                .on_click(move |id, _window, cx| {
                                                    // Navigate to the clicked breadcrumb level
                                                    if let Some(app) = app_entity.upgrade() {
                                                        app.update(cx, |demo, _cx| {
                                                            if let Ok(index) = id.parse::<usize>() {
                                                                // Truncate path to clicked level + 1
                                                                demo.breadcrumb_path.truncate(index + 1);
                                                            }
                                                        });
                                                    }
                                                })
                                        })
                                        .child(
                                            HStack::new()
                                                .gap(px(8.0))
                                                .mt(px(8.0))
                                                .children(vec![
                                                    Button::new("add-level-btn", "Add Level")
                                                        .on_click({
                                                            let app_entity = cx.entity().downgrade();
                                                            move |_, _, cx| {
                                                                if let Some(app) = app_entity.upgrade() {
                                                                    app.update(cx, |demo, _cx| {
                                                                        let level_num = demo.breadcrumb_path.len() + 1;
                                                                        demo.breadcrumb_path.push(format!("Level {}", level_num));
                                                                    });
                                                                }
                                                            }
                                                        }),
                                                    Button::new("reset-btn", "Reset")
                                                        .variant(ButtonVariant::Outline)
                                                        .on_click({
                                                            let app_entity = cx.entity().downgrade();
                                                            move |_, _, cx| {
                                                                if let Some(app) = app_entity.upgrade() {
                                                                    app.update(cx, |demo, _cx| {
                                                                        demo.breadcrumb_path = vec![
                                                                            "Home".to_string(),
                                                                            "Settings".to_string(),
                                                                            "Profile".to_string(),
                                                                        ];
                                                                    });
                                                                }
                                                            }
                                                        }),
                                                ])
                                        ),
                                )
                                // Keyboard Navigation Info
                                .child(
                                    VStack::new()
                                        .w_full()
                                        .gap(px(16.0))
                                        .p(px(24.0))
                                        .bg(theme.tokens.muted.opacity(0.5))
                                        .rounded(theme.tokens.radius_md)
                                        .border_1()
                                        .border_color(theme.tokens.border)
                                        .child(
                                            div()
                                                .text_size(px(16.0))
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .child("⌨️ Keyboard Navigation"),
                                        )
                                        .child(
                                            VStack::new()
                                                .gap(px(8.0))
                                                .text_size(px(14.0))
                                                .child("• Arrow Keys (←/→): Navigate between tabs")
                                                .child("• Home: Jump to first tab")
                                                .child("• End: Jump to last tab")
                                                .child("• Cmd+W: Close current tab (if closeable)")
                                                .child("• Tab: Focus tabs container"),
                                        ),
                                ),
                        ),
                    ),
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
        init_tabs(cx);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(1200.0), px(900.0)),
                    cx,
                ))),
                titlebar: Some(TitlebarOptions {
                    title: Some("Tabs Component Demo".into()),
                    appears_transparent: false,
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_window, cx| cx.new(|_cx| TabsDemo::new()),
        )
        .unwrap();
    });
}
