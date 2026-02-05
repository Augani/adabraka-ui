//! Layout components - High-level layout abstractions for common UI patterns.

use crate::animations::{easings, lerp_f32};
use crate::components::scrollbar::{Scrollbar, ScrollbarAxis, ScrollbarState};
use crate::scroll_physics::ScrollPhysics;
use gpui::*;
use std::cell::RefCell;
use std::panic::Location;
use std::rc::Rc;
use std::sync::atomic::AtomicUsize;

static SCROLL_CONTAINER_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Align {
    Start,
    Center,
    End,
    Stretch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Justify {
    Start,
    Center,
    End,
    Between,
    Around,
    Evenly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlowDirection {
    Horizontal,
    Vertical,
}

pub struct VStack {
    base: Div,
    spacing: Option<Pixels>,
    align: Option<Align>,
}

impl Default for VStack {
    fn default() -> Self {
        Self::new()
    }
}

impl VStack {
    pub fn new() -> Self {
        Self {
            base: div().flex().flex_col(),
            spacing: None,
            align: None,
        }
    }

    pub fn spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        self.spacing = Some(spacing.into());
        self
    }

    pub fn gap(self, gap: impl Into<Pixels>) -> Self {
        self.spacing(gap)
    }

    pub fn align(mut self, align: Align) -> Self {
        self.align = Some(align);
        self
    }

    pub fn fill(mut self) -> Self {
        self.base = self.base.size_full();
        self
    }

    pub fn fill_width(mut self) -> Self {
        self.base = self.base.w_full();
        self
    }

    pub fn fill_height(mut self) -> Self {
        self.base = self.base.h_full();
        self
    }

    pub fn grow(mut self) -> Self {
        self.base = self.base.flex_1();
        self
    }

    pub fn padding(mut self, padding: impl Into<Pixels>) -> Self {
        self.base = self.base.p(padding.into());
        self
    }

    pub fn items_center(mut self) -> Self {
        self.align = Some(Align::Center);
        self
    }
}

impl ParentElement for VStack {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.base.extend(elements);
    }
}

impl Styled for VStack {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl InteractiveElement for VStack {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl StatefulInteractiveElement for VStack {}

impl IntoElement for VStack {
    type Element = Div;

    fn into_element(mut self) -> Self::Element {
        if let Some(spacing) = self.spacing {
            self.base = self.base.gap(spacing);
        }

        if let Some(align) = self.align {
            self.base = match align {
                Align::Start => self.base.items_start(),
                Align::Center => self.base.items_center(),
                Align::End => self.base.items_end(),
                Align::Stretch => self.base,
            };
        }

        self.base
    }
}

pub struct HStack {
    base: Div,
    spacing: Option<Pixels>,
    align: Option<Align>,
    justify: Option<Justify>,
}

impl Default for HStack {
    fn default() -> Self {
        Self::new()
    }
}

impl HStack {
    pub fn new() -> Self {
        Self {
            base: div().flex().flex_row(),
            spacing: None,
            align: None,
            justify: None,
        }
    }

    pub fn spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        self.spacing = Some(spacing.into());
        self
    }

    pub fn gap(self, gap: impl Into<Pixels>) -> Self {
        self.spacing(gap)
    }

    pub fn align(mut self, align: Align) -> Self {
        self.align = Some(align);
        self
    }

    pub fn justify(mut self, justify: Justify) -> Self {
        self.justify = Some(justify);
        self
    }

    pub fn fill(mut self) -> Self {
        self.base = self.base.size_full();
        self
    }

    pub fn fill_width(mut self) -> Self {
        self.base = self.base.w_full();
        self
    }

    pub fn fill_height(mut self) -> Self {
        self.base = self.base.h_full();
        self
    }

    pub fn grow(mut self) -> Self {
        self.base = self.base.flex_1();
        self
    }

