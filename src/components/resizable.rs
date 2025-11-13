//! Resizable panel component - Split-pane layouts with drag handles.

use std::{cell::Cell, ops::Range, rc::Rc};

use gpui::{prelude::FluentBuilder as _, *};

use crate::{theme::use_theme, util::AxisExt};

const PANEL_MIN_SIZE: Pixels = px(100.0);
const HANDLE_PADDING: Pixels = px(4.0);
const HANDLE_SIZE: Pixels = px(1.0);

pub fn h_resizable(id: impl Into<ElementId>, state: Entity<ResizableState>) -> ResizablePanelGroup {
    ResizablePanelGroup::new(id, state).axis(Axis::Horizontal)
}

pub fn v_resizable(id: impl Into<ElementId>, state: Entity<ResizableState>) -> ResizablePanelGroup {
    ResizablePanelGroup::new(id, state).axis(Axis::Vertical)
}

pub fn resizable_panel() -> ResizablePanel {
    ResizablePanel::new()
}

#[derive(Clone, Debug)]
pub enum ResizablePanelEvent {
    Resized {
        panel_index: usize,
        new_size: Pixels,
    },
}

#[derive(Debug, Clone)]
pub struct ResizableState {
    axis: Axis,
    panels: Vec<ResizablePanelState>,
    sizes: Vec<Pixels>,
    resizing_panel_ix: Option<usize>,
    bounds: Bounds<Pixels>,
}

