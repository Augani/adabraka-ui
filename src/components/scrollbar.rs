//! Scrollbar component - Scrollbar control for scrollable containers.

use std::{
    cell::Cell,
    rc::Rc,
    time::Instant,
};

use gpui::{
    point, px, relative, size, App, Axis, Bounds, ContentMask, Corner, CursorStyle,
    Element, GlobalElementId, Hitbox, HitboxBehavior, Hsla, InspectorElementId, IntoElement,
    LayoutId, MouseDownEvent, MouseMoveEvent, MouseUpEvent, Pixels, Point, Position,
    ScrollHandle, ScrollWheelEvent, Size, Style, Window, fill,
};

use crate::theme::use_theme;

pub(crate) const WIDTH: Pixels = px(12.0);
const MIN_THUMB_SIZE: f32 = 48.;

const THUMB_WIDTH: Pixels = px(6.);
const THUMB_RADIUS: Pixels = px(3.);
const THUMB_INSET: Pixels = px(3.);

const THUMB_ACTIVE_WIDTH: Pixels = px(8.);
const THUMB_ACTIVE_RADIUS: Pixels = px(4.);
const THUMB_ACTIVE_INSET: Pixels = px(2.);

const FADE_OUT_DURATION: f32 = 3.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollbarAxis {
    Vertical,
    Horizontal,
    Both,
}

impl ScrollbarAxis {
    pub fn is_vertical(&self) -> bool {
        matches!(self, Self::Vertical)
    }

    pub fn is_horizontal(&self) -> bool {
        matches!(self, Self::Horizontal)
    }

    pub fn is_both(&self) -> bool {
        matches!(self, Self::Both)
    }

    #[inline]
    pub fn has_vertical(&self) -> bool {
        matches!(self, Self::Vertical | Self::Both)
    }

    #[inline]
    pub fn has_horizontal(&self) -> bool {
        matches!(self, Self::Horizontal | Self::Both)
    }

    #[inline]
    fn all(&self) -> Vec<Axis> {
        match self {
            Self::Vertical => vec![Axis::Vertical],
            Self::Horizontal => vec![Axis::Horizontal],
            Self::Both => vec![Axis::Horizontal, Axis::Vertical],
        }
    }
}

#[derive(Debug, Clone)]
pub struct ScrollbarState(Rc<Cell<ScrollbarStateInner>>);

#[derive(Debug, Clone, Copy)]
pub struct ScrollbarStateInner {
    hovered_on_thumb: Option<Axis>,
    dragged_axis: Option<Axis>,
    drag_pos: Point<Pixels>,
    last_scroll_offset: Point<Pixels>,
    last_scroll_time: Option<Instant>,
    last_update: Instant,
}

impl Default for ScrollbarState {
    fn default() -> Self {
        Self(Rc::new(Cell::new(ScrollbarStateInner {
            hovered_on_thumb: None,
            dragged_axis: None,
            drag_pos: point(px(0.), px(0.)),
            last_scroll_offset: point(px(0.), px(0.)),
            last_scroll_time: None,
            last_update: Instant::now(),
        })))
    }
}

impl ScrollbarState {
    pub fn init_visible(&self) {
        let inner = self.0.get();
        self.0.set(inner.with_last_scroll(inner.last_scroll_offset, Some(Instant::now())));
    }
}

impl ScrollbarStateInner {
    fn with_drag_pos(&self, axis: Axis, pos: Point<Pixels>) -> Self {
        let mut state = *self;
        if axis == Axis::Vertical {
            state.drag_pos.y = pos.y;
        } else {
            state.drag_pos.x = pos.x;
        }
        state.dragged_axis = Some(axis);
        state
    }

    fn with_unset_drag_pos(&self) -> Self {
        let mut state = *self;
        state.dragged_axis = None;
        state
    }

    fn with_hovered_on_thumb(&self, axis: Option<Axis>) -> Self {
        let mut state = *self;
        state.hovered_on_thumb = axis;
        if axis.is_some() {
            state.last_scroll_time = Some(Instant::now());
        }
        state
    }

    fn with_last_scroll(&self, last_scroll_offset: Point<Pixels>, last_scroll_time: Option<Instant>) -> Self {
        let mut state = *self;
        state.last_scroll_offset = last_scroll_offset;
        state.last_scroll_time = last_scroll_time;
        state
    }

    fn with_last_update(&self, t: Instant) -> Self {
        let mut state = *self;
        state.last_update = t;
        state
    }

    fn is_scrollbar_visible(&self) -> bool {
        if self.dragged_axis.is_some() {
            return true;
        }

        if let Some(last_time) = self.last_scroll_time {
            let elapsed = Instant::now().duration_since(last_time).as_secs_f32();
            elapsed < FADE_OUT_DURATION
        } else {
            false
        }
    }
}

