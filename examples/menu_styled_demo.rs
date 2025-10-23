use adabraka_ui::{
    prelude::*,
    components::scrollable::scrollable_vertical,
    navigation::menu::{Menu, MenuItem},
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
                        title: Some("Menu Styled Trait Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(1200.0), px(900.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|_| MenuStyledDemo::new()),
            )
            .unwrap();
        });
}

struct MenuStyledDemo;

impl MenuStyledDemo {
    fn new() -> Self {
        Self
    }

    fn sample_menu_items() -> Vec<MenuItem> {
        vec![
            MenuItem::new("new", "New File")
                .with_icon("file-plus")
                .with_shortcut("Cmd+N")
                .on_click(|_, _| println!("New file clicked")),
            MenuItem::new("open", "Open...")
                .with_icon("folder-open")
                .with_shortcut("Cmd+O")
                .on_click(|_, _| println!("Open clicked")),
            MenuItem::separator(),
            MenuItem::new("save", "Save")
                .with_icon("save")
                .with_shortcut("Cmd+S")
                .on_click(|_, _| println!("Save clicked")),
            MenuItem::new("save-as", "Save As...")
                .with_shortcut("Cmd+Shift+S")
                .on_click(|_, _| println!("Save As clicked")),
        ]
    }

    fn sample_menu_items_with_checkbox() -> Vec<MenuItem> {
        vec![
            MenuItem::checkbox("line-numbers", "Show Line Numbers", true)
                .on_click(|_, _| println!("Toggle line numbers")),
            MenuItem::checkbox("minimap", "Show Minimap", false)
                .on_click(|_, _| println!("Toggle minimap")),
            MenuItem::separator(),
            MenuItem::new("preferences", "Preferences...")
                .with_icon("settings")
                .with_shortcut("Cmd+,")
                .on_click(|_, _| println!("Preferences clicked")),
        ]
    }

    fn compact_menu_items() -> Vec<MenuItem> {
        vec![
            MenuItem::new("cut", "Cut")
                .with_shortcut("Cmd+X"),
            MenuItem::new("copy", "Copy")
                .with_shortcut("Cmd+C"),
            MenuItem::new("paste", "Paste")
                .with_shortcut("Cmd+V"),
        ]
    }
}