    pub fn padding(mut self, padding: impl Into<Pixels>) -> Self {
        self.base = self.base.p(padding.into());
        self
    }

    pub fn items_center(mut self) -> Self {
        self.align = Some(Align::Center);
        self
    }

    pub fn space_between(mut self) -> Self {
        self.justify = Some(Justify::Between);
        self
    }
}

impl ParentElement for HStack {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.base.extend(elements);
    }
}

impl Styled for HStack {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl InteractiveElement for HStack {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl StatefulInteractiveElement for HStack {}

impl IntoElement for HStack {
    type Element = Div;

    fn into_element(mut self) -> Self::Element {
        if let Some(spacing) = self.spacing {
            self.base = self.base.gap(spacing);
        }

        if let Some(align) = self.align {
            self.base = match align {
                Align::Start => self.base.items_start(),
                Align::Center => self.base.items_center(),
                Align::End => self.base.items_end(),
                Align::Stretch => self.base,
            };
        }

        if let Some(justify) = self.justify {
            self.base = match justify {
                Justify::Start => self.base.justify_start(),
                Justify::Center => self.base.justify_center(),
                Justify::End => self.base.justify_end(),
                Justify::Between => self.base.justify_between(),
                Justify::Around => self.base.justify_around(),
                Justify::Evenly => self.base.justify_around(),
            };
        }

        self.base
    }
}

pub struct Flow {
    base: Div,
    direction: FlowDirection,
    spacing: Option<Pixels>,
    align: Option<Align>,
}

impl Default for Flow {
    fn default() -> Self {
        Self::new()
    }
}

impl Flow {
    pub fn new() -> Self {
        Self {
            base: div().flex().flex_wrap(),
            direction: FlowDirection::Horizontal,
            spacing: None,
            align: None,
        }
    }

    pub fn direction(mut self, direction: FlowDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        self.spacing = Some(spacing.into());
        self
    }

    pub fn align(mut self, align: Align) -> Self {
        self.align = Some(align);
        self
    }
}

impl ParentElement for Flow {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.base.extend(elements);
    }
}

impl Styled for Flow {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl InteractiveElement for Flow {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl StatefulInteractiveElement for Flow {}

impl IntoElement for Flow {
    type Element = Div;

    fn into_element(mut self) -> Self::Element {
        self.base = match self.direction {
            FlowDirection::Horizontal => self.base.flex_row(),
            FlowDirection::Vertical => self.base.flex_col(),
        };

        if let Some(spacing) = self.spacing {
            self.base = self.base.gap(spacing);
        }

        if let Some(align) = self.align {
            self.base = match align {
                Align::Start => self.base.items_start(),
                Align::Center => self.base.items_center(),
                Align::End => self.base.items_end(),
                Align::Stretch => self.base,
            };
        }

        self.base
    }
}

pub struct Grid {
    base: Div,
    columns: usize,
    gap: Option<Pixels>,
    grid_children: Vec<AnyElement>,
}

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
}

impl Grid {
    pub fn new() -> Self {
        Self {
            base: div().flex().flex_col(),
            columns: 1,
            gap: None,
            grid_children: vec![],
        }
    }

    pub fn columns(mut self, columns: usize) -> Self {
        self.columns = columns.max(1);
        self
    }

    pub fn gap(mut self, gap: impl Into<Pixels>) -> Self {
        self.gap = Some(gap.into());
        self
    }
}

impl ParentElement for Grid {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.grid_children.extend(elements);
    }
}

impl Styled for Grid {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl InteractiveElement for Grid {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl StatefulInteractiveElement for Grid {}

impl IntoElement for Grid {
    type Element = Div;

