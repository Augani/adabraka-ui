use gpui::{prelude::FluentBuilder as _, *};
use std::rc::Rc;

use crate::components::spinner::Spinner;
use crate::theme::use_theme;

#[derive(Clone, Debug, PartialEq)]
pub enum LoadingState {
    Idle,
    Loading,
    Loaded,
    Error(SharedString),
    EndReached,
}

pub struct InfiniteScrollState {
    loading_state: LoadingState,
    page: usize,
    has_more: bool,
    scroll_handle: ScrollHandle,
}

impl InfiniteScrollState {
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|_| Self {
            loading_state: LoadingState::Idle,
            page: 0,
            has_more: true,
            scroll_handle: ScrollHandle::new(),
        })
    }

    pub fn loading_state(&self) -> &LoadingState {
        &self.loading_state
    }

    pub fn page(&self) -> usize {
        self.page
    }

    pub fn set_loading(&mut self) {
        self.loading_state = LoadingState::Loading;
    }

    pub fn set_loaded(&mut self) {
        self.page += 1;
        self.has_more = true;
        self.loading_state = LoadingState::Idle;
    }

    pub fn set_error(&mut self, msg: impl Into<SharedString>) {
        self.loading_state = LoadingState::Error(msg.into());
    }

    pub fn set_end_reached(&mut self) {
        self.has_more = false;
        self.loading_state = LoadingState::EndReached;
    }

    pub fn reset(&mut self) {
        self.page = 0;
        self.has_more = true;
        self.loading_state = LoadingState::Idle;
    }

    pub fn scroll_handle(&self) -> &ScrollHandle {
        &self.scroll_handle
    }
}

#[derive(IntoElement)]
pub struct InfiniteScroll {
    id: ElementId,
    state: Entity<InfiniteScrollState>,
    threshold: f32,
    on_load_more: Option<Rc<dyn Fn(usize, &mut Window, &mut App)>>,
    loading_indicator: Option<AnyElement>,
    end_indicator: Option<AnyElement>,
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl InfiniteScroll {
    #[track_caller]
    pub fn new(state: Entity<InfiniteScrollState>) -> Self {
        let location = std::panic::Location::caller();
        Self {
            id: ElementId::Name(
                format!(
                    "infinite-scroll:{}:{}:{}",
                    location.file(),
                    location.line(),
                    location.column()
                )
                .into(),
            ),
            state,
            threshold: 0.8,
            on_load_more: None,
            loading_indicator: None,
            end_indicator: None,
            children: Vec::new(),
            style: StyleRefinement::default(),
        }
    }

    pub fn threshold(mut self, threshold: f32) -> Self {
        self.threshold = threshold.clamp(0.0, 1.0);
        self
    }

    pub fn on_load_more(
        mut self,
        callback: impl Fn(usize, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_load_more = Some(Rc::new(callback));
        self
    }

    pub fn loading_indicator(mut self, indicator: impl IntoElement) -> Self {
        self.loading_indicator = Some(indicator.into_any_element());
        self
    }

    pub fn end_indicator(mut self, indicator: impl IntoElement) -> Self {
        self.end_indicator = Some(indicator.into_any_element());
        self
    }
}

impl ParentElement for InfiniteScroll {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for InfiniteScroll {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for InfiniteScroll {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let user_style = self.style;
        let (loading_state, scroll_handle) = {
            let s = self.state.read(cx);
            (s.loading_state.clone(), s.scroll_handle.clone())
        };

        let mut container = div()
            .id(self.id)
            .overflow_y_scroll()
            .track_scroll(&scroll_handle)
            .flex()
            .flex_col()
            .size_full();

        if let Some(callback) = self.on_load_more {
            let state_c = self.state.clone();
            let threshold = self.threshold;
            container = container.on_scroll_wheel(move |_event, window, cx| {
                let (should_load, page) = {
                    let s = state_c.read(cx);
                    if s.loading_state != LoadingState::Idle || !s.has_more {
                        return;
                    }
                    let handle = &s.scroll_handle;
                    let offset_y = (-handle.offset().y).max(px(0.0));
                    let max_y = handle.max_offset().height;
                    if max_y > px(0.0) && offset_y >= max_y * threshold {
                        (true, s.page)
                    } else {
                        return;
                    }
                };
                if should_load {
                    callback(page, window, cx);
                }
            });
        }

        container = container.children(self.children);

        match loading_state {
            LoadingState::Loading => {
                let indicator = self.loading_indicator.unwrap_or_else(|| {
                    div()
                        .flex()
                        .justify_center()
                        .py(px(16.0))
                        .child(Spinner::new())
                        .into_any_element()
                });
                container = container.child(indicator);
            }
            LoadingState::EndReached => {
                let indicator = self.end_indicator.unwrap_or_else(|| {
                    div()
                        .flex()
                        .justify_center()
                        .py(px(16.0))
                        .child(
                            div()
                                .text_size(px(14.0))
                                .text_color(theme.tokens.muted_foreground)
                                .font_family(theme.tokens.font_family.clone())
                                .child("No more items"),
                        )
                        .into_any_element()
                });
                container = container.child(indicator);
            }
            LoadingState::Error(ref msg) => {
                container = container.child(
                    div().flex().justify_center().py(px(16.0)).child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.destructive)
                            .font_family(theme.tokens.font_family.clone())
                            .child(msg.clone()),
                    ),
                );
            }
            _ => {}
        }

        container.map(|mut this| {
            this.style().refine(&user_style);
            this
        })
    }
}
