//! ViewRouter - A view stack manager with animated page transitions for multi-screen desktop apps.

use gpui::{prelude::FluentBuilder as _, *};
use std::rc::Rc;
use std::time::Duration;

use crate::animations::easings;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PageTransition {
    SlideLeft,
    SlideRight,
    SlideUp,
    SlideDown,
    Fade,
    None,
}

impl PageTransition {
    fn reverse(&self) -> Self {
        match self {
            Self::SlideLeft => Self::SlideRight,
            Self::SlideRight => Self::SlideLeft,
            Self::SlideUp => Self::SlideDown,
            Self::SlideDown => Self::SlideUp,
            Self::Fade => Self::Fade,
            Self::None => Self::None,
        }
    }
}

struct ViewEntry {
    id: SharedString,
    render: Rc<dyn Fn(&mut Window, &mut App) -> AnyElement>,
}

pub struct ViewRouterState {
    stack: Vec<ViewEntry>,
    previous_render: Option<Rc<dyn Fn(&mut Window, &mut App) -> AnyElement>>,
    transition: PageTransition,
    active_transition: Option<PageTransition>,
    version: usize,
    is_transitioning: bool,
    duration: Duration,
}

impl ViewRouterState {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            previous_render: None,
            transition: PageTransition::SlideLeft,
            active_transition: None,
            version: 0,
            is_transitioning: false,
            duration: Duration::from_millis(300),
        }
    }

    pub fn push(
        &mut self,
        id: impl Into<SharedString>,
        render: impl Fn(&mut Window, &mut App) -> AnyElement + 'static,
        cx: &mut Context<Self>,
    ) {
        self.previous_render = self.stack.last().map(|e| e.render.clone());

        self.stack.push(ViewEntry {
            id: id.into(),
            render: Rc::new(render),
        });

        self.begin_transition(self.transition, cx);
    }

    pub fn pop(&mut self, cx: &mut Context<Self>) {
        if self.stack.len() <= 1 {
            return;
        }

        self.previous_render = self.stack.last().map(|e| e.render.clone());
        self.stack.pop();
        self.begin_transition(self.transition.reverse(), cx);
    }

    pub fn replace(
        &mut self,
        id: impl Into<SharedString>,
        render: impl Fn(&mut Window, &mut App) -> AnyElement + 'static,
        cx: &mut Context<Self>,
    ) {
        self.previous_render = self.stack.last().map(|e| e.render.clone());

        if !self.stack.is_empty() {
            self.stack.pop();
        }

        self.stack.push(ViewEntry {
            id: id.into(),
            render: Rc::new(render),
        });

        self.begin_transition(self.transition, cx);
    }

    fn begin_transition(&mut self, transition: PageTransition, cx: &mut Context<Self>) {
        if matches!(transition, PageTransition::None) {
            self.previous_render = None;
            cx.notify();
            return;
        }

        self.active_transition = Some(transition);
        self.version += 1;
        self.is_transitioning = true;
        cx.notify();

        let duration = self.duration;
        cx.spawn(async move |this, cx| {
            cx.background_executor().timer(duration).await;
            _ = this.update(cx, |this, cx| {
                this.is_transitioning = false;
                this.previous_render = None;
                this.active_transition = None;
                cx.notify();
            });
        })
        .detach();
    }

    pub fn current_id(&self) -> Option<SharedString> {
        self.stack.last().map(|e| e.id.clone())
    }

    pub fn can_go_back(&self) -> bool {
        self.stack.len() > 1
    }

    pub fn depth(&self) -> usize {
        self.stack.len()
    }

    pub fn set_transition(&mut self, transition: PageTransition) {
        self.transition = transition;
    }

    pub fn set_duration(&mut self, duration: Duration) {
        self.duration = duration;
    }
}

#[derive(IntoElement)]
pub struct ViewRouter {
    id: ElementId,
    state: Entity<ViewRouterState>,
    transition_override: Option<PageTransition>,
    style: StyleRefinement,
}

