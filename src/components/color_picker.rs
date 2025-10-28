//! ColorPicker component - Full-featured color selection with HSL/RGB/HEX modes.

use gpui::{prelude::FluentBuilder as _, *};
use std::rc::Rc;
use crate::theme::use_theme;
use crate::overlays::popover::{Popover, PopoverContent};
use crate::components::text::{Text, TextVariant};

const MAX_RECENT_COLORS: usize = 10;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ColorMode {
    HSL,
    RGB,
    HEX,
}

/// State for managing color picker interactions
pub struct ColorPickerState {
    selected_color: Hsla,
    mode: ColorMode,
    recent_colors: Vec<Hsla>,
}

impl ColorPickerState {
    pub fn new(initial_color: Hsla) -> Self {
        Self {
            selected_color: initial_color,
            mode: ColorMode::HSL,
            recent_colors: Vec::new(),
        }
    }

    pub fn set_hue(&mut self, hue: f32) {
        self.selected_color.h = hue.clamp(0.0, 360.0);
    }

    pub fn set_saturation(&mut self, saturation: f32) {
        self.selected_color.s = saturation.clamp(0.0, 1.0);
    }

    pub fn set_lightness(&mut self, lightness: f32) {
        self.selected_color.l = lightness.clamp(0.0, 1.0);
    }

    pub fn set_alpha(&mut self, alpha: f32) {
        self.selected_color.a = alpha.clamp(0.0, 1.0);
    }

    pub fn set_color(&mut self, color: Hsla) {
        self.selected_color = color;
    }

    pub fn add_to_recent(&mut self, color: Hsla) {
        self.recent_colors.retain(|&c| {
            !(c.h == color.h && c.s == color.s && c.l == color.l && c.a == color.a)
        });

        self.recent_colors.insert(0, color);

        if self.recent_colors.len() > MAX_RECENT_COLORS {
            self.recent_colors.truncate(MAX_RECENT_COLORS);
        }
    }

    pub fn selected_color(&self) -> Hsla {
        self.selected_color
    }

    pub fn recent_colors(&self) -> &[Hsla] {
        &self.recent_colors
    }

    pub fn mode(&self) -> ColorMode {
        self.mode
    }

    pub fn set_mode(&mut self, mode: ColorMode) {
        self.mode = mode;
    }
}

#[derive(IntoElement)]
pub struct ColorPicker {
    id: ElementId,
    state: Entity<ColorPickerState>,
    show_alpha: bool,
    swatches: Vec<Hsla>,
    on_change: Option<Rc<dyn Fn(Hsla, &mut Window, &mut App)>>,
    disabled: bool,
    style: StyleRefinement,
}

impl ColorPicker {
    /// Create a new color picker with default settings.
    pub fn new(id: impl Into<ElementId>, state: Entity<ColorPickerState>) -> Self {
        Self {
            id: id.into(),
            state,
            show_alpha: true,
            swatches: default_swatches(),
            on_change: None,
            disabled: false,
            style: StyleRefinement::default(),
        }
    }

    /// Enable or disable the alpha/opacity slider.
    pub fn show_alpha(mut self, show: bool) -> Self {
        self.show_alpha = show;
        self
    }

    /// Set custom color swatches.
    pub fn swatches(mut self, swatches: Vec<Hsla>) -> Self {
        self.swatches = swatches;
        self
    }

    /// Set the change callback.
    pub fn on_change(
        mut self,
        handler: impl Fn(Hsla, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }

    /// Enable or disable the color picker.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Convert HSLA color to HEX string
    fn hsla_to_hex(color: Hsla) -> String {
        let h = color.h;
        let s = color.s;
        let l = color.l;

        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = l - c / 2.0;

        let (r, g, b) = if h < 60.0 {
            (c, x, 0.0)
        } else if h < 120.0 {
            (x, c, 0.0)
        } else if h < 180.0 {
            (0.0, c, x)
        } else if h < 240.0 {
            (0.0, x, c)
        } else if h < 300.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };

        let r_byte = ((r + m) * 255.0) as u8;
        let g_byte = ((g + m) * 255.0) as u8;
        let b_byte = ((b + m) * 255.0) as u8;

        format!("#{:02X}{:02X}{:02X}", r_byte, g_byte, b_byte)
    }

