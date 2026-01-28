# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Development Commands

```bash
# Build (requires nightly Rust)
cargo +nightly build

# Run tests
cargo +nightly test

# Lint
cargo +nightly clippy

# Format
cargo fmt

# Run a specific example
cargo +nightly run --example <example_name>

# Common examples
cargo +nightly run --example demo          # Comprehensive demo
cargo +nightly run --example slider_styled_demo  # Slider component demo
```

## Architecture

This is a GPUI component library (73+ components) inspired by shadcn/ui for building desktop applications in Rust.

### Module Structure

- **`components/`** - Core interactive elements (Button, Input, Slider, Select, Editor, etc.)
- **`display/`** - Presentation components (Table, DataTable, Card, Badge, Accordion)
- **`navigation/`** - Navigation components (Sidebar, Tabs, Menu, Toolbar, StatusBar, Tree)
- **`overlays/`** - Modal dialogs, popovers, tooltips, command palettes, toasts
- **`theme/`** - Design tokens and theming (light/dark variants)
- **`animations.rs`** - Animation presets and easing functions
- **`layout.rs`** - Layout utilities (VStack, HStack, Grid)
- **`prelude.rs`** - Common re-exports for end users

### Key Patterns

**Builder Pattern**: All components use builder pattern for configuration:
```rust
Button::new("id", "Label")
    .variant(ButtonVariant::Primary)
    .size(ButtonSize::Lg)
    .on_click(|_, _, _| {})
```

**Entity-based State**: Complex components use `Entity<T>` for state management:
```rust
let slider_state = cx.new(|cx| SliderState::new(cx));
Slider::new(slider_state.clone()).show_value(true)
```

**Theme System**: Use `use_theme()` for colors, never hardcode:
```rust
let theme = use_theme();
div().bg(theme.tokens.background).text_color(theme.tokens.foreground)
```

**Styled Trait**: All components implement `Styled` for GPUI styling methods:
```rust
Slider::new(state).w(px(400.0)).p(px(16.0)).rounded(px(12.0))
```

### Component Initialization

Some components require initialization in `lib.rs:init()`:
- `components::input::init(cx)`
- `components::select::init_select(cx)`
- `components::combobox::init_combobox(cx)`
- `components::editor::init(cx)`
- `navigation::sidebar::init_sidebar(cx)`
- `overlays::popover::init(cx)`

### Example Naming Convention

Examples follow `<component>_styled_demo.rs` pattern and demonstrate full Styled trait customization.
