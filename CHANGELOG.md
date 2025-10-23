# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[Unreleased]: https://github.com/Augani/adabraka-ui/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/Augani/adabraka-ui/releases/tag/v0.2.0
[0.1.1]: https://github.com/Augani/adabraka-ui/releases/tag/v0.1.1
[0.1.0]: https://github.com/Augani/adabraka-ui/releases/tag/v0.1.0
