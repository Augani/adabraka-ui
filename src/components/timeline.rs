use crate::components::icon::Icon;
use crate::components::icon_source::IconSource;
use crate::theme::use_theme;
use gpui::{prelude::FluentBuilder as _, *};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum TimelineItemVariant {
    #[default]
    Default,
    Success,
    Warning,
    Error,
    Info,
}

impl TimelineItemVariant {
    fn default_icon(&self) -> &'static str {
        match self {
            TimelineItemVariant::Default => "circle",
            TimelineItemVariant::Success => "check-circle",
            TimelineItemVariant::Warning => "alert-triangle",
            TimelineItemVariant::Error => "alert-circle",
            TimelineItemVariant::Info => "info",
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum TimelineOrientation {
    #[default]
    Vertical,
    Horizontal,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum TimelineSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl TimelineSize {
    fn icon_size(&self) -> Pixels {
        match self {
            TimelineSize::Sm => px(14.0),
            TimelineSize::Md => px(18.0),
            TimelineSize::Lg => px(22.0),
        }
    }

    fn dot_size(&self) -> Pixels {
        match self {
            TimelineSize::Sm => px(8.0),
            TimelineSize::Md => px(12.0),
            TimelineSize::Lg => px(16.0),
        }
    }

    fn connector_width(&self) -> Pixels {
        match self {
            TimelineSize::Sm => px(1.0),
            TimelineSize::Md => px(2.0),
            TimelineSize::Lg => px(3.0),
        }
    }

    fn title_size(&self) -> f32 {
        match self {
            TimelineSize::Sm => 13.0,
            TimelineSize::Md => 14.0,
            TimelineSize::Lg => 16.0,
        }
    }

    fn description_size(&self) -> f32 {
        match self {
            TimelineSize::Sm => 12.0,
            TimelineSize::Md => 13.0,
            TimelineSize::Lg => 14.0,
        }
    }

    fn spacing(&self) -> Pixels {
        match self {
            TimelineSize::Sm => px(12.0),
            TimelineSize::Md => px(16.0),
            TimelineSize::Lg => px(20.0),
        }
    }

    fn item_gap(&self) -> Pixels {
        match self {
            TimelineSize::Sm => px(16.0),
            TimelineSize::Md => px(24.0),
            TimelineSize::Lg => px(32.0),
        }
    }
}

#[derive(Clone)]
pub struct TimelineItem {
    pub title: SharedString,
    pub description: Option<SharedString>,
    pub timestamp: Option<SharedString>,
    pub icon: Option<IconSource>,
    pub variant: TimelineItemVariant,
    pub collapsible: bool,
    pub collapsed: bool,
}

impl TimelineItem {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            description: None,
            timestamp: None,
            icon: None,
            variant: TimelineItemVariant::default(),
            collapsible: false,
            collapsed: false,
        }
    }

    pub fn description(mut self, desc: impl Into<SharedString>) -> Self {
        self.description = Some(desc.into());
        self
    }

    pub fn timestamp(mut self, ts: impl Into<SharedString>) -> Self {
        self.timestamp = Some(ts.into());
        self
    }

    pub fn icon(mut self, icon: impl Into<IconSource>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn variant(mut self, variant: TimelineItemVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn success(mut self) -> Self {
        self.variant = TimelineItemVariant::Success;
        self
    }

    pub fn warning(mut self) -> Self {
        self.variant = TimelineItemVariant::Warning;
        self
    }

    pub fn error(mut self) -> Self {
        self.variant = TimelineItemVariant::Error;
        self
    }

    pub fn info(mut self) -> Self {
        self.variant = TimelineItemVariant::Info;
        self
    }

    pub fn collapsible(mut self, collapsible: bool) -> Self {
        self.collapsible = collapsible;
        self
    }

    pub fn collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
        self.collapsible = true;
        self
    }

    fn get_color(&self, theme: &crate::theme::Theme) -> Hsla {
        match self.variant {
            TimelineItemVariant::Default => theme.tokens.muted_foreground,
            TimelineItemVariant::Success => rgb(0x22c55e).into(),
            TimelineItemVariant::Warning => rgb(0xf59e0b).into(),
            TimelineItemVariant::Error => theme.tokens.destructive,
            TimelineItemVariant::Info => theme.tokens.primary,
        }
    }
}

#[derive(IntoElement)]
pub struct Timeline {
    items: Vec<TimelineItem>,
    orientation: TimelineOrientation,
    size: TimelineSize,
    alternating: bool,
    show_icons: bool,
    style: StyleRefinement,
}

impl Timeline {
    pub fn new(items: Vec<TimelineItem>) -> Self {
        Self {
            items,
            orientation: TimelineOrientation::default(),
            size: TimelineSize::default(),
            alternating: false,
            show_icons: false,
            style: StyleRefinement::default(),
        }
    }

    pub fn vertical(items: Vec<TimelineItem>) -> Self {
        Self::new(items).orientation(TimelineOrientation::Vertical)
    }

    pub fn horizontal(items: Vec<TimelineItem>) -> Self {
        Self::new(items).orientation(TimelineOrientation::Horizontal)
    }

    pub fn orientation(mut self, orientation: TimelineOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn size(mut self, size: TimelineSize) -> Self {
        self.size = size;
        self
    }

    pub fn sm(mut self) -> Self {
        self.size = TimelineSize::Sm;
        self
    }

    pub fn md(mut self) -> Self {
        self.size = TimelineSize::Md;
        self
    }

    pub fn lg(mut self) -> Self {
        self.size = TimelineSize::Lg;
        self
    }

    pub fn alternating(mut self, alternating: bool) -> Self {
        self.alternating = alternating;
        self
    }

    pub fn show_icons(mut self, show: bool) -> Self {
        self.show_icons = show;
        self
    }
}

fn render_vertical_item(
    item: &TimelineItem,
    index: usize,
    is_last: bool,
    theme: &crate::theme::Theme,
    size: TimelineSize,
    alternating: bool,
    global_show_icons: bool,
) -> impl IntoElement {
    let item_color = item.get_color(theme);
    let is_right = alternating && index % 2 == 1;
    let show_icons = global_show_icons || item.icon.is_some();

    let indicator = if show_icons {
        let icon_source = item
            .icon
            .clone()
            .unwrap_or_else(|| IconSource::Named(item.variant.default_icon().into()));
        div()
            .flex()
            .items_center()
            .justify_center()
            .size(size.icon_size() + px(8.0))
            .rounded(px(9999.0))
            .bg(item_color.opacity(0.15))
            .border_2()
            .border_color(item_color)
            .child(
                Icon::new(icon_source)
                    .size(size.icon_size())
                    .color(item_color),
            )
            .into_any_element()
    } else {
        div()
            .size(size.dot_size())
            .rounded(px(9999.0))
            .bg(item_color)
            .border_2()
            .border_color(theme.tokens.background)
            .into_any_element()
    };

    let connector_height = size.item_gap();

    let content = div()
        .flex()
        .flex_col()
        .gap(px(4.0))
        .child(
            div()
                .flex()
                .items_center()
                .gap(px(8.0))
                .when(is_right, |this| this.flex_row_reverse())
                .child(
                    div()
                        .text_size(px(size.title_size()))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(theme.tokens.foreground)
                        .child(item.title.clone()),
                )
                .when_some(item.timestamp.clone(), |this, ts| {
                    this.child(
                        div()
                            .text_size(px(size.description_size() - 1.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child(ts),
                    )
                }),
        )
        .when_some(item.description.clone(), |this, desc| {
            this.when(!item.collapsed, |this| {
                this.child(
                    div()
                        .text_size(px(size.description_size()))
                        .text_color(theme.tokens.muted_foreground)
                        .child(desc),
                )
            })
        });

    div()
        .flex()
        .when(is_right, |this| this.flex_row_reverse())
        .gap(size.spacing())
        .child(
            div()
                .flex()
                .flex_col()
                .items_center()
                .child(indicator)
                .when(!is_last, |this| {
                    this.child(
                        div()
                            .w(size.connector_width())
                            .h(connector_height)
                            .bg(theme.tokens.border),
                    )
                }),
        )
        .child(
            div()
                .flex_1()
                .pb(if is_last { px(0.0) } else { size.item_gap() })
                .pt(if show_icons { px(4.0) } else { px(0.0) })
                .when(is_right, |this| this.items_end())
                .child(content),
        )
}

fn render_horizontal_item(
    item: &TimelineItem,
    index: usize,
    is_last: bool,
    theme: &crate::theme::Theme,
    size: TimelineSize,
    alternating: bool,
    global_show_icons: bool,
) -> impl IntoElement {
    let item_color = item.get_color(theme);
    let is_bottom = alternating && index % 2 == 1;
    let show_icons = global_show_icons || item.icon.is_some();

    let indicator = if show_icons {
        let icon_source = item
            .icon
            .clone()
            .unwrap_or_else(|| IconSource::Named(item.variant.default_icon().into()));
        div()
            .flex()
            .items_center()
            .justify_center()
            .size(size.icon_size() + px(8.0))
            .rounded(px(9999.0))
            .bg(item_color.opacity(0.15))
            .border_2()
            .border_color(item_color)
            .child(
                Icon::new(icon_source)
                    .size(size.icon_size())
                    .color(item_color),
            )
            .into_any_element()
    } else {
        div()
            .size(size.dot_size())
            .rounded(px(9999.0))
            .bg(item_color)
            .border_2()
            .border_color(theme.tokens.background)
            .into_any_element()
    };

    let connector_width = size.item_gap();

    let content = div()
        .flex()
        .flex_col()
        .items_center()
        .gap(px(4.0))
        .max_w(px(120.0))
        .child(
            div()
                .text_size(px(size.title_size()))
                .font_weight(FontWeight::MEDIUM)
                .text_color(theme.tokens.foreground)
                .text_center()
                .child(item.title.clone()),
        )
        .when_some(item.timestamp.clone(), |this, ts| {
            this.child(
                div()
                    .text_size(px(size.description_size() - 1.0))
                    .text_color(theme.tokens.muted_foreground)
                    .text_center()
                    .child(ts),
            )
        })
        .when_some(item.description.clone(), |this, desc| {
            this.when(!item.collapsed, |this| {
                this.child(
                    div()
                        .text_size(px(size.description_size()))
                        .text_color(theme.tokens.muted_foreground)
                        .text_center()
                        .child(desc),
                )
            })
        });

    div()
        .flex()
        .flex_col()
        .when(is_bottom, |this| this.flex_col_reverse())
        .gap(size.spacing())
        .child(content)
        .child(
            div()
                .flex()
                .items_center()
                .child(indicator)
                .when(!is_last, |this| {
                    this.child(
                        div()
                            .h(size.connector_width())
                            .w(connector_width)
                            .bg(theme.tokens.border),
                    )
                }),
        )
}

impl Styled for Timeline {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Timeline {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let items = self.items;
        let orientation = self.orientation;
        let size = self.size;
        let alternating = self.alternating;
        let show_icons = self.show_icons;
        let user_style = self.style;
        let items_len = items.len();

        let container = match orientation {
            TimelineOrientation::Vertical => div().flex().flex_col().w_full(),
            TimelineOrientation::Horizontal => div().flex().flex_row().items_start(),
        };

        container
            .children(items.iter().enumerate().map(|(i, item)| {
                let is_last = i == items_len - 1;
                match orientation {
                    TimelineOrientation::Vertical => render_vertical_item(
                        item,
                        i,
                        is_last,
                        &theme,
                        size,
                        alternating,
                        show_icons,
                    )
                    .into_any_element(),
                    TimelineOrientation::Horizontal => render_horizontal_item(
                        item,
                        i,
                        is_last,
                        &theme,
                        size,
                        alternating,
                        show_icons,
                    )
                    .into_any_element(),
                }
            }))
            .map(|this| {
                let mut div = this;
                div.style().refine(&user_style);
                div
            })
    }
}

pub fn timeline(items: Vec<TimelineItem>) -> Timeline {
    Timeline::new(items)
}
