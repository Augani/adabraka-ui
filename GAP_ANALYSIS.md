# adabraka-ui Gap Analysis & GPUI Extension Strategy

## Executive Summary

After comprehensive analysis of adabraka-ui's 80+ components, GPUI's rendering internals, and modern web framework capabilities, this document identifies what's missing and proposes a strategy to make adabraka-ui a self-sufficient UI framework where users never need to import GPUI directly.

---

## Part 1: Current Strengths

### Components (80+)
- **Input**: Button, IconButton, Input, NumberInput, OTPInput, Select, Combobox, Checkbox, Radio, Toggle, Slider, RangeSlider, TagInput, SearchInput, ColorPicker, DatePicker, TimePicker, HotkeyInput, TextField, TextArea, Editor, MentionInput, FileUpload, Rating
- **Display**: Card, Badge, Table, DataTable, Avatar, AvatarGroup, Separator, Skeleton, Progress, CircularProgress, Sparkline, Spinner, EmptyState, Timeline, Countdown, Text (h1-h6, body, caption, code, muted)
- **Navigation**: Tabs, Sidebar, Menu, MenuBar, ContextMenu, Breadcrumbs, Toolbar, StatusBar, TreeList, FileTree, AppMenu, Pagination
- **Overlays**: Dialog, Popover, Toast, Tooltip, CommandPalette, ConfirmDialog
- **Layout**: VStack, HStack, Grid, Flow, Container, Panel, Spacer, Cluster, MasonryGrid, ScrollContainer, ScrollList, SplitPane
- **Media**: AudioPlayer, VideoPlayer, ImageViewer, Carousel
- **Charts**: BarChart, LineChart, PieChart, Chart (composite)

### Animation System
- 20+ easing functions (quad, cubic, quart, expo, circ, elastic, back, spring, bounce, steps, cubic_bezier)
- Spring physics engine (stiffness/damping/mass model)
- ScrollPhysics (momentum + overscroll bounce)
- AnimatedInteraction (hover/press/focus state blending)
- Ripple component, shake animation, content transitions
- Gesture detection (swipe, long-press, pan, tap)
- Animation coordinator with completion callbacks

### Theme System
- 18 pre-built themes (light, dark, dracula, nord, tokyo_night, catppuccin, rose_pine, etc.)
- Design tokens: colors, radii, shadows, fonts, transitions
- Gradient presets, glow shadows, focus ring animations

### Infrastructure
- Responsive breakpoints system
- Virtual list (uniform + variable height)
- Keyboard navigation throughout
- Builder pattern API, Styled trait on all components

---

## Part 2: GPUI Capabilities & Hard Limits

### What GPUI CAN Do
| Capability | Details |
|-----------|---------|
| **Flexbox + Grid** | Full Taffy layout engine |
| **2-stop linear gradients** | `linear_gradient(angle, from, to)` |
| **Box shadows** | Outset only: color, offset, blur, spread |
| **Rounded corners** | Per-corner radii |
| **Opacity** | Per-element, 0.0 - 1.0 |
| **Transforms on sprites** | TransformationMatrix (rotate, scale, translate) for images/icons |
| **Borders** | Per-side width + color + style |
| **Cursors** | 22 cursor styles |
| **Animations** | `with_animation(id, Animation, |el, delta|)` |
| **Hit testing** | Hitbox-based mouse interaction |
| **Deferred rendering** | Priority-based z-ordering |
| **Text rendering** | SubpixelSprite with glyph atlas |
| **Vibrancy** | macOS whole-window vibrancy |