    /// Convert HSLA color to RGB values (0-255)
    fn hsla_to_rgb(color: Hsla) -> (u8, u8, u8) {
        let h = color.h;
        let s = color.s;
        let l = color.l;

        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = l - c / 2.0;

        let (r, g, b) = if h < 60.0 {
            (c, x, 0.0)
        } else if h < 120.0 {
            (x, c, 0.0)
        } else if h < 180.0 {
            (0.0, c, x)
        } else if h < 240.0 {
            (0.0, x, c)
        } else if h < 300.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };

        (
            ((r + m) * 255.0) as u8,
            ((g + m) * 255.0) as u8,
            ((b + m) * 255.0) as u8,
        )
    }
}

impl Styled for ColorPicker {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for ColorPicker {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let state = self.state.clone();
        let color = state.read(cx).selected_color();
        let _show_alpha = self.show_alpha;
        let swatches = self.swatches.clone();
        let on_change = self.on_change.clone();
        let disabled = self.disabled;
        let user_style = self.style;
        let picker_id = self.id.clone();

        let preview_button = div()
            .flex()
            .items_center()
            .gap_2()
            .h(px(40.0))
            .px(px(12.0))
            .bg(theme.tokens.background)
            .border_1()
            .border_color(theme.tokens.border)
            .rounded(theme.tokens.radius_md)
            .when(!disabled, |this| {
                this.cursor(CursorStyle::PointingHand)
                    .hover(|style| style.bg(theme.tokens.accent.opacity(0.1)))
            })
            .when(disabled, |this| this.opacity(0.5))
            .child(
                div()
                    .size(px(24.0))
                    .rounded(px(4.0))
                    .bg(color)
                    .border_1()
                    .border_color(theme.tokens.border)
            )
            .child(
                Text::new(Self::hsla_to_hex(color))
                .variant(TextVariant::Custom)
                .size(px(14.0))
                .color(theme.tokens.foreground)
            )
            .map(|mut this| {
                this.style().refine(&user_style);
                this
            });

        if disabled {
            return preview_button.into_any_element();
        }

        Popover::new(picker_id)
            .trigger(preview_button)
            .content(move |window, cx| {
                let swatches_for_content = swatches.clone();
                let on_change_for_content = on_change.clone();
                let state_for_content = state.clone();

                cx.new(|cx| {
                    PopoverContent::new(window, cx, move |_window, cx| {
                        let _theme = use_theme();

                        // Read state fresh on every render so mode changes work
                        let current_color = state_for_content.read(cx).selected_color();
                        let current_mode = state_for_content.read(cx).mode();
                        let recent_vec = state_for_content.read(cx).recent_colors().to_vec();

                        let swatches_clone = swatches_for_content.clone();
                        let on_change_clone = on_change_for_content.clone();

                        div()
                            .flex()
                            .flex_col()
                            .gap_3()
                            .w(px(280.0))
                            .child(
                                render_color_preview(current_color)
                            )
                            .child(
                                render_mode_selector(current_mode, state_for_content.clone())
                            )
                            .child(
                                render_color_value(current_color, current_mode)
                            )
                            .when(!swatches_clone.is_empty(), |this| {
                                this.child(
                                    render_swatches(swatches_clone, state_for_content.clone(), on_change_clone.clone())
                                )
                            })
                            .when(!recent_vec.is_empty(), |this| {
                                this.child(
                                    render_recent_colors(recent_vec, state_for_content.clone(), on_change_clone.clone())
                                )
                            })
                            .child(
                                render_actions(current_color, state_for_content.clone(), on_change_clone.clone())
                            )
                            .into_any_element()
                    })
                })
            })
            .into_any_element()
    }
}

fn render_color_preview(color: Hsla) -> impl IntoElement {
    let theme = use_theme();

    div()
        .flex()
        .flex_col()
        .gap_2()
        .child(
            Text::new("Selected Color")
                .variant(TextVariant::Custom)
                .size(px(12.0))
                .color(theme.tokens.muted_foreground)
        )
        .child(
            div()
                .w_full()
                .h(px(80.0))
                .rounded(theme.tokens.radius_md)
                .bg(color)
                .border_1()
                .border_color(theme.tokens.border)
        )
}

