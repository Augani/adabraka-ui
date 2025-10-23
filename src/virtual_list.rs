use std::cell::RefCell;
use std::cmp;
use std::collections::HashMap;
use std::ops::Range;
use std::rc::Rc;

use gpui::{
    Along,
    InteractiveElement,
    StatefulInteractiveElement,
    div, point, px, size, AnyElement, App, AvailableSpace, Axis, Bounds, Context, Div, Element,
    ElementId, Entity, GlobalElementId, Hitbox, IntoElement, ListSizingBehavior, Pixels, Size,
    Stateful, Styled, StyleRefinement, Window, Render,
};
use smallvec::SmallVec;
use crate::util::{AxisExt, PixelsExt};

pub struct UniformVirtualList {
    id: ElementId,
    axis: Axis,
    item_count: usize,
    item_extent: Pixels,
    overscan: usize,
    base: Stateful<Div>,
    scroll_handle: gpui::ScrollHandle,
    sizing_behavior: ListSizingBehavior,
    renderer: Box<
        dyn for<'a> Fn(Range<usize>, &'a mut Window, &'a mut App) -> SmallVec<[AnyElement; 64]>,
    >,
    on_visible_range: Option<Box<dyn Fn(Range<usize>, &mut Window, &mut App)>>,
    near_end_threshold: Option<(f32, Rc<RefCell<bool>>, Box<dyn Fn(&mut Window, &mut App)>)>,
}

impl Styled for UniformVirtualList {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl UniformVirtualList {
    pub fn new<R: IntoElement + 'static>(
        id: impl Into<ElementId>,
        axis: Axis,
        item_count: usize,
        item_extent: Pixels,
        renderer: impl 'static + Fn(Range<usize>, &mut Window, &mut App) -> Vec<R>,
    ) -> Self {
        let id = id.into();
        let renderer_boxed = move |range: Range<usize>, window: &mut Window, cx: &mut App| {
            renderer(range, window, cx)
                .into_iter()
                .map(|r| r.into_any_element())
                .collect::<SmallVec<[AnyElement; 64]>>()
        };

        Self {
            id: id.clone(),
            axis,
            item_count,
            item_extent,
            overscan: 5,
            base: div().id(id).size_full().overflow_scroll(),
            scroll_handle: gpui::ScrollHandle::new(),
            sizing_behavior: ListSizingBehavior::Auto,
            renderer: Box::new(renderer_boxed),
            on_visible_range: None,
            near_end_threshold: None,
        }
    }

    pub fn overscan(mut self, items: usize) -> Self {
        self.overscan = items;
        self
    }

    pub fn track_scroll(mut self, handle: &gpui::ScrollHandle) -> Self {
        self.base = self.base.track_scroll(handle);
        self.scroll_handle = handle.clone();
        self
    }

    pub fn with_sizing_behavior(mut self, behavior: ListSizingBehavior) -> Self {
        self.sizing_behavior = behavior;
        self
    }

    pub fn on_visible_range(
        mut self,
        f: impl 'static + Fn(Range<usize>, &mut Window, &mut App),
    ) -> Self {
        self.on_visible_range = Some(Box::new(f));
        self
    }

    pub fn on_near_end(
        mut self,
        threshold: f32,
        f: impl 'static + Fn(&mut Window, &mut App),
    ) -> Self {
        self.near_end_threshold = Some((threshold.clamp(0.0, 1.0), Rc::new(RefCell::new(false)), Box::new(f)));
        self
    }

    pub fn scroll_to(&self, index: usize, strategy: gpui::ScrollStrategy) {
        let index = index.min(self.item_count.saturating_sub(1));
        let extent = self.item_extent;
        let target_origin = extent * index as f32;
        let mut offset = self.scroll_handle.offset();
        match strategy {
            gpui::ScrollStrategy::Top => {
                if self.axis.is_vertical() {
                    offset.y = -target_origin;
                } else {
                    offset.x = -target_origin;
                }
            }
            gpui::ScrollStrategy::Center => {
                if self.axis.is_vertical() {
                    let viewport: f32 = (-self.scroll_handle.offset().y).into();
                    let _ = viewport; // not available: need viewport size at call site; fall back to Top
                    offset.y = -target_origin;
                } else {
                    offset.x = -target_origin;
                }
            }
            _ => {
                if self.axis.is_vertical() {
                    offset.y = -target_origin;
                } else {
                    offset.x = -target_origin;
                }
            }
        }
        self.scroll_handle.set_offset(offset);
    }
}

