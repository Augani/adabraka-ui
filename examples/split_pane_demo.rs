use adabraka_ui::{
    components::scrollable::scrollable_vertical,
    components::split_pane::{CollapsiblePane, SplitDirection, SplitPane, SplitPaneState},
    prelude::*,
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
                        title: Some("SplitPane Component Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(1200.0), px(900.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|cx| SplitPaneDemo::new(cx)),
            )
            .unwrap();
        });
}

struct SplitPaneDemo {
    horizontal_split: Entity<SplitPaneState>,
    vertical_split: Entity<SplitPaneState>,
    nested_outer: Entity<SplitPaneState>,
    nested_inner: Entity<SplitPaneState>,
    constrained_split: Entity<SplitPaneState>,
    collapsible_split: Entity<SplitPaneState>,
}

impl SplitPaneDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        let horizontal_split = cx.new(|cx| {
            let mut state = SplitPaneState::new(cx);
            state.set_ratio(0.3, cx);
            state
        });

        let vertical_split = cx.new(|cx| {
            let mut state = SplitPaneState::new(cx);
            state.set_direction(SplitDirection::Vertical, cx);
            state.set_ratio(0.4, cx);
            state
        });

        let nested_outer = cx.new(|cx| {
            let mut state = SplitPaneState::new(cx);
            state.set_ratio(0.25, cx);
            state
        });

        let nested_inner = cx.new(|cx| {
            let mut state = SplitPaneState::new(cx);
            state.set_direction(SplitDirection::Vertical, cx);
            state.set_ratio(0.5, cx);
            state
        });

        let constrained_split = cx.new(|cx| {
            let mut state = SplitPaneState::new(cx);
            state.set_ratio(0.5, cx);
            state.set_min_first(100.0, cx);
            state.set_max_first(400.0, cx);
            state.set_min_second(150.0, cx);
            state
        });

        let collapsible_split = cx.new(|cx| {
            let mut state = SplitPaneState::new(cx);
            state.set_ratio(0.3, cx);
            state
        });

        Self {
            horizontal_split,
            vertical_split,
            nested_outer,
            nested_inner,
            constrained_split,
            collapsible_split,
        }
    }
}

impl Render for SplitPaneDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        div()
            .size_full()
            .bg(theme.tokens.background)
            .overflow_hidden()
            .child(scrollable_vertical(
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
                                    .child("SplitPane Component Demo"),
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Resizable split pane layouts with drag handles"),
                            ),
                    )
                    .child(self.render_horizontal_split(&theme))
                    .child(self.render_vertical_split(&theme))
                    .child(self.render_nested_splits(&theme))
                    .child(self.render_constrained_split(&theme))
                    .child(self.render_collapsible_split(&theme))
                    .child(self.render_info_box(&theme)),
            ))
    }
}

