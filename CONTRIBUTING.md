# Contributing to adabraka-ui

Thank you for your interest in contributing to adabraka-ui! We welcome contributions from the community and are grateful for your support in making this library better.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [How Can I Contribute?](#how-can-i-contribute)
  - [Reporting Bugs](#reporting-bugs)
  - [Suggesting Enhancements](#suggesting-enhancements)
  - [Contributing Code](#contributing-code)
- [Development Setup](#development-setup)
- [Pull Request Process](#pull-request-process)
- [Coding Guidelines](#coding-guidelines)
- [Component Development Guidelines](#component-development-guidelines)
- [Documentation Guidelines](#documentation-guidelines)
- [Testing Guidelines](#testing-guidelines)

## Code of Conduct

This project adheres to a code of conduct that all contributors are expected to follow. Please be respectful, inclusive, and considerate in all interactions.

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check existing issues to avoid duplicates. When creating a bug report, include as many details as possible:

- **Use a clear and descriptive title**
- **Describe the exact steps to reproduce the problem**
- **Provide specific examples** (code snippets, screenshots)
- **Describe the behavior you observed** and what you expected
- **Include your environment details** (Rust version, GPUI version, OS)

Use our [bug report template](.github/ISSUE_TEMPLATE/bug_report.md) when creating issues.

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion:

- **Use a clear and descriptive title**
- **Provide a detailed description** of the proposed feature
- **Explain why this enhancement would be useful**
- **Provide examples** of how it would be used
- **Consider if this fits the scope** of adabraka-ui

Use our [feature request template](.github/ISSUE_TEMPLATE/feature_request.md) when creating suggestions.

### Contributing Code

We love pull requests! Here's how to contribute:

1. **Fork the repository** and create your branch from `main`
2. **Follow our coding guidelines** (see below)
3. **Add tests** for new functionality
4. **Update documentation** as needed
5. **Ensure all tests pass** and examples compile
6. **Submit your pull request**

## Development Setup

### Prerequisites

- Rust 1.70 or higher
- GPUI 0.2.0 or higher
- Git

### Setup Instructions

1. **Clone your fork:**
   ```bash
   git clone https://github.com/augani/adabraka-ui.git
   cd adabraka-ui
   ```

2. **Build the project:**
   ```bash
   cargo build
   ```

3. **Run tests:**
   ```bash
   cargo test
   ```

4. **Run examples:**
   ```bash
   cargo run --example demo
   ```

### Project Structure

```
adabraka-ui/
├── src/
│   ├── components/     # UI components
│   ├── theme/          # Theme system
│   ├── animations/     # Animation utilities
│   ├── layout/         # Layout utilities
│   └── lib.rs          # Library entry point
├── examples/           # Example applications
├── docs/               # GitHub Pages site
└── tests/              # Integration tests
```

## Pull Request Process

1. **Create a feature branch:**
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes** following our guidelines

3. **Test thoroughly:**
   - Run `cargo test`
   - Run `cargo clippy` for linting
   - Run `cargo fmt` for formatting
   - Test relevant examples

4. **Commit with clear messages:**
   ```bash
   git commit -m "Add feature: brief description

   Detailed description of what changed and why."
   ```

5. **Push to your fork:**
   ```bash
   git push origin feature/your-feature-name
   ```

6. **Open a Pull Request** using our template

7. **Respond to feedback** from reviewers

### PR Requirements

- ✅ All tests pass
- ✅ Code is formatted with `cargo fmt`
- ✅ No clippy warnings
- ✅ Documentation is updated
- ✅ Examples are provided for new components
- ✅ Commit messages are clear and descriptive

## Coding Guidelines

### General Principles

- **Follow Rust idioms** and best practices
- **Keep code simple and readable**
- **Prefer composition over inheritance**
- **Use meaningful variable and function names**
- **Write self-documenting code** with comments for complex logic

### Rust Style

- Use `cargo fmt` with default settings
- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Maximum line length: 100 characters
- Use `rustfmt.toml` configuration if present

### Code Organization

```rust
// 1. Imports (grouped and sorted)
use gpui::*;
use crate::theme::*;

// 2. Type definitions
pub struct MyComponent {
    // fields...
}

// 3. Constructors and builders
impl MyComponent {
    pub fn new() -> Self {
        // ...
    }

    // Builder methods
    pub fn variant(mut self, variant: Variant) -> Self {
        // ...
    }
}

// 4. Trait implementations
impl Render for MyComponent {
    // ...
}

// 5. Helper functions
```

### Naming Conventions

- **Types**: `PascalCase` (e.g., `Button`, `InputState`)
- **Functions/methods**: `snake_case` (e.g., `on_click`, `set_value`)
- **Constants**: `SCREAMING_SNAKE_CASE` (e.g., `DEFAULT_SIZE`)
- **Modules**: `snake_case` (e.g., `text_input`, `data_table`)

## Component Development Guidelines

### Component Structure

Every component should follow this pattern:

```rust
use gpui::*;
use crate::theme::*;

/// Brief description of the component
///
/// # Examples
///
/// ```rust
/// Button::new("Click me")
///     .variant(ButtonVariant::Primary)
///     .on_click(|_, _, _| println!("Clicked!"))
/// ```
pub struct Button {
    label: String,
    variant: ButtonVariant,
    // ... other fields
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ButtonVariant {
    Default,
    Primary,
    Secondary,
    // ...
}

impl Button {
    /// Creates a new button with the given label
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            variant: ButtonVariant::Default,
        }
    }

    /// Sets the button variant
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }
}

impl RenderOnce for Button {
    fn render(self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        div()
            .child(self.label)
            // ... styling
    }
}
```

### Component Checklist

When creating a new component:

- [ ] Follow the builder pattern for configuration
- [ ] Use the theme system for colors and styling
- [ ] Support common variants (size, style, state)
- [ ] Include accessibility features (ARIA, keyboard nav)
- [ ] Add comprehensive documentation
- [ ] Create a dedicated example file
- [ ] Add tests for key functionality
- [ ] Update the main demo example

### Theming

Always use theme tokens instead of hardcoded colors:

```rust
// ✅ Good
let theme = use_theme();
div().bg(theme.tokens.background)

// ❌ Bad
div().bg(rgb(0xffffff))
```

### Accessibility

- Add ARIA labels where appropriate
- Support keyboard navigation
- Provide focus indicators
- Support disabled states
- Test with screen readers when possible

## Documentation Guidelines

### Component Documentation

```rust
/// A button component for user interactions.
///
/// Buttons support multiple variants, sizes, and states. They follow
/// the shadcn/ui design system with support for icons and custom styling.
///
/// # Examples
///
/// Basic usage:
/// ```rust
/// Button::new("Click me")
///     .on_click(|_, _, _| println!("Clicked!"))
/// ```
///
/// With variants and sizes:
/// ```rust
/// Button::new("Primary")
///     .variant(ButtonVariant::Primary)
///     .size(ButtonSize::Large)
/// ```
///
/// # Accessibility
///
/// Buttons automatically receive proper ARIA labels and keyboard support.
/// They can be activated with Enter or Space when focused.
pub struct Button {
    // ...
}
```

### Method Documentation

```rust
/// Sets the button variant.
///
/// # Arguments
///
/// * `variant` - The visual variant to apply
///
/// # Examples
///
/// ```rust
/// Button::new("Save").variant(ButtonVariant::Primary)
/// ```
pub fn variant(mut self, variant: ButtonVariant) -> Self {
    // ...
}
```

### README Updates

When adding new components, update:

- Component list in the main README
- Appropriate category section
- Examples list if you added a new example

## Testing Guidelines

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_variant() {
        let button = Button::new("Test")
            .variant(ButtonVariant::Primary);

        assert_eq!(button.variant, ButtonVariant::Primary);
    }
}
```

### Integration Tests

Place integration tests in the `tests/` directory:

```rust
// tests/button_tests.rs
use adabraka_ui::components::button::*;

#[test]
fn test_button_builder_pattern() {
    // Test builder pattern
}
```

### Example Applications

Create a dedicated example for new components:

```rust
// examples/my_component_demo.rs
use adabraka_ui::prelude::*;
use gpui::*;

fn main() {
    Application::new().run(|cx| {
        // ... example implementation
    });
}
```

## Examples Guidelines

### Example Structure

Examples should:

- Demonstrate all major features
- Be well-commented
- Use realistic scenarios
- Follow the same coding style
- Be runnable with `cargo run --example name`

### Example Categories

- **Component demos**: Showcase a single component
- **Feature demos**: Demonstrate a specific feature (animations, theming)
- **Application demos**: Show components working together (IDE demo, file explorer)

## Commit Message Guidelines

Use clear, descriptive commit messages:

```
Add Button component with variants

- Implement builder pattern for configuration
- Add support for Primary, Secondary, Outline variants
- Include accessibility features (ARIA, keyboard nav)
- Add comprehensive examples and tests

Closes #123
```

Format:
- **First line**: Brief summary (50 chars or less)
- **Body**: Detailed description of changes
- **Footer**: Reference issues, breaking changes

## Questions?

If you have questions:

- Check existing issues and discussions
- Open a new discussion for general questions
- Use issue templates for bugs and feature requests
- Reach out to maintainers for guidance

## Recognition

All contributors will be recognized in our README and release notes. We appreciate every contribution, no matter how small!

## License

By contributing to adabraka-ui, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to adabraka-ui! 🎉
