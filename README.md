# adabraka-ui

[![Crates.io](https://img.shields.io/crates/v/adabraka-ui.svg)](https://crates.io/crates/adabraka-ui)
[![Downloads](https://img.shields.io/crates/d/adabraka-ui.svg)](https://crates.io/crates/adabraka-ui)
[![Documentation](https://docs.rs/adabraka-ui/badge.svg)](https://docs.rs/adabraka-ui)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-nightly-orange.svg)](https://www.rust-lang.org/)
[![GitHub Stars](https://img.shields.io/github/stars/Augani/adabraka-ui?style=social)](https://github.com/Augani/adabraka-ui)

A comprehensive, professional UI component library for [GPUI](https://github.com/zed-industries/zed), the GPU-accelerated UI framework powering the Zed editor. Inspired by [shadcn/ui](https://ui.shadcn.com/), adabraka-ui provides 73+ polished, accessible components for building beautiful desktop applications in Rust.

**[📖 Documentation](https://augani.github.io/adabraka-ui/)** · **[🚀 Getting Started](#installation)** · **[📦 Components](#components)** · **[💡 Examples](#examples)**

## ✨ Features

- 🎨 **Complete Theme System** - Built-in light/dark themes with semantic color tokens
- 🧩 **73+ Components** - Comprehensive library covering all UI needs from buttons to data tables
- 📱 **Responsive Layout** - Flexible layout utilities (VStack, HStack, Grid)
- 🎭 **Professional Animations** - Smooth transitions with cubic-bezier easing and spring physics
- ✍️ **Typography System** - Built-in Text component with semantic variants
- 💻 **Code Editor** - Multi-line editor with syntax highlighting and full keyboard support
- ♿ **Accessibility** - Full keyboard navigation, ARIA labels, and screen reader support
- 🎯 **Type-Safe** - Leverages Rust's type system for compile-time guarantees
- 🚀 **High Performance** - Optimized for GPUI's retained-mode rendering with virtual scrolling
- 📚 **Well Documented** - Extensive examples and comprehensive API documentation

## 🎬 Showcase

See adabraka-ui in action in real applications:

### Desktop Music Player
![Music Player App](docs/assets/images/music-player.png)

A beautiful desktop music player with offline playing capabilities. Features smooth animations, responsive UI, and a polished user experience built entirely with adabraka-ui components.

### Project Task Manager
![Task Manager App](docs/assets/images/task-manager.png)

A powerful task management application used to track the development of this UI library. Features drag-and-drop task organization with smooth animations, showcasing the library's advanced capabilities.

## 🚀 Installation

> **Note:** Currently requires Rust nightly due to GPUI dependencies. Install with: `rustup toolchain install nightly`

Add adabraka-ui to your `Cargo.toml`:

```toml
[dependencies]
adabraka-ui = "0.2.3"
gpui = "0.2.0"
```

Build your project with nightly:
```bash
cargo +nightly build
```

## ✨ What's New in v0.2.3

**Latest Release (November 13, 2025)** - Enhanced slider component with vertical orientation support!

### 📊 Vertical Slider Support
The Slider component now supports both horizontal and vertical orientations. Perfect for volume controls, brightness adjustments, and other vertical UI patterns.

```rust
// Horizontal slider (default)
Slider::new(slider_state.clone())
    .show_value(true)

// Vertical slider
Slider::new(slider_state.clone())
    .vertical()
    .size(SliderSize::Lg)
    .show_value(true)
    .on_change(|value, _, _| {
        println!("Value: {}", value);
    })
```

### 🐛 Slider Thumb Centering Fixed
The slider thumb is now perfectly centered on the track line, providing a more polished and professional appearance across all size variants (Sm, Md, Lg).

### 🎨 Improved Component Architecture
- Separate `render_horizontal()` and `render_vertical()` methods for cleaner code
- Adaptive thumb shape: horizontal oval for horizontal sliders, vertical oval for vertical sliders
- Better positioning logic using container dimensions matching thumb dimensions

### 📚 Enhanced Examples
Updated `slider_styled_demo.rs` with 10 comprehensive examples showcasing horizontal and vertical sliders with various styling options.

---

## Quick Start

```rust
use adabraka_ui::prelude::*;
use gpui::*;

fn main() {
    Application::new().run(|cx| {
        // Initialize the UI library
        adabraka_ui::init(cx);

        // Install a theme
        install_theme(cx, Theme::dark());

        cx.open_window(
            WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some("My App".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_, cx| cx.new(|_| MyApp::new()),
        ).unwrap();
    });
}

struct MyApp;

impl MyApp {
    fn new() -> Self {
        Self
    }
}

impl Render for MyApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        VStack::new()
            .p(px(32.0))
            .gap(px(16.0))
            .child(
                div()
                    .text_size(px(24.0))
                    .font_weight(FontWeight::BOLD)
                    .child("Welcome to adabraka-ui!")
            )
            .child(
                Button::new("get-started", "Get Started")
                    .variant(ButtonVariant::Default)
                    .on_click(|_event, _window, _cx| {
                        println!("Button clicked!");
                    })
            )
    }
}
```

## 🎨 Component Customization with Styled Trait

**All 54 components implement the `Styled` trait**, giving you complete control over styling!

### Full Customization Support

Every component can be customized using GPUI's powerful styling API. Apply any styling method to any component:

```rust
Button::new("custom-btn", "Click Me")
    .variant(ButtonVariant::Primary)  // Use built-in variant
    .bg(rgb(0x8b5cf6))                 // Custom background
    .p_8()                              // Custom padding
    .rounded_xl()                       // Custom border radius
    .border_2()                         // Custom border
    .border_color(rgb(0xa78bfa))        // Custom border color
    .shadow_lg()                        // Shadow effect
    .w_full()                           // Full width
```

### Available Styling Methods

**Backgrounds & Colors:**
- `.bg(color)` - Background color
- `.text_color(color)` - Text color
- `.border_color(color)` - Border color

**Spacing:**
- `.p_4()`, `.p_8()` - Padding (all sides)
- `.px_6()`, `.py_4()` - Padding (horizontal/vertical)
- `.m_4()`, `.mx_auto()` - Margins

**Borders & Radius:**
- `.border_2()`, `.border_4()` - Border width
- `.rounded_sm()`, `.rounded_lg()`, `.rounded_xl()` - Border radius
- `.rounded(px(16.0))` - Custom radius

**Sizing:**
- `.w_full()`, `.h_full()` - Full width/height
- `.w(px(300.0))`, `.h(px(200.0))` - Custom dimensions
- `.min_w()`, `.max_w()` - Min/max constraints

**Effects:**
- `.shadow_sm()`, `.shadow_lg()` - Shadow effects
- `.opacity()` - Opacity control

**And hundreds more!** Any GPUI styling method works.

### Philosophy: Good Defaults, Complete Control

Following the [shadcn/ui philosophy](https://ui.shadcn.com/docs):

> Components ship with sensible defaults that you can completely override.

adabraka-ui provides **great defaults AND 100% control** over every component's styling!

### Examples

Every component now has a `*_styled_demo.rs` example showing full customization capabilities:

```bash
cargo +nightly run --example button_styled_demo
cargo +nightly run --example input_styled_demo
cargo +nightly run --example data_table_styled_demo
# ... and 51 more!
```

## Theme System

### Overview

adabraka-ui provides a complete theming system with semantic color tokens inspired by shadcn/ui. Themes include both light and dark variants with carefully chosen colors for accessibility and visual hierarchy.

### Basic Usage

```rust
use adabraka_ui::theme::{install_theme, Theme, use_theme};

// In your app initialization
install_theme(cx, Theme::dark()); // or Theme::light()

// In your render method
fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
    let theme = use_theme();

    div()
        .bg(theme.tokens.background)
        .text_color(theme.tokens.foreground)
        .child("Themed content")
}
```

### Available Themes

- `Theme::light()` - Clean light theme
- `Theme::dark()` - Dark theme with proper contrast

### Theme Tokens

The theme system provides semantic color tokens:

```rust
// Background colors
background, card, popover, muted

// Text colors
foreground, card_foreground, popover_foreground, muted_foreground

// Interactive colors
primary, primary_foreground, secondary, secondary_foreground
accent, accent_foreground, destructive, destructive_foreground

// UI elements
border, input, ring

// Spacing and sizing
radius_sm, radius_md, radius_lg
font_family, font_mono
```

## Layout Utilities

### VStack and HStack

Vertical and horizontal stack layouts with consistent spacing.

```rust
// Vertical stack
VStack::new()
    .spacing(px(16.0))  // Gap between children
    .align(Align::Center)  // Cross-axis alignment
    .padding(px(24.0))
    .child(Text::new("Item 1"))
    .child(Text::new("Item 2"))

// Horizontal stack
HStack::new()
    .spacing(px(12.0))
    .justify(Justify::Between)  // Main-axis justification
    .align(Align::Center)
    .child(Button::new("cancel", "Cancel"))
    .child(Button::new("save", "Save").variant(ButtonVariant::Default))
```

### Grid Layout

```rust
Grid::new()
    .cols(3)  // 3 columns
    .gap(px(16.0))
    .child(card1)
    .child(card2)
    .child(card3)
```

### Layout Options

- **Align**: `Start`, `Center`, `End`, `Stretch`
- **Justify**: `Start`, `Center`, `End`, `Between`, `Around`, `Evenly`
- **Flow**: `Horizontal`, `Vertical` (for wrapping)

## Components

### Text Component

The Text component provides consistent typography with built-in theming and font handling.

```rust
use adabraka_ui::components::text::*;

// Semantic heading variants
h1("Page Title")
h2("Section Title")
h3("Subsection")
h4("Minor Heading")

// Body text variants
body("Regular paragraph text")
body_large("Larger body text for lead paragraphs")
body_small("Smaller body text")
caption("Small text for captions and metadata")

// Label variants
label("Form Label")
label_small("Compact Label")

// Code/monospace text
code("fn main() { }")
code_small("let x = 42;")

// Muted text (secondary color)
muted("Secondary information")
muted_small("Small secondary text")

// Custom styling
Text::new("Custom text")
    .size(px(18.0))
    .weight(FontWeight::BOLD)
    .color(rgb(0x3b82f6).into())
    .underline()
    .no_wrap()  // Single line
    .truncate()  // Add ellipsis when too long

// Builder pattern with variants
Text::new("Styled text")
    .variant(TextVariant::H2)
    .color(theme.tokens.primary)
```

**Benefits:**
- ✓ No need to manually apply `font_family()` on every text element
- ✓ Consistent typography across your application
- ✓ Easy to change fonts globally by updating the theme
- ✓ Semantic variants for proper content hierarchy
- ✓ Builder pattern for flexible customization

### Buttons

```rust
// Basic button
Button::new("click-me", "Click me")
    .on_click(|_event, _window, _cx| {
        println!("Clicked!");
    })

// Styled variants
Button::new("primary", "Primary").variant(ButtonVariant::Default)
Button::new("secondary", "Secondary").variant(ButtonVariant::Secondary)
Button::new("outline", "Outline").variant(ButtonVariant::Outline)
Button::new("ghost", "Ghost").variant(ButtonVariant::Ghost)
Button::new("link", "Link").variant(ButtonVariant::Link)
Button::new("destructive", "Destructive").variant(ButtonVariant::Destructive)

// Sizes
Button::new("small", "Small").size(ButtonSize::Sm)
Button::new("medium", "Medium").size(ButtonSize::Md)  // default
Button::new("large", "Large").size(ButtonSize::Lg)

// States
Button::new("disabled", "Disabled").disabled(true)

// Icon buttons
IconButton::new(IconSource::Named("search".to_string()))
    .size(px(32.0))
    .on_click(handler)
```

### Input Components

#### Text Input

```rust
// Basic text input
let input_state = cx.new(|cx| InputState::new(cx));

Input::new(input_state.clone(), cx)
    .placeholder("Enter text...")
    .variant(InputVariant::Default)
    .size(InputSize::Md)

// Variants
Input::new(input_state, cx)
    .variant(InputVariant::Outline)
    .variant(InputVariant::Ghost)

// Password input with eye icon toggle (fixed in v0.2.2!)
Input::new(password_input, cx)
    .password(true)  // Enables eye icon toggle for show/hide
    .placeholder("Enter password")

// With prefix/suffix
Input::new(input, cx)
    .prefix(div().child("🔍"))
    .suffix(Button::new("clear", "Clear").size(ButtonSize::Sm))
```

#### Checkbox

```rust
Checkbox::new("checkbox-id")
    .label("Check me")
    .checked(false)
    .on_click(cx.listener(|view, checked, _window, cx| {
        view.is_checked = *checked;
        cx.notify();
    }))

// Sizes
Checkbox::new("small").size(CheckboxSize::Sm)
Checkbox::new("medium").size(CheckboxSize::Md)
Checkbox::new("large").size(CheckboxSize::Lg)

// States
Checkbox::new("disabled").disabled(true)
Checkbox::new("indeterminate").indeterminate(true)
```

#### Toggle

```rust
Toggle::new("toggle-id")
    .label("Enable feature")
    .checked(true)
    .on_click(cx.listener(|view, checked, _window, cx| {
        view.feature_enabled = *checked;
        cx.notify();
    }))

// Sizes and variants available
```

#### Slider

Interactive slider for selecting numeric values with horizontal and vertical orientations:

```rust
// Create slider state
let slider_state = cx.new(|cx| {
    let mut state = SliderState::new(cx);
    state.set_min(0.0, cx);
    state.set_max(100.0, cx);
    state.set_value(50.0, cx);
    state.set_step(1.0, cx);
    state
});

// Horizontal slider (default)
Slider::new(slider_state.clone())
    .size(SliderSize::Md)
    .show_value(true)
    .on_change(|value, _window, _cx| {
        println!("Value changed: {}", value);
    })

// Vertical slider
Slider::new(slider_state.clone())
    .vertical()
    .size(SliderSize::Lg)
    .show_value(true)

// Sizes
Slider::new(slider_state.clone()).size(SliderSize::Sm)  // Small
Slider::new(slider_state.clone()).size(SliderSize::Md)  // Medium (default)
Slider::new(slider_state.clone()).size(SliderSize::Lg)  // Large

// Disabled state
Slider::new(slider_state.clone()).disabled(true)

// Full Styled trait support for custom styling
Slider::new(slider_state.clone())
    .w(px(400.0))
    .bg(rgb(0x1e293b))
    .p(px(16.0))
    .rounded(px(12.0))
    .border_2()
    .border_color(rgb(0x3b82f6))
```

#### Select Dropdown

```rust
let options = vec![
    SelectOption::new("option1", "Option 1"),
    SelectOption::new("option2", "Option 2"),
];

Select::new(cx)
    .options(options)
    .selected_index(Some(0))
    .placeholder("Choose an option")
    .on_change(cx.listener(|view, selected_value, _window, cx| {
        view.selected_option = Some(selected_value.clone());
        cx.notify();
    }))

// Features
Select::new(cx)
    .searchable(true)      // Enable search
    .clearable(true)       // Show clear button
    .loading(true)         // Show loading state
    .disabled(true)        // Disable interaction
```

#### Textarea

```rust
Textarea::new(textarea_state, cx)
    .placeholder("Enter your message...")
    .rows(4)
    .resize(TextareaResize::Vertical)
    .max_length(Some(500))
```

#### SearchInput

Advanced search input with filters, case sensitivity, and results count:

```rust
use adabraka_ui::components::search_input::*;

// Create search input with filters
let search = cx.new(|cx| SearchInput::new(cx)
    .filters(vec![
        SearchFilter::new("*.rs", "rs"),
        SearchFilter::new("*.toml", "toml"),
        SearchFilter::new("*.md", "md"),
    ], cx)
    .on_search(move |query, app_cx| {
        // Handle search query
        println!("Searching for: {}", query);
    }, cx)
    .on_filter_toggle(move |index, app_cx| {
        // Handle filter toggle
        println!("Toggled filter: {}", index);
    }, cx)
);

// Update results count from your component
search.update(cx, |search, cx| {
    search.state().update(cx, |state, cx| {
        state.set_results_count(Some(42), cx);
    });
});

// Check filter states
let active_filters = search.read(cx).state().read(cx).active_filters();
let case_sensitive = search.read(cx).state().read(cx).case_sensitive();
```

**Features:**
- ✓ Search icon and clear button
- ✓ Filter badges/chips that can be toggled
- ✓ Case-sensitive toggle (Aa button)
- ✓ Regex mode toggle (.* button)
- ✓ Loading state indicator
- ✓ Results count display
- ✓ Real-time search callbacks
- ✓ Platform-aware styling

#### ColorPicker

Full-featured color picker with multiple color modes and recent color history:

```rust
use adabraka_ui::components::color_picker::*;

// Create color picker state
let color_state = cx.new(|_| ColorPickerState::new(hsla(210.0, 0.7, 0.5, 1.0)));

// Render color picker
ColorPicker::new("picker-id", color_state.clone())
    .show_alpha(true)
    .on_change({
        let state = color_state.clone();
        move |color, _window, cx| {
            state.update(cx, |s, cx| {
                s.set_color(color);
                cx.notify();
            });
            println!("Color changed: {:?}", color);
        }
    })

// With custom swatches
ColorPicker::new("custom-picker", color_state.clone())
    .swatches(vec![
        hsla(0.0, 0.8, 0.5, 1.0),   // Red
        hsla(120.0, 0.7, 0.5, 1.0), // Green
        hsla(240.0, 0.8, 0.6, 1.0), // Blue
    ])
    .show_alpha(false)
```

**Features:**
- ✓ Three color modes: HSL, RGB, HEX with easy switching
- ✓ Color preview with live updates
- ✓ Recent colors history (stores last 10)
- ✓ Custom color swatches
- ✓ Optional alpha/opacity slider
- ✓ Copy to clipboard (HEX format)
- ✓ Popover-based UI for clean integration
- ✓ Immediate UI updates without mouse movement

#### DatePicker

Advanced date picker with single date and date range selection:

```rust
use adabraka_ui::components::date_picker::*;
use adabraka_ui::components::calendar::*;

// Single date picker
let single_state = cx.new(|cx| DatePickerState::new(cx));

DatePicker::new(single_state.clone())
    .placeholder("Select a date")
    .on_select(cx.listener(|this, date, _window, cx| {
        println!("Selected: {:?}", date);
        cx.notify();
    }))

// Date range picker
let range_state = cx.new(|cx|
    DatePickerState::new_with_mode(DateSelectionMode::Range, cx)
);

DatePicker::new(range_state.clone())
    .placeholder("Select date range")
    .on_select(cx.listener(|this, _date, _window, cx| {
        let state = this.range_state.read(cx);
        if let Some(range) = state.selected_range {
            println!("Range: {} to {}", range.start, range.end);
        }
        cx.notify();
    }))

// With disabled dates
DatePicker::new(picker_state.clone())
    .disable_weekends()
    .disable_dates(vec![
        DateValue::new(2024, 12, 25), // Christmas
        DateValue::new(2024, 1, 1),   // New Year
    ])
```

**Features:**
- ✓ Single date and date range selection modes
- ✓ Visual range highlighting with colored backgrounds
- ✓ Disabled dates with greyed-out styling
- ✓ Weekend disabling helper
- ✓ Auto-close popover after selection
- ✓ Multiple date formats (ISO, US, EU, custom)
- ✓ Locale support for internationalization
- ✓ Month navigation with year selection
- ✓ Today button for quick selection
- ✓ Immediate UI updates

#### Combobox

Searchable dropdown with single and multi-select support:

```rust
use adabraka_ui::components::combobox::*;

// Create combobox state
let combobox_state = cx.new(|_| ComboboxState::new());

// Single select
Combobox::new(combobox_state.clone())
    .items(vec!["Apple", "Banana", "Cherry", "Date"])
    .placeholder("Select a fruit...")
    .on_select(cx.listener(|this, item, _window, cx| {
        println!("Selected: {}", item);
        cx.notify();
    }))

// Multi-select
Combobox::new(multi_state.clone())
    .items(vec!["Rust", "Python", "JavaScript", "Go"])
    .placeholder("Select languages...")
    .multi_select(true)
    .on_select(cx.listener(|this, items, _window, cx| {
        println!("Selected items: {:?}", items);
        cx.notify();
    }))

// With custom display
Combobox::new(custom_state.clone())
    .items(users)
    .display_fn(|user| format!("{} <{}>", user.name, user.email))
    .search_fn(|user, query| {
        user.name.contains(query) || user.email.contains(query)
    })
```

**Features:**
- ✓ Single and multi-select modes
- ✓ Real-time search/filter with immediate updates
- ✓ Keyboard navigation (arrow keys, Enter, Escape)
- ✓ Custom display and search functions
- ✓ Clear selection button
- ✓ Badge display for multi-select
- ✓ Popover-based dropdown
- ✓ Empty state handling
- ✓ Disabled state support

#### Editor

A high-performance multi-line code editor with syntax highlighting, perfect for SQL queries and code editing:

```rust
// Create editor state
let editor_state = cx.new(|cx| {
    let mut state = EditorState::new(cx);
    state.set_language(Language::Sql, cx);
    state.set_content("SELECT * FROM users;", cx);
    state
});

// Render editor
Editor::new(&editor_state)
    .language(Language::Sql, cx)
    .min_lines(10)
    .show_line_numbers(true, cx)

// Get content
let content = editor.get_content(cx);
```

**Features:**
- ✓ Real-time syntax highlighting using syntect
- ✓ Full keyboard navigation (arrows, Home, End, Page Up/Down)
- ✓ Mouse selection support with drag and click
- ✓ Copy/paste/cut clipboard operations
- ✓ Line numbers with proper gutter
- ✓ Vertical scrolling for large files
- ✓ Language support (SQL, PlainText - extensible)
- ✓ EntityInputHandler for OS-level text input

**Keyboard Shortcuts:**
- `Ctrl+A` / `Cmd+A` - Select all
- `Ctrl+C` / `Cmd+C` - Copy
- `Ctrl+X` / `Cmd+X` - Cut
- `Ctrl+V` / `Cmd+V` - Paste
- Arrow keys - Navigate cursor
- `Shift + Arrow` - Extend selection
- `Home` / `End` - Jump to line start/end
- `Ctrl+Home` / `Cmd+Up` - Jump to document start
- `Ctrl+End` / `Cmd+Down` - Jump to document end
- `Page Up` / `Page Down` - Scroll by page

**Example:**
```rust
struct MyApp {
    editor_state: Entity<EditorState>,
}

impl MyApp {
    fn new(cx: &mut Context<Self>) -> Self {
        let editor_state = cx.new(|cx| {
            let mut state = EditorState::new(cx);
            state.set_language(Language::Sql, cx);
            state.set_content(
                "-- Sample SQL Query\n\
                SELECT id, name, email \n\
                FROM users \n\
                WHERE created_at >= '2024-01-01';",
                cx,
            );
            state
        });

        Self { editor_state }
    }
}

impl Render for MyApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .child(
                Editor::new(&self.editor_state)
                    .language(Language::Sql, cx)
                    .min_lines(20)
                    .show_line_numbers(true, cx)
            )
    }
}
```

### Progress & Loading Indicators

Display progress for long-running operations with progress bars and spinners.

#### ProgressBar

Linear progress indicators with determinate and indeterminate modes:

```rust
use adabraka_ui::components::progress::*;

// Determinate progress (0.0 to 1.0)
ProgressBar::new(0.75)  // 75% complete

// With label and percentage
ProgressBar::new(0.45)
    .label("Uploading files...")
    .show_percentage(true)

// Different variants
ProgressBar::new(0.8)
    .variant(ProgressVariant::Success)
    .label("Upload complete")

ProgressBar::new(0.6)
    .variant(ProgressVariant::Warning)
    .label("Storage almost full")

ProgressBar::new(0.4)
    .variant(ProgressVariant::Destructive)
    .label("Critical: Low memory")

// Different sizes
ProgressBar::new(0.5).size(ProgressSize::Sm)  // Thin (4px)
ProgressBar::new(0.5).size(ProgressSize::Md)  // Default (8px)
ProgressBar::new(0.5).size(ProgressSize::Lg)  // Large (12px)

// Indeterminate (loading animation)
ProgressBar::indeterminate()
    .label("Loading...")

ProgressBar::indeterminate()
    .variant(ProgressVariant::Success)
    .label("Syncing data...")
```

#### CircularProgress / Spinner

Circular progress indicators and loading spinners:

```rust
// Determinate circular progress
CircularProgress::new(0.75)  // 75% complete

// Indeterminate spinner
CircularProgress::indeterminate()

// With variants
CircularProgress::indeterminate()
    .variant(ProgressVariant::Success)
    .size(px(32.0))

CircularProgress::indeterminate()
    .variant(ProgressVariant::Warning)
    .size(px(48.0))

CircularProgress::indeterminate()
    .variant(ProgressVariant::Destructive)
    .size(px(64.0))
```

**Features:**
- ✓ Determinate and indeterminate modes
- ✓ Multiple variants (Default, Success, Warning, Destructive)
- ✓ Configurable sizes
- ✓ Optional labels and percentage display
- ✓ Smooth animations for indeterminate state
- ✓ Consistent with shadcn/ui design language

**Use Cases:**
- File uploads/downloads
- Data synchronization
- Processing operations
- Loading states
- Installation progress

### Data Display

#### Table

```rust
let columns = vec![
    Column::new("name", "Name").sortable(true),
    Column::new("email", "Email").sortable(true),
    Column::new("role", "Role"),
];

Table::new(cx)
    .columns(columns)
    .data(user_data)
    .sortable(true)
    .on_row_click(|user, _window, _cx| {
        println!("Clicked user: {}", user.name);
    })
```

#### DataTable

High-performance table for large datasets:

```rust
let columns = vec![
    DataTableColumn::new("id", "ID").width(px(80.0)),
    DataTableColumn::new("name", "Name").width(px(200.0)),
    DataTableColumn::new("email", "Email").width(px(250.0)),
];

DataTable::new(data, columns, cx)
    .sortable(true)
    .on_row_select(|item, _window, _cx| {
        println!("Selected: {:?}", item);
    })
```

#### Badge

```rust
Badge::new("New")
Badge::new("12").variant(BadgeVariant::Secondary)
Badge::new("Error").variant(BadgeVariant::Destructive)
```

#### Card

```rust
Card::new()
    .header(div().child("Card Title"))
    .content(div().child("Card content goes here"))
    .footer(div().child("Card footer"))
```

### Navigation

#### Sidebar

```rust
let items = vec![
    SidebarItem::new("dashboard", "Dashboard")
        .with_icon(IconSource::Named("home".to_string())),
    SidebarItem::new("settings", "Settings")
        .with_icon(IconSource::Named("settings".to_string()))
        .with_badge("3".to_string()),
];

Sidebar::new(cx)
    .items(items)
    .selected_id("dashboard".to_string())
    .variant(SidebarVariant::Collapsible)
    .position(SidebarPosition::Left)
    .expanded_width(px(280.0))
    .collapsed_width(px(64.0))
    .on_select(cx.listener(|view, id, _window, cx| {
        view.current_page = id.clone();
        cx.notify();
    }))
```

#### Tabs

```rust
let tabs = vec![
    TabItem::new("tab1", "Overview"),
    TabItem::new("tab2", "Settings").with_icon(IconSource::Named("settings".to_string())),
];

let panels = vec![
    TabPanel::new("tab1", div().child("Overview content")),
    TabPanel::new("tab2", div().child("Settings content")),
];

Tabs::new(cx)
    .tabs(tabs)
    .panels(panels)
    .selected_index(0)
    .on_change(cx.listener(|view, index, _window, cx| {
        view.active_tab = *index;
        cx.notify();
    }))
```

#### Breadcrumbs

```rust
let items = vec![
    BreadcrumbItem::new("home", "Home"),
    BreadcrumbItem::new("projects", "Projects"),
    BreadcrumbItem::new("current", "Current Project"),
];

Breadcrumbs::new(cx)
    .items(items)
    .on_click(cx.listener(|view, id, _window, cx| {
        view.navigate_to(id.clone());
        cx.notify();
    }))
```

#### Tree

```rust
let nodes = vec![
    TreeNode::new("root", "Root")
        .with_children(vec![
            TreeNode::new("child1", "Child 1"),
            TreeNode::new("child2", "Child 2")
                .with_children(vec![
                    TreeNode::new("grandchild", "Grandchild"),
                ]),
        ]),
];

Tree::new(cx)
    .nodes(nodes)
    .selected_id(Some("child1".to_string()))
    .expanded_ids(vec!["root".to_string()])
    .on_select(cx.listener(|view, id, _window, cx| {
        view.selected_item = Some(id.clone());
        cx.notify();
    }))
```

#### Menu System

A comprehensive menu system for desktop applications with MenuBar, Menu, MenuItem, and ContextMenu components:

```rust
// MenuBar - Top-level horizontal menu bar
let file_menu = vec![
    MenuItem::new("new", "New File")
        .with_icon(IconSource::Named("file-plus".into()))
        .with_shortcut("Cmd+N")
        .on_click(|_window, _cx| println!("New file")),
    MenuItem::separator(),
    MenuItem::new("save", "Save")
        .with_shortcut("Cmd+S")
        .on_click(|_window, _cx| println!("Save")),
];

MenuBar::new(cx, vec![
    MenuBarItem::new("File", file_menu),
    MenuBarItem::new("Edit", edit_menu),
    MenuBarItem::new("View", view_menu),
])

// MenuItem types
MenuItem::new("action", "Regular Action")  // Action item
MenuItem::checkbox("check", "Enable Feature", true)  // Checkbox
MenuItem::new("parent", "Submenu").with_children(vec![...])  // Submenu
MenuItem::separator()  // Visual divider

// Standalone Menu
Menu::new(cx, vec![
    MenuItem::new("copy", "Copy")
        .with_icon(IconSource::Named("copy".into()))
        .with_shortcut("Cmd+C"),
    MenuItem::new("paste", "Paste")
        .with_icon(IconSource::Named("clipboard".into())),
])

// ContextMenu - Right-click context menu
ContextMenu::new(cx, menu_items, position)
    .on_close(|_window, _cx| {
        // Handle close
    })
```

**Features:**
- ✓ MenuBar for application-level menus
- ✓ Nested submenus
- ✓ Checkbox and radio items
- ✓ Keyboard shortcuts display
- ✓ Icons and separators
- ✓ Disabled states
- ✓ Context menus for right-click interactions

#### Toolbar

Action bars with icon buttons, groups, and toggle states for desktop applications:

```rust
// Create toolbar with groups
let formatting_group = ToolbarGroup::new()
    .button(
        ToolbarButton::new("bold", IconSource::Named("bold".into()))
            .tooltip("Bold")
            .variant(ToolbarButtonVariant::Toggle)
            .pressed(is_bold)
            .on_click(|_window, _cx| toggle_bold())
    )
    .button(
        ToolbarButton::new("italic", IconSource::Named("italic".into()))
            .tooltip("Italic")
            .variant(ToolbarButtonVariant::Toggle)
            .pressed(is_italic)
    )
    .separator()
    .button(
        ToolbarButton::new("underline", IconSource::Named("underline".into()))
            .tooltip("Underline")
    );

Toolbar::new()
    .size(ToolbarSize::Md)
    .group(formatting_group)
    .group(alignment_group)

// Button variants
ToolbarButtonVariant::Default      // Regular button
ToolbarButtonVariant::Toggle      // Toggle button (pressed/unpressed)
ToolbarButtonVariant::Dropdown    // Shows dropdown indicator

// Sizes
ToolbarSize::Sm  // 32px buttons
ToolbarSize::Md  // 36px buttons (default)
ToolbarSize::Lg  // 40px buttons

// Toolbar items
ToolbarItem::Button(button)   // Button
ToolbarItem::Separator        // Visual separator
ToolbarItem::Spacer          // Flexible space (push to right)
```

**Features:**
- ✓ Icon buttons with tooltips
- ✓ Toggle states for formatting tools
- ✓ Button groups with separators
- ✓ Flexible spacers for layout control
- ✓ Multiple sizes
- ✓ Disabled states
- ✓ Dropdown button indicators

### Desktop-Specific Components

#### StatusBar

Bottom application status bar with three sections (left, center, right) for displaying app state and contextual information:

```rust
use adabraka_ui::navigation::status_bar::*;

// Create status bar with sections (capturing entity for callbacks)
let entity = cx.entity().clone();
cx.new(|_| {
    StatusBar::new()
        .left(vec![
            StatusItem::icon_text(IconSource::Named("file".into()), "main.rs"),
            StatusItem::text("Ln 42, Col 15"),
        ])
        .center(vec![
            StatusItem::text("Ready"),
        ])
        .right(vec![
            StatusItem::icon_badge(
                IconSource::Named("bell".into()),
                "3"
            )
            .badge_variant(BadgeVariant::Default)
            .tooltip("Notifications")
            .on_click({
                let entity = entity.clone();
                move |_window, app_cx| {
                    app_cx.update_entity(&entity, |this: &mut MyApp, cx| {
                        this.show_notifications(cx);
                    });
                }
            }),
            StatusItem::text("UTF-8")
                .tooltip("File Encoding")
                .on_click({
                    let entity = entity.clone();
                    move |_window, app_cx| {
                        app_cx.update_entity(&entity, |this: &mut MyApp, cx| {
                            this.change_encoding(cx);
                        });
                    }
                }),
            StatusItem::icon_text(IconSource::Named("git-branch".into()), "main")
                .tooltip("Git Branch"),
        ])
})

// Status item types
StatusItem::text("Text")                    // Text only
StatusItem::icon(IconSource::Named(...))    // Icon only
StatusItem::icon_text(icon, "Text")         // Icon with text
StatusItem::badge("3", "Tooltip")           // Badge only
StatusItem::icon_badge(icon, "3")           // Icon with badge

// Item customization
StatusItem::text("Clickable")
    .on_click({
        let entity = entity.clone();
        move |_window, app_cx| {
            app_cx.update_entity(&entity, |this: &mut MyApp, cx| {
                // Handle click
                cx.notify();
            });
        }
    })
    .tooltip("Tooltip text")
    .disabled(true)
    .badge_variant(BadgeVariant::Warning)
```

**Features:**
- ✓ Three sections: left, center, right
- ✓ Icons, text, and badges
- ✓ Click handlers for interactive items
- ✓ Tooltips
- ✓ Disabled states
- ✓ Badge variants (Default, Warning, Destructive)
- ✓ Consistent 28px height

#### KeyboardShortcuts

Display and organize keyboard shortcuts by category with platform-specific key formatting:

```rust
use adabraka_ui::components::keyboard_shortcuts::*;

cx.new(|_| {
    KeyboardShortcuts::new()
        .category("File", vec![
            ShortcutItem::new("New File", "cmd-n"),
            ShortcutItem::new("Open File", "cmd-o"),
            ShortcutItem::new("Save", "cmd-s"),
        ])
        .category("Edit", vec![
            ShortcutItem::new("Undo", "cmd-z"),
            ShortcutItem::new("Redo", "cmd-shift-z"),
            ShortcutItem::new("Cut", "cmd-x"),
        ])
        .category("View", vec![
            ShortcutItem::new("Toggle Sidebar", "cmd-b"),
            ShortcutItem::new("Zoom In", "cmd-="),
        ])
})

// Custom category
let category = ShortcutCategory::new("Custom", vec![
    ShortcutItem::new("Action", "ctrl-alt-k"),
]);

KeyboardShortcuts::new()
    .add_category(category)
```

**Features:**
- ✓ Organized by category
- ✓ Platform-specific key display (⌘ on macOS, Ctrl on Windows/Linux)
- ✓ Automatic key formatting (cmd-n → ⌘N)
- ✓ Clean, readable layout
- ✓ Monospace font for key bindings
- ✓ Hover effects
- ✓ Optional icons per shortcut

### Overlays

#### Command Palette

A searchable command palette (Cmd+K / Ctrl+K style) for quick access to application commands:

```rust
// Create commands
let commands = vec![
    Command::new("file.new", "New File")
        .icon(IconSource::Named("file-plus".into()))
        .description("Create a new file")
        .category("File")
        .shortcut("Cmd+N")
        .on_select(|_window, _cx| create_new_file()),

    Command::new("edit.find", "Find")
        .icon(IconSource::Named("search".into()))
        .description("Find text in current file")
        .category("Edit")
        .shortcut("Cmd+F"),
];

// Show command palette
if self.show_palette {
    CommandPalette::new(window, cx, commands)
        .on_close(|_window, _cx| {
            // Handle close
        })
}
```

**Features:**
- ✓ Fuzzy search with relevance scoring
- ✓ Command categories
- ✓ Icons and keyboard shortcuts display
- ✓ Full keyboard navigation (↑↓ arrows, Enter, Escape)
- ✓ Recent commands tracking
- ✓ Modal overlay with backdrop

### Overlays (continued)

#### Dialog

```rust
// Show dialog conditionally
div()
    .when(self.show_dialog, |this| {
        this.child(
            Dialog::new(cx)
                .title("Confirm Action")
                .content(div().child("Are you sure you want to proceed?"))
                .size(DialogSize::Md)
                .confirm_button(
                    Button::new("confirm", "Confirm")
                        .variant(ButtonVariant::Default)
                        .on_click(cx.listener(|view, _event, _window, cx| {
                            view.confirm_action(cx);
                        }))
                )
                .cancel_button(
                    Button::new("cancel", "Cancel")
                        .on_click(cx.listener(|view, _event, _window, cx| {
                            view.show_dialog = false;
                            cx.notify();
                        }))
                )
        )
    })
```

#### Popover

```rust
Popover::new(cx)
    .trigger(Button::new("open-popover", "Open Popover"))
    .content(
        VStack::new()
            .p(px(16.0))
            .gap(px(8.0))
            .child(div().child("Popover content"))
            .child(Button::new("action", "Action"))
    )
    .position(PopoverPosition::Bottom)
    .alignment(PopoverAlignment::Start)
```

#### Toast Notifications

```rust
// In your app struct
toast_manager: Entity<ToastManager>,

// Initialize
fn new(cx: &mut App) -> Self {
    let toast_manager = cx.new(|cx| ToastManager::new(cx));
    Self { toast_manager }
}

// Show toast
fn show_success(&mut self, cx: &mut Context<Self>) {
    let toast = ToastItem::new(1, "Success!")
        .description("Operation completed successfully")
        .variant(ToastVariant::Success);

    self.toast_manager.update(cx, |manager, cx| {
        manager.add_toast(toast, window, cx);
    });
}

// Render toast manager
fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
    div()
        .child(self.toast_manager.clone())
        // ... other content
}
```

### Animation System

adabraka-ui includes a professional animation system with smooth easing functions and spring physics:

```rust
use adabraka_ui::animations::*;

// Fade animations with cubic easing
div().with_animation("fade", presets::fade_in_normal(), |div, delta| {
    div.opacity(delta)
})

// Scale animations with back easing (subtle overshoot)
div().with_animation("scale", presets::scale_up(), |div, delta| {
    let scale = 0.5 + (0.5 * delta);
    div.size(px(100.0 * scale))
})

// Smooth scale without overshoot
div().with_animation("scale", presets::scale_up_smooth(), |div, delta| {
    let scale = 0.8 + (0.2 * delta);
    div.size(px(100.0 * scale))
})

// Slide animations with smooth cubic easing
div().with_animation("slide", presets::slide_in_left(), |div, delta| {
    div.ml(px(-200.0 * (1.0 - delta)))
})

// Spring-based slide (natural feeling)
div().with_animation("spring", presets::spring_slide_left(), |div, delta| {
    div.ml(px(-200.0 * (1.0 - delta)))
})

// Continuous animations
// Smooth pulse with helper function
div().with_animation("pulse", presets::pulse(), |div, delta| {
    let scale = pulse_scale(delta, 1.0, 1.15);
    div.size(px(80.0 * scale))
})

// Opacity pulse
div().with_animation("pulse-opacity", presets::pulse_slow(), |div, delta| {
    let opacity = pulse_opacity(delta, 0.4, 1.0);
    div.opacity(opacity)
})

// Shake with natural decay
div().with_animation("shake", presets::shake(), |div, delta| {
    let offset = shake_offset(delta, 12.0);
    div.ml(px(offset))
})
```

**Animation Features:**
- ✓ Professional cubic-bezier easing functions (quad, cubic, quart, expo)
- ✓ Spring physics for natural motion
- ✓ Back easing with subtle overshoot for emphasis
- ✓ Helper functions for pulse, shake, and bounce patterns
- ✓ Multiple timing presets (ultra-fast to extra-slow)
- ✓ Smooth, polished animations that feel professional

**Available Easing Functions:**
- `ease_out_cubic` - Most natural for UI (default)
- `ease_in_out_cubic` - Smooth acceleration and deceleration
- `ease_out_quad` - Gentle deceleration
- `ease_out_quart` - Very smooth deceleration
- `ease_out_expo` - Dramatic deceleration
- `ease_out_back` - Slight overshoot for emphasis
- `spring` - Natural bouncy effect
- `smooth_spring` - Subtle spring (recommended for UI)

**Animation Presets:**
- Fade: `fade_in_quick()`, `fade_in_normal()`, `fade_in_slow()`
- Scale: `scale_up()`, `scale_up_smooth()`, `scale_down()`
- Slide: `slide_in_left/right/top/bottom()`, `spring_slide_left/right()`
- Pulse: `pulse()`, `pulse_fast()`, `pulse_slow()`
- Interactive: `shake()`, `shake_strong()`, `bounce_in()`, `spring()`

### Scrolling

Scrollable containers automatically include padding to ensure content at the bottom is fully visible.

```rust
use adabraka_ui::components::scroll::*;

// Default scrolling with 24px padding (recommended)
div()
    .size_full()
    .child(
        scrollable_vertical(
            VStack::new()
                .gap(px(16.0))
                // ... many items
                .child(item1)
                .child(item2)
                // ...
        )
    )

// Custom padding amount
scrollable_vertical_with_padding(content, px(32.0))

// No padding (use carefully - items may be cut off at the bottom)
Scrollable::new(ScrollbarAxis::Vertical, content)
    .no_padding()

// Both directions
scrollable_both(content)
scrollable_both_with_padding(content, px(16.0))
```

**Features:**
- ✓ Default 24px padding prevents content from being cut off at the bottom
- ✓ Customizable padding for different use cases
- ✓ Can disable padding when needed
- ✓ Smooth, macOS-style scrollbars that auto-hide
- ✓ Support for vertical, horizontal, or both directions

## Icon System

adabraka-ui provides flexible icon support with both named icons and custom icon paths. **Note:** Icon assets are **not bundled** with the library to keep the bundle size small. You need to provide your own icon assets.

### Setting Up Icon Assets

To use icons in your application, you need to:

1. **Download icon assets** (we recommend [Lucide Icons](https://lucide.dev/) or [Heroicons](https://heroicons.com/))
2. **Configure the icon base path** in your application initialization
3. **Set up GPUI's AssetSource** to load the icons

```rust
use adabraka_ui::prelude::*;
use gpui::*;
use std::path::PathBuf;

// Define your asset source
struct Assets {
    base: PathBuf,
}

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<std::borrow::Cow<'static, [u8]>>> {
        std::fs::read(self.base.join(path))
            .map(|data| Some(std::borrow::Cow::Owned(data)))
            .map_err(|err| err.into())
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        std::fs::read_dir(self.base.join(path))
            .map(|entries| {
                entries
                    .filter_map(|entry| {
                        entry.ok().and_then(|e| {
                            e.file_name().to_str().map(|s| SharedString::from(s.to_string()))
                        })
                    })
                    .collect()
            })
            .map_err(|err| err.into())
    }
}

fn main() {
    Application::new()
        .with_assets(Assets {
            base: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
        })
        .run(|cx| {
            // Initialize the UI library
            adabraka_ui::init(cx);

            // Configure where icons are located
            // This path is relative to your CARGO_MANIFEST_DIR
            adabraka_ui::set_icon_base_path("assets/icons");

            // Install theme
            install_theme(cx, Theme::dark());

            // ... rest of your app
        });
}
```

### Using Icons

Once configured, you can use icons in two ways:

#### Named Icons

Named icons are automatically resolved using the configured base path:

```rust
use adabraka_ui::components::icon::IconSource;

// Named icons (resolved to: assets/icons/{name}.svg)
IconSource::Named("search".to_string())
IconSource::Named("home".to_string())
IconSource::Named("settings".to_string())

// Use in components
Button::new("Search")
    .prefix(IconSource::Named("search".to_string()))

SidebarItem::new("dashboard", "Dashboard")
    .with_icon(IconSource::Named("home".to_string()))

Icon::new("arrow-up")
    .size(px(24.0))
    .color(theme.tokens.primary)
```

#### Custom Icon Paths

For custom or one-off icons, use direct file paths:

```rust
// Direct file path
IconSource::FilePath("assets/custom/my-icon.svg".into())

// Absolute path
IconSource::FilePath("/path/to/icon.svg".into())

// The library automatically detects paths (contains '/' or ends with '.svg')
Icon::new("assets/custom/logo.svg")  // Treated as file path
Icon::new("search")                  // Treated as named icon
```

### Icon Requirements

Your icon SVG files should:

- Use `stroke="currentColor"` to inherit color from the component
- Have a consistent viewBox (typically `0 0 24 24`)
- Be optimized for performance

Example icon SVG:
```xml
<svg xmlns="http://www.w3.org/2000/svg"
     width="24" height="24"
     viewBox="0 0 24 24"
     fill="none"
     stroke="currentColor"
     stroke-width="2"
     stroke-linecap="round"
     stroke-linejoin="round">
  <!-- icon paths -->
</svg>
```

### Recommended Icon Sets

- **[Lucide Icons](https://lucide.dev/)** - Beautiful, consistent icons (3,000+)
- **[Heroicons](https://heroicons.com/)** - Hand-crafted by Tailwind CSS team
- **[Feather Icons](https://feathericons.com/)** - Simply beautiful open source icons
- **[Phosphor Icons](https://phosphoricons.com/)** - Flexible icon family

### Icon Bundle Size

By not bundling icons, adabraka-ui keeps its package size small (saves ~3,274 icon files). This allows you to:

- ✓ Include only the icons you actually use
- ✓ Choose your preferred icon set
- ✓ Update icons independently from the library
- ✓ Keep your application bundle optimized

### Icon Customization (New in v0.1.2)

The Icon component now supports advanced styling and transformations:

#### Named Icon Sizes

Use semantic size names instead of pixel values:

```rust
use adabraka_ui::prelude::*;

Icon::new("search")
    .size(IconSize::XSmall)   // 12px
    .size(IconSize::Small)    // 14px
    .size(IconSize::Medium)   // 16px (default)
    .size(IconSize::Large)    // 24px
    .size(IconSize::Custom(px(32.0)))  // Custom size

// Backward compatible - Pixels still work:
Icon::new("search").size(px(20.0))  // Auto-converts to Custom
```

#### Full GPUI Styling Support

Icons now implement the `Styled` trait, allowing all GPUI styling methods:

```rust
Icon::new("star")
    .size(IconSize::Large)
    .p_2()                    // Padding
    .bg(gpui::blue())         // Background color
    .rounded_md()             // Rounded corners
    .border_1()               // Border
    .border_color(gpui::gray())
```

#### Icon Rotation

Perfect for loading spinners, directional arrows, and animated icons:

```rust
use gpui::Radians;

// Rotate icon 90 degrees
Icon::new("arrow-right")
    .rotate(Radians::from_degrees(90.0))

// Animated loading spinner
Icon::new("loader")
    .rotate(Radians::TAU * progress)  // Animate with state
```

#### Clickable Icons

Icons support built-in click handlers:

```rust
Icon::new("close")
    .clickable(true)
    .on_click(|window, cx| {
        // Handle click
    })
    .size(IconSize::Large)
```

## Advanced Features

### Resizable Panels

```rust
use adabraka_ui::components::resizable::*;

h_resizable("layout", resizable_state)
    .child(resizable_panel().child(sidebar))
    .child(resizable_panel().child(main_content))
```

### Form Validation

```rust
// Input with validation
let input_state = cx.new(|cx| InputState::new(cx)
    .with_validation_rules(ValidationRules {
        required: true,
        min_length: Some(3),
        max_length: Some(50),
        pattern: Some(regex!("^[a-zA-Z0-9]+$")),
    }));

Input::new(input_state, cx)
    .placeholder("Username")
    .on_blur(|_input, _window, cx| {
        // Trigger validation on blur
        cx.notify();
    })
```

### Drag and Drop

```rust
use adabraka_ui::components::drag_drop::*;

Draggable::new("item-1")
    .child(div().child("Drag me"))
    .on_drag_start(|data, _window, _cx| {
        println!("Started dragging: {:?}", data);
    })

DropZone::new("drop-zone")
    .child(div().child("Drop here"))
    .on_drop(|dropped_data, _window, cx| {
        println!("Dropped: {:?}", dropped_data);
        // Handle the drop
    })
```

## Examples

The library includes 50+ example applications demonstrating all components and features.

### Featured Examples

```bash
# Comprehensive demo with all components
cargo run --example demo

# Full IDE-style application
cargo run --example ide_demo

# File explorer with tree navigation
cargo run --example file_explorer
```

### Component Demos

```bash
# Input & Forms
cargo run --example input_demo
cargo run --example input_validation
cargo run --example password_test
cargo run --example search_input_demo
cargo run --example color_picker_demo
cargo run --example date_picker_demo
cargo run --example combobox_demo

# Navigation
cargo run --example sidebar_demo
cargo run --example tabs_demo
cargo run --example menu_demo
cargo run --example toolbar_demo
cargo run --example app_menu_demo
cargo run --example status_bar_demo
cargo run --example navigation_menu_demo

# Display
cargo run --example data_table_demo
cargo run --example card_demo
cargo run --example accordion_demo
cargo run --example progress_demo
cargo run --example text_demo

# Overlays
cargo run --example overlays_demo
cargo run --example command_palette_demo

# Advanced
cargo run --example editor_demo
cargo run --example drag_drop_demo
cargo run --example animations_demo
cargo run --example transitions_demo
cargo run --example virtual_list_demo

# Layout
cargo run --example layout_demo
cargo run --example complex_layout_demo
cargo run --example scroll_test

# Icons & Assets
cargo run --example icon_showcase
cargo run --example keyboard_shortcuts_demo

# Trees & Lists
cargo run --example tree_list_demo
cargo run --example tree_performance_demo
```

To see all available examples:
```bash
cargo run --example
```

## Architecture

adabraka-ui is built on top of GPUI with these key principles:

- **Entity-based state management** for complex interactive components
- **Immutable configuration** using the builder pattern
- **Type-safe APIs** with comprehensive Rust types
- **Performance-first** with efficient rendering and minimal allocations
- **Accessible** with proper ARIA labels and keyboard navigation

## Contributing

We welcome contributions from the community! Whether you're fixing bugs, adding features, or improving documentation, your help is appreciated.

### Quick Start

1. **Read the [Contributing Guidelines](CONTRIBUTING.md)** for detailed information
2. **Check existing [issues](https://github.com/Augani/adabraka-ui/issues)** and [pull requests](https://github.com/Augani/adabraka-ui/pulls)
3. **Fork the repository** and create your feature branch
4. **Follow our coding guidelines** and add tests
5. **Submit a pull request** using our template

### Reporting Issues

- **Bug Reports**: Use our [bug report template](.github/ISSUE_TEMPLATE/bug_report.md)
- **Feature Requests**: Use our [feature request template](.github/ISSUE_TEMPLATE/feature_request.md)
- **Questions**: Start a [discussion](https://github.com/Augani/adabraka-ui/discussions)

### Development Guidelines

- Follow Rust best practices and idioms
- Use `cargo fmt` for formatting and `cargo clippy` for linting
- Write tests for new functionality
- Update documentation and examples
- Ensure all examples compile and run

For detailed guidelines, see [CONTRIBUTING.md](CONTRIBUTING.md).

## Acknowledgments

This project wouldn't be possible without the amazing work of:

- **[Zed Industries](https://zed.dev/)** - For creating [GPUI](https://github.com/zed-industries/zed), the powerful GPU-accelerated UI framework that powers the Zed editor and makes adabraka-ui possible
- **[Lucide Icons](https://lucide.dev/)** - For providing the beautiful, consistent icon set used throughout the component library
- **[shadcn/ui](https://ui.shadcn.com/)** - For the design inspiration and component patterns

Special thanks to the Zed team for open-sourcing GPUI and making it available for building desktop applications in Rust.

## License

MIT License - see LICENSE file for details.

---

Built with ❤️ using GPUI and inspired by shadcn/ui