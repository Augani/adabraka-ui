use gpui::*;
use adabraka_ui::{
    components::{
        input::{Input, InputVariant, InputSize},
        input_state::{InputState, *},
        button::{Button, ButtonVariant, ButtonSize},
        scrollable::scrollable_vertical,
    },
    layout::{VStack, HStack, Justify, Align},
    theme::{install_theme, Theme},
};

struct InputDemoApp {
    // Interactive inputs
    interactive_input1: Entity<InputState>,
    interactive_input2: Entity<InputState>,
    interactive_input3: Entity<InputState>,

    // Input Variants section
    variant_default: Entity<InputState>,
    variant_outline: Entity<InputState>,
    variant_ghost: Entity<InputState>,

    // Input Sizes section
    size_small: Entity<InputState>,
    size_medium: Entity<InputState>,
    size_large: Entity<InputState>,

    // Input States section
    state_normal: Entity<InputState>,
    state_disabled: Entity<InputState>,
    state_error: Entity<InputState>,
    state_clearable: Entity<InputState>,

    // Password section
    password_input: Entity<InputState>,

    // Prefix/Suffix section
    prefix_input: Entity<InputState>,
    suffix_input: Entity<InputState>,
    both_input: Entity<InputState>,

    // Form example
    form_name: Entity<InputState>,
    form_email: Entity<InputState>,
    form_password: Entity<InputState>,
    form_bio: Entity<InputState>, // We'll use a larger input for this
}

impl InputDemoApp {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            // Create interactive inputs
            interactive_input1: cx.new(|cx| InputState::new(cx).placeholder("Type anything here...")),
            interactive_input2: cx.new(|cx| InputState::new(cx).placeholder("Try copy & paste (Cmd/Ctrl+C/V)")),
            interactive_input3: cx.new(|cx| InputState::new(cx).placeholder("Use arrow keys, Home, End...")),

            // Input Variants
            variant_default: cx.new(|cx| InputState::new(cx)),
            variant_outline: cx.new(|cx| InputState::new(cx)),
            variant_ghost: cx.new(|cx| InputState::new(cx)),

            // Input Sizes
            size_small: cx.new(|cx| InputState::new(cx)),
            size_medium: cx.new(|cx| InputState::new(cx)),
            size_large: cx.new(|cx| InputState::new(cx)),

            // Input States
            state_normal: cx.new(|cx| {
                let mut state = InputState::new(cx);
                state.content = "Some text".into();
                state
            }),
            state_disabled: cx.new(|cx| InputState::new(cx)),
            state_error: cx.new(|cx| {
                let mut state = InputState::new(cx);
                state.content = "invalid@".into();
                state
            }),
            state_clearable: cx.new(|cx| {
                let mut state = InputState::new(cx);
                state.content = "Clear me!".into();
                state
            }),

            // Password
            password_input: cx.new(|cx| {
                let mut state = InputState::new(cx);
                state.content = "secret123".into();
                state
            }),

            // Prefix/Suffix
            prefix_input: cx.new(|cx| InputState::new(cx)),
            suffix_input: cx.new(|cx| {
                let mut state = InputState::new(cx);
                state.content = "100".into();
                state
            }),
            both_input: cx.new(|cx| InputState::new(cx)),

            // Form example
            form_name: cx.new(|cx| InputState::new(cx)),
            form_email: cx.new(|cx| InputState::new(cx)),
            form_password: cx.new(|cx| InputState::new(cx)),
            form_bio: cx.new(|cx| InputState::new(cx)),
        }
    }
}

impl Render for InputDemoApp {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let theme = adabraka_ui::theme::use_theme();

