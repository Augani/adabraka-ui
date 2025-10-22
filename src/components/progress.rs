use gpui::{prelude::FluentBuilder as _, *};
use crate::theme::use_theme;

/// Progress bar variants
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ProgressVariant {
    /// Default blue progress bar
    Default,
    /// Success/complete state (green)
    Success,
    /// Warning state (yellow/orange)
    Warning,
    /// Error/failure state (red)
    Destructive,
}

/// Progress bar sizes
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ProgressSize {
    /// Thin progress bar (h-1)
    Sm,
    /// Default height (h-2)
    Md,
    /// Larger height (h-3)
    Lg,
}

/// Spinner types for circular progress indicators
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SpinnerType {
    /// Single orbiting dot on a track
    Dot,
    /// Arc segment (longer than dot) on a track
    Arc,
    /// Arc segment rotating without visible track
    ArcNoTrack,
    /// Growing circle that fills from dot to complete circle (for determinate progress)
    GrowingCircle,
}

/// Progress bar component with determinate and indeterminate modes
#[derive(IntoElement)]
pub struct ProgressBar {
    /// Progress value (0.0 to 1.0 for determinate, None for indeterminate)
    value: Option<f32>,
    variant: ProgressVariant,
    size: ProgressSize,
    /// Optional label to show percentage or custom text
    label: Option<SharedString>,
    /// Show percentage text overlay
    show_percentage: bool,
    style: StyleRefinement,
}

impl ProgressBar {
    /// Create a new progress bar with a value (0.0 to 1.0)
    pub fn new(value: f32) -> Self {
        Self {
            value: Some(value.clamp(0.0, 1.0)),
            variant: ProgressVariant::Default,
            size: ProgressSize::Md,
            label: None,
            show_percentage: false,
            style: StyleRefinement::default(),
        }
    }

    /// Create an indeterminate progress bar (loading animation)
    pub fn indeterminate() -> Self {
        Self {
            value: None,
            variant: ProgressVariant::Default,
            size: ProgressSize::Md,
            label: None,
            show_percentage: false,
            style: StyleRefinement::default(),
        }
    }

    /// Set the progress variant
    pub fn variant(mut self, variant: ProgressVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the progress size
    pub fn size(mut self, size: ProgressSize) -> Self {
        self.size = size;
        self
    }

    /// Set a custom label
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Show percentage text (only for determinate progress)
    pub fn show_percentage(mut self, show: bool) -> Self {
        self.show_percentage = show;
        self
    }
}

impl Styled for ProgressBar {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for ProgressBar {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let user_style = self.style;

        let height = match self.size {
            ProgressSize::Sm => px(4.0),
            ProgressSize::Md => px(8.0),
            ProgressSize::Lg => px(12.0),
        };

        let bar_color = match self.variant {
            ProgressVariant::Default => theme.tokens.primary,
            ProgressVariant::Success => rgb(0x22c55e).into(), // green-500
            ProgressVariant::Warning => rgb(0xf59e0b).into(), // amber-500
            ProgressVariant::Destructive => theme.tokens.destructive,
        };

        let progress_width = if let Some(value) = self.value {
            relative(value)
        } else {
            relative(0.3) // Indeterminate shows 30% width animated
        };

        let percentage_text = self.value.map(|v| format!("{}%", (v * 100.0) as u32));

        div()
            .flex()
            .flex_col()
            .gap(px(8.0))
            .w_full()
            .when(self.label.is_some() || (self.show_percentage && percentage_text.is_some()), |this| {
                this.child(
                    div()
                        .flex()
                        .justify_between()
                        .items_center()
                        .when_some(self.label, |this, label| {
                            this.child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(theme.tokens.foreground)
                                    .child(label)
                            )
                        })
                        .when(self.show_percentage && percentage_text.is_some(), |this| {
                            this.child(
                                div()
                                    .text_sm()
                                    .text_color(theme.tokens.muted_foreground)
                                    .child(percentage_text.unwrap_or_default())
                            )
                        })
                )
            })
            .child(
                div()
                    .relative()
                    .w_full()
                    .h(height)
                    .rounded(theme.tokens.radius_lg)
                    .bg(theme.tokens.muted)
                    .overflow_hidden()
                    .child(
                        div()
                            .absolute()
                            .top_0()
                            .left_0()
                            .h_full()
                            .w(progress_width)
                            .bg(bar_color)
                            .rounded(theme.tokens.radius_lg)
                            .map(|this| {
                                if self.value.is_none() {
                                    this.with_animation(
                                        "indeterminate-progress",
                                        Animation::new(std::time::Duration::from_secs(2))
                                            .repeat()
                                            .with_easing(crate::animations::easings::linear),
                                        |div, delta| {
                                            let offset = (delta - 0.5) * 2.0;
                                            div.left(relative(offset))
                                        }
                                    )
                                    .into_any_element()
                                } else {
                                    this.into_any_element()
                                }
                            })
                    )
            )
            .map(|this| {
                let mut div = this;
                div.style().refine(&user_style);
                div
            })
    }
}

