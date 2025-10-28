## Adabraka UI Desktop Roadmap

Last updated: 2025-10-28

This roadmap focuses on features desktop apps expect from a modern Rust/gpui UI toolkit. Phases are approximate and can overlap.

---

## 🎉 Completed (73+ Components)

### ✅ Core UI Components
- **Buttons**: Button, IconButton with 6 variants (Default, Secondary, Outline, Ghost, Link, Destructive)
- **Text & Typography**: Text component with semantic variants (h1-h4, body, label, code, muted, caption)
- **Inputs**:
  - Input with validation, masking, various types (Text, Email, Password, Tel, URL, CreditCard, Date, Number)
  - Password input with eye icon toggle ✨ **(NEW: Fixed 2025-10-28)**
  - Textarea with multi-line editing
  - SearchInput with advanced filtering
- **Form Controls**:
  - Checkbox with customizable icons
  - Radio buttons with grouping
  - Toggle switches
  - ToggleGroup for multiple selections
  - Select dropdowns
  - Combobox with search and multi-select
  - Slider with range support
- **Pickers**:
  - ColorPicker with HSL/RGB/HEX modes, recent colors, alpha support
  - DatePicker with single date and range selection
  - Calendar with i18n support (6 built-in locales)
- **Layout**:
  - VStack, HStack with flexible alignment
  - Grid layout
  - Resizable panes with drag handles
  - Collapsible sections
  - Scrollable areas with custom scrollbars
- **Navigation**:
  - NavigationMenu with nested items
  - Pagination controls
- **Feedback**:
  - Tooltip with positioning
  - Progress indicators
  - Skeleton loaders
  - ConfirmDialog for user confirmations
- **Containers**:
  - Avatar with image/text fallback
  - Label for form fields
  - Separator/Divider
- **Advanced**:
  - Icon system with 100+ icons, rotation support
  - Editor component for code/text editing
  - DragDrop utilities
  - KeyboardShortcuts registry

### ✅ Theme System
- Complete theme tokens (colors, spacing, typography, shadows)
- Light and dark themes built-in
- Semantic color tokens for consistency
- Full Styled trait implementation across all 73+ components

### ✅ Examples & Documentation
- 53+ comprehensive examples
- GitHub Pages documentation site
- Professional showcase applications (Music Player, Task Manager)

---

## Phase 1 (Weeks 1–4): Desktop foundations and polish

### 🔄 In Progress
- **Native window chrome APIs**:
  - ⚠️ Custom titlebar (partial - basic support exists)
  - ❌ Draggable regions
  - ❌ Fullscreen toggle
  - ❌ Always-on-top
  - ❌ Vibrancy/acrylic effects
  - ❌ Traffic-light buttons (macOS)
  - ❌ Window shadow/resize customization
  - ❌ DPI-awareness helpers

### 📋 Todo
- **System keyboard shortcuts**:
  - ❌ Accelerator registry
  - ❌ Conflict detection
  - ❌ OS-key display (Cmd/Ctrl)
  - ❌ Global vs window-scoped shortcuts
  - ✅ Basic keyboard shortcuts (exists in KeyboardShortcuts component)

- **App and context menus**:
  - ❌ Native menu bar integration
  - ❌ Nested menus
  - ❌ Separators, checkable items
  - ❌ Dynamic enable/disable
  - ❌ Accelerators in menus
  - ✅ NavigationMenu (UI-level menus exist)

- **System dialogs**:
  - ❌ Open/save file dialogs
  - ❌ Folder picker
  - ❌ Message box
  - ✅ ColorPicker (UI component exists)
  - ❌ Native OS dialogs integration

- **Clipboard**:
  - ❌ Text copy/paste
  - ❌ HTML clipboard
  - ❌ Image clipboard
  - ❌ Files clipboard
  - ❌ MIME type mapping
  - ❌ Cut/copy/paste command wiring

- **Drag-and-drop**:
  - ✅ Basic drag-drop utilities exist
  - ❌ OS <-> app for files
  - ❌ Drag previews
  - ❌ Drop targets with visual feedback

---

## Phase 2 (Weeks 5–8): App shell integrations

### 📋 Todo
- **System tray/dock**:
  - ❌ Tray icon
  - ❌ Tray context menu
  - ❌ Badges
  - ❌ Dock progress bar
  - ❌ Attention request bounce

- **Notifications**:
  - ❌ Native OS notifications
  - ❌ Notification actions
  - ❌ Click callbacks
  - ❌ Permission checks

- **Preferences window kit**:
  - ❌ Template with sidebar sections
  - ❌ Search in preferences
  - ❌ Autosave to settings

- **About dialog kit**:
  - ❌ App metadata display
  - ❌ Links handling
  - ❌ Licenses viewer

- **Error/crash UI**:
  - ✅ ConfirmDialog (basic modal exists)
  - ❌ Fatal error modal with details
  - ❌ Relaunch action
  - ❌ Non-fatal toast pattern
  - ❌ Inline error pattern

- **Auto-update UI**:
  - ❌ Download progress
  - ❌ Verify/restart prompts
  - ❌ Updater backend integration

---

## Phase 3 (Weeks 9–12): Power-user components

### 🔄 In Progress
- **Advanced DataGrid**:
  - ❌ Column resize/reorder
  - ❌ Sticky headers/columns
  - ❌ Sorting
  - ❌ Filtering
  - ❌ Grouping
  - ❌ Editable cells
  - ❌ Copy/paste support
  - ❌ CSV export
  - ❌ Virtualization for 100k+ rows

### 📋 Todo
- **Layout manager**:
  - ✅ Resizable panes (basic support exists)
  - ❌ Docking system
  - ❌ Persistent layouts
  - ❌ Snap-to behavior
  - ❌ Advanced splitters

