use crate::theme::use_theme;
use gpui::{prelude::FluentBuilder as _, *};
use std::rc::Rc;

pub struct TagInputState {
    tags: Vec<SharedString>,
    input_value: String,
    focus_handle: FocusHandle,
    max_tags: Option<usize>,
}

impl TagInputState {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            tags: Vec::new(),
            input_value: String::new(),
            focus_handle: cx.focus_handle(),
            max_tags: None,
        }
    }

    pub fn with_tags(cx: &mut Context<Self>, tags: Vec<impl Into<SharedString>>) -> Self {
        Self {
            tags: tags.into_iter().map(|t| t.into()).collect(),
            input_value: String::new(),
            focus_handle: cx.focus_handle(),
            max_tags: None,
        }
    }

    pub fn tags(&self) -> &[SharedString] {
        &self.tags
    }

    pub fn set_tags(&mut self, tags: Vec<impl Into<SharedString>>, cx: &mut Context<Self>) {
        self.tags = tags.into_iter().map(|t| t.into()).collect();
        cx.notify();
    }

    pub fn add_tag(&mut self, tag: impl Into<SharedString>, cx: &mut Context<Self>) -> bool {
        let tag = tag.into();
        if tag.is_empty() {
            return false;
        }
        if self.tags.iter().any(|t| t.as_ref() == tag.as_ref()) {
            return false;
        }
        if let Some(max) = self.max_tags {
            if self.tags.len() >= max {
                return false;
            }
        }
        self.tags.push(tag);
        cx.notify();
        true
    }

    pub fn remove_tag(&mut self, index: usize, cx: &mut Context<Self>) {
        if index < self.tags.len() {
            self.tags.remove(index);
            cx.notify();
        }
    }

    pub fn remove_last_tag(&mut self, cx: &mut Context<Self>) {
        if !self.tags.is_empty() {
            self.tags.pop();
            cx.notify();
        }
    }

    pub fn clear_tags(&mut self, cx: &mut Context<Self>) {
        self.tags.clear();
        cx.notify();
    }

    pub fn input_value(&self) -> &str {
        &self.input_value
    }

    pub fn set_input_value(&mut self, value: impl Into<String>, cx: &mut Context<Self>) {
        self.input_value = value.into();
        cx.notify();
    }

    pub fn max_tags(&self) -> Option<usize> {
        self.max_tags
    }

    pub fn set_max_tags(&mut self, max: Option<usize>, cx: &mut Context<Self>) {
        self.max_tags = max;
        cx.notify();
    }

    pub fn commit_input(&mut self, cx: &mut Context<Self>) -> bool {
        let value = self.input_value.trim().to_string();
        if self.add_tag(value, cx) {
            self.input_value.clear();
            true
        } else {
            false
        }
    }
}

