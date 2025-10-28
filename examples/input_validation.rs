use adabraka_ui::{
    components::{
        input::{Input, InputType, InputVariant, InputSize},
        input_state::InputState,
        button::{Button, ButtonVariant, ButtonSize},
        scrollable::scrollable_vertical,
    },
    layout::VStack,
    theme::{install_theme, Theme},
};
use gpui::{*, prelude::FluentBuilder};
use std::path::PathBuf;

struct Assets {
    base: PathBuf,
}

impl gpui::AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<std::borrow::Cow<'static, [u8]>>> {
        std::fs::read(self.base.join(path))
            .map(|data| Some(std::borrow::Cow::Owned(data)))
            .map_err(|err| err.into())
    }

    fn list(&self, path: &str) -> Result<Vec<gpui::SharedString>> {
        std::fs::read_dir(self.base.join(path))
            .map(|entries| {
                entries
                    .filter_map(|entry| {
                        entry
                            .ok()
                            .and_then(|entry| entry.file_name().into_string().ok())
                            .map(gpui::SharedString::from)
                    })
                    .collect()
            })
            .map_err(|err| err.into())
    }
}

struct ValidationDemoApp {
    // Various input types with validation
    email_input: Entity<InputState>,
    password_input: Entity<InputState>,
    confirm_password_input: Entity<InputState>,
    phone_input: Entity<InputState>,
    credit_card_input: Entity<InputState>,
    url_input: Entity<InputState>,
    number_input: Entity<InputState>,
    date_input: Entity<InputState>,
    time_input: Entity<InputState>,
    username_input: Entity<InputState>,
    bio_input: Entity<InputState>,

    // Form validation state
    form_is_valid: bool,
    form_submitted: bool,
}

impl ValidationDemoApp {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            // Email with built-in validation
            email_input: cx.new(|cx| InputState::new(cx)
                .input_type(InputType::Email)
                .required(true)
                .placeholder("email@example.com")),

            // Password with minimum length
            password_input: cx.new(|cx| InputState::new(cx)
                .input_type(InputType::Password)
                .required(true)
                .min_length(8)
                .placeholder("Enter password")),

            // Confirm password (will validate match)
            confirm_password_input: cx.new(|cx| InputState::new(cx)
                .input_type(InputType::Password)
                .required(true)
                .placeholder("Confirm password")),

            // Phone with formatting
            phone_input: cx.new(|cx| InputState::new(cx)
                .input_type(InputType::Tel)
                .required(true)
                .placeholder("(555) 555-5555")),

            // Credit card with validation
            credit_card_input: cx.new(|cx| InputState::new(cx)
                .input_type(InputType::CreditCard)
                .placeholder("1234 5678 9012 3456")),

            // URL validation
            url_input: cx.new(|cx| InputState::new(cx)
                .input_type(InputType::Url)
                .placeholder("https://example.com")),

            // Number with min/max
            number_input: cx.new(|cx| {
                let mut state = InputState::new(cx).input_type(InputType::Number);
                state.validation_rules.min_value = Some(1.0);
                state.validation_rules.max_value = Some(100.0);
                state.placeholder = "1-100".into();
                state
            }),

            // Date input
            date_input: cx.new(|cx| InputState::new(cx)
                .input_type(InputType::Date)
                .placeholder("MM/DD/YYYY")),

            // Time input
            time_input: cx.new(|cx| InputState::new(cx)
                .input_type(InputType::Time)
                .placeholder("HH:MM")),

            // Username with length constraints
            username_input: cx.new(|cx| InputState::new(cx)
                .required(true)
                .min_length(3)
                .max_length(20)
                .placeholder("johndoe")),

            // Bio with character count
            bio_input: cx.new(|cx| InputState::new(cx)
                .max_length(200)
                .placeholder("Tell us about yourself...")),

            form_is_valid: false,
            form_submitted: false,
        }
    }

    fn validate_form(&mut self, cx: &mut Context<Self>) {
        // Check if passwords match
        let password = self.password_input.read(cx).content.clone();
        let confirm = self.confirm_password_input.read(cx).content.clone();

        if !password.is_empty() && !confirm.is_empty() && password != confirm {
            self.confirm_password_input.update(cx, |state, cx| {
                state.validation_error = Some(adabraka_ui::components::input::ValidationError {
                    message: "Passwords do not match".into(),
                    field_name: None,
                });
                cx.notify();
            });
        }

        // Check overall form validity
        let email_valid = self.email_input.read(cx).validation_error.is_none();
        let password_valid = self.password_input.read(cx).validation_error.is_none() && password.len() >= 8;
        let passwords_match = password == confirm;
        let phone_valid = self.phone_input.read(cx).validation_error.is_none();
        let username_valid = self.username_input.read(cx).validation_error.is_none();

        self.form_is_valid = email_valid && password_valid && passwords_match && phone_valid && username_valid;
    }

    fn submit_form(&mut self, cx: &mut Context<Self>) {
        // Validate all fields
        let _ = self.email_input.update(cx, |state, cx| state.validate(cx));
        let _ = self.password_input.update(cx, |state, cx| state.validate(cx));
        let _ = self.confirm_password_input.update(cx, |state, cx| state.validate(cx));
        let _ = self.phone_input.update(cx, |state, cx| state.validate(cx));
        let _ = self.username_input.update(cx, |state, cx| state.validate(cx));

        self.validate_form(cx);

        if self.form_is_valid {
            self.form_submitted = true;
            cx.notify();
        }
    }
}

