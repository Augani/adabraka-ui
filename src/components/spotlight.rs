use gpui::{prelude::FluentBuilder as _, *};

pub struct SpotlightState {
    mouse_pos: Option<Point<Pixels>>,
}

impl SpotlightState {
    pub fn new() -> Self {
        Self { mouse_pos: None }
    }
}

#[derive(IntoElement)]
pub struct Spotlight {
    id: ElementId,
    state: Entity<SpotlightState>,
    color: Option<Hsla>,
    spot_size: Pixels,
    intensity: f32,
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl Spotlight {
    pub fn new(id: impl Into<ElementId>, state: Entity<SpotlightState>) -> Self {
        Self {
            id: id.into(),
            state,
            color: None,
            spot_size: px(300.0),
            intensity: 0.15,
            children: Vec::new(),
            style: StyleRefinement::default(),
        }
    }

    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }

    pub fn size(mut self, size: Pixels) -> Self {
        self.spot_size = size;
        self
    }

    pub fn intensity(mut self, intensity: f32) -> Self {
        self.intensity = intensity.clamp(0.0, 1.0);
        self
    }
}

impl Styled for Spotlight {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Spotlight {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Spotlight {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let spot_color = self.color.unwrap_or(hsla(0.6, 0.8, 0.7, 1.0));
        let spot_size = self.spot_size;
        let intensity = self.intensity;
        let mouse_pos = self.state.read(cx).mouse_pos;
        let half_size = px(f32::from(spot_size) / 2.0);
        let user_style = self.style;

        let state_for_move = self.state.clone();

        let mut container = div()
            .id(self.id)
            .relative()
            .overflow_hidden()
            .on_mouse_move(move |event: &MouseMoveEvent, _, cx| {
                state_for_move.update(cx, |s, cx| {
                    s.mouse_pos = Some(event.position);
                    cx.notify();
                });
            })
            .map(|mut el| {
                el.style().refine(&user_style);
                el
            });

        for child in self.children {
            container = container.child(child);
        }

        if let Some(pos) = mouse_pos {
            let glow_color = Hsla {
                a: intensity,
                ..spot_color
            };

            let glow = div()
                .absolute()
                .left(pos.x - half_size)
                .top(pos.y - half_size)
                .w(spot_size)
                .h(spot_size)
                .rounded(spot_size)
                .bg(glow_color);

            container = container.child(glow);
        }

        container
    }
}
