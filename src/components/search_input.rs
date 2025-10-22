//! Search input component - Specialized search with filters and advanced capabilities.

use gpui::{prelude::FluentBuilder as _, InteractiveElement, *};
use std::rc::Rc;
use crate::{
    theme::use_theme,
    components::{
        input::Input,
        input_state::{InputState, InputEvent},
        icon::Icon,
        icon_source::IconSource,
    },
};

#[derive(Clone, Debug, PartialEq)]
pub struct SearchFilter {
    pub label: SharedString,
    pub active: bool,
    pub value: SharedString,
}

impl SearchFilter {
    pub fn new(label: impl Into<SharedString>, value: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            active: false,
            value: value.into(),
        }
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }
}

pub struct SearchInputState {
    input: Entity<InputState>,
    filters: Vec<SearchFilter>,
    case_sensitive: bool,
    use_regex: bool,
    loading: bool,
    results_count: Option<usize>,
    on_search: Option<Rc<dyn Fn(&str, &mut App)>>,
    on_filter_toggle: Option<Rc<dyn Fn(usize, &mut App)>>,
}

impl SearchInputState {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let input = cx.new(|cx| {
            InputState::new(cx).placeholder("Search...")
        });

        Self {
            input,
            filters: Vec::new(),
            case_sensitive: false,
            use_regex: false,
            loading: false,
            results_count: None,
            on_search: None,
            on_filter_toggle: None,
        }
    }

    pub fn placeholder(self, placeholder: impl Into<SharedString>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        self.input.update(cx, |input, cx| {
            input.set_placeholder(placeholder, window, cx);
        });
        self
    }

    pub fn set_filters(&mut self, filters: Vec<SearchFilter>, cx: &mut Context<Self>) {
        self.filters = filters;
        cx.notify();
    }

    pub fn toggle_filter(&mut self, index: usize, cx: &mut Context<Self>) {
        if let Some(filter) = self.filters.get_mut(index) {
            filter.active = !filter.active;
            if let Some(handler) = &self.on_filter_toggle {
                handler(index, cx);
            }
            cx.notify();
        }
    }

    pub fn set_case_sensitive(&mut self, case_sensitive: bool, cx: &mut Context<Self>) {
        self.case_sensitive = case_sensitive;
        cx.notify();
    }

    pub fn toggle_case_sensitive(&mut self, cx: &mut Context<Self>) {
        self.case_sensitive = !self.case_sensitive;
        cx.notify();
    }

    pub fn set_use_regex(&mut self, use_regex: bool, cx: &mut Context<Self>) {
        self.use_regex = use_regex;
        cx.notify();
    }

    pub fn toggle_use_regex(&mut self, cx: &mut Context<Self>) {
        self.use_regex = !self.use_regex;
        cx.notify();
    }

    pub fn set_loading(&mut self, loading: bool, cx: &mut Context<Self>) {
        self.loading = loading;
        cx.notify();
    }

    pub fn set_results_count(&mut self, count: Option<usize>, cx: &mut Context<Self>) {
        self.results_count = count;
        cx.notify();
    }

    pub fn clear(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.input.update(cx, |input, cx| {
            input.set_value("", window, cx);
        });
        self.results_count = None;
        cx.notify();
    }

    pub fn query(&self, cx: &App) -> String {
        self.input.read(cx).content().to_string()
    }

    pub fn active_filters(&self) -> Vec<&SearchFilter> {
        self.filters.iter().filter(|f| f.active).collect()
    }

    pub fn case_sensitive(&self) -> bool {
        self.case_sensitive
    }

    pub fn use_regex(&self) -> bool {
        self.use_regex
    }
}

pub struct SearchInput {
    state: Entity<SearchInputState>,
    style: StyleRefinement,
}

impl SearchInput {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let state = cx.new(|cx| SearchInputState::new(cx));

        let state_clone = state.clone();
        let input_entity = state.read(cx).input.clone();
        cx.subscribe(&input_entity, move |_this, _input, event, cx| {
            match event {
                InputEvent::Change => {
                    let query = state_clone.read(cx).input.read(cx).content().to_string();
                    let handler = state_clone.read(cx).on_search.clone();
                    if let Some(handler) = handler {
                        cx.defer(move |cx| {
                            handler(&query, cx);
                        });
                    }
                }
                _ => {}
            }
        }).detach();

        Self {
            state,
            style: StyleRefinement::default(),
        }
    }

    pub fn placeholder(self, placeholder: impl Into<SharedString>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        self.state.update(cx, |state, cx| {
            state.input.update(cx, |input, cx| {
                input.set_placeholder(placeholder, window, cx);
            });
        });
        self
    }

    pub fn filters(self, filters: Vec<SearchFilter>, cx: &mut Context<Self>) -> Self {
        self.state.update(cx, |state, cx| {
            state.set_filters(filters, cx);
        });
        self
    }

    pub fn on_search<F>(self, handler: F, cx: &mut Context<Self>) -> Self
    where
        F: Fn(&str, &mut App) + 'static,
    {
        self.state.update(cx, |state, _cx| {
            state.on_search = Some(Rc::new(handler));
        });
        self
    }

    pub fn on_filter_toggle<F>(self, handler: F, cx: &mut Context<Self>) -> Self
    where
        F: Fn(usize, &mut App) + 'static,
    {
        self.state.update(cx, |state, _cx| {
            state.on_filter_toggle = Some(Rc::new(handler));
        });
        self
    }

    pub fn state(&self) -> &Entity<SearchInputState> {
        &self.state
    }
}

