# adabraka-ui: Visual Excellence Roadmap

**Goal**: Close the visual gap between native GPUI apps and Electron/web apps, making adabraka-ui the definitive choice for beautiful desktop applications in Rust.

**Current state**: 85+ components, 18 themes, 30+ easings, spring physics, gesture detection, scroll physics, content transitions, animation coordinator, GPUI fork with inset shadows/letter spacing/squircle corners/text shadows/animation cancellation.

---

## 1. NEW COMPONENTS

### A. Animation & Motion Components

**AnimatedPresence** - Manages mount/unmount animations for child elements. Delays DOM removal until exit animation completes. Wraps any element to add enter/exit lifecycle.
- Enables: Consistent enter/exit animations anywhere, not just overlays. List item removal with fade-out. View transitions.
- GPUI fork: No
- Priority: P0 (foundational - dozens of other components need this pattern)
- Complexity: Medium (2 days). Core is a timer + state machine that holds old content while exit animation plays. Already proven in overlay dismissing pattern.

**AnimatedList** - Auto-animates item insert, remove, and reorder. Wraps VStack/HStack, diffs children by key, applies configurable transitions per change type.
- Enables: Chat messages sliding in, todo items animating out on delete, search results filtering with motion, notification list updates.
- GPUI fork: No
- Priority: P0 (lists are everywhere in desktop apps)
- Complexity: High (5 days). Key-based diffing of children, snapshot previous positions, animate to new positions. Reorder animation requires LayoutTransition concepts.

**AnimatedCounter** - Animated number display with digit tween or rolling transitions. Supports formatting (currency, percentage, compact notation).
- Enables: Dashboard KPIs, score displays, analytics counters, file transfer progress bytes.
- GPUI fork: No
- Priority: P0 (every dashboard app needs this)
- Complexity: Medium (2 days). Interpolate between old and new values using spring/easing, render formatted result each frame.

**Shimmer** - Animated gradient sweep overlay for loading states. Renders a diagonal highlight band that sweeps across the element on loop.
- Enables: Polished loading placeholders for cards, text blocks, images. Replaces static Skeleton with motion.
- GPUI fork: No
- Priority: P0 (loading states are universal)
- Complexity: Low (1 day). Animated position offset of a semi-transparent gradient overlay using with_animation. Already have 2-stop linear gradients.

**AnimatedProgress** - ProgressBar variant with animated fill transitions, optional shimmer on the bar, and color transitions at value thresholds.
- Enables: File upload progress, build progress, onboarding completion, health bars in dashboards.
- GPUI fork: No
- Priority: P1
- Complexity: Low (1 day). Extend existing ProgressBar with lerp_f32 on value changes and shimmer overlay.

**Marquee** - Auto-scrolling horizontal/vertical content with configurable speed, pause-on-hover, and seamless loop (duplicate content for infinite scroll illusion).
- Enables: News tickers, announcement bars, music "now playing" scroll, stock tickers.
- GPUI fork: No
- Priority: P1
- Complexity: Medium (2 days). Animated scroll offset with content duplication for seamless loop.

**TypeWriter** - Character-by-character text reveal with configurable speed, cursor blink, optional deletion phase.
- Enables: AI chat streaming responses, terminal-style displays, onboarding hero text, dramatic text reveals.
- GPUI fork: No
- Priority: P1
- Complexity: Medium (2 days). Timer-driven character count increment, cursor blink via animation toggle.

**NumberTicker** - Slot-machine style digit roller. Each digit column animates independently with vertical scroll between 0-9.
- Enables: Dashboards, game scores, analytics counters, countdown displays.
- GPUI fork: No (simulated with clip mask + animated vertical offset per digit)
- Priority: P1
- Complexity: Medium (2 days). Per-digit vertical offset animation within a content-masked column.

**ParticleEmitter** - Lightweight particle system using GPUI Path primitives. Configurable: spawn rate, lifetime, velocity, size curve, color curve, gravity, spread angle.
- Enables: Confetti on achievement, snow/rain ambient effects, spark effects on actions, fireworks.
- GPUI fork: No (uses Path for small shapes or tiny quads)
- Priority: P2
- Complexity: High (4 days). Entity-based state with particle pool, per-frame physics step, path rendering.

**PulseIndicator** - Pulsing dot/ring for live status. Concentric rings scale outward and fade, repeating.
- Enables: Live/online status, recording indicator, unread notification badge, real-time data freshness.
- GPUI fork: No
- Priority: P1
- Complexity: Low (0.5 day). Animated scale + opacity on concentric ring divs.

### B. Visual Effect Components