### What GPUI CANNOT Do (Hard Limits)
| Missing | Impact | Workaround Possible? |
|---------|--------|---------------------|
| **Backdrop blur** | No glassmorphism/frosted glass | Partial: semi-transparent bg simulates |
| **CSS filters** | No blur(), brightness(), contrast(), saturate() | No |
| **Clip-path** | No custom element shapes | No |
| **Blend modes** | No multiply, screen, overlay etc. | No |
| **Text shadow** | No shadow on text glyphs | Partial: layered text at offset |
| **Multi-stop gradients** | Only 2 color stops | Partial: stack multiple 2-stop divs |
| **Radial/conic gradients** | Only linear | No |
| **Inset shadows** | Only outset BoxShadow | Partial: inner gradient overlay |
| **Quad transforms** | No rotate/scale on div elements | Partial: bounds manipulation for scale |
| **Sticky positioning** | No scroll-aware sticky | Partial: manual scroll position tracking |
| **Scroll snap** | No snap points | Partial: programmatic snap on scroll end |
| **Z-index** | No explicit z-index | Use deferred() with priority |
| **Container queries** | No element-size responsive | Partial: measure element bounds |
| **Aspect ratio** | No CSS aspect-ratio | Manual: set h = w * ratio |
| **Keyframe animations** | No multi-step looping | Partial: chain with_animations |
| **Animation interruption** | Can't cancel mid-animation | Partial: change animation ID |
| **Reduced motion** | No OS preference detection | Can check platform API |
| **Custom paint** | No canvas-like arbitrary drawing | Path primitive exists but limited |

---

## Part 3: Gap Analysis vs Web Frameworks

### Critical Gaps (Must Have for Competitive Parity)

#### 1. Component Status

**Already Exist (confirmed by inventory):**
- Drawer/Sheet — `overlays/sheet.rs` (Left/Right/Top/Bottom, 5 sizes)
- BottomSheet — `overlays/bottom_sheet.rs` (drag handle, snap points)
- Accordion — `display/accordion.rs` (single/multi mode, bordered)
- Collapsible — `components/collapsible.rs` (trigger/content pattern)
- HoverCard — `overlays/hover_card.rs` (4 positions, 3 alignments)
- NavigationMenu — `components/navigation_menu.rs` (horizontal/vertical)
- Skeleton — `components/skeleton.rs` (Text/Circle/Rect variants)
- Resizable Panels — `components/split_pane.rs` + `components/resizable.rs`
- ConfirmDialog — `components/confirm_dialog.rs`
- DragDrop — `components/drag_drop.rs`

**Still Missing:**
| Component | Web Equivalent | Priority |
|-----------|---------------|----------|
| **SortableList** | Drag-to-reorder items with animation | HIGH |
| **InfiniteScroll** | Load-more on scroll bottom | HIGH |
| **Form** | Form builder with validation orchestration | HIGH |
| **DataGrid** | Spreadsheet-like editable table | MEDIUM |
| **AspectRatio** | Aspect ratio container | LOW |
| **Popconfirm** | Inline confirmation popover | LOW |

#### 2. Design Token Gaps
| Token Type | What's Missing |
|-----------|---------------|
| **Spacing scale** | No standardized 4/8/12/16/20/24/32/40/48/64px system |
| **Typography scale** | No xs/sm/base/lg/xl/2xl/3xl/4xl text sizes |
| **Animation tokens** | No standardized transition-fast/normal/slow durations |
| **Z-index scale** | No layering system (dropdown/sticky/modal/popover/tooltip) |
| **Breakpoint tokens** | Exist in responsive.rs but not in theme tokens |

#### 3. Visual Effect Gaps
| Effect | Web CSS | Status |
|--------|---------|--------|
| Glassmorphism | `backdrop-filter: blur()` | SIMULATE with semi-transparent bg |
| Text gradient | `background-clip: text` | NOT POSSIBLE |
| Neon glow | `text-shadow` + `box-shadow` | PARTIAL (box-shadow only) |
| Frosted border | Border + backdrop blur | NOT POSSIBLE |
| Neumorphism | Inset + outset shadows | PARTIAL (outset only) |
| Animated gradient | Gradient angle animation | POSSIBLE via with_animation |

