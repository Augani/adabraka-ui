# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0] - 2026-02-06

### Added - Major Release: crates.io Publishing & GPUI Fork Enhancements

#### Published to crates.io
- All 12 adabraka-gpui ecosystem crates published under `adabraka_*` namespace
- adabraka-ui v0.3.0 published - `cargo add adabraka-ui` now works with no git deps
- Proper Zed attribution in all crate descriptions

#### GPUI Fork Enhancements (adabraka-gpui v0.3.0)
- **Inset shadows**: `BoxShadow.inset` field supported across Metal, WGSL, HLSL shader backends
- **Letter spacing**: `TextStyle.letter_spacing` with `tracking_tight()`, `tracking_wide()` convenience methods
- **Animation cancellation**: `AnimationHandle` and `with_cancellable_animation()` for interruptible animations
- **Squircle corners**: `continuous_corners` on `Styled` for iOS-style superellipse corners
- **Text shadow**: `TextShadow` struct with `text_shadow_sm/md/lg()` presets

#### New Components
- **Form** (`form.rs`) - Declarative form builder with validation, field groups, submit
- **InfiniteScroll** (`infinite_scroll.rs`) - Auto-loading paginated data on scroll
- **SortableList** (`sortable_list.rs`) - Drag-to-reorder list with smooth animations
- **DataGrid** (`data_grid.rs`) - Spreadsheet-style grid with inline editing, sorting, column resize
- **Animation Builder** (`animate.rs`) - Preset animations, keyframes, stagger, transitions

#### Animation & Polish System (v2)
- Exit animations for all overlay components (dismissing state + timer pattern)
- Input shake on validation failure (`trigger_shake()` on InputState)
- Ripple effect on Button and IconButton (`.ripple(true)`)
- 30+ easing functions: cubic-bezier, steps, elastic, back, circ, expo, quint
- Spring physics engine (`spring.rs`)
- Gesture recognizers (`gestures.rs`) - swipe, pinch, long-press
- Content transitions (`content_transition.rs`) - animated content swapping
- Responsive utilities (`responsive.rs`) - breakpoint-aware layouts
- Animation coordinator (`animation_coordinator.rs`) - multi-element orchestration
- ScrollPhysics integration with momentum scrolling in layout.rs + virtual_list.rs
- Smooth scroll-to: `scroll_to_y_animated`, `scroll_to_x_animated`
- Elevation shadows, inset shadows, layered gradients in theme tokens

#### Developer Experience
- GPUI re-exports via `gpui_ext.rs`
- `StyledExt` trait: `.center()`, `.stack()`, `.row()`, `.glass()`, `.elevated()`, `.ring()`
- Spacing, duration, z-index tokens on all 18 themes
- Expanded prelude with 15+ additional exports
- Init fixes for Sheet and AlertDialog components

### Changed
- **BREAKING**: GPUI dependency changed from `gpui` to `adabraka-gpui` (fork with enhancements)
- Version bumped from 0.2.4 to 0.3.0 to reflect ecosystem publishing
- Component count increased from 80+ to 85+

## [0.2.4] - 2026-01-15

### Added
- AudioPlayer component with real audio playback via rodio
- VideoPlayer component with flexible video backend integration
- Rating component with half-star support
- Sparkline component (Line, Bar, Area variants)
- MentionInput component with @mention dropdown
- MasonryGrid layout component
- Countdown timer component
- Optional `audio` feature flag for real audio playback

## [0.2.3] - 2025-11-13

### Added
- Vertical slider orientation support
  - New `SliderAxis` enum with `Horizontal` and `Vertical` variants
  - `.vertical()` and `.horizontal()` builder methods
  - Separate rendering logic for horizontal and vertical orientations
  - Vertical sliders grow from bottom to top (0% at bottom, 100% at top)
  - Adaptive thumb shape: horizontal oval for horizontal sliders, vertical oval for vertical sliders
  - `update_from_position_vertical()` method for proper vertical drag handling

### Fixed
- Slider thumb vertical centering
  - Container height now matches thumb height for proper alignment
  - Thumb positioned at `top: 0` instead of calculated offset
  - Track perfectly centered within container using flex layout
  - Thumb now sits centered on the track line instead of above it

### Improved
- Slider component architecture with separate `render_horizontal()` and `render_vertical()` methods
- Cleaner positioning logic using container dimensions matching thumb dimensions
- Better visual consistency across all slider sizes (Sm, Md, Lg)

### Examples
- Updated `slider_styled_demo.rs` with 10 comprehensive examples
  - 7 horizontal slider variations demonstrating sizes, styling, and features
  - 3 vertical slider examples showcasing the new orientation support
  - All examples fully interactive with drag support and onChange handlers