**GlassMorphism** - Container with layered frosted-glass aesthetic. Semi-transparent background, subtle border, noise texture overlay via micro-path pattern, optional tinted color wash.
- Enables: Floating panels, sidebar overlays, media control overlays, modal backdrops with depth.
- GPUI fork: Partial (true backdrop-blur needs fork Extension 4; noise overlay and tint layers are userland)
- Priority: P0 (defining modern desktop aesthetic)
- Complexity: Medium (2 days) for userland approximation. Layered semi-transparent backgrounds + subtle noise via Path dots.

**GradientBorder** - Border effect achieved by rendering a slightly larger background div with gradient fill, overlaid by a child with solid background, creating visible gradient edges.
- Enables: Premium card borders, feature highlights, animated focus rings, neon glow borders.
- GPUI fork: No
- Priority: P1
- Complexity: Low (0.5 day). Nested div technique, well-established pattern.

**GradientText** - Text rendered with gradient colors by splitting text into individual character spans, each colored at the interpolated gradient position.
- Enables: Hero headings, branding text, feature highlights, decorative titles.
- GPUI fork: No (uses per-character color interpolation)
- Priority: P1
- Complexity: Medium (2 days). Character-level span splitting, color interpolation across text width.

**Spotlight** - Radial highlight effect tracking cursor position. Uses a large box-shadow with offset matching cursor coordinates relative to element.
- Enables: Feature callouts, interactive card backgrounds, pricing table hover, hero section interactivity.
- GPUI fork: Partial (radial gradient makes it better, but box-shadow approach works)
- Priority: P1
- Complexity: Medium (2 days). Mouse move tracking + dynamic box-shadow offset.

**Aurora** - Animated background using overlapping large blobs (oversized rounded-corner divs with semi-transparent fills) that drift slowly. Creates organic color-shifting background.
- Enables: App backgrounds, hero sections, onboarding screens, premium decorative panels.
- GPUI fork: No (uses multiple animated oversized divs with large border radius)
- Priority: P2
- Complexity: Medium (2 days). Multiple animated position/color divs layered with overflow hidden.

**DotPattern / GridPattern** - Repeating dot grid or line grid backgrounds using GPUI Path primitives. Configurable spacing, color, size.
- Enables: App backgrounds, empty states, wireframe aesthetics, graph paper effect.
- GPUI fork: No
- Priority: P2
- Complexity: Low (1 day). Path-based dot/line rendering in a grid.

**Noise** - Procedural visual noise texture overlay using scattered micro-paths or tiny quads at pseudo-random positions with varied opacity.
- Enables: Background texture grain, film grain overlay, vintage effects, surface texture.
- GPUI fork: No
- Priority: P2
- Complexity: Medium (1.5 days). Deterministic pseudo-random point placement via hash function.

**Meteors** - Animated diagonal line streaks across a container. Lines spawn at random positions along one edge, travel diagonally, fade out.
- Enables: Decorative hero backgrounds, space-themed UIs, loading screens.
- GPUI fork: No
- Priority: P2
- Complexity: Low (1 day). Animated Path lines with opacity fade.

### C. Typography & Text Components

**AnimatedText** - Per-character stagger animations: fade-in, slide-up, wave, scale-in. Each character animates with a small delay offset.
- Enables: Hero text entrances, feature reveals, loading messages with personality, heading animations.
- GPUI fork: No (individual character spans with staggered with_animation)
- Priority: P1
- Complexity: Medium (2 days). Character splitting + per-char animation with delay offsets.

**TextReveal** - Word-by-word or line-by-line text reveal with configurable direction. Content clips to revealed region.
- Enables: Scroll-triggered content, storytelling UIs, progressive disclosure, AI response streaming.
- GPUI fork: No
- Priority: P1
- Complexity: Medium (1.5 days). Word/line splitting + animated content mask height/width.

**TextHighlight** - Animated background highlight sweep on inline text. Background color animates from left-to-right across the text span.
- Enables: Search result highlighting with motion, educational content, reading position indicators.
- GPUI fork: No
- Priority: P1
- Complexity: Low (1 day). Animated background width on text span.

**CodeBlock** - Syntax-highlighted code display with line numbers, language badge, and copy button. Manual token coloring using theme-derived syntax colors.
- Enables: Documentation, code examples in apps, developer tools, configuration display.
- GPUI fork: No
- Priority: P0 (every developer-facing app needs this)
- Complexity: Medium (3 days). Token-based coloring, line number gutter, scroll for long blocks. Could integrate tree-sitter or use simple regex-based highlighting.

**KBD** - Styled keyboard shortcut display badge. Consistent sizing, theme-aware colors, subtle shadow for key-cap appearance.
- Enables: Shortcut hints in menus, documentation, command palette entries, tooltips.
- GPUI fork: No
- Priority: P1
- Complexity: Low (0.5 day). Styled div with border, shadow, monospace text.

