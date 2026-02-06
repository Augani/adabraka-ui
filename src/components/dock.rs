//! macOS Dock-style magnification row.

use gpui::{prelude::FluentBuilder as _, *};

use crate::theme::use_theme;

pub struct DockState {
    cursor_x: Option<Pixels>,
}

impl DockState {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self { cursor_x: None }
    }
}

impl Render for DockState {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Empty
    }
}

#[allow(dead_code)]
struct DockItemInfo {
    center_x: f32,
    scale: f32,
}

fn compute_scales(
    item_count: usize,
    item_size_f: f32,
    gap: f32,
    cursor_rel_x: Option<f32>,
    max_scale: f32,
    sigma: f32,
) -> Vec<DockItemInfo> {
    let mut items = Vec::with_capacity(item_count);
    let total_width = item_count as f32 * item_size_f + (item_count.saturating_sub(1)) as f32 * gap;
    let start_x = -total_width * 0.5;

    for i in 0..item_count {
        let center = start_x + i as f32 * (item_size_f + gap) + item_size_f * 0.5;
        let scale = if let Some(cx_pos) = cursor_rel_x {
            let dist = (center - cx_pos).abs();
            1.0 + max_scale * (-dist * dist / (2.0 * sigma * sigma)).exp()
        } else {
            1.0
        };
        items.push(DockItemInfo {
            center_x: center,
            scale,
        });
    }
    items
}

#[derive(IntoElement)]
pub struct Dock {
    id: ElementId,
    state: Entity<DockState>,
    max_scale: f32,
    item_size: Pixels,
    gap: Pixels,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Dock {
    pub fn new(id: impl Into<ElementId>, state: Entity<DockState>) -> Self {
        Self {
            id: id.into(),
            state,
            max_scale: 0.5,
            item_size: px(48.0),
            gap: px(4.0),
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn max_scale(mut self, scale: f32) -> Self {
        self.max_scale = scale.max(0.0);
        self
    }

    pub fn item_size(mut self, size: Pixels) -> Self {
        self.item_size = size;
        self
    }

    pub fn gap(mut self, gap: Pixels) -> Self {
        self.gap = gap;
        self
    }
}

impl Styled for Dock {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Dock {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Dock {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let user_style = self.style;
        let state = self.state.read(cx);
        let item_count = self.children.len();
        let item_size_f = self.item_size / px(1.0);
        let gap_f = self.gap / px(1.0);
        let sigma = item_size_f * 1.5;

        let cursor_rel = state.cursor_x.map(|cx_pos| cx_pos / px(1.0));

        let scales = compute_scales(
            item_count,
            item_size_f,
            gap_f,
            cursor_rel,
            self.max_scale,
            sigma,
        );

        let state_move = self.state.clone();
        let state_hover = self.state.clone();

        let mut row = div()
            .id(self.id)
            .flex()
            .flex_row()
            .items_end()
            .gap(self.gap)
            .px(px(8.0))
            .py(px(4.0))
            .rounded(px(12.0))
            .bg(theme.tokens.card)
            .border_1()
            .border_color(theme.tokens.border)
            .on_mouse_move(move |event: &MouseMoveEvent, _window, cx| {
                state_move.update(cx, |s, cx| {
                    s.cursor_x = Some(event.position.x);
                    cx.notify();
                });
            })
            .on_hover(move |hovered: &bool, _window, cx| {
                if !*hovered {
                    state_hover.update(cx, |s, cx| {
                        s.cursor_x = None;
                        cx.notify();
                    });
                }
            })
            .map(|this: Stateful<Div>| {
                let mut el = this;
                el.style().refine(&user_style);
                el
            });

        for (child, info) in self.children.into_iter().zip(scales.iter()) {
            let scaled_size = px(item_size_f * info.scale);
            row = row.child(
                div()
                    .flex_shrink_0()
                    .w(scaled_size)
                    .h(scaled_size)
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(child),
            );
        }

        row
    }
}
