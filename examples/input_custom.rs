use adabraka_ui::{
    components::{
        input::{Input, InputType, InputVariant, InputSize},
        input_state::InputState,
        scrollable::scrollable_vertical,
    },
    layout::VStack,
    theme::{install_theme, Theme},
};
use gpui::{*, prelude::FluentBuilder};

struct CustomInputApp {
    // Simple usage - just a basic input
    simple_input: Entity<InputState>,

    // Custom validation - company email only
    company_email_input: Entity<InputState>,

    // Custom filter - username with specific characters
    username_input: Entity<InputState>,

    // Custom formatter - currency formatting
    currency_input: Entity<InputState>,

    // Complex custom - product code with all three
    product_code_input: Entity<InputState>,
}

impl CustomInputApp {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            // 1. SIMPLE USAGE - Just a basic input, no configuration needed
            simple_input: cx.new(|cx| InputState::new(cx)),

            // 2. Custom validation - company email
            company_email_input: cx.new(|cx| InputState::new(cx)),

            // 3. Custom filter - username
            username_input: cx.new(|cx| InputState::new(cx)),

            // 4. Custom formatter - currency
            currency_input: cx.new(|cx| InputState::new(cx)),

            // 5. Complex custom - product code
            product_code_input: cx.new(|cx| InputState::new(cx)),
        }
    }
}

