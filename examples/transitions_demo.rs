use gpui::{prelude::FluentBuilder as _, *};
use adabraka_ui::{
    transitions::Transition,
    components::button::{Button, ButtonVariant},
    theme::use_theme,
};

actions!(transitions_demo, [Quit]);

fn main() {
    Application::new().run(|cx| {
        // Initialize adabraka-ui
        adabraka_ui::init(cx);

        // Set up actions
        cx.on_action(|_: &Quit, cx| cx.quit());
        cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);

        cx.open_window(
            WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some("Transitions Demo".into()),
                    ..Default::default()
                }),
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(1000.0), px(900.0)),
                    cx,
                ))),
                ..Default::default()
            },
            |window, cx| cx.new(|cx| TransitionsDemo::new(window, cx)),
        )
        .unwrap();
    });
}

struct TransitionsDemo {
    show_fade: bool,
    show_slide_up: bool,
    show_slide_down: bool,
    show_slide_left: bool,
    show_slide_right: bool,
    show_scale: bool,
    show_all: bool,
}

impl TransitionsDemo {
    fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {
            show_fade: false,
            show_slide_up: false,
            show_slide_down: false,
            show_slide_left: false,
            show_slide_right: false,
            show_scale: false,
            show_all: false,
        }
    }

    fn toggle_all(&mut self, cx: &mut Context<Self>) {
        self.show_all = !self.show_all;
        cx.notify();
    }
}

