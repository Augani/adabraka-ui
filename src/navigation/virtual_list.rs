//! Virtual list component for efficient rendering of large item counts.

use std::{
    cmp,
    ops::Range,
    rc::Rc,
};

use gpui::{
    div, point, px, size, Along, AnyElement, App, AvailableSpace, Axis, Bounds, ContentMask,
    Context, Div, Element, ElementId, Entity, GlobalElementId, Hitbox,
    InteractiveElement, IntoElement, Pixels, Render,
    ScrollHandle, Size, Stateful, StatefulInteractiveElement, Styled, Window,
};
use smallvec::SmallVec;

use crate::util::{AxisExt, PixelsExt};

pub struct VirtualListFrameState {
    items: SmallVec<[AnyElement; 32]>,
    size_layout: ItemSizeLayout,
}

#[derive(Default, Clone)]
pub struct ItemSizeLayout {
    items_sizes: Rc<Vec<Size<Pixels>>>,
    content_size: Size<Pixels>,
    sizes: Vec<Pixels>,
    origins: Vec<Pixels>,
}

#[inline]
pub fn v_virtual_list<R, V>(
    view: Entity<V>,
    id: impl Into<ElementId>,
    item_sizes: Rc<Vec<Size<Pixels>>>,
    f: impl 'static + Fn(&mut V, Range<usize>, &mut Window, &mut Context<V>) -> Vec<R>,
) -> VirtualList
where
    R: IntoElement,
    V: Render,
{
    virtual_list(view, id, Axis::Vertical, item_sizes, f)
}

fn virtual_list<R, V>(
    view: Entity<V>,
    id: impl Into<ElementId>,
    axis: Axis,
    item_sizes: Rc<Vec<Size<Pixels>>>,
    f: impl 'static + Fn(&mut V, Range<usize>, &mut Window, &mut Context<V>) -> Vec<R>,
) -> VirtualList
where
    R: IntoElement,
    V: Render,
{
    let id: ElementId = id.into();
    let scroll_handle = ScrollHandle::default();
    let render_range = move |visible_range, window: &mut Window, cx: &mut App| {
        view.update(cx, |this, cx| {
            f(this, visible_range, window, cx)
                .into_iter()
                .map(|component| component.into_any_element())
                .collect()
        })
    };

    VirtualList {
        id: id.clone(),
        axis,
        base: div()
            .id(id)
            .size_full()
            .overflow_scroll()
            .track_scroll(&scroll_handle),
        scroll_handle,
        items_count: item_sizes.len(),
        item_sizes,
        render_items: Box::new(render_range),
    }
}

pub struct VirtualList {
    id: ElementId,
    axis: Axis,
    base: Stateful<Div>,
    scroll_handle: ScrollHandle,
    items_count: usize,
    item_sizes: Rc<Vec<Size<Pixels>>>,
    render_items: Box<
        dyn for<'a> Fn(Range<usize>, &'a mut Window, &'a mut App) -> SmallVec<[AnyElement; 64]>,
    >,
}

impl Styled for VirtualList {
    fn style(&mut self) -> &mut gpui::StyleRefinement {
        self.base.style()
    }
}

impl VirtualList {
    pub fn track_scroll(mut self, scroll_handle: &ScrollHandle) -> Self {
        self.base = self.base.track_scroll(scroll_handle);
        self.scroll_handle = scroll_handle.clone();
        self
    }
}

impl IntoElement for VirtualList {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for VirtualList {
    type RequestLayoutState = VirtualListFrameState;
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
        let rem_size = window.rem_size();
        let font_size = window.text_style().font_size.to_pixels(rem_size);
        let mut size_layout = ItemSizeLayout::default();