impl ResizableState {
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|_| Self {
            axis: Axis::Horizontal,
            panels: vec![],
            sizes: vec![],
            resizing_panel_ix: None,
            bounds: Bounds::default(),
        })
    }

    pub fn insert_panel(
        &mut self,
        size: Option<Pixels>,
        index: Option<usize>,
        cx: &mut Context<Self>,
    ) {
        let panel_state = ResizablePanelState {
            size,
            ..Default::default()
        };

        if let Some(index) = index {
            self.panels.insert(index, panel_state);
            self.sizes.insert(index, size.unwrap_or(PANEL_MIN_SIZE));
        } else {
            self.panels.push(panel_state);
            self.sizes.push(size.unwrap_or(PANEL_MIN_SIZE));
        }

        cx.notify();
    }

    pub fn remove_panel(&mut self, index: usize, cx: &mut Context<Self>) {
        if index >= self.panels.len() {
            return;
        }

        self.panels.remove(index);
        self.sizes.remove(index);

        if let Some(resizing_ix) = self.resizing_panel_ix {
            if resizing_ix > index {
                self.resizing_panel_ix = Some(resizing_ix - 1);
            } else if resizing_ix == index {
                self.resizing_panel_ix = None;
            }
        }

        cx.notify();
    }

    pub fn sizes(&self) -> &Vec<Pixels> {
        &self.sizes
    }

    pub fn total_size(&self) -> Pixels {
        self.sizes.iter().fold(px(0.0), |acc, &size| acc + size)
    }

    pub fn clear(&mut self) {
        self.panels.clear();
        self.sizes.clear();
    }

    fn sync_panels_count(&mut self, axis: Axis, panels_count: usize) {
        self.axis = axis;

        if panels_count > self.panels.len() {
            let diff = panels_count - self.panels.len();
            self.panels
                .extend(vec![ResizablePanelState::default(); diff]);
            self.sizes.extend(vec![PANEL_MIN_SIZE; diff]);
        }
    }

    fn update_panel_size(
        &mut self,
        index: usize,
        bounds: Bounds<Pixels>,
        size_range: Range<Pixels>,
        cx: &mut Context<Self>,
    ) {
        if index >= self.panels.len() {
            return;
        }

        let size = bounds.size.along(self.axis);
        self.sizes[index] = size;
        self.panels[index].size = Some(size);
        self.panels[index].bounds = bounds;
        self.panels[index].size_range = size_range;

        cx.notify();
    }

    fn done_resizing(&mut self, cx: &mut Context<Self>) {
        if let Some(index) = self.resizing_panel_ix {
            let new_size = self.sizes.get(index).copied().unwrap_or(PANEL_MIN_SIZE);

            cx.emit(ResizablePanelEvent::Resized {
                panel_index: index,
                new_size,
            });
        }

        self.resizing_panel_ix = None;
    }

    fn panel_size_range(&self, index: usize) -> Range<Pixels> {
        self.panels
            .get(index)
            .map(|p| p.size_range.clone())
            .unwrap_or(PANEL_MIN_SIZE..Pixels::MAX)
    }

    fn sync_real_panel_sizes(&mut self, _: &App) {
        for (i, panel) in self.panels.iter().enumerate() {
            if i < self.sizes.len() {
                self.sizes[i] = panel.bounds.size.along(self.axis).floor();
            }
        }
    }

    fn resize_panel(&mut self, index: usize, size: Pixels, _: &mut Window, cx: &mut Context<Self>) {
        let old_sizes = self.sizes.clone();

        if index >= old_sizes.len() - 1 {
            return;
        }

        let size = size.floor();
        let container_size = self.bounds.size.along(self.axis);

        self.sync_real_panel_sizes(cx);

        let move_changed = size - old_sizes[index];
        if move_changed == px(0.0) {
            return;
        }

        let size_range = self.panel_size_range(index);
        let new_size = size.clamp(size_range.start, size_range.end);
        let is_expand = move_changed > px(0.0);

        let main_ix = index;
        let mut new_sizes = old_sizes.clone();
        let mut ix = index;

        if is_expand {
            let mut changed = new_size - old_sizes[index];
            new_sizes[index] = new_size;

            while changed > px(0.0) && ix < old_sizes.len() - 1 {
                ix += 1;
                let size_range = self.panel_size_range(ix);
                let available_size = (new_sizes[ix] - size_range.start).max(px(0.0));
                let to_reduce = changed.min(available_size);
                new_sizes[ix] -= to_reduce;
                changed -= to_reduce;
            }
        } else {
            let mut changed = old_sizes[index] - new_size;
            new_sizes[index + 1] += changed;
            new_sizes[index] = new_size;

            let right_size_range = self.panel_size_range(index + 1);
            if new_sizes[index + 1] > right_size_range.end {
                let overflow = new_sizes[index + 1] - right_size_range.end;
                new_sizes[index + 1] = right_size_range.end;
                changed = overflow;

                while changed > px(0.0) && ix > 0 {
                    ix -= 1;
                    let size_range = self.panel_size_range(ix);
                    let available_size = (new_sizes[ix] - size_range.start).max(px(0.0));
                    let to_reduce = changed.min(available_size);
                    changed -= to_reduce;
                    new_sizes[ix] -= to_reduce;
                }
            }
        }

        let total_size: Pixels = new_sizes.iter().fold(px(0.0), |acc, &size| acc + size);
        if total_size > container_size {
            let overflow = total_size - container_size;
            let size_range = self.panel_size_range(main_ix);
            new_sizes[main_ix] = (new_sizes[main_ix] - overflow).max(size_range.start);
        }

        for (i, _) in old_sizes.iter().enumerate() {
            if i < new_sizes.len() && i < self.panels.len() {
                let size = new_sizes[i];
                self.panels[i].size = Some(size);
            }
        }

        self.sizes = new_sizes;
        cx.notify();
    }
}

impl EventEmitter<ResizablePanelEvent> for ResizableState {}

#[derive(Debug, Clone, Default)]
struct ResizablePanelState {
    size: Option<Pixels>,
    size_range: Range<Pixels>,
    bounds: Bounds<Pixels>,
}

/// A container for resizable panels with drag handles between them.
#[derive(IntoElement)]
pub struct ResizablePanelGroup {
    id: ElementId,
    state: Entity<ResizableState>,
    axis: Axis,
    children: Vec<ResizablePanel>,
}

impl ResizablePanelGroup {
    fn new(id: impl Into<ElementId>, state: Entity<ResizableState>) -> Self {
        Self {
            id: id.into(),
            axis: Axis::Horizontal,
            children: vec![],
            state,
        }
    }

