use adabraka_ui::{
    components::inline_edit::{
        init as init_inline_edit, InlineEdit, InlineEditState, InlineEditTrigger,
    },
    layout::VStack,
    theme::{install_theme, use_theme, Theme},
};
use gpui::{prelude::FluentBuilder as _, *};

struct InlineEditDemo {
    basic_edit: Entity<InlineEditState>,
    with_placeholder: Entity<InlineEditState>,
    double_click_edit: Entity<InlineEditState>,
    disabled_edit: Entity<InlineEditState>,
    name_field: Entity<InlineEditState>,
    email_field: Entity<InlineEditState>,
    bio_field: Entity<InlineEditState>,
    event_log: Vec<String>,
}

impl InlineEditDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        let basic_edit = cx.new(|cx| InlineEditState::with_value(cx, "Click to edit this text"));
        let with_placeholder = cx.new(|cx| InlineEditState::new(cx));
        let double_click_edit =
            cx.new(|cx| InlineEditState::with_value(cx, "Double-click to edit"));
        let disabled_edit = cx.new(|cx| InlineEditState::with_value(cx, "Cannot edit this"));
        let name_field = cx.new(|cx| InlineEditState::with_value(cx, "John Doe"));
        let email_field = cx.new(|cx| InlineEditState::with_value(cx, "john@example.com"));
        let bio_field = cx.new(|cx| InlineEditState::with_value(cx, "Software developer"));

        Self {
            basic_edit,
            with_placeholder,
            double_click_edit,
            disabled_edit,
            name_field,
            email_field,
            bio_field,
            event_log: Vec::new(),
        }
    }

    fn add_log(&mut self, message: String) {
        self.event_log.push(message);
        if self.event_log.len() > 8 {
            self.event_log.remove(0);
        }
    }
}