pub struct Scrollbar {
    axis: ScrollbarAxis,
    scroll_handle: ScrollHandle,
    state: ScrollbarState,
    scroll_size: Option<Size<Pixels>>,
    always_visible: bool,
    horizontal_at_top: bool,
}

impl Scrollbar {
    pub fn new(
        axis: ScrollbarAxis,
        state: &ScrollbarState,
        scroll_handle: &ScrollHandle,
    ) -> Self {
        Self {
            state: state.clone(),
            axis,
            scroll_handle: scroll_handle.clone(),
            scroll_size: None,
            always_visible: false,
            horizontal_at_top: false,
        }
    }

    pub fn vertical(state: &ScrollbarState, scroll_handle: &ScrollHandle) -> Self {
        Self::new(ScrollbarAxis::Vertical, state, scroll_handle)
    }

    pub fn horizontal(state: &ScrollbarState, scroll_handle: &ScrollHandle) -> Self {
        Self::new(ScrollbarAxis::Horizontal, state, scroll_handle)
    }

    pub fn both(state: &ScrollbarState, scroll_handle: &ScrollHandle) -> Self {
        Self::new(ScrollbarAxis::Both, state, scroll_handle)
    }

    pub fn always_visible(mut self) -> Self {
        self.always_visible = true;
        self
    }

    pub fn axis(mut self, axis: ScrollbarAxis) -> Self {
        self.axis = axis;
        self
    }

    pub fn horizontal_top(mut self) -> Self {
        self.horizontal_at_top = true;
        self
    }

    pub fn scroll_size(mut self, scroll_size: Size<Pixels>) -> Self {
        self.scroll_size = Some(scroll_size);
        self
    }

    fn get_thumb_color(&self, theme: &crate::theme::Theme) -> Hsla {
        theme.tokens.muted_foreground.opacity(0.6)
    }

    fn get_track_color(&self, theme: &crate::theme::Theme) -> Hsla {
        theme.tokens.muted.opacity(0.3)
    }

    fn get_hover_thumb_color(&self, theme: &crate::theme::Theme) -> Hsla {
        theme.tokens.muted_foreground.opacity(0.8)
    }
}

impl IntoElement for Scrollbar {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

pub struct PrepaintState {
    hitbox: Hitbox,
    states: Vec<AxisPrepaintState>,
}

pub struct AxisPrepaintState {
    axis: Axis,
    bar_hitbox: Hitbox,
    bounds: Bounds<Pixels>,
    radius: Pixels,
    bg: Hsla,
    thumb_bounds: Bounds<Pixels>,
    thumb_fill_bounds: Bounds<Pixels>,
    thumb_bg: Hsla,
    scroll_size: Pixels,
    container_size: Pixels,
    thumb_size: Pixels,
    margin_end: Pixels,
}

impl Element for Scrollbar {
    type RequestLayoutState = ();
    type PrepaintState = PrepaintState;

    fn id(&self) -> Option<gpui::ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut style = Style::default();
        style.position = Position::Absolute;
        style.flex_grow = 1.0;
        style.flex_shrink = 1.0;
        style.size.width = relative(1.).into();
        style.size.height = relative(1.).into();