        VStack::new()
            .size_full()
            .bg(theme.tokens.background)
            // Header (fixed)
            .child(
                VStack::new()
                    .w_full()
                    .p(px(24.0))
                    .bg(theme.tokens.card)
                    .border_b_1()
                    .border_color(theme.tokens.border)
                    .child(
                        div()
                            .text_size(px(32.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(theme.tokens.foreground)
                            .child("Input Components Demo")
                    )
                    .child(
                        div()
                            .text_size(px(16.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("Showcase of Input components with various configurations")
                    )
            )
            // Scrollable content area
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
                                .p(px(40.0))
                                .gap(px(32.0))
                                // Interactive Inputs Section
                                .child(render_interactive_section(
                                    self.interactive_input1.clone(),
                                    self.interactive_input2.clone(),
                                    self.interactive_input3.clone(),
                                ))
                                // Note about interactivity
                                .child(
                                    div()
                                        .w_full()
                                        .p(px(16.0))
                                        .bg(theme.tokens.muted.opacity(0.3))
                                        .border_1()
                                        .border_color(theme.tokens.border)
                                        .rounded(theme.tokens.radius_md)
                                        .child(
                                            VStack::new()
                                                .gap(px(4.0))
                                                .child(
                                                    div()
                                                        .text_size(px(14.0))
                                                        .font_weight(FontWeight::SEMIBOLD)
                                                        .text_color(theme.tokens.foreground)
                                                        .child("✨ All Inputs Below Are Fully Interactive!")
                                                )
                                                .child(
                                                    div()
                                                        .text_size(px(13.0))
                                                        .text_color(theme.tokens.muted_foreground)
                                                        .child("Every input in this demo is editable using Entity<InputState>. Try typing, selecting, copying, and pasting!")
                                                )
                                        )
                                )
                                // Section 1: Input Variants
                                .child(render_section(
                                    "Input Variants",
                                    "Different visual styles for inputs",
                                    vec![
                                        ("Default", Input::new(&self.variant_default)
                                            .placeholder("Default variant")
                                            .variant(InputVariant::Default)),
                                        ("Outline", Input::new(&self.variant_outline)
                                            .placeholder("Outline variant")
                                            .variant(InputVariant::Outline)),
                                        ("Ghost", Input::new(&self.variant_ghost)
                                            .placeholder("Ghost variant")
                                            .variant(InputVariant::Ghost)),
                                    ],
                                ))
                                // Section 2: Input Sizes
                                .child(render_section(
                                    "Input Sizes",
                                    "Small, medium, and large input sizes",
                                    vec![
                                        ("Small", Input::new(&self.size_small)
                                            .placeholder("Small input")
                                            .size(InputSize::Sm)),
                                        ("Medium", Input::new(&self.size_medium)
                                            .placeholder("Medium input (default)")
                                            .size(InputSize::Md)),
                                        ("Large", Input::new(&self.size_large)
                                            .placeholder("Large input")
                                            .size(InputSize::Lg)),
                                    ],
                                ))
                                // Section 3: Input States
                                .child(render_section(
                                    "Input States",
                                    "Different input states and configurations",
                                    vec![
                                        ("Normal", Input::new(&self.state_normal)
                                            .placeholder("Normal state")),
                                        ("Disabled", Input::new(&self.state_disabled)
                                            .placeholder("Disabled state")
                                            .disabled(true)),
                                        ("Error", Input::new(&self.state_error)
                                            .placeholder("Error state")
                                            .error(true)),
                                        ("Clearable", Input::new(&self.state_clearable)
                                            .placeholder("With clear button")
                                            .clearable(true)),
                                    ],
                                ))
                                // Section 4: Password Input
                                .child(render_section(
                                    "Password Input",
                                    "Input with password masking and visibility toggle",
                                    vec![
                                        ("Password", Input::new(&self.password_input)
                                            .placeholder("Enter password")
                                            .password(true)),
                                    ],
                                ))
                                // Section 5: Input with Prefix/Suffix
                                .child(render_prefix_suffix_section(
                                    &self.prefix_input,
                                    &self.suffix_input,
                                    &self.both_input,
                                ))
                                // Section 6: Form Example
                                .child(render_form_example(
                                    &self.form_name,
                                    &self.form_email,
                                    &self.form_password,
                                    &self.form_bio,
                                ))
                        ),
                    ),
            )
    }
}

