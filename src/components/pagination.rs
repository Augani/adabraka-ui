//! Pagination component for navigating through paginated content.

use gpui::{prelude::FluentBuilder as _, *};
use std::rc::Rc;

use crate::theme::use_theme;
use crate::components::button::{Button, ButtonVariant, ButtonSize};

#[derive(IntoElement)]
pub struct Pagination {
    current_page: usize,
    total_pages: usize,
    siblings: usize,
    show_edges: bool,
    on_page_change: Option<Rc<dyn Fn(usize, &mut Window, &mut App)>>,
    style: StyleRefinement,
}

impl Pagination {
    pub fn new() -> Self {
        Self {
            current_page: 1,
            total_pages: 1,
            siblings: 1,
            show_edges: true,
            on_page_change: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn current_page(mut self, page: usize) -> Self {
        self.current_page = page.max(1);
        self
    }

    pub fn total_pages(mut self, total: usize) -> Self {
        self.total_pages = total.max(1);
        self
    }

    pub fn siblings(mut self, siblings: usize) -> Self {
        self.siblings = siblings;
        self
    }

    pub fn show_edges(mut self, show: bool) -> Self {
        self.show_edges = show;
        self
    }

    pub fn on_page_change<F>(mut self, handler: F) -> Self
    where
        F: Fn(usize, &mut Window, &mut App) + 'static,
    {
        self.on_page_change = Some(Rc::new(handler));
        self
    }

    fn get_page_range(&self) -> Vec<PageItem> {
        let mut items = Vec::new();
        let current = self.current_page;
        let total = self.total_pages;
        let siblings = self.siblings;

        if total <= 7 {
            for i in 1..=total {
                items.push(PageItem::Page(i));
            }
            return items;
        }

        let left_sibling = (current.saturating_sub(siblings)).max(1);
        let right_sibling = (current + siblings).min(total);

        let show_left_ellipsis = left_sibling > 2;
        let show_right_ellipsis = right_sibling < total - 1;

        if self.show_edges {
            items.push(PageItem::Page(1));
        }

        if show_left_ellipsis {
            items.push(PageItem::Ellipsis);
        } else if left_sibling == 2 {
            items.push(PageItem::Page(2));
        }

        for i in left_sibling..=right_sibling {
            if i != 1 && i != total {
                items.push(PageItem::Page(i));
            }
        }

        if show_right_ellipsis {
            items.push(PageItem::Ellipsis);
        } else if right_sibling == total - 1 {
            items.push(PageItem::Page(total - 1));
        }

        if self.show_edges {
            items.push(PageItem::Page(total));
        }

        items
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for Pagination {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

#[derive(Clone, Copy, Debug)]
enum PageItem {
    Page(usize),
    Ellipsis,
}

impl RenderOnce for Pagination {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let current_page = self.current_page;
        let total_pages = self.total_pages;
        let page_range = self.get_page_range();
        let on_change = self.on_page_change;
        let user_style = self.style;

        let has_prev = current_page > 1;
        let has_next = current_page < total_pages;

        div()
            .flex()
            .items_center()
            .gap(px(4.0))
            .child({
                let handler = on_change.clone();
                Button::new("previous-btn", "Previous")
                    .variant(ButtonVariant::Outline)
                    .size(ButtonSize::Sm)
                    .disabled(!has_prev)
                    .when(has_prev && handler.is_some(), |btn| {
                        let handler = handler.clone().unwrap();
                        btn.on_click(move |_, window, cx| {
                            handler(current_page - 1, window, cx);
                        })
                    })
            })
            .children({
                let on_change_for_pages = on_change.clone();
                page_range.into_iter().map(move |item| {
                    match item {
                        PageItem::Page(page) => {
                            let is_current = page == current_page;
                            let handler = on_change_for_pages.clone();

                        div()
                            .child(
                                Button::new(("page-btn", page), page.to_string())
                                    .variant(if is_current {
                                        ButtonVariant::Default
                                    } else {
                                        ButtonVariant::Outline
                                    })
                                    .size(ButtonSize::Sm)
                                    .disabled(is_current)
                                    .when(!is_current && handler.is_some(), |btn| {
                                        let handler = handler.clone().unwrap();
                                        btn.on_click(move |_, window, cx| {
                                            handler(page, window, cx);
                                        })
                                    })
                            )
                            .into_any_element()
                    }
                    PageItem::Ellipsis => {
                        div()
                            .flex()
                            .items_center()
                            .justify_center()
                            .h(px(36.0))
                            .w(px(36.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("...")
                            .into_any_element()
                    }
                }
            })
            })
            .child({
                let handler = on_change;
                Button::new("next-btn", "Next")
                    .variant(ButtonVariant::Outline)
                    .size(ButtonSize::Sm)
                    .disabled(!has_next)
                    .when(has_next && handler.is_some(), |btn| {
                        let handler = handler.unwrap();
                        btn.on_click(move |_, window, cx| {
                            handler(current_page + 1, window, cx);
                        })
                    })
            })
            .map(|this| {
                let mut div = this;
                div.style().refine(&user_style);
                div
            })
    }
}