    fn into_element(mut self) -> Self::Element {
        if let Some(gap) = self.gap {
            self.base = self.base.gap(gap);
        }

        let total_children = self.grid_children.len();
        let mut rows = vec![];
        let mut current_row = vec![];

        for (i, child) in self.grid_children.into_iter().enumerate() {
            current_row.push(child);
            if (i + 1) % self.columns == 0 || i == total_children - 1 {
                rows.push(current_row);
                current_row = vec![];
            }
        }

        for row_children in rows {
            let mut row = div().flex().flex_row().w_full();

            if let Some(gap) = self.gap {
                row = row.gap(gap);
            }

            for child in row_children {
                row = row.child(div().flex_1().child(child));
            }

            self.base = self.base.child(row);
        }

        self.base
    }
}

pub struct Cluster {
    base: Div,
    spacing: Option<Pixels>,
    align: Option<Align>,
}

impl Default for Cluster {
    fn default() -> Self {
        Self::new()
    }
}

impl Cluster {
    pub fn new() -> Self {
        Self {
            base: div().flex().flex_row(),
            spacing: None,
            align: None,
        }
    }

    pub fn spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        self.spacing = Some(spacing.into());
        self
    }

    pub fn align(mut self, align: Align) -> Self {
        self.align = Some(align);
        self
    }
}

impl ParentElement for Cluster {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.base.extend(elements);
    }
}

impl Styled for Cluster {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl InteractiveElement for Cluster {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl StatefulInteractiveElement for Cluster {}

impl IntoElement for Cluster {
    type Element = Div;

    fn into_element(mut self) -> Self::Element {
        if let Some(spacing) = self.spacing {
            self.base = self.base.gap(spacing);
        }

        if let Some(align) = self.align {
            self.base = match align {
                Align::Start => self.base.items_start(),
                Align::Center => self.base.items_center(),
                Align::End => self.base.items_end(),
                Align::Stretch => self.base,
            };
        }

        self.base
    }
}

pub struct Spacer {
    size: Option<Pixels>,
}

impl Default for Spacer {
    fn default() -> Self {
        Self::new()
    }
}

impl Spacer {
    pub fn new() -> Self {
        Self { size: None }
    }