impl Render for InlineEditDemo {
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
                    .p(px(32.0))
                    .gap(px(32.0))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .text_size(px(32.0))
                                    .font_weight(FontWeight::BOLD)
                                    .child("Inline Edit Demo"),
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Edit text inline with click or double-click. Press Enter to save, Escape to cancel."),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .gap(px(48.0))
                            .child(
                                div()
                                    .flex_1()
                                    .flex()
                                    .flex_col()
                                    .gap(px(24.0))
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .child(
                                                div()
                                                    .text_size(px(14.0))
                                                    .font_weight(FontWeight::MEDIUM)
                                                    .child("Basic Inline Edit"),
                                            )
                                            .child(
                                                div()
                                                    .text_size(px(12.0))
                                                    .text_color(theme.tokens.muted_foreground)
                                                    .child("Click to start editing"),
                                            )
                                            .child({
                                                let entity = entity.clone();
                                                InlineEdit::new(self.basic_edit.clone())
                                                    .on_save(move |value, _, cx| {
                                                        entity.update(cx, |this, cx| {
                                                            this.add_log(format!(
                                                                "Basic: saved \"{}\"",
                                                                value
                                                            ));
                                                            cx.notify();
                                                        });
                                                    })
                                            }),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .child(
                                                div()
                                                    .text_size(px(14.0))
                                                    .font_weight(FontWeight::MEDIUM)
                                                    .child("With Placeholder"),
                                            )
                                            .child(
                                                div()
                                                    .text_size(px(12.0))
                                                    .text_color(theme.tokens.muted_foreground)
                                                    .child("Shows placeholder when empty"),
                                            )
                                            .child({
                                                let entity = entity.clone();
                                                InlineEdit::new(self.with_placeholder.clone())
                                                    .placeholder("Enter your text here...")
                                                    .on_save(move |value, _, cx| {
                                                        entity.update(cx, |this, cx| {
                                                            this.add_log(format!(
                                                                "Placeholder: saved \"{}\"",
                                                                value
                                                            ));
                                                            cx.notify();
                                                        });
                                                    })
                                            }),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .child(
                                                div()
                                                    .text_size(px(14.0))
                                                    .font_weight(FontWeight::MEDIUM)
                                                    .child("Double-Click to Edit"),
                                            )
                                            .child(
                                                div()
                                                    .text_size(px(12.0))
                                                    .text_color(theme.tokens.muted_foreground)
                                                    .child("Requires double-click to start editing"),
                                            )
                                            .child({
                                                let entity = entity.clone();
                                                InlineEdit::new(self.double_click_edit.clone())
                                                    .trigger(InlineEditTrigger::DoubleClick)
                                                    .on_save(move |value, _, cx| {
                                                        entity.update(cx, |this, cx| {
                                                            this.add_log(format!(
                                                                "DoubleClick: saved \"{}\"",
                                                                value
                                                            ));
                                                            cx.notify();
                                                        });
                                                    })
                                            }),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .child(
                                                div()
                                                    .text_size(px(14.0))
                                                    .font_weight(FontWeight::MEDIUM)
                                                    .child("Disabled"),
                                            )
                                            .child(
                                                div()
                                                    .text_size(px(12.0))
                                                    .text_color(theme.tokens.muted_foreground)
                                                    .child("Cannot be edited"),
                                            )
                                            .child(
                                                InlineEdit::new(self.disabled_edit.clone())
                                                    .disabled(true),
                                            ),
                                    ),
                            )
                            .child(
                                div()
                                    .w(px(320.0))
                                    .flex()
                                    .flex_col()
                                    .gap(px(24.0))
                                    .child(
                                        div()
                                            .text_size(px(16.0))
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .child("Profile Card"),
                                    )
                                    .child(
                                        div()
                                            .p(px(16.0))
                                            .bg(theme.tokens.card)
                                            .border_1()
                                            .border_color(theme.tokens.border)
                                            .rounded(theme.tokens.radius_lg)
                                            .flex()
                                            .flex_col()
                                            .gap(px(16.0))
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .gap(px(4.0))
                                                    .child(
                                                        div()
                                                            .text_size(px(12.0))
                                                            .font_weight(FontWeight::MEDIUM)
                                                            .text_color(
                                                                theme.tokens.muted_foreground,
                                                            )
                                                            .child("Name"),
                                                    )
                                                    .child({
                                                        let entity = entity.clone();
                                                        InlineEdit::new(self.name_field.clone())
                                                            .placeholder("Enter name")
                                                            .on_save(move |value, _, cx| {
                                                                entity.update(cx, |this, cx| {
                                                                    this.add_log(format!(
                                                                        "Name: \"{}\"",
                                                                        value
                                                                    ));
                                                                    cx.notify();
                                                                });
                                                            })
                                                    }),
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .gap(px(4.0))
                                                    .child(
                                                        div()
                                                            .text_size(px(12.0))
                                                            .font_weight(FontWeight::MEDIUM)
                                                            .text_color(
                                                                theme.tokens.muted_foreground,
                                                            )
                                                            .child("Email"),
                                                    )
                                                    .child({
                                                        let entity = entity.clone();
                                                        InlineEdit::new(self.email_field.clone())
                                                            .placeholder("Enter email")
                                                            .on_save(move |value, _, cx| {
                                                                entity.update(cx, |this, cx| {
                                                                    this.add_log(format!(
                                                                        "Email: \"{}\"",
                                                                        value
                                                                    ));
                                                                    cx.notify();
                                                                });
                                                            })
                                                    }),
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .gap(px(4.0))
                                                    .child(
                                                        div()
                                                            .text_size(px(12.0))
                                                            .font_weight(FontWeight::MEDIUM)
                                                            .text_color(
                                                                theme.tokens.muted_foreground,
                                                            )
                                                            .child("Bio"),
                                                    )
                                                    .child({
                                                        let entity = entity.clone();
                                                        InlineEdit::new(self.bio_field.clone())
                                                            .placeholder("Enter bio")
                                                            .on_save(move |value, _, cx| {
                                                                entity.update(cx, |this, cx| {
                                                                    this.add_log(format!(
                                                                        "Bio: \"{}\"",
                                                                        value
                                                                    ));
                                                                    cx.notify();
                                                                });
                                                            })
                                                    }),
                                            ),
                                    ),
                            ),
                    )
                    .when(!self.event_log.is_empty(), |d| {
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
                                        .child("Event Log:"),
                                )
                                .children(self.event_log.iter().map(|log| {
                                    div()
                                        .text_size(px(12.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .font_family(theme.tokens.font_mono.clone())
                                        .child(log.clone())
                                })),
                        )
                    }),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        adabraka_ui::init(cx);
        init_inline_edit(cx);
        install_theme(cx, Theme::dark());

        let bounds = Bounds::centered(None, size(px(900.0), px(650.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("Inline Edit Demo".into()),
                    appears_transparent: false,
                    traffic_light_position: None,
                }),
                ..Default::default()
            },
            |_, cx| cx.new(|cx| InlineEditDemo::new(cx)),
        )
        .unwrap();

        cx.activate(true);
    });
}