## [0.2.2] - 2025-10-28

### Added
- Tab/Shift-Tab keyboard navigation between form inputs
  - Implemented `tab()` and `shift_tab()` handlers in InputState
  - Proper FocusHandle configuration with `.tab_index(0).tab_stop(true)`
  - Window-level focus navigation using `window.focus_next()` and `window.focus_prev()`
- Comprehensive ROADMAP.md with 90+ component inventory
  - Complete component categorization and status tracking
  - Phase-based development plan with desktop integration features
  - Prioritized quick wins and improvements

### Fixed
- Password input eye icon toggle functionality
  - Icon now properly toggles between "eye" and "eye-off"
  - Password masking correctly switches between bullets (••••) and actual text
  - Immediate UI updates with `window.refresh()` after state changes
  - Fixed state reading to use dynamic `input_state.masked` value

### Improved
- Code quality improvements with removal of 13 unnecessary inline comments across 6 files
  - Removed comments from: color_picker.rs, input_state.rs, input.rs, text.rs, lib.rs, transitions.rs
  - Cleaner, more production-ready codebase

### Examples
- Added `password_test.rs` - Demonstrates password toggle functionality with clear instructions

## [0.2.1] - 2025-10-23

### Added - 🎉 Three New Production-Ready Components!

#### ColorPicker Component 🎨
- Full-featured color picker with HSL, RGB, and HEX mode switching
- Recent colors history (stores last 10 colors automatically)
- Custom color swatches support
- Optional alpha/opacity slider
- Copy to clipboard functionality (HEX format)
- Popover-based clean UI integration
- Immediate UI updates with `cx.notify()`

#### DatePicker Component 📅
- Single date and date range selection modes
- Visual range highlighting with colored backgrounds
  - Range endpoints: bold primary color
  - Range middle dates: light background (15% opacity)
- Disabled dates with greyed-out visual styling
- Weekend disabling helper method (`disable_weekends()`)
- Auto-close popover after selection
- Multiple date formats (ISO, US, EU, custom)
- Locale support for internationalization
- Month navigation with year selection
- Today button for quick selection
- Immediate UI updates without mouse movement

#### Combobox Component 🔍
- Single and multi-select modes
- Real-time search/filter with immediate updates
- Full keyboard navigation (arrow keys, Enter, Escape)
- Custom display and search functions
- Clear selection button
- Badge display for multi-select items
- Popover-based dropdown UI
- Empty state handling
- Disabled state support

#### Calendar Component Enhancements
- Added `DateRange` support with visual styling
- Disabled dates checker function (`is_date_disabled`)
- Range endpoints with bold styling
- Range middle dates with light background
- Improved date selection feedback

### Changed
- Updated component count from 70+ to 73+
- Updated examples count from 50+ to 53+
- Enhanced Calendar component with range selection capabilities

### Fixed
- Fixed all compiler warnings (unused fields, variables, methods)
- Removed unused `is_open` field from ColorPickerState
- Fixed GPUI state lifecycle issues using proper `Entity<T>` pattern
- Proper `DismissEvent` emission for popover closing
- Added `cx.notify()` calls throughout for immediate UI updates

### Improved
- Zero compiler warnings - completely clean build
- Immediate UI updates across all new components (no mouse movement required)
- Comprehensive documentation with code examples for all new components
- Updated README with detailed component documentation
- Updated GitHub Pages with new component listings
- Professional visual styling with theme integration
- Full keyboard navigation support for all new components

### Examples
- Added `color_picker_demo.rs` - Demonstrates all ColorPicker features
- Added `date_picker_demo.rs` - Shows single date and range selection
- Added `combobox_demo.rs` - Illustrates search and multi-select

## [0.2.0] - 2025-10-23

### Added - 🎉 MAJOR RELEASE: 100% Styled Trait Coverage!

#### Icon System Enhancements
- **Icon Phase 1**: Consolidated IconSource module across all components
- **Icon Phase 2**: Added IconSize enum with named sizes (XSmall, Small, Medium, Large, Custom)
- Added rotation support for Icon component using Transformation API
- Improved icon path detection with separator-first logic
- Added comprehensive unit tests for IconSource

#### Component Enhancements
- **Text Component**: Fixed italic and strikethrough rendering using HighlightStyle API
- **Button Component**: Improved API with better ID parameter handling
- **Checkbox Component**: Replaced emoji icons with customizable Icon components
- **Calendar Component**: Added full internationalization (i18n) support with CalendarLocale
  - Built-in locales: English, French, Spanish, German, Portuguese, Italian
  - Support for custom locales