        (window.request_layout(style, None, cx), ())
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        window: &mut Window,
        _cx: &mut App,
    ) -> Self::PrepaintState {
        let theme = use_theme();

        let hitbox = window.with_content_mask(Some(ContentMask { bounds }), |window| {
            window.insert_hitbox(bounds, HitboxBehavior::Normal)
        });

        let mut states = vec![];
        let mut has_both = self.axis.is_both();

        let scroll_size = self.scroll_size.unwrap_or_else(|| {
            self.scroll_handle.max_offset() + self.scroll_handle.bounds().size
        });

        for axis in self.axis.all().into_iter() {
            let is_vertical = axis == Axis::Vertical;
            let (scroll_area_size, container_size, scroll_position) = if is_vertical {
                (
                    scroll_size.height,
                    hitbox.size.height,
                    self.scroll_handle.offset().y,
                )
            } else {
                (
                    scroll_size.width,
                    hitbox.size.width,
                    self.scroll_handle.offset().x,
                )
            };

            let margin_end = if has_both && !is_vertical {
                WIDTH
            } else {
                px(0.)
            };

            if scroll_area_size <= container_size {
                has_both = false;
                continue;
            }

            let thumb_length =
                (container_size / scroll_area_size * container_size).max(px(MIN_THUMB_SIZE));
            let thumb_start = -(scroll_position / (scroll_area_size - container_size)
                * (container_size - margin_end - thumb_length));
            let thumb_end = (thumb_start + thumb_length).min(container_size - margin_end);

            let bounds = Bounds {
                origin: if is_vertical {
                    point(hitbox.origin.x + hitbox.size.width - WIDTH, hitbox.origin.y)
                } else if self.horizontal_at_top {
                    // Position horizontal scrollbar at top
                    point(hitbox.origin.x, hitbox.origin.y)
                } else {
                    // Position horizontal scrollbar at bottom (default)
                    point(
                        hitbox.origin.x,
                        hitbox.origin.y + hitbox.size.height - WIDTH,
                    )
                },
                size: size(
                    if is_vertical { WIDTH } else { hitbox.size.width },
                    if is_vertical { hitbox.size.height } else { WIDTH },
                ),
            };

            let state_inner = self.state.0.get();
            let is_hovered_on_thumb = state_inner.hovered_on_thumb == Some(axis);
            let is_dragged = state_inner.dragged_axis == Some(axis);

            let (thumb_bg, track_bg, thumb_width, inset, radius) = if is_dragged || is_hovered_on_thumb {
                (
                    self.get_hover_thumb_color(&theme),
                    self.get_track_color(&theme),
                    THUMB_ACTIVE_WIDTH,
                    THUMB_ACTIVE_INSET,
                    THUMB_ACTIVE_RADIUS,
                )
            } else {
                (
                    self.get_thumb_color(&theme),
                    self.get_track_color(&theme),
                    THUMB_WIDTH,
                    THUMB_INSET,
                    THUMB_RADIUS,
                )
            };

            let thumb_length = thumb_end - thumb_start - inset * 2;
            let thumb_bounds = if is_vertical {
                Bounds::from_corner_and_size(
                    Corner::TopRight,
                    bounds.top_right() + point(-inset, inset + thumb_start),
                    size(WIDTH, thumb_length),
                )
            } else if self.horizontal_at_top {
                Bounds::from_corner_and_size(
                    Corner::TopLeft,
                    bounds.origin + point(inset + thumb_start, inset),
                    size(thumb_length, WIDTH),
                )
            } else {
                Bounds::from_corner_and_size(
                    Corner::BottomLeft,
                    bounds.bottom_left() + point(inset + thumb_start, -inset),
                    size(thumb_length, WIDTH),
                )
            };

            let thumb_fill_bounds = if is_vertical {
                Bounds::from_corner_and_size(
                    Corner::TopRight,
                    bounds.top_right() + point(-inset, inset + thumb_start),
                    size(thumb_width, thumb_length),
                )
            } else if self.horizontal_at_top {
                Bounds::from_corner_and_size(
                    Corner::TopLeft,
                    bounds.origin + point(inset + thumb_start, inset),
                    size(thumb_length, thumb_width),
                )
            } else {
                Bounds::from_corner_and_size(
                    Corner::BottomLeft,
                    bounds.bottom_left() + point(inset + thumb_start, -inset),
                    size(thumb_length, thumb_width),
                )
            };

            let bar_hitbox = window.with_content_mask(Some(ContentMask { bounds }), |window| {
                window.insert_hitbox(bounds, HitboxBehavior::Normal)
            });

            states.push(AxisPrepaintState {
                axis,
                bar_hitbox,
                bounds,
                radius,
                bg: track_bg,
                thumb_bounds,
                thumb_fill_bounds,
                thumb_bg,
                scroll_size: scroll_area_size,
                container_size,
                thumb_size: thumb_length,
                margin_end,
            })
        }

        PrepaintState { hitbox, states }
    }

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        _: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let view_id = window.current_view();
        let hitbox_bounds = prepaint.hitbox.bounds;
        let is_visible = self.state.0.get().is_scrollbar_visible() || self.always_visible;

        if self.scroll_handle.offset() != self.state.0.get().last_scroll_offset {
            self.state.0.set(
                self.state.0.get()
                    .with_last_scroll(self.scroll_handle.offset(), Some(Instant::now()))
            );
            cx.notify(view_id);
        }

        if !is_visible && !self.always_visible {
            return;
        }