    pub fn fixed(size: impl Into<Pixels>) -> Self {
        Self {
            size: Some(size.into()),
        }
    }
}

impl IntoElement for Spacer {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        if let Some(size) = self.size {
            div().size(size)
        } else {
            div().flex_1()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollDirection {
    Vertical,
    Horizontal,
    Both,
}

const PIXELS_PER_LINE: f32 = 20.0;

#[derive(Clone)]
pub struct PhysicsScrollState {
    inner: Rc<RefCell<PhysicsScrollInner>>,
}

struct PhysicsScrollInner {
    physics_y: ScrollPhysics,
    physics_x: ScrollPhysics,
    scroll_target: Option<ScrollAnimTarget>,
    animating: bool,
    generation: u64,
}

struct ScrollAnimTarget {
    start_y: f32,
    end_y: f32,
    start_x: f32,
    end_x: f32,
    progress: f32,
    animate_y: bool,
    animate_x: bool,
}

impl Default for PhysicsScrollState {
    fn default() -> Self {
        Self::new()
    }
}

impl PhysicsScrollState {
    pub fn new() -> Self {
        Self {
            inner: Rc::new(RefCell::new(PhysicsScrollInner {
                physics_y: ScrollPhysics::new(),
                physics_x: ScrollPhysics::new(),
                scroll_target: None,
                animating: false,
                generation: 0,
            })),
        }
    }

    pub fn with_deceleration(self, deceleration: f32) -> Self {
        let mut inner = self.inner.borrow_mut();
        inner.physics_y = inner.physics_y.clone().with_deceleration(deceleration);
        inner.physics_x = inner.physics_x.clone().with_deceleration(deceleration);
        drop(inner);
        self
    }

    pub fn momentum(self, enabled: bool) -> Self {
        let mut inner = self.inner.borrow_mut();
        inner.physics_y = inner.physics_y.clone().momentum(enabled);
        inner.physics_x = inner.physics_x.clone().momentum(enabled);
        drop(inner);
        self
    }

    pub fn overscroll(self, enabled: bool) -> Self {
        let mut inner = self.inner.borrow_mut();
        inner.physics_y = inner.physics_y.clone().overscroll(enabled);
        inner.physics_x = inner.physics_x.clone().overscroll(enabled);
        drop(inner);
        self
    }

    pub fn scroll_to_y_animated(&self, target: f32, handle: &ScrollHandle, window: &Window) {
        let mut inner = self.inner.borrow_mut();
        let current = -f32::from(handle.offset().y);
        inner.physics_y.set_position(current);
        inner.physics_x.set_position(-f32::from(handle.offset().x));
        inner.physics_y.stop();
        inner.scroll_target = Some(ScrollAnimTarget {
            start_y: current,
            end_y: target,
            start_x: 0.0,
            end_x: 0.0,
            progress: 0.0,
            animate_y: true,
            animate_x: false,
        });
        inner.animating = true;
        inner.generation += 1;
        let gen = inner.generation;
        drop(inner);
        drive_physics_frame(self.clone(), handle.clone(), gen, window);
    }

    pub fn scroll_to_x_animated(&self, target: f32, handle: &ScrollHandle, window: &Window) {
        let mut inner = self.inner.borrow_mut();
        let current = -f32::from(handle.offset().x);
        inner.physics_y.set_position(-f32::from(handle.offset().y));
        inner.physics_x.set_position(current);
        inner.physics_x.stop();
        inner.scroll_target = Some(ScrollAnimTarget {
            start_y: 0.0,
            end_y: 0.0,
            start_x: current,
            end_x: target,
            progress: 0.0,
            animate_y: false,
            animate_x: true,
        });
        inner.animating = true;
        inner.generation += 1;
        let gen = inner.generation;
        drop(inner);
        drive_physics_frame(self.clone(), handle.clone(), gen, window);
    }

    pub fn handle_scroll_event(
        &self,
        handle: &ScrollHandle,
        direction: ScrollDirection,
        event: &ScrollWheelEvent,
        window: &Window,
    ) {
        let (delta_x, delta_y) = match &event.delta {
            ScrollDelta::Lines(d) => (d.x * PIXELS_PER_LINE, d.y * PIXELS_PER_LINE),
            ScrollDelta::Pixels(d) => (f32::from(d.x), f32::from(d.y)),
        };

        let mut inner = self.inner.borrow_mut();
        inner.scroll_target = None;
        inner.generation += 1;
        let gen = inner.generation;

        match direction {
            ScrollDirection::Vertical => inner.physics_y.apply_delta(-delta_y),
            ScrollDirection::Horizontal => inner.physics_x.apply_delta(-delta_x),
            ScrollDirection::Both => {
                inner.physics_y.apply_delta(-delta_y);
                inner.physics_x.apply_delta(-delta_x);
            }
        }

        let offset = handle.offset();
        inner.physics_y.set_position(-f32::from(offset.y));
        inner.physics_x.set_position(-f32::from(offset.x));

        inner.animating = true;
        drop(inner);

        drive_physics_frame(self.clone(), handle.clone(), gen, window);
    }

    pub fn is_animating(&self) -> bool {
        self.inner.borrow().animating
    }

    pub fn stop(&self) {
        let mut inner = self.inner.borrow_mut();
        inner.physics_y.stop();
        inner.physics_x.stop();
        inner.scroll_target = None;
        inner.animating = false;
    }
}

fn drive_physics_frame(
    state: PhysicsScrollState,
    handle: ScrollHandle,
    generation: u64,
    window: &Window,
) {
    let state_c = state.clone();
    let handle_c = handle.clone();
    window.on_next_frame(move |window, _cx| {
        let mut inner = state_c.inner.borrow_mut();
        if inner.generation != generation || !inner.animating {
            return;
        }

        let dt = 1.0 / 60.0;
        let active;

        let target_result = if let Some(ref mut target) = inner.scroll_target {
            target.progress = (target.progress + dt * 3.33).min(1.0);
            let t = easings::ease_out_cubic(target.progress);
            let y_pos = if target.animate_y {
                Some(lerp_f32(target.start_y, target.end_y, t))
            } else {
                None
            };
            let x_pos = if target.animate_x {
                Some(lerp_f32(target.start_x, target.end_x, t))
            } else {
                None
            };
            let done = target.progress >= 1.0;
            Some((y_pos, x_pos, done))
        } else {
            None
        };

        if let Some((y_pos, x_pos, done)) = target_result {
            if let Some(y) = y_pos {
                inner.physics_y.set_position(y);
            }
            if let Some(x) = x_pos {
                inner.physics_x.set_position(x);
            }
            if done {
                inner.scroll_target = None;
                active = false;
            } else {
                active = true;
            }
        } else {
            active = inner.physics_y.tick(dt) | inner.physics_x.tick(dt);
        }

        let mut offset = handle_c.offset();
        let y_managed = inner.physics_y.is_moving()
            || inner.physics_y.is_overscrolled()
            || inner.scroll_target.as_ref().is_some_and(|t| t.animate_y);
        let x_managed = inner.physics_x.is_moving()
            || inner.physics_x.is_overscrolled()
            || inner.scroll_target.as_ref().is_some_and(|t| t.animate_x);

        if y_managed {
            offset.y = px(-inner.physics_y.position());
        }
        if x_managed {
            offset.x = px(-inner.physics_x.position());
        }
        handle_c.set_offset(offset);

        if active {
            drop(inner);
            window.refresh();
            drive_physics_frame(state_c.clone(), handle_c.clone(), generation, window);
        } else {
            inner.animating = false;
        }
    });
}

pub struct ScrollContainer {
    base: Div,
    direction: ScrollDirection,
    scroll_handle: Option<ScrollHandle>,
    auto_size: bool,
    smooth_scroll: bool,
    custom_id: Option<ElementId>,
    auto_id: ElementId,
    show_scrollbar: bool,
    horizontal_top: bool,
    scrollbar_state: ScrollbarState,
    physics_state: Option<PhysicsScrollState>,
}

impl ScrollContainer {
    #[track_caller]
    pub fn new(direction: ScrollDirection) -> Self {
        let location = Location::caller();
        let auto_id = ElementId::Name(
            format!(
                "scroll-container:{}:{}:{}",
                location.file(),
                location.line(),
                location.column()
            )
            .into(),
        );

        Self {
            base: div(),
            direction,
            scroll_handle: None,
            auto_size: false,
            smooth_scroll: true,
            custom_id: None,
            auto_id,
            show_scrollbar: false,
            horizontal_top: false,
            scrollbar_state: ScrollbarState::default(),
            physics_state: None,
        }
    }

    #[track_caller]
    pub fn vertical() -> Self {
        Self::new(ScrollDirection::Vertical)
    }

    #[track_caller]
    pub fn horizontal() -> Self {
        Self::new(ScrollDirection::Horizontal)
    }

    #[track_caller]
    pub fn both() -> Self {
        Self::new(ScrollDirection::Both)
    }

    pub fn track_scroll(mut self, handle: &ScrollHandle) -> Self {
        self.scroll_handle = Some(handle.clone());
        self
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.custom_id = Some(id.into());
        self
    }

    pub fn no_auto_size(mut self) -> Self {
        self.auto_size = false;
        self
    }

    /// Smooth scrolling reduces vibrating effect during scroll by allowing concurrent scroll in both axes
    pub fn smooth(mut self) -> Self {
        self.smooth_scroll = true;
        self
    }

    pub fn no_smooth(mut self) -> Self {
        self.smooth_scroll = false;
        self
    }

    pub fn flex_grow(mut self) -> Self {
        self.base = self.base.flex_1();
        self
    }

    pub fn with_scrollbar(mut self) -> Self {
        self.show_scrollbar = true;
        self
    }

    pub fn horizontal_bar_top(mut self) -> Self {
        self.horizontal_top = true;
        self
    }

    pub fn horizontal_bar_bottom(mut self) -> Self {
        self.horizontal_top = false;
        self
    }

    pub fn with_physics(mut self, state: &PhysicsScrollState) -> Self {
        self.physics_state = Some(state.clone());
        self
    }

    pub fn momentum(mut self, enabled: bool) -> Self {
        let state = self
            .physics_state
            .get_or_insert_with(PhysicsScrollState::new);
        {
            let mut inner = state.inner.borrow_mut();
            inner.physics_y = inner.physics_y.clone().momentum(enabled);
            inner.physics_x = inner.physics_x.clone().momentum(enabled);
        }
        self
    }

    pub fn overscroll_bounce(mut self, enabled: bool) -> Self {
        let state = self
            .physics_state
            .get_or_insert_with(PhysicsScrollState::new);
        {
            let mut inner = state.inner.borrow_mut();
            inner.physics_y = inner.physics_y.clone().overscroll(enabled);
            inner.physics_x = inner.physics_x.clone().overscroll(enabled);
        }
        self
    }
}

impl ParentElement for ScrollContainer {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.base.extend(elements);
    }
}

impl Styled for ScrollContainer {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl InteractiveElement for ScrollContainer {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl StatefulInteractiveElement for ScrollContainer {}

impl IntoElement for ScrollContainer {
    type Element = Stateful<Div>;

