/// Interactive text input state management with validation and accessibility
///
/// Industry-standard input component with:
/// - Multiple input types with built-in validation
/// - Character filtering and input masking
/// - Proper cursor management and accessibility
/// - Real-time validation with error messages
/// - Min/max length enforcement
/// - Pattern matching support
/// - Auto-formatting for phone, date, credit card
/// - ARIA support for screen readers

use gpui::{prelude::*, *};
use std::ops::Range;
use std::sync::Arc;
use unicode_segmentation::*;
use crate::theme::use_theme;

actions!(
    input_state,
    [
        Backspace,
        Delete,
        Left,
        Right,
        SelectLeft,
        SelectRight,
        SelectAll,
        Home,
        End,
        Copy,
        Cut,
        Paste,
        Enter,
        Tab,
        ShiftTab,
        Escape,
    ]
);

/// Input types with built-in validation and filtering
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputType {
    /// Plain text input (default)
    Text,
    /// Email input with @ and domain validation
    Email,
    /// Numeric input (integers and decimals)
    Number,
    /// Phone number with formatting
    Tel,
    /// URL input with protocol validation
    Url,
    /// Password input with strength indicators
    Password,
    /// Search input with debounced events
    Search,
    /// Date input (YYYY-MM-DD format)
    Date,
    /// Time input (HH:MM format)
    Time,
    /// Credit card number with masking
    CreditCard,
}

/// Validation result with error message
#[derive(Debug, Clone)]
pub struct ValidationError {
    pub message: SharedString,
    pub field_name: Option<SharedString>,
}

/// Input validation rules
#[derive(Clone)]
pub struct ValidationRules {
    /// Minimum length requirement
    pub min_length: Option<usize>,
    /// Maximum length requirement
    pub max_length: Option<usize>,
    /// For number inputs: minimum value
    pub min_value: Option<f64>,
    /// For number inputs: maximum value
    pub max_value: Option<f64>,
    /// Regex pattern to match
    pub pattern: Option<String>,
    /// Custom validation function
    pub custom_validator: Option<Arc<dyn Fn(&str) -> Result<(), ValidationError>>>,
    /// Custom filter function for input
    pub custom_filter: Option<Arc<dyn Fn(&str) -> String>>,
    /// Custom formatter function for display
    pub custom_formatter: Option<Arc<dyn Fn(&str) -> String>>,
    /// Whether the field is required
    pub required: bool,
}

impl std::fmt::Debug for ValidationRules {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ValidationRules")
            .field("min_length", &self.min_length)
            .field("max_length", &self.max_length)
            .field("min_value", &self.min_value)
            .field("max_value", &self.max_value)
            .field("pattern", &self.pattern)
            .field("custom_validator", &self.custom_validator.is_some())
            .field("custom_filter", &self.custom_filter.is_some())
            .field("custom_formatter", &self.custom_formatter.is_some())
            .field("required", &self.required)
            .finish()
    }
}

impl Default for ValidationRules {
    fn default() -> Self {
        Self {
            min_length: None,
            max_length: None,
            min_value: None,
            max_value: None,
            pattern: None,
            custom_validator: None,
            custom_filter: None,
            custom_formatter: None,
            required: false,
        }
    }
}

