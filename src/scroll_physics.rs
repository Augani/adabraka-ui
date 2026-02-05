#[derive(Clone, Debug)]
pub struct ScrollPhysics {
    velocity: f32,
    position: f32,
    min_bound: f32,
    max_bound: f32,
    deceleration: f32,
    overscroll_resistance: f32,
    momentum_enabled: bool,
    overscroll_enabled: bool,
}

impl Default for ScrollPhysics {
    fn default() -> Self {
        Self::new()
    }
}

impl ScrollPhysics {
    pub fn new() -> Self {
        Self {
            velocity: 0.0,
            position: 0.0,
            min_bound: 0.0,
            max_bound: f32::MAX,
            deceleration: 0.95,
            overscroll_resistance: 0.3,
            momentum_enabled: true,
            overscroll_enabled: true,
        }
    }

    pub fn with_bounds(mut self, min: f32, max: f32) -> Self {
        self.min_bound = min;
        self.max_bound = max;
        self
    }

    pub fn with_deceleration(mut self, deceleration: f32) -> Self {
        self.deceleration = deceleration.clamp(0.8, 0.99);
        self
    }

    pub fn with_overscroll_resistance(mut self, resistance: f32) -> Self {
        self.overscroll_resistance = resistance.clamp(0.0, 1.0);
        self
    }

    pub fn momentum(mut self, enabled: bool) -> Self {
        self.momentum_enabled = enabled;
        self
    }

    pub fn overscroll(mut self, enabled: bool) -> Self {
        self.overscroll_enabled = enabled;
        self
    }

    pub fn set_bounds(&mut self, min: f32, max: f32) {
        self.min_bound = min;
        self.max_bound = max;
    }

    pub fn apply_delta(&mut self, delta: f32) {
        if self.momentum_enabled {
            self.velocity = delta * 0.8 + self.velocity * 0.2;
        } else {
            self.velocity = 0.0;
        }
        self.position += delta;

        if !self.overscroll_enabled {
            self.position = self.position.clamp(self.min_bound, self.max_bound);
        }
    }

    pub fn tick(&mut self, dt: f32) -> bool {
        if !self.momentum_enabled && !self.is_overscrolled() {
            return false;
        }

        self.velocity *= self.deceleration;
        self.position += self.velocity * dt * 60.0;

        if self.overscroll_enabled {
            if self.position < self.min_bound {
                let overshoot = self.min_bound - self.position;
                self.position += overshoot * self.overscroll_resistance;
                self.velocity *= 0.5;
            }
            if self.position > self.max_bound {
                let overshoot = self.position - self.max_bound;
                self.position -= overshoot * self.overscroll_resistance;
                self.velocity *= 0.5;
            }
        } else {
            self.position = self.position.clamp(self.min_bound, self.max_bound);
            if self.position <= self.min_bound || self.position >= self.max_bound {
                self.velocity = 0.0;
            }
        }

        self.velocity.abs() > 0.5 || self.is_overscrolled()
    }

    pub fn position(&self) -> f32 {
        self.position
    }

    pub fn velocity(&self) -> f32 {
        self.velocity
    }

    pub fn is_moving(&self) -> bool {
        self.velocity.abs() > 0.5
    }

    pub fn is_overscrolled(&self) -> bool {
        self.position < self.min_bound || self.position > self.max_bound
    }

    pub fn stop(&mut self) {
        self.velocity = 0.0;
    }

    pub fn reset(&mut self) {
        self.velocity = 0.0;
        self.position = self.min_bound;
    }

    pub fn set_position(&mut self, position: f32) {
        self.position = position;
    }

    pub fn scroll_to(&mut self, position: f32) {
        self.position = position.clamp(self.min_bound, self.max_bound);
        self.velocity = 0.0;
    }

    pub fn fling(&mut self, velocity: f32) {
        self.velocity = velocity;
    }
}
