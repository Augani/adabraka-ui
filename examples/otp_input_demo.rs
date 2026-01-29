use adabraka_ui::prelude::*;
use gpui::*;

struct OTPInputDemoApp {
    otp_state1: Entity<OTPState>,
    otp_state2: Entity<OTPState>,
    otp_state3: Entity<OTPState>,
}

impl OTPInputDemoApp {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            otp_state1: cx.new(|cx| OTPState::new(cx, 6)),
            otp_state2: cx.new(|cx| OTPState::new(cx, 4)),
            otp_state3: cx.new(|cx| OTPState::new(cx, 6)),
        }
    }
}

impl Render for OTPInputDemoApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        let value1 = self.otp_state1.read(cx).value();
        let value2 = self.otp_state2.read(cx).value();

        div()
            .size_full()
            .bg(theme.tokens.background)
            .p(px(40.0))
            .flex()
            .flex_col()
            .gap(px(32.0))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .child(h2("OTP Input Component"))
                    .child(muted(
                        "One-time password input with auto-focus and paste support",
                    )),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(24.0))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(8.0))
                            .child(label("6-Digit OTP (Default)"))
                            .child(OTPInput::new(&self.otp_state1))
                            .child(muted(format!("Value: {}", value1))),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(8.0))
                            .child(label("4-Digit OTP - Small Size"))
                            .child(OTPInput::new(&self.otp_state2).size(OTPInputSize::Sm))
                            .child(muted(format!("Value: {}", value2))),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(8.0))
                            .child(label("Large Size - Masked"))
                            .child(
                                OTPInput::new(&self.otp_state3)
                                    .size(OTPInputSize::Lg)
                                    .masked(true),
                            ),
                    ),
            )
    }
}

fn main() {
    Application::new().run(|cx| {
        adabraka_ui::init(cx);
        install_theme(cx, Theme::dark());

        let bounds = Bounds::centered(None, size(px(500.0), px(500.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(OTPInputDemoApp::new),
        )
        .unwrap();
    });
}
