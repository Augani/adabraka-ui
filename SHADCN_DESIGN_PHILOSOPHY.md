# shadcn/ui Design Philosophy for adabraka-ui

## Core Principles

After studying shadcn/ui components, here are the key principles we should adopt:

### 1. **Unstyled by Default, Styled by Choice**
- Provide sensible defaults but **never lock users in**
- Every component should accept a `className`-equivalent for GPUI styling
- Users should be able to completely override our styles

### 2. **Variants System (CVA Pattern)**
```typescript
// shadcn approach:
const buttonVariants = cva(
  "base classes",  // Always applied
  {
    variants: {
      variant: { default: "...", destructive: "...", outline: "..." },
      size: { default: "...", sm: "...", lg: "..." }
    },
    defaultVariants: { variant: "default", size: "default" }
  }
)
```

**For GPUI/Rust:**
- Use enums for variants (we already do this!)
- Each variant has associated styling
- Users can override with `Styled` trait methods
- Provide `custom()` variant for complete user control

### 3. **Full Props Pass-Through**
```typescript
// shadcn pattern:
function Component({ className, ...props }) {
  return <Primitive className={cn(baseStyles, className)} {...props} />
}
```

**For GPUI/Rust:**
- Implement `Styled` trait (users can call any GPUI style method)
- Accept generic event handlers
- Don't restrict what users can do

### 4. **Composition Over Configuration**
- Small, focused components
- Users combine them to build complex UIs
- Example: Instead of `Button` with 20 options, have `Button` + `ButtonIcon` + `ButtonSpinner`

### 5. **data-slot Pattern**
```typescript
<Component data-slot="button">
  <Icon data-slot="icon" />
</Component>
```

**For GPUI/Rust:**
- Use IDs strategically for targeting
- Allow users to style child elements
- Document the component structure

## What This Means for adabraka-ui

### Current Issues:

1. **Text Component**:
   ```rust
   // ❌ Too opinionated
   Text::new("Hello").variant(TextVariant::H1)  // Forces our H1 styling

   // ✅ Should allow:
   Text::new("Hello")
       .variant(TextVariant::H1)  // Optional base styling
       .text_size(px(48.0))       // User override
       .font_weight(FontWeight::BOLD)
       .italic()                  // Should work!
       .strikethrough()           // Should work!
   ```

2. **Missing: User Control**:
   - Italic/strikethrough exist in API but don't work
   - Users can't easily override variant styles
   - No way to say "use variant as base, but customize"

### Solutions:

#### 1. Use `StyledText` for Rich Text Support

```rust
// Enable italic, strikethrough, underline, highlights
div()
    .child(StyledText::new(content)
        .with_highlights(highlights)  // For search, emphasis
    )
```

#### 2. Make Variants Optional Helpers

```rust
// Variants are just convenient presets
impl Text {
    pub fn h1(content: impl Into<SharedString>) -> Self {
        Self::new(content)
            .size(px(32.0))
            .weight(FontWeight::BOLD)
            .line_height(1.2)
            // But users can still override everything!
    }
}
```

#### 3. Full Styled Trait Support

```rust
// Already done! ✅
Text::new("Content")
    .variant(TextVariant::Body)  // Base styling
    .p_4()                       // User adds padding
    .bg(gpui::blue())            // User adds background
    .rounded_lg()                // User adds border radius
```

#### 4. Support All Text Decorations

```rust
Text::new("Important")
    .italic(true)           // Should use StyledText internally
    .strikethrough(true)    // Should render properly
    .underline(true)        // Already works
    .weight(FontWeight::BOLD)
```

## Implementation Plan

### Phase 1: Text Component Redesign ✅ (Partially Done)
- [x] Add `Styled` trait implementation
- [ ] Switch to `StyledText` for rendering
- [ ] Make italic/strikethrough actually work
- [ ] Add text highlighting support
- [ ] Document override patterns

### Phase 2: Button Component Enhancement
- [ ] Review against shadcn button
- [ ] Ensure full style override capability
- [ ] Add variant system if not comprehensive
- [ ] Document customization patterns

### Phase 3: All Components Audit
- [ ] Ensure every component implements `Styled`
- [ ] Ensure users can override all styling
- [ ] Add "Custom" variants where appropriate
- [ ] Document extension patterns

## Key Takeaways

### DO:
✅ Provide sensible defaults
✅ Make everything overridable
✅ Use variants as convenience, not constraints
✅ Implement `Styled` trait everywhere
✅ Pass through all props/handlers
✅ Document customization patterns
✅ Think "composition over configuration"

### DON'T:
❌ Force users into our styling decisions
❌ Hide or restrict GPUI capabilities
❌ Make features that don't actually work (italic, strikethrough)
❌ Create monolithic components with 50 options
❌ Prevent users from accessing underlying elements

## shadcn Quote

> "Copy and paste the code into your project and customize to your needs. The code is yours."

**Our equivalent:**
> "Use our components as a starting point. Implement `Styled` trait to customize everything. The API is flexible, the control is yours."

## Security & Error Handling

Following shadcn's approach:
- **Type safety** - Use Rust's type system (we do this ✅)
- **Error proof** - Variants prevent invalid states
- **Accessibility** - Include ARIA patterns (TODO)
- **Performance** - Don't add overhead (remove unnecessary wrappers ✅)

## Examples of Good vs Bad

### ❌ Bad (Too Restrictive):
```rust
struct Text {
    content: String,
    variant: TextVariant,  // Can't customize if you use a variant
}
```

### ✅ Good (Flexible):
```rust
struct Text {
    content: SharedString,
    variant: TextVariant,       // Optional base styling
    size: Option<Pixels>,       // User can override
    weight: Option<FontWeight>, // User can override
    style: StyleRefinement,     // User has full control via Styled trait
}
```

## References

- [shadcn/ui GitHub](https://github.com/shadcn-ui/ui)
- [Class Variance Authority (CVA)](https://cva.style/docs)
- [GPUI Styled Trait](https://docs.rs/gpui)
