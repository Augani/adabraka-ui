//! Image crop tool with draggable/resizable selection.

use gpui::{prelude::FluentBuilder as _, *};

use crate::theme::use_theme;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DragHandle {
    None,
    Move,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Top,
    Bottom,
    Left,
    Right,
}

pub struct CropAreaState {
    pub selection: Bounds<f32>,
    is_dragging: bool,
    drag_handle: DragHandle,
    drag_start: Point<f32>,
    selection_start: Bounds<f32>,
    bounds: Option<Bounds<Pixels>>,
}

impl CropAreaState {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            selection: Bounds::new(point(0.1, 0.1), size(0.8, 0.8)),
            is_dragging: false,
            drag_handle: DragHandle::None,
            drag_start: Point::default(),
            selection_start: Bounds::new(point(0.1, 0.1), size(0.8, 0.8)),
            bounds: None,
        }
    }

    pub fn get_selection(&self) -> Bounds<f32> {
        self.selection
    }

    fn to_normalized(&self, pos: Point<Pixels>) -> Point<f32> {
        if let Some(bounds) = self.bounds {
            let x = (pos.x - bounds.left()) / bounds.size.width;
            let y = (pos.y - bounds.top()) / bounds.size.height;
            point(x, y)
        } else {
            Point::default()
        }
    }

    fn hit_test(&self, norm: Point<f32>) -> DragHandle {
        let sel = &self.selection;
        let handle_size = 0.03;

        let left = sel.origin.x;
        let top = sel.origin.y;
        let right = sel.origin.x + sel.size.width;
        let bottom = sel.origin.y + sel.size.height;

        if (norm.x - left).abs() < handle_size && (norm.y - top).abs() < handle_size {
            return DragHandle::TopLeft;
        }
        if (norm.x - right).abs() < handle_size && (norm.y - top).abs() < handle_size {
            return DragHandle::TopRight;
        }
        if (norm.x - left).abs() < handle_size && (norm.y - bottom).abs() < handle_size {
            return DragHandle::BottomLeft;
        }
        if (norm.x - right).abs() < handle_size && (norm.y - bottom).abs() < handle_size {
            return DragHandle::BottomRight;
        }

        if (norm.y - top).abs() < handle_size && norm.x > left && norm.x < right {
            return DragHandle::Top;
        }
        if (norm.y - bottom).abs() < handle_size && norm.x > left && norm.x < right {
            return DragHandle::Bottom;
        }
        if (norm.x - left).abs() < handle_size && norm.y > top && norm.y < bottom {
            return DragHandle::Left;
        }
        if (norm.x - right).abs() < handle_size && norm.y > top && norm.y < bottom {
            return DragHandle::Right;
        }

        if norm.x > left && norm.x < right && norm.y > top && norm.y < bottom {
            return DragHandle::Move;
        }

        DragHandle::None
    }

    fn apply_drag(&mut self, norm: Point<f32>, min_size: f32, aspect_ratio: Option<f32>) {
        let dx = norm.x - self.drag_start.x;
        let dy = norm.y - self.drag_start.y;
        let start = &self.selection_start;

        match self.drag_handle {
            DragHandle::Move => {
                let new_x = (start.origin.x + dx).clamp(0.0, 1.0 - start.size.width);
                let new_y = (start.origin.y + dy).clamp(0.0, 1.0 - start.size.height);
                self.selection.origin.x = new_x;
                self.selection.origin.y = new_y;
            }
            DragHandle::BottomRight => {
                let mut new_w = (start.size.width + dx).max(min_size);
                let mut new_h = (start.size.height + dy).max(min_size);
                if let Some(ratio) = aspect_ratio {
                    new_h = new_w / ratio;
                }
                new_w = new_w.min(1.0 - start.origin.x);
                new_h = new_h.min(1.0 - start.origin.y);
                self.selection.size.width = new_w;
                self.selection.size.height = new_h;
            }
            DragHandle::TopLeft => {
                let mut new_x = start.origin.x + dx;
                let mut new_y = start.origin.y + dy;
                let max_x = start.origin.x + start.size.width - min_size;
                let max_y = start.origin.y + start.size.height - min_size;
                new_x = new_x.clamp(0.0, max_x);
                new_y = new_y.clamp(0.0, max_y);
                let new_w = start.origin.x + start.size.width - new_x;
                let new_h = start.origin.y + start.size.height - new_y;
                if let Some(ratio) = aspect_ratio {
                    let adj_h = new_w / ratio;
                    self.selection = Bounds::new(
                        point(new_x, start.origin.y + start.size.height - adj_h),
                        size(new_w, adj_h),
                    );
                } else {
                    self.selection = Bounds::new(point(new_x, new_y), size(new_w, new_h));
                }
            }
            DragHandle::TopRight => {
                let mut new_w = (start.size.width + dx)
                    .max(min_size)
                    .min(1.0 - start.origin.x);
                let mut new_y = start.origin.y + dy;
                let max_y = start.origin.y + start.size.height - min_size;
                new_y = new_y.clamp(0.0, max_y);
                let new_h = start.origin.y + start.size.height - new_y;
                if let Some(ratio) = aspect_ratio {
                    let adj_h = new_w / ratio;
                    self.selection = Bounds::new(
                        point(start.origin.x, start.origin.y + start.size.height - adj_h),
                        size(new_w, adj_h),
                    );
                } else {
                    self.selection = Bounds::new(point(start.origin.x, new_y), size(new_w, new_h));
                }
            }
            DragHandle::BottomLeft => {
                let mut new_x = start.origin.x + dx;
                let max_x = start.origin.x + start.size.width - min_size;
                new_x = new_x.clamp(0.0, max_x);
                let new_w = start.origin.x + start.size.width - new_x;
                let mut new_h = (start.size.height + dy)
                    .max(min_size)
                    .min(1.0 - start.origin.y);
                if let Some(ratio) = aspect_ratio {
                    new_h = new_w / ratio;
                }
                self.selection = Bounds::new(point(new_x, start.origin.y), size(new_w, new_h));
            }
            DragHandle::Top => {
                let mut new_y = start.origin.y + dy;
                let max_y = start.origin.y + start.size.height - min_size;
                new_y = new_y.clamp(0.0, max_y);
                let new_h = start.origin.y + start.size.height - new_y;
                self.selection.origin.y = new_y;
                self.selection.size.height = new_h;
            }
            DragHandle::Bottom => {
                let new_h = (start.size.height + dy)
                    .max(min_size)
                    .min(1.0 - start.origin.y);
                self.selection.size.height = new_h;
            }
            DragHandle::Left => {
                let mut new_x = start.origin.x + dx;
                let max_x = start.origin.x + start.size.width - min_size;
                new_x = new_x.clamp(0.0, max_x);
                let new_w = start.origin.x + start.size.width - new_x;
                self.selection.origin.x = new_x;
                self.selection.size.width = new_w;
            }
            DragHandle::Right => {
                let new_w = (start.size.width + dx)
                    .max(min_size)
                    .min(1.0 - start.origin.x);
                self.selection.size.width = new_w;
            }
            DragHandle::None => {}
        }
    }
}