    fn into_element(self) -> Self::Element {
        let id_to_use = self.custom_id.clone().unwrap_or(self.auto_id.clone());
        let handle = self.scroll_handle.clone().unwrap_or_else(ScrollHandle::new);

        if !self.show_scrollbar {
            let mut scrollable = self.base.id(id_to_use);

            if let Some(handle) = &self.scroll_handle {
                scrollable = scrollable.track_scroll(handle);
            }

            scrollable = match self.direction {
                ScrollDirection::Vertical => scrollable.overflow_y_scroll(),
                ScrollDirection::Horizontal => scrollable.overflow_x_scroll(),
                ScrollDirection::Both => scrollable.overflow_scroll(),
            };

            if let Some(ref physics) = self.physics_state {
                let physics_c = physics.clone();
                let handle_c = handle.clone();
                let dir = self.direction;
                scrollable = scrollable.on_scroll_wheel(move |event, window, _cx| {
                    physics_c.handle_scroll_event(&handle_c, dir, event, window);
                });
            }

            return scrollable;
        } else {
            let scrollbar_state = self.scrollbar_state.clone();
            scrollbar_state.init_visible();

            let axis = match self.direction {
                ScrollDirection::Vertical => ScrollbarAxis::Vertical,
                ScrollDirection::Horizontal => ScrollbarAxis::Horizontal,
                ScrollDirection::Both => ScrollbarAxis::Both,
            };
            let mut scrollbar = Scrollbar::both(&scrollbar_state, &handle).axis(axis);
            if self.horizontal_top {
                scrollbar = scrollbar.horizontal_top();
            }

            let mut scrollable = self.base.id(id_to_use.clone()).track_scroll(&handle);

            scrollable = match self.direction {
                ScrollDirection::Vertical => scrollable.overflow_y_scroll(),
                ScrollDirection::Horizontal => scrollable.overflow_x_scroll(),
                ScrollDirection::Both => scrollable.overflow_scroll(),
            };

            scrollable = scrollable.relative().size_full();

            if let Some(ref physics) = self.physics_state {
                let physics_c = physics.clone();
                let handle_c = handle.clone();
                let dir = self.direction;
                scrollable = scrollable.on_scroll_wheel(move |event, window, _cx| {
                    physics_c.handle_scroll_event(&handle_c, dir, event, window);
                });
            }

            let wrapper = div()
                .id(ElementId::Name(format!("{}-wrapper", id_to_use).into()))
                .relative()
                .size_full()
                .child(scrollable)
                .child(scrollbar);

            wrapper
        }
    }
}

pub struct Panel {
    base: Div,
}

impl Default for Panel {
    fn default() -> Self {
        Self::new()
    }
}

impl Panel {
    pub fn new() -> Self {
        Self { base: div() }
    }

