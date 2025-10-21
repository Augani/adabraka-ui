use adabraka_ui::components::input::{self, Input, InputState, InputVariant, InputSize};
use gpui::{prelude::*, *};

struct InputTestApp {
    input_state: Entity<InputState>,
    password_input_state: Entity<InputState>,
    output_text: SharedString,
}

impl InputTestApp {
    fn new(cx: &mut Context<Self>) -> Self {
        let input_state = cx.new(|cx| InputState::new(cx));
        let password_input_state = cx.new(|cx| InputState::new(cx));

        Self {
            input_state,
            password_input_state,
            output_text: "No input yet".into(),
        }
    }
}

impl Render for InputTestApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_4()
            .p_8()
            .bg(rgb(0x1e1e2e))
            .size_full()
            .child(
                div()
                    .text_xl()
                    .text_color(rgb(0xcdd6f4))
                    .child("Input Component Test")
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0xa6adc8))
                            .child("Basic Input with Clear Button")
                    )
                    .child(
                        Input::new(&self.input_state)
                            .placeholder("Enter your name...")
                            .clearable(true)
                            .on_change({
                                let entity = cx.entity();
                                move |value, cx| {
                                    entity.update(cx, |this, cx| {
                                        this.output_text = format!("You typed: {}", value).into();
                                        cx.notify();
                                    });
                                }
                            })
                            .on_enter({
                                let entity = cx.entity();
                                move |value, cx| {
                                    entity.update(cx, |this, cx| {
                                        this.output_text = format!("Enter pressed with: {}", value).into();
                                        cx.notify();
                                    });
                                }
                            })
                    )
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0xa6adc8))
                            .child("Password Input with Toggle")
                    )
                    .child(
                        Input::new(&self.password_input_state)
                            .placeholder("Enter password...")
                            .password(true)
                            .variant(InputVariant::Outline)
                            .size(InputSize::Lg)
                    )
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0xa6adc8))
                            .child("Output")
                    )
                    .child(
                        div()
                            .p_4()
                            .bg(rgb(0x313244))
                            .rounded_md()
                            .text_color(rgb(0xf5e0dc))
                            .child(self.output_text.clone())
                    )
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        // Initialize input key bindings
        input::init(cx);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: point(px(100.), px(100.)),
                    size: size(px(800.), px(600.)),
                })),
                ..Default::default()
            },
            |window, cx| {
                cx.new(|cx| InputTestApp::new(cx))
            },
        )
        .unwrap();
    });
}