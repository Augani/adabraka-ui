//! Text field component - Simple text input field.

use gpui::{prelude::FluentBuilder as _, *};
use crate::theme::use_theme;
use std::ops::Range;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TextFieldSize { Sm, Md, Lg }

pub struct TextFieldState {
    focus_handle: FocusHandle,
    text: String,
    cursor_position: usize,
    marked_range: Option<Range<usize>>,
}

impl TextFieldState {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            text: String::new(),
            cursor_position: 0,
            marked_range: None,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, text: String, cx: &mut Context<Self>) {
        self.text = text;
        self.cursor_position = self.text.len();
        cx.notify();
    }

    fn insert_text(&mut self, text: &str, cx: &mut Context<Self>) {
        self.text.insert_str(self.cursor_position, text);
        self.cursor_position += text.len();
        cx.notify();
    }
}

impl Focusable for TextFieldState {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EntityInputHandler for TextFieldState {
    fn text_for_range(
        &mut self,
        range_utf16: Range<usize>,
        actual_range: &mut Option<Range<usize>>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<String> {
        actual_range.replace(range_utf16.clone());
        let start = range_utf16.start.min(self.text.len());
        let end = range_utf16.end.min(self.text.len());
        Some(self.text[start..end].to_string())
    }

    fn selected_text_range(
        &mut self,
        _ignore_disabled_input: bool,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<UTF16Selection> {
        Some(UTF16Selection {
            range: self.cursor_position..self.cursor_position,
            reversed: false,
        })
    }

    fn marked_text_range(
        &self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Range<usize>> {
        self.marked_range.clone()
    }

    fn unmark_text(&mut self, _window: &mut Window, _cx: &mut Context<Self>) {
        self.marked_range = None;
    }

    fn replace_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(range) = range_utf16 {
            let start = range.start.min(self.text.len());
            let end = range.end.min(self.text.len());
            self.text.replace_range(start..end, new_text);
            self.cursor_position = start + new_text.len();
        } else {
            self.insert_text(new_text, cx);
        }
        cx.notify();
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        new_selected_range_utf16: Option<Range<usize>>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(range) = range_utf16 {
            let start = range.start.min(self.text.len());
            let end = range.end.min(self.text.len());
            self.text.replace_range(start..end, new_text);
            self.cursor_position = start + new_text.len();

            if !new_text.is_empty() {
                self.marked_range = Some(start..start + new_text.len());
            }
        }

        if let Some(sel_range) = new_selected_range_utf16 {
            self.cursor_position = sel_range.end.min(self.text.len());
        }

        cx.notify();
    }

    fn bounds_for_range(
        &mut self,
        _range_utf16: Range<usize>,
        _bounds: Bounds<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Bounds<Pixels>> {
        None
    }

    fn character_index_for_point(
        &mut self,
        _point: Point<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<usize> {
        None
    }
}

impl Render for TextFieldState {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}

#[derive(IntoElement)]
pub struct TextField {
    state: Entity<TextFieldState>,
    size: TextFieldSize,
    placeholder: Option<SharedString>,
    disabled: bool,
    invalid: bool,
    style: StyleRefinement,
}

impl TextField {
    pub fn new(cx: &mut App) -> Self {
        let state = cx.new(|cx| TextFieldState::new(cx));
        Self {
            state,
            size: TextFieldSize::Md,
            placeholder: None,
            disabled: false,
            invalid: false,
            style: StyleRefinement::default(),
        }
    }

    pub fn size(mut self, size: TextFieldSize) -> Self {
        self.size = size;
        self
    }

    pub fn placeholder<T: Into<SharedString>>(mut self, placeholder: T) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    pub fn value<T: Into<String>>(self, value: T, cx: &mut App) -> Self {
        self.state.update(cx, |state, cx| {
            state.set_text(value.into(), cx);
        });
        self
    }

    pub fn text(&self, cx: &App) -> String {
        self.state.read(cx).text().to_string()
    }

    pub fn on_change<F: Fn(&str, &mut Window, &mut App) + 'static>(self, _f: F) -> Self {
        self
    }
}

impl Styled for TextField {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for TextField {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let user_style = self.style;

        let (height, padding_x, padding_y, text_size) = match self.size {
            TextFieldSize::Sm => (px(36.0), px(12.0), px(8.0), px(14.0)),
            TextFieldSize::Md => (px(40.0), px(12.0), px(8.0), px(14.0)),
            TextFieldSize::Lg => (px(44.0), px(12.0), px(10.0), px(16.0)),
        };

        let border_color = if self.invalid {
            theme.tokens.destructive
        } else {
            theme.tokens.input
        };

        let focus_handle = self.state.read(cx).focus_handle.clone();
        let focus_handle_for_mouse = focus_handle.clone();
        let text_content = self.state.read(cx).text().to_string();

        let mut base = div()
            .id(("text-field", self.state.entity_id()))
            .track_focus(&focus_handle)
            .h(height)
            .px(padding_x)
            .py(padding_y)
            .bg(theme.tokens.background)
            .border_1()
            .border_color(border_color)
            .rounded(theme.tokens.radius_md);

        if self.disabled {
            base = base.opacity(0.5);
        }

        base
            .map(|this| {
                let mut div = this;
                div.style().refine(&user_style);
                div
            })
            .child(
                div()
                    .size_full()
                    .flex()
                    .items_center()
                    .child(
                        canvas(
                            move |bounds, window, cx| {
                                let focus_handle = focus_handle.clone();
                                window.handle_input(
                                    &focus_handle,
                                    ElementInputHandler::new(bounds, self.state.clone()),
                                    cx,
                                );
                            },
                            move |bounds, _data, window, cx| {
                                if !text_content.is_empty() {
                                    let text_style = window.text_style();
                                    let text_run = TextRun {
                                        len: text_content.len(),
                                        font: text_style.font(),
                                        color: theme.tokens.foreground,
                                        background_color: None,
                                        underline: None,
                                        strikethrough: None,
                                    };

                                    let shaped = window.text_system().shape_line(
                                        text_content.clone().into(),
                                        text_size,
                                        &[text_run],
                                        None,
                                    );

                                    let _ = shaped.paint(
                                        point(bounds.left(), bounds.top()),
                                        height,
                                        window,
                                        cx,
                                    );
                                } else if let Some(placeholder) = &self.placeholder {
                                    let text_style = window.text_style();
                                    let text_run = TextRun {
                                        len: placeholder.len(),
                                        font: text_style.font(),
                                        color: theme.tokens.muted_foreground,
                                        background_color: None,
                                        underline: None,
                                        strikethrough: None,
                                    };

                                    let shaped = window.text_system().shape_line(
                                        placeholder.clone().into(),
                                        text_size,
                                        &[text_run],
                                        None,
                                    );

                                    let _ = shaped.paint(
                                        point(bounds.left(), bounds.top()),
                                        height,
                                        window,
                                        cx,
                                    );
                                }
                            },
                        )
                        .size_full(),
                    ),
            )
            .on_mouse_down(MouseButton::Left, move |_event, window, _cx| {
                window.focus(&focus_handle_for_mouse);
            })
    }
}