        window.with_content_mask(
            Some(ContentMask {
                bounds: hitbox_bounds,
            }),
            |window| {
                for state in prepaint.states.iter() {
                    let axis = state.axis;
                    let radius = state.radius;
                    let bounds = state.bounds;
                    let thumb_bounds = state.thumb_bounds;
                    let scroll_area_size = state.scroll_size;
                    let container_size = state.container_size;
                    let thumb_size = state.thumb_size;
                    let margin_end = state.margin_end;
                    let is_vertical = axis == Axis::Vertical;

                    window.set_cursor_style(CursorStyle::default(), &state.bar_hitbox);

                    window.paint_layer(hitbox_bounds, |cx| {
                        cx.paint_quad(fill(state.bounds, state.bg));

                        cx.paint_quad(
                            fill(state.thumb_fill_bounds, state.thumb_bg).corner_radii(radius),
                        );
                    });

                    window.on_mouse_event({
                        let state = self.state.clone();
                        let scroll_handle = self.scroll_handle.clone();

                        move |event: &ScrollWheelEvent, phase, _hitbox, cx| {
                            if phase.bubble() && hitbox_bounds.contains(&event.position) {
                                if scroll_handle.offset() != state.0.get().last_scroll_offset {
                                    state.0.set(state.0.get().with_last_scroll(
                                        scroll_handle.offset(),
                                        Some(Instant::now()),
                                    ));
                                    cx.notify(view_id);
                                }
                            }
                        }
                    });

                    let safe_range = (-scroll_area_size + container_size)..px(0.);

                    window.on_mouse_event({
                        let state = self.state.clone();
                        let scroll_handle = self.scroll_handle.clone();

                        move |event: &MouseDownEvent, phase, _hitbox, cx| {
                            if phase.bubble() && bounds.contains(&event.position) {
                                cx.stop_propagation();

                                if thumb_bounds.contains(&event.position) {
                                    let pos = event.position - thumb_bounds.origin;
                                    state.0.set(state.0.get().with_drag_pos(axis, pos));
                                    cx.notify(view_id);
                                } else {
                                    let offset = scroll_handle.offset();
                                    let percentage = if is_vertical {
                                        (event.position.y - thumb_size / 2. - bounds.origin.y)
                                            / (bounds.size.height - thumb_size)
                                    } else {
                                        (event.position.x - thumb_size / 2. - bounds.origin.x)
                                            / (bounds.size.width - thumb_size)
                                    }
                                    .min(1.);

                                    if is_vertical {
                                        scroll_handle.set_offset(point(
                                            offset.x,
                                            (-scroll_area_size * percentage)
                                                .clamp(safe_range.start, safe_range.end),
                                        ));
                                    } else {
                                        scroll_handle.set_offset(point(
                                            (-scroll_area_size * percentage)
                                                .clamp(safe_range.start, safe_range.end),
                                            offset.y,
                                        ));
                                    }
                                }
                            }
                        }
                    });

                    window.on_mouse_event({
                        let scroll_handle = self.scroll_handle.clone();
                        let state = self.state.clone();

                        move |event: &MouseMoveEvent, _phase, _hitbox, cx| {
                            let mut notify = false;

                            if thumb_bounds.contains(&event.position) {
                                if state.0.get().hovered_on_thumb != Some(axis) {
                                    state.0.set(state.0.get().with_hovered_on_thumb(Some(axis)));
                                    notify = true;
                                }
                            } else {
                                if state.0.get().hovered_on_thumb == Some(axis) {
                                    state.0.set(state.0.get().with_hovered_on_thumb(None));
                                    notify = true;
                                }
                            }

                            if state.0.get().dragged_axis == Some(axis) && event.dragging() {
                                let drag_pos = state.0.get().drag_pos;

                                let percentage = (if is_vertical {
                                    (event.position.y - drag_pos.y - bounds.origin.y)
                                        / (bounds.size.height - thumb_size)
                                } else {
                                    (event.position.x - drag_pos.x - bounds.origin.x)
                                        / (bounds.size.width - thumb_size - margin_end)
                                })
                                .clamp(0., 1.);

                                let offset = if is_vertical {
                                    point(
                                        scroll_handle.offset().x,
                                        (-(scroll_area_size - container_size) * percentage)
                                            .clamp(safe_range.start, safe_range.end),
                                    )
                                } else {
                                    point(
                                        (-(scroll_area_size - container_size) * percentage)
                                            .clamp(safe_range.start, safe_range.end),
                                        scroll_handle.offset().y,
                                    )
                                };

                                if (scroll_handle.offset().y - offset.y).abs() > px(1.)
                                    || (scroll_handle.offset().x - offset.x).abs() > px(1.)
                                {
                                    scroll_handle.set_offset(offset);
                                    state.0.set(state.0.get().with_last_update(Instant::now()));
                                    notify = true;
                                }
                            }

                            if notify {
                                cx.notify(view_id);
                            }
                        }
                    });

                    window.on_mouse_event({
                        let state = self.state.clone();

                        move |_event: &MouseUpEvent, phase, _hitbox, cx| {
                            if phase.bubble() {
                                state.0.set(state.0.get().with_unset_drag_pos());
                                cx.notify(view_id);
                            }
                        }
                    });
                }
            },
        );
    }
}