fn render_interactive_section(
    input1: Entity<InputState>,
    input2: Entity<InputState>,
    input3: Entity<InputState>,
) -> impl IntoElement {
    let theme = adabraka_ui::theme::use_theme();

    VStack::new()
        .w_full()
        .gap(px(16.0))
        .p(px(24.0))
        .bg(theme.tokens.primary.opacity(0.1))
        .border_2()
        .border_color(theme.tokens.primary)
        .rounded(theme.tokens.radius_lg)
        .child(
            VStack::new()
                .gap(px(4.0))
                .child(
                    div()
                        .text_size(px(20.0))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(theme.tokens.foreground)
                        .child("✨ Interactive Text Inputs")
                )
                .child(
                    div()
                        .text_size(px(14.0))
                        .text_color(theme.tokens.muted_foreground)
                        .child("These are fully interactive inputs using Entity<InputState>. You can type, edit, copy/paste, and use keyboard shortcuts!")
                )
        )
        .child(
            VStack::new()
                .w_full()
                .gap(px(16.0))
                .child(
                    VStack::new()
                        .w_full()
                        .gap(px(8.0))
                        .child(
                            div()
                                .text_size(px(14.0))
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(theme.tokens.foreground)
                                .child("Basic Text Input")
                        )
                        .child(
                            div()
                                .w_full()
                                .h(px(40.0))
                                .p(px(8.0))
                                .bg(theme.tokens.card)
                                .border_1()
                                .border_color(theme.tokens.border)
                                .rounded(theme.tokens.radius_md)
                                .child(input1)
                        )
                )
                .child(
                    VStack::new()
                        .w_full()
                        .gap(px(8.0))
                        .child(
                            div()
                                .text_size(px(14.0))
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(theme.tokens.foreground)
                                .child("Copy & Paste")
                        )
                        .child(
                            div()
                                .w_full()
                                .h(px(40.0))
                                .p(px(8.0))
                                .bg(theme.tokens.card)
                                .border_1()
                                .border_color(theme.tokens.border)
                                .rounded(theme.tokens.radius_md)
                                .child(input2)
                        )
                )
                .child(
                    VStack::new()
                        .w_full()
                        .gap(px(8.0))
                        .child(
                            div()
                                .text_size(px(14.0))
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(theme.tokens.foreground)
                                .child("Keyboard Navigation")
                        )
                        .child(
                            div()
                                .w_full()
                                .h(px(40.0))
                                .p(px(8.0))
                                .bg(theme.tokens.card)
                                .border_1()
                                .border_color(theme.tokens.border)
                                .rounded(theme.tokens.radius_md)
                                .child(input3)
                        )
                )
                .child(
                    div()
                        .w_full()
                        .p(px(12.0))
                        .bg(theme.tokens.muted.opacity(0.3))
                        .border_1()
                        .border_color(theme.tokens.border)
                        .rounded(theme.tokens.radius_md)
                        .child(
                            VStack::new()
                                .gap(px(4.0))
                                .child(
                                    div()
                                        .text_size(px(12.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(theme.tokens.foreground)
                                        .child("Keyboard Shortcuts:")
                                )
                                .child(
                                    div()
                                        .text_size(px(11.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child("Backspace/Delete • Left/Right arrows • Home/End • Cmd/Ctrl+A (Select all) • Cmd/Ctrl+C/V/X (Copy/Paste/Cut)")
                                )
                        )
                )
        )
}

fn render_section<I: IntoElement + 'static>(
    title: impl Into<SharedString>,
    description: impl Into<SharedString>,
    inputs: Vec<(&'static str, I)>,
) -> impl IntoElement {
    let theme = adabraka_ui::theme::use_theme();

    VStack::new()
        .w_full()
        .gap(px(16.0))
        .p(px(24.0))
        .bg(theme.tokens.card)
        .border_1()
        .border_color(theme.tokens.border)
        .rounded(theme.tokens.radius_lg)
        // Section header
        .child(
            VStack::new()
                .gap(px(4.0))
                .child(
                    div()
                        .text_size(px(20.0))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(theme.tokens.foreground)
                        .child(title.into())
                )
                .child(
                    div()
                        .text_size(px(14.0))
                        .text_color(theme.tokens.muted_foreground)
                        .child(description.into())
                )
        )
        // Input examples
        .child(
            VStack::new()
                .w_full()
                .gap(px(16.0))
                .children(
                    inputs.into_iter().map(|(label, input)| {
                        VStack::new()
                            .w_full()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(theme.tokens.foreground)
                                    .child(label)
                            )
                            .child(input)
                    })
                )
        )
}

fn render_prefix_suffix_section(
    prefix_input: &Entity<InputState>,
    suffix_input: &Entity<InputState>,
    both_input: &Entity<InputState>,
) -> impl IntoElement {
    let theme = adabraka_ui::theme::use_theme();

    VStack::new()
        .w_full()
        .gap(px(16.0))
        .p(px(24.0))
        .bg(theme.tokens.card)
        .border_1()
        .border_color(theme.tokens.border)
        .rounded(theme.tokens.radius_lg)
        .child(
            VStack::new()
                .gap(px(4.0))
                .child(
                    div()
                        .text_size(px(20.0))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(theme.tokens.foreground)
                        .child("Input with Prefix/Suffix")
                )
                .child(
                    div()
                        .text_size(px(14.0))
                        .text_color(theme.tokens.muted_foreground)
                        .child("Inputs can have prefix and suffix elements like icons or labels")
                )
        )
        .child(
            VStack::new()
                .w_full()
                .gap(px(16.0))
                .child(
                    VStack::new()
                        .w_full()
                        .gap(px(8.0))
                        .child(
                            div()
                                .text_size(px(14.0))
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(theme.tokens.foreground)
                                .child("With Prefix")
                        )
                        .child(
                            Input::new(prefix_input)
                                .placeholder("Enter URL")
                                .prefix(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child("https://")
                                )
                        )
                )
                .child(
                    VStack::new()
                        .w_full()
                        .gap(px(8.0))
                        .child(
                            div()
                                .text_size(px(14.0))
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(theme.tokens.foreground)
                                .child("With Suffix")
                        )
                        .child(
                            Input::new(suffix_input)
                                .placeholder("Enter amount")
                                .suffix(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child("USD")
                                )
                        )
                )
                .child(
                    VStack::new()
                        .w_full()
                        .gap(px(8.0))
                        .child(
                            div()
                                .text_size(px(14.0))
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(theme.tokens.foreground)
                                .child("With Both")
                        )
                        .child(
                            Input::new(both_input)
                                .placeholder("0.00")
                                .prefix(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child("$")
                                )
                                .suffix(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child(".00")
                                )
                        )
                )
        )
}

fn render_form_example(
    form_name: &Entity<InputState>,
    form_email: &Entity<InputState>,
    form_password: &Entity<InputState>,
    form_bio: &Entity<InputState>,
) -> impl IntoElement {
    let theme = adabraka_ui::theme::use_theme();

    VStack::new()
        .w_full()
        .gap(px(16.0))
        .p(px(24.0))
        .bg(theme.tokens.card)
        .border_1()
        .border_color(theme.tokens.border)
        .rounded(theme.tokens.radius_lg)
        .child(
            VStack::new()
                .gap(px(4.0))
                .child(
                    div()
                        .text_size(px(20.0))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(theme.tokens.foreground)
                        .child("Complete Form Example")
                )
                .child(
                    div()
                        .text_size(px(14.0))
                        .text_color(theme.tokens.muted_foreground)
                        .child("A typical form using various input components")
                )
        )
        .child(
            VStack::new()
                .w_full()
                .gap(px(20.0))
                // Name field
                .child(
                    VStack::new()
                        .w_full()
                        .gap(px(8.0))
                        .child(
                            div()
                                .text_size(px(14.0))
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(theme.tokens.foreground)
                                .child("Full Name")
                        )
                        .child(
                            Input::new(form_name)
                                .placeholder("John Doe")
                                .variant(InputVariant::Outline)
                        )
                )
                // Email field
                .child(
                    VStack::new()
                        .w_full()
                        .gap(px(8.0))
                        .child(
                            div()
                                .text_size(px(14.0))
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(theme.tokens.foreground)
                                .child("Email")
                        )
                        .child(
                            Input::new(form_email)
                                .placeholder("john@example.com")
                                .variant(InputVariant::Outline)
                                .prefix(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child("📧")
                                )
                        )
                )
                // Password field
                .child(
                    VStack::new()
                        .w_full()
                        .gap(px(8.0))
                        .child(
                            div()
                                .text_size(px(14.0))
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(theme.tokens.foreground)
                                .child("Password")
                        )
                        .child(
                            Input::new(form_password)
                                .placeholder("Enter a secure password")
                                .variant(InputVariant::Outline)
                                .password(true)
                        )
                )
                // Bio field (using a regular input but with a note)
                .child(
                    VStack::new()
                        .w_full()
                        .gap(px(8.0))
                        .child(
                            div()
                                .text_size(px(14.0))
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(theme.tokens.foreground)
                                .child("Bio (Note: Multi-line textarea coming soon)")
                        )
                        .child(
                            Input::new(form_bio)
                                .placeholder("Tell us about yourself...")
                                .variant(InputVariant::Outline)
                                .size(InputSize::Lg)
                        )
                )
                // Form actions
                .child(
                    HStack::new()
                        .w_full()
                        .gap(px(12.0))
                        .justify(Justify::End)
                        .child(
                            Button::new("cancel-btn", "Cancel")
                                .variant(ButtonVariant::Outline)
                                .size(ButtonSize::Md)
                        )
                        .child(
                            Button::new("submit-btn", "Submit")
                                .variant(ButtonVariant::Default)
                                .size(ButtonSize::Md)
                        )
                )
        )
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
                    size(px(900.0), px(800.0)),
                    cx,
                ))),
                titlebar: Some(TitlebarOptions {
                    title: Some("Input Components Demo".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_window, cx| cx.new(|cx| InputDemoApp::new(cx)),
        )
        .unwrap();
    });
}