impl Render for MenuStyledDemo {
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
                            .child("Menu Styled Trait Customization Demo")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("Demonstrating full GPUI styling control on Menu component via Styled trait")
                    )
            )
            // 1. Default Menu
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("1. Default Menu (No Custom Styling)")
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
                                            .child("Default styling applied")
                                    )
                                    .child(
                                        Menu::new(Self::sample_menu_items())
                                    )
                            )
                    )
            )
            // 2. Custom Background Color
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("2. Custom Background Colors")
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
                                            .child("Blue gradient background")
                                    )
                                    .child(
                                        Menu::new(Self::sample_menu_items())
                                            .bg(rgb(0x1e3a8a))
                                    )
                            )
                            .child(
                                VStack::new()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(14.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("Purple background")
                                    )
                                    .child(
                                        Menu::new(Self::sample_menu_items_with_checkbox())
                                            .bg(rgb(0x581c87))
                                    )
                            )
                            .child(
                                VStack::new()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(14.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("Green background")
                                    )
                                    .child(
                                        Menu::new(Self::compact_menu_items())
                                            .bg(rgb(0x14532d))
                                    )
                            )
                    )
            )
            // 3. Custom Border Styling
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("3. Custom Border Styling")
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
                                            .child("Thick blue border")
                                    )
                                    .child(
                                        Menu::new(Self::sample_menu_items())
                                            .border_4()
                                            .border_color(rgb(0x3b82f6))
                                    )
                            )
                            .child(
                                VStack::new()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(14.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("Red accent border")
                                    )
                                    .child(
                                        Menu::new(Self::sample_menu_items_with_checkbox())
                                            .border_2()
                                            .border_color(rgb(0xef4444))
                                    )
                            )
                    )
            )
            // 4. Custom Border Radius
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("4. Custom Border Radius")
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
                                            .child("Sharp corners (no radius)")
                                    )
                                    .child(
                                        Menu::new(Self::sample_menu_items())
                                            .rounded(px(0.0))
                                    )
                            )
                            .child(
                                VStack::new()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(14.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("Large radius (rounded(24))")
                                    )
                                    .child(
                                        Menu::new(Self::compact_menu_items())
                                            .rounded(px(24.0))
                                    )
                            )
                    )
            )
            // 5. Custom Width Control
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("5. Custom Width Control")
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(
                                VStack::new()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(14.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("Narrow menu with w(px(180))")
                                    )
                                    .child(
                                        Menu::new(Self::compact_menu_items())
                                            .w(px(180.0))
                                    )
                            )
                            .child(
                                VStack::new()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(14.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("Wide menu with w(px(400))")
                                    )
                                    .child(
                                        Menu::new(Self::sample_menu_items())
                                            .w(px(400.0))
                                    )
                            )
                    )
            )
            // 6. Custom Padding
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("6. Custom Padding")
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
                                            .child("No padding (p(0))")
                                    )
                                    .child(
                                        Menu::new(Self::compact_menu_items())
                                            .p(px(0.0))
                                    )
                            )
                            .child(
                                VStack::new()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(14.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("Large padding (p_8)")
                                    )
                                    .child(
                                        Menu::new(Self::compact_menu_items())
                                            .p_8()
                                    )
                            )
                    )
            )
            // 7. Shadow Effects
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("7. Shadow Effects")
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
                                            .child("Shadow small (shadow_sm)")
                                    )
                                    .child(
                                        Menu::new(Self::compact_menu_items())
                                            .shadow_sm()
                                    )
                            )
                            .child(
                                VStack::new()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(14.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("Shadow large (shadow_lg)")
                                    )
                                    .child(
                                        Menu::new(Self::sample_menu_items())
                                            .shadow_lg()
                                    )
                            )
                    )
            )
            // 8. Combined Advanced Styling
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("8. Combined Advanced Styling (Multiple Styled Methods)")
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
                                            .child("Purple card style")
                                    )
                                    .child(
                                        Menu::new(Self::sample_menu_items())
                                            .bg(rgb(0x6b21a8))
                                            .p_8()
                                            .rounded(px(16.0))
                                            .shadow_lg()
                                            .border_2()
                                            .border_color(rgb(0xa855f7))
                                    )
                            )
                            .child(
                                VStack::new()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(14.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("Teal modern design")
                                    )
                                    .child(
                                        Menu::new(Self::sample_menu_items_with_checkbox())
                                            .bg(rgb(0x134e4a))
                                            .p(px(16.0))
                                            .rounded(px(20.0))
                                            .shadow_md()
                                            .border_1()
                                            .border_color(rgb(0x14b8a6))
                                            .w(px(300.0))
                                    )
                            )
                            .child(
                                VStack::new()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(14.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("Compact orange style")
                                    )
                                    .child(
                                        Menu::new(Self::compact_menu_items())
                                            .bg(rgb(0x9a3412))
                                            .px(px(12.0))
                                            .py(px(8.0))
                                            .rounded(px(12.0))
                                            .shadow_sm()
                                            .border_2()
                                            .border_color(rgb(0xf97316))
                                            .w(px(220.0))
                                    )
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
                            .child("All customizations above use the Styled trait for full GPUI styling control on the Menu component!")
                    )
                    .child(
                        div()
                            .mt(px(8.0))
                            .text_size(px(12.0))
                            .text_color(theme.tokens.accent_foreground)
                            .child("Methods used: .bg(), .border_1/2/4(), .border_color(), .rounded(), .w(), .p(), .p_8(), .px(), .py(), .shadow_sm/md/lg()")
                    )
            )
                )
            )
    }
}
