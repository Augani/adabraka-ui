use gpui::{prelude::FluentBuilder as _, *};
use adabraka_ui::{
    components::search_input::{SearchInput, SearchInputState, SearchFilter},
    layout::VStack,
    theme::use_theme,
};

actions!(search_input_demo, [Quit]);

struct SearchInputDemo {
    search: Entity<SearchInput>,
    current_query: String,
    results: Vec<String>,
    filtered_results: Vec<String>,
}

impl SearchInputDemo {
    fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        // Sample data to search through
        let results = vec![
            "main.rs",
            "lib.rs",
            "Cargo.toml",
            "README.md",
            "src/components/button.rs",
            "src/components/input.rs",
            "src/components/checkbox.rs",
            "src/theme/tokens.rs",
            "src/theme/theme.rs",
            "examples/button_demo.rs",
            "examples/input_demo.rs",
            "tests/button_test.rs",
            "tests/input_test.rs",
        ].into_iter().map(String::from).collect::<Vec<_>>();

        let entity = cx.entity().clone();

        let search = cx.new(|cx| {
            SearchInput::new(cx)
                .filters(vec![
                    SearchFilter::new("*.rs", "rs"),
                    SearchFilter::new("*.toml", "toml"),
                    SearchFilter::new("*.md", "md"),
                    SearchFilter::new("src/", "src"),
                    SearchFilter::new("examples/", "examples"),
                    SearchFilter::new("tests/", "tests"),
                ], cx)
                .on_search({
                    let entity = entity.clone();
                    move |query, app_cx: &mut App| {
                        app_cx.update_entity(&entity, |this: &mut SearchInputDemo, cx| {
                            this.handle_search(query, cx);
                        });
                    }
                }, cx)
                .on_filter_toggle({
                    let entity = entity.clone();
                    move |_idx, app_cx: &mut App| {
                        app_cx.update_entity(&entity, |this: &mut SearchInputDemo, cx| {
                            let query = this.current_query.clone();
                            this.handle_search(&query, cx);
                        });
                    }
                }, cx)
        });

        Self {
            search,
            current_query: String::new(),
            filtered_results: results.clone(),
            results,
        }
    }

    fn handle_search(&mut self, query: &str, cx: &mut Context<Self>) {
        self.current_query = query.to_string();

        // Get active filters
        let active_filters: Vec<String> = self.search.read(cx).state().read(cx)
            .active_filters()
            .iter()
            .map(|f| f.value.to_string())
            .collect();

        let case_sensitive = self.search.read(cx).state().read(cx).case_sensitive();

        // Filter results
        self.filtered_results = self.results.iter()
            .filter(|item| {
                // Check if matches query
                let matches_query = if query.is_empty() {
                    true
                } else {
                    let search_term = if case_sensitive {
                        query.to_string()
                    } else {
                        query.to_lowercase()
                    };

                    let item_text = if case_sensitive {
                        item.to_string()
                    } else {
                        item.to_lowercase()
                    };

                    item_text.contains(&search_term)
                };

                // Check if matches any active filter
                let matches_filter = if active_filters.is_empty() {
                    true
                } else {
                    active_filters.iter().any(|filter: &String| {
                        if filter.starts_with("*.") {
                            // Extension filter
                            item.ends_with(&filter[1..])
                        } else {
                            // Path filter
                            item.starts_with(filter)
                        }
                    })
                };

                matches_query && matches_filter
            })
            .cloned()
            .collect();

        // Update results count
        self.search.update(cx, |_search: &mut SearchInput, cx| {
            _search.state().update(cx, |state: &mut SearchInputState, cx| {
                state.set_results_count(Some(self.filtered_results.len()), cx);
            });
        });

        cx.notify();
    }
}