    pub fn axis(mut self, axis: Axis) -> Self {
        self.axis = axis;
        self
    }

    pub fn child(mut self, panel: impl Into<ResizablePanel>) -> Self {
        self.children.push(panel.into());
        self
    }

    pub fn children<I>(mut self, panels: impl IntoIterator<Item = I>) -> Self
    where
        I: Into<ResizablePanel>,
    {
        self.children = panels.into_iter().map(|panel| panel.into()).collect();
        self
    }

    pub fn group(self, group: ResizablePanelGroup) -> Self {
        self.child(resizable_panel().child(group.into_any_element()))
    }
}

impl<T> From<T> for ResizablePanel
where
    T: Into<AnyElement>,
{
    fn from(value: T) -> Self {
        resizable_panel().child(value.into())
    }
}

impl EventEmitter<ResizablePanelEvent> for ResizablePanelGroup {}

impl RenderOnce for ResizablePanelGroup {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = self.state.clone();

        let panels_count = self.children.len();
        self.state.update(cx, |state, _| {
            state.sync_panels_count(self.axis, panels_count);
        });

        let container = div()
            .id(self.id)
            .flex()
            .size_full()
            .when(self.axis.is_horizontal(), |this| this.flex_row())
            .when(self.axis.is_vertical(), |this| this.flex_col());

        container
            .children(
                self.children
                    .into_iter()
                    .enumerate()
                    .map(|(index, mut panel)| {
                        panel.index = index;
                        panel.axis = self.axis;
                        panel.state = Some(self.state.clone());
                        panel
                    }),
            )
            .child({
                canvas(
                    move |bounds, _, cx| {
                        state.update(cx, |state, _| {
                            state.bounds = bounds;
                        })
                    },
                    |_, _, _, _| {},
                )
                .absolute()
                .size_full()
            })
            .child(ResizePanelGroupElement {
                state: self.state.clone(),
                axis: self.axis,
            })
    }
}

/// A single resizable panel within a ResizablePanelGroup.
#[derive(IntoElement)]
pub struct ResizablePanel {
    axis: Axis,
    index: usize,
    state: Option<Entity<ResizableState>>,
    initial_size: Option<Pixels>,
    size_range: Range<Pixels>,
    children: Vec<AnyElement>,
    visible: bool,
    style: StyleRefinement,
}

impl ResizablePanel {
    fn new() -> Self {
        Self {
            index: 0,
            initial_size: None,
            state: None,
            size_range: (PANEL_MIN_SIZE..Pixels::MAX),
            axis: Axis::Horizontal,
            children: vec![],
            visible: true,
            style: StyleRefinement::default(),
        }
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.initial_size = Some(size.into());
        self
    }

    pub fn size_range(mut self, range: impl Into<Range<Pixels>>) -> Self {
        self.size_range = range.into();
        self
    }

    pub fn min_size(mut self, min: impl Into<Pixels>) -> Self {
        self.size_range.start = min.into();
        self
    }

    pub fn max_size(mut self, max: impl Into<Pixels>) -> Self {
        self.size_range.end = max.into();
        self
    }
}

impl Styled for ResizablePanel {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for ResizablePanel {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        if !self.visible {
            return div()
                .id(("resizable-panel", self.index))
                .into_any_element();
        }

        let state = self
            .state
            .as_ref()
            .expect("ResizablePanel must be used within a ResizablePanelGroup");

        let panel_state = state.read(cx).panels.get(self.index).cloned();

        let size_range = self.size_range.clone();
        let has_custom_size = self.initial_size.is_some()
            || panel_state.as_ref().and_then(|p| p.size).is_some();

        let mut panel_div = div()
            .id(("resizable-panel", self.index))
            .flex()
            .flex_grow()
            .size_full()
            .relative();

        panel_div = panel_div.when(self.axis.is_vertical(), |this| {
            this.min_h(size_range.start).max_h(size_range.end)
        });

        panel_div = panel_div.when(self.axis.is_horizontal(), |this| {
            this.min_w(size_range.start).max_w(size_range.end)
        });