impl Styled for SearchInput {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl Render for SearchInput {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();
        let state = self.state.read(cx);
        let has_query = !state.input.read(cx).content().is_empty();
        let user_style = self.style.clone();

        div()
            .flex()
            .flex_col()
            .gap(px(8.0))
            // Main search input row
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .px(px(12.0))
                    .py(px(8.0))
                    .bg(theme.tokens.input)
                    .border_1()
                    .border_color(theme.tokens.border)
                    .rounded(theme.tokens.radius_md)
                    .child(
                        Icon::new(if state.loading {
                            IconSource::Named("loader".into())
                        } else {
                            IconSource::Named("search".into())
                        })
                        .size(px(16.0))
                        .color(theme.tokens.muted_foreground)
                    )
                    .child(
                        div()
                            .flex_1()
                            .child(Input::new(&state.input))
                    )
                    .when_some(state.results_count, |parent_div, count| {
                        parent_div.child(
                            div()
                                .px(px(8.0))
                                .py(px(2.0))
                                .rounded(theme.tokens.radius_sm)
                                .bg(theme.tokens.muted)
                                .text_size(px(12.0))
                                .text_color(theme.tokens.muted_foreground)
                                .child(format!("{} results", count))
                        )
                    })
                    .child(
                        div()
                            .px(px(8.0))
                            .py(px(4.0))
                            .rounded(theme.tokens.radius_sm)
                            .cursor(CursorStyle::PointingHand)
                            .when(state.case_sensitive, |div| div.bg(theme.tokens.accent))
                            .when(!state.case_sensitive, |div| {
                                div.hover(|style| style.bg(theme.tokens.muted))
                            })
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                this.state.update(cx, |state, cx| {
                                    state.toggle_case_sensitive(cx);
                                });
                            }))
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(if state.case_sensitive {
                                        theme.tokens.accent_foreground
                                    } else {
                                        theme.tokens.muted_foreground
                                    })
                                    .child("Aa")
                            )
                    )
                    .child(
                        div()
                            .px(px(8.0))
                            .py(px(4.0))
                            .rounded(theme.tokens.radius_sm)
                            .cursor(CursorStyle::PointingHand)
                            .when(state.use_regex, |div| div.bg(theme.tokens.accent))
                            .when(!state.use_regex, |div| {
                                div.hover(|style| style.bg(theme.tokens.muted))
                            })
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                this.state.update(cx, |state, cx| {
                                    state.toggle_use_regex(cx);
                                });
                            }))
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(if state.use_regex {
                                        theme.tokens.accent_foreground
                                    } else {
                                        theme.tokens.muted_foreground
                                    })
                                    .child(".*")
                            )
                    )
                    .when(has_query, |parent_div| {
                        parent_div.child(
                            div()
                                .p(px(4.0))
                                .rounded(theme.tokens.radius_sm)
                                .cursor(CursorStyle::PointingHand)
                                .hover(|style| style.bg(theme.tokens.muted))
                                .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, window, cx| {
                                    this.state.update(cx, |state, cx| {
                                        state.clear(window, cx);
                                    });
                                }))
                                .child(
                                    Icon::new(IconSource::Named("x".into()))
                                        .size(px(14.0))
                                        .color(theme.tokens.muted_foreground)
                                )
                        )
                    })
            )
            .when(!state.filters.is_empty(), |parent_div| {
                parent_div.child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(8.0))
                        .flex_wrap()
                        .children(state.filters.iter().enumerate().map(|(idx, filter)| {
                            let filter = filter.clone();
                            div()
                                .px(px(8.0))
                                .py(px(4.0))
                                .rounded(theme.tokens.radius_sm)
                                .border_1()
                                .border_color(if filter.active {
                                    theme.tokens.accent
                                } else {
                                    theme.tokens.border
                                })
                                .bg(if filter.active {
                                    theme.tokens.accent.opacity(0.1)
                                } else {
                                    theme.tokens.background
                                })
                                .cursor(CursorStyle::PointingHand)
                                .hover(|style| style.bg(theme.tokens.muted))
                                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, _window, cx| {
                                    this.state.update(cx, |state, cx| {
                                        state.toggle_filter(idx, cx);
                                    });
                                }))
                                .child(
                                    div()
                                        .text_size(px(12.0))
                                        .text_color(if filter.active {
                                            theme.tokens.accent
                                        } else {
                                            theme.tokens.foreground
                                        })
                                        .child(filter.label.clone())
                                )
                                .into_any_element()
                        }))
                )
            })
            .map(|this| {
                let mut div = this;
                div.style().refine(&user_style);
                div
            })
    }
}
