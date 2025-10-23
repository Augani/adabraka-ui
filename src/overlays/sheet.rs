//! Sheet component for slide-in side panels.

use gpui::{prelude::FluentBuilder as _, *};
use std::rc::Rc;

use crate::theme::use_theme;
use crate::components::button::{Button, ButtonVariant, ButtonSize};

actions!(sheet, [SheetClose]);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SheetSide {
    Left,
    Right,
    Top,
    Bottom,
}

impl Default for SheetSide {
    fn default() -> Self {
        Self::Right
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SheetSize {
    Sm,
    Md,
    Lg,
    Xl,
    Custom,
}

impl Default for SheetSize {
    fn default() -> Self {
        Self::Md
    }
}

impl SheetSize {
    fn size(&self) -> Pixels {
        match self {
            Self::Sm => px(320.0),
            Self::Md => px(400.0),
            Self::Lg => px(500.0),
            Self::Xl => px(640.0),
            Self::Custom => px(400.0),
        }
    }
}

pub struct Sheet {
    focus_handle: FocusHandle,
    side: SheetSide,
    size: SheetSize,
    custom_width: Option<Pixels>,
    custom_height: Option<Pixels>,
    title: Option<SharedString>,
    description: Option<SharedString>,
    content: Option<AnyElement>,
    footer: Option<AnyElement>,
    show_close_button: bool,
    close_on_backdrop_click: bool,
    on_close: Option<Rc<dyn Fn(&mut Window, &mut App)>>,
    style: StyleRefinement,
}

impl Sheet {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            side: SheetSide::default(),
            size: SheetSize::default(),
            custom_width: None,
            custom_height: None,
            title: None,
            description: None,
            content: None,
            footer: None,
            show_close_button: true,
            close_on_backdrop_click: true,
            on_close: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn side(mut self, side: SheetSide) -> Self {
        self.side = side;
        self
    }

    pub fn size(mut self, size: SheetSize) -> Self {
        self.size = size;
        self
    }

    pub fn width(mut self, width: impl Into<Pixels>) -> Self {
        self.custom_width = Some(width.into());
        self.size = SheetSize::Custom;
        self
    }

    pub fn height(mut self, height: impl Into<Pixels>) -> Self {
        self.custom_height = Some(height.into());
        self.size = SheetSize::Custom;
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

    pub fn footer(mut self, footer: impl IntoElement) -> Self {
        self.footer = Some(footer.into_any_element());
        self
    }

    pub fn show_close_button(mut self, show: bool) -> Self {
        self.show_close_button = show;
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

    fn handle_close(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(handler) = &self.on_close {
            handler(window, cx);
        }
    }

    fn handle_escape(&mut self, _: &SheetClose, window: &mut Window, cx: &mut Context<Self>) {
        self.handle_close(window, cx);
    }

    fn get_sheet_size(&self) -> Pixels {
        if let Some(width) = self.custom_width {
            return width;
        }
        if let Some(height) = self.custom_height {
            return height;
        }
        self.size.size()
    }
}

pub fn init_sheet(_cx: &mut App) {
}

impl Styled for Sheet {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl Render for Sheet {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();
        let has_header = self.title.is_some() || self.description.is_some() || self.show_close_button;
        let sheet_size = self.get_sheet_size();
        let user_style = self.style.clone();

        div()
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::handle_escape))
            .absolute()
            .inset_0()
            .flex()
            .bg(hsla(0.0, 0.0, 0.0, 0.5))
            .when(self.close_on_backdrop_click, |this: Div| {
                this.on_mouse_down(MouseButton::Left, cx.listener(|this, _, window, cx| {
                    this.handle_close(window, cx);
                }))
            })
            .child(
                div()
                    .occlude()
                    .flex()
                    .flex_col()
                    .bg(theme.tokens.background)
                    .border_color(theme.tokens.border)
                    .shadow(vec![BoxShadow {
                        color: hsla(0.0, 0.0, 0.0, 0.2),
                        offset: point(px(0.0), px(0.0)),
                        blur_radius: px(16.0),
                        spread_radius: px(0.0),
                    }])
                    .on_mouse_down(MouseButton::Left, |_, _, _| {})
                    .when(self.side == SheetSide::Right, |this: Div| {
                        this.absolute()
                            .right_0()
                            .top_0()
                            .bottom_0()
                            .w(sheet_size)
                            .border_l_1()
                    })
                    .when(self.side == SheetSide::Left, |this: Div| {
                        this.absolute()
                            .left_0()
                            .top_0()
                            .bottom_0()
                            .w(sheet_size)
                            .border_r_1()
                    })
                    .when(self.side == SheetSide::Top, |this: Div| {
                        this.absolute()
                            .top_0()
                            .left_0()
                            .right_0()
                            .h(sheet_size)
                            .border_b_1()
                    })
                    .when(self.side == SheetSide::Bottom, |this: Div| {
                        this.absolute()
                            .bottom_0()
                            .left_0()
                            .right_0()
                            .h(sheet_size)
                            .border_t_1()
                    })
                    .when(has_header, |this: Div| {
                        this.child(
                            div()
                                .flex()
                                .items_start()
                                .justify_between()
                                .px(px(24.0))
                                .pt(px(24.0))
                                .pb(px(20.0))
                                .border_b_1()
                                .border_color(theme.tokens.border)
                                .child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .gap(px(4.0))
                                        .when_some(self.title.clone(), |this: Div, title| {
                                            this.child(
                                                div()
                                                    .text_size(px(18.0))
                                                    .font_weight(FontWeight::SEMIBOLD)
                                                    .text_color(theme.tokens.foreground)
                                                    .child(title)
                                            )
                                        })
                                        .when_some(self.description.clone(), |this: Div, desc| {
                                            this.child(
                                                div()
                                                    .text_size(px(14.0))
                                                    .text_color(theme.tokens.muted_foreground)
                                                    .child(desc)
                                            )
                                        })
                                )
                                .when(self.show_close_button, |this: Div| {
                                    this.child(
                                        Button::new("sheet-close-btn", "×")
                                            .variant(ButtonVariant::Ghost)
                                            .size(ButtonSize::Sm)
                                            .on_click(cx.listener(|this, _, window, cx| {
                                                this.handle_close(window, cx);
                                            }))
                                    )
                                })
                        )
                    })
                    .when_some(self.content.take(), |this: Div, content| {
                        this.child(
                            div()
                                .flex_1()
                                .overflow_hidden()
                                .child(content)
                        )
                    })
                    .map(|this| {
                        let mut div = this;
                        div.style().refine(&user_style);
                        div
                    })
                    .when_some(self.footer.take(), |this: Div, footer| {
                        this.child(
                            div()
                                .px(px(24.0))
                                .py(px(16.0))
                                .border_t_1()
                                .border_color(theme.tokens.border)
                                .child(footer)
                        )
                    })
            )
    }
}