### D. Data Visualization Components

**AreaChart** - Line chart with filled area below using GPUI Path. Supports stacked areas, gradient fills (top-to-bottom fade), and animated data transitions.
- Enables: Revenue over time, resource utilization, analytics trends, stock charts.
- GPUI fork: No (Path fill + existing 2-stop gradient for area fade)
- Priority: P0 (most requested chart type after line/bar)
- Complexity: Medium (2 days). Extend LineChart with closed path fill and gradient background.

**DonutChart** - PieChart variant with configurable inner radius cutout and center label/value display.
- Enables: Summary statistics, budget breakdowns, storage usage, completion percentages.
- GPUI fork: No (extend existing PieChart)
- Priority: P1
- Complexity: Low (1 day). Add inner_radius to PieChart, render center content.

**RadarChart** - Spider/radar chart with polygon shapes via GPUI Path. Multiple overlaid semi-transparent datasets.
- Enables: Skill assessments, product feature comparisons, performance metrics, multi-axis analysis.
- GPUI fork: No
- Priority: P1
- Complexity: Medium (3 days). Polar coordinate math, Path polygon rendering, axis labels.

**Heatmap** - Grid of colored cells with value-to-color mapping. Configurable color scales, tooltips, and labels.
- Enables: GitHub-style contribution calendars, correlation matrices, schedule views, activity maps.
- GPUI fork: No
- Priority: P1
- Complexity: Medium (2 days). Grid of colored quads with legend.

**Gauge** - Semicircular or circular gauge with animated needle/fill arc. Configurable ranges, colors, and labels.
- Enables: CPU/memory meters, speedometers, score indicators, health dashboards.
- GPUI fork: No (Path-based arc rendering)
- Priority: P1
- Complexity: Medium (2 days). Arc path rendering, animated value interpolation.

**TreeMap** - Nested rectangles representing hierarchical data proportionally sized by value.
- Enables: Disk usage visualization, portfolio allocation, org budgets, code repository size analysis.
- GPUI fork: No
- Priority: P2
- Complexity: High (4 days). Squarified treemap algorithm, recursive rectangle subdivision.

### E. Media & Canvas Components

**Canvas** - Direct GPUI painting surface. User provides a paint callback that receives the bounds and window context, can draw arbitrary Paths and Quads.
- Enables: Custom visualizations, drawing tools, game UIs, generative art, specialized charts, signature capture.
- GPUI fork: No (wraps existing Path/Quad paint APIs)
- Priority: P0 (escape hatch for anything the component library does not cover)
- Complexity: Medium (2 days). Element that delegates its paint phase to a user closure.

**SVGRenderer** - Parses SVG path data (M, L, C, Z commands) and renders via GPUI Path. Supports basic shapes, paths, fill/stroke colors, viewBox scaling.
- Enables: Custom icons beyond the icon set, illustrations, logos, complex graphics without images.
- GPUI fork: No
- Priority: P1
- Complexity: High (5 days). SVG path parser, bezier curve to GPUI Path conversion, coordinate transforms.

**QRCode** - QR code generator rendering modules as small colored quads. Configurable size, colors, error correction level.
- Enables: Share links, device pairing, 2FA setup, payment flows, WiFi sharing.
- GPUI fork: No (use qrcode crate for generation, render as grid of quads)
- Priority: P1
- Complexity: Medium (2 days). QR generation via crate, grid rendering.

**Waveform** - Audio waveform visualization using vertical bars (quads) or continuous Path. Real-time or pre-computed amplitude data.
- Enables: Audio editors, music players, voice message previews, podcast apps.
- GPUI fork: No
- Priority: P2
- Complexity: Medium (2 days). Amplitude bar rendering with optional animation.

**CropArea** - Image crop tool with draggable/resizable selection rectangle, aspect ratio constraints, and dimmed overlay on non-selected region.
- Enables: Avatar upload, image editors, screenshot annotation, document scanning.
- GPUI fork: No
- Priority: P2
- Complexity: High (4 days). Drag handles, aspect ratio math, overlay mask rendering.

### F. Layout Animation Components

**LayoutTransition** - Wraps a layout region and automatically animates position/size changes of children between renders. Snapshots previous bounds per key, interpolates to new bounds on re-render.
- Enables: Smooth reordering, filter/sort animations on grids, accordion expansion, any layout change becoming animated automatically.
- GPUI fork: No (snapshots element bounds in after_layout, animates via margin/size adjustments)
- Priority: P0 (the single most impactful animation pattern for perceived polish)
- Complexity: Very High (7 days). Requires per-child bound snapshotting, key-based matching across renders, animation of position offset and size.

