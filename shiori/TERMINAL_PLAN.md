# Terminal Enhancement Plan for Claude Code

## Phase 1: Core TUI Support (Critical) ✅ COMPLETE

### 1.1 Line Drawing Characters (G0/G1 Character Sets) ✅
- Handle `ESC ( 0` - Switch to DEC Special Graphics (line drawing)
- Handle `ESC ( B` - Switch back to ASCII
- Handle `SI` (0x0F) and `SO` (0x0E) for charset shifting
- Map line drawing characters (┌ ┐ └ ┘ ─ │ etc.)
- Files: `ansi_parser.rs`, `terminal_state.rs`

### 1.2 Synchronized Output (DEC Mode 2026) ✅
- Handle `CSI ? 2026 h` - Begin synchronized update
- Handle `CSI ? 2026 l` - End synchronized update
- Files: `ansi_parser.rs`, `terminal_state.rs`

### 1.3 Focus Events ✅
- Track window focus state
- Send `ESC [ I` when window gains focus
- Send `ESC [ O` when window loses focus
- Only send if focus_tracking is enabled
- Files: `terminal_view.rs`

## Phase 2: Text Rendering Enhancements ✅ COMPLETE

### 2.1 Bold/Italic Font Support ✅
- Use proper bold font weight for bold text
- Use italic font style for italic text
- Combine bold+italic when both set
- Files: `terminal_view.rs`

### 2.2 Underline/Strikethrough Rendering ✅
- Render underline decoration
- Render strikethrough decoration via `.line_through()`
- Files: `terminal_view.rs`

### 2.3 Cursor Rendering Improvements ✅
- Blinking cursor support
- Proper cursor colors (foreground/background swap)
- Block, underline, and bar cursor styles

## Phase 3: Advanced Features ✅ COMPLETE

### 3.1 Hyperlink Support (OSC 8) ✅
- Parse `OSC 8 ; params ; url ST`
- Store link info per cell
- Render links with underline styling
- Cmd+click to open URL
- Files: `ansi_parser.rs`, `terminal_state.rs`, `terminal_view.rs`

### 3.2 Selection Improvements ✅
- Visual highlight of selected text
- Double-click to select word
- Triple-click to select line
- Cmd+C to copy selection
- Files: `terminal_view.rs`

### 3.3 OSC 52 Clipboard ✅
- Parse `OSC 52 ; c ; base64-data ST`
- Write decoded data to clipboard
- Built-in base64 decoder
- Files: `ansi_parser.rs`, `terminal_view.rs`

## Phase 4: Polish ✅ COMPLETE

### 4.1 Desktop Notifications (OSC 9/777) ✅
- Parse `OSC 9 ; message ST` (iTerm2 style)
- Parse `OSC 777 ; notify ; title ; body ST`
- Infrastructure ready (actual notification display TBD when GPUI supports it)
- Files: `ansi_parser.rs`, `terminal_view.rs`

### 4.2 Window Title Updates ✅
- OSC 0/1/2 properly update tab title
- Title shown in terminal tab bar
- Files: `terminal_view.rs`, `app.rs`

### 4.3 Image Support (Future/Deferred)
- Sixel graphics (complex, deferred)
- iTerm2 inline images (complex, deferred)

## Implementation Status

| Phase | Feature | Status |
|-------|---------|--------|
| 1.1 | Line Drawing Characters | ✅ Done |
| 1.2 | Synchronized Output | ✅ Done |
| 1.3 | Focus Events | ✅ Done |
| 2.1 | Bold/Italic Fonts | ✅ Done |
| 2.2 | Underline/Strikethrough | ✅ Done |
| 2.3 | Cursor Rendering | ✅ Done |
| 3.1 | Hyperlinks (OSC 8) | ✅ Done |
| 3.2 | Selection (word/line) | ✅ Done |
| 3.3 | OSC 52 Clipboard | ✅ Done |
| 4.1 | Notifications | ✅ Done |
| 4.2 | Window Title | ✅ Done |
| 4.3 | Image Support | Deferred |

## Files Modified

| File | Changes |
|------|---------|
| `ansi_parser.rs` | G0/G1 charset, sync output, OSC 8/52/9/777, base64 decode |
| `terminal_state.rs` | Charset tracking, hyperlink storage per cell |
| `terminal_view.rs` | Focus events, strikethrough, hyperlink rendering/click, word/line selection, clipboard handling |
| `app.rs` | Fullscreen toggle, focus tracking |
| `Cargo.toml` | Added `open` crate for URL handling |

## Key Features Summary

- **TUI Support**: Full line drawing character support for box-drawing in TUI apps like htop, vim, etc.
- **Rich Text**: Bold, italic, underline, strikethrough rendering
- **Hyperlinks**: OSC 8 hyperlinks with Cmd+click to open, visual underline
- **Selection**: Single click drag, double-click word select, triple-click line select
- **Clipboard**: OSC 52 allows terminal apps to write to clipboard
- **Notifications**: Infrastructure for OSC 9/777 notifications
- **Focus Events**: Terminal apps can track focus state
