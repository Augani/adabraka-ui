//! Direct GPUI painting surface with Styled support.

use gpui::{prelude::FluentBuilder as _, *};
use std::rc::Rc;

type PaintCallback = Rc<dyn Fn(Bounds<Pixels>, &mut Window, &mut App)>;
type PrepareCallback = Rc<dyn Fn(Bounds<Pixels>, &mut Window, &mut App)>;

#[derive(IntoElement)]
pub struct CanvasComponent {
    id: ElementId,
    on_paint: Option<PaintCallback>,
    on_prepare: Option<PrepareCallback>,
    style: StyleRefinement,
}

impl CanvasComponent {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            on_paint: None,
            on_prepare: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn on_paint(
        mut self,
        callback: impl Fn(Bounds<Pixels>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_paint = Some(Rc::new(callback));
        self
    }

    pub fn on_prepare(
        mut self,
        callback: impl Fn(Bounds<Pixels>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_prepare = Some(Rc::new(callback));
        self
    }
}

impl Styled for CanvasComponent {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for CanvasComponent {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let user_style = self.style;
        let paint_cb = self.on_paint;
        let prepare_cb = self.on_prepare;

        div()
            .id(self.id)
            .relative()
            .child(
                canvas(
                    move |bounds, window, cx| {
                        if let Some(ref cb) = prepare_cb {
                            cb(bounds, window, cx);
                        }
                    },
                    move |bounds, _, window, cx| {
                        if let Some(ref cb) = paint_cb {
                            cb(bounds, window, cx);
                        }
                    },
                )
                .absolute()
                .inset_0()
                .size_full(),
            )
            .map(|this| {
                let mut el = this;
                el.style().refine(&user_style);
                el
            })
    }
}