**SharedElementTransition** - "Hero animation" between views. An element appears to fly from one location to another. Snapshots source element position, renders animated clone traveling to target position.
- Enables: Photo grid to detail view, card to full page, avatar to profile, list item to detail.
- GPUI fork: No (position snapshotting + animated absolute-positioned clone)
- Priority: P1
- Complexity: Very High (7 days). Cross-view position coordination, element cloning during transition, z-index management.

**AnimatedCollapsible** - Enhancement to existing Collapsible with smooth height animation. Measures content height, animates clip region and container height from 0 to measured value.
- Enables: Sidebar sections, accordion panels, FAQ lists, settings groups, expandable descriptions.
- GPUI fork: No
- Priority: P0 (basic UX expectation; current collapsible snaps)
- Complexity: Medium (2 days). Measure content height, animate container height + content mask.

**AnimatedSwitch** - Animates between two content regions with slide, scale, or morph options (extends ContentTransition beyond just crossfade).
- Enables: Tab content with slide direction, step-by-step wizards, toggle views, before/after comparisons.
- GPUI fork: No
- Priority: P1
- Complexity: Medium (2 days). Multiple transition modes: slide-left, slide-right, scale, flip.

**Dock** - macOS Dock-style magnification. Items in a row scale up based on distance to cursor, with spring-based animation.
- Enables: Application launchers, tool palettes, icon grids, creative app toolbars.
- GPUI fork: Partial (true scale transform is better, but can approximate with dynamic width/height)
- Priority: P2
- Complexity: High (4 days). Per-item scale calculation based on cursor proximity, spring animation on scale values.

### G. Navigation & Transition Components

**ViewRouter** - Simple view stack manager with push/pop semantics and animated transitions. Maintains a stack of views with configurable enter/exit animation pairs.
- Enables: Multi-screen apps, settings hierarchies, wizard flows, detail drill-down patterns.
- GPUI fork: No
- Priority: P0 (every non-trivial app needs navigation)
- Complexity: High (4 days). View stack, transition coordinator, back navigation, gesture support for swipe-back.

**PageTransition** - Configurable enter/exit animation pairs for ViewRouter. Presets: slide-left, slide-right, fade, scale-up, slide-up (modal style).
- Enables: Platform-appropriate navigation animations, custom branded transitions.
- GPUI fork: No
- Priority: P0 (companion to ViewRouter)
- Complexity: Medium (1.5 days). Animation configuration struct + integration with ViewRouter lifecycle.

**SegmentedNavigation** - Animated sliding highlight/underline on the active segment. Highlight element smoothly translates to the position of the selected item.
- Enables: View mode switching, filter tabs, category selection, settings section tabs.
- GPUI fork: No
- Priority: P1
- Complexity: Medium (2 days). Measure item positions, animate highlight offset.

**DrawerNavigation** - Gesture-driven side drawer. Swipe from edge to open, swipe to close. Animated slide with backdrop dim.
- Enables: Mobile-style navigation on desktop, settings panels, secondary navigation, filter panels.
- GPUI fork: No
- Priority: P1
- Complexity: Medium (2 days). Gesture integration + animated slide + backdrop.

### H. Micro-interaction Components

**CopyButton** - Button with clipboard write and animated checkmark confirmation feedback. After click, icon morphs from copy to checkmark, reverts after timeout.
- Enables: Code blocks, URL sharing, configuration display, API key display.
- GPUI fork: No
- Priority: P0 (trivial to build, used everywhere)
- Complexity: Low (0.5 day). Clipboard write + icon swap with timer reset.

**SkeletonLoader** - Wrapper that renders shimmer placeholders matching approximate dimensions of its children until data is available. Toggles between skeleton and real content.
- Enables: Any async data loading with polished placeholders. Cards, lists, profiles, dashboards.
- GPUI fork: No
- Priority: P0
- Complexity: Medium (2 days). Dimension estimation from children, shimmer rendering, transition to real content.

**MagneticButton** - Button whose position subtly shifts toward the cursor when nearby. Spring-animated return to center when cursor leaves proximity.
- Enables: CTAs, primary actions, playful interfaces, creative apps.
- GPUI fork: No (uses dynamic margin offset based on cursor position)
- Priority: P1
- Complexity: Medium (1.5 days). Cursor proximity detection, spring-animated offset.

