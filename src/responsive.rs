use gpui::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Breakpoint {
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
    Xxl,
}

impl Breakpoint {
    pub fn min_width(&self) -> f32 {
        match self {
            Self::Xs => 0.0,
            Self::Sm => 640.0,
            Self::Md => 768.0,
            Self::Lg => 1024.0,
            Self::Xl => 1280.0,
            Self::Xxl => 1536.0,
        }
    }

    pub fn from_width(width: f32) -> Self {
        if width >= 1536.0 {
            Self::Xxl
        } else if width >= 1280.0 {
            Self::Xl
        } else if width >= 1024.0 {
            Self::Lg
        } else if width >= 768.0 {
            Self::Md
        } else if width >= 640.0 {
            Self::Sm
        } else {
            Self::Xs
        }
    }
}

pub struct Responsive {
    builders: Vec<(
        Breakpoint,
        Box<dyn FnOnce(&mut Window, &mut App) -> AnyElement>,
    )>,
}

impl Responsive {
    pub fn new() -> Self {
        Self {
            builders: Vec::new(),
        }
    }

    pub fn xs(self, builder: impl FnOnce(&mut Window, &mut App) -> AnyElement + 'static) -> Self {
        self.at(Breakpoint::Xs, builder)
    }

    pub fn sm(self, builder: impl FnOnce(&mut Window, &mut App) -> AnyElement + 'static) -> Self {
        self.at(Breakpoint::Sm, builder)
    }

    pub fn md(self, builder: impl FnOnce(&mut Window, &mut App) -> AnyElement + 'static) -> Self {
        self.at(Breakpoint::Md, builder)
    }

    pub fn lg(self, builder: impl FnOnce(&mut Window, &mut App) -> AnyElement + 'static) -> Self {
        self.at(Breakpoint::Lg, builder)
    }

    pub fn xl(self, builder: impl FnOnce(&mut Window, &mut App) -> AnyElement + 'static) -> Self {
        self.at(Breakpoint::Xl, builder)
    }

    pub fn xxl(self, builder: impl FnOnce(&mut Window, &mut App) -> AnyElement + 'static) -> Self {
        self.at(Breakpoint::Xxl, builder)
    }

    fn at(
        mut self,
        breakpoint: Breakpoint,
        builder: impl FnOnce(&mut Window, &mut App) -> AnyElement + 'static,
    ) -> Self {
        self.builders.push((breakpoint, Box::new(builder)));
        self
    }

    pub fn build(mut self, window: &mut Window, cx: &mut App) -> AnyElement {
        let viewport = window.viewport_size();
        let width = f32::from(viewport.width);
        let current = Breakpoint::from_width(width);

        self.builders.sort_by(|a, b| b.0.cmp(&a.0));

        for (breakpoint, builder) in self.builders {
            if current >= breakpoint {
                return (builder)(window, cx);
            }
        }

        div().into_any_element()
    }
}

impl Default for Responsive {
    fn default() -> Self {
        Self::new()
    }
}

pub fn current_breakpoint(window: &Window) -> Breakpoint {
    let viewport = window.viewport_size();
    Breakpoint::from_width(f32::from(viewport.width))
}

pub fn responsive_value<T: Clone>(window: &Window, xs: T, sm: T, md: T, lg: T) -> T {
    match current_breakpoint(window) {
        Breakpoint::Xs => xs,
        Breakpoint::Sm => sm,
        Breakpoint::Md | Breakpoint::Lg => md,
        Breakpoint::Xl | Breakpoint::Xxl => lg,
    }
}

pub fn responsive_columns(window: &Window) -> usize {
    match current_breakpoint(window) {
        Breakpoint::Xs => 1,
        Breakpoint::Sm => 2,
        Breakpoint::Md => 3,
        Breakpoint::Lg => 4,
        Breakpoint::Xl | Breakpoint::Xxl => 6,
    }
}
