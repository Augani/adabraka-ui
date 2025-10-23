//! Toolbar component with icon buttons and grouping.

use gpui::{prelude::FluentBuilder as _, *};
use std::rc::Rc;
use crate::{
    theme::use_theme,
    components::icon::Icon,
    components::icon_source::IconSource,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ToolbarButtonVariant {
    Default,
    Toggle,
    Dropdown,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ToolbarSize {
    Sm,
    Md,
    Lg,
}

impl ToolbarSize {
    fn button_size(&self) -> Pixels {
        match self {
            Self::Sm => px(32.0),
            Self::Md => px(36.0),
            Self::Lg => px(40.0),
        }
    }

    fn icon_size(&self) -> Pixels {
        match self {
            Self::Sm => px(14.0),
            Self::Md => px(16.0),
            Self::Lg => px(18.0),
        }
    }
}

#[derive(Clone)]
pub struct ToolbarButton {
    pub id: SharedString,
    pub icon: IconSource,
    pub tooltip: Option<SharedString>,
    pub variant: ToolbarButtonVariant,
    pub pressed: bool,
    pub disabled: bool,
    pub on_click: Option<Rc<dyn Fn(&mut Window, &mut App)>>,
}

impl ToolbarButton {
    pub fn new(id: impl Into<SharedString>, icon: impl Into<IconSource>) -> Self {
        Self {
            id: id.into(),
            icon: icon.into(),
            tooltip: None,
            variant: ToolbarButtonVariant::Default,
            pressed: false,
            disabled: false,
            on_click: None,
        }
    }

    pub fn tooltip(mut self, tooltip: impl Into<SharedString>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }

    pub fn variant(mut self, variant: ToolbarButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn pressed(mut self, pressed: bool) -> Self {
        self.pressed = pressed;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_click<F>(mut self, handler: F) -> Self
    where
        F: Fn(&mut Window, &mut App) + 'static,
    {
        self.on_click = Some(Rc::new(handler));
        self
    }
}

#[derive(Clone)]
pub enum ToolbarItem {
    Button(ToolbarButton),
    Separator,
    Spacer,
}

#[derive(Clone)]
pub struct ToolbarGroup {
    pub items: Vec<ToolbarItem>,
}

impl ToolbarGroup {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn button(mut self, button: ToolbarButton) -> Self {
        self.items.push(ToolbarItem::Button(button));
        self
    }

    pub fn separator(mut self) -> Self {
        self.items.push(ToolbarItem::Separator);
        self
    }

    pub fn spacer(mut self) -> Self {
        self.items.push(ToolbarItem::Spacer);
        self
    }

    pub fn buttons(mut self, buttons: Vec<ToolbarButton>) -> Self {
        for button in buttons {
            self.items.push(ToolbarItem::Button(button));
        }
        self
    }
}

impl Default for ToolbarGroup {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Toolbar {
    groups: Vec<ToolbarGroup>,
    size: ToolbarSize,
    style: StyleRefinement,
}

impl Toolbar {
    pub fn new() -> Self {
        Self {
            groups: Vec::new(),
            size: ToolbarSize::Md,
            style: StyleRefinement::default(),
        }
    }

    pub fn size(mut self, size: ToolbarSize) -> Self {
        self.size = size;
        self
    }

    pub fn group(mut self, group: ToolbarGroup) -> Self {
        self.groups.push(group);
        self
    }

    pub fn groups(mut self, groups: Vec<ToolbarGroup>) -> Self {
        self.groups.extend(groups);
        self
    }
}

impl Default for Toolbar {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for Toolbar {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl Render for Toolbar {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();
        let button_size = self.size.button_size();
        let icon_size = self.size.icon_size();
        let user_style = self.style.clone();

        div()
            .flex()
            .items_center()
            .gap(px(8.0))
            .px(px(8.0))
            .py(px(6.0))
            .bg(theme.tokens.background)
            .border_b_1()
            .border_color(theme.tokens.border)
            .children(self.groups.iter().enumerate().map(|(group_idx, group)| {
                let is_last_group = group_idx == self.groups.len() - 1;

                div()
                    .flex()
                    .items_center()
                    .gap(px(4.0))
                    .children(group.items.iter().map(|item| {
                        match item {
                            ToolbarItem::Button(button) => {
                                render_toolbar_button(button.clone(), button_size, icon_size).into_any_element()
                            }
                            ToolbarItem::Separator => {
                                div()
                                    .w(px(1.0))
                                    .h(button_size * 0.6)
                                    .bg(theme.tokens.border)
                                    .mx(px(4.0))
                                    .into_any_element()
                            }
                            ToolbarItem::Spacer => {
                                div().flex_1().into_any_element()
                            }
                        }
                    }))
                    .when(!is_last_group, |this| {
                        this.child(
                            div()
                                .w(px(1.0))
                                .h(button_size * 0.6)
                                .bg(theme.tokens.border)
                                .mx(px(4.0))
                        )
                    })
            }))
            .map(|this| {
                let mut div = this;
                div.style().refine(&user_style);
                div
            })
    }
}

fn render_toolbar_button(button: ToolbarButton, button_size: Pixels, icon_size: Pixels) -> impl IntoElement {
    let theme = use_theme();

    div()
        .size(button_size)
        .flex()
        .items_center()
        .justify_center()
        .rounded(theme.tokens.radius_sm)
        .cursor(if button.disabled {
            CursorStyle::Arrow
        } else {
            CursorStyle::PointingHand
        })
        .when(button.disabled, |div| div.opacity(0.5))
        .when(button.pressed && !button.disabled, |div| {
            div.bg(theme.tokens.accent)
        })
        .when(!button.disabled, |div| {
            div.hover(|style| {
                if button.pressed {
                    style.bg(theme.tokens.accent.opacity(0.8))
                } else {
                    style.bg(theme.tokens.muted)
                }
            })
        })
        .when_some(button.on_click.filter(|_| !button.disabled), |div, handler| {
            div.on_mouse_down(MouseButton::Left, move |_event, window, cx| {
                handler(window, cx);
            })
        })
        .child(
            Icon::new(button.icon)
                .size(icon_size)
                .color(if button.disabled {
                    theme.tokens.muted_foreground
                } else {
                    theme.tokens.foreground
                })
        )
        .children(
            (button.variant == ToolbarButtonVariant::Dropdown).then(|| {
                div()
                    .absolute()
                    .bottom(px(2.0))
                    .right(px(2.0))
                    .child(
                        Icon::new("chevron-down")
                            .size(px(10.0))
                            .color(theme.tokens.foreground)
                    )
                    .into_any_element()
            })
        )
}
