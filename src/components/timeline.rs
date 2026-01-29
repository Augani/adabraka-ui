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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum TimelineLayout {
    #[default]
    Left,
    Right,
    Center,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum TimelineConnectorStyle {
    #[default]
    Solid,
    Dashed,
    None,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum TimelineIndicatorStyle {
    #[default]
    Dot,
    Icon,
    Number,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TimelineItemPosition {
    Left,
    Right,
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
            TimelineSize::Sm => px(10.0),
            TimelineSize::Md => px(14.0),
            TimelineSize::Lg => px(18.0),
        }
    }

    fn number_size(&self) -> Pixels {
        match self {
            TimelineSize::Sm => px(20.0),
            TimelineSize::Md => px(26.0),
            TimelineSize::Lg => px(32.0),
        }
    }

    fn connector_width(&self) -> Pixels {
        match self {
            TimelineSize::Sm => px(2.0),
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
            TimelineSize::Sm => px(20.0),
            TimelineSize::Md => px(28.0),
            TimelineSize::Lg => px(36.0),
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
    pub position: Option<TimelineItemPosition>,
}

impl TimelineItem {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            description: None,
            timestamp: None,
            icon: None,
            variant: TimelineItemVariant::default(),
            position: None,
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

    pub fn position(mut self, position: TimelineItemPosition) -> Self {
        self.position = Some(position);
        self
    }

    pub fn left(mut self) -> Self {
        self.position = Some(TimelineItemPosition::Left);
        self
    }

    pub fn right(mut self) -> Self {
        self.position = Some(TimelineItemPosition::Right);
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
    layout: TimelineLayout,
    alternating: bool,
    indicator_style: TimelineIndicatorStyle,
    connector_style: TimelineConnectorStyle,
    connector_color: Option<Hsla>,
    content_width: Option<Pixels>,
    style: StyleRefinement,
}

impl Timeline {
    pub fn new(items: Vec<TimelineItem>) -> Self {
        Self {
            items,
            orientation: TimelineOrientation::default(),
            size: TimelineSize::default(),
            layout: TimelineLayout::default(),
            alternating: false,
            indicator_style: TimelineIndicatorStyle::default(),
            connector_style: TimelineConnectorStyle::default(),
            connector_color: None,
            content_width: None,
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

    pub fn layout(mut self, layout: TimelineLayout) -> Self {
        self.layout = layout;
        self
    }

    pub fn left_layout(mut self) -> Self {
        self.layout = TimelineLayout::Left;
        self
    }

    pub fn right_layout(mut self) -> Self {
        self.layout = TimelineLayout::Right;
        self
    }

    pub fn center_layout(mut self) -> Self {
        self.layout = TimelineLayout::Center;
        self
    }

    pub fn alternating(mut self, alternating: bool) -> Self {
        self.alternating = alternating;
        if alternating {
            self.layout = TimelineLayout::Center;
        }
        self
    }

    pub fn indicator_style(mut self, style: TimelineIndicatorStyle) -> Self {
        self.indicator_style = style;
        self
    }

    pub fn dot_indicators(mut self) -> Self {
        self.indicator_style = TimelineIndicatorStyle::Dot;
        self
    }

    pub fn icon_indicators(mut self) -> Self {
        self.indicator_style = TimelineIndicatorStyle::Icon;
        self
    }

    pub fn number_indicators(mut self) -> Self {
        self.indicator_style = TimelineIndicatorStyle::Number;
        self
    }

    pub fn connector_style(mut self, style: TimelineConnectorStyle) -> Self {
        self.connector_style = style;
        self
    }

    pub fn solid_connector(mut self) -> Self {
        self.connector_style = TimelineConnectorStyle::Solid;
        self
    }

    pub fn dashed_connector(mut self) -> Self {
        self.connector_style = TimelineConnectorStyle::Dashed;
        self
    }

    pub fn no_connector(mut self) -> Self {
        self.connector_style = TimelineConnectorStyle::None;
        self
    }

    pub fn connector_color(mut self, color: Hsla) -> Self {
        self.connector_color = Some(color);
        self
    }

    pub fn content_width(mut self, width: Pixels) -> Self {
        self.content_width = Some(width);
        self
    }

    #[deprecated(note = "Use indicator_style(TimelineIndicatorStyle::Icon) instead")]
    pub fn show_icons(mut self, show: bool) -> Self {
        if show {
            self.indicator_style = TimelineIndicatorStyle::Icon;
        }
        self
    }
}

impl Timeline {
    fn render_indicator(
        &self,
        item: &TimelineItem,
        index: usize,
        theme: &crate::theme::Theme,
    ) -> AnyElement {
        let item_color = item.get_color(theme);
        let size = self.size;

        match self.indicator_style {
            TimelineIndicatorStyle::Dot => div()
                .size(size.dot_size())
                .rounded_full()
                .bg(item_color)
                .flex_shrink_0()
                .into_any_element(),

            TimelineIndicatorStyle::Icon => {
                let icon_source = item
                    .icon
                    .clone()
                    .unwrap_or_else(|| IconSource::Named(item.variant.default_icon().into()));
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .size(size.icon_size() + px(10.0))
                    .rounded_full()
                    .bg(item_color.opacity(0.15))
                    .border_2()
                    .border_color(item_color)
                    .flex_shrink_0()
                    .child(
                        Icon::new(icon_source)
                            .size(size.icon_size())
                            .color(item_color),
                    )
                    .into_any_element()
            }

            TimelineIndicatorStyle::Number => div()
                .flex()
                .items_center()
                .justify_center()
                .size(size.number_size())
                .rounded_full()
                .bg(item_color)
                .flex_shrink_0()
                .text_size(px(size.title_size() - 2.0))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(theme.tokens.background)
                .child(format!("{}", index + 1))
                .into_any_element(),
        }
    }

    fn render_connector(&self, theme: &crate::theme::Theme, length: Pixels) -> Option<AnyElement> {
        if self.connector_style == TimelineConnectorStyle::None {
            return None;
        }

        let color = self.connector_color.unwrap_or(theme.tokens.border);
        let width = self.size.connector_width();

        let connector = match self.connector_style {
            TimelineConnectorStyle::Solid => div().w(width).h(length).bg(color).into_any_element(),
            TimelineConnectorStyle::Dashed => {
                let num_dashes = ((length / px(8.0)) as usize).max(1);
                div()
                    .w(width)
                    .h(length)
                    .flex()
                    .flex_col()
                    .gap(px(4.0))
                    .overflow_hidden()
                    .children(
                        (0..num_dashes)
                            .map(|_| div().w(width).h(px(4.0)).bg(color).into_any_element()),
                    )
                    .into_any_element()
            }
            TimelineConnectorStyle::None => return None,
        };

        Some(connector)
    }

    fn render_content(
        &self,
        item: &TimelineItem,
        theme: &crate::theme::Theme,
        align_right: bool,
    ) -> AnyElement {
        let size = self.size;

        div()
            .flex()
            .flex_col()
            .gap(px(4.0))
            .when(align_right, |d| d.items_end())
            .when_some(self.content_width, |d, w| d.w(w))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(2.0))
                    .when(align_right, |d| d.items_end())
                    .child(
                        div()
                            .text_size(px(size.title_size()))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(theme.tokens.foreground)
                            .when(align_right, |d| d.text_align(TextAlign::Right))
                            .child(item.title.clone()),
                    )
                    .when_some(item.timestamp.clone(), |d, ts| {
                        d.child(
                            div()
                                .text_size(px(size.description_size() - 1.0))
                                .text_color(theme.tokens.muted_foreground)
                                .when(align_right, |d| d.text_align(TextAlign::Right))
                                .child(ts),
                        )
                    }),
            )
            .when_some(item.description.clone(), |d, desc| {
                d.child(
                    div()
                        .text_size(px(size.description_size()))
                        .text_color(theme.tokens.muted_foreground)
                        .when(align_right, |d| d.text_align(TextAlign::Right))
                        .child(desc),
                )
            })
            .into_any_element()
    }

    fn render_vertical_left_item(
        &self,
        item: &TimelineItem,
        index: usize,
        is_last: bool,
        theme: &crate::theme::Theme,
    ) -> AnyElement {
        let indicator = self.render_indicator(item, index, theme);
        let connector = if !is_last {
            self.render_connector(theme, self.size.item_gap())
        } else {
            None
        };
        let content = self.render_content(item, theme, false);

        div()
            .flex()
            .gap(self.size.spacing())
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .child(indicator)
                    .when_some(connector, |d, c| d.child(c)),
            )
            .child(
                div()
                    .flex_1()
                    .pb(if is_last {
                        px(0.0)
                    } else {
                        self.size.item_gap()
                    })
                    .child(content),
            )
            .into_any_element()
    }

    fn render_vertical_right_item(
        &self,
        item: &TimelineItem,
        index: usize,
        is_last: bool,
        theme: &crate::theme::Theme,
    ) -> AnyElement {
        let indicator = self.render_indicator(item, index, theme);
        let connector = if !is_last {
            self.render_connector(theme, self.size.item_gap())
        } else {
            None
        };
        let content = self.render_content(item, theme, true);

        div()
            .flex()
            .flex_row_reverse()
            .gap(self.size.spacing())
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .child(indicator)
                    .when_some(connector, |d, c| d.child(c)),
            )
            .child(
                div()
                    .flex_1()
                    .pb(if is_last {
                        px(0.0)
                    } else {
                        self.size.item_gap()
                    })
                    .child(content),
            )
            .into_any_element()
    }

    fn render_vertical_center_item(
        &self,
        item: &TimelineItem,
        index: usize,
        is_last: bool,
        theme: &crate::theme::Theme,
        on_left: bool,
    ) -> AnyElement {
        let indicator = self.render_indicator(item, index, theme);
        let connector = if !is_last {
            self.render_connector(theme, self.size.item_gap())
        } else {
            None
        };
        let content = self.render_content(item, theme, on_left);

        let left_content = if on_left { Some(content) } else { None };

        let right_content = if !on_left {
            Some(self.render_content(item, theme, false))
        } else {
            None
        };

        div()
            .flex()
            .w_full()
            .child(
                div()
                    .flex_1()
                    .flex()
                    .justify_end()
                    .pr(self.size.spacing())
                    .when_some(left_content, |d, c| d.child(c)),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .child(indicator)
                    .when_some(connector, |d, c| d.child(c)),
            )
            .child(
                div()
                    .flex_1()
                    .pl(self.size.spacing())
                    .pb(if is_last {
                        px(0.0)
                    } else {
                        self.size.item_gap()
                    })
                    .when_some(right_content, |d, c| d.child(c)),
            )
            .into_any_element()
    }

    fn render_horizontal_item(
        &self,
        item: &TimelineItem,
        index: usize,
        is_last: bool,
        theme: &crate::theme::Theme,
    ) -> AnyElement {
        let indicator = self.render_indicator(item, index, theme);
        let content = div()
            .flex()
            .flex_col()
            .items_center()
            .gap(px(4.0))
            .max_w(px(120.0))
            .child(
                div()
                    .text_size(px(self.size.title_size()))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(theme.tokens.foreground)
                    .text_center()
                    .child(item.title.clone()),
            )
            .when_some(item.timestamp.clone(), |d, ts| {
                d.child(
                    div()
                        .text_size(px(self.size.description_size() - 1.0))
                        .text_color(theme.tokens.muted_foreground)
                        .text_center()
                        .child(ts),
                )
            })
            .when_some(item.description.clone(), |d, desc| {
                d.child(
                    div()
                        .text_size(px(self.size.description_size()))
                        .text_color(theme.tokens.muted_foreground)
                        .text_center()
                        .child(desc),
                )
            });

        let connector = if !is_last {
            self.render_connector(theme, self.size.item_gap()).map(|_| {
                div()
                    .h(self.size.connector_width())
                    .w(self.size.item_gap())
                    .bg(self.connector_color.unwrap_or(theme.tokens.border))
                    .into_any_element()
            })
        } else {
            None
        };

        div()
            .flex()
            .flex_col()
            .items_center()
            .gap(self.size.spacing())
            .child(content)
            .child(
                div()
                    .flex()
                    .items_center()
                    .child(indicator)
                    .when_some(connector, |d, c| d.child(c)),
            )
            .into_any_element()
    }
}

impl Styled for Timeline {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Timeline {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let items = self.items.clone();
        let items_len = items.len();
        let user_style = self.style.clone();

        let container = match self.orientation {
            TimelineOrientation::Vertical => div().flex().flex_col().w_full(),
            TimelineOrientation::Horizontal => div().flex().flex_row().items_start(),
        };

        container
            .children(items.iter().enumerate().map(|(i, item)| {
                let is_last = i == items_len - 1;

                match self.orientation {
                    TimelineOrientation::Vertical => {
                        let item_position = item.position.unwrap_or_else(|| {
                            if self.alternating {
                                if i % 2 == 0 {
                                    TimelineItemPosition::Left
                                } else {
                                    TimelineItemPosition::Right
                                }
                            } else {
                                TimelineItemPosition::Left
                            }
                        });

                        match self.layout {
                            TimelineLayout::Left => {
                                self.render_vertical_left_item(item, i, is_last, &theme)
                            }
                            TimelineLayout::Right => {
                                self.render_vertical_right_item(item, i, is_last, &theme)
                            }
                            TimelineLayout::Center => {
                                let on_left = item_position == TimelineItemPosition::Left;
                                self.render_vertical_center_item(item, i, is_last, &theme, on_left)
                            }
                        }
                    }
                    TimelineOrientation::Horizontal => {
                        self.render_horizontal_item(item, i, is_last, &theme)
                    }
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