impl Render for TransitionsDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(theme.tokens.background)
            .text_color(theme.tokens.foreground)
            .p(px(32.0))
            .gap(px(32.0))
            // Header
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .child(
                        div()
                            .text_size(px(32.0))
                            .font_weight(FontWeight::BOLD)
                            .child("Transitions Demo")
                    )
                    .child(
                        div()
                            .text_size(px(16.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("Smooth animations for appearing components")
                    )
            )
            // Show All Button
            .child(
                Button::new("toggle-all-btn", "Toggle All Transitions")
                    .variant(ButtonVariant::Secondary)
                    .on_click(_cx.listener(|this, _, _, cx| {
                        this.toggle_all(cx);
                    }))
            )
            // Transition Examples Grid
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(24.0))
                    // Fade Transition
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(12.0))
                            .child(
                                div()
                                    .text_size(px(18.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("Fade In")
                            )
                            .child(
                                div()
                                    .flex()
                                    .gap(px(12.0))
                                    .items_start()
                                    .child(
                                        Button::new("toggle-fade-btn", "Toggle")
                                            .variant(ButtonVariant::Outline)
                                            .on_click(_cx.listener(|this, _, _, cx| {
                                                this.show_fade = !this.show_fade;
                                                cx.notify();
                                            }))
                                    )
                                    .when(self.show_fade || self.show_all, |parent| {
                                        parent.child(
                                            Transition::fade_normal()
                                                .id("fade-demo")
                                                .child(
                                                    div()
                                                        .px(px(16.0))
                                                        .py(px(12.0))
                                                        .bg(theme.tokens.primary)
                                                        .text_color(theme.tokens.primary_foreground)
                                                        .rounded(theme.tokens.radius_md)
                                                        .child("Faded in smoothly!")
                                                )
                                        )
                                    })
                            )
                    )
                    // Slide Up Transition
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(12.0))
                            .child(
                                div()
                                    .text_size(px(18.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("Slide Up with Fade")
                            )
                            .child(
                                div()
                                    .flex()
                                    .gap(px(12.0))
                                    .items_start()
                                    .child(
                                        Button::new("toggle-slide-up-btn", "Toggle")
                                            .variant(ButtonVariant::Outline)
                                            .on_click(_cx.listener(|this, _, _, cx| {
                                                this.show_slide_up = !this.show_slide_up;
                                                cx.notify();
                                            }))
                                    )
                                    .when(self.show_slide_up || self.show_all, |parent| {
                                        parent.child(
                                            Transition::slide_up()
                                                .id("slide-up-demo")
                                                .child(
                                                    div()
                                                        .px(px(16.0))
                                                        .py(px(12.0))
                                                        .bg(theme.tokens.accent)
                                                        .text_color(theme.tokens.accent_foreground)
                                                        .rounded(theme.tokens.radius_md)
                                                        .child("Slid up from bottom!")
                                                )
                                        )
                                    })
                            )
                    )
                    // Slide Down Transition
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(12.0))
                            .child(
                                div()
                                    .text_size(px(18.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("Slide Down with Fade")
                            )
                            .child(
                                div()
                                    .flex()
                                    .gap(px(12.0))
                                    .items_start()
                                    .child(
                                        Button::new("toggle-slide-down-btn", "Toggle")
                                            .variant(ButtonVariant::Outline)
                                            .on_click(_cx.listener(|this, _, _, cx| {
                                                this.show_slide_down = !this.show_slide_down;
                                                cx.notify();
                                            }))
                                    )
                                    .when(self.show_slide_down || self.show_all, |parent| {
                                        parent.child(
                                            Transition::slide_down()
                                                .id("slide-down-demo")
                                                .child(
                                                    div()
                                                        .px(px(16.0))
                                                        .py(px(12.0))
                                                        .bg(theme.tokens.secondary)
                                                        .text_color(theme.tokens.secondary_foreground)
                                                        .rounded(theme.tokens.radius_md)
                                                        .child("Slid down from top!")
                                                )
                                        )
                                    })
                            )
                    )
                    // Slide Left Transition
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(12.0))
                            .child(
                                div()
                                    .text_size(px(18.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("Slide from Left")
                            )
                            .child(
                                div()
                                    .flex()
                                    .gap(px(12.0))
                                    .items_start()
                                    .child(
                                        Button::new("toggle-slide-left-btn", "Toggle")
                                            .variant(ButtonVariant::Outline)
                                            .on_click(_cx.listener(|this, _, _, cx| {
                                                this.show_slide_left = !this.show_slide_left;
                                                cx.notify();
                                            }))
                                    )
                                    .when(self.show_slide_left || self.show_all, |parent| {
                                        parent.child(
                                            Transition::slide_left()
                                                .id("slide-left-demo")
                                                .child(
                                                    div()
                                                        .px(px(16.0))
                                                        .py(px(12.0))
                                                        .bg(theme.tokens.primary)
                                                        .text_color(theme.tokens.primary_foreground)
                                                        .rounded(theme.tokens.radius_md)
                                                        .child("Slid in from left!")
                                                )
                                        )
                                    })
                            )
                    )
                    // Slide Right Transition
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(12.0))
                            .child(
                                div()
                                    .text_size(px(18.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("Slide from Right")
                            )
                            .child(
                                div()
                                    .flex()
                                    .gap(px(12.0))
                                    .items_start()
                                    .child(
                                        Button::new("toggle-slide-right-btn", "Toggle")
                                            .variant(ButtonVariant::Outline)
                                            .on_click(_cx.listener(|this, _, _, cx| {
                                                this.show_slide_right = !this.show_slide_right;
                                                cx.notify();
                                            }))
                                    )
                                    .when(self.show_slide_right || self.show_all, |parent| {
                                        parent.child(
                                            Transition::slide_right()
                                                .id("slide-right-demo")
                                                .child(
                                                    div()
                                                        .px(px(16.0))
                                                        .py(px(12.0))
                                                        .bg(theme.tokens.accent)
                                                        .text_color(theme.tokens.accent_foreground)
                                                        .rounded(theme.tokens.radius_md)
                                                        .child("Slid in from right!")
                                                )
                                        )
                                    })
                            )
                    )
                    // Scale Transition
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(12.0))
                            .child(
                                div()
                                    .text_size(px(18.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("Scale In (Smooth)")
                            )
                            .child(
                                div()
                                    .flex()
                                    .gap(px(12.0))
                                    .items_start()
                                    .child(
                                        Button::new("toggle-scale-btn", "Toggle")
                                            .variant(ButtonVariant::Outline)
                                            .on_click(_cx.listener(|this, _, _, cx| {
                                                this.show_scale = !this.show_scale;
                                                cx.notify();
                                            }))
                                    )
                                    .when(self.show_scale || self.show_all, |parent| {
                                        parent.child(
                                            Transition::scale_smooth()
                                                .id("scale-demo")
                                                .child(
                                                    div()
                                                        .px(px(16.0))
                                                        .py(px(12.0))
                                                        .bg(theme.tokens.destructive)
                                                        .text_color(theme.tokens.destructive_foreground)
                                                        .rounded(theme.tokens.radius_md)
                                                        .child("Scaled in smoothly!")
                                                )
                                        )
                                    })
                            )
                    )
                    // Custom Transition
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(12.0))
                            .child(
                                div()
                                    .text_size(px(18.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("Custom Transition (Scale + Bounce)")
                            )
                            .child(
                                div()
                                    .flex()
                                    .gap(px(12.0))
                                    .items_start()
                                    .when(self.show_all, |parent| {
                                        parent.child(
                                            Transition::scale_bounce()
                                                .id("custom-demo")
                                                .child(
                                                    div()
                                                        .px(px(20.0))
                                                        .py(px(16.0))
                                                        .bg(theme.tokens.primary)
                                                        .text_color(theme.tokens.primary_foreground)
                                                        .rounded(theme.tokens.radius_lg)
                                                        .shadow_md()
                                                        .child(
                                                            div()
                                                                .flex()
                                                                .flex_col()
                                                                .gap(px(4.0))
                                                                .child(
                                                                    div()
                                                                        .text_size(px(16.0))
                                                                        .font_weight(FontWeight::SEMIBOLD)
                                                                        .child("Spring Animation!")
                                                                )
                                                                .child(
                                                                    div()
                                                                        .text_size(px(14.0))
                                                                        .child("Scales in with a subtle bounce effect")
                                                                )
                                                        )
                                                )
                                        )
                                    })
                            )
                    )
            )
    }
}
