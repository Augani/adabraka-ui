use gpui::*;
use adabraka_ui::{
    theme::{install_theme, Theme, use_theme},
    components::{
        text::{h1, h2, body, caption},
        button::{Button, ButtonVariant},
        progress::{ProgressBar, CircularProgress, ProgressVariant, ProgressSize, SpinnerType},
        scrollable::scrollable_vertical,
    },
};

struct ProgressDemo {
    progress: f32,
    growing_circle_progress: f32,
    frame_counter: u32,
}

impl ProgressDemo {
    fn new() -> Self {
        Self {
            progress: 0.45,
            growing_circle_progress: 0.0,
            frame_counter: 0,
        }
    }
}

impl Render for ProgressDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        // Animate the growing circle progress using frame counter
        self.frame_counter += 1;
        if self.frame_counter % 10 == 0 { // Update every 10 frames for slower animation
            self.growing_circle_progress += 0.005; // Slow increment
            if self.growing_circle_progress > 1.0 {
                self.growing_circle_progress = 0.0; // Reset to create continuous loop
            }
            cx.notify(); // Trigger re-render to show animation
        }

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(theme.tokens.background)
            .overflow_hidden()
            .child(
                // Header
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .p(px(24.0))
                    .border_b_1()
                    .border_color(theme.tokens.border)
                    .child(h1("Progress Indicators"))
                    .child(caption("Linear progress bars and circular spinners with multiple variants and sizes"))
            )
            .child(
                scrollable_vertical(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(32.0))
                        .p(px(24.0))
                    // Linear Progress Bars - Determinate
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(h2("Linear Progress - Determinate"))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(20.0))
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .child(body("Default"))
                                            .child(ProgressBar::new(self.progress))
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .child(body("With Label"))
                                            .child(
                                                ProgressBar::new(self.progress)
                                                    .label("Processing files...")
                                            )
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .child(body("With Percentage"))
                                            .child(
                                                ProgressBar::new(self.progress)
                                                    .label("Upload Progress")
                                                    .show_percentage(true)
                                            )
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .child(body("Success Variant"))
                                            .child(
                                                ProgressBar::new(0.75)
                                                    .variant(ProgressVariant::Success)
                                                    .label("Download Complete")
                                                    .show_percentage(true)
                                            )
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .child(body("Warning Variant"))
                                            .child(
                                                ProgressBar::new(0.6)
                                                    .variant(ProgressVariant::Warning)
                                                    .label("Storage Almost Full")
                                                    .show_percentage(true)
                                            )
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .child(body("Destructive Variant"))
                                            .child(
                                                ProgressBar::new(0.4)
                                                    .variant(ProgressVariant::Destructive)
                                                    .label("Critical: Low Memory")
                                                    .show_percentage(true)
                                            )
                                    )
                            )
                    )
                    // Linear Progress Bars - Sizes
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(h2("Linear Progress - Sizes"))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(20.0))
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .child(body("Small"))
                                            .child(
                                                ProgressBar::new(0.7)
                                                    .size(ProgressSize::Sm)
                                                    .label("Small progress bar")
                                            )
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .child(body("Medium (Default)"))
                                            .child(
                                                ProgressBar::new(0.7)
                                                    .size(ProgressSize::Md)
                                                    .label("Medium progress bar")
                                            )
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .child(body("Large"))
                                            .child(
                                                ProgressBar::new(0.7)
                                                    .size(ProgressSize::Lg)
                                                    .label("Large progress bar")
                                            )
                                    )
                            )
                    )
                    // Linear Progress Bars - Indeterminate
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(h2("Linear Progress - Indeterminate"))
                            .child(caption("Animated loading indicators for when progress is unknown"))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(20.0))
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .child(body("Default Loading"))
                                            .child(
                                                ProgressBar::indeterminate()
                                                    .label("Loading...")
                                            )
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .child(body("Success Loading"))
                                            .child(
                                                ProgressBar::indeterminate()
                                                    .variant(ProgressVariant::Success)
                                                    .label("Syncing data...")
                                            )
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .child(body("Warning Loading"))
                                            .child(
                                                ProgressBar::indeterminate()
                                                    .variant(ProgressVariant::Warning)
                                                    .label("Processing with warnings...")
                                            )
                                    )
                            )
                    )
                    // Circular Progress / Spinners
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(h2("Circular Progress / Spinners"))
                            .child(
                                div()
                                    .flex()
                                    .flex_wrap()
                                    .gap(px(32.0))
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .items_center()
                                            .gap(px(8.0))
                                            .child(CircularProgress::new(self.progress))
                                            .child(caption("Determinate"))
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .items_center()
                                            .gap(px(8.0))
                                            .child(
                                                CircularProgress::new(self.growing_circle_progress)
                                                    .spinner_type(SpinnerType::GrowingCircle)
                                            )
                                            .child(caption("Growing Circle"))
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .items_center()
                                            .gap(px(8.0))
                                            .child(CircularProgress::indeterminate())
                                            .child(caption("Dot Spinner"))
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .items_center()
                                            .gap(px(8.0))
                                            .child(
                                                CircularProgress::indeterminate()
                                                    .spinner_type(SpinnerType::Arc)
                                            )
                                            .child(caption("Arc Spinner"))
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .items_center()
                                            .gap(px(8.0))
                                            .child(
                                                CircularProgress::indeterminate()
                                                    .spinner_type(SpinnerType::ArcNoTrack)
                                            )
                                            .child(caption("Arc No Track"))
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .items_center()
                                            .gap(px(8.0))
                                            .child(
                                                CircularProgress::indeterminate()
                                                    .spinner_type(SpinnerType::GrowingCircle)
                                            )
                                            .child(caption("Growing Circle Spin"))
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .items_center()
                                            .gap(px(8.0))
                                            .child(
                                                CircularProgress::indeterminate()
                                                    .variant(ProgressVariant::Success)
                                                    .size(px(32.0))
                                            )
                                            .child(caption("Success (Small)"))
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .items_center()
                                            .gap(px(8.0))
                                            .child(
                                                CircularProgress::indeterminate()
                                                    .variant(ProgressVariant::Warning)
                                                    .spinner_type(SpinnerType::Arc)
                                                    .size(px(48.0))
                                            )
                                            .child(caption("Warning Arc"))
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .items_center()
                                            .gap(px(8.0))
                                            .child(
                                                CircularProgress::indeterminate()
                                                    .variant(ProgressVariant::Destructive)
                                                    .spinner_type(SpinnerType::ArcNoTrack)
                                                    .size(px(64.0))
                                            )
                                            .child(caption("Destructive Arc"))
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .items_center()
                                            .gap(px(8.0))
                                            .child(
                                                CircularProgress::indeterminate()
                                                    .variant(ProgressVariant::Success)
                                                    .spinner_type(SpinnerType::Dot)
                                                    .size(px(56.0))
                                            )
                                            .child(caption("Success Dot"))
                                    )
                            )
                    )
                    // Usage Examples
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(h2("Usage Examples"))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(12.0))
                                    .p(px(16.0))
                                    .rounded(theme.tokens.radius_md)
                                    .bg(theme.tokens.muted.opacity(0.3))
                                    .child(caption("// Determinate progress"))
                                    .child(caption("ProgressBar::new(0.75)"))
                                    .child(caption("    .variant(ProgressVariant::Success)"))
                                    .child(caption("    .label(\"Upload Complete\")"))
                                    .child(caption("    .show_percentage(true)"))
                                    .child(caption(""))
                                    .child(caption("// Indeterminate loading"))
                                    .child(caption("ProgressBar::indeterminate()"))
                                    .child(caption("    .label(\"Loading...\")"))
                                    .child(caption(""))
                                    .child(caption("// Circular spinners"))
                                    .child(caption("// Dot spinner (default)"))
                                    .child(caption("CircularProgress::indeterminate()"))
                                    .child(caption("    .spinner_type(SpinnerType::Dot)"))
                                    .child(caption(""))
                                    .child(caption("// Arc spinner with track"))
                                    .child(caption("CircularProgress::indeterminate()"))
                                    .child(caption("    .spinner_type(SpinnerType::Arc)"))
                                    .child(caption(""))
                                    .child(caption("// Arc spinner without track"))
                                    .child(caption("CircularProgress::indeterminate()"))
                                    .child(caption("    .spinner_type(SpinnerType::ArcNoTrack)"))
                                    .child(caption(""))
                                    .child(caption("// Growing circle spinner"))
                                    .child(caption("CircularProgress::indeterminate()"))
                                    .child(caption("    .spinner_type(SpinnerType::GrowingCircle)"))
                                    .child(caption(""))
                                    .child(caption("// Determinate growing circle"))
                                    .child(caption("CircularProgress::new(0.75)"))
                                    .child(caption("    .spinner_type(SpinnerType::GrowingCircle)"))
                                    .child(caption("    .size(px(48.0))"))
                                    .child(caption("    .variant(ProgressVariant::Default)"))
                            )
                    )
                    // Control Buttons
                    .child(
                        div()
                            .flex()
                            .gap(px(12.0))
                            .justify_center()
                            .mt(px(16.0))
                            .child(
                                Button::new("increase-progress-btn", "Increase Progress")
                                    .variant(ButtonVariant::Default)
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.progress = (this.progress + 0.1).min(1.0);
                                        cx.notify();
                                    }))
                            )
                            .child(
                                Button::new("decrease-progress-btn", "Decrease Progress")
                                    .variant(ButtonVariant::Outline)
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.progress = (this.progress - 0.1).max(0.0);
                                        cx.notify();
                                    }))
                            )
                            .child(
                                Button::new("reset-btn", "Reset")
                                    .variant(ButtonVariant::Outline)
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.progress = 0.0;
                                        cx.notify();
                                    }))
                            )
                    )
                )
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        // Initialize the UI library
        adabraka_ui::init(cx);

        // Install dark theme
        install_theme(cx, Theme::dark());

        let bounds = Bounds::centered(None, size(px(1000.0), px(900.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("Progress Demo".into()),
                    appears_transparent: false,
                    traffic_light_position: None,
                }),
                ..Default::default()
            },
            |_, cx| cx.new(|_| ProgressDemo::new()),
        )
        .unwrap();

        cx.activate(true);
    });
}