        panel_div = panel_div.when(!has_custom_size, |this| this.flex_shrink());

        if let Some(initial_size) = self.initial_size {
            let should_use_flex_none = panel_state
                .as_ref()
                .map(|p| p.size.is_none() && !initial_size.is_zero())
                .unwrap_or(false);

            panel_div = panel_div
                .when(should_use_flex_none, |this| this.flex_none())
                .flex_basis(initial_size);
        }

        if let Some(panel_state) = panel_state.as_ref() {
            if let Some(size) = panel_state.size {
                panel_div = panel_div.flex_basis(size);
            }
        }

        let user_style = self.style;

        panel_div
            .map(|this| {
                let mut div = this;
                div.style().refine(&user_style);
                div
            })
            .children(self.children)
            .when(self.index > 0, |this| {
                let handle_index = self.index - 1;
                let state = state.clone();

                this.child(ResizeHandle::new(
                    ("resizable-handle", handle_index),
                    self.axis,
                    DragPanel,
                    move |drag_panel, _, _, cx| {
                        cx.stop_propagation();
                        state.update(cx, |state, _| {
                            state.resizing_panel_ix = Some(handle_index);
                        });
                        cx.new(|_| (*drag_panel).clone())
                    },
                ))
            })
            .child({
                let state = state.clone();
                let index = self.index;
                let size_range = self.size_range.clone();

                canvas(
                    move |bounds, _, cx| {
                        state.update(cx, |state, cx| {
                            state.update_panel_size(index, bounds, size_range.clone(), cx)
                        })
                    },
                    |_, _, _, _| {},
                )
                .absolute()
                .size_full()
            })
            .into_any_element()
    }
}

#[derive(Clone)]
struct DragPanel;

impl Render for DragPanel {
    fn render(&mut self, _: &mut Window, _: &mut Context<'_, Self>) -> impl IntoElement {
        Empty
    }
}

struct ResizeHandle<T: 'static, E: 'static + Render> {
    id: ElementId,
    axis: Axis,
    drag_value: Rc<T>,
    on_drag: Rc<dyn Fn(Rc<T>, &Point<Pixels>, &mut Window, &mut App) -> Entity<E>>,
}

impl<T: 'static, E: 'static + Render> ResizeHandle<T, E> {
    fn new(
        id: impl Into<ElementId>,
        axis: Axis,
        value: T,
        f: impl Fn(Rc<T>, &Point<Pixels>, &mut Window, &mut App) -> Entity<E> + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            axis,
            drag_value: Rc::new(value),
            on_drag: Rc::new(f),
        }
    }
}

#[derive(Default, Debug, Clone)]
struct ResizeHandleState {
    active: Cell<bool>,
}

impl ResizeHandleState {
    fn set_active(&self, active: bool) {
        self.active.set(active);
    }

    fn is_active(&self) -> bool {
        self.active.get()
    }
}

