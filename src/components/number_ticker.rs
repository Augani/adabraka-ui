use gpui::{prelude::FluentBuilder as _, *};
use std::time::Duration;

use crate::animations::easings;
use crate::fonts::mono_font_family;
use crate::theme::use_theme;

#[derive(IntoElement)]
pub struct NumberTicker {
    _id: ElementId,
    base: Div,
    value: i64,
    separator: Option<char>,
    prefix: Option<SharedString>,
    suffix: Option<SharedString>,
    duration: Duration,
}

impl NumberTicker {
    pub fn new(id: impl Into<ElementId>, value: i64) -> Self {
        Self {
            _id: id.into(),
            base: div(),
            value,
            separator: None,
            prefix: None,
            suffix: None,
            duration: Duration::from_millis(600),
        }
    }

    pub fn separator(mut self, sep: char) -> Self {
        self.separator = Some(sep);
        self
    }

    pub fn prefix(mut self, prefix: impl Into<SharedString>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }

    pub fn suffix(mut self, suffix: impl Into<SharedString>) -> Self {
        self.suffix = Some(suffix.into());
        self
    }

    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }
}

impl Styled for NumberTicker {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

fn format_with_separator(value: i64, separator: Option<char>) -> Vec<DigitOrSeparator> {
    let is_negative = value < 0;
    let abs_str = value.unsigned_abs().to_string();
    let mut result = Vec::new();

    if is_negative {
        result.push(DigitOrSeparator::Separator('-'));
    }

    let digits: Vec<u8> = abs_str.bytes().map(|b| b - b'0').collect();
    let len = digits.len();

    for (i, &digit) in digits.iter().enumerate() {
        result.push(DigitOrSeparator::Digit(digit));
        if let Some(sep) = separator {
            let remaining = len - 1 - i;
            if remaining > 0 && remaining % 3 == 0 {
                result.push(DigitOrSeparator::Separator(sep));
            }
        }
    }

    result
}

#[derive(Clone, Copy)]
enum DigitOrSeparator {
    Digit(u8),
    Separator(char),
}

impl RenderOnce for NumberTicker {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let chars = format_with_separator(self.value, self.separator);
        let duration = self.duration;
        let digit_height = px(24.0);
        let column_height = digit_height * 10.0;

        self.base
            .flex()
            .flex_row()
            .items_center()
            .text_color(theme.tokens.foreground)
            .font_family(mono_font_family())
            .when_some(self.prefix.clone(), |el, prefix| {
                el.child(div().child(prefix))
            })
            .children(chars.iter().enumerate().map(move |(pos, item)| {
                match *item {
                    DigitOrSeparator::Separator(ch) => div()
                        .flex_shrink_0()
                        .child(SharedString::from(String::from(ch)))
                        .into_any_element(),
                    DigitOrSeparator::Digit(digit) => {
                        let target_offset = -(digit_height * digit as f32);

                        div()
                            .flex_shrink_0()
                            .h(digit_height)
                            .overflow_hidden()
                            .child(
                                div()
                                    .id(("digit-col", pos as u32))
                                    .flex()
                                    .flex_col()
                                    .h(column_height)
                                    .children((0..10u8).map(|d| {
                                        div()
                                            .h(digit_height)
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .child(SharedString::from(d.to_string()))
                                    }))
                                    .with_animation(
                                        ("digit-roll", pos as u32),
                                        Animation::new(duration)
                                            .with_easing(easings::ease_out_cubic),
                                        move |el, delta| {
                                            let offset = target_offset * delta;
                                            el.top(offset)
                                        },
                                    ),
                            )
                            .into_any_element()
                    }
                }
            }))
            .when_some(self.suffix, |el, suffix| el.child(div().child(suffix)))
    }
}