pub struct UniformFrameState {
    items: SmallVec<[AnyElement; 32]>,
}

impl IntoElement for UniformVirtualList {
    type Element = Self;
    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for UniformVirtualList {
    type RequestLayoutState = UniformFrameState;
    type PrepaintState = Option<Hitbox>;

    fn id(&self) -> Option<ElementId> {
        Some(self.id.clone())
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        global_id: Option<&GlobalElementId>,
        inspector_id: Option<&gpui::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (gpui::LayoutId, Self::RequestLayoutState) {
        let axis = self.axis;
        let item_count = self.item_count;
        let item_extent = self.item_extent;
        let behavior = self.sizing_behavior;

        let layout_id = self.base.interactivity().request_layout(
            global_id,
            inspector_id,
            window,
            cx,
            move |style, window: &mut Window, cx: &mut App| match behavior {
                ListSizingBehavior::Infer => window.request_measured_layout(style, move |_k, available, _, _| {
                    let mut sz = Size::default();
                    if axis.is_horizontal() {
                        sz.width = match available.width {
                            AvailableSpace::Definite(w) => w,
                            _ => px(item_count as f32 * item_extent.as_f32()),
                        };
                        sz.height = match available.height {
                            AvailableSpace::Definite(h) => h,
                            _ => px(0.),
                        };
                    } else {
                        sz.width = match available.width {
                            AvailableSpace::Definite(w) => w,
                            _ => px(0.),
                        };
                        sz.height = match available.height {
                            AvailableSpace::Definite(h) => h,
                            _ => px(item_count as f32 * item_extent.as_f32()),
                        };
                    }
                    sz
                }),
                ListSizingBehavior::Auto => window.request_layout(style, None, cx),
            },
        );

        (layout_id, UniformFrameState { items: SmallVec::new() })
    }

    fn prepaint(
        &mut self,
        global_id: Option<&GlobalElementId>,
        inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        let style = self.base.interactivity().compute_style(global_id, None, window, cx);
        let border = style.border_widths.to_pixels(window.rem_size());
        let padding = style.padding.to_pixels(bounds.size.into(), window.rem_size());

        let content_bounds = Bounds::from_corners(
            bounds.origin + point(border.left + padding.left, border.top + padding.top),
            bounds.bottom_right()
                - point(border.right + padding.right, border.bottom + padding.bottom),
        );

        let offset = self.scroll_handle.offset();
        let viewport_len = content_bounds.size.along(self.axis);
        let extent = self.item_extent;

        let base = -offset.along(self.axis);
        let first = if extent.as_f32() > 0.0 {
            (base.as_f32() / extent.as_f32()).floor().max(0.0) as usize
        } else {
            0
        };
        let last = if extent.as_f32() > 0.0 {
            ((base + viewport_len).as_f32() / extent.as_f32()).ceil().max(0.0) as usize
        } else {
            0
        };

        let start = first.saturating_sub(self.overscan);
        let mut end = cmp::min(last + self.overscan, self.item_count);
        if end == 0 { end = cmp::min(self.item_count, self.overscan); }

        let visible = start..end;

        if let Some(cb) = &self.on_visible_range {
            cb(visible.clone(), window, cx);
        }

        if let Some((threshold, fired, cb)) = &self.near_end_threshold {
            let progress = if self.item_count == 0 { 0.0 } else { visible.end as f32 / self.item_count as f32 };
            let mut was_fired = fired.borrow_mut();
            if progress >= *threshold && !*was_fired {
                *was_fired = true;
                cb(window, cx);
            }
            if progress < *threshold {
                *was_fired = false;
            }
        }

        let items = (self.renderer)(visible.clone(), window, cx);

        self.base.interactivity().prepaint(
            global_id,
            inspector_id,
            bounds,
            Size {
                width: if self.axis.is_horizontal() {
                    px(self.item_count as f32 * extent.as_f32())
                } else {
                    content_bounds.size.width
                },
                height: if self.axis.is_vertical() {
                    px(self.item_count as f32 * extent.as_f32())
                } else {
                    content_bounds.size.height
                },
            },
            window,
            cx,
            |_style, _, hitbox, window, cx| {
                let available = match self.axis {
                    Axis::Horizontal => size(AvailableSpace::Definite(extent), AvailableSpace::Definite(content_bounds.size.height)),
                    Axis::Vertical => size(AvailableSpace::Definite(content_bounds.size.width), AvailableSpace::Definite(extent)),
                };

                for (mut item, ix) in items.into_iter().zip(visible) {
                    let item_origin = match self.axis {
                        Axis::Horizontal => content_bounds.origin + point(px(ix as f32 * extent.as_f32()) + offset.x, offset.y),
                        Axis::Vertical => content_bounds.origin + point(offset.x, px(ix as f32 * extent.as_f32()) + offset.y),
                    };
                    item.layout_as_root(available, window, cx);
                    item.prepaint_at(item_origin, window, cx);
                    layout.items.push(item);
                }

                hitbox
            },
        )
    }

    fn paint(
        &mut self,
        global_id: Option<&GlobalElementId>,
        inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        layout: &mut Self::RequestLayoutState,
        hitbox: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        self.base.interactivity().paint(
            global_id,
            inspector_id,
            bounds,
            hitbox.as_ref(),
            window,
            cx,
            |_, window, cx| {
                for item in &mut layout.items {
                    item.paint(window, cx);
                }
            },
        )
    }
}

pub trait ItemExtentProvider {
    fn extent(&self, index: usize) -> Pixels;
}

const CHUNK_SIZE: usize = 1024;

struct ChunkedExtents<P: ItemExtentProvider> {
    provider: P,
    item_count: usize,
    chunk_totals: Vec<Pixels>,
    chunk_offsets: Vec<Pixels>,
    intra_prefix: HashMap<usize, Rc<Vec<Pixels>>>,
}

impl<P: ItemExtentProvider> ChunkedExtents<P> {
    fn new(provider: P, item_count: usize) -> Self {
        let chunk_count = (item_count + CHUNK_SIZE - 1) / CHUNK_SIZE;
        Self {
            provider,
            item_count,
            chunk_totals: vec![px(0.0); chunk_count],
            chunk_offsets: vec![px(0.0); chunk_count + 1],
            intra_prefix: HashMap::new(),
        }
    }

    fn initialize_totals(&mut self) {
        if self.item_count == 0 { return; }
        let chunk_count = self.chunk_totals.len();
        for c in 0..chunk_count {
            let start = c * CHUNK_SIZE;
            let end = ((c + 1) * CHUNK_SIZE).min(self.item_count);
            let mut sum = 0.0;
            for i in start..end {
                sum += self.provider.extent(i).as_f32();
            }
            self.chunk_totals[c] = px(sum);
        }
        let mut accum = 0.0;
        self.chunk_offsets[0] = px(0.0);
        for c in 0..chunk_count {
            accum += self.chunk_totals[c].as_f32();
            self.chunk_offsets[c + 1] = px(accum);
        }
    }

    fn total_extent(&self) -> Pixels {
        if self.chunk_offsets.is_empty() { return px(0.0); }
        *self.chunk_offsets.last().unwrap()
    }

    fn ensure_intra_prefix(&mut self, chunk_index: usize) -> Rc<Vec<Pixels>> {
        if let Some(v) = self.intra_prefix.get(&chunk_index) { return v.clone(); }
        let start = chunk_index * CHUNK_SIZE;
        let end = ((chunk_index + 1) * CHUNK_SIZE).min(self.item_count);
        let mut origins = Vec::with_capacity(end - start);
        let mut sum = 0.0;
        for i in start..end {
            origins.push(px(sum));
            sum += self.provider.extent(i).as_f32();
        }
        let rc = Rc::new(origins);
        self.intra_prefix.insert(chunk_index, rc.clone());
        rc
    }

    fn find_index_for_offset(&mut self, offset: Pixels) -> usize {
        if self.item_count == 0 { return 0; }
        let target = offset.as_f32();
        let mut lo = 0usize;
        let mut hi = self.chunk_offsets.len() - 1;
        while lo < hi {
            let mid = (lo + hi) / 2;
            if self.chunk_offsets[mid].as_f32() <= target {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        let chunk = lo.saturating_sub(1).min(self.chunk_totals.len().saturating_sub(1));
        let chunk_base = self.chunk_offsets[chunk].as_f32();
        let within = target - chunk_base;
        let intra = self.ensure_intra_prefix(chunk);
        let mut lo_i = 0usize;
        let mut hi_i = intra.len();
        while lo_i < hi_i {
            let mid = (lo_i + hi_i) / 2;
            if intra[mid].as_f32() <= within {
                lo_i = mid + 1;
            } else {
                hi_i = mid;
            }
        }
        let idx_in_chunk = lo_i.saturating_sub(1).min(intra.len().saturating_sub(1));
        (chunk * CHUNK_SIZE + idx_in_chunk).min(self.item_count.saturating_sub(1))
    }

    fn item_origin(&mut self, index: usize) -> Pixels {
        if self.item_count == 0 { return px(0.0); }
        let chunk = index / CHUNK_SIZE;
        let intra_ix = index % CHUNK_SIZE;
        let intra = self.ensure_intra_prefix(chunk);
        self.chunk_offsets[chunk] + intra[intra_ix]
    }
}

pub struct VariableVirtualList<P: ItemExtentProvider> {
    id: ElementId,
    axis: Axis,
    overscan: usize,
    base: Stateful<Div>,
    scroll_handle: gpui::ScrollHandle,
    sizing_behavior: ListSizingBehavior,
    engine: ChunkedExtents<P>,
    renderer: Box<
        dyn for<'a> Fn(Range<usize>, &'a mut Window, &'a mut App) -> SmallVec<[AnyElement; 64]>,
    >,
    on_visible_range: Option<Box<dyn Fn(Range<usize>, &mut Window, &mut App)>>,
    near_end_threshold: Option<(f32, Rc<RefCell<bool>>, Box<dyn Fn(&mut Window, &mut App)>)>,
}

impl<P: ItemExtentProvider> Styled for VariableVirtualList<P> {
    fn style(&mut self) -> &mut StyleRefinement { self.base.style() }
}

impl<P: ItemExtentProvider + 'static> VariableVirtualList<P> {
    pub fn new<R: IntoElement + 'static>(
        id: impl Into<ElementId>,
        axis: Axis,
        item_count: usize,
        provider: P,
        renderer: impl 'static + Fn(Range<usize>, &mut Window, &mut App) -> Vec<R>,
    ) -> Self {
        let id = id.into();
        let renderer_boxed = move |range: Range<usize>, window: &mut Window, cx: &mut App| {
            renderer(range, window, cx)
                .into_iter()
                .map(|r| r.into_any_element())
                .collect::<SmallVec<[AnyElement; 64]>>()
        };

        let mut engine = ChunkedExtents::new(provider, item_count);
        engine.initialize_totals();

        Self {
            id: id.clone(),
            axis,
            overscan: 5,
            base: div().id(id).size_full().overflow_scroll(),
            scroll_handle: gpui::ScrollHandle::new(),
            sizing_behavior: ListSizingBehavior::Auto,
            engine,
            renderer: Box::new(renderer_boxed),
            on_visible_range: None,
            near_end_threshold: None,
        }
    }

    pub fn overscan(mut self, items: usize) -> Self { self.overscan = items; self }
    pub fn track_scroll(mut self, handle: &gpui::ScrollHandle) -> Self { self.base = self.base.track_scroll(handle); self.scroll_handle = handle.clone(); self }
    pub fn with_sizing_behavior(mut self, behavior: ListSizingBehavior) -> Self { self.sizing_behavior = behavior; self }

    pub fn on_visible_range(mut self, f: impl 'static + Fn(Range<usize>, &mut Window, &mut App)) -> Self {
        self.on_visible_range = Some(Box::new(f));
        self
    }

    pub fn on_near_end(mut self, threshold: f32, f: impl 'static + Fn(&mut Window, &mut App)) -> Self {
        self.near_end_threshold = Some((threshold.clamp(0.0, 1.0), Rc::new(RefCell::new(false)), Box::new(f)));
        self
    }

    pub fn scroll_to(&mut self, index: usize, strategy: gpui::ScrollStrategy) {
        let index = index.min(self.engine.item_count.saturating_sub(1));
        let target = self.engine.item_origin(index);
        let mut offset = self.scroll_handle.offset();
        match strategy {
            gpui::ScrollStrategy::Top => {
                if self.axis.is_vertical() { offset.y = -target; } else { offset.x = -target; }
            }
            _ => { if self.axis.is_vertical() { offset.y = -target; } else { offset.x = -target; } }
        }
        self.scroll_handle.set_offset(offset);
    }
}

pub struct VariableFrameState { items: SmallVec<[AnyElement; 32]> }

impl<P: ItemExtentProvider + 'static> IntoElement for VariableVirtualList<P> { type Element = Self; fn into_element(self) -> Self::Element { self } }

impl<P: ItemExtentProvider + 'static> Element for VariableVirtualList<P> {
    type RequestLayoutState = VariableFrameState;
    type PrepaintState = Option<Hitbox>;

    fn id(&self) -> Option<ElementId> { Some(self.id.clone()) }
    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> { None }

    fn request_layout(
        &mut self,
        global_id: Option<&GlobalElementId>,
        inspector_id: Option<&gpui::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (gpui::LayoutId, Self::RequestLayoutState) {
        let axis = self.axis;
        let behavior = self.sizing_behavior;
        let engine_total = self.engine.total_extent();

        let layout_id = self.base.interactivity().request_layout(
            global_id,
            inspector_id,
            window,
            cx,
            move |style, window: &mut Window, cx: &mut App| match behavior {
                ListSizingBehavior::Infer => window.request_measured_layout(style, move |_k, available, _, _| {
                    let mut sz = Size::default();
                    if axis.is_horizontal() {
                        sz.width = match available.width { AvailableSpace::Definite(w) => w, _ => engine_total };
                        sz.height = match available.height { AvailableSpace::Definite(h) => h, _ => px(0.) };
                    } else {
                        sz.width = match available.width { AvailableSpace::Definite(w) => w, _ => px(0.) };
                        sz.height = match available.height { AvailableSpace::Definite(h) => h, _ => engine_total };
                    }
                    sz
                }),
                ListSizingBehavior::Auto => window.request_layout(style, None, cx),
            },
        );
        (layout_id, VariableFrameState { items: SmallVec::new() })
    }

    fn prepaint(
        &mut self,
        global_id: Option<&GlobalElementId>,
        inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        let style = self.base.interactivity().compute_style(global_id, None, window, cx);
        let border = style.border_widths.to_pixels(window.rem_size());
        let padding = style.padding.to_pixels(bounds.size.into(), window.rem_size());

        let content_bounds = Bounds::from_corners(
            bounds.origin + point(border.left + padding.left, border.top + padding.top),
            bounds.bottom_right() - point(border.right + padding.right, border.bottom + padding.bottom),
        );

        let mut offset = self.scroll_handle.offset();
        let viewport_len = content_bounds.size.along(self.axis);

        let total = self.engine.total_extent();
        let min_scroll_offset = viewport_len - total;
        if min_scroll_offset.as_f32() >= 0.0 { offset.x = px(0.); offset.y = px(0.); }

        if self.axis.is_vertical() {
            if offset.y < min_scroll_offset { offset.y = min_scroll_offset; }
        } else if offset.x < min_scroll_offset { offset.x = min_scroll_offset; }

        let start_px = -offset.along(self.axis);
        let end_px = start_px + viewport_len;

        let mut start_ix = self.engine.find_index_for_offset(start_px.max(px(0.)));
        start_ix = start_ix.saturating_sub(self.overscan);

        let mut end_ix = self.engine.find_index_for_offset(end_px.max(px(0.)));
        end_ix = (end_ix + 1 + self.overscan).min(self.engine.item_count);

        let visible = start_ix..end_ix;

        if let Some(cb) = &self.on_visible_range { cb(visible.clone(), window, cx); }
        if let Some((threshold, fired, cb)) = &self.near_end_threshold {
            let progress = if self.engine.item_count == 0 { 0.0 } else { visible.end as f32 / self.engine.item_count as f32 };
            let mut was_fired = fired.borrow_mut();
            if progress >= *threshold && !*was_fired { *was_fired = true; cb(window, cx); }
            if progress < *threshold { *was_fired = false; }
        }

        let items = (self.renderer)(visible.clone(), window, cx);

        let content_size = if self.axis.is_horizontal() {
            Size { width: total, height: content_bounds.size.height }
        } else {
            Size { width: content_bounds.size.width, height: total }
        };

        self.base.interactivity().prepaint(
            global_id,
            inspector_id,
            bounds,
            content_size,
            window,
            cx,
            |_style, _, hitbox, window, cx| {
                for (mut item, ix) in items.into_iter().zip(visible) {
                    let origin_along = self.engine.item_origin(ix);
                    let item_origin = match self.axis {
                        Axis::Horizontal => content_bounds.origin + point(origin_along + offset.x, offset.y),
                        Axis::Vertical => content_bounds.origin + point(offset.x, origin_along + offset.y),
                    };

                    let available = match self.axis {
                        Axis::Horizontal => size(AvailableSpace::Definite(px(CHUNK_SIZE as f32)), AvailableSpace::Definite(content_bounds.size.height)),
                        Axis::Vertical => size(AvailableSpace::Definite(content_bounds.size.width), AvailableSpace::Definite(px(CHUNK_SIZE as f32))),
                    };

                    item.layout_as_root(available, window, cx);
                    item.prepaint_at(item_origin, window, cx);
                    layout.items.push(item);
                }
                hitbox
            },
        )
    }

    fn paint(
        &mut self,
        global_id: Option<&GlobalElementId>,
        inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        layout: &mut Self::RequestLayoutState,
        hitbox: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        self.base.interactivity().paint(
            global_id,
            inspector_id,
            bounds,
            hitbox.as_ref(),
            window,
            cx,
            |_, window, cx| {
                for item in &mut layout.items { item.paint(window, cx); }
            },
        )
    }
}

pub fn vlist_uniform<R: IntoElement + 'static>(
    id: impl Into<ElementId>,
    item_count: usize,
    item_extent: Pixels,
    renderer: impl 'static + Fn(Range<usize>, &mut Window, &mut App) -> Vec<R>,
) -> UniformVirtualList {
    UniformVirtualList::new(id, Axis::Vertical, item_count, item_extent, renderer)
}

pub fn hlist_uniform<R: IntoElement + 'static>(
    id: impl Into<ElementId>,
    item_count: usize,
    item_extent: Pixels,
    renderer: impl 'static + Fn(Range<usize>, &mut Window, &mut App) -> Vec<R>,
) -> UniformVirtualList {
    UniformVirtualList::new(id, Axis::Horizontal, item_count, item_extent, renderer)
}

