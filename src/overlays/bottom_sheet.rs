//! Bottom sheet component for slide-up panels.

use gpui::{prelude::FluentBuilder as _, *};
use std::rc::Rc;

use crate::theme::use_theme;
use crate::components::text::{Text, TextVariant};
use crate::animations::presets;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BottomSheetSize {
    Sm,
    Md,
    Lg,
    Xl,
    Custom,
}

impl Default for BottomSheetSize {
    fn default() -> Self {
        Self::Md
    }
}

impl BottomSheetSize {
    fn height(&self) -> Pixels {
        match self {
            Self::Sm => px(300.0),
            Self::Md => px(400.0),
            Self::Lg => px(500.0),
            Self::Xl => px(600.0),
            Self::Custom => px(400.0),
        }
    }
}

#[derive(IntoElement)]
pub struct BottomSheet {
    size: BottomSheetSize,
    custom_height: Option<Pixels>,
    title: Option<SharedString>,
    description: Option<SharedString>,
    content: Option<AnyElement>,
    actions: Option<AnyElement>,
    show_drag_handle: bool,
    close_on_backdrop_click: bool,
    on_close: Option<Rc<dyn Fn(&mut Window, &mut App)>>,
    style: StyleRefinement,
}

impl BottomSheet {
    pub fn new() -> Self {
        Self {
            size: BottomSheetSize::default(),
            custom_height: None,
            title: None,
            description: None,
            content: None,
            actions: None,
            show_drag_handle: true,
            close_on_backdrop_click: true,
            on_close: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn size(mut self, size: BottomSheetSize) -> Self {
        self.size = size;
        self
    }

    pub fn height(mut self, height: impl Into<Pixels>) -> Self {
        self.custom_height = Some(height.into());
        self.size = BottomSheetSize::Custom;
        self
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn content(mut self, content: impl IntoElement) -> Self {
        self.content = Some(content.into_any_element());
        self
    }

    pub fn actions(mut self, actions: impl IntoElement) -> Self {
        self.actions = Some(actions.into_any_element());
        self
    }

    pub fn show_drag_handle(mut self, show: bool) -> Self {
        self.show_drag_handle = show;
        self
    }

    pub fn close_on_backdrop_click(mut self, close: bool) -> Self {
        self.close_on_backdrop_click = close;
        self
    }

    pub fn on_close<F>(mut self, handler: F) -> Self
    where
        F: Fn(&mut Window, &mut App) + 'static,
    {
        self.on_close = Some(Rc::new(handler));
        self
    }

    fn get_sheet_height(&self) -> Pixels {
        if let Some(height) = self.custom_height {
            return height;
        }
        self.size.height()
    }
}

impl Default for BottomSheet {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for BottomSheet {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for BottomSheet {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let has_header = self.title.is_some() || self.description.is_some() || self.actions.is_some();
        let sheet_height = self.get_sheet_height();
        let on_close = self.on_close.clone();
        let user_style = self.style;

        deferred(
            div()
                .absolute()
                .inset_0()
                .flex()
                .flex_col()
                .bg(hsla(0.0, 0.0, 0.0, 0.6))
                .when(self.close_on_backdrop_click, |this: Div| {
                    let on_close = on_close.clone();
                    this.on_mouse_down(MouseButton::Left, move |_, window, cx| {
                        if let Some(handler) = on_close.as_ref() {
                            handler(window, cx);
                        }
                    })
                })
                .child(
                    div()
                        .id("bottom-sheet-panel")
                        .occlude()
                        .absolute()
                        .bottom_0()
                        .left_0()
                        .right_0()
                        .h(sheet_height)
                        .flex()
                        .flex_col()
                        .bg(theme.tokens.background)
                        .border_t_1()
                        .border_color(theme.tokens.border)
                        .rounded_tl(theme.tokens.radius_xl)
                        .rounded_tr(theme.tokens.radius_xl)
                        .shadow(vec![BoxShadow {
                            color: hsla(0.0, 0.0, 0.0, 0.3),
                            offset: point(px(0.0), px(-4.0)),
                            blur_radius: px(24.0),
                            spread_radius: px(0.0),
                        }])
                        .map(|this| {
                            let mut div = this;
                            div.style().refine(&user_style);
                            div
                        })
                        .when(self.show_drag_handle, |this| {
                            this.child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .items_center()
                                    .pt(px(12.0))
                                    .pb(px(8.0))
                                    .child(
                                        div()
                                            .w(px(40.0))
                                            .h(px(4.0))
                                            .bg(theme.tokens.muted.opacity(0.5))
                                            .rounded(px(2.0))
                                    )
                            )
                        })
                        .when(has_header, |this| {
                            this.child(
                                div()
                                    .flex()
                                    .items_start()
                                    .justify_between()
                                    .px(px(24.0))
                                    .pt(px(16.0))
                                    .pb(px(16.0))
                                    .border_b_1()
                                    .border_color(theme.tokens.border)
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(4.0))
                                            .when_some(self.title, |this: Div, title| {
                                                this.child(
                                                    Text::new(title)
                                                        .variant(TextVariant::H4)
                                                )
                                            })
                                            .when_some(self.description, |this: Div, desc| {
                                                this.child(
                                                    Text::new(desc)
                                                        .variant(TextVariant::Caption)
                                                        .color(theme.tokens.muted_foreground)
                                                )
                                            })
                                    )
                                    .when_some(self.actions, |this: Div, actions| {
                                        this.child(
                                            div()
                                                .flex()
                                                .items_center()
                                                .gap(px(8.0))
                                                .child(actions)
                                        )
                                    })
                            )
                        })
                        .when_some(self.content, |this, content| {
                            this.child(
                                div()
                                    .flex_1()
                                    .overflow_hidden()
                                    .child(content)
                            )
                        })
                        .on_mouse_down(MouseButton::Left, |_, _, _| {})
                        .with_animation(
                            "bottom-sheet-slide",
                            presets::slide_in_bottom(),
                            |div, delta| {
                                div.mb(px(-600.0 * (1.0 - delta)))
                            }
                        )
                )
        )
        .with_priority(1)
    }
}