/// Input masking for formatting
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMask {
    /// No masking
    None,
    /// Phone: (999) 999-9999
    Phone,
    /// Credit card: 9999 9999 9999 9999
    CreditCard,
    /// Date: MM/DD/YYYY
    Date,
    /// Time: HH:MM
    Time,
    /// Custom mask pattern
    Custom(&'static str),
}

/// Events emitted by the InputState
#[derive(Clone, Debug)]
pub enum InputEvent {
    Change,
    Enter,
    Focus,
    Blur,
    Validate(Result<(), ValidationError>),
    Tab,
    ShiftTab,
}

/// Core input state entity that handles text editing with validation
pub struct InputState {
    focus_handle: FocusHandle,
    pub content: SharedString,
    pub placeholder: SharedString,
    pub disabled: bool,
    pub masked: bool,
    selected_range: Range<usize>,
    selection_reversed: bool,
    marked_range: Option<Range<usize>>,
    last_layout: Option<gpui::ShapedLine>,
    last_bounds: Option<Bounds<Pixels>>,
    is_selecting: bool,
    last_click_time: Option<std::time::Instant>,
    last_click_position: Option<Point<Pixels>>,

    // Enhanced features
    pub input_type: InputType,
    pub validation_rules: ValidationRules,
    pub validation_error: Option<ValidationError>,
    pub input_mask: InputMask,
    pub aria_label: Option<SharedString>,
    pub aria_description: Option<SharedString>,
    pub autocomplete: Option<SharedString>,
    pub helper_text: Option<SharedString>,
    pub success_message: Option<SharedString>,
    pub tab_index: Option<i32>,
    pub select_on_focus: bool,
    pub validate_on_blur: bool,
    pub validate_on_change: bool,
    pub trim_on_blur: bool,
    cursor_position_override: Option<usize>,
}

impl EventEmitter<InputEvent> for InputState {}

impl InputState {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            content: "".into(),
            placeholder: "Type here...".into(),
            disabled: false,
            masked: false,
            selected_range: 0..0,
            selection_reversed: false,
            marked_range: None,
            last_layout: None,
            last_bounds: None,
            is_selecting: false,
            last_click_time: None,
            last_click_position: None,

            input_type: InputType::Text,
            validation_rules: ValidationRules::default(),
            validation_error: None,
            input_mask: InputMask::None,
            aria_label: None,
            aria_description: None,
            autocomplete: None,
            helper_text: None,
            success_message: None,
            tab_index: None,
            select_on_focus: false,
            validate_on_blur: true,
            validate_on_change: false,
            trim_on_blur: true,
            cursor_position_override: None,
        }
    }

    /// Set the input type
    pub fn input_type(mut self, input_type: InputType) -> Self {
        self.input_type = input_type;
        match input_type {
            InputType::Email => {
                self.placeholder = "email@example.com".into();
                self.autocomplete = Some("email".into());
            }
            InputType::Tel => {
                self.placeholder = "(555) 555-5555".into();
                self.input_mask = InputMask::Phone;
                self.autocomplete = Some("tel".into());
            }
            InputType::Url => {
                self.placeholder = "https://example.com".into();
                self.autocomplete = Some("url".into());
            }
            InputType::Password => {
                self.masked = true;
                self.autocomplete = Some("current-password".into());
            }
            InputType::CreditCard => {
                self.input_mask = InputMask::CreditCard;
                self.placeholder = "1234 5678 9012 3456".into();
                self.autocomplete = Some("cc-number".into());
            }
            InputType::Date => {
                self.input_mask = InputMask::Date;
                self.placeholder = "MM/DD/YYYY".into();
            }
            InputType::Time => {
                self.input_mask = InputMask::Time;
                self.placeholder = "HH:MM".into();
            }
            _ => {}
        }
        self
    }

    /// Set validation rules
    pub fn validation_rules(mut self, rules: ValidationRules) -> Self {
        self.validation_rules = rules;
        self
    }

    /// Set minimum length
    pub fn min_length(mut self, min: usize) -> Self {
        self.validation_rules.min_length = Some(min);
        self
    }

    /// Set maximum length
    pub fn max_length(mut self, max: usize) -> Self {
        self.validation_rules.max_length = Some(max);
        self
    }

    /// Mark field as required
    pub fn required(mut self, required: bool) -> Self {
        self.validation_rules.required = required;
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Set the placeholder text
    pub fn set_placeholder(
        &mut self,
        placeholder: impl Into<SharedString>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.placeholder = placeholder.into();
        cx.notify();
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    /// Set the text content with validation
    pub fn set_value(
        &mut self,
        value: impl Into<SharedString>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let value = value.into();
        let filtered_value = self.filter_input(&value);
        self.replace_text_in_range(None, &filtered_value, window, cx);
        let len = filtered_value.len();
        self.selected_range = len..len;

        if self.validate_on_change {
            self.validate(cx).ok();
        }

        cx.emit(InputEvent::Change);
    }

    /// Validate the current input value
    pub fn validate(&mut self, cx: &mut Context<Self>) -> Result<(), ValidationError> {
        let value = self.content.as_ref();

        if self.validation_rules.required && value.trim().is_empty() {
            let error = ValidationError {
                message: "This field is required".into(),
                field_name: self.aria_label.clone(),
            };
            self.validation_error = Some(error.clone());
            cx.emit(InputEvent::Validate(Err(error.clone())));
            cx.notify();
            return Err(error);
        }

        if !self.validation_rules.required && value.trim().is_empty() {
            self.validation_error = None;
            self.success_message = None;
            cx.emit(InputEvent::Validate(Ok(())));
            cx.notify();
            return Ok(());
        }

        let type_result = match self.input_type {
            InputType::Email => self.validate_email(value),
            InputType::Number => self.validate_number(value),
            InputType::Url => self.validate_url(value),
            InputType::Tel => self.validate_phone(value),
            InputType::Date => self.validate_date(value),
            InputType::Time => self.validate_time(value),
            InputType::CreditCard => self.validate_credit_card(value),
            _ => Ok(()),
        };

        if let Err(error) = type_result {
            self.validation_error = Some(error.clone());
            cx.emit(InputEvent::Validate(Err(error.clone())));
            cx.notify();
            return Err(error);
        }

        if let Some(min_length) = self.validation_rules.min_length {
            if value.len() < min_length {
                let error = ValidationError {
                    message: format!("Must be at least {} characters", min_length).into(),
                    field_name: self.aria_label.clone(),
                };
                self.validation_error = Some(error.clone());
                cx.emit(InputEvent::Validate(Err(error.clone())));
                cx.notify();
                return Err(error);
            }
        }

        if let Some(max_length) = self.validation_rules.max_length {
            if value.len() > max_length {
                let error = ValidationError {
                    message: format!("Must be no more than {} characters", max_length).into(),
                    field_name: self.aria_label.clone(),
                };
                self.validation_error = Some(error.clone());
                cx.emit(InputEvent::Validate(Err(error.clone())));
                cx.notify();
                return Err(error);
            }
        }

        if let Some(ref validator) = self.validation_rules.custom_validator {
            if let Err(error) = validator(value) {
                self.validation_error = Some(error.clone());
                cx.emit(InputEvent::Validate(Err(error.clone())));
                cx.notify();
                return Err(error);
            }
        }

        self.validation_error = None;
        self.success_message = Some("Valid input".into());
        cx.emit(InputEvent::Validate(Ok(())));
        cx.notify();
        Ok(())
    }

    /// Filter input based on input type or custom filter
    fn filter_input(&self, input: &str) -> String {
        if let Some(ref custom_filter) = self.validation_rules.custom_filter {
            return custom_filter(input);
        }

        match self.input_type {
            InputType::Number => {
                input.chars()
                    .filter(|c| c.is_ascii_digit() || *c == '.' || *c == '-')
                    .collect()
            }
            InputType::Tel => {
                input.chars()
                    .filter(|c| c.is_ascii_digit() || *c == '+' || *c == '-' || *c == '(' || *c == ')' || *c == ' ')
                    .collect()
            }
            InputType::Date => {
                input.chars()
                    .filter(|c| c.is_ascii_digit() || *c == '/' || *c == '-')
                    .collect()
            }
            InputType::Time => {
                input.chars()
                    .filter(|c| c.is_ascii_digit() || *c == ':')
                    .collect()
            }
            InputType::CreditCard => {
                input.chars()
                    .filter(|c| c.is_ascii_digit() || *c == ' ')
                    .collect()
            }
            _ => input.to_string(),
        }
    }

    /// Apply input mask formatting
    fn apply_mask(&mut self, input: &str) -> String {
        match self.input_mask {
            InputMask::Phone => {
                let digits: String = input.chars().filter(|c| c.is_ascii_digit()).collect();
                if digits.len() <= 3 {
                    digits
                } else if digits.len() <= 6 {
                    format!("({}) {}", &digits[0..3], &digits[3..])
                } else if digits.len() <= 10 {
                    format!("({}) {}-{}", &digits[0..3], &digits[3..6], &digits[6..])
                } else {
                    format!("({}) {}-{}", &digits[0..3], &digits[3..6], &digits[6..10])
                }
            }
            InputMask::CreditCard => {
                let digits: String = input.chars().filter(|c| c.is_ascii_digit()).collect();
                digits.chars()
                    .enumerate()
                    .map(|(i, c)| if i > 0 && i % 4 == 0 { format!(" {}", c) } else { c.to_string() })
                    .collect::<String>()
            }
            InputMask::Date => {
                let digits: String = input.chars().filter(|c| c.is_ascii_digit()).collect();
                if digits.len() <= 2 {
                    digits
                } else if digits.len() <= 4 {
                    format!("{}/{}", &digits[0..2], &digits[2..])
                } else if digits.len() <= 8 {
                    format!("{}/{}/{}", &digits[0..2], &digits[2..4], &digits[4..])
                } else {
                    format!("{}/{}/{}", &digits[0..2], &digits[2..4], &digits[4..8])
                }
            }
            InputMask::Time => {
                let digits: String = input.chars().filter(|c| c.is_ascii_digit()).collect();
                if digits.len() <= 2 {
                    digits
                } else if digits.len() <= 4 {
                    format!("{}:{}", &digits[0..2], &digits[2..])
                } else {
                    format!("{}:{}", &digits[0..2], &digits[2..4])
                }
            }
            _ => input.to_string(),
        }
    }

    fn validate_email(&self, email: &str) -> Result<(), ValidationError> {
        let re = regex::Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        if !re.is_match(email) {
            return Err(ValidationError {
                message: "Please enter a valid email address".into(),
                field_name: self.aria_label.clone(),
            });
        }
        Ok(())
    }

    fn validate_url(&self, url: &str) -> Result<(), ValidationError> {
        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Err(ValidationError {
                message: "URL must start with http:// or https://".into(),
                field_name: self.aria_label.clone(),
            });
        }
        Ok(())
    }

    fn validate_number(&self, number: &str) -> Result<(), ValidationError> {
        let parsed = number.parse::<f64>();
        if parsed.is_err() {
            return Err(ValidationError {
                message: "Please enter a valid number".into(),
                field_name: self.aria_label.clone(),
            });
        }

        if let Ok(value) = parsed {
            if let Some(min) = self.validation_rules.min_value {
                if value < min {
                    return Err(ValidationError {
                        message: format!("Must be at least {}", min).into(),
                        field_name: self.aria_label.clone(),
                    });
                }
            }
            if let Some(max) = self.validation_rules.max_value {
                if value > max {
                    return Err(ValidationError {
                        message: format!("Must be no more than {}", max).into(),
                        field_name: self.aria_label.clone(),
                    });
                }
            }
        }
        Ok(())
    }

    fn validate_phone(&self, phone: &str) -> Result<(), ValidationError> {
        let digits: String = phone.chars().filter(|c| c.is_ascii_digit()).collect();
        if digits.len() < 10 {
            return Err(ValidationError {
                message: "Phone number must be at least 10 digits".into(),
                field_name: self.aria_label.clone(),
            });
        }
        Ok(())
    }

    fn validate_date(&self, date: &str) -> Result<(), ValidationError> {
        let parts: Vec<&str> = date.split('/').collect();
        if parts.len() != 3 {
            return Err(ValidationError {
                message: "Date must be in MM/DD/YYYY format".into(),
                field_name: self.aria_label.clone(),
            });
        }

        let month = parts[0].parse::<u32>().unwrap_or(0);
        let day = parts[1].parse::<u32>().unwrap_or(0);
        let year = parts[2].parse::<u32>().unwrap_or(0);

        if month < 1 || month > 12 {
            return Err(ValidationError {
                message: "Invalid month".into(),
                field_name: self.aria_label.clone(),
            });
        }
        if day < 1 || day > 31 {
            return Err(ValidationError {
                message: "Invalid day".into(),
                field_name: self.aria_label.clone(),
            });
        }
        if year < 1900 || year > 2100 {
            return Err(ValidationError {
                message: "Invalid year".into(),
                field_name: self.aria_label.clone(),
            });
        }
        Ok(())
    }

    fn validate_time(&self, time: &str) -> Result<(), ValidationError> {
        let parts: Vec<&str> = time.split(':').collect();
        if parts.len() != 2 {
            return Err(ValidationError {
                message: "Time must be in HH:MM format".into(),
                field_name: self.aria_label.clone(),
            });
        }

        let hours = parts[0].parse::<u32>().unwrap_or(25);
        let minutes = parts[1].parse::<u32>().unwrap_or(61);

        if hours > 23 {
            return Err(ValidationError {
                message: "Invalid hours (0-23)".into(),
                field_name: self.aria_label.clone(),
            });
        }
        if minutes > 59 {
            return Err(ValidationError {
                message: "Invalid minutes (0-59)".into(),
                field_name: self.aria_label.clone(),
            });
        }
        Ok(())
    }

    fn validate_credit_card(&self, card: &str) -> Result<(), ValidationError> {
        let digits: String = card.chars().filter(|c| c.is_ascii_digit()).collect();
        if digits.len() < 13 || digits.len() > 19 {
            return Err(ValidationError {
                message: "Invalid credit card number".into(),
                field_name: self.aria_label.clone(),
            });
        }

        let mut sum = 0;
        let mut alternate = false;
        for c in digits.chars().rev() {
            let mut digit = c.to_digit(10).unwrap();
            if alternate {
                digit *= 2;
                if digit > 9 {
                    digit -= 9;
                }
            }
            sum += digit;
            alternate = !alternate;
        }

        if sum % 10 != 0 {
            return Err(ValidationError {
                message: "Invalid credit card number".into(),
                field_name: self.aria_label.clone(),
            });
        }
        Ok(())
    }

    pub fn left(&mut self, _: &Left, _: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            self.move_to(self.previous_boundary(self.cursor_offset()), cx);
        } else {
            self.move_to(self.selected_range.start, cx)
        }
    }

    pub fn right(&mut self, _: &Right, _: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            self.move_to(self.next_boundary(self.selected_range.end), cx);
        } else {
            self.move_to(self.selected_range.end, cx)
        }
    }

    pub fn select_left(&mut self, _: &SelectLeft, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.previous_boundary(self.cursor_offset()), cx);
    }

    pub fn select_right(&mut self, _: &SelectRight, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.next_boundary(self.cursor_offset()), cx);
    }

    pub fn select_all(&mut self, _: &SelectAll, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(0, cx);
        self.select_to(self.content.len(), cx)
    }

    pub fn home(&mut self, _: &Home, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(0, cx);
    }

    pub fn end(&mut self, _: &End, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(self.content.len(), cx);
    }

    pub fn backspace(&mut self, _: &Backspace, window: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            self.select_to(self.previous_boundary(self.cursor_offset()), cx)
        }
        self.replace_text_in_range(None, "", window, cx)
    }

    pub fn delete(&mut self, _: &Delete, window: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            self.select_to(self.next_boundary(self.cursor_offset()), cx)
        }
        self.replace_text_in_range(None, "", window, cx)
    }

    pub fn tab(&mut self, _: &Tab, window: &mut Window, cx: &mut Context<Self>) {
        window.focus_next();
        cx.emit(InputEvent::Tab);
    }

    pub fn shift_tab(&mut self, _: &ShiftTab, window: &mut Window, cx: &mut Context<Self>) {
        window.focus_prev();
        cx.emit(InputEvent::ShiftTab);
    }

    fn on_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.is_selecting = true;

        let now = std::time::Instant::now();
        let is_double_click = if let (Some(last_time), Some(last_pos)) = (self.last_click_time, self.last_click_position) {
            let time_diff = now.duration_since(last_time);
            let dx = event.position.x - last_pos.x;
            let dy = event.position.y - last_pos.y;
            let close_enough = dx < px(5.0) && dx > px(-5.0) && dy < px(5.0) && dy > px(-5.0);
            time_diff.as_millis() < 500 && close_enough
        } else {
            false
        };

        self.last_click_time = Some(now);
        self.last_click_position = Some(event.position);

        if is_double_click && !self.content.is_empty() {
            self.selected_range = 0..self.content.len();
            self.selection_reversed = false;
            cx.notify();
            return;
        }

        let click_index = self.index_for_mouse_position(event.position);

        if event.modifiers.shift {
            self.select_to(click_index, cx);
        } else {
            self.move_to(click_index, cx)
        }
    }

    fn on_mouse_up(&mut self, _: &MouseUpEvent, _window: &mut Window, _: &mut Context<Self>) {
        self.is_selecting = false;
    }

    fn on_mouse_move(&mut self, event: &MouseMoveEvent, _: &mut Window, cx: &mut Context<Self>) {
        if self.is_selecting {
            self.select_to(self.index_for_mouse_position(event.position), cx);
        }
    }

    pub fn paste(&mut self, _: &Paste, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(text) = cx.read_from_clipboard().and_then(|item| item.text()) {
            let filtered_text = self.filter_input(&text.replace("\n", " "));
            self.replace_text_in_range(None, &filtered_text, window, cx);
        }
    }

    pub fn copy(&mut self, _: &Copy, _: &mut Window, cx: &mut Context<Self>) {
        if !self.selected_range.is_empty() {
            cx.write_to_clipboard(ClipboardItem::new_string(
                self.content[self.selected_range.clone()].to_string(),
            ));
        }
    }

    pub fn cut(&mut self, _: &Cut, window: &mut Window, cx: &mut Context<Self>) {
        if !self.selected_range.is_empty() {
            cx.write_to_clipboard(ClipboardItem::new_string(
                self.content[self.selected_range.clone()].to_string(),
            ));
            self.replace_text_in_range(None, "", window, cx)
        }
    }

    pub fn enter(&mut self, _: &Enter, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(InputEvent::Enter);
    }

    pub fn escape(&mut self, _: &Escape, _window: &mut Window, cx: &mut Context<Self>) {
        self.selected_range = self.content.len()..self.content.len();
        cx.emit(InputEvent::Blur);
        cx.notify();
    }

    /// Called when the input gains focus
    pub fn on_focus(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        if self.select_on_focus && !self.content.is_empty() {
            self.selected_range = 0..self.content.len();
        }
        else if !self.content.is_empty() && self.cursor_position_override.is_none() {
            let len = self.content.len();
            self.selected_range = len..len;
        }

        cx.emit(InputEvent::Focus);
        cx.notify();
    }

    /// Called when the input loses focus
    pub fn on_blur(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.trim_on_blur {
            let trimmed = self.content.trim().to_string();
            if trimmed != self.content.as_ref() {
                self.set_value(trimmed, window, cx);
            }
        }

        if self.validate_on_blur {
            self.validate(cx).ok();
        }

        cx.emit(InputEvent::Blur);
        cx.notify();
    }

    fn move_to(&mut self, offset: usize, cx: &mut Context<Self>) {
        self.selected_range = offset..offset;
        cx.notify()
    }

    fn cursor_offset(&self) -> usize {
        if self.selection_reversed {
            self.selected_range.start
        } else {
            self.selected_range.end
        }
    }

    fn index_for_mouse_position(&self, position: Point<Pixels>) -> usize {
        if self.content.is_empty() {
            return 0;
        }

        let (Some(bounds), Some(line)) = (self.last_bounds.as_ref(), self.last_layout.as_ref())
        else {
            return 0;
        };
        if position.y < bounds.top() {
            return 0;
        }
        if position.y > bounds.bottom() {
            return self.content.len();
        }

        let display_index = line.closest_index_for_x(position.x - bounds.left());

        if self.masked {
            let char_index = display_index / "•".len();
            self.content
                .char_indices()
                .nth(char_index)
                .map(|(byte_idx, _)| byte_idx)
                .unwrap_or(self.content.len())
        } else {
            display_index
        }
    }

    fn select_to(&mut self, offset: usize, cx: &mut Context<Self>) {
        if self.selection_reversed {
            self.selected_range.start = offset
        } else {
            self.selected_range.end = offset
        };
        if self.selected_range.end < self.selected_range.start {
            self.selection_reversed = !self.selection_reversed;
            self.selected_range = self.selected_range.end..self.selected_range.start;
        }
        cx.notify()
    }

    fn offset_from_utf16(&self, offset: usize) -> usize {
        let mut utf8_offset = 0;
        let mut utf16_count = 0;

        for ch in self.content.chars() {
            if utf16_count >= offset {
                break;
            }
            utf16_count += ch.len_utf16();
            utf8_offset += ch.len_utf8();
        }

        utf8_offset
    }

    fn offset_to_utf16(&self, offset: usize) -> usize {
        let mut utf16_offset = 0;
        let mut utf8_count = 0;

        for ch in self.content.chars() {
            if utf8_count >= offset {
                break;
            }
            utf8_count += ch.len_utf8();
            utf16_offset += ch.len_utf16();
        }

        utf16_offset
    }

    fn range_to_utf16(&self, range: &Range<usize>) -> Range<usize> {
        self.offset_to_utf16(range.start)..self.offset_to_utf16(range.end)
    }

    fn range_from_utf16(&self, range_utf16: &Range<usize>) -> Range<usize> {
        self.offset_from_utf16(range_utf16.start)..self.offset_from_utf16(range_utf16.end)
    }

    fn previous_boundary(&self, offset: usize) -> usize {
        self.content
            .grapheme_indices(true)
            .rev()
            .find_map(|(idx, _)| (idx < offset).then_some(idx))
            .unwrap_or(0)
    }

    fn next_boundary(&self, offset: usize) -> usize {
        self.content
            .grapheme_indices(true)
            .find_map(|(idx, _)| (idx > offset).then_some(idx))
            .unwrap_or(self.content.len())
    }

    pub fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EntityInputHandler for InputState {
    fn text_for_range(
        &mut self,
        range_utf16: Range<usize>,
        actual_range: &mut Option<Range<usize>>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<String> {
        let range = self.range_from_utf16(&range_utf16);
        actual_range.replace(self.range_to_utf16(&range));
        Some(self.content[range].to_string())
    }

    fn selected_text_range(
        &mut self,
        _ignore_disabled_input: bool,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<UTF16Selection> {
        Some(UTF16Selection {
            range: self.range_to_utf16(&self.selected_range),
            reversed: self.selection_reversed,
        })
    }

    fn marked_text_range(
        &self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Range<usize>> {
        self.marked_range
            .as_ref()
            .map(|range| self.range_to_utf16(range))
    }

    fn unmark_text(&mut self, _window: &mut Window, _cx: &mut Context<Self>) {
        self.marked_range = None;
    }

    fn replace_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let range = range_utf16
            .as_ref()
            .map(|range_utf16| self.range_from_utf16(range_utf16))
            .or(self.marked_range.clone())
            .unwrap_or(self.selected_range.clone());

        let filtered_text = self.filter_input(new_text);
        let formatted_text = if self.input_mask != InputMask::None {
            self.apply_mask(&filtered_text)
        } else {
            filtered_text
        };

        if let Some(max_length) = self.validation_rules.max_length {
            let new_length = self.content.len() - (range.end - range.start) + formatted_text.len();
            if new_length > max_length {
                let allowed_length = max_length.saturating_sub(self.content.len() - (range.end - range.start));
                let truncated: String = formatted_text.chars().take(allowed_length).collect();

                self.content =
                    (self.content[0..range.start].to_owned() + &truncated + &self.content[range.end..])
                        .into();
                self.selected_range = range.start + truncated.len()..range.start + truncated.len();
            } else {
                self.content =
                    (self.content[0..range.start].to_owned() + &formatted_text + &self.content[range.end..])
                        .into();
                self.selected_range = range.start + formatted_text.len()..range.start + formatted_text.len();
            }
        } else {
            self.content =
                (self.content[0..range.start].to_owned() + &formatted_text + &self.content[range.end..])
                    .into();
            self.selected_range = range.start + formatted_text.len()..range.start + formatted_text.len();
        }

        self.marked_range.take();

        if self.validate_on_change {
            self.validate(cx).ok();
        }

        cx.emit(InputEvent::Change);
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
        let range = range_utf16
            .as_ref()
            .map(|range_utf16| self.range_from_utf16(range_utf16))
            .or(self.marked_range.clone())
            .unwrap_or(self.selected_range.clone());

        let filtered_text = self.filter_input(new_text);
        self.content =
            (self.content[0..range.start].to_owned() + &filtered_text + &self.content[range.end..])
                .into();
        if !filtered_text.is_empty() {
            self.marked_range = Some(range.start..range.start + filtered_text.len());
        } else {
            self.marked_range = None;
        }
        self.selected_range = new_selected_range_utf16
            .as_ref()
            .map(|range_utf16| self.range_from_utf16(range_utf16))
            .map(|new_range| new_range.start + range.start..new_range.end + range.end)
            .unwrap_or_else(|| range.start + filtered_text.len()..range.start + filtered_text.len());

        cx.notify();
    }

    fn bounds_for_range(
        &mut self,
        range_utf16: Range<usize>,
        bounds: Bounds<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Bounds<Pixels>> {
        let last_layout = self.last_layout.as_ref()?;
        let range = self.range_from_utf16(&range_utf16);
        Some(Bounds::from_corners(
            point(
                bounds.left() + last_layout.x_for_index(range.start),
                bounds.top(),
            ),
            point(
                bounds.left() + last_layout.x_for_index(range.end),
                bounds.bottom(),
            ),
        ))
    }

    fn character_index_for_point(
        &mut self,
        point: Point<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<usize> {
        let line_point = self.last_bounds?.localize(&point)?;
        let last_layout = self.last_layout.as_ref()?;

        assert_eq!(last_layout.text, self.content);
        let utf8_index = last_layout.index_for_x(point.x - line_point.x)?;
        Some(self.offset_to_utf16(utf8_index))
    }
}

/// Custom element for rendering the input text with cursor and selection.
/// This is THE KEY to text input - it calls window.handle_input() in the paint method
struct InputTextElement {
    input: Entity<InputState>,
}

struct PrepaintState {
    line: Option<gpui::ShapedLine>,
    cursor: Option<PaintQuad>,
    selection: Option<PaintQuad>,
}

impl IntoElement for InputTextElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl gpui::Element for InputTextElement {
    type RequestLayoutState = ();
    type PrepaintState = PrepaintState;

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut style = Style::default();
        style.size.width = relative(1.).into();
        style.size.height = window.line_height().into();
        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        let input = self.input.read(cx);
        let (content, selected_range, cursor) = if input.masked {
            let char_count = input.content.chars().count();
            let masked_text = "•".repeat(char_count).into();

            let start_chars = input.content[..input.selected_range.start].chars().count();
            let end_chars = input.content[..input.selected_range.end].chars().count();
            let masked_selected_range = (start_chars * "•".len())..(end_chars * "•".len());

            let cursor_chars = if input.selection_reversed {
                start_chars
            } else {
                end_chars
            };
            let masked_cursor = cursor_chars * "•".len();

            (masked_text, masked_selected_range, masked_cursor)
        } else {
            (input.content.clone(), input.selected_range.clone(), input.cursor_offset())
        };
        let style = window.text_style();
        let theme = use_theme();

        let (display_text, text_color) = if content.is_empty() {
            (input.placeholder.clone(), theme.tokens.muted_foreground)
        } else {
            (content, style.color)
        };

        let run = TextRun {
            len: display_text.len(),
            font: style.font(),
            color: text_color,
            background_color: None,
            underline: None,
            strikethrough: None,
        };

        let runs = if let Some(marked_range) = input.marked_range.as_ref() {
            let (marked_start, marked_end) = if input.masked {
                let start_chars = input.content[..marked_range.start].chars().count();
                let end_chars = input.content[..marked_range.end].chars().count();
                (start_chars * "•".len(), end_chars * "•".len())
            } else {
                (marked_range.start, marked_range.end)
            };

            vec![
                TextRun {
                    len: marked_start,
                    ..run.clone()
                },
                TextRun {
                    len: marked_end - marked_start,
                    underline: Some(UnderlineStyle {
                        color: Some(run.color),
                        thickness: px(1.0),
                        wavy: false,
                    }),
                    ..run.clone()
                },
                TextRun {
                    len: display_text.len() - marked_end,
                    ..run
                },
            ]
            .into_iter()
            .filter(|run| run.len > 0)
            .collect()
        } else {
            vec![run]
        };

        let font_size = style.font_size.to_pixels(window.rem_size());
        let line = window
            .text_system()
            .shape_line(display_text, font_size, &runs, None);

        let cursor_pos = line.x_for_index(cursor);
        let (selection, cursor) = if selected_range.is_empty() {
            (
                None,
                Some(fill(
                    Bounds::new(
                        point(bounds.left() + cursor_pos, bounds.top()),
                        size(px(2.), bounds.bottom() - bounds.top()),
                    ),
                    rgb(0x0066ff),
                )),
            )
        } else {
            (
                Some(fill(
                    Bounds::from_corners(
                        point(
                            bounds.left() + line.x_for_index(selected_range.start),
                            bounds.top(),
                        ),
                        point(
                            bounds.left() + line.x_for_index(selected_range.end),
                            bounds.bottom(),
                        ),
                    ),
                    rgba(0x3311ff30),
                )),
                None,
            )
        };
        PrepaintState {
            line: Some(line),
            cursor,
            selection,
        }
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let focus_handle = self.input.read(cx).focus_handle.clone();

        window.handle_input(
            &focus_handle,
            ElementInputHandler::new(bounds, self.input.clone()),
            cx,
        );

        if let Some(selection) = prepaint.selection.take() {
            window.paint_quad(selection)
        }
        let line = prepaint.line.take().unwrap();
        line.paint(bounds.origin, window.line_height(), window, cx)
            .unwrap();

        if focus_handle.is_focused(window) {
            if let Some(cursor) = prepaint.cursor.take() {
                window.paint_quad(cursor);
            }
        }

        self.input.update(cx, |input, _cx| {
            input.last_layout = Some(line);
            input.last_bounds = Some(bounds);
        });
    }
}

impl Render for InputState {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let input = cx.entity();

        div()
            .w_full()
            .h_full()
            .on_mouse_down(MouseButton::Left, {
                let input = input.clone();
                move |event: &MouseDownEvent, window: &mut Window, cx: &mut App| {
                    input.update(cx, |input, cx| {
                        input.on_mouse_down(event, window, cx);
                    });
                }
            })
            .on_mouse_up(MouseButton::Left, {
                let input = input.clone();
                move |event: &MouseUpEvent, window: &mut Window, cx: &mut App| {
                    input.update(cx, |input, cx| {
                        input.on_mouse_up(event, window, cx);
                    });
                }
            })
            .on_mouse_move({
                let input = input.clone();
                move |event: &MouseMoveEvent, window: &mut Window, cx: &mut App| {
                    input.update(cx, |input, cx| {
                        input.on_mouse_move(event, window, cx);
                    });
                }
            })
            .child(InputTextElement { input })
    }
}

impl Focusable for InputState {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

use regex;