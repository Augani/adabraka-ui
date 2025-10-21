# Fonts Directory

This directory contains the font files used by the adabraka-ui component library.

## Adding Fonts

The library is configured to use **Inter** for UI text and **JetBrains Mono** for monospace text. To enable custom fonts:

### 1. Download the fonts

**Inter** (UI Font):
- Download from: https://rsms.me/inter/
- Get the TrueType (TTF) files for:
  - Inter-Regular.ttf
  - Inter-Medium.ttf
  - Inter-SemiBold.ttf
  - Inter-Bold.ttf

**JetBrains Mono** (Monospace Font):
- Download from: https://www.jetbrains.com/lp/mono/
- Get the TrueType (TTF) files for:
  - JetBrainsMono-Regular.ttf
  - JetBrainsMono-Bold.ttf

### 2. Place font files in this directory

Copy the downloaded `.ttf` files into this `assets/fonts/` directory.

### 3. Enable font loading

In `src/lib.rs`, uncomment these lines:

```rust
// Uncomment this:
pub mod fonts;

// And in the init() function, uncomment:
fonts::register_fonts(cx);
```

### 4. Using fonts in your application

The fonts will be automatically registered when you call `adabraka_ui::init(cx)`:

```rust
use adabraka_ui;

Application::new().run(|cx| {
    // Initialize UI (registers fonts automatically)
    adabraka_ui::init(cx);

    // Install theme
    adabraka_ui::theme::install_theme(cx, adabraka_ui::theme::Theme::dark());

    // Your app code...
});
```

## Using Different Fonts

To use different fonts:

1. Update the font constants in `src/fonts.rs`:
   ```rust
   pub const UI_FONT_FAMILY: &str = "YourFontName";
   pub const UI_MONO_FONT_FAMILY: &str = "YourMonoFontName";
   ```

2. Update the `include_bytes!` paths to match your font files

3. Update the registration calls in `register_fonts()` function

## Applying Fonts to Components

Once fonts are registered, you can use them with GPUI's font APIs:

```rust
use gpui::*;

// Use the UI font
div()
    .font_family(adabraka_ui::fonts::ui_font_family())
    .child("Text with custom font")

// Use the mono font
div()
    .font_family(adabraka_ui::fonts::mono_font_family())
    .child("Code with monospace font")
```

The theme system will automatically use these fonts throughout the component library once registered.
