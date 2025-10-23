//! Toggle group component - Grouped toggle buttons for toolbars and view switchers.

use gpui::{prelude::FluentBuilder as _, *};
use std::rc::Rc;

use crate::theme::use_theme;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToggleGroupVariant {
    Single,
    Multiple,
}

impl Default for ToggleGroupVariant {
    fn default() -> Self {
        Self::Single
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToggleGroupSize {
    Sm,
    Md,
    Lg,
}

impl Default for ToggleGroupSize {
    fn default() -> Self {
        Self::Md
    }
}

impl ToggleGroupSize {
    fn height(&self) -> Pixels {
        match self {
            Self::Sm => px(32.0),
            Self::Md => px(36.0),
            Self::Lg => px(40.0),
        }
    }

    fn px_value(&self) -> Pixels {
        match self {
            Self::Sm => px(8.0),
            Self::Md => px(12.0),
            Self::Lg => px(16.0),
        }
    }

    fn text_size(&self) -> Pixels {
        match self {
            Self::Sm => px(12.0),
            Self::Md => px(14.0),
            Self::Lg => px(16.0),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ToggleGroupItem {
    value: SharedString,
    label: SharedString,
    icon: Option<SharedString>,
    disabled: bool,
}

impl ToggleGroupItem {
    pub fn new(value: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            icon: None,
            disabled: false,
        }
    }

    pub fn icon(mut self, icon: impl Into<SharedString>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

#[derive(IntoElement)]
pub struct ToggleGroup {
    variant: ToggleGroupVariant,
    size: ToggleGroupSize,
    items: Vec<ToggleGroupItem>,
    value: Option<SharedString>,        // For single selection
    values: Vec<SharedString>,          // For multiple selection
    disabled: bool,
    on_change: Option<Rc<dyn Fn(&SharedString, &mut Window, &mut App)>>,
    on_multiple_change: Option<Rc<dyn Fn(&Vec<SharedString>, &mut Window, &mut App)>>,
    style: StyleRefinement,
}

impl ToggleGroup {
    /// Create a new toggle group
    pub fn new() -> Self {
        Self {
            variant: ToggleGroupVariant::default(),
            size: ToggleGroupSize::default(),
            items: Vec::new(),
            value: None,
            values: Vec::new(),
            disabled: false,
            on_change: None,
            on_multiple_change: None,
            style: StyleRefinement::default(),
        }
    }

    /// Set the selection behavior variant
    pub fn variant(mut self, variant: ToggleGroupVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the size of toggle buttons
    pub fn size(mut self, size: ToggleGroupSize) -> Self {
        self.size = size;
        self
    }

    /// Add an item to the group
    pub fn item(mut self, item: ToggleGroupItem) -> Self {
        self.items.push(item);
        self
    }

    /// Add multiple items to the group
    pub fn items(mut self, items: Vec<ToggleGroupItem>) -> Self {
        self.items.extend(items);
        self
    }

    /// Set the selected value (for single selection)
    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Set the selected values (for multiple selection)
    pub fn values(mut self, values: Vec<SharedString>) -> Self {
        self.values = values;
        self
    }

    /// Set whether the entire group is disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set the change handler for single selection
    pub fn on_change<F>(mut self, handler: F) -> Self
    where
        F: Fn(&SharedString, &mut Window, &mut App) + 'static,
    {
        self.on_change = Some(Rc::new(handler));
        self
    }

    /// Set the change handler for multiple selection
    pub fn on_multiple_change<F>(mut self, handler: F) -> Self
    where
        F: Fn(&Vec<SharedString>, &mut Window, &mut App) + 'static,
    {
        self.on_multiple_change = Some(Rc::new(handler));
        self
    }
}

impl Default for ToggleGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for ToggleGroup {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for ToggleGroup {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let theme = use_theme();

        let variant = self.variant;
        let current_value = self.value;
        let current_values = self.values;
        let disabled = self.disabled;
        let size = self.size;
        let on_change = self.on_change;
        let user_style = self.style;

        div()
            .flex()
            .items_center()
            .gap(px(2.0))
            .p(px(2.0))
            .bg(theme.tokens.muted.opacity(0.3))
            .rounded(theme.tokens.radius_md)
            .children(
                self.items.into_iter().map(move |item| {
                    let is_selected = match variant {
                        ToggleGroupVariant::Single => {
                            current_value.as_ref().map_or(false, |v| v == &item.value)
                        }
                        ToggleGroupVariant::Multiple => {
                            current_values.contains(&item.value)
                        }
                    };
                    let is_disabled = disabled || item.disabled;
                    let value = item.value.clone();
                    let handler = on_change.clone();

                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .gap(px(6.0))
                        .h(size.height())
                        .px(size.px_value())
                        .rounded(theme.tokens.radius_sm)
                        .text_size(size.text_size())
                        .font_weight(FontWeight::MEDIUM)
                        .cursor(if is_disabled {
                            CursorStyle::Arrow
                        } else {
                            CursorStyle::PointingHand
                        })
                        .when(is_selected, |this: Div| {
                            this.bg(theme.tokens.background)
                                .text_color(theme.tokens.foreground)
                                .shadow(vec![BoxShadow {
                                    color: hsla(0.0, 0.0, 0.0, 0.05),
                                    offset: point(px(0.0), px(1.0)),
                                    blur_radius: px(2.0),
                                    spread_radius: px(0.0),
                                }])
                        })
                        .when(!is_selected, |this: Div| {
                            this.text_color(theme.tokens.muted_foreground)
                        })
                        .when(is_disabled, |this: Div| {
                            this.opacity(0.5)
                        })
                        .when(!is_disabled && !is_selected, |this: Div| {
                            this.hover(|style| {
                                style.bg(theme.tokens.muted.opacity(0.5))
                                    .text_color(theme.tokens.foreground)
                            })
                        })
                        .when(!is_disabled, |this: Div| {
                            this.on_mouse_down(MouseButton::Left, move |_, window, cx| {
                                if let Some(h) = &handler {
                                    h(&value, window, cx);
                                }
                            })
                        })
                        .when_some(item.icon, |this: Div, _icon| {
                            // TODO: Render icon when icon component is integrated
                            this
                        })
                        .child(item.label)
                })
            )
            .map(|this| {
                let mut div = this;
                div.style().refine(&user_style);
                div
            })
    }
}
