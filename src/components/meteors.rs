//! Meteors effect - animated diagonal line streaks across a container.

use gpui::prelude::FluentBuilder as _;
use gpui::*;
use std::time::Duration;

pub struct MeteorState {
    version: usize,
}

impl MeteorState {
    pub fn new(cx: &mut Context<Self>) -> Self {
        cx.spawn(async move |this, cx| {
            loop {
                cx.background_executor().timer(Duration::from_millis(100)).await;
                let result = this.update(cx, |state, cx| {
                    state.version = state.version.wrapping_add(1);
                    cx.notify();
                });
                if result.is_err() {
                    break;
                }
            }
        })
        .detach();

        Self { version: 0 }
    }
}

fn meteor_hash(seed: u32) -> f32 {
    let mut h = seed;
    h ^= h >> 16;
    h = h.wrapping_mul(0x45d9f3b);
    h ^= h >> 16;
    h = h.wrapping_mul(0x45d9f3b);
    h ^= h >> 16;
    (h & 0xFFFF) as f32 / 65535.0
}

#[derive(IntoElement)]
pub struct Meteors {
    id: ElementId,
    state: Entity<MeteorState>,
    count: usize,
    speed: f32,
    angle: f32,
    color: Option<Hsla>,
    trail_length: Pixels,
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl Meteors {
    pub fn new(id: impl Into<ElementId>, state: Entity<MeteorState>) -> Self {
        Self {
            id: id.into(),
            state,
            count: 8,
            speed: 1.0,
            angle: 215.0,
            color: None,
            trail_length: px(120.0),
            children: Vec::new(),
            style: StyleRefinement::default(),
        }
    }

    pub fn count(mut self, count: usize) -> Self {
        self.count = count.clamp(1, 50);
        self
    }

    pub fn speed(mut self, speed: f32) -> Self {
        self.speed = speed.max(0.1);
        self
    }

    pub fn angle(mut self, angle: f32) -> Self {
        self.angle = angle;
        self
    }

    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }

    pub fn trail_length(mut self, length: Pixels) -> Self {
        self.trail_length = length;
        self
    }
}

impl Styled for Meteors {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Meteors {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Meteors {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let user_style = self.style;
        let version = self.state.read(cx).version;
        let count = self.count;
        let speed = self.speed;
        let color = self.color.unwrap_or(hsla(0.0, 0.0, 1.0, 0.6));
        let trail_length = self.trail_length;

        let angle_rad = self.angle.to_radians();
        let cos_a = angle_rad.cos();
        let sin_a = angle_rad.sin();

        let mut container = div()
            .id(self.id)
            .relative()
            .w_full()
            .h_full()
            .overflow_hidden();

        for idx in 0..count {
            let base_seed = (idx as u32).wrapping_mul(31337);
            let start_x_pct = meteor_hash(base_seed);
            let delay_pct = meteor_hash(base_seed.wrapping_add(1));
            let speed_var = 0.5 + meteor_hash(base_seed.wrapping_add(2));

            let base_dur = (3000.0 / speed / speed_var) as u64;
            let duration = Duration::from_millis(base_dur.max(400));

            let trail_f32 = trail_length / px(1.0);
            let meteor_color = color;

            let time_offset = version as f32 * 0.016 * speed * speed_var;
            let cycle_secs = duration.as_secs_f32();
            let phase = ((time_offset + delay_pct * cycle_secs) % cycle_secs) / cycle_secs;

            let travel = 1.5 + trail_f32 / 500.0;
            let pos = -0.3 + phase * travel;

            let meteor_x = start_x_pct + pos * cos_a;
            let meteor_y = -0.1 + pos * (-sin_a);

            if meteor_y > 1.3 || meteor_x < -0.5 || meteor_x > 1.5 {
                continue;
            }

            let brightness = if phase < 0.1 {
                phase / 0.1
            } else if phase > 0.8 {
                (1.0 - phase) / 0.2
            } else {
                1.0
            };

            let segments = 6;
            for seg in 0..segments {
                let seg_pct = seg as f32 / segments as f32;
                let seg_len = trail_f32 / segments as f32;
                let offset = seg_pct * trail_f32 / 500.0;

                let seg_x = meteor_x - offset * cos_a;
                let seg_y = meteor_y + offset * sin_a;
                let seg_alpha = meteor_color.a * brightness * (1.0 - seg_pct * 0.8);
                let seg_width = 2.0 - seg_pct * 1.2;

                let seg_color = Hsla {
                    a: seg_alpha.max(0.0),
                    ..meteor_color
                };

                container = container.child(
                    div()
                        .absolute()
                        .left(relative(seg_x))
                        .top(relative(seg_y))
                        .w(px(seg_len))
                        .h(px(seg_width.max(0.5)))
                        .rounded(px(seg_width))
                        .bg(seg_color),
                );
            }
        }

        container
            .child(div().relative().size_full().children(self.children))
            .map(|this| {
                let mut el = this;
                el.style().refine(&user_style);
                el
            })
    }
}
