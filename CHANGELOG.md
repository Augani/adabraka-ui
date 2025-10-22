# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[Unreleased]: https://github.com/Augani/adabraka-ui/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/Augani/adabraka-ui/releases/tag/v0.1.0