#### Styled Trait Implementation - **ALL 54 COMPONENTS!**
- **Components (14)**: Button, Input, Checkbox, IconButton, Label, Radio, Toggle, Textarea, Avatar, Progress, Slider, Separator, SearchInput, Select
- **Display (6)**: Card, Badge, Accordion, Table, DataTable, Collapsible
- **Navigation (9)**: Menu, Tabs, Toolbar, Sidebar, Breadcrumbs, NavigationMenu, StatusBar, Tree, AppMenu
- **Overlays (11)**: Dialog, Sheet, AlertDialog, Toast, BottomSheet, CommandPalette, ContextMenu, HoverCard, Popover, PopoverMenu, Tooltip
- **Advanced (9)**: TextField, Pagination, ToggleGroup, KeyboardShortcuts, Calendar, Resizable, Editor, Draggable, DropZone

#### 54 New Styled Demos
Created comprehensive styled demonstration examples for every component showing full customization capabilities

### Changed
- **BREAKING**: Icon component now returns AnyElement instead of Div for non-clickable icons (performance improvement)
- All components now support full GPUI styling methods via Styled trait
- User styles now properly override component defaults using `.refine()` pattern
- Removed 3,274 inline comments for cleaner, production-ready code

### Fixed
- Fixed Editor component `.when()` pattern to `.map()` for style application
- Fixed DropZone naming conflict by renaming internal `style` field to `drop_style`
- Fixed Button API usage across 21 example files
- Fixed various component compilation errors and import issues
- Fixed sidebar_demo and menu_demo import paths

### Improved
- **shadcn Philosophy Alignment**: All components now follow "good defaults with complete user control"
- Every component supports customization: `.bg()`, `.border_2()`, `.rounded_lg()`, `.p_4()`, `.shadow_lg()`, and hundreds more
- Added SHADCN_DESIGN_PHILOSOPHY.md documenting design principles
- Better developer experience with consistent Styled trait API across all components
- Production-ready code quality with clean, documented implementations

## [0.1.1] - 2025-10-22

### Changed
- **BREAKING**: Icons are no longer bundled with the library (reduces package size by 95%)
- Added configurable icon path system with `set_icon_base_path()` function
- Users must now provide their own icon assets (see README for setup instructions)

### Fixed
- Fixed 20+ examples with incorrect API usage
- Fixed `scroll` module imports (changed to `scrollable`)
- Fixed VStack compatibility with scrollable_vertical
- Fixed Menu and MenuItem API usage
- Fixed toolbar click handlers to use `on_mouse_down`
- Removed 3 broken test examples

### Improved
- Removed unnecessary inline comments for cleaner, production-ready code
- Added comprehensive icon setup documentation in README
- All 53 working examples now compile successfully
- Updated examples with proper AssetSource configuration

## [0.1.0] - 2025-10-21

### Added
- Initial release of adabraka-ui
- 70+ UI components organized into categories:
  - Core components (Button, Input, Checkbox, Toggle, Select, Slider, etc.)
  - Display components (Card, Badge, Table, DataTable, Accordion)
  - Navigation components (Tabs, Breadcrumbs, Tree, Sidebar, Menu, Toolbar, StatusBar)
  - Overlay components (Dialog, Popover, Toast, CommandPalette, Sheet, etc.)
  - Advanced components (Editor, Scrollable, Resizable, DragDrop, Progress)
- Complete theme system with light and dark modes
- Semantic color tokens inspired by shadcn/ui
- Professional animation system with cubic-bezier easing and spring physics
- Typography system with semantic text variants
- Code editor with syntax highlighting support
- Virtual scrolling for large datasets
- Full keyboard navigation and accessibility support
- Comprehensive documentation and examples

### Features
- Builder pattern API for ergonomic component construction
- Entity-based state management for complex components
- Type-safe APIs with compile-time guarantees
- Performance-optimized for GPUI's retained-mode rendering
- Consistent styling across all components
- Platform-aware UI elements
- Responsive layout utilities (VStack, HStack, Grid)

[Unreleased]: https://github.com/Augani/adabraka-ui/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/Augani/adabraka-ui/releases/tag/v0.3.0
[0.2.4]: https://github.com/Augani/adabraka-ui/releases/tag/v0.2.4
[0.2.3]: https://github.com/Augani/adabraka-ui/releases/tag/v0.2.3
[0.2.2]: https://github.com/Augani/adabraka-ui/releases/tag/v0.2.2
[0.2.1]: https://github.com/Augani/adabraka-ui/releases/tag/v0.2.1
[0.2.0]: https://github.com/Augani/adabraka-ui/releases/tag/v0.2.0
[0.1.1]: https://github.com/Augani/adabraka-ui/releases/tag/v0.1.1
[0.1.0]: https://github.com/Augani/adabraka-ui/releases/tag/v0.1.0
