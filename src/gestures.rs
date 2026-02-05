use gpui::*;
use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwipeDirection {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone, Debug)]
pub struct SwipeGesture {
    pub direction: SwipeDirection,
    pub velocity: f32,
    pub distance: f32,
}

#[derive(Clone, Debug)]
pub struct LongPressGesture {
    pub position: Point<Pixels>,
    pub duration: Duration,
}

#[derive(Clone, Debug)]
pub struct TapGesture {
    pub position: Point<Pixels>,
    pub count: u32,
}

#[derive(Clone, Debug)]
pub struct PanGesture {
    pub delta: Point<Pixels>,
    pub velocity: Point<Pixels>,
    pub total_distance: Point<Pixels>,
}

#[derive(Clone, Debug)]
pub enum GestureEvent {
    Swipe(SwipeGesture),
    LongPress(LongPressGesture),
    Tap(TapGesture),
    PanStart(Point<Pixels>),
    PanUpdate(PanGesture),
    PanEnd(PanGesture),
}

const SWIPE_MIN_DISTANCE: f32 = 50.0;
const SWIPE_MIN_VELOCITY: f32 = 200.0;
const LONG_PRESS_DURATION: Duration = Duration::from_millis(500);
const DOUBLE_TAP_INTERVAL: Duration = Duration::from_millis(300);
const PAN_THRESHOLD: f32 = 5.0;

#[derive(Clone, Debug)]
pub struct GestureDetector {
    press_start: Option<(Point<Pixels>, Instant)>,
    last_position: Option<Point<Pixels>>,
    velocity: Point<Pixels>,
    total_delta: Point<Pixels>,
    is_panning: bool,
    tap_count: u32,
    last_tap_time: Option<Instant>,
    last_tap_position: Option<Point<Pixels>>,
    long_press_triggered: bool,
    long_press_duration: Duration,
    swipe_min_distance: f32,
}

impl Default for GestureDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl GestureDetector {
    pub fn new() -> Self {
        Self {
            press_start: None,
            last_position: None,
            velocity: point(px(0.0), px(0.0)),
            total_delta: point(px(0.0), px(0.0)),
            is_panning: false,
            tap_count: 0,
            last_tap_time: None,
            last_tap_position: None,
            long_press_triggered: false,
            long_press_duration: LONG_PRESS_DURATION,
            swipe_min_distance: SWIPE_MIN_DISTANCE,
        }
    }

    pub fn with_long_press_duration(mut self, duration: Duration) -> Self {
        self.long_press_duration = duration;
        self
    }

    pub fn with_swipe_distance(mut self, distance: f32) -> Self {
        self.swipe_min_distance = distance;
        self
    }

    pub fn on_mouse_down(&mut self, position: Point<Pixels>) -> Vec<GestureEvent> {
        self.press_start = Some((position, Instant::now()));
        self.last_position = Some(position);
        self.velocity = point(px(0.0), px(0.0));
        self.total_delta = point(px(0.0), px(0.0));
        self.is_panning = false;
        self.long_press_triggered = false;
        Vec::new()
    }

    pub fn on_mouse_move(&mut self, position: Point<Pixels>) -> Vec<GestureEvent> {
        let mut events = Vec::new();

        let Some(last) = self.last_position else {
            return events;
        };
        let Some((start, _)) = self.press_start else {
            return events;
        };

        let delta = point(position.x - last.x, position.y - last.y);
        self.total_delta = point(position.x - start.x, position.y - start.y);

        self.velocity = point(delta.x * 60.0, delta.y * 60.0);

        let total_distance =
            (f32::from(self.total_delta.x).powi(2) + f32::from(self.total_delta.y).powi(2)).sqrt();

        if !self.is_panning && total_distance > PAN_THRESHOLD {
            self.is_panning = true;
            events.push(GestureEvent::PanStart(start));
        }

        if self.is_panning {
            events.push(GestureEvent::PanUpdate(PanGesture {
                delta,
                velocity: self.velocity,
                total_distance: self.total_delta,
            }));
        }

        self.last_position = Some(position);
        events
    }

    pub fn on_mouse_up(&mut self, position: Point<Pixels>) -> Vec<GestureEvent> {
        let mut events = Vec::new();

        let Some((start, start_time)) = self.press_start.take() else {
            return events;
        };

        let delta = point(position.x - start.x, position.y - start.y);
        let distance = (f32::from(delta.x).powi(2) + f32::from(delta.y).powi(2)).sqrt();

        if self.is_panning {
            events.push(GestureEvent::PanEnd(PanGesture {
                delta: point(
                    position.x - self.last_position.unwrap_or(start).x,
                    position.y - self.last_position.unwrap_or(start).y,
                ),
                velocity: self.velocity,
                total_distance: delta,
            }));

            let vel_x = f32::from(self.velocity.x).abs();
            let vel_y = f32::from(self.velocity.y).abs();
            let max_vel = vel_x.max(vel_y);

            if distance >= self.swipe_min_distance && max_vel >= SWIPE_MIN_VELOCITY {
                let dx = f32::from(delta.x);
                let dy = f32::from(delta.y);
                let direction = if dx.abs() > dy.abs() {
                    if dx > 0.0 {
                        SwipeDirection::Right
                    } else {
                        SwipeDirection::Left
                    }
                } else if dy > 0.0 {
                    SwipeDirection::Down
                } else {
                    SwipeDirection::Up
                };

                events.push(GestureEvent::Swipe(SwipeGesture {
                    direction,
                    velocity: max_vel,
                    distance,
                }));
            }
        } else {
            let elapsed = start_time.elapsed();

            if elapsed >= self.long_press_duration && !self.long_press_triggered {
                events.push(GestureEvent::LongPress(LongPressGesture {
                    position,
                    duration: elapsed,
                }));
            } else {
                let is_double = self
                    .last_tap_time
                    .map(|t| t.elapsed() < DOUBLE_TAP_INTERVAL)
                    .unwrap_or(false);

                if is_double {
                    self.tap_count += 1;
                } else {
                    self.tap_count = 1;
                }

                self.last_tap_time = Some(Instant::now());
                self.last_tap_position = Some(position);

                events.push(GestureEvent::Tap(TapGesture {
                    position,
                    count: self.tap_count,
                }));
            }
        }

        self.last_position = None;
        self.is_panning = false;
        events
    }

    pub fn check_long_press(&mut self) -> Option<GestureEvent> {
        if self.long_press_triggered || self.is_panning {
            return None;
        }

        let (position, start_time) = self.press_start?;

        if start_time.elapsed() >= self.long_press_duration {
            self.long_press_triggered = true;
            Some(GestureEvent::LongPress(LongPressGesture {
                position,
                duration: start_time.elapsed(),
            }))
        } else {
            None
        }
    }

    pub fn is_pressed(&self) -> bool {
        self.press_start.is_some()
    }

    pub fn is_panning(&self) -> bool {
        self.is_panning
    }

    pub fn reset(&mut self) {
        self.press_start = None;
        self.last_position = None;
        self.is_panning = false;
        self.long_press_triggered = false;
    }
}