/// Circular progress/spinner component
#[derive(IntoElement)]
pub struct CircularProgress {
    /// Progress value (0.0 to 1.0 for determinate, None for indeterminate)
    value: Option<f32>,
    size: Pixels,
    stroke_width: Pixels,
    variant: ProgressVariant,
    spinner_type: SpinnerType,
    style: StyleRefinement,
}

impl CircularProgress {
    /// Create a new circular progress with a value
    pub fn new(value: f32) -> Self {
        Self {
            value: Some(value.clamp(0.0, 1.0)),
            size: px(40.0),
            stroke_width: px(4.0),
            variant: ProgressVariant::Default,
            spinner_type: SpinnerType::Dot,
            style: StyleRefinement::default(),
        }
    }

    /// Create an indeterminate circular progress (spinner)
    pub fn indeterminate() -> Self {
        Self {
            value: None,
            size: px(40.0),
            stroke_width: px(4.0),
            variant: ProgressVariant::Default,
            spinner_type: SpinnerType::Dot,
            style: StyleRefinement::default(),
        }
    }

    /// Set the size
    pub fn size(mut self, size: Pixels) -> Self {
        self.size = size;
        self.stroke_width = size * 0.1; // Stroke is 10% of size
        self
    }

    /// Set the variant
    pub fn variant(mut self, variant: ProgressVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the spinner type
    pub fn spinner_type(mut self, spinner_type: SpinnerType) -> Self {
        self.spinner_type = spinner_type;
        self
    }
}

impl Styled for CircularProgress {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for CircularProgress {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let user_style = self.style;

        let stroke_color = match self.variant {
            ProgressVariant::Default => theme.tokens.primary,
            ProgressVariant::Success => rgb(0x22c55e).into(),
            ProgressVariant::Warning => rgb(0xf59e0b).into(),
            ProgressVariant::Destructive => theme.tokens.destructive,
        };