impl Render for ValidationDemoApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = adabraka_ui::theme::use_theme();

        VStack::new()
            .size_full()
            .bg(theme.tokens.background)
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
                            .text_size(px(28.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(theme.tokens.foreground)
                            .child("Industry-Standard Input Validation Demo")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("Comprehensive validation with real-time feedback, masking, and accessibility")
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
                                // Registration Form Section
                                .child(
                                    VStack::new()
                                        .w_full()
                                        .max_w(px(600.0))
                                        .gap(px(20.0))
                                        .p(px(24.0))
                                        .bg(theme.tokens.card)
                                        .border_1()
                                        .border_color(theme.tokens.border)
                                        .rounded(theme.tokens.radius_lg)
                                        .child(
                                            div()
                                                .text_size(px(20.0))
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .text_color(theme.tokens.foreground)
                                                .child("User Registration Form")
                                        )
                                        // Username field
                                        .child(
                                            VStack::new()
                                                .w_full()
                                                .gap(px(4.0))
                                                .child(
                                                    div()
                                                        .text_size(px(14.0))
                                                        .font_weight(FontWeight::MEDIUM)
                                                        .text_color(theme.tokens.foreground)
                                                        .child("Username *")
                                                )
                                                .child(
                                                    Input::new(&self.username_input)
                                                        .variant(InputVariant::Outline)
                                                        .helper_text("3-20 characters, letters and numbers only")
                                                        .show_character_count(true)
                                                        .aria_label("Username")
                                                        .on_change({
                                                            let entity = cx.entity();
                                                            move |_value, cx| {
                                                                entity.update(cx, |app, cx| {
                                                                    app.validate_form(cx);
                                                                });
                                                            }
                                                        })
                                                )
                                        )
                                        // Email field
                                        .child(
                                            VStack::new()
                                                .w_full()
                                                .gap(px(4.0))
                                                .child(
                                                    div()
                                                        .text_size(px(14.0))
                                                        .font_weight(FontWeight::MEDIUM)
                                                        .text_color(theme.tokens.foreground)
                                                        .child("Email Address *")
                                                )
                                                .child(
                                                    Input::new(&self.email_input)
                                                        .input_type(InputType::Email)
                                                        .variant(InputVariant::Outline)
                                                        .required(true)
                                                        .helper_text("We'll never share your email")
                                                        .aria_label("Email address")
                                                        .autocomplete("email")
                                                        .on_blur({
                                                            let entity = cx.entity();
                                                            move |_value, cx| {
                                                                entity.update(cx, |app, cx| {
                                                                    app.validate_form(cx);
                                                                });
                                                            }
                                                        })
                                                )
                                        )
                                        // Phone field
                                        .child(
                                            VStack::new()
                                                .w_full()
                                                .gap(px(4.0))
                                                .child(
                                                    div()
                                                        .text_size(px(14.0))
                                                        .font_weight(FontWeight::MEDIUM)
                                                        .text_color(theme.tokens.foreground)
                                                        .child("Phone Number *")
                                                )
                                                .child(
                                                    Input::new(&self.phone_input)
                                                        .input_type(InputType::Tel)
                                                        .variant(InputVariant::Outline)
                                                        .required(true)
                                                        .helper_text("US format: (555) 555-5555")
                                                        .aria_label("Phone number")
                                                        .autocomplete("tel")
                                                        .on_change({
                                                            let entity = cx.entity();
                                                            move |_value, cx| {
                                                                entity.update(cx, |app, cx| {
                                                                    app.validate_form(cx);
                                                                });
                                                            }
                                                        })
                                                )
                                        )
                                        // Password field
                                        .child(
                                            VStack::new()
                                                .w_full()
                                                .gap(px(4.0))
                                                .child(
                                                    div()
                                                        .text_size(px(14.0))
                                                        .font_weight(FontWeight::MEDIUM)
                                                        .text_color(theme.tokens.foreground)
                                                        .child("Password *")
                                                )
                                                .child(
                                                    Input::new(&self.password_input)
                                                        .input_type(InputType::Password)
                                                        .password(true)
                                                        .variant(InputVariant::Outline)
                                                        .required(true)
                                                        .min_length(8)
                                                        .helper_text("At least 8 characters")
                                                        .aria_label("Password")
                                                        .autocomplete("new-password")
                                                        .on_change({
                                                            let entity = cx.entity();
                                                            move |_value, cx| {
                                                                entity.update(cx, |app, cx| {
                                                                    app.validate_form(cx);
                                                                    cx.notify();
                                                                });
                                                            }
                                                        })
                                                )
                                        )
                                        // Confirm Password field
                                        .child(
                                            VStack::new()
                                                .w_full()
                                                .gap(px(4.0))
                                                .child(
                                                    div()
                                                        .text_size(px(14.0))
                                                        .font_weight(FontWeight::MEDIUM)
                                                        .text_color(theme.tokens.foreground)
                                                        .child("Confirm Password *")
                                                )
                                                .child(
                                                    Input::new(&self.confirm_password_input)
                                                        .input_type(InputType::Password)
                                                        .password(true)
                                                        .variant(InputVariant::Outline)
                                                        .required(true)
                                                        .aria_label("Confirm password")
                                                        .autocomplete("new-password")
                                                        .on_change({
                                                            let entity = cx.entity();
                                                            move |_value, cx| {
                                                                entity.update(cx, |app, cx| {
                                                                    app.validate_form(cx);
                                                                    cx.notify();
                                                                });
                                                            }
                                                        })
                                                        .on_blur({
                                                            let entity = cx.entity();
                                                            move |_value, cx| {
                                                                entity.update(cx, |app, cx| {
                                                                    app.validate_form(cx);
                                                                    cx.notify();
                                                                });
                                                            }
                                                        })
                                                )
                                        )
                                        // Bio field
                                        .child(
                                            VStack::new()
                                                .w_full()
                                                .gap(px(4.0))
                                                .child(
                                                    div()
                                                        .text_size(px(14.0))
                                                        .font_weight(FontWeight::MEDIUM)
                                                        .text_color(theme.tokens.foreground)
                                                        .child("Bio (optional)")
                                                )
                                                .child(
                                                    Input::new(&self.bio_input)
                                                        .variant(InputVariant::Outline)
                                                        .size(InputSize::Lg)
                                                        .max_length(200)
                                                        .show_character_count(true)
                                                        .helper_text("Brief description about yourself")
                                                        .aria_label("Biography")
                                                )
                                        )
                                        // Submit button
                                        .child(
                                            div()
                                                .w_full()
                                                .child(
                                                    Button::new("submit-registration-btn", "Submit Registration")
                                                        .variant(ButtonVariant::Default)
                                                        .size(ButtonSize::Lg)
                                                        .on_click({
                                                            let entity = cx.entity();
                                                            move |_event, _window, cx| {
                                                                entity.update(cx, |app, cx| {
                                                                    app.submit_form(cx);
                                                                });
                                                            }
                                                        })
                                                )
                                        )
                                        .when(self.form_submitted, |v| {
                                            v.child(
                                                div()
                                                    .w_full()
                                                    .p(px(12.0))
                                                    .bg(theme.tokens.primary.opacity(0.1))
                                                    .border_1()
                                                    .border_color(theme.tokens.primary)
                                                    .rounded(theme.tokens.radius_md)
                                                    .child(
                                                        div()
                                                            .text_color(theme.tokens.primary)
                                                            .text_size(px(14.0))
                                                            .child("✓ Form submitted successfully!")
                                                    )
                                            )
                                        })
                                )
                                // Other Input Types Section
                                .child(
                                    VStack::new()
                                        .w_full()
                                        .max_w(px(600.0))
                                        .gap(px(20.0))
                                        .p(px(24.0))
                                        .bg(theme.tokens.card)
                                        .border_1()
                                        .border_color(theme.tokens.border)
                                        .rounded(theme.tokens.radius_lg)
                                        .child(
                                            div()
                                                .text_size(px(20.0))
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .text_color(theme.tokens.foreground)
                                                .child("Specialized Input Types")
                                        )
                                        // Credit card
                                        .child(
                                            VStack::new()
                                                .w_full()
                                                .gap(px(4.0))
                                                .child(
                                                    div()
                                                        .text_size(px(14.0))
                                                        .font_weight(FontWeight::MEDIUM)
                                                        .text_color(theme.tokens.foreground)
                                                        .child("Credit Card Number")
                                                )
                                                .child(
                                                    Input::new(&self.credit_card_input)
                                                        .input_type(InputType::CreditCard)
                                                        .variant(InputVariant::Outline)
                                                        .helper_text("Test with: 4111 1111 1111 1111")
                                                        .aria_label("Credit card number")
                                                        .autocomplete("cc-number")
                                                )
                                        )
                                        // URL
                                        .child(
                                            VStack::new()
                                                .w_full()
                                                .gap(px(4.0))
                                                .child(
                                                    div()
                                                        .text_size(px(14.0))
                                                        .font_weight(FontWeight::MEDIUM)
                                                        .text_color(theme.tokens.foreground)
                                                        .child("Website URL")
                                                )
                                                .child(
                                                    Input::new(&self.url_input)
                                                        .input_type(InputType::Url)
                                                        .variant(InputVariant::Outline)
                                                        .helper_text("Must start with http:// or https://")
                                                        .aria_label("Website URL")
                                                        .autocomplete("url")
                                                )
                                        )
                                        // Number
                                        .child(
                                            VStack::new()
                                                .w_full()
                                                .gap(px(4.0))
                                                .child(
                                                    div()
                                                        .text_size(px(14.0))
                                                        .font_weight(FontWeight::MEDIUM)
                                                        .text_color(theme.tokens.foreground)
                                                        .child("Quantity (1-100)")
                                                )
                                                .child(
                                                    Input::new(&self.number_input)
                                                        .input_type(InputType::Number)
                                                        .variant(InputVariant::Outline)
                                                        .helper_text("Enter a number between 1 and 100")
                                                        .aria_label("Quantity")
                                                )
                                        )
                                        // Date
                                        .child(
                                            VStack::new()
                                                .w_full()
                                                .gap(px(4.0))
                                                .child(
                                                    div()
                                                        .text_size(px(14.0))
                                                        .font_weight(FontWeight::MEDIUM)
                                                        .text_color(theme.tokens.foreground)
                                                        .child("Date of Birth")
                                                )
                                                .child(
                                                    Input::new(&self.date_input)
                                                        .input_type(InputType::Date)
                                                        .variant(InputVariant::Outline)
                                                        .helper_text("Format: MM/DD/YYYY")
                                                        .aria_label("Date of birth")
                                                        .autocomplete("bday")
                                                )
                                        )
                                        // Time
                                        .child(
                                            VStack::new()
                                                .w_full()
                                                .gap(px(4.0))
                                                .child(
                                                    div()
                                                        .text_size(px(14.0))
                                                        .font_weight(FontWeight::MEDIUM)
                                                        .text_color(theme.tokens.foreground)
                                                        .child("Appointment Time")
                                                )
                                                .child(
                                                    Input::new(&self.time_input)
                                                        .input_type(InputType::Time)
                                                        .variant(InputVariant::Outline)
                                                        .helper_text("24-hour format: HH:MM")
                                                        .aria_label("Appointment time")
                                                )
                                        )
                                )
                                // Features showcase
                                .child(
                                    VStack::new()
                                        .w_full()
                                        .max_w(px(600.0))
                                        .gap(px(12.0))
                                        .p(px(20.0))
                                        .bg(theme.tokens.muted.opacity(0.3))
                                        .border_1()
                                        .border_color(theme.tokens.border)
                                        .rounded(theme.tokens.radius_lg)
                                        .child(
                                            div()
                                                .text_size(px(16.0))
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .text_color(theme.tokens.foreground)
                                                .child("✨ Features Demonstrated")
                                        )
                                        .child(
                                            div()
                                                .text_size(px(13.0))
                                                .text_color(theme.tokens.muted_foreground)
                                                .line_height(relative(1.6))
                                                .child("• Real-time validation with error messages\n• Character filtering (try typing letters in number fields)\n• Input masking (phone, credit card, date formatting)\n• Min/max length enforcement\n• Character counter\n• Helper text and error states\n• Password matching validation\n• Accessibility with ARIA labels\n• Autocomplete attributes\n• Click on filled input places cursor at end\n• Proper focus management\n• Industry-standard UX patterns")
                                        )
                                )
                        ),
                    ),
            )
    }
}

fn main() {
    Application::new()
        .with_assets(Assets { base: PathBuf::from(env!("CARGO_MANIFEST_DIR")) })
        .run(move |cx| {
        // Install dark theme
        install_theme(cx, Theme::dark());

        // Initialize input system
        adabraka_ui::init(cx);
        adabraka_ui::set_icon_base_path("assets/icons");

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(900.0), px(800.0)),
                    cx,
                ))),
                titlebar: Some(TitlebarOptions {
                    title: Some("Input Validation Demo - Industry Standard".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_window, cx| cx.new(|cx| ValidationDemoApp::new(cx)),
        )
        .unwrap();
    });
}