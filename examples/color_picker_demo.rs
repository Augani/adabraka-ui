use adabraka_ui::{
    prelude::*,
    components::{scrollable::scrollable_vertical, color_picker::ColorPickerState},
};
use gpui::*;
use std::path::PathBuf;

struct Assets {
    base: PathBuf,
}

impl gpui::AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<std::borrow::Cow<'static, [u8]>>> {
        std::fs::read(self.base.join(path))
            .map(|data| Some(std::borrow::Cow::Owned(data)))
            .map_err(|err| err.into())
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        std::fs::read_dir(self.base.join(path))
            .map(|entries| {
                entries
                    .filter_map(|entry| {
                        entry
                            .ok()
                            .and_then(|entry| entry.file_name().into_string().ok())
                            .map(SharedString::from)
                    })
                    .collect()
            })
            .map_err(|err| err.into())
    }
}

fn main() {
    Application::new()
        .with_assets(Assets {
            base: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
        })
        .run(|cx| {
            adabraka_ui::init(cx);
            adabraka_ui::set_icon_base_path("assets/icons");
            install_theme(cx, Theme::dark());

            cx.open_window(
                WindowOptions {
                    titlebar: Some(TitlebarOptions {
                        title: Some("ColorPicker Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(900.0), px(800.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(ColorPickerDemo::new),
            )
            .unwrap();
        });
}

struct ColorPickerDemo {
    basic_state: Entity<ColorPickerState>,
    alpha_state: Entity<ColorPickerState>,
    custom_swatches_state: Entity<ColorPickerState>,
    styled_state: Entity<ColorPickerState>,
    disabled_state: Entity<ColorPickerState>,
    background_state: Entity<ColorPickerState>,
    text_state: Entity<ColorPickerState>,
}

impl ColorPickerDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            basic_state: cx.new(|_| ColorPickerState::new(hsla(210.0, 0.7, 0.5, 1.0))),
            alpha_state: cx.new(|_| ColorPickerState::new(hsla(120.0, 0.6, 0.4, 0.5))),
            custom_swatches_state: cx.new(|_| ColorPickerState::new(hsla(0.0, 0.8, 0.5, 1.0))),
            styled_state: cx.new(|_| ColorPickerState::new(hsla(270.0, 0.6, 0.5, 1.0))),
            disabled_state: cx.new(|_| ColorPickerState::new(hsla(180.0, 0.6, 0.5, 1.0))),
            background_state: cx.new(|_| ColorPickerState::new(hsla(220.0, 0.2, 0.15, 1.0))),
            text_state: cx.new(|_| ColorPickerState::new(hsla(60.0, 0.9, 0.6, 1.0))),
        }
    }
}

impl Render for ColorPickerDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();
        let basic_color = self.basic_state.read(cx).selected_color();
        let alpha_color = self.alpha_state.read(cx).selected_color();
        let custom_swatches_color = self.custom_swatches_state.read(cx).selected_color();
        let styled_color = self.styled_state.read(cx).selected_color();
        let background_color = self.background_state.read(cx).selected_color();
        let text_color = self.text_state.read(cx).selected_color();

        div()
            .size_full()
            .bg(theme.tokens.background)
            .overflow_hidden()
            .child(
                scrollable_vertical(
                    div()
                        .flex()
                        .flex_col()
                        .text_color(theme.tokens.foreground)
                        .p(px(32.0))
                        .gap(px(32.0))
                        .child(
                            VStack::new()
                                .gap(px(8.0))
                                .child(
                                    div()
                                        .text_size(px(24.0))
                                        .font_weight(FontWeight::BOLD)
                                        .child("ColorPicker Component Demo")
                                )
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child("Full-featured color selection with HSL/RGB/HEX modes, swatches, and customization")
                                )
                        )
                        // 1. Basic Color Picker
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("1. Basic Color Picker")
                                )
                                .child(
                                    VStack::new()
                                        .gap(px(12.0))
                                        .child(
                                            div()
                                                .text_size(px(13.0))
                                                .text_color(theme.tokens.muted_foreground)
                                                .child("Default color picker with all features:")
                                        )
                                        .child(
                                            ColorPicker::new("basic-picker", self.basic_state.clone())
                                                .on_change({
                                                    let state = self.basic_state.clone();
                                                    move |color, _window, cx| {
                                                        state.update(cx, |s, cx| {
                                                            s.set_color(color);
                                                            cx.notify();
                                                        });
                                                    }
                                                })
                                        )
                                        .child(
                                            div()
                                                .p(px(16.0))
                                                .mt(px(12.0))
                                                .bg(basic_color)
                                                .rounded(px(8.0))
                                                .text_size(px(14.0))
                                                .text_color(gpui::white())
                                                .child("This box uses the selected color as background")
                                        )
                                )
                        )
                        // 2. Color Picker with Alpha/Opacity
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("2. Color Picker with Alpha/Opacity")
                                )
                                .child(
                                    VStack::new()
                                        .gap(px(12.0))
                                        .child(
                                            div()
                                                .text_size(px(13.0))
                                                .text_color(theme.tokens.muted_foreground)
                                                .child("Control opacity with alpha slider:")
                                        )
                                        .child(
                                            ColorPicker::new("alpha-picker", self.alpha_state.clone())
                                                .show_alpha(true)
                                                .on_change({
                                                    let state = self.alpha_state.clone();
                                                    move |color, _window, cx| {
                                                        state.update(cx, |s, cx| {
                                                            s.set_color(color);
                                                            cx.notify();
                                                        });
                                                    }
                                                })
                                        )
                                        .child(
                                            div()
                                                .relative()
                                                .p(px(16.0))
                                                .mt(px(12.0))
                                                .rounded(px(8.0))
                                                .overflow_hidden()
                                                .child(
                                                    div()
                                                        .absolute()
                                                        .size_full()
                                                        .top_0()
                                                        .left_0()
                                                        .child(
                                                            div()
                                                                .size_full()
                                                                .bg(alpha_color)
                                                        )
                                                )
                                                .child(
                                                    div()
                                                        .relative()
                                                        .text_size(px(14.0))
                                                        .text_color(gpui::white())
                                                        .child(format!("Alpha: {:.0}%", alpha_color.a * 100.0))
                                                )
                                        )
                                )
                        )
                        // 3. Custom Swatches
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("3. Color Picker with Custom Swatches")
                                )
                                .child(
                                    VStack::new()
                                        .gap(px(12.0))
                                        .child(
                                            div()
                                                .text_size(px(13.0))
                                                .text_color(theme.tokens.muted_foreground)
                                                .child("Predefined brand colors:")
                                        )
                                        .child(
                                            ColorPicker::new("swatches-picker", self.custom_swatches_state.clone())
                                                .swatches(vec![
                                                    // Brand colors
                                                    hsla(0.0, 0.8, 0.5, 1.0),    // Red
                                                    hsla(30.0, 0.9, 0.5, 1.0),   // Orange
                                                    hsla(60.0, 0.9, 0.5, 1.0),   // Yellow
                                                    hsla(120.0, 0.6, 0.4, 1.0),  // Green
                                                    hsla(210.0, 0.8, 0.5, 1.0),  // Blue
                                                    hsla(270.0, 0.6, 0.5, 1.0),  // Purple
                                                    // Grays
                                                    hsla(0.0, 0.0, 0.2, 1.0),
                                                    hsla(0.0, 0.0, 0.5, 1.0),
                                                    hsla(0.0, 0.0, 0.8, 1.0),
                                                ])
                                                .on_change({
                                                    let state = self.custom_swatches_state.clone();
                                                    move |color, _window, cx| {
                                                        state.update(cx, |s, cx| {
                                                            s.set_color(color);
                                                            cx.notify();
                                                        });
                                                    }
                                                })
                                        )
                                )
                        )
                        // 4. Styled Color Picker
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("4. Styled Color Picker (Using Styled Trait)")
                                )
                                .child(
                                    VStack::new()
                                        .gap(px(12.0))
                                        .child(
                                            div()
                                                .text_size(px(13.0))
                                                .text_color(theme.tokens.muted_foreground)
                                                .child("Custom styled with .w_full(), .p(), .bg(), etc:")
                                        )
                                        .child(
                                            ColorPicker::new("styled-picker", self.styled_state.clone())
                                                .w_full()
                                                .p(px(12.0))
                                                .bg(theme.tokens.accent.opacity(0.1))
                                                .rounded(px(12.0))
                                                .border_2()
                                                .border_color(theme.tokens.primary)
                                                .on_change({
                                                    let state = self.styled_state.clone();
                                                    move |color, _window, cx| {
                                                        state.update(cx, |s, cx| {
                                                            s.set_color(color);
                                                            cx.notify();
                                                        });
                                                    }
                                                })
                                        )
                                )
                        )
                        // 5. Disabled State
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("5. Disabled Color Picker")
                                )
                                .child(
                                    VStack::new()
                                        .gap(px(12.0))
                                        .child(
                                            div()
                                                .text_size(px(13.0))
                                                .text_color(theme.tokens.muted_foreground)
                                                .child("Disabled state (non-interactive):")
                                        )
                                        .child(
                                            ColorPicker::new("disabled-picker", self.disabled_state.clone())
                                                .disabled(true)
                                        )
                                )
                        )
                        // 6. Multiple Color Pickers (Theme Editor Example)
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("6. Multiple Color Pickers (Theme Editor)")
                                )
                                .child(
                                    VStack::new()
                                        .gap(px(16.0))
                                        .child(
                                            div()
                                                .text_size(px(13.0))
                                                .text_color(theme.tokens.muted_foreground)
                                                .child("Coordinating multiple colors for a theme:")
                                        )
                                        .child(
                                            HStack::new()
                                                .gap(px(16.0))
                                                .items_center()
                                                .child(
                                                    div()
                                                        .w(px(120.0))
                                                        .text_size(px(13.0))
                                                        .child("Background:")
                                                )
                                                .child(
                                                    ColorPicker::new("bg-picker", self.background_state.clone())
                                                        .on_change({
                                                            let state = self.background_state.clone();
                                                            move |color, _window, cx| {
                                                                state.update(cx, |s, cx| {
                                                                    s.set_color(color);
                                                                    cx.notify();
                                                                });
                                                            }
                                                        })
                                                )
                                        )
                                        .child(
                                            HStack::new()
                                                .gap(px(16.0))
                                                .items_center()
                                                .child(
                                                    div()
                                                        .w(px(120.0))
                                                        .text_size(px(13.0))
                                                        .child("Text Color:")
                                                )
                                                .child(
                                                    ColorPicker::new("text-picker", self.text_state.clone())
                                                        .on_change({
                                                            let state = self.text_state.clone();
                                                            move |color, _window, cx| {
                                                                state.update(cx, |s, cx| {
                                                                    s.set_color(color);
                                                                    cx.notify();
                                                                });
                                                            }
                                                        })
                                                )
                                        )
                                        .child(
                                            div()
                                                .mt(px(16.0))
                                                .p(px(24.0))
                                                .bg(background_color)
                                                .rounded(px(8.0))
                                                .border_1()
                                                .border_color(theme.tokens.border)
                                                .child(
                                                    div()
                                                        .text_size(px(16.0))
                                                        .text_color(text_color)
                                                        .child("Preview: This text uses your selected theme colors!")
                                                )
                                        )
                                )
                        )
                        // Info Box
                        .child(
                            div()
                                .mt(px(16.0))
                                .p(px(16.0))
                                .bg(theme.tokens.accent)
                                .rounded(px(8.0))
                                .child(
                                    VStack::new()
                                        .gap(px(8.0))
                                        .child(
                                            div()
                                                .text_size(px(14.0))
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .text_color(theme.tokens.accent_foreground)
                                                .child("Features:")
                                        )
                                        .child(
                                            div()
                                                .text_size(px(12.0))
                                                .text_color(theme.tokens.accent_foreground)
                                                .flex()
                                                .flex_col()
                                                .gap(px(4.0))
                                                .child("• HSL/RGB/HEX color modes")
                                                .child("• Saturation/Lightness gradient selector")
                                                .child("• Hue slider with rainbow gradient")
                                                .child("• Alpha/opacity slider")
                                                .child("• Customizable color swatches")
                                                .child("• Recent colors history")
                                                .child("• Copy to clipboard")
                                                .child("• Full Styled trait support")
                                                .child("• Disabled state")
                                        )
                                )
                        )
                )
            )
    }
}
