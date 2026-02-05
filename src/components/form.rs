use crate::components::input_state::InputState;
use crate::layout::VStack;
use gpui::prelude::FluentBuilder as _;
use gpui::*;
use std::collections::{HashMap, HashSet};

pub struct FormState {
    fields: Vec<(SharedString, Entity<InputState>)>,
    errors: HashMap<SharedString, Vec<String>>,
    dirty: HashSet<SharedString>,
    submitted: bool,
}

impl FormState {
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|_cx| Self {
            fields: Vec::new(),
            errors: HashMap::new(),
            dirty: HashSet::new(),
            submitted: false,
        })
    }

    pub fn register_field(&mut self, name: impl Into<SharedString>, state: Entity<InputState>) {
        let name = name.into();
        self.fields.push((name, state));
    }

    pub fn validate_all(&mut self, cx: &mut Context<Self>) -> bool {
        self.errors.clear();
        self.submitted = true;

        let fields: Vec<_> = self.fields.clone();
        let mut all_valid = true;

        for (name, field_entity) in fields {
            let result = field_entity.update(cx, |state, cx| state.validate(cx));

            if let Err(error) = result {
                all_valid = false;
                self.errors
                    .entry(name)
                    .or_default()
                    .push(error.message.to_string());
            }
        }

        cx.notify();
        all_valid
    }

    pub fn field_errors(&self, name: &str) -> Option<&Vec<String>> {
        self.errors.get(name)
    }

    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn is_dirty(&self) -> bool {
        !self.dirty.is_empty()
    }

    pub fn mark_dirty(&mut self, name: impl Into<SharedString>) {
        self.dirty.insert(name.into());
    }

    pub fn reset(&mut self, cx: &mut Context<Self>) {
        self.errors.clear();
        self.dirty.clear();
        self.submitted = false;
        cx.notify();
    }

    pub fn is_submitted(&self) -> bool {
        self.submitted
    }
}

#[derive(IntoElement)]
pub struct Form {
    state: Entity<FormState>,
    on_submit: Option<Box<dyn Fn(&mut Window, &mut App)>>,
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl Form {
    pub fn new(state: Entity<FormState>) -> Self {
        Self {
            state,
            on_submit: None,
            children: Vec::new(),
            style: StyleRefinement::default(),
        }
    }

    pub fn on_submit(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_submit = Some(Box::new(handler));
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self {
        self.children
            .extend(children.into_iter().map(|c| c.into_any_element()));
        self
    }
}

impl Styled for Form {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Form {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let state = self.state.clone();
        let on_submit = self.on_submit;
        let user_style = self.style;

        VStack::new()
            .w_full()
            .gap(px(16.0))
            .children(self.children)
            .when_some(on_submit, |this, handler| {
                let handler = std::rc::Rc::new(handler);
                this.on_key_down(
                    move |event: &KeyDownEvent, window: &mut Window, cx: &mut App| {
                        if event.keystroke.key == "enter" {
                            let is_valid =
                                state.update(cx, |form_state, cx| form_state.validate_all(cx));
                            if is_valid {
                                (handler)(window, cx);
                            }
                        }
                    },
                )
            })
            .map(|this| {
                let mut vstack = this;
                vstack.style().refine(&user_style);
                vstack
            })
    }
}