fn render_mode_selector(
    current_mode: ColorMode,
    state: Entity<ColorPickerState>,
) -> impl IntoElement {
    div()
        .flex()
        .gap_1()
        .child(render_mode_button("HSL", ColorMode::HSL, current_mode, state.clone()))
        .child(render_mode_button("RGB", ColorMode::RGB, current_mode, state.clone()))
        .child(render_mode_button("HEX", ColorMode::HEX, current_mode, state))
}

fn render_mode_button(
    label: &'static str,
    mode: ColorMode,
    current_mode: ColorMode,
    state: Entity<ColorPickerState>,
) -> impl IntoElement {
    let theme = use_theme();
    let is_active = mode == current_mode;

    div()
        .flex_1()
        .py(px(6.0))
        .px(px(12.0))
        .rounded(theme.tokens.radius_sm)
        .text_size(px(12.0))
        .text_align(TextAlign::Center)
        .when(is_active, |this| {
            this.bg(theme.tokens.primary)
                .text_color(theme.tokens.primary_foreground)
        })
        .when(!is_active, |this| {
            this.bg(theme.tokens.muted.opacity(0.3))
                .text_color(theme.tokens.foreground)
                .cursor(CursorStyle::PointingHand)
                .hover(|style| style.bg(theme.tokens.muted.opacity(0.5)))
                .on_mouse_down(MouseButton::Left, move |_, _, cx| {
                    state.update(cx, |state, cx| {
                        state.set_mode(mode);
                        cx.notify();
                    });
                })
        })
        .child(label)
}

fn render_color_value(color: Hsla, mode: ColorMode) -> impl IntoElement {
    let theme = use_theme();

    let value = match mode {
        ColorMode::HSL => {
            format!(
                "hsl({:.0}, {:.0}%, {:.0}%)",
                color.h,
                color.s * 100.0,
                color.l * 100.0
            )
        }
        ColorMode::RGB => {
            let (r, g, b) = ColorPicker::hsla_to_rgb(color);
            format!(
                "rgb({}, {}, {})",
                r, g, b
            )
        }
        ColorMode::HEX => {
            ColorPicker::hsla_to_hex(color)
        }
    };

    div()
        .flex()
        .items_center()
        .justify_between()
        .p(px(8.0))
        .bg(theme.tokens.muted.opacity(0.2))
        .rounded(theme.tokens.radius_sm)
        .child(
            Text::new(value)
                .variant(TextVariant::Custom)
                .size(px(13.0))
                .color(theme.tokens.foreground)
        )
}

fn render_swatches(
    swatches: Vec<Hsla>,
    state: Entity<ColorPickerState>,
    on_change: Option<Rc<dyn Fn(Hsla, &mut Window, &mut App)>>,
) -> impl IntoElement {
    let theme = use_theme();

    div()
        .flex()
        .flex_col()
        .gap_2()
        .child(
            Text::new("Swatches")
                .variant(TextVariant::Custom)
                .size(px(12.0))
                .color(theme.tokens.muted_foreground)
        )
        .child(
            div()
                .flex()
                .flex_wrap()
                .gap_2()
                .children(swatches.into_iter().map(move |swatch| {
                    render_color_swatch(swatch, state.clone(), on_change.clone())
                }))
        )
}

fn render_recent_colors(
    recent: Vec<Hsla>,
    state: Entity<ColorPickerState>,
    on_change: Option<Rc<dyn Fn(Hsla, &mut Window, &mut App)>>,
) -> impl IntoElement {
    let theme = use_theme();

    div()
        .flex()
        .flex_col()
        .gap_2()
        .child(
            Text::new("Recent Colors")
                .variant(TextVariant::Custom)
                .size(px(12.0))
                .color(theme.tokens.muted_foreground)
        )
        .child(
            div()
                .flex()
                .flex_wrap()
                .gap_2()
                .children(recent.into_iter().map(move |color| {
                    render_color_swatch(color, state.clone(), on_change.clone())
                }))
        )
}