    pub fn card(mut self) -> Self {
        self.base = self.base.border_1().rounded(px(8.0)).p(px(16.0));
        self
    }

    pub fn elevated(mut self) -> Self {
        self.base = self.base.border_1().rounded(px(8.0));
        self
    }

    pub fn section(mut self) -> Self {
        self.base = self.base.border_b_1().p(px(12.0));
        self
    }

    pub fn border(mut self) -> Self {
        self.base = self.base.border_1();
        self
    }

    pub fn rounded(mut self) -> Self {
        self.base = self.base.rounded(px(8.0));
        self
    }

    pub fn padded(mut self) -> Self {
        self.base = self.base.p(px(16.0));
        self
    }
}

impl ParentElement for Panel {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.base.extend(elements);
    }
}

impl Styled for Panel {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl InteractiveElement for Panel {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl StatefulInteractiveElement for Panel {}

impl IntoElement for Panel {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        self.base
    }
}

pub struct Container {
    base: Div,
    max_width: Option<Pixels>,
    centered: bool,
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}

impl Container {
    pub fn new() -> Self {
        Self {
            base: div().w_full(),
            max_width: None,
            centered: false,
        }
    }

    pub fn max_w(mut self, width: impl Into<Pixels>) -> Self {
        self.max_width = Some(width.into());
        self
    }

