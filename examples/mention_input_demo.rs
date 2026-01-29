use adabraka_ui::components::scrollable::scrollable_vertical;
use adabraka_ui::prelude::*;
use gpui::{prelude::FluentBuilder, *};
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
            init_mention_input(cx);
            install_theme(cx, Theme::dark());

            cx.open_window(
                WindowOptions {
                    titlebar: Some(TitlebarOptions {
                        title: Some("MentionInput Demo - Adabraka UI".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(900.0), px(700.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|cx| MentionInputDemo::new(cx)),
            )
            .unwrap();
        });
}

struct MentionInputDemo {
    basic_state: Entity<MentionInputState>,
    custom_trigger_state: Entity<MentionInputState>,
    with_avatars_state: Entity<MentionInputState>,
    disabled_state: Entity<MentionInputState>,

    last_mentioned: Option<String>,
    message_content: String,
}

impl MentionInputDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        let basic_state = cx.new(|cx| MentionInputState::new(cx));
        let custom_trigger_state = cx.new(|cx| MentionInputState::new(cx));
        let with_avatars_state = cx.new(|cx| MentionInputState::new(cx));
        let disabled_state = cx.new(|cx| {
            let mut state = MentionInputState::new(cx);
            state.disabled = true;
            state
        });

        Self {
            basic_state,
            custom_trigger_state,
            with_avatars_state,
            disabled_state,
            last_mentioned: None,
            message_content: String::new(),
        }
    }

    fn sample_users() -> Vec<MentionItem> {
        vec![
            MentionItem::new("1", "Alice Johnson"),
            MentionItem::new("2", "Bob Smith"),
            MentionItem::new("3", "Charlie Brown"),
            MentionItem::new("4", "Diana Prince"),
            MentionItem::new("5", "Eve Adams"),
            MentionItem::new("6", "Frank Miller"),
            MentionItem::new("7", "Grace Lee"),
            MentionItem::new("8", "Henry Wilson"),
            MentionItem::new("9", "Ivy Chen"),
            MentionItem::new("10", "Jack Davis"),
        ]
    }

    fn users_with_avatars() -> Vec<MentionItem> {
        vec![
            MentionItem::new("1", "Alice Johnson").with_avatar("https://i.pravatar.cc/150?u=alice"),
            MentionItem::new("2", "Bob Smith").with_avatar("https://i.pravatar.cc/150?u=bob"),
            MentionItem::new("3", "Charlie Brown")
                .with_avatar("https://i.pravatar.cc/150?u=charlie"),
            MentionItem::new("4", "Diana Prince").with_avatar("https://i.pravatar.cc/150?u=diana"),
            MentionItem::new("5", "Eve Adams").with_avatar("https://i.pravatar.cc/150?u=eve"),
            MentionItem::new("6", "Frank Miller").with_avatar("https://i.pravatar.cc/150?u=frank"),
        ]
    }

    fn channels() -> Vec<MentionItem> {
        vec![
            MentionItem::new("general", "general"),
            MentionItem::new("random", "random"),
            MentionItem::new("engineering", "engineering"),
            MentionItem::new("design", "design"),
            MentionItem::new("marketing", "marketing"),
            MentionItem::new("support", "support"),
        ]
    }
}

