//! Breadcrumb navigation component for hierarchical navigation.

use gpui::{prelude::FluentBuilder as _, *};
use crate::theme::use_theme;
use crate::components::icon::Icon;
use crate::components::icon_source::IconSource;
use std::sync::Arc;

pub struct BreadcrumbItem<T> {
    pub id: T,
    pub label: SharedString,
    pub icon: Option<IconSource>,
}

#[derive(IntoElement)]
pub struct Breadcrumbs<T: Clone + 'static> {
    items: Vec<BreadcrumbItem<T>>,
    on_click: Option<Arc<dyn Fn(&T, &mut Window, &mut App) + Send + Sync + 'static>>,
    style: StyleRefinement,
}

impl<T: Clone + 'static> Breadcrumbs<T> {
    pub fn new(_cx: &mut App) -> Self {
        Self {
            items: Vec::new(),
            on_click: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn items(mut self, items: Vec<BreadcrumbItem<T>>) -> Self {
        self.items = items;
        self
    }

    pub fn on_click<F: Fn(&T, &mut Window, &mut App) + Send + Sync + 'static>(mut self, f: F) -> Self {
        self.on_click = Some(Arc::new(f));
        self
    }
}

impl<T: Clone + 'static> Styled for Breadcrumbs<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl<T: Clone + 'static> RenderOnce for Breadcrumbs<T> {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let user_style = self.style;

        if self.items.is_empty() {
            return div().into();
        }

        let mut elements: Vec<gpui::Div> = Vec::new();
        let on_click = self.on_click.clone();

        for (index, item) in self.items.iter().enumerate() {
            let item_id = item.id.clone();
            let is_last = index == self.items.len() - 1;
            let is_first = index == 0;

            if index > 0 {
                let separator = div()
                    .mx(px(6.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .w(px(16.0))
                    .h(px(16.0))
                    .text_color(theme.tokens.muted_foreground)
                    .child("❯");
                elements.push(separator);
            }

            let mut breadcrumb_element = div()
                .flex()
                .items_center()
                .gap(px(4.0))
                .px(px(2.0))
                .py(px(2.0))
                .rounded(px(4.0))
                .text_size(px(14.0))
                .font_family(theme.tokens.font_family.clone());

            if let Some(icon_source) = &item.icon {
                breadcrumb_element = breadcrumb_element.child(
                    Icon::new(icon_source.clone())
                        .size(px(14.0))
                        .color(if is_last {
                            theme.tokens.foreground
                        } else {
                            theme.tokens.primary
                        })
                );
            } else if is_first {
                breadcrumb_element = breadcrumb_element.child(
                    Icon::new(IconSource::Named("globe".to_string()))
                        .size(px(14.0))
                        .color(if is_last {
                            theme.tokens.foreground
                        } else {
                            theme.tokens.primary
                        })
                );
            }

            if is_last {
                breadcrumb_element = breadcrumb_element
                    .text_color(theme.tokens.foreground)
                    .font_weight(FontWeight::SEMIBOLD)
                    .child(item.label.clone());
            } else {
                let item_id_clone = item_id.clone();
                let on_click_clone = on_click.clone();

                breadcrumb_element = breadcrumb_element
                    .text_color(theme.tokens.primary)
                    .cursor(CursorStyle::PointingHand)
                    .hover(|style| {
                        style
                            .bg(theme.tokens.accent.opacity(0.1))
                            .text_color(theme.tokens.primary.opacity(0.8))
                    })
                    .on_mouse_down(MouseButton::Left, {
                        let on_click_clone = on_click_clone.clone();
                        let item_id_clone = item_id_clone.clone();
                        move |_, window, cx| {
                            if let Some(on_click) = on_click_clone.clone() {
                                on_click(&item_id_clone, window, cx);
                            }
                        }
                    })
                    .child(item.label.clone());
            }

            elements.push(breadcrumb_element);
        }
        div()
            .flex()
            .items_center()
            .flex_wrap()
            .gap(px(2.0))
            .children(elements)
            .map(|this| {
                let mut div = this;
                div.style().refine(&user_style);
                div
            })
    }
}
