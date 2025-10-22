use adabraka_ui::{
    components::{
        input::{Input, InputType, InputVariant},
        input_state::{InputState, InputEvent},
        scrollable::scrollable_vertical,
    },
    layout::{VStack, HStack},
    theme::{install_theme, Theme},
};
use gpui::*;

struct FocusTestApp {
    // Form inputs to test tab navigation
    first_name: Entity<InputState>,
    last_name: Entity<InputState>,
    email: Entity<InputState>,
    phone: Entity<InputState>,
    password: Entity<InputState>,

    // Track which input has focus
    current_focus_index: usize,
}

impl FocusTestApp {
    fn new(cx: &mut Context<Self>) -> Self {
        let mut app = Self {
            first_name: cx.new(|cx| InputState::new(cx)),
            last_name: cx.new(|cx| InputState::new(cx)),
            email: cx.new(|cx| InputState::new(cx).input_type(InputType::Email)),
            phone: cx.new(|cx| InputState::new(cx).input_type(InputType::Tel)),
            password: cx.new(|cx| InputState::new(cx).input_type(InputType::Password)),
            current_focus_index: 0,
        };

        // Set up tab navigation handlers
        app.setup_tab_navigation(cx);
        app
    }

    fn setup_tab_navigation(&mut self, cx: &mut Context<Self>) {
        // Create a list of all inputs in order
        let inputs = vec![
            self.first_name.clone(),
            self.last_name.clone(),
            self.email.clone(),
            self.phone.clone(),
            self.password.clone(),
        ];

        // Set up tab navigation for each input
        for (i, input) in inputs.iter().enumerate() {
            let inputs_clone = inputs.clone();
            let current_index = i;

            cx.subscribe(input, move |_this, _emitter: Entity<InputState>, event: &InputEvent, cx| {
                match event {
                    InputEvent::Tab => {
                        // Queue focus change for next input
                        let next_index = (current_index + 1) % inputs_clone.len();
                        let next_input = inputs_clone[next_index].clone();
                        cx.defer(move |cx| {
                            cx.update_window(cx.active_window().unwrap(), |_, window, cx| {
                                window.focus(&next_input.read(cx).focus_handle(cx));
                            }).ok();
                        });
                    }
                    InputEvent::ShiftTab => {
                        // Queue focus change for previous input
                        let prev_index = if current_index == 0 {
                            inputs_clone.len() - 1
                        } else {
                            current_index - 1
                        };
                        let prev_input = inputs_clone[prev_index].clone();
                        cx.defer(move |cx| {
                            cx.update_window(cx.active_window().unwrap(), |_, window, cx| {
                                window.focus(&prev_input.read(cx).focus_handle(cx));
                            }).ok();
                        });
                    }
                    _ => {}
                }
            })
            .detach();
        }
    }
}

