//! Button whose position subtly shifts toward cursor when nearby.

use gpui::{prelude::FluentBuilder as _, *};

pub struct MagneticButtonState {
    mouse_offset: Point<f32>,
    is_hovering: bool,
}

impl MagneticButtonState {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            mouse_offset: Point::default(),
            is_hovering: false,
        }
    }
}

impl Render for MagneticButtonState {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Empty
    }
}

#[derive(IntoElement)]
pub struct MagneticButton {
    id: ElementId,
    state: Entity<MagneticButtonState>,
    strength: f32,
    #[allow(dead_code)]
    range: Pixels,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl MagneticButton {
    pub fn new(id: impl Into<ElementId>, state: Entity<MagneticButtonState>) -> Self {
        Self {
            id: id.into(),
            state,
            strength: 0.3,
            range: px(100.0),
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn strength(mut self, strength: f32) -> Self {
        self.strength = strength.clamp(0.0, 1.0);
        self
    }

    pub fn range(mut self, range: Pixels) -> Self {
        self.range = range;
        self
    }
}

impl Styled for MagneticButton {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for MagneticButton {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for MagneticButton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let user_style = self.style;
        let state = self.state.read(cx);
        let strength = self.strength;

        let (final_x, final_y) = if state.is_hovering {
            (
                state.mouse_offset.x * strength * 20.0,
                state.mouse_offset.y * strength * 20.0,
            )
        } else {
            (0.0, 0.0)
        };

        let state_move = self.state.clone();
        let state_hover = self.state.clone();

        div()
            .id(self.id)
            .relative()
            .cursor_pointer()
            .child(
                div()
                    .ml(px(final_x))
                    .mt(px(final_y))
                    .children(self.children),
            )
            .on_mouse_move(move |event: &MouseMoveEvent, _window, cx| {
                state_move.update(cx, |s, cx| {
                    s.mouse_offset.x = event.position.x / px(1.0);
                    s.mouse_offset.y = event.position.y / px(1.0);
                    cx.notify();
                });
            })
            .on_hover(move |hovered: &bool, _window, cx| {
                state_hover.update(cx, |s, cx| {
                    s.is_hovering = *hovered;
                    if !*hovered {
                        s.mouse_offset = Point::default();
                    }
                    cx.notify();
                });
            })
            .map(|this: Stateful<Div>| {
                let mut el = this;
                el.style().refine(&user_style);
                el
            })
    }
}
