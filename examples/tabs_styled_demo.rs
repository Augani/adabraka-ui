use adabraka_ui::{
    prelude::*,
    components::scrollable::scrollable_vertical,
    navigation::tabs::{Tabs, TabItem, TabPanel, TabVariant},
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
                        title: Some("Tabs Styled Trait Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(900.0), px(900.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|_| TabsStyledDemo::new()),
            )
            .unwrap();
        });
}

struct TabsStyledDemo {
    selected_tab: usize,
}

impl TabsStyledDemo {
    fn new() -> Self {
        Self { selected_tab: 0 }
    }

    fn render_demo_section(
        title: impl Into<SharedString>,
        description: impl Into<SharedString>,
        tabs: impl IntoElement,
        theme: &Theme,
    ) -> impl IntoElement {
        let title: SharedString = title.into();
        let description: SharedString = description.into();

        VStack::new()
            .gap(px(12.0))
            .p(px(20.0))
            .child(
                VStack::new()
                    .gap(px(4.0))
                    .child(
                        div()
                            .text_size(px(16.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child(title)
                    )
                    .child(
                        div()
                            .text_size(px(13.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child(description)
                    )
            )
            .child(
                div()
                    .w(px(600.0))
                    .h(px(300.0))
                    .child(tabs)
            )
    }
}

impl Render for TabsStyledDemo {
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
                                        .text_size(px(28.0))
                                        .font_weight(FontWeight::BOLD)
                                        .child("Tabs Component - Styled Trait Demo")
                                )
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child("Comprehensive examples of styling variations using the Styled trait")
                                )
                        )
                        // Example 1: Custom Background and Border
                        .child(Self::render_demo_section(
                            "1. Custom Background & Border",
                            "Tabs with custom background color, border, and rounded corners",
                            Tabs::<usize>::new()
                                .tabs(vec![
                                    TabItem::new(0, "Overview"),
                                    TabItem::new(1, "Features"),
                                    TabItem::new(2, "Documentation"),
                                ])
                                .panels(vec![
                                    TabPanel::new(|| div().child("Overview content")),
                                    TabPanel::new(|| div().child("Features content")),
                                    TabPanel::new(|| div().child("Documentation content")),
                                ])
                                .variant(TabVariant::Underline)
                                .bg(theme.tokens.muted)
                                .border_2()
                                .border_color(theme.tokens.primary)
                                .rounded(px(12.0))
                                .p(px(16.0)),
                            &theme,
                        ))
                        // Example 2: Shadow and Padding
                        .child(Self::render_demo_section(
                            "2. Shadow & Padding Styles",
                            "Enhanced depth with shadow effects and custom padding",
                            Tabs::<usize>::new()
                                .tabs(vec![
                                    TabItem::new(0, "Dashboard"),
                                    TabItem::new(1, "Analytics"),
                                    TabItem::new(2, "Settings"),
                                ])
                                .panels(vec![
                                    TabPanel::new(|| div().child("Dashboard content")),
                                    TabPanel::new(|| div().child("Analytics content")),
                                    TabPanel::new(|| div().child("Settings content")),
                                ])
                                .variant(TabVariant::Pills)
                                .bg(theme.tokens.card)
                                .p(px(24.0))
                                .shadow(vec![BoxShadow {
                                    offset: point(px(0.0), px(4.0)),
                                    blur_radius: px(12.0),
                                    spread_radius: px(0.0),
                                    color: hsla(0.0, 0.0, 0.0, 0.1),
                                }])
                                .rounded(px(16.0)),
                            &theme,
                        ))
                        // Example 3: Custom Width and Height
                        .child(Self::render_demo_section(
                            "3. Custom Dimensions",
                            "Fixed width and height with overflow handling",
                            Tabs::<usize>::new()
                                .tabs(vec![
                                    TabItem::new(0, "Projects"),
                                    TabItem::new(1, "Tasks"),
                                    TabItem::new(2, "Team"),
                                ])
                                .panels(vec![
                                    TabPanel::new(|| div().child("Projects content with long text that might overflow")),
                                    TabPanel::new(|| div().child("Tasks content")),
                                    TabPanel::new(|| div().child("Team content")),
                                ])
                                .variant(TabVariant::Enclosed)
                                .w(px(500.0))
                                .h(px(250.0))
                                .bg(theme.tokens.muted)
                                .p(px(12.0)),
                            &theme,
                        ))
                        // Example 4: Enhanced Border Style
                        .child(Self::render_demo_section(
                            "4. Enhanced Border Styling",
                            "Modern border styling with transparency",
                            Tabs::<usize>::new()
                                .tabs(vec![
                                    TabItem::new(0, "Home")
                                        .icon("home"),
                                    TabItem::new(1, "Profile")
                                        .icon("user"),
                                    TabItem::new(2, "Messages")
                                        .icon("mail")
                                        .badge("3"),
                                ])
                                .panels(vec![
                                    TabPanel::new(|| div().child("Home panel content")),
                                    TabPanel::new(|| div().child("Profile panel content")),
                                    TabPanel::new(|| div().child("Messages panel content")),
                                ])
                                .variant(TabVariant::Pills)
                                .bg(theme.tokens.muted)
                                .p(px(20.0))
                                .rounded(px(20.0))
                                .border_2()
                                .border_color(theme.tokens.border),
                            &theme,
                        ))
                        // Example 5: Minimal Styling
                        .child(Self::render_demo_section(
                            "5. Minimal & Clean",
                            "Subtle styling with minimal borders",
                            Tabs::<usize>::new()
                                .tabs(vec![
                                    TabItem::new(0, "Inbox"),
                                    TabItem::new(1, "Sent"),
                                    TabItem::new(2, "Drafts"),
                                    TabItem::new(3, "Trash"),
                                ])
                                .panels(vec![
                                    TabPanel::new(|| div().child("Inbox content")),
                                    TabPanel::new(|| div().child("Sent content")),
                                    TabPanel::new(|| div().child("Drafts content")),
                                    TabPanel::new(|| div().child("Trash content")),
                                ])
                                .variant(TabVariant::Underline)
                                .border_1()
                                .border_color(theme.tokens.border)
                                .rounded(px(8.0))
                                .p(px(16.0)),
                            &theme,
                        ))
                        // Example 6: Compact Layout
                        .child(Self::render_demo_section(
                            "6. Compact Layout",
                            "Space-efficient design with tight spacing",
                            Tabs::<usize>::new()
                                .tabs(vec![
                                    TabItem::new(0, "Tab 1"),
                                    TabItem::new(1, "Tab 2"),
                                    TabItem::new(2, "Tab 3"),
                                ])
                                .panels(vec![
                                    TabPanel::new(|| div().child("Content 1")),
                                    TabPanel::new(|| div().child("Content 2")),
                                    TabPanel::new(|| div().child("Content 3")),
                                ])
                                .variant(TabVariant::Pills)
                                .p(px(8.0))
                                .gap(px(8.0))
                                .bg(theme.tokens.muted)
                                .rounded(px(6.0)),
                            &theme,
                        ))
                        // Example 7: Bordered Card Style
                        .child(Self::render_demo_section(
                            "7. Bordered Card Style",
                            "Card-like appearance with prominent borders",
                            Tabs::<usize>::new()
                                .tabs(vec![
                                    TabItem::new(0, "Account")
                                        .icon("user"),
                                    TabItem::new(1, "Security")
                                        .icon("shield"),
                                    TabItem::new(2, "Billing")
                                        .icon("credit-card"),
                                ])
                                .panels(vec![
                                    TabPanel::new(|| div().child("Account settings")),
                                    TabPanel::new(|| div().child("Security settings")),
                                    TabPanel::new(|| div().child("Billing information")),
                                ])
                                .variant(TabVariant::Enclosed)
                                .bg(theme.tokens.card)
                                .border_2()
                                .border_color(theme.tokens.border)
                                .rounded(px(12.0))
                                .p(px(20.0))
                                .shadow(vec![BoxShadow {
                                    offset: point(px(0.0), px(2.0)),
                                    blur_radius: px(8.0),
                                    spread_radius: px(0.0),
                                    color: hsla(0.0, 0.0, 0.0, 0.05),
                                }]),
                            &theme,
                        ))
                        // Example 8: Accent Color Theme
                        .child(Self::render_demo_section(
                            "8. Accent Color Theme",
                            "Using accent colors for emphasis",
                            Tabs::<usize>::new()
                                .tabs(vec![
                                    TabItem::new(0, "Code"),
                                    TabItem::new(1, "Preview"),
                                    TabItem::new(2, "Console"),
                                ])
                                .panels(vec![
                                    TabPanel::new(|| div().child("Code editor panel")),
                                    TabPanel::new(|| div().child("Preview panel")),
                                    TabPanel::new(|| div().child("Console output")),
                                ])
                                .variant(TabVariant::Pills)
                                .bg(theme.tokens.accent)
                                .p(px(16.0))
                                .rounded(px(10.0))
                                .border_1()
                                .border_color(theme.tokens.border),
                            &theme,
                        ))
                )
            )
    }
}