pub fn vlist_variable<R: IntoElement + 'static, P: ItemExtentProvider + 'static>(
    id: impl Into<ElementId>,
    item_count: usize,
    provider: P,
    renderer: impl 'static + Fn(Range<usize>, &mut Window, &mut App) -> Vec<R>,
) -> VariableVirtualList<P> {
    VariableVirtualList::new(id, Axis::Vertical, item_count, provider, renderer)
}

pub fn hlist_variable<R: IntoElement + 'static, P: ItemExtentProvider + 'static>(
    id: impl Into<ElementId>,
    item_count: usize,
    provider: P,
    renderer: impl 'static + Fn(Range<usize>, &mut Window, &mut App) -> Vec<R>,
) -> VariableVirtualList<P> {
    VariableVirtualList::new(id, Axis::Horizontal, item_count, provider, renderer)
}

pub fn vlist_uniform_view<R, V>(
    view: Entity<V>,
    id: impl Into<ElementId>,
    item_count: usize,
    item_extent: Pixels,
    f: impl 'static + Fn(&mut V, Range<usize>, &mut Window, &mut Context<V>) -> Vec<R>,
) -> UniformVirtualList
where
    R: IntoElement,
    V: Render,
{
    let id: ElementId = id.into();
    let render_range = move |visible_range: Range<usize>, window: &mut Window, cx: &mut App| {
        view.update(cx, |this, cx| {
            f(this, visible_range, window, cx)
                .into_iter()
                .map(|component| component.into_any_element())
                .collect::<SmallVec<[AnyElement; 64]>>()
        })
    };

    UniformVirtualList::new(id, Axis::Vertical, item_count, item_extent, move |range, window, cx| {
        render_range(range, window, cx).into_iter().collect::<Vec<_>>()
    })
}