        div()
            .flex()
            .items_center()
            .justify_center()
            .size(self.size)
            .child(
                div()
                    .size(self.size)
                    .rounded(px(9999.0))
                    .relative()
                    .map(|container| {
                        let track_color = theme.tokens.muted;
                        let stroke_w = self.stroke_width;
                        let container = container.child(
                            div()
                                .absolute()
                                .inset_0()
                                .border(stroke_w)
                                .border_color(track_color)
                                .rounded(px(9999.0)),
                        );

                        match (self.value, self.spinner_type) {
                            (Some(value), SpinnerType::GrowingCircle) => {
                                let size_px = self.size;
                                let center = size_px * 0.5;
                                let path_radius = (size_px - stroke_w) * 0.5;
                                let num_segments = 32; // More segments for smoother circle

                                container
                                    .children((0..num_segments).map(move |i| {
                                        let segment_angle = (i as f32 / num_segments as f32) * std::f32::consts::TAU;
                                        let progress_threshold = i as f32 / num_segments as f32;

                                        div()
                                            .absolute()
                                            .size(stroke_w * 1.2)
                                            .rounded(px(9999.0))
                                            .bg(stroke_color)
                                            .left(center + path_radius * segment_angle.cos() - stroke_w * 0.6)
                                            .top(center + path_radius * segment_angle.sin() - stroke_w * 0.6)
                                            .opacity(if value >= progress_threshold { 1.0 } else { 0.0 })
                                    }))
                                    .into_any_element()
                            }
                            (Some(value), _) => {
                                container
                                    .child(
                                        div()
                                            .absolute()
                                            .inset_0()
                                            .border(stroke_w)
                                            .border_color(stroke_color)
                                            .rounded(px(9999.0))
                                            .opacity((value * 0.7 + 0.3) as f32),
                                    )
                                    .into_any_element()
                            }
                            (None, SpinnerType::Dot) => {
                                let size_px = self.size;
                                let dot_diameter = stroke_w * 1.5;
                                let dot_radius = dot_diameter * 0.5;
                                let center = size_px * 0.5;
                                let path_radius = (size_px - stroke_w) * 0.5;

                                container
                                    .child(
                                        div()
                                            .absolute()
                                            .size(dot_diameter)
                                            .rounded(px(9999.0))
                                            .bg(stroke_color)
                                            .with_animation(
                                                "spinner-orbit",
                                                Animation::new(std::time::Duration::from_millis(800))
                                                    .repeat()
                                                    .with_easing(crate::animations::easings::linear),
                                                move |dot, delta| {
                                                    let angle = delta * std::f32::consts::TAU;
                                                    let x = center + path_radius * angle.cos() - dot_radius;
                                                    let y = center + path_radius * angle.sin() - dot_radius;
                                                    dot.left(x).top(y)
                                                },
                                            ),
                                    )
                                    .into_any_element()
                            }
                            (None, SpinnerType::Arc) => {
                                let size_px = self.size;
                                let center = size_px * 0.5;
                                let path_radius = (size_px - stroke_w) * 0.5;
                                let num_dots = 8; // Number of dots in the arc

                                container
                                    .children((0..num_dots).map(move |i| {
                                        let dot_angle = (i as f32 / num_dots as f32) * std::f32::consts::PI * 0.75; // Arc of about 135 degrees
                                        div()
                                            .absolute()
                                            .size(stroke_w * 1.2)
                                            .rounded(px(9999.0))
                                            .bg(stroke_color)
                                            .left(center + path_radius * dot_angle.cos() - stroke_w * 0.6)
                                            .top(center + path_radius * dot_angle.sin() - stroke_w * 0.6)
                                            .with_animation(
                                                ("spinner-arc", i as u32),
                                                Animation::new(std::time::Duration::from_millis(1000))
                                                    .repeat()
                                                    .with_easing(crate::animations::easings::linear),
                                                move |dot, delta| {
                                                    let visibility = ((delta + i as f32 / num_dots as f32) % 1.0) < 0.6;
                                                    dot.opacity(if visibility { 1.0 } else { 0.0 })
                                                },
                                            )
                                    }))
                                    .into_any_element()
                            }
                            (None, SpinnerType::ArcNoTrack) => {
                                let size_px = self.size;
                                let center = size_px * 0.5;
                                let path_radius = (size_px - stroke_w) * 0.5;
                                let num_dots = 8;

                                div()
                                    .size(size_px)
                                    .relative()
                                    .children((0..num_dots).map(move |i| {
                                        div()
                                            .absolute()
                                            .size(stroke_w * 1.2)
                                            .rounded(px(9999.0))
                                            .bg(stroke_color)
                                            .with_animation(
                                                ("spinner-arc-no-track", i as u32),
                                                Animation::new(std::time::Duration::from_millis(1000))
                                                    .repeat()
                                                    .with_easing(crate::animations::easings::linear),
                                                move |dot, delta| {
                                                    let angle = delta * std::f32::consts::TAU + (i as f32 / num_dots as f32) * std::f32::consts::PI * 0.75;
                                                    let x = center + path_radius * angle.cos() - stroke_w * 0.6;
                                                    let y = center + path_radius * angle.sin() - stroke_w * 0.6;
                                                    dot.left(x).top(y)
                                                },
                                            )
                                    }))
                                    .into_any_element()
                            }
                            (None, SpinnerType::GrowingCircle) => {
                                let size_px = self.size;
                                let center = size_px * 0.5;
                                let path_radius = (size_px - stroke_w) * 0.5;
                                let num_segments = 32;

                                container
                                    .children((0..num_segments).map(move |i| {
                                        div()
                                            .absolute()
                                            .size(stroke_w * 1.2)
                                            .rounded(px(9999.0))
                                            .bg(stroke_color)
                                            .with_animation(
                                                ("growing-circle", i as u32),
                                                Animation::new(std::time::Duration::from_millis(2000))
                                                    .repeat()
                                                    .with_easing(crate::animations::easings::linear),
                                                move |dot, delta| {
                                                    let segment_angle = (i as f32 / num_segments as f32) * std::f32::consts::TAU;
                                                    let x = center + path_radius * segment_angle.cos() - stroke_w * 0.6;
                                                    let y = center + path_radius * segment_angle.sin() - stroke_w * 0.6;
                                                    let segment_progress = i as f32 / num_segments as f32;
                                                    let visibility = (delta - segment_progress + 1.0) % 1.0 < 0.3;
                                                    dot.left(x).top(y).opacity(if visibility { 1.0 } else { 0.0 })
                                                },
                                            )
                                    }))
                                    .into_any_element()
                            }
                        }
                    })
            )
            .map(|this| {
                let mut div = this;
                div.style().refine(&user_style);
                div
            })
    }
}
