//! Accordion - Collapsible content sections with smooth animations.

use gpui::{prelude::FluentBuilder as _, *};
use std::rc::Rc;
use crate::{
    theme::use_theme,
    components::icon::Icon,
    components::icon_source::IconSource,
};

#[derive(IntoElement)]
pub struct Accordion {
    id: ElementId,
    items: Vec<AccordionItem>,
    multiple: bool,
    bordered: bool,
    disabled: bool,
    open_indices: Vec<usize>,
    on_change: Option<Rc<dyn Fn(&[usize], &mut Window, &mut App)>>,
    style: StyleRefinement,
}

impl Accordion {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            items: Vec::new(),
            multiple: false,
            bordered: true,
            disabled: false,
            open_indices: Vec::new(),
            on_change: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn multiple(mut self, multiple: bool) -> Self {
        self.multiple = multiple;
        self
    }

    pub fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn item<F>(mut self, builder: F) -> Self
    where
        F: FnOnce(AccordionItem) -> AccordionItem,
    {
        let item = builder(AccordionItem::new(self.items.len()));
        if item.is_open {
            self.open_indices.push(self.items.len());
        }
        self.items.push(item);
        self
    }

    pub fn on_change<F>(mut self, callback: F) -> Self
    where
        F: Fn(&[usize], &mut Window, &mut App) + 'static,
    {
        self.on_change = Some(Rc::new(callback));
        self
    }
}

impl Styled for Accordion {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Accordion {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let _theme = use_theme();
        let user_style = self.style;
        let open_indices = Rc::new(std::cell::RefCell::new(self.open_indices));
        let multiple = self.multiple;
        let on_change = self.on_change;

        div()
            .id(self.id)
            .flex()
            .flex_col()
            .w_full()
            .gap(if self.bordered { px(8.0) } else { px(0.0) })
            .children(self.items.into_iter().map(|item| {
                let item_index = item.index;
                let is_open = open_indices.borrow().contains(&item_index);
                let open_indices_clone = open_indices.clone();
                let on_change_clone = on_change.clone();

                item.bordered(self.bordered)
                    .disabled(self.disabled)
                    .is_open(is_open)
                    .on_toggle(move |is_opening, window, cx| {
                        let mut indices = open_indices_clone.borrow_mut();

                        if is_opening {
                            if !multiple {
                                indices.clear();
                            }
                            indices.push(item_index);
                        } else {
                            indices.retain(|&i| i != item_index);
                        }

                        if let Some(ref callback) = on_change_clone {
                            let open_vec: Vec<usize> = indices.iter().copied().collect();
                            callback(&open_vec, window, cx);
                        }
                    })
            }))
            .map(|this| {
                let mut div = this;
                div.style().refine(&user_style);
                div
            })
    }
}

#[derive(IntoElement)]
pub struct AccordionItem {
    index: usize,
    title: SharedString,
    content: Option<AnyElement>,
    icon: Option<IconSource>,
    is_open: bool,
    bordered: bool,
    disabled: bool,
    on_toggle: Option<Rc<dyn Fn(bool, &mut Window, &mut App)>>,
}

impl AccordionItem {
    fn new(index: usize) -> Self {
        Self {
            index,
            title: SharedString::from(""),
            content: None,
            icon: None,
            is_open: false,
            bordered: true,
            disabled: false,
            on_toggle: None,
        }
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = title.into();
        self
    }

    pub fn content(mut self, content: impl IntoElement) -> Self {
        self.content = Some(content.into_any_element());
        self
    }

    pub fn icon(mut self, icon: impl Into<IconSource>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.is_open = open;
        self
    }

    fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }

    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    fn is_open(mut self, is_open: bool) -> Self {
        self.is_open = is_open;
        self
    }

    fn on_toggle<F>(mut self, callback: F) -> Self
    where
        F: Fn(bool, &mut Window, &mut App) + 'static,
    {
        self.on_toggle = Some(Rc::new(callback));
        self
    }
}

impl RenderOnce for AccordionItem {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let is_open = self.is_open;

        div()
            .flex()
            .flex_col()
            .w_full()
            .overflow_hidden()
            .bg(theme.tokens.card)
            .when(self.bordered, |div| {
                div.border_1()
                    .border_color(theme.tokens.border)
                    .rounded(theme.tokens.radius_md)
            })
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .px(px(16.0))
                    .py(px(12.0))
                    .cursor(if self.disabled {
                        CursorStyle::Arrow
                    } else {
                        CursorStyle::PointingHand
                    })
                    .when(!self.disabled, |div| {
                        div.hover(|style| style.bg(theme.tokens.muted.opacity(0.5)))
                    })
                    .when(self.is_open && self.bordered, |div| {
                        div.border_b_1().border_color(theme.tokens.border)
                    })
                    .when_some(self.on_toggle.filter(|_| !self.disabled), |div, callback| {
                        div.on_mouse_down(MouseButton::Left, move |_event, window, cx| {
                            callback(!is_open, window, cx);
                        })
                    })
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(12.0))
                            .when_some(self.icon, |div, icon| {
                                div.child(
                                    Icon::new(icon)
                                        .size(px(18.0))
                                        .color(theme.tokens.muted_foreground)
                                )
                            })
                            .child(
                                div()
                                    .text_size(px(15.0))
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(theme.tokens.foreground)
                                    .child(self.title)
                            )
                    )
                    .child(
                        Icon::new(if is_open { "chevron-up" } else { "chevron-down" })
                            .size(px(16.0))
                            .color(theme.tokens.muted_foreground)
                    )
            )
            .when(is_open, |parent| {
                parent.child(
                    div()
                        .px(px(16.0))
                        .py(px(12.0))
                        .text_size(px(14.0))
                        .text_color(theme.tokens.muted_foreground)
                        .when_some(self.content, |content_div, content| {
                            content_div.child(content)
                        })
                )
            })
    }
}