    pub fn centered(mut self) -> Self {
        self.centered = true;
        self
    }

    pub fn sm() -> Self {
        Self::new().max_w(px(640.0)).centered()
    }

    pub fn md() -> Self {
        Self::new().max_w(px(768.0)).centered()
    }

    pub fn lg() -> Self {
        Self::new().max_w(px(1024.0)).centered()
    }

    pub fn xl() -> Self {
        Self::new().max_w(px(1280.0)).centered()
    }

    pub fn xxl() -> Self {
        Self::new().max_w(px(1536.0)).centered()
    }
}

impl ParentElement for Container {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.base.extend(elements);
    }
}

impl Styled for Container {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl InteractiveElement for Container {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl StatefulInteractiveElement for Container {}

impl IntoElement for Container {
    type Element = Div;

    fn into_element(mut self) -> Self::Element {
        if let Some(max_width) = self.max_width {
            self.base = self.base.max_w(max_width);
        }

        if self.centered {
            self.base = self.base.mx_auto();
        }

        self.base
    }
}

pub struct MasonryItem {
    element: AnyElement,
    estimated_height: f32,
}

impl MasonryItem {
    pub fn new(element: impl IntoElement, estimated_height: f32) -> Self {
        Self {
            element: element.into_any_element(),
            estimated_height,
        }
    }
}

pub struct MasonryGrid {
    base: Div,
    columns: usize,
    gap: Option<Pixels>,
    items: Vec<MasonryItem>,
}

impl Default for MasonryGrid {
    fn default() -> Self {
        Self::new()
    }
}

impl MasonryGrid {
    pub fn new() -> Self {
        Self {
            base: div().flex().flex_row(),
            columns: 3,
            gap: None,
            items: vec![],
        }
    }

    pub fn columns(mut self, columns: usize) -> Self {
        self.columns = columns.max(1);
        self
    }

    pub fn gap(mut self, gap: impl Into<Pixels>) -> Self {
        self.gap = Some(gap.into());
        self
    }

    pub fn fill(mut self) -> Self {
        self.base = self.base.size_full();
        self
    }

    pub fn fill_width(mut self) -> Self {
        self.base = self.base.w_full();
        self
    }

