use crate::components::button::{Button, ButtonVariant};
use crate::components::empty_state::EmptyState;
use crate::components::icon::Icon;
use crate::components::icon_source::IconSource;
use crate::components::scrollable::scrollable_vertical;
use crate::theme::use_theme;
use gpui::{prelude::FluentBuilder as _, *};
use std::rc::Rc;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum NotificationVariant {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

impl NotificationVariant {
    fn icon_name(&self) -> &'static str {
        match self {
            NotificationVariant::Info => "info",
            NotificationVariant::Success => "check-circle",
            NotificationVariant::Warning => "alert-triangle",
            NotificationVariant::Error => "x-circle",
        }
    }

    fn color(&self, theme: &crate::theme::Theme) -> Hsla {
        match self {
            NotificationVariant::Info => theme.tokens.primary,
            NotificationVariant::Success => gpui::hsla(142.0 / 360.0, 0.71, 0.45, 1.0),
            NotificationVariant::Warning => gpui::hsla(48.0 / 360.0, 0.96, 0.53, 1.0),
            NotificationVariant::Error => theme.tokens.destructive,
        }
    }
}

#[derive(Clone)]
pub struct NotificationAction {
    pub label: SharedString,
    pub handler: Rc<dyn Fn(&mut Window, &mut App)>,
}

#[derive(Clone)]
pub struct NotificationItem {
    pub id: ElementId,
    pub title: SharedString,
    pub message: Option<SharedString>,
    pub timestamp: Option<SharedString>,
    pub variant: NotificationVariant,
    pub read: bool,
    pub icon: Option<IconSource>,
    pub action: Option<NotificationAction>,
}

impl NotificationItem {
    pub fn new(id: impl Into<ElementId>, title: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            message: None,
            timestamp: None,
            variant: NotificationVariant::default(),
            read: false,
            icon: None,
            action: None,
        }
    }

    pub fn message(mut self, message: impl Into<SharedString>) -> Self {
        self.message = Some(message.into());
        self
    }

    pub fn timestamp(mut self, timestamp: impl Into<SharedString>) -> Self {
        self.timestamp = Some(timestamp.into());
        self
    }

    pub fn variant(mut self, variant: NotificationVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn read(mut self, read: bool) -> Self {
        self.read = read;
        self
    }

    pub fn icon(mut self, icon: impl Into<IconSource>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn action(
        mut self,
        label: impl Into<SharedString>,
        handler: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        self.action = Some(NotificationAction {
            label: label.into(),
            handler: Rc::new(handler),
        });
        self
    }
}

pub struct NotificationCenterState {
    notifications: Vec<NotificationItem>,
    _focus_handle: FocusHandle,
}

impl NotificationCenterState {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            notifications: Vec::new(),
            _focus_handle: cx.focus_handle(),
        }
    }

    pub fn add(&mut self, notification: NotificationItem, cx: &mut Context<Self>) {
        self.notifications.insert(0, notification);
        cx.notify();
    }

    pub fn remove(&mut self, id: &ElementId, cx: &mut Context<Self>) {
        self.notifications.retain(|n| &n.id != id);
        cx.notify();
    }

    pub fn mark_read(&mut self, id: &ElementId, cx: &mut Context<Self>) {
        if let Some(notification) = self.notifications.iter_mut().find(|n| &n.id == id) {
            notification.read = true;
            cx.notify();
        }
    }

    pub fn mark_all_read(&mut self, cx: &mut Context<Self>) {
        for notification in &mut self.notifications {
            notification.read = true;
        }
        cx.notify();
    }

    pub fn clear_all(&mut self, cx: &mut Context<Self>) {
        self.notifications.clear();
        cx.notify();
    }

    pub fn unread_count(&self) -> usize {
        self.notifications.iter().filter(|n| !n.read).count()
    }

    pub fn notifications(&self) -> &[NotificationItem] {
        &self.notifications
    }

    pub fn is_empty(&self) -> bool {
        self.notifications.is_empty()
    }
}

impl EventEmitter<()> for NotificationCenterState {}

#[derive(IntoElement)]
pub struct NotificationCenter {
    state: Entity<NotificationCenterState>,
    max_visible: usize,
    show_timestamps: bool,
    group_by_date: bool,
    on_notification_click: Option<Rc<dyn Fn(&NotificationItem, &mut Window, &mut App)>>,
    style: StyleRefinement,
}

