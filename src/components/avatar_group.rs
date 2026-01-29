use crate::components::avatar::{Avatar, AvatarSize};
use crate::components::tooltip::tooltip;
use crate::theme::use_theme;
use gpui::{prelude::FluentBuilder as _, *};

#[derive(Debug, Clone)]
pub struct AvatarItem {
    pub name: Option<SharedString>,
    pub src: Option<SharedString>,
    pub fallback_text: Option<SharedString>,
}

impl AvatarItem {
    pub fn new() -> Self {
        Self {
            name: None,
            src: None,
            fallback_text: None,
        }
    }

    pub fn name(mut self, name: impl Into<SharedString>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn src(mut self, src: impl Into<SharedString>) -> Self {
        self.src = Some(src.into());
        self
    }

    pub fn fallback_text(mut self, text: impl Into<SharedString>) -> Self {
        self.fallback_text = Some(text.into());
        self
    }
}

impl Default for AvatarItem {
    fn default() -> Self {
        Self::new()
    }
}

fn get_overlap(size: AvatarSize, spacing: Option<f32>) -> f32 {
    if let Some(spacing) = spacing {
        return spacing;
    }

    match size {
        AvatarSize::Xs => -8.0,
        AvatarSize::Sm => -10.0,
        AvatarSize::Md => -12.0,
        AvatarSize::Lg => -14.0,
        AvatarSize::Xl => -18.0,
    }
}

fn get_size_px(size: AvatarSize) -> f32 {
    match size {
        AvatarSize::Xs => 24.0,
        AvatarSize::Sm => 32.0,
        AvatarSize::Md => 40.0,
        AvatarSize::Lg => 48.0,
        AvatarSize::Xl => 64.0,
    }
}

fn get_text_size(size: AvatarSize) -> f32 {
    match size {
        AvatarSize::Xs => 9.0,
        AvatarSize::Sm => 11.0,
        AvatarSize::Md => 13.0,
        AvatarSize::Lg => 15.0,
        AvatarSize::Xl => 18.0,
    }
}

fn create_avatar(item: &AvatarItem, size: AvatarSize) -> Avatar {
    let mut avatar = Avatar::new().size(size);

    if let Some(ref src) = item.src {
        avatar = avatar.src(src.clone());
    }
    if let Some(ref name) = item.name {
        avatar = avatar.name(name.clone());
    }
    if let Some(ref fallback) = item.fallback_text {
        avatar = avatar.fallback_text(fallback.clone());
    }

    avatar
}

#[derive(IntoElement)]
pub struct AvatarGroup {
    items: Vec<AvatarItem>,
    size: AvatarSize,
    max_visible: Option<usize>,
    show_tooltips: bool,
    spacing: Option<f32>,
    style: StyleRefinement,
}

impl AvatarGroup {
    pub fn new(items: Vec<AvatarItem>) -> Self {
        Self {
            items,
            size: AvatarSize::default(),
            max_visible: None,
            show_tooltips: false,
            spacing: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn size(mut self, size: AvatarSize) -> Self {
        self.size = size;
        self
    }

    pub fn max_visible(mut self, max: usize) -> Self {
        self.max_visible = Some(max);
        self
    }

    pub fn show_tooltips(mut self, show: bool) -> Self {
        self.show_tooltips = show;
        self
    }

    pub fn spacing(mut self, spacing: Pixels) -> Self {
        self.spacing = Some(f32::from(spacing));
        self
    }
}

impl Default for AvatarGroup {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

impl Styled for AvatarGroup {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for AvatarGroup {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let theme = use_theme();

        let size = self.size;
        let show_tooltips = self.show_tooltips;
        let spacing = self.spacing;
        let max_visible = self.max_visible;
        let items = self.items;
        let user_style = self.style;

        let overlap = get_overlap(size, spacing);
        let size_px = get_size_px(size);
        let text_size = get_text_size(size);

        let total_count = items.len();
        let max_vis = max_visible.unwrap_or(total_count);
        let visible_count = max_vis.min(total_count);
        let overflow_count = total_count.saturating_sub(visible_count);

        let visible_items: Vec<_> = items.iter().take(visible_count).cloned().collect();
        let overflow_names: Vec<String> = items
            .iter()
            .skip(visible_count)
            .filter_map(|item| item.name.as_ref().map(|n| n.to_string()))
            .collect();

        div()
            .flex()
            .flex_row_reverse()
            .items_center()
            .map(|this| {
                let mut div = this;
                div.style().refine(&user_style);
                div
            })
            .when(overflow_count > 0, |this| {
                let overflow_indicator = div()
                    .relative()
                    .mr(px(overlap))
                    .size(px(size_px))
                    .flex()
                    .flex_shrink_0()
                    .items_center()
                    .justify_center()
                    .rounded_full()
                    .bg(theme.tokens.muted)
                    .text_color(theme.tokens.muted_foreground)
                    .text_size(px(text_size))
                    .font_weight(FontWeight::MEDIUM)
                    .font_family(theme.tokens.font_family.clone())
                    .border_2()
                    .border_color(theme.tokens.background)
                    .child(format!("+{}", overflow_count));

                if show_tooltips && !overflow_names.is_empty() {
                    let tooltip_content = overflow_names.join(", ");
                    this.child(tooltip(overflow_indicator, tooltip_content))
                } else {
                    this.child(overflow_indicator)
                }
            })
            .children(visible_items.iter().enumerate().rev().map(|(index, item)| {
                let avatar = create_avatar(item, size);
                let is_last_in_iteration = index == visible_count - 1;
                let margin_right = if is_last_in_iteration && overflow_count == 0 {
                    0.0
                } else {
                    overlap
                };

                let avatar_wrapper = div().relative().mr(px(margin_right)).child(avatar);

                if show_tooltips {
                    if let Some(ref name) = item.name {
                        tooltip(avatar_wrapper, name.clone()).into_any_element()
                    } else {
                        avatar_wrapper.into_any_element()
                    }
                } else {
                    avatar_wrapper.into_any_element()
                }
            }))
    }
}