    pub fn item(mut self, element: impl IntoElement, estimated_height: f32) -> Self {
        self.items.push(MasonryItem::new(element, estimated_height));
        self
    }

    pub fn items<I, E>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = (E, f32)>,
        E: IntoElement,
    {
        for (element, height) in items {
            self.items.push(MasonryItem::new(element, height));
        }
        self
    }
}

impl ParentElement for MasonryGrid {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        for element in elements {
            self.items.push(MasonryItem {
                element,
                estimated_height: 100.0,
            });
        }
    }
}

impl Styled for MasonryGrid {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl InteractiveElement for MasonryGrid {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl StatefulInteractiveElement for MasonryGrid {}

impl IntoElement for MasonryGrid {
    type Element = Div;

    fn into_element(mut self) -> Self::Element {
        if let Some(gap) = self.gap {
            self.base = self.base.gap(gap);
        }

        let mut column_heights: Vec<f32> = vec![0.0; self.columns];
        let mut column_items: Vec<Vec<AnyElement>> =
            (0..self.columns).map(|_| Vec::new()).collect();

        let gap_value: f32 = self.gap.map(|g| f32::from(g)).unwrap_or(0.0);

        for item in self.items {
            let min_column = column_heights
                .iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                .map(|(idx, _)| idx)
                .unwrap_or(0);

            column_heights[min_column] += item.estimated_height + gap_value;
            column_items[min_column].push(item.element);
        }

        for column_children in column_items {
            let mut column = div().flex().flex_col().flex_1();

            if let Some(gap) = self.gap {
                column = column.gap(gap);
            }

            for child in column_children {
                column = column.child(child);
            }

            self.base = self.base.child(column);
        }

        self.base
    }
}

pub struct ScrollList {
    scroll_container: ScrollContainer,
    stack: VStack,
}

impl Default for ScrollList {
    fn default() -> Self {
        Self::new()
    }
}

impl ScrollList {
    pub fn new() -> Self {
        Self {
            scroll_container: ScrollContainer::vertical().flex_grow(),
            stack: VStack::new(),
        }
    }

    pub fn spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        self.stack = self.stack.spacing(spacing);
        self
    }

    pub fn align(mut self, align: Align) -> Self {
        self.stack = self.stack.align(align);
        self
    }

    pub fn track_scroll(mut self, handle: &ScrollHandle) -> Self {
        self.scroll_container = self.scroll_container.track_scroll(handle);
        self
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.scroll_container = self.scroll_container.id(id);
        self
    }

    pub fn no_flex_grow(mut self) -> Self {
        let id = SharedString::from(format!(
            "scroll-list-{}",
            SCROLL_CONTAINER_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        ));
        self.scroll_container = ScrollContainer::vertical().id(id);
        self
    }

    pub fn momentum(mut self, enabled: bool) -> Self {
        self.scroll_container = self.scroll_container.momentum(enabled);
        self
    }

    pub fn overscroll_bounce(mut self, enabled: bool) -> Self {
        self.scroll_container = self.scroll_container.overscroll_bounce(enabled);
        self
    }

    pub fn with_physics(mut self, state: &PhysicsScrollState) -> Self {
        self.scroll_container = self.scroll_container.with_physics(state);
        self
    }
}

impl ParentElement for ScrollList {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.stack.extend(elements);
    }
}

impl Styled for ScrollList {
    fn style(&mut self) -> &mut StyleRefinement {
        self.scroll_container.style()
    }
}

impl InteractiveElement for ScrollList {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.scroll_container.interactivity()
    }
}

impl StatefulInteractiveElement for ScrollList {}

impl IntoElement for ScrollList {
    type Element = Stateful<Div>;

    fn into_element(self) -> Self::Element {
        let mut scroll_container = self.scroll_container;
        scroll_container = scroll_container.child(self.stack);
        scroll_container.into_element()
    }
}
