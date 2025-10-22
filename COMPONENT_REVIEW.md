# Component Review and Improvement Plan

## Button Component Analysis

### Current Implementation Status
**Location**: `src/components/button.rs`

**Strengths**:
- ✅ Clean builder pattern API
- ✅ Proper focus handling with FocusHandle
- ✅ Good variant support (Default, Secondary, Destructive, Outline, Ghost, Link)
- ✅ Size variants (Sm, Md, Lg, Icon)
- ✅ Disabled state properly implemented
- ✅ Uses StatefulInteractiveElement correctly
- ✅ Proper mouse event prevention (`window.prevent_default()`)
- ✅ Stop propagation on click events

**Issues Identified**:

1. **Missing Core Features** (compared to gc library):
   - ❌ No icon support (icon/prefix/suffix)
   - ❌ No tooltip support
   - ❌ No loading state with spinner
   - ❌ No compact mode
   - ❌ No children support for custom content
   - ❌ No on_hover handler
   - ❌ No tab_index/tab_stop control
   - ❌ No selected state
   - ❌ No outline variant as separate mode
   - ❌ No border corner/edge control
   - ❌ Limited rounded control (only uses theme.tokens.radius_md)

2. **API Inconsistencies**:
   - ❌ ID generation from label can cause duplicates (line 39: `ElementId::Name(SharedString::from(format!("button-{}", label)))`)
   - ❌ Should accept explicit ID like gc: `Button::new(id: impl Into<ElementId>)`
   - ❌ hover_fg calculated but never used (line 98-140)

3. **GPUI Pattern Issues**:
   - ❌ Not implementing Styled trait for custom styling support
   - ❌ Not implementing ParentElement trait for children support
   - ❌ Using Rc<dyn Fn> instead of Box<dyn Fn> (both work, but Box is more common in Rust)

4. **Theme/Styling Issues**:
   - ⚠️ Shadow is applied on Ghost/Link variants in some states (line 171-173) - should never have shadow
   - ⚠️ Text uses Text component wrapper which adds overhead - gc uses simple label string
   - ⚠️ Active state not implemented (gc has `.active()` styling)

### Recommended Fixes (Priority Order)

#### P0 - Critical (Breaks consistency/correctness)
1. **Fix ID generation** - Accept explicit ID parameter instead of generating from label
2. **Fix hover_fg unused** - Either use it or remove from tuple
3. **Remove shadow from Ghost/Link** - These should never have shadows

#### P1 - Important (Missing essential features)
4. **Add icon support** - icon/prefix/suffix with IconSource
5. **Add loading state** - with optional custom loading icon
6. **Add tooltip support** - with optional action hints
7. **Add selected state** - for toggle buttons
8. **Implement ParentElement** - for children support
9. **Add active state styling** - `.active()` for button pressed state

#### P2 - Nice to have (Enhances flexibility)
10. **Add compact mode** - reduced padding
11. **Add on_hover handler** - for hover state tracking
12. **Add tab_index/tab_stop control** - for accessibility
13. **Add rounded control** - flexible border radius
14. **Implement Styled trait** - for custom style overrides

### Implementation Status

**Phase 1**: Fix Critical Issues (P0) ✅ **COMPLETED**
- ✅ Updated Button::new() to accept ElementId parameter
  - Prevents ID collisions from duplicate labels
  - Consistent with GPUI patterns and gc library
- ✅ Fixed hover_fg usage by including in variant tuple and applying in hover state
  - All variants now properly use hover_fg color
- ✅ Fixed shadow logic using has_shadow boolean instead of variant checks
  - Ghost, Link, and Outline variants correctly never show shadows
- ✅ Updated 113+ Button::new() calls across 26 files (components and examples)
  - All components: pagination, calendar, dialog, alert_dialog, sheet
  - All 21 example files updated

**Phase 2**: Add Essential Features (P1) ✅ **COMPLETED**
- ✅ Added icon support with IconSource (Named and FilePath)
  - Supports both named icons and custom file paths
  - Properly resolves paths using icon_config
- ✅ Added IconPosition enum (Start, End) for icon placement
  - Icons can appear before or after label
  - Exported in prelude for easy access
- ✅ Added loading state with loading spinner
  - Replaces icon when loading
  - Prevents clicks during loading
  - Shows at configured icon position
- ✅ Added selected state with proper styling
  - Uses accent colors for selected state
  - Perfect for toggle button groups
- ✅ Added tooltip field (rendering to be implemented with Tooltip component)
  - API ready, full implementation pending Tooltip component review
- ✅ Updated clickable() logic to prevent clicks during loading
  - `!self.disabled && !self.loading && self.on_click.is_some()`
- ✅ Created render_icon() and render_loading_spinner() helper functions
  - Standalone functions to avoid borrow checker issues
  - Clean, reusable icon rendering
- ✅ Created comprehensive button_features_demo.rs example
  - Demonstrates all variants (Default, Secondary, Destructive, Outline, Ghost, Link)
  - Shows all sizes (Sm, Md, Lg)
  - Demonstrates icon support with Start/End positions
  - Shows loading state toggle
  - Demonstrates selected state for toggle buttons
  - Shows disabled state variations

**Phase 3**: Enhanced Flexibility (P2) - **TODO**
- Add compact mode (reduced padding for dense UIs)
- Add on_hover handler for hover state tracking
- Add tab_index/tab_stop control for advanced accessibility
- Add rounded control (flexible border radius beyond theme default)
- Implement Styled trait for custom style overrides
- Implement ParentElement trait for children support
- Add active state styling (.active() for button pressed visual feedback)

---

## IconButton Component Analysis

### Current Implementation Status
**Location**: `src/components/icon_button.rs`

**Strengths**:
- ✅ Good IconSource enum with smart From<&str> detection
- ✅ Proper SVG path resolution
- ✅ no_background mode for transparent buttons
- ✅ Configurable icon_size separate from button size

**Issues**:
1. **ID generation from icon name** - Same issue as Button
2. **Duplicate code** - Shares all variant styling logic with Button
3. **Missing features**: tooltip, loading, selected state

**Recommendation**:
- Once Button supports icons, IconButton can be a thin wrapper or removed entirely
- OR: Keep IconButton but make it use Button internally with icon-only mode

---

## Next Components to Review
1. Input & InputState
2. Icon component
3. Text component
4. Layout components (VStack, HStack, Grid)
5. Scrollable system
6. Theme system
