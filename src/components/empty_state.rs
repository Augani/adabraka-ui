use crate::components::button::{Button, ButtonVariant};
use crate::components::icon::Icon;
use crate::components::icon_source::IconSource;
use crate::theme::use_theme;
use gpui::{prelude::FluentBuilder as _, *};
use std::rc::Rc;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum EmptyStateSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl EmptyStateSize {
    fn icon_size(self) -> Pixels {
        match self {
            EmptyStateSize::Sm => px(32.0),
            EmptyStateSize::Md => px(48.0),
            EmptyStateSize::Lg => px(64.0),
        }
    }

    fn title_size(self) -> Pixels {
        match self {
            EmptyStateSize::Sm => px(14.0),
            EmptyStateSize::Md => px(18.0),
            EmptyStateSize::Lg => px(24.0),
        }
    }

    fn description_size(self) -> Pixels {
        match self {
            EmptyStateSize::Sm => px(12.0),
            EmptyStateSize::Md => px(14.0),
            EmptyStateSize::Lg => px(16.0),
        }
    }

    fn gap(self) -> Pixels {
        match self {
            EmptyStateSize::Sm => px(12.0),
            EmptyStateSize::Md => px(16.0),
            EmptyStateSize::Lg => px(20.0),
        }
    }
}

#[derive(IntoElement)]
pub struct EmptyState {
    id: ElementId,
    icon: Option<IconSource>,
    title: SharedString,
    description: Option<SharedString>,
    action: Option<(SharedString, Rc<dyn Fn(&mut Window, &mut App)>)>,
    secondary_action: Option<(SharedString, Rc<dyn Fn(&mut Window, &mut App)>)>,
    size: EmptyStateSize,
    style: StyleRefinement,
}

impl EmptyState {
    pub fn new(id: impl Into<ElementId>, title: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            icon: None,
            title: title.into(),
            description: None,
            action: None,
            secondary_action: None,
            size: EmptyStateSize::default(),
            style: StyleRefinement::default(),
        }
    }

    pub fn icon(mut self, icon: impl Into<IconSource>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn action(
        mut self,
        label: impl Into<SharedString>,
        handler: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        self.action = Some((label.into(), Rc::new(handler)));
        self
    }

    pub fn secondary_action(
        mut self,
        label: impl Into<SharedString>,
        handler: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        self.secondary_action = Some((label.into(), Rc::new(handler)));
        self
    }

    pub fn size(mut self, size: EmptyStateSize) -> Self {
        self.size = size;
        self
    }
}

impl Styled for EmptyState {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for EmptyState {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let user_style = self.style;
        let icon_size = self.size.icon_size();
        let title_size = self.size.title_size();
        let description_size = self.size.description_size();
        let gap = self.size.gap();
        let id = self.id.clone();

        div()
            .id(self.id)
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap(gap)
            .p(px(24.0))
            .map(|mut this| {
                this.style().refine(&user_style);
                this
            })
            .when_some(self.icon, |d, icon_src| {
                d.child(
                    Icon::new(icon_src)
                        .size(icon_size)
                        .color(theme.tokens.muted_foreground),
                )
            })
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap(px(8.0))
                    .child(
                        div()
                            .text_size(title_size)
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(theme.tokens.foreground)
                            .font_family(theme.tokens.font_family.clone())
                            .text_align(TextAlign::Center)
                            .child(self.title),
                    )
                    .when_some(self.description, |d, desc| {
                        d.child(
                            div()
                                .text_size(description_size)
                                .text_color(theme.tokens.muted_foreground)
                                .font_family(theme.tokens.font_family.clone())
                                .text_align(TextAlign::Center)
                                .max_w(px(320.0))
                                .child(desc),
                        )
                    }),
            )
            .when(
                self.action.is_some() || self.secondary_action.is_some(),
                |d| {
                    d.child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(12.0))
                            .mt(px(8.0))
                            .when_some(self.action, |d, (label, handler)| {
                                let handler_clone = handler.clone();
                                d.child(
                                    Button::new(
                                        ElementId::Name(format!("{}-action", id).into()),
                                        label,
                                    )
                                    .variant(ButtonVariant::Default)
                                    .on_click(
                                        move |_, window, cx| {
                                            (handler_clone)(window, cx);
                                        },
                                    ),
                                )
                            })
                            .when_some(self.secondary_action, |d, (label, handler)| {
                                let handler_clone = handler.clone();
                                d.child(
                                    Button::new(
                                        ElementId::Name(format!("{}-secondary", id).into()),
                                        label,
                                    )
                                    .variant(ButtonVariant::Ghost)
                                    .on_click(
                                        move |_, window, cx| {
                                            (handler_clone)(window, cx);
                                        },
                                    ),
                                )
                            }),
                    )
                },
            )
    }
}
