use adabraka_ui::{
    prelude::*,
    components::{
        scrollable::scrollable_vertical,
        navigation_menu::{NavigationMenu, NavigationMenuItem, NavigationMenuOrientation},
    },
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
                        title: Some("NavigationMenu Styled Trait Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(1000.0), px(900.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|_| NavigationMenuStyledDemo::new()),
            )
            .unwrap();
        });
}

struct NavigationMenuStyledDemo;

impl NavigationMenuStyledDemo {
    fn new() -> Self {
        Self
    }
}

impl Render for NavigationMenuStyledDemo {
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
                            .child("NavigationMenu Styled Trait Customization Demo")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("Demonstrating full GPUI styling control via Styled trait")
                    )
            )
            // 1. Basic Vertical Menu (Default)
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("1. Basic Vertical Menu (Default Styling)")
                    )
                    .child(
                        div()
                            .w(px(250.0))
                            .child(
                                NavigationMenu::new()
                                    .orientation(NavigationMenuOrientation::Vertical)
                                    .items(vec![
                                        NavigationMenuItem::new("file-explorer", "File Explorer")
                                            .with_icon("folder"),
                                        NavigationMenuItem::new("search", "Search")
                                            .with_icon("search"),
                                        NavigationMenuItem::new("source-control", "Source Control")
                                            .with_icon("git-branch"),
                                        NavigationMenuItem::new("view", "View")
                                            .with_icon("eye")
                                            .with_children(vec![
                                                NavigationMenuItem::new("command-palette", "Command Palette"),
                                                NavigationMenuItem::new("output", "Output"),
                                                NavigationMenuItem::new("terminal", "Terminal"),
                                            ]),
                                    ])
                            )
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
                        HStack::new()
                            .gap(px(16.0))
                            .child(
                                VStack::new()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(14.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("With p_4()")
                                    )
                                    .child(
                                        div()
                                            .w(px(250.0))
                                            .child(
                                                NavigationMenu::new()
                                                    .orientation(NavigationMenuOrientation::Vertical)
                                                    .items(vec![
                                                        NavigationMenuItem::new("item1", "Item 1").with_icon("circle"),
                                                        NavigationMenuItem::new("item2", "Item 2").with_icon("square"),
                                                    ])
                                                    .p_4()  // ← Styled trait
                                            )
                                    )
                            )
                            .child(
                                VStack::new()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(14.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("With p_8()")
                                    )
                                    .child(
                                        div()
                                            .w(px(250.0))
                                            .child(
                                                NavigationMenu::new()
                                                    .orientation(NavigationMenuOrientation::Vertical)
                                                    .items(vec![
                                                        NavigationMenuItem::new("item3", "Item 3").with_icon("circle"),
                                                        NavigationMenuItem::new("item4", "Item 4").with_icon("square"),
                                                    ])
                                                    .p_8()  // ← Styled trait
                                            )
                                    )
                            )
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
                        HStack::new()
                            .gap(px(16.0))
                            .child(
                                VStack::new()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(14.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("Dark Blue Background")
                                    )
                                    .child(
                                        div()
                                            .w(px(250.0))
                                            .child(
                                                NavigationMenu::new()
                                                    .orientation(NavigationMenuOrientation::Vertical)
                                                    .items(vec![
                                                        NavigationMenuItem::new("nav1", "Navigation 1").with_icon("home"),
                                                        NavigationMenuItem::new("nav2", "Navigation 2").with_icon("settings"),
                                                    ])
                                                    .bg(rgb(0x1e3a8a))  // ← Styled trait
                                                    .p_4()
                                                    .rounded(px(8.0))  // ← Styled trait
                                            )
                                    )
                            )
                            .child(
                                VStack::new()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(14.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("With Border & Shadow")
                                    )
                                    .child(
                                        div()
                                            .w(px(250.0))
                                            .child(
                                                NavigationMenu::new()
                                                    .orientation(NavigationMenuOrientation::Vertical)
                                                    .items(vec![
                                                        NavigationMenuItem::new("nav3", "Navigation 3").with_icon("home"),
                                                        NavigationMenuItem::new("nav4", "Navigation 4").with_icon("settings"),
                                                    ])
                                                    .border_2()  // ← Styled trait
                                                    .border_color(rgb(0x3b82f6))
                                                    .rounded(px(12.0))  // ← Styled trait
                                                    .shadow_md()  // ← Styled trait
                                                    .p_4()
                                            )
                                    )
                            )
                    )
            )
            // 4. Horizontal Menu with Custom Styling
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("4. Horizontal Menu with Custom Styling")
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Default Horizontal")
                            )
                            .child(
                                NavigationMenu::new()
                                    .orientation(NavigationMenuOrientation::Horizontal)
                                    .items(vec![
                                        NavigationMenuItem::new("home", "Home").with_icon("home"),
                                        NavigationMenuItem::new("about", "About").with_icon("info"),
                                        NavigationMenuItem::new("products", "Products")
                                            .with_icon("package")
                                            .with_children(vec![
                                                NavigationMenuItem::new("electronics", "Electronics"),
                                                NavigationMenuItem::new("clothing", "Clothing"),
                                            ]),
                                        NavigationMenuItem::new("contact", "Contact").with_icon("mail"),
                                    ])
                            )
                            .child(
                                div()
                                    .mt(px(16.0))
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Custom Horizontal with Background, Padding & Rounded Corners")
                            )
                            .child(
                                NavigationMenu::new()
                                    .orientation(NavigationMenuOrientation::Horizontal)
                                    .items(vec![
                                        NavigationMenuItem::new("home2", "Home").with_icon("home"),
                                        NavigationMenuItem::new("about2", "About").with_icon("info"),
                                        NavigationMenuItem::new("contact2", "Contact").with_icon("mail"),
                                    ])
                                    .bg(rgb(0x1f2937))  // ← Styled trait
                                    .p(px(12.0))  // ← Styled trait
                                    .rounded(px(16.0))  // ← Styled trait
                                    .border_1()  // ← Styled trait
                                    .border_color(rgb(0x374151))
                            )
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
                        VStack::new()
                            .gap(px(12.0))
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Full Width Menu")
                            )
                            .child(
                                NavigationMenu::new()
                                    .orientation(NavigationMenuOrientation::Vertical)
                                    .items(vec![
                                        NavigationMenuItem::new("fw1", "Full Width Item 1").with_icon("folder"),
                                        NavigationMenuItem::new("fw2", "Full Width Item 2").with_icon("file"),
                                    ])
                                    .w_full()  // ← Styled trait
                                    .p_4()
                                    .border_1()
                                    .border_color(theme.tokens.border)
                                    .rounded(px(8.0))
                            )
                            .child(
                                div()
                                    .mt(px(8.0))
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Fixed Width (400px)")
                            )
                            .child(
                                NavigationMenu::new()
                                    .orientation(NavigationMenuOrientation::Vertical)
                                    .items(vec![
                                        NavigationMenuItem::new("fixed1", "Fixed Width Item 1").with_icon("folder"),
                                        NavigationMenuItem::new("fixed2", "Fixed Width Item 2").with_icon("file"),
                                    ])
                                    .w(px(400.0))  // ← Styled trait
                                    .p_4()
                                    .border_1()
                                    .border_color(theme.tokens.border)
                                    .rounded(px(8.0))
                            )
                    )
            )
            // 6. Combined Custom Styling
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("6. Combined Custom Styling (Multiple Styled Trait Methods)")
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Sidebar-style Navigation with Full Customization")
                            )
                            .child(
                                div()
                                    .w(px(300.0))
                                    .child(
                                        NavigationMenu::new()
                                            .orientation(NavigationMenuOrientation::Vertical)
                                            .items(vec![
                                                NavigationMenuItem::new("custom-1", "Dashboard")
                                                    .with_icon("layout-dashboard"),
                                                NavigationMenuItem::new("group-1", "Projects")
                                                    .with_icon("folder")
                                                    .with_children(vec![
                                                        NavigationMenuItem::new("proj-1", "Web App"),
                                                        NavigationMenuItem::new("proj-2", "Mobile App"),
                                                        NavigationMenuItem::new("proj-3", "Desktop App"),
                                                    ]),
                                                NavigationMenuItem::new("custom-2", "Analytics")
                                                    .with_icon("bar-chart"),
                                                NavigationMenuItem::new("custom-3", "Settings")
                                                    .with_icon("settings"),
                                            ])
                                            // All the styling below uses the Styled trait
                                            .bg(rgb(0x18181b))  // ← Styled trait
                                            .p(px(16.0))  // ← Styled trait
                                            .rounded(px(16.0))  // ← Styled trait
                                            .border_2()  // ← Styled trait
                                            .border_color(rgb(0x27272a))
                                            .shadow_lg()  // ← Styled trait
                                            .w(px(300.0))  // ← Styled trait
                                    )
                            )
                            .child(
                                div()
                                    .mt(px(16.0))
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Compact Horizontal Nav Bar")
                            )
                            .child(
                                NavigationMenu::new()
                                    .orientation(NavigationMenuOrientation::Horizontal)
                                    .items(vec![
                                        NavigationMenuItem::new("nav-home", "Home").with_icon("home"),
                                        NavigationMenuItem::new("nav-explore", "Explore").with_icon("compass"),
                                        NavigationMenuItem::new("nav-library", "Library").with_icon("library"),
                                        NavigationMenuItem::new("nav-profile", "Profile").with_icon("user"),
                                    ])
                                    // All the styling below uses the Styled trait
                                    .w_full()  // ← Styled trait
                                    .bg(rgb(0x6366f1))  // ← Styled trait
                                    .px(px(24.0))  // ← Styled trait
                                    .py(px(12.0))  // ← Styled trait
                                    .rounded(px(12.0))  // ← Styled trait
                                    .shadow_md()  // ← Styled trait
                            )
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
                            .child("Methods used: .p(), .p_4(), .p_8(), .px(), .py(), .bg(), .border_1(), .border_2(), .border_color(), .rounded(), .w_full(), .w(), .shadow_md(), .shadow_lg()")
                    )
            )
                )
            )
    }
}