impl Focusable for TagInputState {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for TagInputState {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}

#[derive(IntoElement)]
pub struct TagInput {
    state: Entity<TagInputState>,
    placeholder: SharedString,
    disabled: bool,
    suggestions: Vec<SharedString>,
    on_change: Option<Rc<dyn Fn(&[SharedString], &mut Window, &mut App)>>,
    style: StyleRefinement,
}

impl TagInput {
    pub fn new(state: Entity<TagInputState>) -> Self {
        Self {
            state,
            placeholder: "Add tag...".into(),
            disabled: false,
            suggestions: Vec::new(),
            on_change: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn suggestions(mut self, suggestions: Vec<impl Into<SharedString>>) -> Self {
        self.suggestions = suggestions.into_iter().map(|s| s.into()).collect();
        self
    }

    pub fn on_change(
        mut self,
        handler: impl Fn(&[SharedString], &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }
}

impl Styled for TagInput {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for TagInput {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let user_style = self.style;
        let state_data = self.state.read(cx);
        let tags = state_data.tags.clone();
        let input_value = state_data.input_value.clone();
        let focus_handle = state_data.focus_handle(cx);
        let is_focused = focus_handle.is_focused(window);
        let state = self.state.clone();

        div()
            .map(|this| {
                let mut div = this;
                div.style().refine(&user_style);
                div
            })
            .child(
                div()
                    .id("tag-input-container")
                    .flex()
                    .flex_wrap()
                    .items_center()
                    .gap(px(6.0))
                    .min_h(px(40.0))
                    .px(px(10.0))
                    .py(px(6.0))
                    .bg(theme.tokens.background)
                    .border_1()
                    .border_color(if is_focused {
                        theme.tokens.ring
                    } else {
                        theme.tokens.input
                    })
                    .rounded(theme.tokens.radius_md)
                    .when(self.disabled, |d| d.opacity(0.5))
                    .when(!self.disabled, |d| {
                        d.track_focus(&focus_handle.tab_index(0).tab_stop(true))
                    })
                    .children(tags.iter().enumerate().map(|(idx, tag)| {
                        let state_for_remove = state.clone();
                        let on_change = self.on_change.clone();
                        let disabled = self.disabled;

                        div()
                            .id(SharedString::from(format!("tag-{}", idx)))
                            .flex()
                            .items_center()
                            .gap(px(4.0))
                            .px(px(8.0))
                            .py(px(4.0))
                            .bg(theme.tokens.primary.opacity(0.1))
                            .text_color(theme.tokens.primary)
                            .rounded(px(4.0))
                            .text_size(px(13.0))
                            .font_family(theme.tokens.font_family.clone())
                            .child(tag.clone())
                            .when(!disabled, |d| {
                                d.child(
                                    div()
                                        .id(SharedString::from(format!("tag-remove-{}", idx)))
                                        .ml(px(2.0))
                                        .cursor_pointer()
                                        .rounded(px(2.0))
                                        .hover(|s| s.bg(theme.tokens.primary.opacity(0.2)))
                                        .text_size(px(12.0))
                                        .on_click(move |_, window, cx| {
                                            state_for_remove.update(cx, |s, cx| {
                                                s.remove_tag(idx, cx);
                                                if let Some(ref handler) = on_change {
                                                    handler(&s.tags, window, cx);
                                                }
                                            });
                                        })
                                        .child("×"),
                                )
                            })
                    }))
                    .when(!self.disabled, {
                        let state_for_input = state.clone();
                        let on_change = self.on_change.clone();
                        let placeholder = self.placeholder.clone();

                        move |container| {
                            container.child(
                                div().flex_1().min_w(px(80.0)).child(
                                    div()
                                        .id("tag-input-field")
                                        .min_w(px(60.0))
                                        .text_size(px(14.0))
                                        .text_color(if input_value.is_empty() {
                                            theme.tokens.muted_foreground
                                        } else {
                                            theme.tokens.foreground
                                        })
                                        .font_family(theme.tokens.font_family.clone())
                                        .on_key_down({
                                            let state = state_for_input.clone();
                                            let on_change = on_change.clone();
                                            move |event, window, cx| match event
                                                .keystroke
                                                .key
                                                .as_str()
                                            {
                                                "enter" => {
                                                    state.update(cx, |s, cx| {
                                                        if s.commit_input(cx) {
                                                            if let Some(ref handler) = on_change {
                                                                handler(&s.tags, window, cx);
                                                            }
                                                        }
                                                    });
                                                    cx.stop_propagation();
                                                }
                                                "backspace" => {
                                                    state.update(cx, |s, cx| {
                                                        if s.input_value.is_empty()
                                                            && !s.tags.is_empty()
                                                        {
                                                            s.remove_last_tag(cx);
                                                            if let Some(ref handler) = on_change {
                                                                handler(&s.tags, window, cx);
                                                            }
                                                            cx.stop_propagation();
                                                        }
                                                    });
                                                }
                                                _ => {}
                                            }
                                        })
                                        .child(if input_value.is_empty() {
                                            placeholder.to_string()
                                        } else {
                                            input_value
                                        }),
                                ),
                            )
                        }
                    }),
            )
    }
}
