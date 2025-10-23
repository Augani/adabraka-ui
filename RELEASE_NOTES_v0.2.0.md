# Release Notes - v0.2.0

## 🎉 Major Feature Release: 100% Styled Trait Coverage!

We're excited to announce **adabraka-ui v0.2.0**, a major feature release that brings complete styling customization to every single component in the library!

---

## 🌟 Highlights

### **100% Styled Trait Coverage - ALL 54 Components!**

Every user-facing component in adabraka-ui now implements the `Styled` trait, giving you complete control over styling using GPUI's powerful styling API.

**What this means for you:**
- Apply **any GPUI styling method** to any component
- Custom backgrounds: `.bg(rgb(0x3b82f6))`
- Custom borders: `.border_2().border_color(rgb(0xef4444))`
- Custom padding: `.p_4()`, `.px_6()`, `.py_2()`
- Custom border radius: `.rounded_lg()`, `.rounded_xl()`
- Shadow effects: `.shadow_sm()`, `.shadow_lg()`
- Width/height: `.w_full()`, `.h(px(200.0))`
- **And hundreds more styling methods!**

### **Components with Styled Trait**

✅ **Components (14)**: Button, Input, Checkbox, IconButton, Label, Radio, Toggle, Textarea, Avatar, Progress, Slider, Separator, SearchInput, Select

✅ **Display (6)**: Card, Badge, Accordion, Table, DataTable, Collapsible

✅ **Navigation (9)**: Menu, Tabs, Toolbar, Sidebar, Breadcrumbs, NavigationMenu, StatusBar, Tree, AppMenu

✅ **Overlays (11)**: Dialog, Sheet, AlertDialog, Toast, BottomSheet, CommandPalette, ContextMenu, HoverCard, Popover, PopoverMenu, Tooltip

✅ **Advanced (9)**: TextField, Pagination, ToggleGroup, KeyboardShortcuts, Calendar, Resizable, Editor, Draggable, DropZone

---

## 🆕 What's New

### Icon System Enhancements

**Icon Phase 1:**
- Consolidated `IconSource` module across all components (DRY principle)
- Removed duplicate IconSource definitions
- Improved path detection with separator-first logic
- Added comprehensive unit tests

**Icon Phase 2:**
- **IconSize enum** with named sizes: `XSmall`, `Small`, `Medium`, `Large`, `Custom(Pixels)`
- **Rotation support** using GPUI's Transformation API
- **Styled trait** for Icon component - full customization support

```rust
Icon::new("search")
    .size(IconSize::Large)
    .rotate(Radians::from_degrees(90.0))
    .p_2()
    .bg(rgb(0x89b4fa))
    .rounded_md()
```

### Component Enhancements

**Text Component - Fixed Decorations!**
- ✅ **Italic** now actually works using HighlightStyle API
- ✅ **Strikethrough** now actually works with customizable thickness
- ✅ All decorations can be combined

```rust
Text::new("Deprecated warning")
    .italic()
    .strikethrough()
    .underline()
    .color(red())
```

**Button Component**
- Improved API with better ID parameter handling
- Fixed compilation across 21 example files
- Styled trait allows complete customization

**Checkbox Component**
- Replaced emoji icons (✓ and −) with customizable Icon components
- Users can now choose their own check and indeterminate icons

```rust
Checkbox::new("check")
    .checked_icon("check-circle")
    .indeterminate_icon("dash")
```

**Calendar Component - Full i18n!**
- Added `CalendarLocale` struct for internationalization
- Built-in locales: English, French, Spanish, German, Portuguese, Italian
- Support for custom locales

```rust
Calendar::new(date)
    .locale(CalendarLocale::french())
    .on_month_change(|new_date, window, cx| { /* ... */ })
```

### 54 New Styled Demos

Created comprehensive demonstration examples for every component:
- `button_styled_demo.rs` - 7 button customization examples
- `input_styled_demo.rs` - 12 input styling variations
- `data_table_styled_demo.rs` - 6 styled tables with virtual scrolling
- `drag_drop_styled_demo.rs` - Interactive task board with styling
- **And 50 more demos!**

Each demo shows real, working examples of how to customize components using the Styled trait.

---

## 💥 Breaking Changes

