//! Drag and drop components with draggable elements and drop zones.

use gpui::{prelude::FluentBuilder as _, *};
use std::fmt::Debug;

use crate::theme::use_theme;

use std::rc::Rc;

pub struct DragData<T: Clone + Debug> {
    pub data: T,
    pub label: Option<SharedString>,
    pub preview_factory: Option<Rc<dyn Fn() -> AnyElement>>,
    pub position: Point<Pixels>,
}
impl<T: Clone + Debug> Clone for DragData<T> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            label: self.label.clone(),
            preview_factory: self.preview_factory.clone(), // Rc can be cloned
            position: self.position.clone(),
        }
    }
}
impl<T: Clone + Debug> Debug for DragData<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DragData")
            .field("data", &self.data)
            .field("label", &self.label)
            .field("preview_factory", &self.preview_factory.is_some())
            .field("position", &self.position)
            .finish()
    }
}

impl<T: Clone + Debug> DragData<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            label: None,
            preview_factory: None,
            position: Point::default(),
        }
    }

    pub fn with_label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn with_preview<F>(mut self, factory: F) -> Self
    where
        F: Fn() -> AnyElement + 'static,
    {
        self.preview_factory = Some(Rc::new(move || factory()));
        self
    }

    pub fn with_position(mut self, position: Point<Pixels>) -> Self {
        self.position = position;
        self
    }
}

impl<T: Clone + Debug + 'static> Render for DragData<T> {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        if let Some(factory) = &self.preview_factory {
            let preview = factory();
            return div()
                .absolute()
                .left(self.position.x)
                .top(self.position.y)
                .child(preview);
        }

        let size = gpui::size(px(250.0), px(80.0));

        div()
            .pl(self.position.x - size.width / 2.0)
            .pt(self.position.y - size.height / 2.0)
            .child(
                div()
                    .flex()
                    .justify_center()
                    .items_center()
                    .min_w(size.width)
                    .max_w(px(300.0))
                    .min_h(size.height)
                    .px(px(16.0))
                    .py(px(12.0))
                    .bg(theme.tokens.card.opacity(0.95))
                    .border_1()
                    .border_color(theme.tokens.border)
                    .text_color(theme.tokens.foreground)
                    .font_family(theme.tokens.font_family.clone())
                    .text_size(px(14.0))
                    .font_weight(FontWeight::MEDIUM)
                    .rounded(theme.tokens.radius_md)
                    .shadow(vec![BoxShadow {
                        color: hsla(0.0, 0.0, 0.0, 0.3),
                        offset: point(px(0.0), px(4.0)),
                        blur_radius: px(12.0),
                        spread_radius: px(0.0),
                    }])
                    .when_some(self.label.clone(), |this, label| {
                        this.child(label)
                    })
                    .when(self.label.is_none(), |this| {
                        this.child("Dragging...")
                    })
            )
    }
}

#[derive(IntoElement)]
pub struct Draggable<T: Clone + Debug + 'static> {
    base: Stateful<Div>,
    drag_data: DragData<T>,
    cursor_style: CursorStyle,
    hover_bg: Option<Hsla>,
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl<T: Clone + Debug + 'static> Draggable<T> {
    pub fn new(id: impl Into<ElementId>, drag_data: DragData<T>) -> Self {
        Self {
            base: div().id(id.into()),
            drag_data,
            cursor_style: CursorStyle::PointingHand,
            hover_bg: None,
            children: Vec::new(),
            style: StyleRefinement::default(),
        }
    }

    pub fn cursor_style(mut self, cursor: CursorStyle) -> Self {
        self.cursor_style = cursor;
        self
    }

    pub fn hover_bg(mut self, color: Hsla) -> Self {
        self.hover_bg = Some(color);
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    pub fn children<I>(mut self, children: impl IntoIterator<Item = I>) -> Self
    where
        I: IntoElement,
    {
        for child in children {
            self.children.push(child.into_any_element());
        }
        self
    }
}

