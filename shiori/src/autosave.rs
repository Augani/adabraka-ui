use gpui::Task;

pub struct AutosaveManager {
    tasks: Vec<Option<Task<()>>>,
}

impl AutosaveManager {
    pub fn new(count: usize) -> Self {
        let mut tasks = Vec::with_capacity(count);
        tasks.resize_with(count, || None);
        Self { tasks }
    }

    pub fn push(&mut self) {
        self.tasks.push(None);
    }

    pub fn remove(&mut self, idx: usize) {
        if idx < self.tasks.len() {
            self.tasks.remove(idx);
        }
    }

    pub fn set(&mut self, idx: usize, task: Task<()>) {
        if idx < self.tasks.len() {
            self.tasks[idx] = Some(task);
        }
    }

    pub fn cancel(&mut self, idx: usize) {
        if idx < self.tasks.len() {
            self.tasks[idx] = None;
        }
    }
}
