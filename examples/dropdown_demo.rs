use adabraka_ui::{
    components::dropdown::{Dropdown, DropdownAlign, DropdownItem, DropdownState},
    layout::VStack,
    prelude::Button,
    theme::{install_theme, use_theme, Theme},
};
use gpui::{prelude::FluentBuilder as _, *};

struct DropdownDemo {
    basic_dropdown: Entity<DropdownState>,
    icon_dropdown: Entity<DropdownState>,
    aligned_dropdown: Entity<DropdownState>,
    last_action: Option<String>,
}

impl DropdownDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            basic_dropdown: cx.new(|cx| DropdownState::new(cx)),
            icon_dropdown: cx.new(|cx| DropdownState::new(cx)),
            aligned_dropdown: cx.new(|cx| DropdownState::new(cx)),
            last_action: None,
        }
    }
}

impl Render for DropdownDemo {
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
                                    .child("Dropdown Demo"),
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Contextual menus with icons, separators, and actions"),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .gap(px(16.0))
                            .flex_wrap()
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
                                            .child("Basic Dropdown"),
                                    )
                                    .child(
                                        Dropdown::new(
                                            self.basic_dropdown.clone(),
                                            Button::new("basic-trigger", "Options"),
                                        )
                                        .items(vec![
                                            DropdownItem::new("edit", "Edit").on_click({
                                                let entity = entity.clone();
                                                move |_, cx| {
                                                    entity.update(cx, |this, cx| {
                                                        this.last_action =
                                                            Some("Edit clicked".into());
                                                        cx.notify();
                                                    });
                                                }
                                            }),
                                            DropdownItem::new("duplicate", "Duplicate").on_click({
                                                let entity = entity.clone();
                                                move |_, cx| {
                                                    entity.update(cx, |this, cx| {
                                                        this.last_action =
                                                            Some("Duplicate clicked".into());
                                                        cx.notify();
                                                    });
                                                }
                                            }),
                                            DropdownItem::separator(),
                                            DropdownItem::new("archive", "Archive").on_click({
                                                let entity = entity.clone();
                                                move |_, cx| {
                                                    entity.update(cx, |this, cx| {
                                                        this.last_action =
                                                            Some("Archive clicked".into());
                                                        cx.notify();
                                                    });
                                                }
                                            }),
                                            DropdownItem::new("delete", "Delete")
                                                .destructive(true)
                                                .on_click({
                                                    let entity = entity.clone();
                                                    move |_, cx| {
                                                        entity.update(cx, |this, cx| {
                                                            this.last_action =
                                                                Some("Delete clicked".into());
                                                            cx.notify();
                                                        });
                                                    }
                                                }),
                                        ]),
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
                                            .child("With Icons"),
                                    )
                                    .child(
                                        Dropdown::new(
                                            self.icon_dropdown.clone(),
                                            Button::new("icon-trigger", "Actions"),
                                        )
                                        .items(vec![
                                            DropdownItem::new("new-file", "New File")
                                                .icon("file-plus")
                                                .on_click({
                                                    let entity = entity.clone();
                                                    move |_, cx| {
                                                        entity.update(cx, |this, cx| {
                                                            this.last_action =
                                                                Some("New File clicked".into());
                                                            cx.notify();
                                                        });
                                                    }
                                                }),
                                            DropdownItem::new("new-folder", "New Folder")
                                                .icon("folder-plus")
                                                .on_click({
                                                    let entity = entity.clone();
                                                    move |_, cx| {
                                                        entity.update(cx, |this, cx| {
                                                            this.last_action =
                                                                Some("New Folder clicked".into());
                                                            cx.notify();
                                                        });
                                                    }
                                                }),
                                            DropdownItem::separator(),
                                            DropdownItem::new("download", "Download")
                                                .icon("download")
                                                .on_click({
                                                    let entity = entity.clone();
                                                    move |_, cx| {
                                                        entity.update(cx, |this, cx| {
                                                            this.last_action =
                                                                Some("Download clicked".into());
                                                            cx.notify();
                                                        });
                                                    }
                                                }),
                                            DropdownItem::new("share", "Share")
                                                .icon("share")
                                                .disabled(true),
                                        ]),
                                    )
                            })
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(14.0))
                                            .font_weight(FontWeight::MEDIUM)
                                            .child("End Aligned"),
                                    )
                                    .child(
                                        Dropdown::new(
                                            self.aligned_dropdown.clone(),
                                            Button::new("aligned-trigger", "More"),
                                        )
                                        .align(DropdownAlign::End)
                                        .items(vec![
                                            DropdownItem::new("settings", "Settings")
                                                .icon("settings"),
                                            DropdownItem::new("help", "Help").icon("help-circle"),
                                            DropdownItem::separator(),
                                            DropdownItem::new("logout", "Logout")
                                                .icon("log-out")
                                                .destructive(true),
                                        ]),
                                    ),
                            ),
                    )
                    .when_some(self.last_action.as_ref(), |d, action| {
                        d.child(
                            div()
                                .mt(px(16.0))
                                .p(px(12.0))
                                .bg(theme.tokens.muted)
                                .rounded(theme.tokens.radius_md)
                                .text_size(px(14.0))
                                .child(format!("Last action: {}", action)),
                        )
                    }),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        adabraka_ui::init(cx);
        install_theme(cx, Theme::dark());

        let bounds = Bounds::centered(None, size(px(700.0), px(500.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("Dropdown Demo".into()),
                    appears_transparent: false,
                    traffic_light_position: None,
                }),
                ..Default::default()
            },
            |_, cx| cx.new(|cx| DropdownDemo::new(cx)),
        )
        .unwrap();

        cx.activate(true);
    });
}