**TiltCard** - Card with 3D tilt illusion based on cursor position. Uses asymmetric box-shadow displacement and subtle padding shifts to simulate perspective.
- Enables: Product cards, portfolio items, interactive previews, pricing tables.
- GPUI fork: Partial (real transform-based tilt requires Extension 1; shadow-based illusion is userland)
- Priority: P1
- Complexity: Medium (2 days) for shadow-based illusion. Low (0.5 day) additional once transforms exist.

**ExpandableCard** - Card that expands to fill a larger area (or full screen) with animated content reveal. Source position animates to target size.
- Enables: Dashboard widget expansion, email preview to full view, image lightbox from grid.
- GPUI fork: No
- Priority: P1
- Complexity: Medium (3 days). Position snapshot, animated size/position interpolation, content transition.

**FloatingActionButton** - Fixed-position circular button with expand-on-click revealing a radial or vertical list of action buttons (speed dial pattern).
- Enables: Quick-create actions, compose button, mobile-style FAB on desktop.
- GPUI fork: No
- Priority: P1
- Complexity: Medium (2 days). Animated expand/collapse of action items with stagger.

**Confetti** - Celebration particle burst triggered on events. Pre-configured ParticleEmitter with confetti-specific physics (slow fall, rotation, varied colors/shapes).
- Enables: Achievement celebrations, form submission success, game rewards, onboarding completion.
- GPUI fork: No (depends on ParticleEmitter or standalone simpler implementation)
- Priority: P2
- Complexity: Low (1 day) if ParticleEmitter exists; Medium (3 days) standalone.

---

## 2. GPUI FORK EXTENSIONS REQUIRED

### Extension 1: Element Transforms on Quads [P0]

**What it enables**: rotate(), scale(), skewX(), skewY(), translate() on any quad element. This is the single biggest rendering gap vs the web platform.

**Unlocks**: Card flips, carousel 3D effects (coverflow), icon rotation animations, loading spinner rotation, page curl transitions, parallax scroll effects, hover lift with real scale, Dock magnification, image rotation in editors.

**Technical approach**:
1. Add `transform: Option<TransformationMatrix>` field to `Quad` in `gpui/src/scene.rs`
2. Add `transform_origin: Option<Point<f32>>` (0.0-1.0 relative to element bounds)
3. Metal shader (`quad.metal`): In vertex function, translate to origin, apply 2D affine transform matrix, translate back, then apply projection. The `TransformationMatrix` struct already exists for sprites.
4. WGSL shader (`quad.wgsl`): Same vertex transform. WGSL has mat3x3<f32> for 2D affine.
5. HLSL shader (`quad.hlsl`): Same vertex transform.
6. Expose on `Styled` trait via Refineable macro: `rotate: Option<f32>`, `scale: Option<Point<f32>>`, `translate: Option<Point<Pixels>>`, `transform_origin: Option<Point<f32>>`
7. In `Style::apply()`, compose these into a single `TransformationMatrix`

**Complexity**: High (5-7 days)
- Day 1-2: Quad struct changes, Scene serialization, instance buffer layout update
- Day 3-4: Shader changes across 3 backends
- Day 5: Styled trait integration, Refineable fields
- Day 6-7: Testing, edge cases (hit testing with transforms, content masks under transform)

**Risk**: Hit testing (mouse events) must account for transforms. Content masks (clipping) under transforms need correct math.

**Dependencies**: TiltCard (real), Dock (real scale), Carousel 3D, spinner rotation, all rotation-based animations.

### Extension 2: Multi-stop Linear Gradients [P1]

**What it enables**: Rich gradient backgrounds, shimmer effects, gradient progress bars, aurora backgrounds, heatmap color scales.

**Technical approach**:
1. Extend gradient from `[Hsla; 2]` to `SmallVec<[(Hsla, f32); 8]>` (color + stop position, max 8 stops)
2. Add `stop_count: u32` field
3. Shader: Piecewise linear interpolation. For position t, find the two surrounding stops, lerp between them.
4. Instance buffer layout: Pack up to 8 color-stop pairs. Use a fixed-size array (32 floats for 8 RGBA stops + 8 position floats = 40 floats).
5. 2-stop gradients use fast path (no loop).

**Complexity**: Medium (3-4 days)
- Day 1: Rust struct changes, instance buffer layout
- Day 2: Metal shader
- Day 3: WGSL + HLSL shaders
- Day 4: API surface, testing

**Dependencies**: Aurora (better), Shimmer (better), GradientText (better), gradient borders, animated backgrounds.

### Extension 3: Radial Gradients [P1]

**What it enables**: Spotlight effects, vignettes, circular progress indicators, radial menus, depth perception.

**Technical approach**:
1. New `RadialGradient` struct: center (Point<f32>), radius (f32), color stops (reuse multi-stop struct)
2. New primitive type or variant field on existing gradient
3. Shader: Compute distance from fragment to center, normalize by radius, use as t for stop interpolation
4. Supports elliptical gradients by separate x/y radii