impl Render for CropAreaState {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Empty
    }
}

#[derive(Clone)]
struct CropPaintData {
    selection: Bounds<f32>,
    dim_color: Hsla,
    border_color: Hsla,
    handle_color: Hsla,
}

#[derive(IntoElement)]
pub struct CropArea {
    id: ElementId,
    state: Entity<CropAreaState>,
    aspect_ratio: Option<f32>,
    min_size: f32,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl CropArea {
    pub fn new(id: impl Into<ElementId>, state: Entity<CropAreaState>) -> Self {
        Self {
            id: id.into(),
            state,
            aspect_ratio: None,
            min_size: 0.05,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn aspect_ratio(mut self, ratio: f32) -> Self {
        self.aspect_ratio = Some(ratio);
        self
    }

    pub fn min_size(mut self, min: f32) -> Self {
        self.min_size = min.clamp(0.01, 0.5);
        self
    }
}

impl Styled for CropArea {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for CropArea {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for CropArea {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let user_style = self.style;
        let selection = self.state.read(cx).selection;

        let dim_color = hsla(0.0, 0.0, 0.0, 0.5);
        let border_color = theme.tokens.primary;
        let handle_color = theme.tokens.background;

        let paint_data = CropPaintData {
            selection,
            dim_color,
            border_color,
            handle_color,
        };

        let state_down = self.state.clone();
        let state_move = self.state.clone();
        let state_up = self.state.clone();
        let min_size = self.min_size;
        let aspect_ratio = self.aspect_ratio;

        div()
            .id(self.id)
            .relative()
            .overflow_hidden()
            .cursor_crosshair()
            .children(self.children)
            .child(
                canvas(
                    move |_, _, _| paint_data,
                    move |bounds, data, window, _cx| {
                        let sel = &data.selection;
                        let bw = bounds.size.width;
                        let bh = bounds.size.height;

                        let sel_left = bounds.left() + bw * sel.origin.x;
                        let sel_top = bounds.top() + bh * sel.origin.y;
                        let sel_w = bw * sel.size.width;
                        let sel_h = bh * sel.size.height;

                        let top_dim = Bounds::new(bounds.origin, size(bw, sel_top - bounds.top()));
                        window.paint_quad(fill(top_dim, data.dim_color));

                        let bottom_dim = Bounds::new(
                            point(bounds.left(), sel_top + sel_h),
                            size(bw, bounds.bottom() - (sel_top + sel_h)),
                        );
                        window.paint_quad(fill(bottom_dim, data.dim_color));

                        let left_dim = Bounds::new(
                            point(bounds.left(), sel_top),
                            size(sel_left - bounds.left(), sel_h),
                        );
                        window.paint_quad(fill(left_dim, data.dim_color));

                        let right_dim = Bounds::new(
                            point(sel_left + sel_w, sel_top),
                            size(bounds.right() - (sel_left + sel_w), sel_h),
                        );
                        window.paint_quad(fill(right_dim, data.dim_color));

                        let border_w = px(2.0);
                        window.paint_quad(PaintQuad {
                            bounds: Bounds::new(point(sel_left, sel_top), size(sel_w, sel_h)),
                            corner_radii: Corners::default(),
                            background: gpui::transparent_black().into(),
                            border_widths: Edges::all(border_w),
                            border_color: data.border_color,
                            border_style: BorderStyle::default(),
                            continuous_corners: false,
                            transform: Default::default(),
                        });

                        let handle_sz = px(8.0);
                        let half = handle_sz * 0.5;
                        let handle_corners: [(Pixels, Pixels); 4] = [
                            (sel_left, sel_top),
                            (sel_left + sel_w, sel_top),
                            (sel_left, sel_top + sel_h),
                            (sel_left + sel_w, sel_top + sel_h),
                        ];

                        for (hx, hy) in handle_corners {
                            window.paint_quad(PaintQuad {
                                bounds: Bounds::new(
                                    point(hx - half, hy - half),
                                    size(handle_sz, handle_sz),
                                ),
                                corner_radii: Corners::all(px(2.0)),
                                background: data.handle_color.into(),
                                border_widths: Edges::all(px(1.5)),
                                border_color: data.border_color,
                                border_style: BorderStyle::default(),
                                continuous_corners: false,
                                transform: Default::default(),
                            });
                        }
                    },
                )
                .absolute()
                .inset_0()
                .size_full(),
            )
            .on_mouse_down(
                MouseButton::Left,
                move |event: &MouseDownEvent, _window, cx| {
                    state_down.update(cx, |s, cx| {
                        let norm = s.to_normalized(event.position);
                        let handle = s.hit_test(norm);
                        if handle != DragHandle::None {
                            s.is_dragging = true;
                            s.drag_handle = handle;
                            s.drag_start = norm;
                            s.selection_start = s.selection;
                            cx.notify();
                        }
                    });
                },
            )
            .on_mouse_move(move |event: &MouseMoveEvent, _window, cx| {
                state_move.update(cx, |s, cx| {
                    if s.is_dragging {
                        let norm = s.to_normalized(event.position);
                        s.apply_drag(norm, min_size, aspect_ratio);
                        cx.notify();
                    }
                });
            })
            .on_mouse_up(
                MouseButton::Left,
                move |_event: &MouseUpEvent, _window, cx| {
                    state_up.update(cx, |s, cx| {
                        s.is_dragging = false;
                        s.drag_handle = DragHandle::None;
                        cx.notify();
                    });
                },
            )
            .map(|this: Stateful<Div>| {
                let mut el = this;
                el.style().refine(&user_style);
                el
            })
    }
}
