use adabraka_ui::{
    prelude::*,
    components::{
        select::{Select, SelectOption},
        scrollable::scrollable_vertical,
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
                        title: Some("Select Styled Trait Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(900.0), px(800.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|cx| SelectStyledDemo::new(cx)),
            )
            .unwrap();
        });
}

struct SelectStyledDemo {
    select_default: Entity<Select<String>>,
    select_wide: Entity<Select<String>>,
    select_rounded: Entity<Select<String>>,
    select_custom_bg: Entity<Select<String>>,
    select_bordered: Entity<Select<String>>,
    select_shadowed: Entity<Select<String>>,
    select_combined: Entity<Select<String>>,
}

impl SelectStyledDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            select_default: cx.new(|cx| {
                Select::new(cx)
                    .options(vec![
                        SelectOption::new("option1".to_string(), "Default Style"),
                        SelectOption::new("option2".to_string(), "No Custom Styling"),
                        SelectOption::new("option3".to_string(), "Standard Look"),
                    ])
                    .placeholder("Default Select")
            }),
            select_wide: cx.new(|cx| {
                Select::new(cx)
                    .options(vec![
                        SelectOption::new("option1".to_string(), "Wide Option 1"),
                        SelectOption::new("option2".to_string(), "Wide Option 2"),
                        SelectOption::new("option3".to_string(), "Wide Option 3"),
                    ])
                    .placeholder("Full Width Select")
                    .w_full()  // <- Styled trait
            }),
            select_rounded: cx.new(|cx| {
                Select::new(cx)
                    .options(vec![
                        SelectOption::new("option1".to_string(), "Rounded Option 1"),
                        SelectOption::new("option2".to_string(), "Rounded Option 2"),
                        SelectOption::new("option3".to_string(), "Rounded Option 3"),
                    ])
                    .placeholder("Custom Rounded")
                    .rounded(px(16.0))  // <- Styled trait
            }),
            select_custom_bg: cx.new(|cx| {
                Select::new(cx)
                    .options(vec![
                        SelectOption::new("option1".to_string(), "Blue Background"),
                        SelectOption::new("option2".to_string(), "Custom Color"),
                        SelectOption::new("option3".to_string(), "Styled Theme"),
                    ])
                    .placeholder("Custom Background")
                    .bg(rgb(0x1e3a8a))  // <- Styled trait
                    .text_color(gpui::white())
            }),
            select_bordered: cx.new(|cx| {
                Select::new(cx)
                    .options(vec![
                        SelectOption::new("option1".to_string(), "Thick Border"),
                        SelectOption::new("option2".to_string(), "Custom Border"),
                        SelectOption::new("option3".to_string(), "Styled Border"),
                    ])
                    .placeholder("Custom Border")
                    .border_2()  // <- Styled trait
                    .border_color(rgb(0x8b5cf6))
            }),
            select_shadowed: cx.new(|cx| {
                Select::new(cx)
                    .options(vec![
                        SelectOption::new("option1".to_string(), "Shadow Option 1"),
                        SelectOption::new("option2".to_string(), "Shadow Option 2"),
                        SelectOption::new("option3".to_string(), "Shadow Option 3"),
                    ])
                    .placeholder("With Shadow")
                    .shadow_lg()  // <- Styled trait
            }),
            select_combined: cx.new(|cx| {
                Select::new(cx)
                    .options(vec![
                        SelectOption::new("option1".to_string(), "Combined Option 1"),
                        SelectOption::new("option2".to_string(), "Combined Option 2"),
                        SelectOption::new("option3".to_string(), "Combined Option 3"),
                    ])
                    .placeholder("Ultra Custom Select")
                    .w_full()  // <- Styled trait
                    .p(px(16.0))  // <- Styled trait
                    .bg(rgb(0x059669))  // <- Styled trait
                    .text_color(gpui::white())
                    .rounded(px(12.0))  // <- Styled trait
                    .border_2()  // <- Styled trait
                    .border_color(rgb(0x10b981))
                    .shadow_md()  // <- Styled trait
            }),
        }
    }
}

impl Render for SelectStyledDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
                                        .child("Select Styled Trait Customization Demo")
                                )
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child("Demonstrating full GPUI styling control via Styled trait for Select component")
                                )
                        )
                        // 1. Default vs Full Width
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("1. Width Control")
                                )
                                .child(
                                    VStack::new()
                                        .gap(px(12.0))
                                        .child(
                                            div()
                                                .text_size(px(13.0))
                                                .text_color(theme.tokens.muted_foreground)
                                                .child("Default width:")
                                        )
                                        .child(self.select_default.clone())
                                        .child(
                                            div()
                                                .mt(px(12.0))
                                                .text_size(px(13.0))
                                                .text_color(theme.tokens.muted_foreground)
                                                .child("Full width using .w_full():")
                                        )
                                        .child(self.select_wide.clone())
                                )
                        )
                        // 2. Custom Border Radius
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("2. Custom Border Radius")
                                )
                                .child(
                                    VStack::new()
                                        .gap(px(12.0))
                                        .child(
                                            div()
                                                .text_size(px(13.0))
                                                .text_color(theme.tokens.muted_foreground)
                                                .child("Custom rounded corners using .rounded(px(16.0)):")
                                        )
                                        .child(self.select_rounded.clone())
                                )
                        )
                        // 3. Custom Background Color
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("3. Custom Background Color")
                                )
                                .child(
                                    VStack::new()
                                        .gap(px(12.0))
                                        .child(
                                            div()
                                                .text_size(px(13.0))
                                                .text_color(theme.tokens.muted_foreground)
                                                .child("Blue background using .bg(rgb(0x1e3a8a)):")
                                        )
                                        .child(self.select_custom_bg.clone())
                                )
                        )
                        // 4. Custom Border
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("4. Custom Border")
                                )
                                .child(
                                    VStack::new()
                                        .gap(px(12.0))
                                        .child(
                                            div()
                                                .text_size(px(13.0))
                                                .text_color(theme.tokens.muted_foreground)
                                                .child("Purple border using .border_2() and .border_color():")
                                        )
                                        .child(self.select_bordered.clone())
                                )
                        )
                        // 5. Shadow Effects
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("5. Shadow Effects")
                                )
                                .child(
                                    VStack::new()
                                        .gap(px(12.0))
                                        .child(
                                            div()
                                                .text_size(px(13.0))
                                                .text_color(theme.tokens.muted_foreground)
                                                .child("Large shadow using .shadow_lg():")
                                        )
                                        .child(self.select_shadowed.clone())
                                )
                        )
                        // 6. Combined Styling
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("6. Combined Styling (Multiple Styled Trait Methods)")
                                )
                                .child(
                                    VStack::new()
                                        .gap(px(12.0))
                                        .child(
                                            div()
                                                .text_size(px(13.0))
                                                .text_color(theme.tokens.muted_foreground)
                                                .child("All customizations combined:")
                                        )
                                        .child(self.select_combined.clone())
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
                                        .child("Methods used: .w_full(), .p(), .bg(), .text_color(), .rounded(), .border_2(), .border_color(), .shadow_lg(), .shadow_md()")
                                )
                        )
                )
            )
    }
}