#### 4. Interaction Gaps
| Interaction | Web | Status |
|------------|-----|--------|
| Drag-to-reorder | HTML5 DnD / libraries | NEED TO BUILD |
| Scroll snap | CSS scroll-snap | NEED TO SIMULATE |
| Intersection observer | IntersectionObserver API | NEED TO BUILD |
| Resize observer | ResizeObserver API | NEED TO BUILD |
| Copy to clipboard | Clipboard API | GPUI has clipboard support |
| Focus trap | focus-trap library | EXISTS in Dialog |

---

## Part 4: GPUI Wrapper Strategy

### Goal: Users import ONLY `adabraka_ui` — never `gpui` directly

### 4.1 Re-export Layer (`src/gpui_ext.rs`)

Re-export all commonly used GPUI types so users don't need `use gpui::*`:

```
pub use gpui::{
    // Core types
    App, Window, Context, Entity, Global, SharedString, ElementId,

    // Element traits
    IntoElement, Element, ParentElement, Styled, InteractiveElement,
    StatefulInteractiveElement, Focusable, Render, RenderOnce,

    // Layout
    px, relative, rems, Pixels, Length, DefiniteLength,
    Point, Size, Bounds, Edges, Corners,

    // Color
    Hsla, Rgba, hsla, rgba, black, white, transparent, opaque_grey,
    Background, Fill, linear_gradient, linear_color_stop,

    // Events
    MouseButton, MouseDownEvent, MouseUpEvent, MouseMoveEvent,
    ScrollWheelEvent, KeyDownEvent, KeyUpEvent,
    FocusEvent, BlurEvent, DismissEvent,

    // Animation
    Animation, AnimationExt,

    // Styling
    StyleRefinement, BoxShadow, FontWeight, FontStyle,
    CursorStyle, Corner, FlexDirection,

    // Layout helpers
    div, svg, img, canvas, deferred, anchored,

    // Focus
    FocusHandle,

    // Actions
    actions, KeyBinding,

    // Prelude
    prelude::FluentBuilder,
};
```

### 4.2 Extended Styled Trait (`src/styled_ext.rs`)

New trait that extends GPUI's Styled with common utilities:

```
trait StyledExt: Styled {
    // Spacing scale
    fn gap_1(self) -> Self;    // 4px
    fn gap_2(self) -> Self;    // 8px
    fn gap_3(self) -> Self;    // 12px
    fn gap_4(self) -> Self;    // 16px
    fn gap_5(self) -> Self;    // 20px
    fn gap_6(self) -> Self;    // 24px
    fn gap_8(self) -> Self;    // 32px
    fn gap_10(self) -> Self;   // 40px
    fn gap_12(self) -> Self;   // 48px
    fn gap_16(self) -> Self;   // 64px

    // Same for p_1..p_16, m_1..m_16

    // Typography scale
    fn text_xs(self) -> Self;    // 12px
    fn text_sm(self) -> Self;    // 14px
    fn text_base(self) -> Self;  // 16px
    fn text_lg(self) -> Self;    // 18px
    fn text_xl(self) -> Self;    // 20px
    fn text_2xl(self) -> Self;   // 24px
    fn text_3xl(self) -> Self;   // 30px
    fn text_4xl(self) -> Self;   // 36px

    // Aspect ratio
    fn aspect_square(self) -> Self;
    fn aspect_video(self) -> Self;   // 16:9
    fn aspect_ratio(self, w: f32, h: f32) -> Self;

    // Visual effects (simulated)
    fn glass(self) -> Self;           // Semi-transparent bg + border
    fn glass_dark(self) -> Self;
    fn frosted(self) -> Self;         // Higher opacity glass
    fn elevated(self, level: u8) -> Self; // Shadow by elevation level

    // Quick layout
    fn center(self) -> Self;          // flex + items_center + justify_center
    fn stack(self) -> Self;           // flex + flex_col
    fn row(self) -> Self;             // flex + flex_row
    fn wrap(self) -> Self;            // flex_wrap
    fn between(self) -> Self;         // justify_between
    fn stretch(self) -> Self;         // items_stretch

    // Truncation
    fn truncate(self) -> Self;        // overflow_hidden + text_ellipsis
    fn line_clamp(self, lines: u8) -> Self;

    // Ring (focus indicator)
    fn ring(self, color: Hsla) -> Self;
    fn ring_primary(self) -> Self;
    fn ring_error(self) -> Self;

    // Transition helpers
    fn transition_all(self) -> Self;
    fn transition_colors(self) -> Self;
    fn transition_opacity(self) -> Self;
}
```