**Complexity**: Medium (3-4 days). Shader math is straightforward. Reuses multi-stop interpolation logic.

**Dependencies**: Spotlight (true), Gauge (gradient fill), vignette overlay, circular progress.

### Extension 4: Per-element Gaussian Blur [P1]

**What it enables**: True frosted glass (backdrop-filter), depth-of-field, blur-behind overlays, image background blur, focus/defocus transitions.

**Technical approach**:
1. Add render-to-texture capability: elements marked for blur render their backdrop to an off-screen texture
2. Implement separable Gaussian blur as two-pass shader (horizontal + vertical)
3. For `.blur(radius)`: Render element to texture, apply blur, composite back
4. For `.backdrop_blur(radius)`: Render everything below element to texture, blur, composite, then render element on top
5. Configurable blur radius (px)

**Complexity**: Very High (10-14 days). This is the most invasive GPUI change because it requires:
- Off-screen render target allocation and management
- Multi-pass rendering (currently single-pass)
- Texture readback and recompositing
- Memory management for temporary textures

**Fallback**: Without this, GlassMorphism uses layered semi-transparent backgrounds (already implemented in StyledExt::glass). macOS window-level blur can serve as partial workaround.

**Dependencies**: True GlassMorphism, frosted sidebars, overlay backdrop blur, depth-of-field effects.

### Extension 5: Per-element Opacity [P1]

**What it enables**: Opacity on container elements affecting all children uniformly.

**Technical approach (simple)**:
1. Add `opacity: Option<f32>` to Style
2. In paint phase, multiply all descendant colors' alpha by parent opacity
3. This is correct for non-overlapping children