fn render_color_swatch(
    color: Hsla,
    state: Entity<ColorPickerState>,
    on_change: Option<Rc<dyn Fn(Hsla, &mut Window, &mut App)>>,
) -> impl IntoElement {
    let theme = use_theme();

    div()
        .size(px(28.0))
        .rounded(theme.tokens.radius_sm)
        .bg(color)
        .border_1()
        .border_color(theme.tokens.border)
        .cursor(CursorStyle::PointingHand)
        .hover(|style| style.shadow_sm())
        .on_mouse_down(MouseButton::Left, move |_, window, cx| {
            state.update(cx, |state, _| {
                state.set_color(color);
            });

            if let Some(handler) = on_change.as_ref() {
                handler(color, window, cx);
            }
        })
}

fn render_actions(
    color: Hsla,
    state: Entity<ColorPickerState>,
    on_change: Option<Rc<dyn Fn(Hsla, &mut Window, &mut App)>>,
) -> impl IntoElement {
    let theme = use_theme();

    div()
        .flex()
        .gap_2()
        .child(
            div()
                .flex_1()
                .py(px(8.0))
                .px(px(12.0))
                .bg(theme.tokens.secondary)
                .text_color(theme.tokens.secondary_foreground)
                .rounded(theme.tokens.radius_sm)
                .text_size(px(13.0))
                .text_align(TextAlign::Center)
                .cursor(CursorStyle::PointingHand)
                .hover(|style| style.bg(theme.tokens.secondary.opacity(0.8)))
                .child("Copy")
                .on_mouse_down(MouseButton::Left, move |_, _window, cx| {
                    let hex = ColorPicker::hsla_to_hex(color);
                    cx.write_to_clipboard(ClipboardItem::new_string(hex));
                    cx.stop_propagation();
                })
        )
        .child(
            div()
                .flex_1()
                .py(px(8.0))
                .px(px(12.0))
                .bg(theme.tokens.primary)
                .text_color(theme.tokens.primary_foreground)
                .rounded(theme.tokens.radius_sm)
                .text_size(px(13.0))
                .text_align(TextAlign::Center)
                .cursor(CursorStyle::PointingHand)
                .hover(|style| style.bg(theme.tokens.primary.opacity(0.9)))
                .child("Apply")
                .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                    state.update(cx, |state, _| {
                        state.add_to_recent(color);
                    });

                    if let Some(handler) = on_change.as_ref() {
                        handler(color, window, cx);
                    }
                })
        )
}

fn default_swatches() -> Vec<Hsla> {
    vec![
        hsla(0.0, 0.7, 0.5, 1.0),
        hsla(0.0, 0.8, 0.4, 1.0),
        hsla(0.0, 0.9, 0.3, 1.0),
        hsla(30.0, 0.8, 0.5, 1.0),
        hsla(40.0, 0.9, 0.5, 1.0),
        hsla(60.0, 0.8, 0.5, 1.0),
        hsla(50.0, 0.9, 0.6, 1.0),
        hsla(120.0, 0.6, 0.4, 1.0),
        hsla(140.0, 0.7, 0.4, 1.0),
        hsla(160.0, 0.6, 0.4, 1.0),
        hsla(180.0, 0.6, 0.5, 1.0),
        hsla(190.0, 0.7, 0.5, 1.0),
        hsla(210.0, 0.7, 0.5, 1.0),
        hsla(220.0, 0.8, 0.5, 1.0),
        hsla(240.0, 0.7, 0.5, 1.0),
        hsla(270.0, 0.6, 0.5, 1.0),
        hsla(290.0, 0.6, 0.5, 1.0),
        hsla(310.0, 0.7, 0.5, 1.0),
        hsla(330.0, 0.7, 0.5, 1.0),
        hsla(0.0, 0.0, 0.2, 1.0),
        hsla(0.0, 0.0, 0.4, 1.0),
        hsla(0.0, 0.0, 0.6, 1.0),
        hsla(0.0, 0.0, 0.8, 1.0),
        hsla(0.0, 0.0, 0.95, 1.0),
    ]
}