impl<T: 'static, E: 'static + Render> IntoElement for ResizeHandle<T, E> {
    type Element = ResizeHandle<T, E>;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl<T: 'static, E: 'static + Render> Element for ResizeHandle<T, E> {
    type RequestLayoutState = AnyElement;
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        Some(self.id.clone())
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        id: Option<&GlobalElementId>,
        _: Option<&gpui::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (gpui::LayoutId, Self::RequestLayoutState) {
        let neg_offset = -HANDLE_PADDING;
        let axis = self.axis;
        let theme = use_theme();

        window.with_element_state(id.unwrap(), |state, window| {
            let state = state.unwrap_or_else(ResizeHandleState::default);

            let bg_color = if state.is_active() {
                theme.tokens.accent
            } else {
                theme.tokens.border
            };

            let mut handle_element = div()
                .id(self.id.clone())
                .occlude()
                .absolute()
                .flex_shrink_0()
                .group("handle");

            let on_drag = self.on_drag.clone();
            let drag_value = self.drag_value.clone();
            handle_element = handle_element.on_drag(drag_value.clone(), move |_, position, window, cx| {
                (on_drag)(drag_value.clone(), &position, window, cx)
            });

            handle_element = match axis {
                Axis::Horizontal => handle_element
                    .cursor_col_resize()
                    .top_0()
                    .left(neg_offset)
                    .h_full()
                    .w(HANDLE_SIZE)
                    .px(HANDLE_PADDING),
                Axis::Vertical => handle_element
                    .cursor_row_resize()
                    .top(neg_offset)
                    .left_0()
                    .w_full()
                    .h(HANDLE_SIZE)
                    .py(HANDLE_PADDING),
            };

            handle_element = handle_element.child(
                div()
                    .bg(bg_color)
                    .group_hover("handle", |this| this.bg(theme.tokens.accent))
                    .when(axis.is_horizontal(), |this| this.h_full().w(HANDLE_SIZE))
                    .when(axis.is_vertical(), |this| this.w_full().h(HANDLE_SIZE)),
            );

            let mut el = handle_element.into_any_element();
            let layout_id = el.request_layout(window, cx);

            ((layout_id, el), state)
        })
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&gpui::InspectorElementId>,
        _: gpui::Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        request_layout.prepaint(window, cx);
    }

    fn paint(
        &mut self,
        id: Option<&GlobalElementId>,
        _: Option<&gpui::InspectorElementId>,
        bounds: gpui::Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        _: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        request_layout.paint(window, cx);

        window.with_element_state(id.unwrap(), |state: Option<ResizeHandleState>, window| {
            let state = state.unwrap_or_else(ResizeHandleState::default);

            window.on_mouse_event({
                let state = state.clone();
                move |event: &MouseDownEvent, phase, window, _| {
                    if bounds.contains(&event.position) && phase.bubble() {
                        state.set_active(true);
                        window.refresh();
                    }
                }
            });

            window.on_mouse_event({
                let state = state.clone();
                move |_: &MouseUpEvent, _, window, _| {
                    if state.is_active() {
                        state.set_active(false);
                        window.refresh();
                    }
                }
            });

            ((), state)
        });
    }
}

struct ResizePanelGroupElement {
    state: Entity<ResizableState>,
    axis: Axis,
}

impl IntoElement for ResizePanelGroupElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for ResizePanelGroupElement {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<gpui::ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _: Option<&gpui::GlobalElementId>,
        _: Option<&gpui::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (gpui::LayoutId, Self::RequestLayoutState) {
        (window.request_layout(Style::default(), None, cx), ())
    }

    fn prepaint(
        &mut self,
        _: Option<&gpui::GlobalElementId>,
        _: Option<&gpui::InspectorElementId>,
        _: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        _: &mut Window,
        _: &mut App,
    ) -> Self::PrepaintState {
    }

    fn paint(
        &mut self,
        _: Option<&gpui::GlobalElementId>,
        _: Option<&gpui::InspectorElementId>,
        _: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        _: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        window.on_mouse_event({
            let state = self.state.clone();
            let axis = self.axis;
            let current_ix = state.read(cx).resizing_panel_ix;

            move |event: &MouseMoveEvent, phase, window, cx| {
                if !phase.bubble() {
                    return;
                }

                let Some(panel_index) = current_ix else {
                    return;
                };

                state.update(cx, |state, cx| {
                    if let Some(panel) = state.panels.get(panel_index) {
                        let new_size = match axis {
                            Axis::Horizontal => event.position.x - panel.bounds.left(),
                            Axis::Vertical => event.position.y - panel.bounds.top(),
                        };

                        state.resize_panel(panel_index, new_size, window, cx);
                    }

                    cx.notify();
                });
            }
        });

        window.on_mouse_event({
            let state = self.state.clone();
            let current_ix = state.read(cx).resizing_panel_ix;

            move |_: &MouseUpEvent, phase, _, cx| {
                if current_ix.is_none() {
                    return;
                }

                if phase.bubble() {
                    state.update(cx, |state, cx| {
                        state.done_resizing(cx);
                    });
                }
            }
        });
    }
}
