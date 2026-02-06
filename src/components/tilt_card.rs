//! Card with 3D tilt illusion based on cursor position using asymmetric shadows.

use crate::theme::use_theme;
use gpui::{prelude::FluentBuilder as _, *};
use smallvec::smallvec;

pub struct TiltCardState {
    mouse_position: Option<Point<Pixels>>,
    #[allow(dead_code)]
    bounds: Option<Bounds<Pixels>>,
}

impl TiltCardState {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            mouse_position: None,
            bounds: None,
        }
    }
}

impl Render for TiltCardState {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Empty
    }
}

#[derive(IntoElement)]
pub struct TiltCard {
    id: ElementId,
    state: Entity<TiltCardState>,
    intensity: f32,
    shadow_color: Option<Hsla>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl TiltCard {
    pub fn new(id: impl Into<ElementId>, state: Entity<TiltCardState>) -> Self {
        Self {
            id: id.into(),
            state,
            intensity: 1.0,
            shadow_color: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn intensity(mut self, intensity: f32) -> Self {
        self.intensity = intensity.clamp(0.0, 3.0);
        self
    }

    pub fn shadow_color(mut self, color: Hsla) -> Self {
        self.shadow_color = Some(color);
        self
    }
}

impl Styled for TiltCard {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for TiltCard {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for TiltCard {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let user_style = self.style;
        let state = self.state.read(cx);

        let shadow_base = self
            .shadow_color
            .unwrap_or_else(|| hsla(0.0, 0.0, 0.0, 0.2));
        let intensity = self.intensity;

        let (shadow_offset_x, shadow_offset_y, highlight_opacity) =
            if let Some(mouse) = state.mouse_position {
                let norm_x: f32 = mouse.x / px(1.0);
                let norm_y: f32 = mouse.y / px(1.0);

                let offset_x = -norm_x * 8.0 * intensity;
                let offset_y = -norm_y * 8.0 * intensity;
                let opacity: f32 = (1.0 - (norm_x.abs() + norm_y.abs()) * 0.25).max(0.0);

                (offset_x, offset_y, opacity)
            } else {
                (0.0_f32, 4.0_f32, 1.0_f32)
            };

        let shadow = BoxShadow {
            color: hsla(
                shadow_base.h,
                shadow_base.s,
                shadow_base.l,
                shadow_base.a * highlight_opacity,
            ),
            offset: point(px(shadow_offset_x), px(shadow_offset_y)),
            blur_radius: px(16.0 * intensity),
            spread_radius: px(0.0),
            inset: false,
        };

        let state_move = self.state.clone();
        let state_hover = self.state.clone();

        div()
            .id(self.id)
            .overflow_hidden()
            .rounded(px(8.0))
            .bg(theme.tokens.card)
            .border_1()
            .border_color(theme.tokens.border)
            .shadow(smallvec![shadow])
            .children(self.children)
            .on_mouse_move(move |event: &MouseMoveEvent, _window, cx| {
                state_move.update(cx, |s, cx| {
                    s.mouse_position = Some(event.position);
                    cx.notify();
                });
            })
            .on_hover(move |hovered: &bool, _window, cx| {
                if !*hovered {
                    state_hover.update(cx, |s, cx| {
                        s.mouse_position = None;
                        cx.notify();
                    });
                }
            })
            .map(|this: Stateful<Div>| {
                let mut el = this;
                el.style().refine(&user_style);
                el
            })
    }
}
