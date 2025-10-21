# Layout Components Quick Reference

Quick reference card for adabraka-ui layout components.

---

## Stack Layouts

### VStack (Vertical)
```rust
VStack::new()
    .spacing(16.0)          // Gap between items
    .align(Align::Center)   // Start | Center | End | Stretch
    .fill()                 // Fill width & height
    .fill_width()           // Fill width only
    .fill_height()          // Fill height only
    .grow()                 // Flex grow
    .padding(16.0)          // All sides padding
    .items_center()         // Shorthand for align(Center)
    .child(item)
```

### HStack (Horizontal)
```rust
HStack::new()
    .spacing(8.0)
    .align(Align::Center)
    .justify(Justify::Between)  // Start | Center | End | Between | Around | Evenly
    .fill()
    .items_center()
    .space_between()            // Shorthand for justify(Between)
    .child(item)
```

---

## Scrolling

### ScrollContainer
```rust
// Automatic ID generation
ScrollContainer::vertical()
    .h(px(400.0))
    .child(content)

ScrollContainer::horizontal()
    .w(px(800.0))
    .child(content)

ScrollContainer::both()
    .size(px(800.0), px(600.0))
    .child(content)

// With scroll handle
let handle = ScrollHandle::new();
ScrollContainer::vertical()
    .track_scroll(&handle)
    .child(content)

// Custom ID
ScrollContainer::vertical()
    .id("my-scroll")
    .child(content)
```

### ScrollList
```rust
ScrollList::new()
    .spacing(8.0)
    .align(Align::Start)
    .h(px(400.0))
    .track_scroll(&handle)
    .children(items)
```

---

## Containers

### Panel
```rust
// Presets
Panel::new().card()       // border + rounded + padding
Panel::new().section()    // border-bottom + padding
Panel::new().elevated()   // border + rounded

// Manual
Panel::new()
    .border()
    .rounded()
    .padded()
    .child(content)
```

### Container
```rust
// Responsive presets
Container::sm()    // 640px max-width, centered
Container::md()    // 768px max-width, centered
Container::lg()    // 1024px max-width, centered
Container::xl()    // 1280px max-width, centered
Container::xxl()   // 1536px max-width, centered

// Custom
Container::new()
    .max_w(px(1200.0))
    .centered()
    .px(px(24.0))
    .child(content)
```

---

## Flexible Layouts

### Flow
```rust
Flow::new()
    .direction(FlowDirection::Horizontal)  // Horizontal | Vertical
    .spacing(8.0)
    .align(Align::Center)
    .children(items)
```

### Grid
```rust
Grid::new()
    .columns(3)
    .gap(16.0)
    .children(items)
```

---

## Utilities

### Spacer
```rust
// Flexible (expands to fill)
Spacer::new()

// Fixed size
Spacer::fixed(px(24.0))
```

### Cluster
```rust
Cluster::new()
    .spacing(4.0)
    .align(Align::Center)
    .children(items)
```

---

## Alignment & Justification

### Align (Cross-axis)
- `Align::Start` - Align to start
- `Align::Center` - Center items
- `Align::End` - Align to end
- `Align::Stretch` - Stretch to fill (default)

### Justify (Main-axis)
- `Justify::Start` - Pack at start
- `Justify::Center` - Center items
- `Justify::End` - Pack at end
- `Justify::Between` - Space between
- `Justify::Around` - Space around
- `Justify::Evenly` - Space evenly

---

## ScrollHandle API

```rust
let handle = ScrollHandle::new();

// Get state
handle.offset()                    // Current offset
handle.max_offset()                // Maximum scroll
handle.top_item()                  // Top visible item index
handle.bottom_item()               // Bottom visible item index
handle.bounds()                    // Container bounds
handle.bounds_for_item(ix)         // Item bounds
handle.logical_scroll_top()        // (index, offset)
handle.children_count()            // Number of children

// Scroll programmatically
handle.scroll_to_item(ix)          // Scroll to make item visible
handle.scroll_to_top_of_item(ix)   // Scroll item to top
handle.scroll_to_bottom()          // Scroll to bottom
handle.set_offset(point)           // Set explicit offset
```

---

## Common Patterns Cheat Sheet

### App Layout
```rust
HStack::new()
    .size_full()
    .child(sidebar)              // Left sidebar
    .child(
        VStack::new()
            .grow()
            .child(toolbar)      // Top toolbar
            .child(content)      // Main content (grows)
            .child(status_bar)   // Bottom status bar
    )
```

### Toolbar
```rust
HStack::new()
    .space_between()
    .items_center()
    .px(px(16.0))
    .py(px(8.0))
    .border_b_1()
    .child(left_section)
    .child(Spacer::new())
    .child(right_section)
```

### Card
```rust
Panel::new()
    .card()
    .bg(theme.surface)
    .child(
        VStack::new()
            .spacing(12.0)
            .child(header)
            .child(content)
            .child(actions)
    )
```

### Scrollable List
```rust
ScrollList::new()
    .spacing(8.0)
    .h(px(400.0))
    .px(px(16.0))
    .children(
        items.iter().map(|item| render_item(item))
    )
```

### Centered Page
```rust
Container::lg()
    .px(px(24.0))
    .py(px(32.0))
    .child(
        VStack::new()
            .spacing(24.0)
            .child(page_header)
            .child(page_content)
    )
```

### Split View
```rust
HStack::new()
    .size_full()
    .child(
        Panel::new()
            .w(px(300.0))
            .h_full()
            .border()
            .child(left_pane)
    )
    .child(
        VStack::new()
            .grow()
            .child(right_pane)
    )
```

---

## Styling Tips

### Spacing Scale
```rust
4.0   // XS - Tight spacing
8.0   // SM - Compact spacing
12.0  // MD - Normal spacing
16.0  // LG - Comfortable spacing
24.0  // XL - Spacious sections
32.0  // 2XL - Major sections
```

### Container Widths
```rust
640px   // sm - Mobile-first
768px   // md - Tablet
1024px  // lg - Desktop
1280px  // xl - Wide desktop
1536px  // xxl - Ultra-wide
```

### Common Heights
```rust
32.0   // Button height
40.0   // Input height
48.0   // Touch-friendly
400.0  // Scrollable list
600.0  // Large content area
```

---

## Key Differences from Raw Div

| Raw Div | Layout Component | Benefit |
|---------|------------------|---------|
| `.flex().flex_col().gap()` | `VStack::new().spacing()` | Cleaner API |
| `.flex().flex_row().justify_between()` | `HStack::new().space_between()` | Semantic |
| `.id(ElementId::Name(...)).overflow_y_scroll()` | `ScrollContainer::vertical()` | Auto ID |
| Manual composition | `ScrollList::new()` | Optimized |
| `.border_1().rounded().p()` | `Panel::new().card()` | Reusable |

---

## Import

```rust
use adabraka_ui::layout::*;
use adabraka_ui::prelude::*;
```

---

**Pro Tip:** Start with high-level components (`ScrollList`, `Panel`, `Container`) and only drop down to lower-level primitives (`VStack`, `HStack`) when you need more control.