**Technical approach (correct)**:
1. Render subtree to texture
2. Composite with opacity
3. Handles overlapping children correctly (semi-transparent siblings don't show through each other)

**Complexity**: Medium (3 days) for simple approach. Very High (bundled with Extension 4) for correct approach.

**Dependencies**: AnimatedPresence (fade entire subtrees), overlay transitions, disabled state dimming, drag ghost elements.

### Extension 6: Conic Gradients [P2]

**What it enables**: Sweep-style progress indicators, color wheels, pie-chart-without-Path.

**Technical approach**:
1. Use `atan2(y - center.y, x - center.x)` in fragment shader to compute angle
2. Map angle (0..2PI) to color stops
3. New gradient variant

**Complexity**: Medium (2-3 days). Well-known shader math.

**Dependencies**: Gauge (sweep variant), color wheel, circular progress alternatives.

### Extension 7: Blend Modes [P2]

**What it enables**: Duotone images, texture overlays, creative compositing, difference blending for contrast-adaptive text.

**Technical approach**:
1. Add `BlendMode` enum to paint operations
2. Set blend state per draw call in Metal/WGSL/HLSL
3. Modes: Multiply, Screen, Overlay, SoftLight, Difference, Exclusion

**Complexity**: Medium (3-4 days). GPU APIs support blend states natively.

**Dependencies**: Image overlays, duotone effects, creative compositing.

---

## 3. PHASED IMPLEMENTATION ROADMAP

### Phase 1: Userland Quick Wins (Weeks 1-3)

No GPUI fork changes required. High impact, immediately shippable.

**Week 1 - Animation Foundations**
| Component | Complexity | Days |
|-----------|-----------|------|
| AnimatedPresence | Medium | 2 |
| AnimatedCounter | Medium | 2 |
| Shimmer | Low | 1 |

**Week 2 - Visual Polish & Text**
| Component | Complexity | Days |
|-----------|-----------|------|
| CodeBlock | Medium | 3 |
| GradientBorder | Low | 0.5 |
| CopyButton | Low | 0.5 |
| KBD | Low | 0.5 |
| PulseIndicator | Low | 0.5 |
| GlassMorphism (userland approx) | Medium | 1 |

**Week 3 - Layout & Navigation**
| Component | Complexity | Days |
|-----------|-----------|------|
| ViewRouter + PageTransition | High + Med | 5 |
| AnimatedCollapsible | Medium | 2 |
| AnimatedSwitch | Medium | 2 |

**Phase 1 Deliverable**: 12 new components. Every animation pattern that does not require transforms or blur. ViewRouter gives apps real navigation.

### Phase 2: Core GPUI Extensions (Weeks 4-7)

**Week 4-5 - Element Transforms (Extension 1)**
- Quad struct + shader changes across Metal/WGSL/HLSL
- Styled trait: .rotate(), .scale(), .translate(), .transform_origin()
- Hit testing under transforms
- Testing across platforms

**Week 6 - Transform-dependent Components**
| Component | Complexity | Days |
|-----------|-----------|------|
| TiltCard (real transforms) | Medium | 1 |
| Dock (scale magnification) | High | 3 |
| MagneticButton | Medium | 1.5 |

Enhance existing components:
- Carousel: Add 3D coverflow transition using rotate + translate
- Spinner: Native rotation animation (not frame-based)

**Week 7 - Multi-stop + Radial Gradients (Extensions 2 & 3)**
| Extension | Complexity | Days |
|-----------|-----------|------|
| Multi-stop linear gradients | Medium | 3 |
| Radial gradients | Medium | 3 |

New components using gradients:
- Spotlight (radial gradient tracking cursor)
- Gauge (gradient arc fill)

**Phase 2 Deliverable**: Transform system (the biggest visual unlock), gradient extensions, 5 new components. This is the phase that makes apps look dramatically better.

### Phase 3: Advanced Effects & Data Viz (Weeks 8-11)

**Week 8-9 - Render-to-texture + Blur (Extension 4)**
- Off-screen render target infrastructure
- Separable Gaussian blur shader
- .blur(px) and .backdrop_blur(px) on Styled
- Per-element opacity (Extension 5) bundled in

**Week 10 - Blur-dependent Components**
| Component | Complexity | Days |
|-----------|-----------|------|
| GlassMorphism (true backdrop-blur) | Medium | 1 |
| Enhanced overlays with backdrop-blur | Low | 1 |
| AnimatedList | High | 4 |

**Week 11 - Data Visualization**
| Component | Complexity | Days |
|-----------|-----------|------|
| AreaChart | Medium | 2 |
| DonutChart | Low | 1 |
| RadarChart | Medium | 3 |
| Heatmap | Medium | 2 |
| Canvas | Medium | 2 |

**Phase 3 Deliverable**: Blur pipeline (true glassmorphism), 5 new chart types, AnimatedList, Canvas escape hatch.

### Phase 4: Premium Capabilities (Weeks 12-15)

**Week 12 - Advanced Animation Patterns**
| Component | Complexity | Days |
|-----------|-----------|------|
| LayoutTransition | Very High | 5 |
| SharedElementTransition | Very High | 5 |

**Week 13 - Rich Content**
| Component | Complexity | Days |
|-----------|-----------|------|
| SVGRenderer | High | 5 |
| QRCode | Medium | 2 |
| ParticleEmitter | High | 4 |
| Confetti (uses ParticleEmitter) | Low | 1 |

**Week 14 - Creative Polish**
| Extension/Component | Complexity | Days |
|-----------|-----------|------|
| Conic gradients (Extension 6) | Medium | 2.5 |
| Blend modes (Extension 7) | Medium | 3 |
| Aurora | Medium | 2 |
| DotPattern / GridPattern | Low | 1 |
| Noise | Medium | 1.5 |
| Meteors | Low | 1 |

**Week 15 - Remaining Components & Integration**
| Component | Complexity | Days |
|-----------|-----------|------|
| GradientText | Medium | 2 |
| AnimatedText | Medium | 2 |
| TextReveal | Medium | 1.5 |
| TextHighlight | Low | 1 |
| Marquee | Medium | 2 |
| TypeWriter | Medium | 2 |
| NumberTicker | Medium | 2 |
| AnimatedProgress | Low | 1 |
| TreeMap | High | 4 |
| Waveform | Medium | 2 |
| SkeletonLoader | Medium | 2 |
| SegmentedNavigation | Medium | 2 |
| DrawerNavigation | Medium | 2 |
| ExpandableCard | Medium | 3 |
| FloatingActionButton | Medium | 2 |
| CropArea | High | 4 |

(Week 15 is realistically 2-3 weeks of implementation; listed as single phase for planning.)

**Phase 4 Deliverable**: Layout animations (hero transitions), SVG, particles, blend modes, remaining 20+ components.

---

## 4. THE "ELECTRON KILLER" PITCH

### What Becomes Possible

After this roadmap, adabraka-ui enables native desktop applications with visual richness that matches or exceeds Electron, while delivering performance that web cannot touch.

**Performance that web cannot match:**
- Guaranteed 60fps without garbage collection pauses, layout thrashing, or style recalculation
- Sub-millisecond input latency (native event handling, no JS event loop)
- 50-100MB memory vs 300-800MB for Electron
- Instant cold start (< 200ms) vs 2-5 second Electron boot
- GPU-accelerated rendering with no "promote to compositor layer" hacks

**Apps that become viable:**
- **Creative tools**: Photo editors, design tools, DAWs - where input latency and rendering performance directly affect user experience. Figma-class tools without needing a browser.
- **Real-time dashboards**: Hundreds of animated charts, gauges, and live-updating values without frame drops. Financial terminals, monitoring systems, analytics platforms.
- **Developer tools**: Code editors, database browsers, API clients - VS Code polish at native performance, a fraction of the memory.
- **Communication apps**: Chat, email, collaboration tools - smooth animations, glassmorphic overlays, instant response. What Slack and Discord could be without Electron overhead.
- **Enterprise applications**: CRM, ERP, project management - complex data-heavy UIs that stay responsive with 100k+ row data grids, real-time updates, rich visualizations.

**Why developers choose adabraka-ui:**
1. **Same vocabulary**: 128+ components, shadcn-style builder pattern, familiar names (Button, Input, Select, Dialog, Sheet, Toast, DataGrid). React developers feel at home.
2. **Single binary**: Ship a 10-20MB binary, not a 200MB Electron installer bundling Chromium + Node.js.
3. **Rust safety**: No null pointer crashes, no "Cannot read property of undefined", no memory leaks. Type-safe component APIs with compile-time guarantees.
4. **Native integration**: Real system tray, native file dialogs, proper multi-window, OS-level accessibility. Not approximations through browser APIs.
5. **Visual quality**: Transforms, blur, gradients, spring physics, gesture-driven animations, GPU-rendered charts - all composable, all at 60fps. The visual tools web developers expect, running on metal.
6. **Performance is a feature**: Users feel the difference every day. Scrolling is smoother. Animations are silkier. The app launches instantly. These are not benchmarks - they are daily quality-of-life improvements that determine whether users love or tolerate their tools.

### The Bottom Line

The web won the UI war through ecosystem, not rendering quality. HTML/CSS/JS are mediocre rendering technologies hidden behind great tooling. adabraka-ui attacks from the other direction: build the component library first (128+ components), make the DX familiar (shadcn patterns, theme tokens, builder APIs), and deliver visual richness that feels premium - because every pixel is GPU-rendered without a browser engine in the way.

---

## Appendix A: Component Count

| Category | Existing | New (Phase 1-4) | Total |
|----------|----------|-----------------|-------|
| Animation & Motion | 6 modules | +10 | 16 |
| Visual Effects | 3 helpers | +8 | 11 |
| Typography & Text | 12 variants | +5 | 17 |
| Data Visualization | 4 chart types | +6 | 10 |
| Media & Canvas | 3 | +5 | 8 |
| Layout Animation | 1 | +5 | 6 |
| Navigation & Transition | 9 | +4 | 13 |
| Micro-interactions | 2 | +7 | 9 |
| **Total** | **85+** | **+43** | **128+** |

## Appendix B: GPUI Fork Extensions

| # | Extension | Priority | Complexity | Phase | Components Unlocked |
|---|-----------|----------|------------|-------|-------------------|
| 1 | Element Transforms | P0 | High (5-7d) | 2 | TiltCard, Dock, Carousel 3D, spinner rotation |
| 2 | Multi-stop Gradients | P1 | Medium (3-4d) | 2 | Aurora, Shimmer+, gradient borders |
| 3 | Radial Gradients | P1 | Medium (3-4d) | 2 | Spotlight, Gauge, vignette |
| 4 | Gaussian Blur | P1 | Very High (10-14d) | 3 | True GlassMorphism, backdrop-blur |
| 5 | Per-element Opacity | P1 | Medium (3d) | 3 | AnimatedPresence, fade transitions |
| 6 | Conic Gradients | P2 | Medium (2-3d) | 4 | Sweep gauge, color wheel |
| 7 | Blend Modes | P2 | Medium (3-4d) | 4 | Duotone, texture overlay |

## Appendix C: Risks & Mitigations

| Risk | Impact | Mitigation |
|------|--------|-----------|
| Render-to-texture too invasive for GPUI | Blocks blur, correct opacity | Use macOS window-level blur + layered semi-transparent quads as fallback. Ship GlassMorphism userland approximation in Phase 1. |
| Element transforms break hit testing | Mouse events miss transformed elements | Inverse-transform mouse coordinates in hit test. Add transform-aware bounds calculation. |
| Multi-stop gradient instance buffer too large | GPU memory pressure with many gradients | Cap at 8 stops (covers 99% of cases). Use fixed-size array, not dynamic allocation. |
| Too many new components to maintain | Quality regression, API churn | Phase 1 components are userland-only and can ship independently. Gate Phase 2+ on Phase 1 stability. |
| SharedElementTransition too complex | Cross-view state coordination is hard | Start with single-window transitions only. Multi-window hero animations are Phase 5+ if ever. |