### 4.3 Animation Builder (`src/animate.rs`)

Declarative animation API wrapping GPUI's `with_animation`:

```
// CSS-like transition
element.transition()
    .property(Property::Opacity)
    .property(Property::BackgroundColor)
    .duration(ms(200))
    .easing(Easing::EaseOut)
    .build()

// Keyframe animation
element.keyframes("bounce")
    .at(0.0, |el| el.mt(px(0.0)))
    .at(0.5, |el| el.mt(px(-20.0)))
    .at(1.0, |el| el.mt(px(0.0)))
    .duration(ms(600))
    .repeat(Repeat::Infinite)
    .build()

// Stagger children
container.stagger()
    .delay(ms(50))
    .animation(Animation::new(ms(300)).with_easing(ease_out_cubic))
    .effect(|el, delta| el.opacity(delta).mt(px(8.0 * (1.0 - delta))))
    .build()
```

### 4.4 New Components to Build

#### Priority 1 (Essential)
| Component | Description | Estimated Effort |
|-----------|-------------|-----------------|
| **Drawer** | Slide-in panel from any edge, overlay or push | Medium |
| **Sheet** | Bottom sheet with drag-to-dismiss | Medium |
| **Form** | Form builder with typed fields, validation, error coordination | High |
| **SortableList** | Drag-to-reorder with animation | High |
| **InfiniteScroll** | Lazy loading with scroll threshold detection | Medium |
| **Accordion** | Animated collapsible sections with single/multi mode | Low |
| **DataGrid** | Editable cells, column resize, sort, filter | High |

#### Priority 2 (Nice to Have)
| Component | Description |
|-----------|-------------|
| **Popconfirm** | Confirmation popover before action |
| **Tour/Onboarding** | Step-by-step feature highlight |
| **Kanban** | Drag-and-drop board columns |
| **RichTextEditor** | Markdown/WYSIWYG with formatting toolbar |
| **VirtualTable** | Virtualized rows + columns for huge datasets |
| **Dock/Panel** | IDE-like dockable panels |
| **Notification** | System-level notifications (macOS native) |

### 4.5 Design Token Enhancement

Add to `ThemeTokens`:

```
// Spacing scale
spacing_1: Pixels,   // 4px
spacing_2: Pixels,   // 8px
spacing_3: Pixels,   // 12px
spacing_4: Pixels,   // 16px
spacing_5: Pixels,   // 20px
spacing_6: Pixels,   // 24px
spacing_8: Pixels,   // 32px
spacing_10: Pixels,  // 40px
spacing_12: Pixels,  // 48px
spacing_16: Pixels,  // 64px

// Typography scale
text_xs: Pixels,     // 12px
text_sm: Pixels,     // 14px
text_base: Pixels,   // 16px
text_lg: Pixels,     // 18px
text_xl: Pixels,     // 20px
text_2xl: Pixels,    // 24px
text_3xl: Pixels,    // 30px
text_4xl: Pixels,    // 36px
text_5xl: Pixels,    // 48px

// Line heights
leading_none: f32,    // 1.0
leading_tight: f32,   // 1.25
leading_snug: f32,    // 1.375
leading_normal: f32,  // 1.5
leading_relaxed: f32, // 1.625
leading_loose: f32,   // 2.0

// Z-index layers
z_dropdown: u32,  // 1000
z_sticky: u32,    // 1100
z_modal: u32,     // 1300
z_popover: u32,   // 1400
z_tooltip: u32,   // 1500

// Animation durations
duration_fastest: Duration,  // 50ms
duration_faster: Duration,   // 100ms
duration_fast: Duration,     // 150ms
duration_normal: Duration,   // 200ms
duration_slow: Duration,     // 300ms
duration_slower: Duration,   // 400ms
duration_slowest: Duration,  // 500ms
```