impl ViewRouter {
    pub fn new(id: impl Into<ElementId>, state: Entity<ViewRouterState>) -> Self {
        Self {
            id: id.into(),
            state,
            transition_override: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn transition(mut self, transition: PageTransition) -> Self {
        self.transition_override = Some(transition);
        self
    }
}

impl Styled for ViewRouter {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for ViewRouter {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let user_style = self.style;

        let (version, duration, transition, current_render, previous_render) = {
            let state = self.state.read(cx);
            let trans = self
                .transition_override
                .or(state.active_transition)
                .unwrap_or(PageTransition::Fade);
            let prev = if state.is_transitioning {
                state.previous_render.clone()
            } else {
                None
            };
            (
                state.version,
                state.duration,
                trans,
                state.stack.last().map(|e| e.render.clone()),
                prev,
            )
        };

        let current_content = current_render.map(|f| f(window, cx));
        let previous_content = previous_render.map(|f| f(window, cx));

        let id = self.id;

        let mut container = div().size_full().overflow_hidden().relative();

        if let Some(old) = previous_content {
            let exit_id = ElementId::Name(format!("{}-exit-{}", id, version).into());
            let enter_id = ElementId::Name(format!("{}-enter-{}", id, version).into());

            container = container.child(
                render_exit(old, exit_id, transition, duration),
            );

            container = container.when_some(current_content, |this, new| {
                this.child(render_enter(new, enter_id, transition, duration))
            });
        } else {
            container = container.when_some(current_content, |this, content| {
                this.child(div().size_full().child(content))
            });
        }

        container.map(|this| {
            let mut d = this;
            d.style().refine(&user_style);
            d
        })
    }
}

fn render_exit(
    content: AnyElement,
    anim_id: ElementId,
    transition: PageTransition,
    duration: Duration,
) -> impl IntoElement {
    div()
        .absolute()
        .inset_0()
        .size_full()
        .child(content)
        .with_animation(anim_id, anim(duration), move |el, delta| {
            apply_exit(el, delta, transition)
        })
}

fn render_enter(
    content: AnyElement,
    anim_id: ElementId,
    transition: PageTransition,
    duration: Duration,
) -> impl IntoElement {
    div()
        .size_full()
        .child(content)
        .with_animation(anim_id, anim(duration), move |el, delta| {
            apply_enter(el, delta, transition)
        })
}

fn anim(duration: Duration) -> Animation {
    Animation::new(duration).with_easing(easings::ease_out_cubic)
}

fn apply_exit(el: Div, delta: f32, transition: PageTransition) -> Div {
    let progress = delta;
    match transition {
        PageTransition::SlideLeft => el
            .left(px(-300.0 * progress))
            .opacity(1.0 - progress),
        PageTransition::SlideRight => el
            .left(px(300.0 * progress))
            .opacity(1.0 - progress),
        PageTransition::SlideUp => el
            .top(px(-300.0 * progress))
            .opacity(1.0 - progress),
        PageTransition::SlideDown => el
            .top(px(300.0 * progress))
            .opacity(1.0 - progress),
        PageTransition::Fade => el.opacity(1.0 - progress),
        PageTransition::None => el,
    }
}

fn apply_enter(el: Div, delta: f32, transition: PageTransition) -> Div {
    let progress = delta;
    match transition {
        PageTransition::SlideLeft => el
            .left(px(300.0 * (1.0 - progress)))
            .opacity(progress),
        PageTransition::SlideRight => el
            .left(px(-300.0 * (1.0 - progress)))
            .opacity(progress),
        PageTransition::SlideUp => el
            .top(px(300.0 * (1.0 - progress)))
            .opacity(progress),
        PageTransition::SlideDown => el
            .top(px(-300.0 * (1.0 - progress)))
            .opacity(progress),
        PageTransition::Fade => el.opacity(progress),
        PageTransition::None => el,
    }
}