impl Render for MentionInputDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();
        let entity = cx.entity().clone();

        div()
            .size_full()
            .bg(theme.tokens.background)
            .overflow_hidden()
            .child(
                scrollable_vertical(
                    div()
                        .flex()
                        .flex_col()
                        .p(px(32.0))
                        .gap(px(32.0))
                        .text_color(theme.tokens.foreground)
                        .child(
                            VStack::new().gap(px(8.0)).child(
                                div()
                                    .text_size(px(28.0))
                                    .font_weight(FontWeight::BOLD)
                                    .child("MentionInput Component Showcase"),
                            ).child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Text input with @-mention support, dropdown suggestions, and keyboard navigation"),
                            ),
                        )
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("1. Basic MentionInput"),
                                )
                                .child(
                                    div()
                                        .text_size(px(13.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child("Type @ to trigger the mention dropdown. Use arrow keys to navigate, Enter to select."),
                                )
                                .child(
                                    MentionInput::new(&self.basic_state, Self::sample_users())
                                        .placeholder("Type @ to mention someone...")
                                        .on_mention({
                                            let entity = entity.clone();
                                            move |item, cx| {
                                                entity.update(cx, |demo, cx| {
                                                    demo.last_mentioned =
                                                        Some(item.name.to_string());
                                                    cx.notify();
                                                });
                                            }
                                        })
                                        .on_change({
                                            let entity = entity.clone();
                                            move |content, cx| {
                                                entity.update(cx, |demo, cx| {
                                                    demo.message_content = content.to_string();
                                                    cx.notify();
                                                });
                                            }
                                        })
                                        .w(px(400.0)),
                                )
                                .when(self.last_mentioned.is_some(), |stack| {
                                    stack.child(
                                        div()
                                            .p(px(12.0))
                                            .bg(theme.tokens.accent)
                                            .rounded(px(6.0))
                                            .text_size(px(13.0))
                                            .text_color(theme.tokens.accent_foreground)
                                            .child(format!(
                                                "Last mentioned: {}",
                                                self.last_mentioned.as_ref().unwrap()
                                            )),
                                    )
                                })
                                .when(!self.message_content.is_empty(), |stack| {
                                    stack.child(
                                        div()
                                            .p(px(12.0))
                                            .bg(theme.tokens.muted)
                                            .rounded(px(6.0))
                                            .text_size(px(13.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child(format!("Content: {}", self.message_content)),
                                    )
                                }),
                        )
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("2. Custom Trigger Character"),
                                )
                                .child(
                                    div()
                                        .text_size(px(13.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child("Use # instead of @ to mention channels. Great for Slack-like interfaces."),
                                )
                                .child(
                                    MentionInput::new(&self.custom_trigger_state, Self::channels())
                                        .placeholder("Type # to mention a channel...")
                                        .trigger_char('#')
                                        .w(px(400.0)),
                                ),
                        )
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("3. With Avatar Support"),
                                )
                                .child(
                                    div()
                                        .text_size(px(13.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child("MentionItems can include avatar URLs for richer dropdown display."),
                                )
                                .child(
                                    MentionInput::new(
                                        &self.with_avatars_state,
                                        Self::users_with_avatars(),
                                    )
                                    .placeholder("Type @ to mention users with avatars...")
                                    .max_dropdown_height(px(250.0))
                                    .w(px(400.0)),
                                ),
                        )
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("4. Disabled State"),
                                )
                                .child(
                                    div()
                                        .text_size(px(13.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child("MentionInput can be disabled to prevent user interaction."),
                                )
                                .child(
                                    MentionInput::new(&self.disabled_state, Self::sample_users())
                                        .placeholder("This input is disabled...")
                                        .disabled(true)
                                        .w(px(400.0)),
                                ),
                        )
                        .child(
                            div()
                                .mt(px(16.0))
                                .p(px(20.0))
                                .bg(theme.tokens.primary.opacity(0.1))
                                .border_1()
                                .border_color(theme.tokens.primary)
                                .rounded(px(8.0))
                                .child(
                                    VStack::new()
                                        .gap(px(12.0))
                                        .child(
                                            div()
                                                .text_size(px(16.0))
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .text_color(theme.tokens.primary)
                                                .child("Keyboard Shortcuts"),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(13.0))
                                                .text_color(theme.tokens.foreground)
                                                .child("@ (or custom trigger) - Open mention dropdown"),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(13.0))
                                                .text_color(theme.tokens.foreground)
                                                .child("Up/Down arrows - Navigate suggestions"),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(13.0))
                                                .text_color(theme.tokens.foreground)
                                                .child("Enter - Select highlighted mention"),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(13.0))
                                                .text_color(theme.tokens.foreground)
                                                .child("Escape - Close dropdown without selecting"),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(13.0))
                                                .text_color(theme.tokens.foreground)
                                                .child("Continue typing - Filter suggestions"),
                                        ),
                                ),
                        )
                        .child(
                            div()
                                .p(px(20.0))
                                .bg(theme.tokens.accent)
                                .rounded(px(8.0))
                                .child(
                                    VStack::new()
                                        .gap(px(12.0))
                                        .child(
                                            div()
                                                .text_size(px(16.0))
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .text_color(theme.tokens.accent_foreground)
                                                .child("Component Features"),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(13.0))
                                                .text_color(theme.tokens.accent_foreground)
                                                .child("- Trigger dropdown on configurable character (default @)"),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(13.0))
                                                .text_color(theme.tokens.accent_foreground)
                                                .child("- Real-time filtering of suggestions as you type"),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(13.0))
                                                .text_color(theme.tokens.accent_foreground)
                                                .child("- Highlighted mention styling in the text"),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(13.0))
                                                .text_color(theme.tokens.accent_foreground)
                                                .child("- Support for multiple mentions in one input"),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(13.0))
                                                .text_color(theme.tokens.accent_foreground)
                                                .child("- Avatar support for rich user display"),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(13.0))
                                                .text_color(theme.tokens.accent_foreground)
                                                .child("- Full keyboard navigation support"),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(13.0))
                                                .text_color(theme.tokens.accent_foreground)
                                                .child("- on_change and on_mention callbacks"),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(13.0))
                                                .text_color(theme.tokens.accent_foreground)
                                                .child("- Proper theming using use_theme()"),
                                        ),
                                ),
                        ),
                ),
            )
    }
}
