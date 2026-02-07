use gpui::{prelude::FluentBuilder as _, *};
use std::rc::Rc;

use crate::theme::use_theme;

#[derive(Clone)]
pub struct SortableItemDrag {
    index: usize,
    position: Point<Pixels>,
}

impl std::fmt::Debug for SortableItemDrag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SortableItemDrag")
            .field("index", &self.index)
            .finish()
    }
}

impl Render for SortableItemDrag {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();
        div().pl(self.position.x).pt(self.position.y).child(
            div()
                .px(px(12.0))
                .py(px(8.0))
                .bg(theme.tokens.card.opacity(0.95))
                .border_1()
                .border_color(theme.tokens.primary)
                .rounded(theme.tokens.radius_md)
                .shadow(smallvec::smallvec![BoxShadow {
                    color: hsla(0.0, 0.0, 0.0, 0.2),
                    offset: point(px(0.0), px(4.0)),
                    blur_radius: px(8.0),
                    spread_radius: px(0.0),
                    inset: false,
                }])
                .text_size(px(14.0))
                .text_color(theme.tokens.foreground)
                .font_family(theme.tokens.font_family.clone())
                .child("Moving..."),
        )
    }
}

pub struct SortableListState<T: Clone + 'static> {
    items: Vec<T>,
    dragging_index: Option<usize>,
    hover_index: Option<usize>,
}

impl<T: Clone + 'static> SortableListState<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self {
            items,
            dragging_index: None,
            hover_index: None,
        }
    }

    pub fn items(&self) -> &[T] {
        &self.items
    }

    pub fn set_items(&mut self, items: Vec<T>) {
        self.items = items;
    }

    pub fn dragging_index(&self) -> Option<usize> {
        self.dragging_index
    }

    pub fn hover_index(&self) -> Option<usize> {
        self.hover_index
    }
}

#[derive(IntoElement)]
pub struct SortableList<T: Clone + 'static> {
    state: Entity<SortableListState<T>>,
    item_renderer: Rc<dyn Fn(&T, usize, bool) -> AnyElement>,
    on_reorder: Option<Rc<dyn Fn(Vec<T>, &mut Window, &mut App)>>,
    direction: Axis,
    gap: Pixels,
    style: StyleRefinement,
}

impl<T: Clone + 'static> SortableList<T> {
    pub fn new(
        state: Entity<SortableListState<T>>,
        renderer: impl Fn(&T, usize, bool) -> AnyElement + 'static,
    ) -> Self {
        Self {
            state,
            item_renderer: Rc::new(renderer),
            on_reorder: None,
            direction: Axis::Vertical,
            gap: px(4.0),
            style: StyleRefinement::default(),
        }
    }

    pub fn on_reorder(
        mut self,
        callback: impl Fn(Vec<T>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_reorder = Some(Rc::new(callback));
        self
    }

    pub fn direction(mut self, direction: Axis) -> Self {
        self.direction = direction;
        self
    }

    pub fn gap(mut self, gap: impl Into<Pixels>) -> Self {
        self.gap = gap.into();
        self
    }
}

impl<T: Clone + 'static> Styled for SortableList<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl<T: Clone + 'static> RenderOnce for SortableList<T> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let user_style = self.style;
        let items = self.state.read(cx).items.clone();
        let dragging_index = self.state.read(cx).dragging_index;
        let drag_over_bg = theme.tokens.primary.opacity(0.1);
        let indicator_color = theme.tokens.primary;

        let mut container = div()
            .flex()
            .when(self.direction == Axis::Vertical, |d| d.flex_col())
            .gap(self.gap);

        for (idx, item) in items.iter().enumerate() {
            let is_dragging = dragging_index == Some(idx);
            let rendered = (self.item_renderer)(item, idx, is_dragging);

            let state_drop = self.state.clone();
            let on_reorder = self.on_reorder.clone();
            let state_drag = self.state.clone();

            let item_el = div()
                .id(ElementId::Name(format!("sortable-item-{}", idx).into()))
                .child(rendered)
                .on_drag(
                    SortableItemDrag {
                        index: idx,
                        position: Point::default(),
                    },
                    move |data: &SortableItemDrag, pos, _window, cx| {
                        state_drag.update(cx, |s, _| {
                            s.dragging_index = Some(data.index);
                        });
                        cx.new(|_| SortableItemDrag {
                            index: data.index,
                            position: pos,
                        })
                    },
                )
                .drag_over::<SortableItemDrag>(move |style, _, _, _| {
                    style
                        .bg(drag_over_bg)
                        .border_t(px(2.0))
                        .border_color(indicator_color)
                })
                .on_drop(move |dragged: &SortableItemDrag, window, cx| {
                    let from = dragged.index;
                    let to = idx;
                    if from == to {
                        state_drop.update(cx, |s, ctx| {
                            s.dragging_index = None;
                            s.hover_index = None;
                            ctx.notify();
                        });
                        return;
                    }

                    state_drop.update(cx, |s, ctx| {
                        let mut reordered = s.items.clone();
                        if from < reordered.len() {
                            let moved = reordered.remove(from);
                            let insert_at = to.min(reordered.len());
                            reordered.insert(insert_at, moved);
                            s.items = reordered;
                        }
                        s.dragging_index = None;
                        s.hover_index = None;
                        ctx.notify();
                    });

                    if let Some(ref callback) = on_reorder {
                        let reordered_items = state_drop.read(cx).items.clone();
                        callback(reordered_items, window, cx);
                    }
                })
                .when(is_dragging, |d| d.opacity(0.5));

            container = container.child(item_el);
        }

        container.map(|mut this| {
            this.style().refine(&user_style);
            this
        })
    }
}