        let layout_id = self.base.interactivity().request_layout(
            global_id,
            inspector_id,
            window,
            cx,
            |style, window, cx| {
                size_layout = window.with_element_state(
                    global_id.unwrap(),
                    |state: Option<ItemSizeLayout>, _window| {
                        let mut state = state.unwrap_or(ItemSizeLayout::default());

                        let gap = style
                            .gap
                            .along(self.axis)
                            .to_pixels(font_size.into(), rem_size);

                        if state.items_sizes != self.item_sizes {
                            state.items_sizes = self.item_sizes.clone();
                            state.sizes = self
                                .item_sizes
                                .iter()
                                .enumerate()
                                .map(|(i, size)| {
                                    let size = size.along(self.axis);
                                    if i + 1 == self.items_count {
                                        size
                                    } else {
                                        size + gap
                                    }
                                })
                                .collect::<Vec<_>>();

                            state.origins = state
                                .sizes
                                .iter()
                                .scan(px(0.), |cumulative, size| match self.axis {
                                    Axis::Horizontal => {
                                        let x = *cumulative;
                                        *cumulative += *size;
                                        Some(x)
                                    }
                                    Axis::Vertical => {
                                        let y = *cumulative;
                                        *cumulative += *size;
                                        Some(y)
                                    }
                                })
                                .collect::<Vec<_>>();

                            state.content_size = if self.axis.is_horizontal() {
                                Size {
                                    width: px(state
                                        .sizes
                                        .iter()
                                        .map(|size| size.as_f32())
                                        .sum::<f32>()),
                                    height: state
                                        .items_sizes
                                        .get(0)
                                        .map_or(px(0.), |size| size.height),
                                }
                            } else {
                                Size {
                                    width: state
                                        .items_sizes
                                        .get(0)
                                        .map_or(px(0.), |size| size.width),
                                    height: px(state
                                        .sizes
                                        .iter()
                                        .map(|size| size.as_f32())
                                        .sum::<f32>()),
                                }
                            };
                        }

                        (state.clone(), state)
                    },
                );

                window
                    .with_text_style(style.text_style().cloned(), |window| {
                        window.request_layout(style, None, cx)
                    })
            },
        );

        (
            layout_id,
            VirtualListFrameState {
                items: SmallVec::new(),
                size_layout,
            },
        )
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
        let style = self
            .base
            .interactivity()
            .compute_style(global_id, None, window, cx);
        let border_widths = style.border_widths.to_pixels(window.rem_size());
        let paddings = style
            .padding
            .to_pixels(bounds.size.into(), window.rem_size());

        let item_sizes = &layout.size_layout.sizes;
        let item_origins = &layout.size_layout.origins;

        let content_bounds = Bounds::from_corners(
            bounds.origin
                + point(
                    border_widths.left + paddings.left,
                    border_widths.top + paddings.top,
                ),
            bounds.bottom_right()
                - point(
                    border_widths.right + paddings.right,
                    border_widths.bottom + paddings.bottom,
                ),
        );

        let scroll_offset = self.scroll_handle.offset().min(&point(px(0.), px(0.)));

        self.base.interactivity().prepaint(
            global_id,
            inspector_id,
            bounds,
            layout.size_layout.content_size,
            window,
            cx,
            |_style, _, hitbox, window, cx| {
                if self.items_count > 0 {
                    let (first_visible_element_ix, last_visible_element_ix) = match self.axis {
                        Axis::Vertical => {
                            let mut cumulative_size = px(0.);
                            let mut first_visible_element_ix = 0;
                            for (i, &size) in item_sizes.iter().enumerate() {
                                cumulative_size += size;
                                if cumulative_size > -(scroll_offset.y + paddings.top) {
                                    first_visible_element_ix = i;
                                    break;
                                }
                            }

                            cumulative_size = px(0.);
                            let mut last_visible_element_ix = 0;
                            for (i, &size) in item_sizes.iter().enumerate() {
                                cumulative_size += size;
                                if cumulative_size > (-scroll_offset.y + content_bounds.size.height)
                                {
                                    last_visible_element_ix = i + 1;
                                    break;
                                }
                            }
                            if last_visible_element_ix == 0 {
                                last_visible_element_ix = self.items_count;
                            } else {
                                last_visible_element_ix += 1;
                            }
                            (first_visible_element_ix, last_visible_element_ix)
                        }
                        Axis::Horizontal => {
                            (0, self.items_count)
                        }
                    };

                    let visible_range = first_visible_element_ix
                        ..cmp::min(last_visible_element_ix, self.items_count);

                    let items = (self.render_items)(visible_range.clone(), window, cx);

                    let content_mask = ContentMask { bounds };
                    window.with_content_mask(Some(content_mask), |window| {
                        for (mut item, ix) in items.into_iter().zip(visible_range.clone()) {
                            let item_origin = match self.axis {
                                Axis::Horizontal => {
                                    content_bounds.origin
                                        + point(item_origins[ix] + scroll_offset.x, scroll_offset.y)
                                }
                                Axis::Vertical => {
                                    content_bounds.origin
                                        + point(scroll_offset.x, item_origins[ix] + scroll_offset.y)
                                }
                            };

                            let available_space = match self.axis {
                                Axis::Horizontal => size(
                                    AvailableSpace::Definite(item_sizes[ix]),
                                    AvailableSpace::Definite(content_bounds.size.height),
                                ),
                                Axis::Vertical => size(
                                    AvailableSpace::Definite(content_bounds.size.width),
                                    AvailableSpace::Definite(item_sizes[ix]),
                                ),
                            };

                            item.layout_as_root(available_space, window, cx);
                            item.prepaint_at(item_origin, window, cx);
                            layout.items.push(item);
                        }
                    });
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