impl Render for SearchInputDemo {
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
                VStack::new()
                    .p(px(32.0))
                    .gap(px(32.0))
                    .size_full()
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
                                    .child("Search Input Demo")
                            )
                            .child(
                                div()
                                    .text_size(px(16.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Advanced search input with filters, case sensitivity, and regex support")
                            )
                    )
                    // Instructions
                    .child(
                        div()
                            .p(px(16.0))
                            .bg(theme.tokens.muted.opacity(0.3))
                            .rounded(theme.tokens.radius_md)
                            .border_1()
                            .border_color(theme.tokens.border)
                            .flex()
                            .flex_col()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .text_size(px(16.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("How to Use")
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .child("• Type in the search box to filter files")
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .child("• Click filter badges (*.rs, *.toml, etc.) to filter by file type or path")
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .child("• Click 'Aa' to toggle case-sensitive search")
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .child("• Click '.*' to toggle regex mode (not implemented in this demo)")
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .child("• Click the X button to clear the search")
                            )
                    )
                    // Search input
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .text_size(px(16.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("Search Files")
                            )
                            .child(self.search.clone())
                    )
                    // Results
                    .child(
                        div()
                            .flex_1()
                            .flex()
                            .flex_col()
                            .gap(px(8.0))
                            .overflow_hidden()
                            .child(
                                div()
                                    .text_size(px(16.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child(format!("Results ({})", self.filtered_results.len()))
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .p(px(16.0))
                                    .bg(theme.tokens.card)
                                    .rounded(theme.tokens.radius_md)
                                    .border_1()
                                    .border_color(theme.tokens.border)
                                    .flex()
                                    .flex_col()
                                    .gap(px(4.0))
                                    .when(self.filtered_results.is_empty(), |parent_div: Div| {
                                        parent_div.child(
                                            div()
                                                .flex()
                                                .items_center()
                                                .justify_center()
                                                .h(px(200.0))
                                                .text_color(theme.tokens.muted_foreground)
                                                .child("No results found")
                                        )
                                    })
                                    .children(self.filtered_results.iter().map(|result| {
                                        div()
                                            .px(px(12.0))
                                            .py(px(8.0))
                                            .rounded(theme.tokens.radius_sm)
                                            .hover(|style| style.bg(theme.tokens.muted))
                                            .cursor(CursorStyle::PointingHand)
                                            .child(
                                                div()
                                                    .text_size(px(14.0))
                                                    .child(result.clone())
                                            )
                                            .into_any_element()
                                    }))
                            )
                    )
                    // Features
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(
                                div()
                                    .text_size(px(20.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("Features Demonstrated")
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .flex()
                                            .gap(px(8.0))
                                            .child(
                                                div()
                                                    .text_color(theme.tokens.accent)
                                                    .child("✓")
                                            )
                                            .child("Real-time search filtering")
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .gap(px(8.0))
                                            .child(
                                                div()
                                                    .text_color(theme.tokens.accent)
                                                    .child("✓")
                                            )
                                            .child("Multiple filter badges (file type, path)")
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .gap(px(8.0))
                                            .child(
                                                div()
                                                    .text_color(theme.tokens.accent)
                                                    .child("✓")
                                            )
                                            .child("Case-sensitive toggle")
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .gap(px(8.0))
                                            .child(
                                                div()
                                                    .text_color(theme.tokens.accent)
                                                    .child("✓")
                                            )
                                            .child("Regex mode toggle")
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .gap(px(8.0))
                                            .child(
                                                div()
                                                    .text_color(theme.tokens.accent)
                                                    .child("✓")
                                            )
                                            .child("Results count display")
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .gap(px(8.0))
                                            .child(
                                                div()
                                                    .text_color(theme.tokens.accent)
                                                    .child("✓")
                                            )
                                            .child("Clear button when text is entered")
                                    )
                            )
                    )
            )
    }
}

fn main() {
    Application::new().run(move |cx: &mut App| {
        // Install dark theme
        adabraka_ui::theme::install_theme(cx, adabraka_ui::theme::Theme::dark());

        // Initialize UI system
        adabraka_ui::init(cx);

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
                    size(px(1000.0), px(800.0)),
                    cx,
                ))),
                titlebar: Some(TitlebarOptions {
                    title: Some("Search Input Demo".into()),
                    appears_transparent: false,
                    ..Default::default()
                }),
                ..Default::default()
            },
            |window, cx| cx.new(|cx| SearchInputDemo::new(window, cx)),
        )
        .unwrap();
    });
}
