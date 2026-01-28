use adabraka_ui::{
    components::number_input::{NumberInput, NumberInputSize, NumberInputState},
    layout::VStack,
    theme::{install_theme, use_theme, Theme},
};
use gpui::{prelude::FluentBuilder as _, *};

struct NumberInputDemo {
    basic: Entity<NumberInputState>,
    with_range: Entity<NumberInputState>,
    decimal: Entity<NumberInputState>,
    quantity: Entity<NumberInputState>,
    last_change: Option<f64>,
}

impl NumberInputDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        let basic = cx.new(|cx| NumberInputState::new(cx));
        let with_range = cx.new(|cx| {
            let mut state = NumberInputState::with_value(cx, 5.0);
            state.set_min(Some(0.0), cx);
            state.set_max(Some(10.0), cx);
            state
        });
        let decimal = cx.new(|cx| {
            let mut state = NumberInputState::with_value(cx, 0.5);
            state.set_step(0.1);
            state.set_precision(1);
            state.set_min(Some(0.0), cx);
            state.set_max(Some(1.0), cx);
            state
        });
        let quantity = cx.new(|cx| {
            let mut state = NumberInputState::with_value(cx, 1.0);
            state.set_min(Some(1.0), cx);
            state.set_max(Some(99.0), cx);
            state
        });
        Self {
            basic,
            with_range,
            decimal,
            quantity,
            last_change: None,
        }
    }
}

impl Render for NumberInputDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();
        let entity = cx.entity().clone();

        div()
            .size_full()
            .bg(theme.tokens.background)
            .text_color(theme.tokens.foreground)
            .font_family(theme.tokens.font_family.clone())
            .child(
                VStack::new()
                    .p(px(24.0))
                    .gap(px(24.0))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .text_size(px(32.0))
                                    .font_weight(FontWeight::BOLD)
                                    .child("Number Input Demo"),
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Numeric input with increment/decrement buttons"),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(20.0))
                            .child({
                                let entity = entity.clone();
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(14.0))
                                            .font_weight(FontWeight::MEDIUM)
                                            .child("Basic"),
                                    )
                                    .child(NumberInput::new(self.basic.clone()).on_change(
                                        move |value, _, cx| {
                                            entity.update(cx, |this, cx| {
                                                this.last_change = Some(value);
                                                cx.notify();
                                            });
                                        },
                                    ))
                            })
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(14.0))
                                            .font_weight(FontWeight::MEDIUM)
                                            .child("With Range (0-10)"),
                                    )
                                    .child(NumberInput::new(self.with_range.clone()))
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child(format!(
                                                "Value: {}",
                                                self.with_range.read(cx).value() as i64
                                            )),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(14.0))
                                            .font_weight(FontWeight::MEDIUM)
                                            .child("Decimal (step 0.1)"),
                                    )
                                    .child(NumberInput::new(self.decimal.clone()))
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child(format!(
                                                "Value: {:.1}",
                                                self.decimal.read(cx).value()
                                            )),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(14.0))
                                            .font_weight(FontWeight::MEDIUM)
                                            .child("Sizes"),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap(px(16.0))
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .gap(px(4.0))
                                                    .child(
                                                        NumberInput::new(self.quantity.clone())
                                                            .size(NumberInputSize::Sm),
                                                    )
                                                    .child(
                                                        div()
                                                            .text_size(px(11.0))
                                                            .text_color(
                                                                theme.tokens.muted_foreground,
                                                            )
                                                            .child("Small"),
                                                    ),
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .gap(px(4.0))
                                                    .child(
                                                        NumberInput::new(self.quantity.clone())
                                                            .size(NumberInputSize::Md),
                                                    )
                                                    .child(
                                                        div()
                                                            .text_size(px(11.0))
                                                            .text_color(
                                                                theme.tokens.muted_foreground,
                                                            )
                                                            .child("Medium"),
                                                    ),
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .gap(px(4.0))
                                                    .child(
                                                        NumberInput::new(self.quantity.clone())
                                                            .size(NumberInputSize::Lg),
                                                    )
                                                    .child(
                                                        div()
                                                            .text_size(px(11.0))
                                                            .text_color(
                                                                theme.tokens.muted_foreground,
                                                            )
                                                            .child("Large"),
                                                    ),
                                            ),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(14.0))
                                            .font_weight(FontWeight::MEDIUM)
                                            .child("Disabled"),
                                    )
                                    .child(NumberInput::new(self.basic.clone()).disabled(true)),
                            ),
                    )
                    .when_some(self.last_change, |d, value| {
                        d.child(
                            div()
                                .mt(px(8.0))
                                .p(px(12.0))
                                .bg(theme.tokens.muted)
                                .rounded(theme.tokens.radius_md)
                                .text_size(px(14.0))
                                .child(format!("Last change callback: {}", value)),
                        )
                    }),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        adabraka_ui::init(cx);
        install_theme(cx, Theme::dark());

        let bounds = Bounds::centered(None, size(px(500.0), px(600.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("Number Input Demo".into()),
                    appears_transparent: false,
                    traffic_light_position: None,
                }),
                ..Default::default()
            },
            |_, cx| cx.new(|cx| NumberInputDemo::new(cx)),
        )
        .unwrap();

        cx.activate(true);
    });
}