impl NotificationCenter {
    pub fn new(state: Entity<NotificationCenterState>) -> Self {
        Self {
            state,
            max_visible: 10,
            show_timestamps: true,
            group_by_date: false,
            on_notification_click: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn max_visible(mut self, max: usize) -> Self {
        self.max_visible = max;
        self
    }

    pub fn show_timestamps(mut self, show: bool) -> Self {
        self.show_timestamps = show;
        self
    }

    pub fn group_by_date(mut self, group: bool) -> Self {
        self.group_by_date = group;
        self
    }

    pub fn on_notification_click(
        mut self,
        handler: impl Fn(&NotificationItem, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_notification_click = Some(Rc::new(handler));
        self
    }
}

impl Styled for NotificationCenter {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for NotificationCenter {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let user_style = self.style;
        let state = self.state.read(cx);
        let notifications = state.notifications().to_vec();
        let is_empty = notifications.is_empty();
        let total_count = notifications.len();
        let show_more = total_count > self.max_visible;
        let visible_notifications: Vec<_> =
            notifications.into_iter().take(self.max_visible).collect();

        let state_entity = self.state.clone();
        let on_click = self.on_notification_click.clone();
        let show_timestamps = self.show_timestamps;

        let shadow_lg = theme.tokens.shadow_lg.clone();

        div()
            .flex()
            .flex_col()
            .w(px(380.0))
            .max_h(px(500.0))
            .bg(theme.tokens.card)
            .border_1()
            .border_color(theme.tokens.border)
            .rounded(theme.tokens.radius_lg)
            .shadow(vec![shadow_lg])
            .overflow_hidden()
            .map(|mut this| {
                this.style().refine(&user_style);
                this
            })
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .px(px(16.0))
                    .py(px(12.0))
                    .border_b_1()
                    .border_color(theme.tokens.border)
                    .child(
                        div()
                            .text_size(px(14.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(theme.tokens.foreground)
                            .font_family(theme.tokens.font_family.clone())
                            .child("Notifications"),
                    )
                    .when(!is_empty, {
                        let state_clone = state_entity.clone();
                        |d| {
                            d.child(
                                Button::new("mark-all-read", "Mark all read")
                                    .variant(ButtonVariant::Ghost)
                                    .size(crate::components::button::ButtonSize::Sm)
                                    .on_click(move |_, _, cx| {
                                        state_clone.update(cx, |state, cx| {
                                            state.mark_all_read(cx);
                                        });
                                    }),
                            )
                        }
                    }),
            )
            .when(is_empty, |d| {
                d.child(
                    EmptyState::new("notification-empty", "No notifications")
                        .icon("bell-off")
                        .description("You're all caught up!")
                        .size(crate::components::empty_state::EmptyStateSize::Sm)
                        .py(px(32.0)),
                )
            })
            .when(!is_empty, |d| {
                d.child(
                    scrollable_vertical(div().flex().flex_col().children(
                        visible_notifications.into_iter().map(|notification| {
                            let id = notification.id.clone();
                            let state_for_click = state_entity.clone();
                            let state_for_dismiss = state_entity.clone();
                            let on_click_handler = on_click.clone();
                            let notification_clone = notification.clone();
                            let is_read = notification.read;
                            let variant = notification.variant;
                            let variant_color = variant.color(&theme);

                            div()
                                .id(id.clone())
                                .flex()
                                .gap(px(12.0))
                                .px(px(16.0))
                                .py(px(12.0))
                                .border_b_1()
                                .border_color(theme.tokens.border)
                                .bg(if is_read {
                                    gpui::transparent_black()
                                } else {
                                    theme.tokens.accent.opacity(0.3)
                                })
                                .cursor(CursorStyle::PointingHand)
                                .hover(|style| style.bg(theme.tokens.accent))
                                .on_mouse_down(MouseButton::Left, {
                                    let id = id.clone();
                                    move |_, window, cx| {
                                        state_for_click.update(cx, |state, cx| {
                                            state.mark_read(&id, cx);
                                        });
                                        if let Some(ref handler) = on_click_handler {
                                            handler(&notification_clone, window, cx);
                                        }
                                    }
                                })
                                .child(
                                    div().flex_shrink_0().mt(px(2.0)).child(
                                        Icon::new(
                                            notification
                                                .icon
                                                .clone()
                                                .unwrap_or_else(|| variant.icon_name().into()),
                                        )
                                        .size(px(18.0))
                                        .color(variant_color),
                                    ),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .flex_1()
                                        .gap(px(4.0))
                                        .overflow_hidden()
                                        .child(
                                            div()
                                                .flex()
                                                .items_center()
                                                .justify_between()
                                                .gap(px(8.0))
                                                .child(
                                                    div()
                                                        .text_size(px(13.0))
                                                        .font_weight(if is_read {
                                                            FontWeight::NORMAL
                                                        } else {
                                                            FontWeight::SEMIBOLD
                                                        })
                                                        .text_color(theme.tokens.foreground)
                                                        .font_family(
                                                            theme.tokens.font_family.clone(),
                                                        )
                                                        .truncate()
                                                        .child(notification.title.clone()),
                                                )
                                                .when(
                                                    show_timestamps
                                                        && notification.timestamp.is_some(),
                                                    |d| {
                                                        d.child(
                                                            div()
                                                                .flex_shrink_0()
                                                                .text_size(px(11.0))
                                                                .text_color(
                                                                    theme.tokens.muted_foreground,
                                                                )
                                                                .font_family(
                                                                    theme
                                                                        .tokens
                                                                        .font_family
                                                                        .clone(),
                                                                )
                                                                .child(
                                                                    notification
                                                                        .timestamp
                                                                        .clone()
                                                                        .unwrap_or_default(),
                                                                ),
                                                        )
                                                    },
                                                ),
                                        )
                                        .when_some(notification.message.clone(), |d, msg| {
                                            d.child(
                                                div()
                                                    .text_size(px(12.0))
                                                    .text_color(theme.tokens.muted_foreground)
                                                    .font_family(theme.tokens.font_family.clone())
                                                    .line_height(relative(1.4))
                                                    .child(msg),
                                            )
                                        })
                                        .when_some(notification.action.clone(), |d, action| {
                                            let handler = action.handler.clone();
                                            d.child(
                                                div().mt(px(4.0)).child(
                                                    Button::new(
                                                        ElementId::Name(
                                                            format!("action-{:?}", id).into(),
                                                        ),
                                                        action.label.clone(),
                                                    )
                                                    .variant(ButtonVariant::Outline)
                                                    .size(crate::components::button::ButtonSize::Sm)
                                                    .on_click(move |_, window, cx| {
                                                        (handler)(window, cx);
                                                    }),
                                                ),
                                            )
                                        }),
                                )
                                .child(
                                    div()
                                        .flex_shrink_0()
                                        .w(px(20.0))
                                        .h(px(20.0))
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .rounded(theme.tokens.radius_sm)
                                        .text_color(theme.tokens.muted_foreground)
                                        .text_size(px(14.0))
                                        .font_family(theme.tokens.font_family.clone())
                                        .hover(|style| style.bg(theme.tokens.accent))
                                        .on_mouse_down(MouseButton::Left, {
                                            let id = id.clone();
                                            move |_, _, cx| {
                                                cx.stop_propagation();
                                                state_for_dismiss.update(cx, |state, cx| {
                                                    state.remove(&id, cx);
                                                });
                                            }
                                        })
                                        .child(
                                            Icon::new("x")
                                                .size(px(14.0))
                                                .color(theme.tokens.muted_foreground),
                                        ),
                                )
                        }),
                    ))
                    .max_h(px(350.0)),
                )
            })
            .when(show_more, |d| {
                d.child(
                    div()
                        .px(px(16.0))
                        .py(px(8.0))
                        .border_t_1()
                        .border_color(theme.tokens.border)
                        .child(
                            div()
                                .text_size(px(12.0))
                                .text_color(theme.tokens.muted_foreground)
                                .font_family(theme.tokens.font_family.clone())
                                .text_align(TextAlign::Center)
                                .child(format!(
                                    "+ {} more notifications",
                                    total_count - self.max_visible
                                )),
                        ),
                )
            })
            .when(!is_empty, {
                let state_clone = state_entity.clone();
                |d| {
                    d.child(
                        div()
                            .flex()
                            .justify_center()
                            .px(px(16.0))
                            .py(px(8.0))
                            .border_t_1()
                            .border_color(theme.tokens.border)
                            .child(
                                Button::new("clear-all", "Clear all")
                                    .variant(ButtonVariant::Ghost)
                                    .size(crate::components::button::ButtonSize::Sm)
                                    .on_click(move |_, _, cx| {
                                        state_clone.update(cx, |state, cx| {
                                            state.clear_all(cx);
                                        });
                                    }),
                            ),
                    )
                }
            })
    }
}

#[derive(IntoElement)]
pub struct NotificationBell {
    id: ElementId,
    state: Entity<NotificationCenterState>,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
    style: StyleRefinement,
}

impl NotificationBell {
    pub fn new(state: Entity<NotificationCenterState>) -> Self {
        Self {
            id: ElementId::Name("notification-bell".into()),
            state,
            on_click: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }
}

impl Styled for NotificationBell {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for NotificationBell {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let user_style = self.style;
        let unread_count = self.state.read(cx).unread_count();
        let on_click = self.on_click.clone();

        div()
            .id(self.id)
            .relative()
            .w(px(40.0))
            .h(px(40.0))
            .flex()
            .items_center()
            .justify_center()
            .rounded(theme.tokens.radius_md)
            .cursor(CursorStyle::PointingHand)
            .hover(|style| style.bg(theme.tokens.accent))
            .map(|mut this| {
                this.style().refine(&user_style);
                this
            })
            .when_some(on_click, |d, handler| {
                d.on_click(move |event, window, cx| {
                    (handler)(event, window, cx);
                })
            })
            .child(
                Icon::new("bell")
                    .size(px(20.0))
                    .color(theme.tokens.foreground),
            )
            .when(unread_count > 0, |d| {
                d.child(
                    div()
                        .absolute()
                        .top(px(4.0))
                        .right(px(4.0))
                        .min_w(px(18.0))
                        .h(px(18.0))
                        .px(px(5.0))
                        .flex()
                        .items_center()
                        .justify_center()
                        .rounded_full()
                        .bg(theme.tokens.destructive)
                        .text_size(px(10.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(theme.tokens.destructive_foreground)
                        .font_family(theme.tokens.font_family.clone())
                        .child(if unread_count > 99 {
                            "99+".to_string()
                        } else {
                            unread_count.to_string()
                        }),
                )
            })
    }
}
