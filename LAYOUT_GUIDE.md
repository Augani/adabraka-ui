# Layout Components Guide

Comprehensive guide to building complex desktop application layouts with adabraka-ui's layout components.

## Table of Contents

1. [Quick Start](#quick-start)
2. [Component Overview](#component-overview)
3. [Common Patterns](#common-patterns)
4. [Advanced Techniques](#advanced-techniques)
5. [Performance Tips](#performance-tips)
6. [Best Practices](#best-practices)

---

## Quick Start

### Basic Stack Layout

```rust
use adabraka_ui::layout::*;

// Vertical stack with spacing
VStack::new()
    .spacing(16.0)
    .w_full()
    .child(heading)
    .child(content)
    .child(footer)

// Horizontal stack with justification
HStack::new()
    .spacing(8.0)
    .justify(Justify::Between)
    .items_center()
    .child(left_content)
    .child(right_content)
```

### Auto-Scrolling Container

```rust
// Simple vertical scroll - auto-generates unique ID
ScrollContainer::vertical()
    .h(px(400.0))
    .child(long_content())

// With programmatic control
let scroll_handle = ScrollHandle::new();
ScrollContainer::vertical()
    .track_scroll(&scroll_handle)
    .child(content())

// Later: scroll programmatically
scroll_handle.scroll_to_bottom();
scroll_handle.set_offset(point(px(0.0), px(100.0)));
```

---

## Component Overview

### Stack Layouts

#### VStack (Vertical Stack)

Arranges children vertically with consistent spacing.

```rust
VStack::new()
    .spacing(12.0)           // Gap between children
    .align(Align::Center)    // Cross-axis alignment
    .fill_width()            // Take full available width
    .padding(16.0)           // All-sides padding
    .children(items)
```

**Common Use Cases:**
- Form layouts
- Vertical navigation menus
- List items
- Card content

#### HStack (Horizontal Stack)

Arranges children horizontally with spacing and justification.

```rust
HStack::new()
    .spacing(8.0)
    .justify(Justify::Between)    // Main-axis justification
    .align(Align::Center)         // Cross-axis alignment
    .fill_width()
    .children(items)
```

**Common Use Cases:**
- Toolbars and action bars
- Button groups
- Horizontal navigation
- Header layouts

### Scrolling Components

#### ScrollContainer

Auto-managed scrolling with unique IDs.

```rust
// Vertical scroll (most common)
ScrollContainer::vertical()
    .h(px(500.0))
    .child(content)

// Horizontal scroll
ScrollContainer::horizontal()
    .w(px(800.0))
    .child(wide_content)

// Both directions
ScrollContainer::both()
    .size(px(800.0), px(600.0))
    .child(large_content)

// Custom ID (overrides auto-generation)
ScrollContainer::vertical()
    .id("my-scroll-area")
    .child(content)
```

**Key Features:**
- ✅ Auto-generates unique IDs (no manual ID management)
- ✅ Optional scroll handle for programmatic control
- ✅ Works with any content (single child or nested layouts)

#### ScrollList

Optimized for vertical lists - combines ScrollContainer + VStack.

```rust
ScrollList::new()
    .spacing(8.0)
    .align(Align::Start)
    .h(px(400.0))
    .children(
        messages.iter().map(|msg| render_message(msg))
    )
```

**When to Use:**
- Message lists
- Comment threads
- Activity feeds
- Any vertical list of items

### Container Components

#### Panel

General-purpose container with styling presets.

```rust
// Card style (border, rounded, padding)
Panel::new()
    .card()
    .bg(theme.surface)
    .child(content)

// Section style (border-bottom, padding)
Panel::new()
    .section()
    .child(section_content)

// Custom styling
Panel::new()
    .border()
    .rounded()
    .padded()
    .bg(theme.background)
    .child(content)
```

**Presets:**
- `.card()` - Border + rounded corners + padding
- `.section()` - Border bottom + padding
- `.elevated()` - Border + rounded (shadow effect)

#### Container

Centered container with max-width constraints.

```rust
// Responsive presets
Container::sm()   // 640px max-width, centered
Container::md()   // 768px max-width, centered
Container::lg()   // 1024px max-width, centered
Container::xl()   // 1280px max-width, centered
Container::xxl()  // 1536px max-width, centered

// Custom max-width
Container::new()
    .max_w(px(1200.0))
    .centered()
    .px(px(24.0))
    .child(page_content)
```

**Use Cases:**
- Page content areas
- Centered layouts
- Responsive content widths

### Flexible Layouts

#### Flow

Auto-wrapping layout for tags, badges, chips.

```rust
Flow::new()
    .direction(FlowDirection::Horizontal)
    .spacing(8.0)
    .children(
        tags.iter().map(|tag| Badge::new(tag))
    )
```

#### Grid

Fixed-column grid with automatic row wrapping.

```rust
Grid::new()
    .columns(3)
    .gap(16.0)
    .children(
        items.iter().map(|item| GridItem::new(item))
    )
```

### Utilities

#### Spacer

Flexible spacing that expands to fill available space.

```rust
HStack::new()
    .child(left_button)
    .child(Spacer::new())      // Push to edges
    .child(right_button)

// Fixed-size spacer
VStack::new()
    .child(content)
    .child(Spacer::fixed(px(24.0)))  // 24px gap
    .child(footer)
```

#### Cluster

Inline grouping for tightly related items.

```rust
Cluster::new()
    .spacing(4.0)
    .align(Align::Center)
    .child(Icon::user())
    .child(Text::new("John Doe"))
    .child(Badge::new("Admin"))
```

---

## Common Patterns

### App Layout: Sidebar + Main Content

```rust
HStack::new()
    .size_full()
    .child(
        // Sidebar
        Panel::new()
            .border()
            .w(px(240.0))
            .h_full()
            .child(sidebar_content)
    )
    .child(
        // Main content area
        VStack::new()
            .grow()
            .child(toolbar)
            .child(
                ScrollContainer::vertical()
                    .grow()
                    .child(main_content)
            )
            .child(status_bar)
    )
```

### Dashboard Grid Layout

```rust
Container::xl()
    .px(px(24.0))
    .py(px(24.0))
    .child(
        Grid::new()
            .columns(3)
            .gap(16.0)
            .children(
                stats.iter().map(|stat| {
                    Panel::new()
                        .card()
                        .child(stat_card(stat))
                })
            )
    )
```

### Chat Interface

```rust
VStack::new()
    .size_full()
    .child(
        // Header
        Panel::new()
            .section()
            .child(chat_header)
    )
    .child(
        // Messages (scrollable)
        ScrollList::new()
            .id("chat-messages")
            .spacing(12.0)
            .grow()
            .px(px(16.0))
            .track_scroll(&scroll_handle)
            .children(
                messages.iter().map(|msg| message_bubble(msg))
            )
    )
    .child(
        // Input area
        Panel::new()
            .border()
            .p(px(12.0))
            .child(message_input)
    )
```

### Settings Panel

```rust
Container::md()
    .px(px(24.0))
    .child(
        VStack::new()
            .spacing(24.0)
            .child(
                Panel::new()
                    .card()
                    .child(
                        VStack::new()
                            .spacing(12.0)
                            .child(section_title("Account"))
                            .child(account_settings)
                    )
            )
            .child(
                Panel::new()
                    .card()
                    .child(
                        VStack::new()
                            .spacing(12.0)
                            .child(section_title("Privacy"))
                            .child(privacy_settings)
                    )
            )
    )
```

### Three-Column Layout

```rust
HStack::new()
    .size_full()
    .spacing(0.0)
    .child(
        // Left sidebar
        Panel::new()
            .w(px(200.0))
            .h_full()
            .border()
            .child(navigation)
    )
    .child(
        // Middle content
        VStack::new()
            .grow()
            .border()
            .child(content)
    )
    .child(
        // Right sidebar
        Panel::new()
            .w(px(280.0))
            .h_full()
            .border()
            .child(info_panel)
    )
```

---

## Advanced Techniques

### Programmatic Scrolling

```rust
struct MyView {
    scroll_handle: ScrollHandle,
}

impl MyView {
    fn scroll_to_message(&self, index: usize) {
        // Scroll to specific item
        self.scroll_handle.scroll_to_item(index);
    }

    fn scroll_to_top(&self) {
        self.scroll_handle.set_offset(point(px(0.0), px(0.0)));
    }

    fn scroll_to_bottom(&self) {
        self.scroll_handle.scroll_to_bottom();
    }

    fn get_visible_range(&self) -> (usize, usize) {
        (
            self.scroll_handle.top_item(),
            self.scroll_handle.bottom_item()
        )
    }
}
```

### Responsive Layouts

```rust
// Use Container presets for responsive widths
fn render_page(&self, cx: &mut Context<Self>) -> impl IntoElement {
    Container::lg()  // Max 1024px, auto-centered
        .px(px(24.0))
        .child(
            VStack::new()
                .spacing(24.0)
                .child(header)
                .child(content)
        )
}
```

### Nested Scrolling

```rust
// Outer scroll container
ScrollContainer::vertical()
    .h_full()
    .child(
        VStack::new()
            .spacing(16.0)
            .child(section_1)
            .child(
                // Inner scroll container (horizontal)
                ScrollContainer::horizontal()
                    .h(px(300.0))
                    .child(wide_gallery)
            )
            .child(section_2)
    )
```

### Dynamic Lists with Scroll Management

```rust
impl MyListView {
    fn add_item(&mut self, item: Item, cx: &mut Context<Self>) {
        self.items.push(item);
        
        // Auto-scroll to bottom when new item added
        cx.defer(|this, _| {
            this.scroll_handle.scroll_to_bottom();
        });
        
        cx.notify();
    }

    fn remove_item(&mut self, index: usize, cx: &mut Context<Self>) {
        self.items.remove(index);
        
        // Maintain scroll position
        let current_top = self.scroll_handle.top_item();
        cx.defer(move |this, _| {
            this.scroll_handle.scroll_to_item(current_top);
        });
        
        cx.notify();
    }
}
```

---

## Performance Tips

### 1. Use ScrollList for Long Lists

```rust
// ✅ Good: Optimized for lists
ScrollList::new()
    .spacing(8.0)
    .children(items.iter().map(|item| render(item)))

// ❌ Avoid: Manual composition for simple lists
ScrollContainer::vertical()
    .child(
        VStack::new()
            .spacing(8.0)
            .children(items.iter().map(|item| render(item)))
    )
```

### 2. Reuse Scroll Handles

```rust
// ✅ Good: Single handle per scroll area
struct MyView {
    scroll_handle: ScrollHandle,
}

// ❌ Avoid: Creating new handles on every render
fn render(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
    let handle = ScrollHandle::new();  // Don't do this!
    ScrollContainer::vertical()
        .track_scroll(&handle)
}
```

### 3. Defer Scroll Operations

```rust
// ✅ Good: Defer scroll after layout
fn handle_action(&mut self, cx: &mut Context<Self>) {
    self.update_data();
    
    cx.defer(|this, _| {
        this.scroll_handle.scroll_to_item(index);
    });
    
    cx.notify();
}
```

---

## Best Practices

### 1. Component Composition

Build complex layouts from simple primitives:

```rust
// ✅ Good: Composable, reusable
fn render_card(&self, title: &str, content: impl IntoElement) -> impl IntoElement {
    Panel::new()
        .card()
        .child(
            VStack::new()
                .spacing(12.0)
                .child(div().font_weight(FontWeight::BOLD).child(title))
                .child(content)
        )
}

// Use it:
render_card("Settings", settings_form())
render_card("Profile", profile_content())
```

### 2. Consistent Spacing

Use a spacing scale for visual harmony:

```rust
const SPACING_XS: f32 = 4.0;
const SPACING_SM: f32 = 8.0;
const SPACING_MD: f32 = 12.0;
const SPACING_LG: f32 = 16.0;
const SPACING_XL: f32 = 24.0;

VStack::new()
    .spacing(SPACING_MD)
    .child(item1)
    .child(item2)
```

### 3. Semantic Layout Structure

Use descriptive variable names and comments:

```rust
// ✅ Good: Clear structure
let toolbar = render_toolbar();
let main_content = render_content_area();
let sidebar = render_sidebar();

HStack::new()
    .child(sidebar)
    .child(
        VStack::new()
            .child(toolbar)
            .child(main_content)
    )
```

### 4. Extract Complex Layouts

Break down complex layouts into methods:

```rust
impl MyView {
    fn render_header(&self) -> impl IntoElement { /* ... */ }
    fn render_sidebar(&self) -> impl IntoElement { /* ... */ }
    fn render_content(&self) -> impl IntoElement { /* ... */ }
    fn render_footer(&self) -> impl IntoElement { /* ... */ }
}

impl Render for MyView {
    fn render(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        VStack::new()
            .child(self.render_header())
            .child(
                HStack::new()
                    .child(self.render_sidebar())
                    .child(self.render_content())
            )
            .child(self.render_footer())
    }
}
```

### 5. Use Auto-IDs for Scroll Containers

Let the library manage IDs automatically:

```rust
// ✅ Good: Auto-managed IDs
ScrollContainer::vertical()
    .child(content)

// Only use custom IDs when you need specific identification
ScrollContainer::vertical()
    .id("main-message-list")  // Explicit when needed
    .child(content)
```

---

## Migration from Raw Div

If you're using raw `div()` elements, here's how to migrate:

### Before (Raw Div)

```rust
div()
    .flex()
    .flex_col()
    .gap(px(16.0))
    .items_center()
    .w_full()
    .child(item1)
    .child(item2)
```

### After (VStack)

```rust
VStack::new()
    .spacing(16.0)
    .align(Align::Center)
    .fill_width()
    .child(item1)
    .child(item2)
```

### Before (Manual Scroll)

```rust
let id = ElementId::Name("my-scroll-1".into());
div()
    .id(id)
    .overflow_y_scroll()
    .h(px(400.0))
    .child(content)
```

### After (ScrollContainer)

```rust
ScrollContainer::vertical()
    .h(px(400.0))
    .child(content)
```

---

## Troubleshooting

### Scroll Container Not Scrolling

**Problem:** Content doesn't scroll even with `ScrollContainer`.

**Solution:** Ensure the ScrollContainer has a fixed height:

```rust
// ❌ Won't scroll (no height constraint)
ScrollContainer::vertical()
    .child(content)

// ✅ Will scroll
ScrollContainer::vertical()
    .h(px(400.0))  // Fixed height
    .child(content)

// ✅ Or use grow in a flex container
VStack::new()
    .size_full()
    .child(header)
    .child(
        ScrollContainer::vertical()
            .grow()  // Takes remaining space
            .child(content)
    )
```

### Items Not Spacing Correctly

**Problem:** `spacing()` doesn't seem to work.

**Solution:** Make sure you're using the right container:

```rust
// ❌ spacing() doesn't exist on div
div()
    .flex()
    .flex_col()
    .spacing(16.0)  // Error!

// ✅ Use VStack
VStack::new()
    .spacing(16.0)  // Works!
    .children(items)
```

### Programmatic Scroll Not Working

**Problem:** `scroll_handle.scroll_to_item()` doesn't work.

**Solution:** Defer scroll operations until after layout:

```rust
// ❌ Immediate scroll (before layout)
fn handle_click(&mut self, cx: &mut Context<Self>) {
    self.scroll_handle.scroll_to_item(10);
    cx.notify();
}

// ✅ Deferred scroll (after layout)
fn handle_click(&mut self, cx: &mut Context<Self>) {
    cx.defer(|this, _| {
        this.scroll_handle.scroll_to_item(10);
    });
    cx.notify();
}
```

---

## Additional Resources

- [GPUI Documentation](https://github.com/zed-industries/zed/tree/main/crates/gpui/docs)
- [Complex Layout Example](./examples/complex_layout_demo.rs)
- [API Reference](./src/layout.rs)

---

**Happy building! 🚀**