impl Render for FocusTestApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = adabraka_ui::theme::use_theme();

        div()
            .size_full()
            .bg(theme.tokens.background)
            .child(
                VStack::new()
                    .size_full()
                    // Header
                    .child(
                        VStack::new()
                            .w_full()
                            .p(px(24.0))
                            .bg(theme.tokens.card)
                            .border_b_1()
                            .border_color(theme.tokens.border)
                            .child(
                                div()
                                    .text_size(px(24.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(theme.tokens.foreground)
                                    .child("Focus Management Demo")
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Test tab navigation and click-outside-to-blur behavior")
                            )
                    )
                    // Content
                    .child(
                        div()
                            .flex_1()
                            .w_full()
                            .overflow_hidden()
                            .child(
                                scrollable_vertical(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .w_full()
                                        .p(px(32.0))
                                        .gap(px(24.0))
                                        // Instructions
                                        .child(
                                            VStack::new()
                                                .w_full()
                                                .gap(px(12.0))
                                                .p(px(20.0))
                                                .bg(theme.tokens.primary.opacity(0.1))
                                                .border_1()
                                                .border_color(theme.tokens.primary)
                                                .rounded(theme.tokens.radius_lg)
                                                .child(
                                                    div()
                                                        .text_size(px(16.0))
                                                        .font_weight(FontWeight::SEMIBOLD)
                                                        .text_color(theme.tokens.foreground)
                                                        .child("📋 Instructions")
                                                )
                                                .child(
                                                    div()
                                                        .text_size(px(14.0))
                                                        .text_color(theme.tokens.foreground)
                                                        .line_height(relative(1.6))
                                                        .child("• Click on an input to focus it\n• Double-click to select all text\n• Press Tab to move to next input, Shift+Tab for previous\n• Press Escape to blur the focused input\n• Use all standard keyboard shortcuts (arrows, Home/End, Cmd/Ctrl+A/C/V/X)")
                                                )
                                        )
                                        // Form container
                                        .child(
                                            VStack::new()
                                                .w_full()
                                                .max_w(px(500.0))
                                                .gap(px(20.0))
                                                .p(px(24.0))
                                                .bg(theme.tokens.card)
                                                .border_1()
                                                .border_color(theme.tokens.border)
                                                .rounded(theme.tokens.radius_lg)
                                                .child(
                                                    div()
                                                        .text_size(px(18.0))
                                                        .font_weight(FontWeight::SEMIBOLD)
                                                        .text_color(theme.tokens.foreground)
                                                        .child("Sample Registration Form")
                                                )
                                                // First Name
                                                .child(
                                                    VStack::new()
                                                        .w_full()
                                                        .gap(px(4.0))
                                                        .child(
                                                            div()
                                                                .text_size(px(14.0))
                                                                .font_weight(FontWeight::MEDIUM)
                                                                .text_color(theme.tokens.foreground)
                                                                .child("First Name")
                                                        )
                                                        .child(
                                                            Input::new(&self.first_name)
                                                                .placeholder("John")
                                                                .variant(InputVariant::Outline)
                                                        )
                                                )
                                                // Last Name
                                                .child(
                                                    VStack::new()
                                                        .w_full()
                                                        .gap(px(4.0))
                                                        .child(
                                                            div()
                                                                .text_size(px(14.0))
                                                                .font_weight(FontWeight::MEDIUM)
                                                                .text_color(theme.tokens.foreground)
                                                                .child("Last Name")
                                                        )
                                                        .child(
                                                            Input::new(&self.last_name)
                                                                .placeholder("Doe")
                                                                .variant(InputVariant::Outline)
                                                        )
                                                )
                                                // Email
                                                .child(
                                                    VStack::new()
                                                        .w_full()
                                                        .gap(px(4.0))
                                                        .child(
                                                            div()
                                                                .text_size(px(14.0))
                                                                .font_weight(FontWeight::MEDIUM)
                                                                .text_color(theme.tokens.foreground)
                                                                .child("Email Address")
                                                        )
                                                        .child(
                                                            Input::new(&self.email)
                                                                .input_type(InputType::Email)
                                                                .placeholder("john.doe@example.com")
                                                                .variant(InputVariant::Outline)
                                                        )
                                                )
                                                // Phone
                                                .child(
                                                    VStack::new()
                                                        .w_full()
                                                        .gap(px(4.0))
                                                        .child(
                                                            div()
                                                                .text_size(px(14.0))
                                                                .font_weight(FontWeight::MEDIUM)
                                                                .text_color(theme.tokens.foreground)
                                                                .child("Phone Number")
                                                        )
                                                        .child(
                                                            Input::new(&self.phone)
                                                                .input_type(InputType::Tel)
                                                                .placeholder("(555) 123-4567")
                                                                .variant(InputVariant::Outline)
                                                        )
                                                )
                                                // Password
                                                .child(
                                                    VStack::new()
                                                        .w_full()
                                                        .gap(px(4.0))
                                                        .child(
                                                            div()
                                                                .text_size(px(14.0))
                                                                .font_weight(FontWeight::MEDIUM)
                                                                .text_color(theme.tokens.foreground)
                                                                .child("Password")
                                                        )
                                                        .child(
                                                            Input::new(&self.password)
                                                                .input_type(InputType::Password)
                                                                .placeholder("Enter password")
                                                                .variant(InputVariant::Outline)
                                                        )
                                                )
                                        )
                                        // Status indicators
                                        .child(
                                            VStack::new()
                                                .w_full()
                                                .max_w(px(500.0))
                                                .gap(px(12.0))
                                                .p(px(16.0))
                                                .bg(theme.tokens.muted.opacity(0.3))
                                                .border_1()
                                                .border_color(theme.tokens.border)
                                                .rounded(theme.tokens.radius_lg)
                                                .child(
                                                    div()
                                                        .text_size(px(14.0))
                                                        .font_weight(FontWeight::SEMIBOLD)
                                                        .text_color(theme.tokens.foreground)
                                                        .child("Focus Status")
                                                )
                                                .child(
                                                    VStack::new()
                                                        .gap(px(6.0))
                                                        .child(
                                                            HStack::new()
                                                                .gap(px(8.0))
                                                                .child(
                                                                    div()
                                                                        .w(px(8.0))
                                                                        .h(px(8.0))
                                                                        .rounded(px(4.0))
                                                                        .bg(if self.first_name.read(cx).focus_handle(cx).is_focused(window) {
                                                                            theme.tokens.primary
                                                                        } else {
                                                                            theme.tokens.muted
                                                                        })
                                                                )
                                                                .child(
                                                                    div()
                                                                        .text_size(px(13.0))
                                                                        .text_color(theme.tokens.muted_foreground)
                                                                        .child("First Name")
                                                                )
                                                        )
                                                        .child(
                                                            HStack::new()
                                                                .gap(px(8.0))
                                                                .child(
                                                                    div()
                                                                        .w(px(8.0))
                                                                        .h(px(8.0))
                                                                        .rounded(px(4.0))
                                                                        .bg(if self.last_name.read(cx).focus_handle(cx).is_focused(window) {
                                                                            theme.tokens.primary
                                                                        } else {
                                                                            theme.tokens.muted
                                                                        })
                                                                )
                                                                .child(
                                                                    div()
                                                                        .text_size(px(13.0))
                                                                        .text_color(theme.tokens.muted_foreground)
                                                                        .child("Last Name")
                                                                )
                                                        )
                                                        .child(
                                                            HStack::new()
                                                                .gap(px(8.0))
                                                                .child(
                                                                    div()
                                                                        .w(px(8.0))
                                                                        .h(px(8.0))
                                                                        .rounded(px(4.0))
                                                                        .bg(if self.email.read(cx).focus_handle(cx).is_focused(window) {
                                                                            theme.tokens.primary
                                                                        } else {
                                                                            theme.tokens.muted
                                                                        })
                                                                )
                                                                .child(
                                                                    div()
                                                                        .text_size(px(13.0))
                                                                        .text_color(theme.tokens.muted_foreground)
                                                                        .child("Email")
                                                                )
                                                        )
                                                        .child(
                                                            HStack::new()
                                                                .gap(px(8.0))
                                                                .child(
                                                                    div()
                                                                        .w(px(8.0))
                                                                        .h(px(8.0))
                                                                        .rounded(px(4.0))
                                                                        .bg(if self.phone.read(cx).focus_handle(cx).is_focused(window) {
                                                                            theme.tokens.primary
                                                                        } else {
                                                                            theme.tokens.muted
                                                                        })
                                                                )
                                                                .child(
                                                                    div()
                                                                        .text_size(px(13.0))
                                                                        .text_color(theme.tokens.muted_foreground)
                                                                        .child("Phone")
                                                                )
                                                        )
                                                        .child(
                                                            HStack::new()
                                                                .gap(px(8.0))
                                                                .child(
                                                                    div()
                                                                        .w(px(8.0))
                                                                        .h(px(8.0))
                                                                        .rounded(px(4.0))
                                                                        .bg(if self.password.read(cx).focus_handle(cx).is_focused(window) {
                                                                            theme.tokens.primary
                                                                        } else {
                                                                            theme.tokens.muted
                                                                        })
                                                                )
                                                                .child(
                                                                    div()
                                                                        .text_size(px(13.0))
                                                                        .text_color(theme.tokens.muted_foreground)
                                                                        .child("Password")
                                                                )
                                                        )
                                                )
                                        )
                                ),
                            ),
                    ),
            )
    }
}

fn main() {
    Application::new().run(move |cx| {
        // Install dark theme
        install_theme(cx, Theme::dark());

        // Initialize input system
        adabraka_ui::init(cx);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(800.0), px(700.0)),
                    cx,
                ))),
                titlebar: Some(TitlebarOptions {
                    title: Some("Input Focus Management Demo".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_window, cx| cx.new(|cx| FocusTestApp::new(cx)),
        )
        .unwrap();
    });
}