impl SplitPaneDemo {
    fn render_horizontal_split(&self, theme: &Theme) -> impl IntoElement {
        VStack::new()
            .gap(px(16.0))
            .child(
                div()
                    .text_size(px(20.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .child("1. Horizontal Split"),
            )
            .child(
                div()
                    .text_size(px(14.0))
                    .text_color(theme.tokens.muted_foreground)
                    .child("Drag the divider to resize panes horizontally"),
            )
            .child(
                div()
                    .h(px(250.0))
                    .border_1()
                    .border_color(theme.tokens.border)
                    .rounded(theme.tokens.radius_md)
                    .overflow_hidden()
                    .child(
                        SplitPane::horizontal(self.horizontal_split.clone())
                            .first(
                                self.render_panel("Left Panel", theme.tokens.primary.opacity(0.2)),
                            )
                            .second(
                                self.render_panel(
                                    "Right Panel",
                                    theme.tokens.secondary.opacity(0.2),
                                ),
                            )
                            .on_resize(|ratio, _, _| {
                                println!("Horizontal split ratio: {:.2}", ratio);
                            }),
                    ),
            )
    }

    fn render_vertical_split(&self, theme: &Theme) -> impl IntoElement {
        VStack::new()
            .gap(px(16.0))
            .child(
                div()
                    .text_size(px(20.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .child("2. Vertical Split"),
            )
            .child(
                div()
                    .text_size(px(14.0))
                    .text_color(theme.tokens.muted_foreground)
                    .child("Drag the divider to resize panes vertically"),
            )
            .child(
                div()
                    .h(px(300.0))
                    .border_1()
                    .border_color(theme.tokens.border)
                    .rounded(theme.tokens.radius_md)
                    .overflow_hidden()
                    .child(
                        SplitPane::vertical(self.vertical_split.clone())
                            .first(self.render_panel("Top Panel", theme.tokens.accent.opacity(0.3)))
                            .second(
                                self.render_panel("Bottom Panel", theme.tokens.muted.opacity(0.5)),
                            )
                            .on_resize(|ratio, _, _| {
                                println!("Vertical split ratio: {:.2}", ratio);
                            }),
                    ),
            )
    }

    fn render_nested_splits(&self, theme: &Theme) -> impl IntoElement {
        VStack::new()
            .gap(px(16.0))
            .child(
                div()
                    .text_size(px(20.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .child("3. Nested Splits"),
            )
            .child(
                div()
                    .text_size(px(14.0))
                    .text_color(theme.tokens.muted_foreground)
                    .child("Complex layouts with nested split panes"),
            )
            .child(
                div()
                    .h(px(350.0))
                    .border_1()
                    .border_color(theme.tokens.border)
                    .rounded(theme.tokens.radius_md)
                    .overflow_hidden()
                    .child(
                        SplitPane::horizontal(self.nested_outer.clone())
                            .first(self.render_panel("Sidebar", theme.tokens.primary.opacity(0.15)))
                            .second(
                                SplitPane::vertical(self.nested_inner.clone())
                                    .first(self.render_panel(
                                        "Main Content",
                                        theme.tokens.secondary.opacity(0.15),
                                    ))
                                    .second(self.render_panel(
                                        "Console",
                                        theme.tokens.accent.opacity(0.15),
                                    )),
                            ),
                    ),
            )
    }

    fn render_constrained_split(&self, theme: &Theme) -> impl IntoElement {
        VStack::new()
            .gap(px(16.0))
            .child(
                div()
                    .text_size(px(20.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .child("4. Constrained Split"),
            )
            .child(
                div()
                    .text_size(px(14.0))
                    .text_color(theme.tokens.muted_foreground)
                    .child("Min/max size constraints: Left 100-400px, Right min 150px"),
            )
            .child(
                div()
                    .h(px(200.0))
                    .border_1()
                    .border_color(theme.tokens.border)
                    .rounded(theme.tokens.radius_md)
                    .overflow_hidden()
                    .child(
                        SplitPane::horizontal(self.constrained_split.clone())
                            .first(self.render_panel(
                                "Constrained (100-400px)",
                                theme.tokens.destructive.opacity(0.2),
                            ))
                            .second(
                                self.render_panel("Min 150px", theme.tokens.primary.opacity(0.2)),
                            ),
                    ),
            )
    }

    fn render_collapsible_split(&self, theme: &Theme) -> impl IntoElement {
        VStack::new()
            .gap(px(16.0))
            .child(
                div()
                    .text_size(px(20.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .child("5. Collapsible Split"),
            )
            .child(
                div()
                    .text_size(px(14.0))
                    .text_color(theme.tokens.muted_foreground)
                    .child("Click the collapse buttons to hide/show panes"),
            )
            .child(
                div()
                    .flex()
                    .gap(px(12.0))
                    .child(self.render_collapse_button(CollapsiblePane::First, theme))
                    .child(self.render_collapse_button(CollapsiblePane::Second, theme))
                    .child(self.render_expand_button(theme)),
            )
            .child(
                div()
                    .h(px(250.0))
                    .border_1()
                    .border_color(theme.tokens.border)
                    .rounded(theme.tokens.radius_md)
                    .overflow_hidden()
                    .child(
                        SplitPane::horizontal(self.collapsible_split.clone())
                            .first(
                                self.render_panel(
                                    "Collapsible Left",
                                    theme.tokens.accent.opacity(0.2),
                                ),
                            )
                            .second(self.render_panel(
                                "Collapsible Right",
                                theme.tokens.secondary.opacity(0.2),
                            ))
                            .show_collapse_buttons(true),
                    ),
            )
    }

    fn render_collapse_button(&self, pane: CollapsiblePane, theme: &Theme) -> impl IntoElement {
        let state = self.collapsible_split.clone();
        let label = match pane {
            CollapsiblePane::First => "Collapse Left",
            CollapsiblePane::Second => "Collapse Right",
        };

        div()
            .id(match pane {
                CollapsiblePane::First => "collapse-left-btn",
                CollapsiblePane::Second => "collapse-right-btn",
            })
            .px(px(12.0))
            .py(px(8.0))
            .bg(theme.tokens.muted)
            .hover(|s| s.bg(theme.tokens.accent))
            .rounded(theme.tokens.radius_md)
            .cursor_pointer()
            .text_size(px(13.0))
            .child(label)
            .on_click(move |_, _, cx| {
                state.update(cx, |state: &mut SplitPaneState, cx| {
                    state.collapse(pane, cx);
                });
            })
    }

    fn render_expand_button(&self, theme: &Theme) -> impl IntoElement {
        let state = self.collapsible_split.clone();

        div()
            .id("expand-btn")
            .px(px(12.0))
            .py(px(8.0))
            .bg(theme.tokens.primary)
            .hover(|s| s.bg(theme.tokens.primary.opacity(0.8)))
            .rounded(theme.tokens.radius_md)
            .cursor_pointer()
            .text_size(px(13.0))
            .text_color(theme.tokens.primary_foreground)
            .child("Expand All")
            .on_click(move |_, _, cx| {
                state.update(cx, |state: &mut SplitPaneState, cx| {
                    state.expand(cx);
                });
            })
    }

    fn render_panel(&self, title: &str, bg_color: Hsla) -> impl IntoElement {
        let theme = use_theme();

        div()
            .size_full()
            .bg(bg_color)
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .text_size(px(16.0))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(theme.tokens.foreground)
                    .child(title.to_string()),
            )
    }

    fn render_info_box(&self, theme: &Theme) -> impl IntoElement {
        div()
            .mt(px(16.0))
            .p(px(16.0))
            .bg(theme.tokens.accent)
            .rounded(px(8.0))
            .child(
                div()
                    .text_size(px(14.0))
                    .text_color(theme.tokens.accent_foreground)
                    .child("SplitPane Features:"),
            )
            .child(
                div()
                    .mt(px(8.0))
                    .text_size(px(12.0))
                    .text_color(theme.tokens.accent_foreground)
                    .child(
                        "- Horizontal and vertical split orientations\n\
                         - Draggable dividers for resizing\n\
                         - Min/max size constraints per pane\n\
                         - Collapsible panes with expand buttons\n\
                         - Nested splits for complex layouts\n\
                         - on_resize callback for tracking changes",
                    ),
            )
    }
}
