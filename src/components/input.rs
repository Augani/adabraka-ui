//! Input component - Advanced text input with validation, masking, and accessibility.

use gpui::{prelude::FluentBuilder as _, *};
use std::rc::Rc;
use std::sync::Arc;
use crate::theme::use_theme;
use crate::layout::{HStack, VStack};
use crate::components::icon::Icon;
pub use crate::components::input_state::{
    InputState, InputEvent, InputType, ValidationError, ValidationRules, InputMask,
    Backspace, Delete, Left, Right, SelectLeft, SelectRight, SelectAll, Home, End, Copy, Cut, Paste, Enter, Tab, ShiftTab, Escape
};

pub fn init(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("backspace", Backspace, Some("Input")),
        KeyBinding::new("delete", Delete, Some("Input")),
        KeyBinding::new("left", Left, Some("Input")),
        KeyBinding::new("right", Right, Some("Input")),
        KeyBinding::new("shift-left", SelectLeft, Some("Input")),
        KeyBinding::new("shift-right", SelectRight, Some("Input")),
        KeyBinding::new("home", Home, Some("Input")),
        KeyBinding::new("end", End, Some("Input")),
        KeyBinding::new("enter", Enter, Some("Input")),
        KeyBinding::new("tab", Tab, Some("Input")),
        KeyBinding::new("shift-tab", ShiftTab, Some("Input")),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-a", SelectAll, Some("Input")),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-a", SelectAll, Some("Input")),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-c", Copy, Some("Input")),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-c", Copy, Some("Input")),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-x", Cut, Some("Input")),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-x", Cut, Some("Input")),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-v", Paste, Some("Input")),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-v", Paste, Some("Input")),
        KeyBinding::new("escape", Escape, Some("Input")),
    ]);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputVariant {
    Default,
    Outline,
    Ghost,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputSize {
    Sm,
    Md,
    Lg,
}

impl Default for InputSize {
    fn default() -> Self {
        Self::Md
    }
}

#[derive(IntoElement)]
pub struct Input {
    state: Entity<InputState>,
    placeholder: SharedString,
    variant: InputVariant,
    size: InputSize,
    disabled: bool,
    error: bool,
    password: bool,
    clearable: bool,
    prefix: Option<AnyElement>,
    suffix: Option<AnyElement>,
    initial_value: Option<SharedString>,

    // Enhanced features
    input_type: Option<InputType>,
    validation_rules: Option<ValidationRules>,
    helper_text: Option<SharedString>,
    show_character_count: bool,
    aria_label: Option<SharedString>,
    aria_description: Option<SharedString>,
    autocomplete: Option<SharedString>,
    required: bool,

    // Custom functions for extensibility
    custom_validator: Option<Rc<dyn Fn(&str) -> Result<(), String>>>,
    custom_filter: Option<Rc<dyn Fn(&str) -> String>>,
    custom_formatter: Option<Rc<dyn Fn(&str) -> String>>,

    // Callbacks
    on_change: Option<Rc<dyn Fn(SharedString, &mut App)>>,
    on_enter: Option<Rc<dyn Fn(SharedString, &mut App)>>,
    on_focus: Option<Rc<dyn Fn(SharedString, &mut App)>>,
    on_blur: Option<Rc<dyn Fn(SharedString, &mut App)>>,
    on_validate: Option<Rc<dyn Fn(Result<(), ValidationError>, &mut App)>>,

    // Style refinement for Styled trait
    style: StyleRefinement,
}

impl Input {
    /// Create a new input with an InputState entity
    pub fn new(state: &Entity<InputState>) -> Self {
        Self {
            state: state.clone(),
            placeholder: "".into(),
            variant: InputVariant::Default,
            size: InputSize::default(),
            disabled: false,
            error: false,
            password: false,
            clearable: false,
            prefix: None,
            suffix: None,
            initial_value: None,

            // Enhanced features
            input_type: None,
            validation_rules: None,
            helper_text: None,
            show_character_count: false,
            aria_label: None,
            aria_description: None,
            autocomplete: None,
            required: false,

            // Custom functions
            custom_validator: None,
            custom_filter: None,
            custom_formatter: None,

            // Callbacks
            on_change: None,
            on_enter: None,
            on_focus: None,
            on_blur: None,
            on_validate: None,

            // Style refinement
            style: StyleRefinement::default(),
        }
    }

    /// Set the initial value (will be set when rendering)
    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.initial_value = Some(value.into());
        self
    }

    /// Set placeholder text
    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn cleanable(mut self) -> Self {
        self.clearable = true;
        self
    }


    /// Set the input variant
    pub fn variant(mut self, variant: InputVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the input size
    pub fn size(mut self, size: InputSize) -> Self {
        self.size = size;
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set error state (shows error styling)
    pub fn error(mut self, error: bool) -> Self {
        self.error = error;
        self
    }

    /// Enable password masking
    pub fn password(mut self, password: bool) -> Self {
        self.password = password;
        self
    }

    /// Enable clear button when input has value
    pub fn clearable(mut self, clearable: bool) -> Self {
        self.clearable = clearable;
        self
    }

    /// Add a prefix element (icon, label, etc.)
    pub fn prefix(mut self, prefix: impl IntoElement) -> Self {
        self.prefix = Some(prefix.into_any_element());
        self
    }

    /// Add a suffix element (icon, label, etc.)
    pub fn suffix(mut self, suffix: impl IntoElement) -> Self {
        self.suffix = Some(suffix.into_any_element());
        self
    }

    /// Set callback when value changes
    pub fn on_change<F>(mut self, callback: F) -> Self
    where
        F: Fn(SharedString, &mut App) + 'static,
    {
        self.on_change = Some(Rc::new(callback));
        self
    }

    /// Set callback when Enter key is pressed
    pub fn on_enter<F>(mut self, callback: F) -> Self
    where
        F: Fn(SharedString, &mut App) + 'static,
    {
        self.on_enter = Some(Rc::new(callback));
        self
    }

    /// Set callback when input gains focus
    pub fn on_focus<F>(mut self, callback: F) -> Self
    where
        F: Fn(SharedString, &mut App) + 'static,
    {
        self.on_focus = Some(Rc::new(callback));
        self
    }

    /// Set callback when input loses focus
    pub fn on_blur<F>(mut self, callback: F) -> Self
    where
        F: Fn(SharedString, &mut App) + 'static,
    {
        self.on_blur = Some(Rc::new(callback));
        self
    }

    /// Set the input type (email, number, tel, etc.)
    pub fn input_type(mut self, input_type: InputType) -> Self {
        self.input_type = Some(input_type);
        self
    }

    /// Set validation rules
    pub fn validation_rules(mut self, rules: ValidationRules) -> Self {
        self.validation_rules = Some(rules);
        self
    }

    /// Set minimum length requirement
    pub fn min_length(mut self, min: usize) -> Self {
        if self.validation_rules.is_none() {
            self.validation_rules = Some(ValidationRules::default());
        }
        if let Some(ref mut rules) = self.validation_rules {
            rules.min_length = Some(min);
        }
        self
    }

    /// Set maximum length requirement
    pub fn max_length(mut self, max: usize) -> Self {
        if self.validation_rules.is_none() {
            self.validation_rules = Some(ValidationRules::default());
        }
        if let Some(ref mut rules) = self.validation_rules {
            rules.max_length = Some(max);
        }
        self
    }

    /// Mark field as required
    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        if self.validation_rules.is_none() {
            self.validation_rules = Some(ValidationRules::default());
        }
        if let Some(ref mut rules) = self.validation_rules {
            rules.required = required;
        }
        self
    }

    /// Set helper text displayed below the input
    pub fn helper_text(mut self, text: impl Into<SharedString>) -> Self {
        self.helper_text = Some(text.into());
        self
    }

    /// Show character count indicator
    pub fn show_character_count(mut self, show: bool) -> Self {
        self.show_character_count = show;
        self
    }

    /// Set ARIA label for accessibility
    pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self {
        self.aria_label = Some(label.into());
        self
    }

    /// Set ARIA description for accessibility
    pub fn aria_description(mut self, description: impl Into<SharedString>) -> Self {
        self.aria_description = Some(description.into());
        self
    }

    /// Set autocomplete attribute
    pub fn autocomplete(mut self, autocomplete: impl Into<SharedString>) -> Self {
        self.autocomplete = Some(autocomplete.into());
        self
    }

    /// Set callback for validation events
    pub fn on_validate<F>(mut self, callback: F) -> Self
    where
        F: Fn(Result<(), ValidationError>, &mut App) + 'static,
    {
        self.on_validate = Some(Rc::new(callback));
        self
    }

    /// Set a custom validation function
    ///
    /// # Example
    /// ```rust
    /// Input::new(&state)
    ///     .custom_validator(|value| {
    ///         if value.contains("@company.com") {
    ///             Ok(())
    ///         } else {
    ///             Err("Must be a company email".to_string())
    ///         }
    ///     })
    /// ```
    pub fn custom_validator<F>(mut self, validator: F) -> Self
    where
        F: Fn(&str) -> Result<(), String> + 'static,
    {
        self.custom_validator = Some(Rc::new(validator));
        self
    }

    /// Set a custom filter function to control which characters are allowed
    ///
    /// # Example
    /// ```rust
    /// Input::new(&state)
    ///     .custom_filter(|input| {
    ///         // Only allow alphanumeric and underscores
    ///         input.chars()
    ///             .filter(|c| c.is_alphanumeric() || *c == '_')
    ///             .collect()
    ///     })
    /// ```
    pub fn custom_filter<F>(mut self, filter: F) -> Self
    where
        F: Fn(&str) -> String + 'static,
    {
        self.custom_filter = Some(Rc::new(filter));
        self
    }

    /// Set a custom formatter function to format the input value
    ///
    /// # Example
    /// ```rust
    /// Input::new(&state)
    ///     .custom_formatter(|input| {
    ///         // Format as currency
    ///         format!("${:.2}", input.parse::<f64>().unwrap_or(0.0))
    ///     })
    /// ```
    pub fn custom_formatter<F>(mut self, formatter: F) -> Self
    where
        F: Fn(&str) -> String + 'static,
    {
        self.custom_formatter = Some(Rc::new(formatter));
        self
    }

    /// Get height based on size
    fn height(&self) -> Pixels {
        match self.size {
            InputSize::Sm => px(32.0),
            InputSize::Md => px(40.0),
            InputSize::Lg => px(48.0),
        }
    }

    /// Get horizontal padding based on size
    fn padding_x(&self) -> Pixels {
        match self.size {
            InputSize::Sm => px(8.0),
            InputSize::Md => px(12.0),
            InputSize::Lg => px(16.0),
        }
    }

    /// Get font size based on size
    fn font_size(&self) -> Pixels {
        match self.size {
            InputSize::Sm => px(13.0),
            InputSize::Md => px(14.0),
            InputSize::Lg => px(16.0),
        }
    }

    /// Get gap between elements based on size
    fn element_gap(&self) -> Pixels {
        match self.size {
            InputSize::Sm => px(6.0),
            InputSize::Md => px(8.0),
            InputSize::Lg => px(10.0),
        }
    }
}

impl Styled for Input {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Input {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let height = self.height();
        let padding_x = self.padding_x();
        let font_size = self.font_size();
        let gap = self.element_gap();

        self.state.update(cx, |state, cx| {
            state.disabled = self.disabled;
            state.placeholder = self.placeholder.clone();

            // If password flag is enabled, ensure password input type is set.
            // Do not force `masked` here so user interactions can toggle it.
            if self.password {
                state.input_type = InputType::Password;
            }

            if let Some(input_type) = self.input_type {
                state.input_type = input_type;
                match input_type {
                    InputType::Tel => state.input_mask = InputMask::Phone,
                    InputType::CreditCard => state.input_mask = InputMask::CreditCard,
                    InputType::Date => state.input_mask = InputMask::Date,
                    InputType::Time => state.input_mask = InputMask::Time,
                    _ => {}
                }
            }

            if let Some(mut rules) = self.validation_rules.clone() {
                if let Some(ref custom_validator) = self.custom_validator {
                    let validator = custom_validator.clone();
                    rules.custom_validator = Some(Arc::new(move |value| {
                        match validator(value) {
                            Ok(()) => Ok(()),
                            Err(msg) => Err(ValidationError {
                                message: msg.into(),
                                field_name: None,
                            }),
                        }
                    }));
                }

                if let Some(ref custom_filter) = self.custom_filter {
                    let filter = custom_filter.clone();
                    rules.custom_filter = Some(Arc::new(move |input| filter(input)));
                }

                if let Some(ref custom_formatter) = self.custom_formatter {
                    let formatter = custom_formatter.clone();
                    rules.custom_formatter = Some(Arc::new(move |input| formatter(input)));
                }

                state.validation_rules = rules;
            } else if self.custom_validator.is_some() || self.custom_filter.is_some() || self.custom_formatter.is_some() {
                let mut rules = ValidationRules::default();

                if let Some(ref custom_validator) = self.custom_validator {
                    let validator = custom_validator.clone();
                    rules.custom_validator = Some(Arc::new(move |value| {
                        match validator(value) {
                            Ok(()) => Ok(()),
                            Err(msg) => Err(ValidationError {
                                message: msg.into(),
                                field_name: None,
                            }),
                        }
                    }));
                }

                if let Some(ref custom_filter) = self.custom_filter {
                    let filter = custom_filter.clone();
                    rules.custom_filter = Some(Arc::new(move |input| filter(input)));
                }

                if let Some(ref custom_formatter) = self.custom_formatter {
                    let formatter = custom_formatter.clone();
                    rules.custom_formatter = Some(Arc::new(move |input| formatter(input)));
                }

                state.validation_rules = rules;
            }

            state.aria_label = self.aria_label.clone();
            state.aria_description = self.aria_description.clone();
            state.autocomplete = self.autocomplete.clone();
            state.helper_text = self.helper_text.clone();

            if let Some(value) = self.initial_value.clone() {
                state.set_value(value, window, cx);
            }
        });

        let on_change_callback = self.on_change.clone();
        let on_enter_callback = self.on_enter.clone();
        let on_focus_callback = self.on_focus.clone();
        let on_blur_callback = self.on_blur.clone();
        let on_validate_callback = self.on_validate.clone();

        if on_change_callback.is_some() || on_enter_callback.is_some() || on_focus_callback.is_some() || on_blur_callback.is_some() || on_validate_callback.is_some() {
            let state_entity = self.state.clone();
            let state_for_callback = state_entity.clone();
            cx.subscribe(&state_entity, move |_emitter: Entity<InputState>, event: &InputEvent, cx: &mut App| {
                match event {
                    InputEvent::Change => {
                        if let Some(callback) = on_change_callback.as_ref() {
                            let value = state_for_callback.read(cx).content.clone();
                            callback(value, cx);
                        }
                    }
                    InputEvent::Enter => {
                        if let Some(callback) = on_enter_callback.as_ref() {
                            let value = state_for_callback.read(cx).content.clone();
                            callback(value, cx);
                        }
                    }
                    InputEvent::Focus => {
                        if let Some(callback) = on_focus_callback.as_ref() {
                            let value = state_for_callback.read(cx).content.clone();
                            callback(value, cx);
                        }
                    }
                    InputEvent::Blur => {
                        if let Some(callback) = on_blur_callback.as_ref() {
                            let value = state_for_callback.read(cx).content.clone();
                            callback(value, cx);
                        }
                    }
                    InputEvent::Validate(result) => {
                        if let Some(callback) = on_validate_callback.as_ref() {
                            callback(result.clone(), cx);
                        }
                    }
                    InputEvent::Tab => {
                        // Focus navigation handled in InputState action handlers
                    }
                    InputEvent::ShiftTab => {
                        // Focus navigation handled in InputState action handlers
                    }
                }
            })
            .detach();
        }

        let (bg_color, border_color, text_color) = if self.disabled {
            (
                theme.tokens.muted.opacity(0.5),
                theme.tokens.border,
                theme.tokens.muted_foreground,
            )
        } else if self.error {
            match self.variant {
                InputVariant::Default => (
                    theme.tokens.background,
                    theme.tokens.destructive,
                    theme.tokens.foreground,
                ),
                InputVariant::Outline => (
                    theme.tokens.background,
                    theme.tokens.destructive,
                    theme.tokens.foreground,
                ),
                InputVariant::Ghost => (
                    gpui::transparent_black(),
                    theme.tokens.destructive.opacity(0.3),
                    theme.tokens.foreground,
                ),
            }
        } else {
            match self.variant {
                InputVariant::Default => (
                    theme.tokens.background,
                    theme.tokens.input,
                    theme.tokens.foreground,
                ),
                InputVariant::Outline => (
                    theme.tokens.background,
                    theme.tokens.border,
                    theme.tokens.foreground,
                ),
                InputVariant::Ghost => (
                    gpui::transparent_black(),
                    theme.tokens.border.opacity(0.3),
                    theme.tokens.foreground,
                ),
            }
        };

        let has_value = !self.state.read(cx).content.is_empty();
        let show_clear = self.clearable && has_value && !self.disabled;
        let state_for_clear = self.state.clone();
        let state_for_password = self.state.clone();

        let input_state = self.state.read(cx);
        let validation_error = input_state.validation_error.clone();
        let success_message = input_state.success_message.clone();
        let content_length = input_state.content.len();
        let max_length = input_state.validation_rules.max_length;
        let is_focused = input_state.focus_handle(cx).is_focused(window);
        let is_masked = input_state.masked;

        let shadow_xs = BoxShadow {
            offset: theme.tokens.shadow_xs.offset,
            blur_radius: theme.tokens.shadow_xs.blur_radius,
            spread_radius: theme.tokens.shadow_xs.spread_radius,
            color: theme.tokens.shadow_xs.color,
        };
        let focus_ring = theme.tokens.focus_ring_light();
        let error_ring_focused = theme.tokens.error_ring();
        let error_ring_unfocused = theme.tokens.error_ring();
        let ring_color = theme.tokens.ring;
        let destructive_color = theme.tokens.destructive;

        let user_style = self.style;

        VStack::new()
            .w_full()
            .gap(px(4.0))
            .child(
                div()
                    .id(("input", self.state.entity_id()))
                    .key_context("Input")
                    .track_focus(&self.state.read(cx).focus_handle(cx).tab_index(0).tab_stop(true))
                    .when(!self.disabled, |this| {
                        this.on_action(window.listener_for(&self.state, InputState::backspace))
                            .on_action(window.listener_for(&self.state, InputState::delete))
                            .on_action(window.listener_for(&self.state, InputState::left))
                            .on_action(window.listener_for(&self.state, InputState::right))
                            .on_action(window.listener_for(&self.state, InputState::select_left))
                            .on_action(window.listener_for(&self.state, InputState::select_right))
                            .on_action(window.listener_for(&self.state, InputState::select_all))
                            .on_action(window.listener_for(&self.state, InputState::home))
                            .on_action(window.listener_for(&self.state, InputState::end))
                            .on_action(window.listener_for(&self.state, InputState::copy))
                            .on_action(window.listener_for(&self.state, InputState::cut))
                            .on_action(window.listener_for(&self.state, InputState::paste))
                            .on_action(window.listener_for(&self.state, InputState::enter))
                            .on_action(window.listener_for(&self.state, InputState::tab))
                            .on_action(window.listener_for(&self.state, InputState::shift_tab))
                            .on_action(window.listener_for(&self.state, InputState::escape))
                    })
                    .child(
                        HStack::new()
                    .h(height)
                    .w_full()
                    .px(padding_x)
                    .gap(gap)
                    .bg(bg_color)
                    .border_1()
                    .border_color(border_color)
                    .rounded(theme.tokens.radius_md)
                    .items_center()
                    .text_size(font_size)
                    .font_family(theme.tokens.font_mono.clone())
                    .text_color(text_color)
                    .shadow(vec![shadow_xs])
                    .when(!self.disabled, |h| {
                        h.hover(move |style| {
                            style.border_color(if self.error {
                                destructive_color
                            } else {
                                ring_color
                            })
                        })
                    })
                    .when(is_focused && !self.disabled, |h| {
                        if self.error {
                            h.border_color(destructive_color)
                                .shadow(vec![error_ring_focused])
                        } else {
                            h.border_color(ring_color)
                                .shadow(vec![focus_ring])
                        }
                    })
                    .when(self.error && !is_focused, |h| {
                        h.shadow(vec![error_ring_unfocused])
                    })
                    .children(self.prefix)
                    .child(
                        div()
                            .flex_1()
                            .overflow_hidden()
                            .child(self.state.clone())
                    )
                    .when(show_clear, |h| {
                        h.child(
                            div()
                                .px(px(4.0))
                                .py(px(4.0))
                                .rounded(px(4.0))
                                .cursor_pointer()
                                .hover(|style| style.bg(theme.tokens.muted))
                                .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                                    state_for_clear.update(cx, |state, cx| {
                                        state.set_value("", window, cx);
                                    })
                                })
                                .child("×")
                                .text_color(theme.tokens.muted_foreground),
                        )
                    })
                    .when(self.password, |h| {
                        h.child(
                            div()
                                .px(px(4.0))
                                .py(px(4.0))
                                .rounded(px(4.0))
                                .cursor_pointer()
                                .hover(|style| style.bg(theme.tokens.muted))
                                .on_mouse_down(MouseButton::Left, {
                                    let state = state_for_password.clone();
                                    move |_, window, cx| {
                                        state.update(cx, |state, cx| {
                                            state.masked = !state.masked;
                                            cx.notify();
                                        });
                                        window.refresh();
                                    }
                                })
                                .child(
                                    Icon::new(if is_masked { "eye" } else { "eye-off" })
                                        .size(px(16.0))
                                        .color(theme.tokens.muted_foreground)
                                ),
                        )
                    })
                    .children(self.suffix)
                )
            )
            .when(
                self.helper_text.is_some() || validation_error.is_some() || success_message.is_some() || self.show_character_count,
                |v| {
                    v.child(
                        HStack::new()
                            .w_full()
                            .px(px(2.0))
                            .child(
                                div()
                                    .flex_1()
                                    .text_size(px(12.0))
                                    .font_family(theme.tokens.font_family.clone())
                                    .child(
                                        if let Some(error) = validation_error {
                                            div()
                                                .text_color(theme.tokens.destructive)
                                                .child(error.message)
                                        } else if let Some(success) = success_message {
                                            if has_value {
                                                div()
                                                    .text_color(theme.tokens.primary)
                                                    .child(success)
                                            } else {
                                                div()
                                            }
                                        } else if let Some(helper) = self.helper_text {
                                            div()
                                                .text_color(theme.tokens.muted_foreground)
                                                .child(helper)
                                        } else {
                                            div()
                                        }
                                    )
                            )
                            .when(self.show_character_count, |h| {
                                h.child(
                                    div()
                                        .text_size(px(12.0))
                                        .font_family(theme.tokens.font_family.clone())
                                        .text_color(
                                            if max_length.is_some() && content_length >= max_length.unwrap() {
                                                theme.tokens.destructive
                                            } else {
                                                theme.tokens.muted_foreground
                                            }
                                        )
                                        .child(
                                            if let Some(max) = max_length {
                                                format!("{}/{}", content_length, max)
                                            } else {
                                                format!("{}", content_length)
                                            }
                                        )
                                )
                            })
                    )
                }
            )
            .map(|this| {
                let mut vstack = this;
                vstack.style().refine(&user_style);
                vstack
            })
    }
}
