use std::time::Duration;

#[derive(Clone, Debug)]
pub struct Spring {
    pub position: f32,
    pub velocity: f32,
    pub target: f32,
    stiffness: f32,
    damping: f32,
    mass: f32,
    rest_threshold: f32,
}

impl Default for Spring {
    fn default() -> Self {
        Self::gentle()
    }
}

impl Spring {
    pub fn new(stiffness: f32, damping: f32, mass: f32) -> Self {
        Self {
            position: 0.0,
            velocity: 0.0,
            target: 0.0,
            stiffness: stiffness.max(0.1),
            damping: damping.max(0.0),
            mass: mass.max(0.01),
            rest_threshold: 0.001,
        }
    }

    pub fn gentle() -> Self {
        Self::new(120.0, 14.0, 1.0)
    }

    pub fn wobbly() -> Self {
        Self::new(180.0, 12.0, 1.0)
    }

    pub fn stiff() -> Self {
        Self::new(210.0, 20.0, 1.0)
    }

    pub fn slow() -> Self {
        Self::new(280.0, 60.0, 1.0)
    }

    pub fn snappy() -> Self {
        Self::new(400.0, 30.0, 1.0)
    }

    pub fn with_position(mut self, position: f32) -> Self {
        self.position = position;
        self
    }

    pub fn with_target(mut self, target: f32) -> Self {
        self.target = target;
        self
    }

    pub fn with_velocity(mut self, velocity: f32) -> Self {
        self.velocity = velocity;
        self
    }

    pub fn with_rest_threshold(mut self, threshold: f32) -> Self {
        self.rest_threshold = threshold.max(0.0001);
        self
    }

    pub fn set_target(&mut self, target: f32) {
        self.target = target;
    }

    pub fn set_position(&mut self, position: f32) {
        self.position = position;
    }

    pub fn impulse(&mut self, velocity: f32) {
        self.velocity += velocity;
    }

    pub fn tick(&mut self, dt: f32) -> bool {
        let dt = dt.min(0.064);

        let displacement = self.position - self.target;
        let spring_force = -self.stiffness * displacement;
        let damping_force = -self.damping * self.velocity;
        let acceleration = (spring_force + damping_force) / self.mass;

        self.velocity += acceleration * dt;
        self.position += self.velocity * dt;

        let is_moving = self.velocity.abs() > self.rest_threshold
            || (self.position - self.target).abs() > self.rest_threshold;

        if !is_moving {
            self.position = self.target;
            self.velocity = 0.0;
        }

        is_moving
    }

    pub fn tick_duration(&mut self, duration: Duration) -> bool {
        self.tick(duration.as_secs_f32())
    }

    pub fn is_at_rest(&self) -> bool {
        self.velocity.abs() <= self.rest_threshold
            && (self.position - self.target).abs() <= self.rest_threshold
    }

    pub fn progress(&self) -> f32 {
        if (self.target - self.position).abs() < self.rest_threshold {
            return 1.0;
        }
        let start = 0.0_f32;
        let total = self.target - start;
        if total.abs() < f32::EPSILON {
            return 1.0;
        }
        ((self.position - start) / total).clamp(0.0, 1.5)
    }

    pub fn reset(&mut self) {
        self.position = 0.0;
        self.velocity = 0.0;
    }

    pub fn snap_to_target(&mut self) {
        self.position = self.target;
        self.velocity = 0.0;
    }
}
