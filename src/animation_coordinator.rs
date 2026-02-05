use std::collections::HashMap;
use std::time::{Duration, Instant};

type CompletionCallback = Box<dyn FnOnce() + 'static>;

struct AnimationEntry {
    start: Instant,
    duration: Duration,
    on_complete: Option<CompletionCallback>,
    completed: bool,
}

pub struct AnimationCoordinator {
    animations: HashMap<String, AnimationEntry>,
}

impl Default for AnimationCoordinator {
    fn default() -> Self {
        Self::new()
    }
}

impl AnimationCoordinator {
    pub fn new() -> Self {
        Self {
            animations: HashMap::new(),
        }
    }

    pub fn start(&mut self, id: impl Into<String>, duration: Duration) {
        let id = id.into();
        self.animations.insert(
            id,
            AnimationEntry {
                start: Instant::now(),
                duration,
                on_complete: None,
                completed: false,
            },
        );
    }

    pub fn start_with_callback(
        &mut self,
        id: impl Into<String>,
        duration: Duration,
        on_complete: impl FnOnce() + 'static,
    ) {
        let id = id.into();
        self.animations.insert(
            id,
            AnimationEntry {
                start: Instant::now(),
                duration,
                on_complete: Some(Box::new(on_complete)),
                completed: false,
            },
        );
    }

    pub fn on_complete(&mut self, id: &str, callback: impl FnOnce() + 'static) {
        if let Some(entry) = self.animations.get_mut(id) {
            entry.on_complete = Some(Box::new(callback));
        }
    }

    pub fn progress(&self, id: &str) -> Option<f32> {
        self.animations.get(id).map(|entry| {
            let elapsed = entry.start.elapsed().as_secs_f32();
            let total = entry.duration.as_secs_f32();
            if total <= 0.0 {
                1.0
            } else {
                (elapsed / total).clamp(0.0, 1.0)
            }
        })
    }

    pub fn is_active(&self, id: &str) -> bool {
        self.animations
            .get(id)
            .map(|entry| !entry.completed && entry.start.elapsed() < entry.duration)
            .unwrap_or(false)
    }

    pub fn is_complete(&self, id: &str) -> bool {
        self.animations
            .get(id)
            .map(|entry| entry.completed || entry.start.elapsed() >= entry.duration)
            .unwrap_or(false)
    }

    pub fn tick(&mut self) -> Vec<String> {
        let mut completed = Vec::new();

        let keys: Vec<String> = self.animations.keys().cloned().collect();
        for key in keys {
            let entry = self.animations.get(&key).unwrap();
            if !entry.completed && entry.start.elapsed() >= entry.duration {
                completed.push(key.clone());
            }
        }

        for key in &completed {
            if let Some(mut entry) = self.animations.remove(key) {
                entry.completed = true;
                if let Some(callback) = entry.on_complete.take() {
                    (callback)();
                }
            }
        }

        completed
    }

    pub fn cancel(&mut self, id: &str) {
        self.animations.remove(id);
    }

    pub fn cancel_all(&mut self) {
        self.animations.clear();
    }

    pub fn has_active_animations(&self) -> bool {
        self.animations
            .values()
            .any(|entry| !entry.completed && entry.start.elapsed() < entry.duration)
    }

    pub fn active_count(&self) -> usize {
        self.animations
            .values()
            .filter(|entry| !entry.completed && entry.start.elapsed() < entry.duration)
            .count()
    }
}
