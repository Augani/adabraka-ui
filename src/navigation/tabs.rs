//! Tab navigation component with multiple visual variants.

use gpui::{prelude::FluentBuilder as _, *};
use std::sync::Arc;
use crate::theme::use_theme;
use crate::components::icon::Icon;
use crate::components::icon_source::IconSource;

actions!(tabs, [TabNext, TabPrevious, TabFirst, TabLast, TabClose]);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabVariant {
    Underline,
    Enclosed,
    Pills,
}

impl Default for TabVariant {
    fn default() -> Self {
        Self::Underline
    }
}

#[derive(Clone)]
pub struct TabItem<T: Clone> {
    pub id: T,
    pub label: SharedString,
    pub icon: Option<IconSource>,
    pub badge: Option<SharedString>,
    pub disabled: bool,
    pub closeable: bool,
}

impl<T: Clone> TabItem<T> {
    pub fn new(id: T, label: impl Into<SharedString>) -> Self {
        Self {
            id,
            label: label.into(),
            icon: None,
            badge: None,
            disabled: false,
            closeable: false,
        }
    }

    pub fn icon(mut self, icon: impl Into<IconSource>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn badge(mut self, badge: impl Into<SharedString>) -> Self {
        self.badge = Some(badge.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn closeable(mut self, closeable: bool) -> Self {
        self.closeable = closeable;
        self
    }
}

pub struct TabPanel {
    content: Box<dyn Fn() -> AnyElement + Send + Sync>,
}

impl TabPanel {
    pub fn new<F, E>(render_fn: F) -> Self
    where
        F: Fn() -> E + Send + Sync + 'static,
        E: IntoElement,
    {
        Self {
            content: Box::new(move || render_fn().into_any_element()),
        }
    }

    fn render(&self) -> AnyElement {
        (self.content)()
    }
}

#[derive(IntoElement)]
pub struct Tabs<T: Clone + PartialEq + 'static> {
    tabs: Vec<TabItem<T>>,
    panels: Vec<TabPanel>,
    selected_index: Option<usize>,
    variant: TabVariant,
    on_change: Option<Arc<dyn Fn(&usize, &mut Window, &mut App) + Send + Sync + 'static>>,
    on_close: Option<Arc<dyn Fn(&T, &mut Window, &mut App) + Send + Sync + 'static>>,
    style: StyleRefinement,
}

impl<T: Clone + PartialEq + 'static> Tabs<T> {
    pub fn new() -> Self {
        Self {
            tabs: Vec::new(),
            panels: Vec::new(),
            selected_index: Some(0),
            variant: TabVariant::default(),
            on_change: None,
            on_close: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn tabs(mut self, tabs: Vec<TabItem<T>>) -> Self {
        self.tabs = tabs;
        if let Some(index) = self.selected_index {
            if index >= self.tabs.len() {
                self.selected_index = Some(self.tabs.len().saturating_sub(1));
            }
        }
        self
    }

    pub fn panels(mut self, panels: Vec<TabPanel>) -> Self {
        self.panels = panels;
        self
    }

    pub fn variant(mut self, variant: TabVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn selected_index(mut self, index: usize) -> Self {
        self.selected_index = Some(index.min(self.tabs.len().saturating_sub(1)));
        self
    }

    pub fn selected_id(mut self, id: T) -> Self {
        if let Some(index) = self.tabs.iter().position(|tab| tab.id == id) {
            self.selected_index = Some(index);
        }
        self
    }

    pub fn on_change<F>(mut self, f: F) -> Self
    where
        F: Fn(&usize, &mut Window, &mut App) + Send + Sync + 'static,
    {
        self.on_change = Some(Arc::new(f));
        self
    }

    pub fn on_close<F>(mut self, f: F) -> Self
    where
        F: Fn(&T, &mut Window, &mut App) + Send + Sync + 'static,
    {
        self.on_close = Some(Arc::new(f));
        self
    }

    pub fn selected_tab_id(&self) -> Option<&T> {
        self.selected_index
            .and_then(|index| self.tabs.get(index))
            .map(|tab| &tab.id)
    }

    fn render_tab_button(
        variant: TabVariant,
        tab: &TabItem<T>,
        index: usize,
        is_active: bool,
        theme: &crate::theme::Theme,
        on_change: Option<Arc<dyn Fn(&usize, &mut Window, &mut App) + Send + Sync + 'static>>,
        on_close: Option<Arc<dyn Fn(&T, &mut Window, &mut App) + Send + Sync + 'static>>,
    ) -> impl IntoElement {
        let base = div()
            .flex()
            .items_center()
            .gap(px(6.0))
            .px(px(12.0))
            .py(px(8.0))
            .text_size(px(14.0))
            .font_family(theme.tokens.font_family.clone())
            .cursor(if tab.disabled {
                CursorStyle::Arrow
            } else {
                CursorStyle::PointingHand
            });

        let styled = match variant {
            TabVariant::Underline => base
                .text_color(if tab.disabled {
                    theme.tokens.muted_foreground
                } else {
                    theme.tokens.primary
                })
                .border_b_2()
                .border_color(if is_active {
                    theme.tokens.primary
                } else {
                    gpui::transparent_black()
                })
                .when(!tab.disabled && !is_active, |div| {
                    div.hover(|style| {
                        style.text_color(theme.tokens.primary)
                    })
                }),

            TabVariant::Enclosed => base
                .border_1()
                .border_color(if is_active {
                    theme.tokens.border
                } else {
                    gpui::transparent_black()
                })
                .rounded_tl(theme.tokens.radius_md)
                .rounded_tr(theme.tokens.radius_md)
                .bg(if is_active {
                    theme.tokens.background
                } else {
                    theme.tokens.muted
                })
                .text_color(if is_active {
                    theme.tokens.primary
                } else if tab.disabled {
                    theme.tokens.muted_foreground
                } else {
                    theme.tokens.primary
                })
                .when(!tab.disabled && !is_active, |div| {
                    div.hover(|mut style| {
                        style.background = Some(theme.tokens.accent.into());
                        style
                    })
                }),

            TabVariant::Pills => base
                .rounded(theme.tokens.radius_md)
                .bg(if is_active {
                    theme.tokens.primary
                } else if tab.disabled {
                    gpui::transparent_black()
                } else {
                    theme.tokens.muted
                })
                .text_color(if is_active {
                    theme.tokens.primary_foreground
                } else if tab.disabled {
                    theme.tokens.muted_foreground
                } else {
                    theme.tokens.foreground
                })
                .when(!tab.disabled && !is_active, |div| {
                    div.hover(|mut style| {
                        style.background = Some(theme.tokens.accent.into());
                        style
                    })
                }),
        };

        let with_icon = styled
            .when_some(tab.icon.as_ref(), |div, icon| {
                div.child(
                    Icon::new(icon.clone())
                        .size(px(14.0))
                        .color(if is_active && variant == TabVariant::Pills {
                            theme.tokens.primary_foreground
                        } else if tab.disabled {
                            theme.tokens.muted_foreground
                        } else {
                            theme.tokens.primary
                        }),
                )
            });

        let with_label = with_icon.child(div().child(tab.label.clone()));

        let with_badge = with_label.when_some(tab.badge.as_ref(), |parent, badge| {
            parent.child(
                div()
                    .px(px(6.0))
                    .py(px(2.0))
                    .rounded(px(10.0))
                    .bg(if is_active && variant == TabVariant::Pills {
                        theme.tokens.primary_foreground.opacity(0.2)
                    } else {
                        theme.tokens.muted
                    })
                    .text_size(px(11.0))
                    .font_family(theme.tokens.font_family.clone())
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(if is_active && variant == TabVariant::Pills {
                        theme.tokens.primary_foreground
                    } else {
                        theme.tokens.muted_foreground
                    })
                    .child(badge.clone()),
            )
        });

        let with_close = with_badge.when(tab.closeable, |parent| {
            parent.child(
                div()
                    .ml(px(4.0))
                    .p(px(2.0))
                    .rounded(px(4.0))
                    .cursor(CursorStyle::PointingHand)
                    .hover(|mut style| {
                        style.background = Some(if is_active && variant == TabVariant::Pills {
                            theme.tokens.primary_foreground.opacity(0.2).into()
                        } else {
                            theme.tokens.muted.into()
                        });
                        style
                    })
                    .on_mouse_down(MouseButton::Left, {
                        let on_close = on_close.clone();
                        let tab_id = tab.id.clone();
                        move |_, window, cx| {
                            if let Some(on_close) = on_close.clone() {
                                on_close(&tab_id, window, cx);
                            }
                        }
                    })
                    .child(
                        Icon::new("x")
                            .size(px(12.0))
                            .color(if is_active && variant == TabVariant::Pills {
                                theme.tokens.primary_foreground
                            } else {
                                theme.tokens.muted_foreground
                            }),
                    ),
            )
        });

        with_close.when(!tab.disabled, |this| {
            this.on_mouse_down(MouseButton::Left, {
                let on_change = on_change.clone();
                move |_, window, cx| {
                    if let Some(on_change) = on_change.clone() {
                        on_change(&index, window, cx);
                    }
                }
            })
        })
    }
}

impl<T: Clone + PartialEq + 'static> Styled for Tabs<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl<T: Clone + PartialEq + 'static> RenderOnce for Tabs<T> {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let user_style = self.style;

        if self.tabs.is_empty() {
            return div().child("No tabs");
        }

        let mut tab_list = div()
            .flex()
            .gap(px(4.0))
            .when(self.variant == TabVariant::Underline, |div| {
                div.border_b_1().border_color(theme.tokens.border)
            })
            .when(self.variant == TabVariant::Pills, |div| {
                div.p(px(4.0))
                    .bg(theme.tokens.muted)
                    .rounded(theme.tokens.radius_md)
            });

        for (index, tab) in self.tabs.iter().enumerate() {
            let is_active = Some(index) == self.selected_index;
            tab_list = tab_list.child(Self::render_tab_button(
                self.variant,
                tab,
                index,
                is_active,
                &theme,
                self.on_change.clone(),
                self.on_close.clone(),
            ));
        }

        let tab_list = tab_list;

        let active_panel = self
            .selected_index
            .and_then(|index| self.panels.get(index))
            .map(|panel| panel.render());

        let mut root = div()
            .flex()
            .flex_col()
            .size_full()
            .gap(px(16.0))
            .child(tab_list);

        if let Some(panel) = active_panel {
            root = root.child(
                div()
                    .flex_1()
                    .min_h(px(0.0))
                    .overflow_hidden()
                    .child(div().size_full().child(panel))
            );
        }

        root.map(|this| {
            let mut div = this;
            div.style().refine(&user_style);
            div
        })
    }
}

pub fn init_tabs(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("right", TabNext, Some("Tabs")),
        KeyBinding::new("left", TabPrevious, Some("Tabs")),
        KeyBinding::new("home", TabFirst, Some("Tabs")),
        KeyBinding::new("end", TabLast, Some("Tabs")),
        KeyBinding::new("cmd-w", TabClose, Some("Tabs")),
    ]);
}
