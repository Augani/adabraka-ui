use gpui::*;
use adabraka_ui::{
    components::search_input::{SearchInput, SearchFilter},
    layout::VStack,
    theme::use_theme,
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

actions!(search_input_styled_demo, [Quit]);

struct SearchInputStyledDemo {
    search1: Entity<SearchInput>,
    search2: Entity<SearchInput>,
    search3: Entity<SearchInput>,
    search4: Entity<SearchInput>,
    search5: Entity<SearchInput>,
}

impl SearchInputStyledDemo {
    fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            // Default SearchInput
            search1: cx.new(|cx| SearchInput::new(cx)),

            // SearchInput with custom padding via Styled trait
            search2: cx.new(|cx| {
                SearchInput::new(cx)
                    .p(px(16.0))  // Styled trait method - adds extra padding
            }),

            // SearchInput with custom background and border via Styled trait
            search3: cx.new(|cx| {
                SearchInput::new(cx)
                    .bg(hsla(220.0 / 360.0, 0.7, 0.25, 0.3))  // Styled trait
                    .border_2()  // Styled trait
                    .border_color(rgb(0x3b82f6))
                    .rounded(px(12.0))  // Styled trait
            }),

            // SearchInput with filters and custom styling
            search4: cx.new(|cx| {
                SearchInput::new(cx)
                    .filters(vec![
                        SearchFilter::new("*.rs", "rs"),
                        SearchFilter::new("*.toml", "toml"),
                        SearchFilter::new("*.md", "md"),
                    ], cx)
                    .p(px(12.0))  // Styled trait
                    .bg(hsla(280.0 / 360.0, 0.7, 0.30, 0.2))  // Styled trait
                    .rounded(px(16.0))  // Styled trait
            }),

            // Fully customized SearchInput with all Styled trait features
            search5: cx.new(|cx| {
                SearchInput::new(cx)
                    .filters(vec![
                        SearchFilter::new("Active", "active"),
                        SearchFilter::new("Completed", "completed"),
                    ], cx)
                    .p(px(20.0))  // Styled trait
                    .bg(hsla(160.0 / 360.0, 0.8, 0.30, 0.2))  // Styled trait
                    .rounded(px(20.0))  // Styled trait
                    .border_2()  // Styled trait
                    .border_color(rgb(0x10b981))
                    .shadow_lg()  // Styled trait
            }),
        }
    }
}

impl Render for SearchInputStyledDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        div()
            .size_full()
            .bg(theme.tokens.background)
            .text_color(theme.tokens.foreground)
            .on_action(cx.listener(|_this, _: &Quit, _window, cx| {
                cx.quit();
            }))
            .child(
                div()
                    .size_full()
                    .overflow_hidden()
                    .child(
                        VStack::new()
                            .p(px(32.0))
                            .gap(px(32.0))
                            // Header
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(32.0))
                                            .font_weight(FontWeight::BOLD)
                                            .child("SearchInput Styled Trait Demo")
                                    )
                                    .child(
                                        div()
                                            .text_size(px(16.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("Demonstrating full GPUI styling control via Styled trait")
                                    )
                            )
                            // 1. Default SearchInput
                            .child(
                                VStack::new()
                                    .gap(px(16.0))
                                    .child(
                                        div()
                                            .text_size(px(18.0))
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .child("1. Default SearchInput")
                                    )
                                    .child(self.search1.clone())
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("Standard SearchInput with no custom styling")
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
                                            .child("2. Custom Padding via Styled Trait")
                                    )
                                    .child(self.search2.clone())
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("SearchInput with .p(px(16.0)) for extra padding")
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
                                            .child("3. Custom Background, Border & Radius")
                                    )
                                    .child(self.search3.clone())
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("Blue themed with .bg(), .border_2(), .border_color(), and .rounded()")
                                    )
                            )
                            // 4. With Filters
                            .child(
                                VStack::new()
                                    .gap(px(16.0))
                                    .child(
                                        div()
                                            .text_size(px(18.0))
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .child("4. SearchInput with Filters + Custom Styling")
                                    )
                                    .child(self.search4.clone())
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("Purple themed SearchInput with file type filters, padding, and rounded corners")
                                    )
                            )
                            // 5. Fully Customized
                            .child(
                                VStack::new()
                                    .gap(px(16.0))
                                    .child(
                                        div()
                                            .text_size(px(18.0))
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .child("5. Fully Customized SearchInput")
                                    )
                                    .child(self.search5.clone())
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("Green themed with all Styled trait features: padding, background, border, shadow, and large radius")
                                    )
                            )
                            // Info Box
                            .child(
                                div()
                                    .mt(px(16.0))
                                    .p(px(16.0))
                                    .bg(theme.tokens.accent)
                                    .rounded(px(8.0))
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(14.0))
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(theme.tokens.accent_foreground)
                                            .child("Styled Trait Power")
                                    )
                                    .child(
                                        div()
                                            .text_size(px(14.0))
                                            .text_color(theme.tokens.accent_foreground)
                                            .child("All customizations use the Styled trait for full GPUI styling control!")
                                    )
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .text_color(theme.tokens.accent_foreground)
                                            .child("Methods used: .p(), .bg(), .rounded(), .border_2(), .border_color(), .shadow_lg()")
                                    )
                                    .child(
                                        div()
                                            .mt(px(4.0))
                                            .text_size(px(12.0))
                                            .text_color(theme.tokens.accent_foreground)
                                            .child("The Styled trait merges your custom styles with component defaults using .refine()")
                                    )
                            )
                    )
            )
    }
}

fn main() {
    Application::new()
        .with_assets(Assets {
            base: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
        })
        .run(move |cx: &mut App| {
            // Install dark theme
            adabraka_ui::theme::install_theme(cx, adabraka_ui::theme::Theme::dark());

            // Initialize UI system
            adabraka_ui::init(cx);
            adabraka_ui::set_icon_base_path("assets/icons");

            // Set up actions
            cx.on_action(|_: &Quit, cx| cx.quit());

            cx.bind_keys([
                KeyBinding::new("cmd-q", Quit, None),
            ]);
            cx.activate(true);

            // Create window
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                        None,
                        size(px(900.0), px(1000.0)),
                        cx,
                    ))),
                    titlebar: Some(TitlebarOptions {
                        title: Some("SearchInput Styled Trait Demo".into()),
                        appears_transparent: false,
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                |window, cx| cx.new(|cx| SearchInputStyledDemo::new(window, cx)),
            )
            .unwrap();
        });
}
