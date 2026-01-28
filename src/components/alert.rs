use crate::components::icon::Icon;
use crate::components::icon_source::IconSource;
use crate::theme::use_theme;
use gpui::{prelude::FluentBuilder as _, *};
use std::rc::Rc;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum AlertVariant {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

impl AlertVariant {
    fn default_icon(&self) -> &'static str {
        match self {
            AlertVariant::Info => "info",
            AlertVariant::Success => "check-circle",
            AlertVariant::Warning => "alert-triangle",
            AlertVariant::Error => "alert-circle",
        }
    }
}

#[derive(IntoElement)]
pub struct Alert {
    variant: AlertVariant,
    title: Option<SharedString>,
    description: Option<SharedString>,
    icon: Option<IconSource>,
    show_icon: bool,
    dismissible: bool,
    on_dismiss: Option<Rc<dyn Fn(&mut Window, &mut App)>>,
    action: Option<(SharedString, Rc<dyn Fn(&mut Window, &mut App)>)>,
    style: StyleRefinement,
}

impl Alert {
    pub fn new() -> Self {
        Self {
            variant: AlertVariant::default(),
            title: None,
            description: None,
            icon: None,
            show_icon: true,
            dismissible: false,
            on_dismiss: None,
            action: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn info() -> Self {
        Self::new().variant(AlertVariant::Info)
    }

    pub fn success() -> Self {
        Self::new().variant(AlertVariant::Success)
    }

    pub fn warning() -> Self {
        Self::new().variant(AlertVariant::Warning)
    }

    pub fn error() -> Self {
        Self::new().variant(AlertVariant::Error)
    }

    pub fn variant(mut self, variant: AlertVariant) -> Self {
        self.variant = variant;
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

    pub fn icon(mut self, icon: impl Into<IconSource>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn show_icon(mut self, show: bool) -> Self {
        self.show_icon = show;
        self
    }

    pub fn dismissible(mut self, dismissible: bool) -> Self {
        self.dismissible = dismissible;
        self
    }

    pub fn on_dismiss(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_dismiss = Some(Rc::new(handler));
        self.dismissible = true;
        self
    }

    pub fn action(
        mut self,
        label: impl Into<SharedString>,
        handler: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        self.action = Some((label.into(), Rc::new(handler)));
        self
    }

    fn get_colors(&self, theme: &crate::theme::Theme) -> (Hsla, Hsla, Hsla) {
        match self.variant {
            AlertVariant::Info => (
                theme.tokens.primary.opacity(0.1),
                theme.tokens.primary,
                theme.tokens.primary,
            ),
            AlertVariant::Success => {
                let success_color: Hsla = rgb(0x22c55e).into();
                (success_color.opacity(0.1), success_color, success_color)
            }
            AlertVariant::Warning => {
                let warning_color: Hsla = rgb(0xf59e0b).into();
                (warning_color.opacity(0.1), warning_color, warning_color)
            }
            AlertVariant::Error => (
                theme.tokens.destructive.opacity(0.1),
                theme.tokens.destructive,
                theme.tokens.destructive,
            ),
        }
    }
}

impl Default for Alert {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for Alert {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Alert {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let (bg_color, border_color, accent_color) = self.get_colors(&theme);
        let user_style = self.style;

        let icon_source = self
            .icon
            .unwrap_or_else(|| IconSource::Named(self.variant.default_icon().into()));

        let has_content = self.title.is_some() || self.description.is_some();

        div()
            .flex()
            .w_full()
            .p(px(16.0))
            .rounded(theme.tokens.radius_md)
            .bg(bg_color)
            .border_1()
            .border_color(border_color.opacity(0.3))
            .gap(px(12.0))
            .when(self.show_icon, |this| {
                this.child(
                    div()
                        .flex_shrink_0()
                        .pt(px(2.0))
                        .child(Icon::new(icon_source).size(px(20.0)).color(accent_color)),
                )
            })
            .when(has_content, |this| {
                this.child(
                    div()
                        .flex()
                        .flex_col()
                        .flex_1()
                        .gap(px(4.0))
                        .when_some(self.title.clone(), |this, title| {
                            this.child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(theme.tokens.foreground)
                                    .child(title),
                            )
                        })
                        .when_some(self.description.clone(), |this, desc| {
                            this.child(
                                div()
                                    .text_sm()
                                    .text_color(theme.tokens.muted_foreground)
                                    .child(desc),
                            )
                        })
                        .when_some(self.action.clone(), |this, (label, handler)| {
                            this.child(
                                div().mt(px(8.0)).child(
                                    div()
                                        .id("alert-action")
                                        .text_sm()
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(accent_color)
                                        .cursor(CursorStyle::PointingHand)
                                        .hover(|style| style.opacity(0.8))
                                        .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                                            (handler)(window, cx);
                                        })
                                        .child(label),
                                ),
                            )
                        }),
                )
            })
            .when(self.dismissible, |this| {
                let dismiss_handler = self.on_dismiss.clone();
                this.child(
                    div()
                        .flex_shrink_0()
                        .id("alert-dismiss")
                        .cursor(CursorStyle::PointingHand)
                        .rounded(theme.tokens.radius_sm)
                        .p(px(4.0))
                        .hover(|style| style.bg(theme.tokens.muted.opacity(0.5)))
                        .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                            if let Some(ref handler) = dismiss_handler {
                                (handler)(window, cx);
                            }
                        })
                        .child(
                            Icon::new("x")
                                .size(px(16.0))
                                .color(theme.tokens.muted_foreground),
                        ),
                )
            })
            .map(|this| {
                let mut div = this;
                div.style().refine(&user_style);
                div
            })
    }
}

pub fn alert() -> Alert {
    Alert::new()
}