- **Tabs with detaching**:
  - ❌ Tab component
  - ❌ Move tabs across windows
  - ❌ Overflow with scroll/menus
  - ❌ Pinned tabs

- **Rich Text Editor**:
  - ✅ Basic Editor component exists
  - ❌ Inline formatting (bold, italic)
  - ❌ Lists (ordered/unordered)
  - ❌ Links
  - ❌ Markdown shortcuts
  - ❌ Clipboard fidelity
  - ❌ IME-safe text input

- **Virtualized Tree/List pro**:
  - ✅ Basic scrollable lists exist
  - ❌ Type-to-select
  - ❌ Async node loading
  - ❌ Drag-reorder
  - ❌ Keyboard multi-select

---

## Phase 4 (Weeks 13–16): UX, accessibility, internationalization

### 🔄 In Progress
- **Theming and OS sync**:
  - ✅ Dynamic theme tokens (complete)
  - ✅ Light/dark themes
  - ❌ Auto light/dark based on OS
  - ❌ High-contrast mode
  - ❌ Per-monitor DPI

### 📋 Todo
- **Accessibility**:
  - ❌ Roles/labels map to gpui
  - ❌ Focus order tooling
  - ✅ Full keyboard nav (partial - exists in many components)
  - ❌ Color contrast checks
  - ❌ Screen reader support
  - ❌ ARIA attributes

- **Internationalization**:
  - ✅ Calendar i18n (6 locales)
  - ❌ RTL layout support
  - ❌ Locale-aware number formatting
  - ❌ Locale-aware date formatting
  - ❌ Pluralization
  - ❌ Bidi text support
  - ❌ IME correctness

- **Performance tuning**:
  - ❌ Frame pacing profiling
  - ❌ Input latency measurement
  - ❌ Batch update optimization
  - ❌ Faster list diffing

---

## Cross-cutting (ongoing)

### ✅ Completed
- **Design tokens**: Complete color/spacing/typography system with semantic aliases
- **Component consistency**: All 73+ components implement Styled trait
- **Examples**: 53+ comprehensive examples covering all components

### 🔄 In Progress
- **API stability**:
  - ✅ Component props convention established
  - ❌ Breaking-change policy
  - ❌ Deprecation path

- **Docs and gallery**:
  - ✅ GitHub Pages site
  - ✅ Component showcase apps
  - ❌ Live playground app
  - ❌ Interactive code snippets
  - ❌ "Choose X vs Y" guidance

- **Testing**:
  - ❌ Visual snapshots per-OS
  - ❌ Input E2E tests
  - ❌ Perf benches for lists/editor
  - ❌ Accessibility checks
  - ❌ Unit tests for all components

- **State and persistence**:
  - ✅ Basic state management in components
  - ❌ Reactive settings API (disk-backed)
  - ❌ Undo/redo provider
  - ✅ Form validation patterns (Input component)

---

## Prioritized quick wins (next 1–2 weeks)

### 🎯 High Priority
1. **Native menus + accelerators**
   - System menu bar integration
   - Keyboard shortcut display
   - Menu item enable/disable states

2. **Clipboard + system dialogs**
   - Text clipboard operations
   - File open/save dialogs
   - Native folder picker

3. **System tray + notifications**
   - Tray icon with menu
   - Native OS notifications
   - Badge and attention APIs

4. **Virtualized DataGrid v1**
   - Read-only grid with 100k+ rows
   - Sortable columns
   - Sticky headers
   - Keyboard navigation

5. **Tabs component**
   - Basic tabbed interface
   - Closable tabs
   - Keyboard navigation

---

## Recent Accomplishments (2025-10-28)

### ✨ Latest Updates
- **Password Input Enhancement**: Fixed eye icon toggle functionality with proper state management
- **Component Count**: 73+ production-ready components
- **Example Count**: 53+ comprehensive examples
- **Zero Warnings**: Clean build with all warnings resolved
- **Theme Coverage**: 100% Styled trait implementation across all components

---

## Proposed crates/modules

### Current Structure
- **adabraka-ui** (main): All 73+ UI components, theme system, layouts

### Future Modules (TBD)
- **adabraka-desktop**: `window`, `menu`, `tray`, `clipboard`, `dragdrop`, `dialogs`, `notifications`
- **adabraka-components-pro**: `datagrid`, `dock_layout`, `tabs_pro`, `rte`
- **adabraka-accessibility**: roles, focus tools, test helpers
- **adabraka-playground**: showcase + visual tests

---

## Acceptance criteria (samples)

### Completed
- ✅ **73+ Components**: All implement Styled trait for full customization
- ✅ **Theme System**: Complete with semantic tokens, light/dark modes
- ✅ **Form Controls**: Comprehensive input validation and masking
- ✅ **Pickers**: ColorPicker, DatePicker with range support
- ✅ **Password Input**: Eye icon toggle with proper masked/unmasked states

### In Progress
- **Menus**: Dynamic enable/disable via app state; accelerators display OS-native; E2E test for dispatch
- **Dialogs**: Async open/save with cancel; filters; multi-select; snapshot paths mocked in CI
- **Tray**: Icon, context menu works; click events; badge update API verified on macOS/Windows
- **DataGrid v1**: 100k rows at 60fps scroll; sort on header click; sticky header; keyboard nav

---

## Tracking & updates

- Maintain per-feature issues linked to these sections
- Tag milestones by phase (P1–P4)
- Update progress weekly
- Add measurements for perf goals (fps, memory, latency) to acceptance criteria as they are profiled

---

## Legend
- ✅ **Completed**: Feature is implemented and tested
- 🔄 **In Progress**: Currently being worked on
- ❌ **Todo**: Not started yet
- ⚠️ **Partial**: Partially implemented, needs completion