---

## Part 5: Implementation Roadmap

### Phase 1: Foundation (Self-Sufficiency)
1. **GPUI re-export layer** (`src/gpui_ext.rs`) — Users import only `adabraka_ui`
2. **StyledExt trait** (`src/styled_ext.rs`) — Spacing/typography/layout shortcuts
3. **Design token enhancement** — Spacing, typography, z-index, duration scales in ThemeTokens

### Phase 2: Missing Components
4. **Form builder** — Typed fields, validation orchestration, error coordination
5. **InfiniteScroll** — Scroll threshold detection + loading states (uses on_near_end from virtual_list)
6. **SortableList** — Drag-to-reorder with smooth animation
7. **Animation builder** — Declarative keyframes + stagger + transitions API

### Phase 3: Advanced Interaction
8. **Scroll snap simulation** — Programmatic snap on scroll end
9. **Intersection observer** — Element visibility detection via scroll position + bounds
10. **Resize observer** — Element size change detection
11. **Sheet gesture dismiss** — Enhance existing Sheet with swipe-to-dismiss

### Phase 4: Power Features
12. **DataGrid** — Editable, sortable, filterable table
13. **Stagger animation** — Cascaded child entry animations
14. **Visual effect presets** — Glass, neumorphism, glow compositions
15. **Component explorer app** — Interactive demo of all 85+ components

### Phase 5: Upstream GPUI Contributions (Long-term)
16. Quad transforms (rotate/scale on any element)
17. Multi-stop gradients (>2 color stops)
18. Inset shadows
19. Text shadows
20. Per-element backdrop blur
21. Animation cancellation/interruption
22. Letter-spacing / character spacing
23. Accessibility tree / ARIA roles

---

## Part 6: Quick Wins (Can Ship This Week)

1. **GPUI re-exports** in `src/gpui_ext.rs` — 1 hour
2. **Spacing/typography tokens** in ThemeTokens — 1 hour
3. **StyledExt trait** basics (center, stack, glass, truncate) — 2 hours
4. **InfiniteScroll wrapper** — 2 hours
5. **Sheet init in lib.rs** — Sheet/BottomSheet exist but aren't initialized in `init()` — 10 min
6. **Form builder** basics — 4 hours

Total: ~10 hours of work for massive DX improvement.

Note: Drawer/Sheet, Accordion, Collapsible, HoverCard, NavigationMenu, BottomSheet
all already exist and were confirmed by the inventory analysis.

---

## Conclusion

adabraka-ui is already the most comprehensive GPUI component library with **85+ components** across 62 component files, 11 overlay types, 11 navigation modules, and 18 theme variants. The biggest remaining gaps are:

1. **Self-sufficiency** — Users still need `use gpui::*` for basic types
2. **Design tokens** — Missing spacing/typography scales that web devs expect
3. **DX shortcuts** — No Tailwind-like utility methods (StyledExt)
4. **Missing components** — Form builder, SortableList, InfiniteScroll, DataGrid
5. **Animation DX** — No declarative keyframe/transition/stagger API

The GPUI rendering limitations (no backdrop blur, no transforms on divs, no filters) are real but don't block 95% of real-world UIs. The simulated workarounds (glass effect, elevation shadows, stacked gradients) cover most use cases.

**Strategy: Own the layer above GPUI.** Make adabraka-ui the only import users need. Wrap GPUI types, extend them with utilities, and fill every component gap until switching from web frameworks has zero friction.
