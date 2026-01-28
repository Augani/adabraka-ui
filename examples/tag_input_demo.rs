use adabraka_ui::{
    components::tag_input::{TagInput, TagInputState},
    layout::VStack,
    theme::{install_theme, use_theme, Theme},
};
use gpui::{prelude::FluentBuilder as _, *};

struct TagInputDemo {
    basic_tags: Entity<TagInputState>,
    limited_tags: Entity<TagInputState>,
    prefilled_tags: Entity<TagInputState>,
    tag_changes: Vec<String>,
}

impl TagInputDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        let basic_tags = cx.new(|cx| TagInputState::new(cx));
        let limited_tags = cx.new(|cx| {
            let mut state = TagInputState::new(cx);
            state.set_max_tags(Some(5), cx);
            state
        });
        let prefilled_tags = cx.new(|cx| TagInputState::with_tags(cx, vec!["rust", "gpui", "ui"]));
        Self {
            basic_tags,
            limited_tags,
            prefilled_tags,
            tag_changes: Vec::new(),
        }
    }
}

impl Render for TagInputDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();
        let entity = cx.entity().clone();

        div()
            .size_full()
            .bg(theme.tokens.background)
            .text_color(theme.tokens.foreground)
            .font_family(theme.tokens.font_family.clone())
            .child(
                VStack::new()
                    .p(px(24.0))
                    .gap(px(24.0))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .text_size(px(32.0))
                                    .font_weight(FontWeight::BOLD)
                                    .child("Tag Input Demo"),
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Add and remove tags with keyboard support (Enter to add, Backspace to remove)"),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(24.0))
                            .child({
                                let entity = entity.clone();
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(14.0))
                                            .font_weight(FontWeight::MEDIUM)
                                            .child("Basic Tag Input"),
                                    )
                                    .child(
                                        TagInput::new(self.basic_tags.clone())
                                            .placeholder("Add tags...")
                                            .on_change({
                                                move |tags, _, cx| {
                                                    entity.update(cx, |this, cx| {
                                                        this.tag_changes.push(format!(
                                                            "Basic: {} tags",
                                                            tags.len()
                                                        ));
                                                        if this.tag_changes.len() > 5 {
                                                            this.tag_changes.remove(0);
                                                        }
                                                        cx.notify();
                                                    });
                                                }
                                            }),
                                    )
                            })
                            .child({
                                let entity = entity.clone();
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(14.0))
                                            .font_weight(FontWeight::MEDIUM)
                                            .child("Limited to 5 Tags"),
                                    )
                                    .child(
                                        TagInput::new(self.limited_tags.clone())
                                            .placeholder("Max 5 tags...")
                                            .on_change({
                                                move |tags, _, cx| {
                                                    entity.update(cx, |this, cx| {
                                                        this.tag_changes.push(format!(
                                                            "Limited: {}/5 tags",
                                                            tags.len()
                                                        ));
                                                        if this.tag_changes.len() > 5 {
                                                            this.tag_changes.remove(0);
                                                        }
                                                        cx.notify();
                                                    });
                                                }
                                            }),
                                    )
                            })
                            .child({
                                let entity = entity.clone();
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(14.0))
                                            .font_weight(FontWeight::MEDIUM)
                                            .child("Pre-filled Tags"),
                                    )
                                    .child(
                                        TagInput::new(self.prefilled_tags.clone())
                                            .placeholder("Add more tags...")
                                            .on_change({
                                                move |tags, _, cx| {
                                                    entity.update(cx, |this, cx| {
                                                        this.tag_changes.push(format!(
                                                            "Prefilled: {} tags",
                                                            tags.len()
                                                        ));
                                                        if this.tag_changes.len() > 5 {
                                                            this.tag_changes.remove(0);
                                                        }
                                                        cx.notify();
                                                    });
                                                }
                                            }),
                                    )
                            }),
                    )
                    .when(!self.tag_changes.is_empty(), |d| {
                        d.child(
                            div()
                                .mt(px(16.0))
                                .p(px(12.0))
                                .bg(theme.tokens.muted)
                                .rounded(theme.tokens.radius_md)
                                .flex()
                                .flex_col()
                                .gap(px(4.0))
                                .child(
                                    div()
                                        .text_size(px(12.0))
                                        .font_weight(FontWeight::MEDIUM)
                                        .child("Recent Changes:"),
                                )
                                .children(self.tag_changes.iter().map(|change| {
                                    div()
                                        .text_size(px(12.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child(change.clone())
                                })),
                        )
                    }),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        adabraka_ui::init(cx);
        install_theme(cx, Theme::dark());

        let bounds = Bounds::centered(None, size(px(600.0), px(550.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("Tag Input Demo".into()),
                    appears_transparent: false,
                    traffic_light_position: None,
                }),
                ..Default::default()
            },
            |_, cx| cx.new(|cx| TagInputDemo::new(cx)),
        )
        .unwrap();

        cx.activate(true);
    });
}