impl<T: Clone + Debug + 'static> Styled for Draggable<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl<T: Clone + Debug + 'static> ParentElement for Draggable<T> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl<T: Clone + Debug + 'static> RenderOnce for Draggable<T> {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let drag_data = self.drag_data.clone();
        let user_style = self.style;

        self.base
            .cursor(self.cursor_style)
            .when_some(self.hover_bg, |this, bg| {
                this.hover(move |style| style.bg(bg))
            })
            .on_drag(drag_data, |data: &DragData<T>, position, _, cx| {
                cx.new(|_| data.clone().with_position(position))
            })
            .map(|this| {
                let mut div = this;
                div.style().refine(&user_style);
                div
            })
            .children(self.children)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DropZoneStyle {
    Dashed,
    Solid,
    Filled,
}

#[derive(IntoElement)]
pub struct DropZone<T: Clone + Debug + 'static> {
    base: Stateful<Div>,
    drop_style: DropZoneStyle,
    active: bool,
    min_height: Option<Pixels>,
    children: Vec<AnyElement>,
    user_style: StyleRefinement,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Clone + Debug + 'static> DropZone<T> {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            base: div().id(id.into()),
            drop_style: DropZoneStyle::Dashed,
            active: false,
            min_height: None,
            children: Vec::new(),
            user_style: StyleRefinement::default(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn drop_zone_style(mut self, style: DropZoneStyle) -> Self {
        self.drop_style = style;
        self
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    pub fn min_h(mut self, height: impl Into<Pixels>) -> Self {
        self.min_height = Some(height.into());
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    pub fn children<I>(mut self, children: impl IntoIterator<Item = I>) -> Self
    where
        I: IntoElement,
    {
        for child in children {
            self.children.push(child.into_any_element());
        }
        self
    }
}

impl<T: Clone + Debug + 'static> Styled for DropZone<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.user_style
    }
}

impl<T: Clone + Debug + 'static> InteractiveElement for DropZone<T> {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl<T: Clone + Debug + 'static> StatefulInteractiveElement for DropZone<T> {}

impl<T: Clone + Debug + 'static> ParentElement for DropZone<T> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl<T: Clone + Debug + 'static> RenderOnce for DropZone<T> {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let user_style = self.user_style;

        let (border_width, border_color, bg_color) = match (self.drop_style, self.active) {
            (DropZoneStyle::Dashed, false) => (
                px(2.0),
                theme.tokens.border,
                gpui::transparent_black(),
            ),
            (DropZoneStyle::Dashed, true) => (
                px(2.0),
                theme.tokens.primary,
                theme.tokens.primary.opacity(0.05),
            ),
            (DropZoneStyle::Solid, false) => (
                px(2.0),
                theme.tokens.border,
                gpui::transparent_black(),
            ),
            (DropZoneStyle::Solid, true) => (
                px(2.0),
                theme.tokens.primary,
                theme.tokens.primary.opacity(0.1),
            ),
            (DropZoneStyle::Filled, false) => (
                px(1.0),
                theme.tokens.border,
                theme.tokens.muted,
            ),
            (DropZoneStyle::Filled, true) => (
                px(2.0),
                theme.tokens.primary,
                theme.tokens.primary.opacity(0.15),
            ),
        };

        self.base
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .w_full()
            .when_some(self.min_height, |this, h| this.min_h(h))
            .px(px(16.0))
            .py(px(16.0))
            .rounded(theme.tokens.radius_lg)
            .bg(bg_color)
            .border_color(border_color)
            .when(self.drop_style == DropZoneStyle::Dashed, |this| {
                this.border(border_width)
            })
            .when(self.drop_style != DropZoneStyle::Dashed, |this| {
                this.border(border_width)
            })
            .map(|this| {
                let mut div = this;
                div.style().refine(&user_style);
                div
            })
            .children(self.children)
    }
}