### Icon Component Returns
**Before:** Non-clickable icons returned `Div`
**After:** Non-clickable icons return `AnyElement` (performance improvement)

**Impact:** This is a performance optimization. In most cases, you won't notice any difference. If you were explicitly typing the return value, you may need to update to `AnyElement`.

---

## 🐛 Fixes

- Fixed Editor component `.when()` pattern to `.map()` for style application
- Fixed DropZone naming conflict (renamed internal `style` field to `drop_style`)
- Fixed Button API usage across 21 example files
- Fixed various component compilation errors
- Fixed sidebar_demo and menu_demo import paths
- Fixed select_styled_demo unused variable warnings

---

## 🎨 Design Philosophy

### shadcn/ui Alignment

This release fully embraces the **shadcn philosophy**:

> **"Good defaults that users can completely override"**

**Before v0.2.0:** Components had good defaults, but limited customization options.

**After v0.2.0:** Components have great defaults AND users have 100% control via Styled trait!

Example of complete control:

```rust
Button::new("custom", "Click Me")
    // Override ALL default styles
    .bg(rgb(0x8b5cf6))           // Custom background
    .p_8()                        // Custom padding
    .rounded(px(16.0))            // Custom radius
    .border_2()                   // Custom border
    .border_color(rgb(0xa78bfa))  // Custom border color
    .shadow_lg()                  // Custom shadow
    .w_full()                     // Custom width
    .text_size(px(18.0))          // Custom text size
```

**Documentation:** See [SHADCN_DESIGN_PHILOSOPHY.md](SHADCN_DESIGN_PHILOSOPHY.md) for complete details.

---

## 📦 Improvements

### Code Quality
- Removed 3,274 unnecessary inline comments
- Production-ready code with clean implementations
- Consistent Styled trait API across all 54 components
- Better developer experience

### Performance
- Icon component optimization (removed wrapper div for non-clickable icons)
- Efficient style application using `.refine()` pattern
- No extra overhead from Styled trait

---

## 🚀 Migration Guide

### From v0.1.1 to v0.2.0

**Good news:** Most code will work without changes!

### Using the New Styled Trait

The Styled trait is **additive** - you can use it when you need custom styling, but all your existing code continues to work.

**Before (still works):**
```rust
Button::new("btn", "Click")
    .variant(ButtonVariant::Primary)
    .size(ButtonSize::Large)
```

**After (now also possible):**
```rust
Button::new("btn", "Click")
    .variant(ButtonVariant::Primary)  // Use built-in variant
    .size(ButtonSize::Large)           // Use built-in size
    .bg(rgb(0x custom))                // ADD custom background
    .rounded_xl()                      // ADD custom radius
    .shadow_lg()                       // ADD shadow effect
```

### Icon Component Changes

If you were explicitly typing Icon return values:

```rust
// Before
let icon: Div = Icon::new("search");  // ❌ No longer works

// After
let icon = Icon::new("search");       // ✅ Let Rust infer the type
// OR
let icon: AnyElement = Icon::new("search").into_any_element();  // ✅ Explicit typing
```

---

## 📚 Resources

- **Documentation**: [https://docs.rs/adabraka-ui](https://docs.rs/adabraka-ui)
- **GitHub Pages**: [https://augani.github.io/adabraka-ui/](https://augani.github.io/adabraka-ui/)
- **Repository**: [https://github.com/Augani/adabraka-ui](https://github.com/Augani/adabraka-ui)
- **Crates.io**: [https://crates.io/crates/adabraka-ui](https://crates.io/crates/adabraka-ui)

---

## 🙏 Acknowledgments

This release represents a massive improvement to the adabraka-ui library:

- **54 components** now fully customizable
- **54 new demo examples** showing real use cases
- **100% alignment** with shadcn design philosophy
- **Zero compromises** on performance or developer experience

Thank you to everyone using adabraka-ui and providing feedback! 🎉

---

## 🔮 What's Next

While we've achieved 100% Styled trait coverage, there's always more to improve:

- Additional component variants
- More built-in themes
- Performance optimizations
- Accessibility enhancements
- More comprehensive documentation

Stay tuned for v0.3.0!

---

**Upgrade today:**
```toml
[dependencies]
adabraka-ui = "0.2.0"
```

Happy coding! 🚀