impl Render for CustomInputApp {
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
                            .child("Flexible Input Component Demo")
                    )
                    .child(
                        div()
                            .text_size(px(16.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("Simple to use, powerful when you need it")
                    )
            )
            // Scrollable content
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
                                .gap(px(32.0))
                                // 1. SIMPLE USAGE
                                .child(
                                    VStack::new()
                                        .gap(px(16.0))
                                        .p(px(20.0))
                                        .bg(theme.tokens.card)
                                        .border_1()
                                        .border_color(theme.tokens.border)
                                        .rounded(theme.tokens.radius_lg)
                                        .child(
                                            div()
                                                .text_size(px(18.0))
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .text_color(theme.tokens.foreground)
                                                .child("1. Simple Usage - Just Works™")
                                        )
                                        .child(
                                            div()
                                                .text_size(px(14.0))
                                                .text_color(theme.tokens.muted_foreground)
                                                .child("The simplest possible usage - just create an Input with a state:")
                                        )
                                        .child(
                                            // Simplest possible usage
                                            Input::new(&self.simple_input)
                                                .placeholder("Just type anything...")
                                        )
                                        .child(
                                            div()
                                                .p(px(12.0))
                                                .bg(theme.tokens.muted.opacity(0.1))
                                                .rounded(theme.tokens.radius_md)
                                                .font_family("monospace")
                                                .text_size(px(12.0))
                                                .child("Input::new(&state).placeholder(\"Just type anything...\")")
                                        )
                                )
                                // 2. CUSTOM VALIDATION
                                .child(
                VStack::new()
                    .gap(px(16.0))
                    .p(px(20.0))
                    .bg(theme.tokens.card)
                    .border_1()
                    .border_color(theme.tokens.border)
                    .rounded(theme.tokens.radius_lg)
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(theme.tokens.foreground)
                            .child("2. Custom Validation - Your Rules")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("Only accepts @company.com email addresses:")
                    )
                    .child(
                        Input::new(&self.company_email_input)
                            .placeholder("john@company.com")
                            .helper_text("Must be a company email address")
                            .custom_validator(|value| {
                                if value.is_empty() {
                                    return Ok(());
                                }
                                if value.contains("@company.com") {
                                    Ok(())
                                } else {
                                    Err("Email must end with @company.com".to_string())
                                }
                            })
                    )
                    .child(
                        div()
                            .p(px(12.0))
                            .bg(theme.tokens.muted.opacity(0.1))
                            .rounded(theme.tokens.radius_md)
                            .font_family("monospace")
                            .text_size(px(12.0))
                            .line_height(relative(1.5))
                            .child(".custom_validator(|value| {\n    if value.contains(\"@company.com\") {\n        Ok(())\n    } else {\n        Err(\"Must be company email\")\n    }\n})")
                    )
                                )
                                // 3. CUSTOM FILTER
                                .child(
                VStack::new()
                    .gap(px(16.0))
                    .p(px(20.0))
                    .bg(theme.tokens.card)
                    .border_1()
                    .border_color(theme.tokens.border)
                    .rounded(theme.tokens.radius_lg)
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(theme.tokens.foreground)
                            .child("3. Custom Filter - Control Input Characters")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("Username that only allows letters, numbers, and underscores:")
                    )
                    .child(
                        Input::new(&self.username_input)
                            .placeholder("john_doe_123")
                            .helper_text("Letters, numbers, and underscores only")
                            .custom_filter(|input| {
                                input.chars()
                                    .filter(|c| c.is_alphanumeric() || *c == '_')
                                    .collect()
                            })
                    )
                    .child(
                        div()
                            .p(px(12.0))
                            .bg(theme.tokens.muted.opacity(0.1))
                            .rounded(theme.tokens.radius_md)
                            .font_family("monospace")
                            .text_size(px(12.0))
                            .line_height(relative(1.5))
                            .child(".custom_filter(|input| {\n    input.chars()\n        .filter(|c| c.is_alphanumeric() || *c == '_')\n        .collect()\n})")
                    )
                                )
                                // 4. CUSTOM FORMATTER
                                .child(
                VStack::new()
                    .gap(px(16.0))
                    .p(px(20.0))
                    .bg(theme.tokens.card)
                    .border_1()
                    .border_color(theme.tokens.border)
                    .rounded(theme.tokens.radius_lg)
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(theme.tokens.foreground)
                            .child("4. Custom Formatter - Display Formatting")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("Currency input with automatic formatting (type numbers):")
                    )
                    .child(
                        Input::new(&self.currency_input)
                            .placeholder("0.00")
                            .prefix(
                                div()
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("$")
                            )
                            .helper_text("Enter amount in dollars")
                            .custom_filter(|input| {
                                // Only allow numbers and decimal point
                                input.chars()
                                    .filter(|c| c.is_ascii_digit() || *c == '.')
                                    .collect()
                            })
                            .custom_validator(|value| {
                                if value.is_empty() {
                                    return Ok(());
                                }
                                match value.parse::<f64>() {
                                    Ok(_) => Ok(()),
                                    Err(_) => Err("Invalid currency format".to_string()),
                                }
                            })
                    )
                    .child(
                        div()
                            .p(px(12.0))
                            .bg(theme.tokens.muted.opacity(0.1))
                            .rounded(theme.tokens.radius_md)
                            .font_family("monospace")
                            .text_size(px(12.0))
                            .child("Filter: numbers and '.' only | Validate: must be valid number")
                    )
                                )
                                // 5. COMPLEX EXAMPLE
                                .child(
                VStack::new()
                    .gap(px(16.0))
                    .p(px(20.0))
                    .bg(theme.tokens.card)
                    .border_1()
                    .border_color(theme.tokens.border)
                    .rounded(theme.tokens.radius_lg)
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(theme.tokens.foreground)
                            .child("5. Complex Example - Product Code")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("Product code with format ABC-1234 (3 letters, dash, 4 numbers):")
                    )
                    .child(
                        Input::new(&self.product_code_input)
                            .placeholder("ABC-1234")
                            .helper_text("Format: XXX-#### (3 letters, dash, 4 digits)")
                            .max_length(8)
                            .custom_filter(|input| {
                                let mut result = String::new();
                                let mut letter_count = 0;
                                let mut digit_count = 0;
                                let mut has_dash = false;

                                for c in input.chars() {
                                    if letter_count < 3 && c.is_ascii_alphabetic() && !has_dash {
                                        result.push(c.to_ascii_uppercase());
                                        letter_count += 1;
                                    } else if letter_count == 3 && c == '-' && !has_dash {
                                        result.push(c);
                                        has_dash = true;
                                    } else if has_dash && digit_count < 4 && c.is_ascii_digit() {
                                        result.push(c);
                                        digit_count += 1;
                                    }

                                    // Auto-insert dash after 3 letters
                                    if letter_count == 3 && !has_dash && !result.contains('-') {
                                        result.push('-');
                                        has_dash = true;
                                    }
                                }

                                result
                            })
                            .custom_validator(|value| {
                                if value.is_empty() {
                                    return Ok(());
                                }

                                let parts: Vec<&str> = value.split('-').collect();
                                if parts.len() != 2 {
                                    return Err("Must be in format XXX-####".to_string());
                                }

                                let letters = parts[0];
                                let numbers = parts[1];

                                if letters.len() != 3 || !letters.chars().all(|c| c.is_ascii_alphabetic()) {
                                    return Err("First part must be exactly 3 letters".to_string());
                                }

                                if numbers.len() != 4 || !numbers.chars().all(|c| c.is_ascii_digit()) {
                                    return Err("Second part must be exactly 4 digits".to_string());
                                }

                                Ok(())
                            })
                    )
                    .child(
                        div()
                            .p(px(12.0))
                            .bg(theme.tokens.muted.opacity(0.1))
                            .rounded(theme.tokens.radius_md)
                            .font_family("monospace")
                            .text_size(px(12.0))
                            .child("Auto-formats as you type, validates format, enforces max length")
                    )
                                )
                                // Summary
                                .child(
                VStack::new()
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
                            .child("✨ Key Features")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.foreground)
                            .line_height(relative(1.6))
                            .child("• Start simple - Just Input::new(&state)\n• Add validation when needed - custom_validator()\n• Control input characters - custom_filter()\n• Format display - custom_formatter()\n• Combine with built-in types (Email, Number, Tel, etc.)\n• Full accessibility and keyboard support included\n• Industry-standard UX patterns by default")
                    )
                                )
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
                    size(px(900.0), px(900.0)),
                    cx,
                ))),
                titlebar: Some(TitlebarOptions {
                    title: Some("Custom Input Extensions Demo".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_window, cx| cx.new(|cx| CustomInputApp::new(cx)),
        )
        .unwrap();
    });
}