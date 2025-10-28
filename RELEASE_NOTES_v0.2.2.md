# adabraka-ui v0.2.2 Release Notes

Release Date: October 28, 2025

## Overview

This release focuses on improving form usability and developer experience with keyboard navigation enhancements, password input fixes, and comprehensive project documentation.

## What's New

### Keyboard Navigation

We've added full Tab/Shift-Tab navigation support for form inputs, making it easier to create accessible forms:

```rust
// Tab navigation works automatically with proper FocusHandle configuration
Input::new(&email_input)
    .placeholder("Email")

Input::new(&password_input)
    .placeholder("Password")
    .password(true)

// Press Tab to move to next input, Shift-Tab to go back
```

**Implementation Details:**
- FocusHandle configured with `.tab_index(0).tab_stop(true)` for proper focus management
- Window-level navigation using `window.focus_next()` and `window.focus_prev()`
- Emits `InputEvent::Tab` and `InputEvent::ShiftTab` for custom handling

### Password Input Eye Icon Toggle

The password input eye icon now works correctly, allowing users to toggle password visibility:

```rust
Input::new(&password_input)
    .password(true)  // Enables the eye icon toggle
    .placeholder("Enter password")
```

**Features:**
- Icon toggles between "eye" (hidden) and "eye-off" (visible)
- Password masking switches between bullets (••••) and actual text
- Immediate visual feedback with proper state management
- Clean, intuitive UX aligned with modern web standards

### Comprehensive ROADMAP

Added detailed [ROADMAP.md](ROADMAP.md) with complete component inventory:

- **90+ Components** organized by category (Display, Forms, Navigation, Overlays, etc.)
- Status indicators (✅ Complete, 🔄 In Progress, ❌ Todo, ⚠️ Partial)
- Phase-based development plan with desktop integration features
- Prioritized quick wins and improvements

## Improvements

### Code Quality
- Removed 13 unnecessary inline comments across 6 files
- Cleaner, more production-ready codebase
- Files improved: color_picker.rs, input_state.rs, input.rs, text.rs, lib.rs, transitions.rs

## New Examples

- **password_test.rs** - Demonstrates password toggle functionality with clear instructions

## Bug Fixes

- Fixed password input eye icon not toggling correctly
- Fixed password masking not switching between hidden/visible states
- Fixed state reading to use dynamic `input_state.masked` value for immediate updates

## Technical Details

### Password Toggle Implementation

The password toggle now properly reads state dynamically and triggers UI refresh:

```rust
// Dynamic state reading
let is_masked = input_state.masked;

// Toggle on click with immediate refresh
state.update(cx, |state, cx| {
    state.masked = !state.masked;
    cx.notify();
});
window.refresh();
```

### Keyboard Navigation Architecture

Built on GPUI's focus system with proper FocusHandle configuration:

```rust
// FocusHandle is the source of truth for tab properties
.track_focus(&self.state.read(cx).focus_handle(cx).tab_index(0).tab_stop(true))

// Tab handlers in InputState
pub fn tab(&mut self, _: &Tab, window: &mut Window, cx: &mut Context<Self>) {
    window.focus_next();
    cx.emit(InputEvent::Tab);
}
```

## Migration Guide

No breaking changes in this release. All existing code will continue to work without modifications.

To use the new keyboard navigation features, ensure your inputs are rendered in the proper order and they will automatically support Tab/Shift-Tab navigation.

## Installation

```toml
[dependencies]
adabraka-ui = "0.2.2"
```

## What's Next

Check out our [ROADMAP.md](ROADMAP.md) for upcoming features including:
- File upload component
- Virtualized list improvements
- Multi-window support
- System tray integration
- And much more!

## Contributors

Thank you to everyone who contributed to this release!

## Resources

- [GitHub Repository](https://github.com/augani/adabraka-ui)
- [Documentation](https://augani.github.io/adabraka-ui/)
- [Examples](https://github.com/augani/adabraka-ui/tree/main/examples)
- [ROADMAP](https://github.com/augani/adabraka-ui/blob/main/ROADMAP.md)

---

For questions or feedback, please open an issue on [GitHub](https://github.com/augani/adabraka-ui/issues).
