use adabraka_ui::{
    charts::donut_chart::DonutChart,
    charts::gauge::Gauge,
    charts::pie_chart::PieChartSegment,
    charts::treemap::{TreeMap, TreeMapNode},
    components::{
        animated_counter::AnimatedCounterState,
        animated_progress::AnimatedProgress,
        animated_switch::{AnimatedSwitch, AnimatedSwitchTransition},
        animated_text::{AnimatedText, TextAnimation},
        aurora::Aurora,
        confetti::{Confetti, ConfettiState},
        copy_button::{CopyButton, CopyButtonState},
        expandable_card::{ExpandableCard, ExpandableCardState},
        floating_action_button::{FABSize, FABState, FloatingActionButton},
        gradient_text::GradientText,
        kbd::KBD,
        marquee::{Marquee, MarqueeDirection},
        number_ticker::NumberTicker,
        pulse_indicator::PulseIndicator,
        qr_code::QRCodeComponent,
        scrollable::scrollable_vertical,
        segmented_nav::{SegmentedNav, SegmentedNavState},
        shimmer::Shimmer,
        text::{body, caption, h1, h2, h3, muted},
        text_reveal::{RevealMode, TextReveal},
        type_writer::{TypeWriter, TypeWriterState},
    },
    display::badge::{Badge, BadgeVariant},
    theme::{install_theme, use_theme, Theme},
};
use gpui::{prelude::FluentBuilder as _, *};
use std::time::Duration;

struct ShowcaseApp {
    segmented_state: Entity<SegmentedNavState>,
    typewriter_state: Entity<TypeWriterState>,
    counter_state: Entity<AnimatedCounterState>,
    copy_state: Entity<CopyButtonState>,
    confetti_state: Entity<ConfettiState>,
    expandable_state_1: Entity<ExpandableCardState>,
    expandable_state_2: Entity<ExpandableCardState>,
    fab_state: Entity<FABState>,
    progress_value: f32,
    ticker_value: i64,
    gauge_value: f32,
    switch_key: usize,
    tick: usize,
}

impl ShowcaseApp {
    fn new(cx: &mut Context<Self>) -> Self {
        let typewriter_state = cx.new(|_cx| {
            TypeWriterState::new("Welcome to adabraka-ui — a beautiful component library for GPUI desktop apps.")
                .with_speed(Duration::from_millis(40))
        });

        let tw = typewriter_state.clone();
        cx.spawn(async move |_this, cx| {
            cx.background_executor()
                .timer(Duration::from_millis(500))
                .await;
            let _ = tw.update(cx, |state, cx| state.start(cx));
        })
        .detach();

        let counter_state = cx.new(|_cx| AnimatedCounterState::new(0.0));

        cx.spawn(async move |this, cx| {
            loop {
                cx.background_executor()
                    .timer(Duration::from_millis(100))
                    .await;
                let result = this.update(cx, |state, cx| {
                    state.tick = state.tick.wrapping_add(1);
                    if state.tick % 40 == 0 {
                        state.progress_value = if state.progress_value >= 1.0 {
                            0.0
                        } else {
                            (state.progress_value + 0.15).min(1.0)
                        };
                    }
                    if state.tick % 30 == 0 {
                        state.ticker_value += 1247;
                        state.gauge_value = ((state.tick as f32 * 0.02).sin() * 0.4 + 0.5).clamp(0.0, 1.0);
                    }
                    cx.notify();
                });
                if result.is_err() {
                    break;
                }
            }
        })
        .detach();

        Self {
            segmented_state: cx.new(|_cx| SegmentedNavState::new("text")),
            typewriter_state,
            counter_state,
            copy_state: cx.new(|_cx| {
                CopyButtonState::new("cargo add adabraka-ui".into())
            }),
            confetti_state: cx.new(ConfettiState::new),
            expandable_state_1: cx.new(|_cx| ExpandableCardState::new()),
            expandable_state_2: cx.new(|_cx| ExpandableCardState::new()),
            fab_state: cx.new(|_cx| FABState::new()),
            progress_value: 0.35,
            ticker_value: 42_195,
            gauge_value: 0.72,
            switch_key: 0,
            tick: 0,
        }
    }
}

fn section(theme: &Theme) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(16.0))
        .p(px(24.0))
        .bg(theme.tokens.card)
        .border_1()
        .border_color(theme.tokens.border)
        .rounded(px(12.0))
}

fn row() -> Div {
    div().flex().flex_wrap().gap(px(16.0)).items_center()
}

fn col() -> Div {
    div().flex().flex_col().gap(px(8.0))
}

fn demo_label(text: &str, theme: &Theme) -> Div {
    div()
        .text_size(px(11.0))
        .text_color(theme.tokens.muted_foreground)
        .child(text.to_string())
}

impl Render for ShowcaseApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        let active_tab = self.segmented_state.read(cx).active().clone();
        let seg_state = self.segmented_state.clone();

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(theme.tokens.background)
            .text_color(theme.tokens.foreground)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .p(px(24.0))
                    .pb(px(16.0))
                    .border_b_1()
                    .border_color(theme.tokens.border)
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(12.0))
                            .child(h1("adabraka-ui"))
                            .child(
                                Badge::new("v0.3")
                                    .variant(BadgeVariant::Default),
                            ),
                    )
                    .child(muted("85+ components for beautiful GPUI desktop applications"))
                    .child(
                        div().pt(px(8.0)).child(
                            SegmentedNav::new("main-nav", seg_state)
                                .item("text", "Text & Type")
                                .item("data", "Data Viz")
                                .item("interactive", "Interactive")
                                .item("effects", "Effects"),
                        ),
                    ),
            )
            .child(
                scrollable_vertical(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(24.0))
                        .p(px(24.0))
                        .when(active_tab.as_ref() == "text", |el| {
                            el.child(self.render_text_section(&theme, cx))
                        })
                        .when(active_tab.as_ref() == "data", |el| {
                            el.child(self.render_data_section(&theme, cx))
                        })
                        .when(active_tab.as_ref() == "interactive", |el| {
                            el.child(self.render_interactive_section(&theme, cx))
                        })
                        .when(active_tab.as_ref() == "effects", |el| {
                            el.child(self.render_effects_section(&theme, cx))
                        }),
                ),
            )
    }
}

impl ShowcaseApp {
    fn render_text_section(&self, theme: &Theme, _cx: &mut Context<Self>) -> Div {
        div()
            .flex()
            .flex_col()
            .gap(px(24.0))
            .child(
                section(theme)
                    .child(h2("Gradient Text"))
                    .child(caption("Rich gradient text with customizable colors"))
                    .child(
                        row()
                            .child(
                                GradientText::new("Build beautiful apps")
                                    .start_color(hsla(0.75, 0.8, 0.6, 1.0))
                                    .end_color(hsla(0.55, 0.8, 0.6, 1.0))
                                    .text_size(px(32.0))
                                    .font_weight(FontWeight::BOLD),
                            ),
                    )
                    .child(
                        row()
                            .child(
                                GradientText::new("Sunrise Gradient")
                                    .start_color(hsla(0.08, 0.9, 0.55, 1.0))
                                    .end_color(hsla(0.0, 0.8, 0.55, 1.0))
                                    .text_size(px(24.0))
                                    .font_weight(FontWeight::SEMIBOLD),
                            )
                            .child(
                                GradientText::new("Ocean Breeze")
                                    .start_color(hsla(0.5, 0.8, 0.55, 1.0))
                                    .end_color(hsla(0.6, 0.9, 0.4, 1.0))
                                    .text_size(px(24.0))
                                    .font_weight(FontWeight::SEMIBOLD),
                            ),
                    ),
            )
            .child(
                section(theme)
                    .child(h2("Animated Text"))
                    .child(caption("Character-by-character text animations"))
                    .child(
                        col()
                            .gap(px(12.0))
                            .child(
                                row()
                                    .gap(px(24.0))
                                    .child(
                                        col()
                                            .child(demo_label("FadeUp", theme))
                                            .child(
                                                AnimatedText::new("fade-up", "Hello World")
                                                    .animation(TextAnimation::FadeUp)
                                                    .text_size(px(20.0))
                                                    .font_weight(FontWeight::SEMIBOLD),
                                            ),
                                    )
                                    .child(
                                        col()
                                            .child(demo_label("Wave", theme))
                                            .child(
                                                AnimatedText::new("wave", "Wavy Motion")
                                                    .animation(TextAnimation::Wave)
                                                    .text_size(px(20.0))
                                                    .font_weight(FontWeight::SEMIBOLD),
                                            ),
                                    )
                                    .child(
                                        col()
                                            .child(demo_label("Scale", theme))
                                            .child(
                                                AnimatedText::new("scale", "Pop In!")
                                                    .animation(TextAnimation::Scale)
                                                    .text_size(px(20.0))
                                                    .font_weight(FontWeight::SEMIBOLD),
                                            ),
                                    ),
                            ),
                    ),
            )
            .child(
                section(theme)
                    .child(h2("Text Reveal"))
                    .child(caption("Progressive text reveal with configurable modes"))
                    .child(
                        col()
                            .gap(px(12.0))
                            .child(
                                col()
                                    .child(demo_label("By Word", theme))
                                    .child(
                                        TextReveal::new(
                                            "reveal-word",
                                            "The quick brown fox jumps over the lazy dog",
                                        )
                                        .mode(RevealMode::ByWord)
                                        .text_size(px(16.0)),
                                    ),
                            )
                            .child(
                                col()
                                    .child(demo_label("By Character", theme))
                                    .child(
                                        TextReveal::new("reveal-char", "Character by character reveal")
                                            .mode(RevealMode::ByCharacter)
                                            .stagger(Duration::from_millis(30))
                                            .text_size(px(16.0)),
                                    ),
                            ),
                    ),
            )
            .child(
                section(theme)
                    .child(h2("TypeWriter"))
                    .child(caption("Realistic typing animation with cursor"))
                    .child(
                        TypeWriter::new("typewriter-demo", self.typewriter_state.clone())
                            .cursor(true)
                            .text_size(px(16.0)),
                    ),
            )
            .child(
                section(theme)
                    .child(h2("Marquee"))
                    .child(caption("Auto-scrolling text ticker"))
                    .child(
                        Marquee::new("marquee-demo", || {
                            div()
                                .flex()
                                .gap(px(24.0))
                                .child(body("Breaking: adabraka-ui v0.3 released with 85+ components"))
                                .child(
                                    div()
                                        .text_color(hsla(0.55, 0.8, 0.6, 1.0))
                                        .child("New: Transforms, Gradients, Blend Modes in GPUI fork"),
                                )
                                .child(body("Star us on GitHub!"))
                                .into_any_element()
                        })
                        .speed(60.0)
                        .direction(MarqueeDirection::Left)
                        .h(px(32.0)),
                    ),
            )
    }

    fn render_data_section(&self, theme: &Theme, _cx: &mut Context<Self>) -> Div {
        let _counter = self.counter_state.clone();
        let ticker_val = self.ticker_value;
        let progress_val = self.progress_value;
        let gauge_val = self.gauge_value;

        div()
            .flex()
            .flex_col()
            .gap(px(24.0))
            .child(
                section(theme)
                    .child(h2("Number Ticker"))
                    .child(caption("Animated number transitions with formatting"))
                    .child(
                        row()
                            .gap(px(32.0))
                            .child(
                                col()
                                    .child(demo_label("Revenue", theme))
                                    .child(
                                        NumberTicker::new("ticker-revenue", ticker_val)
                                            .prefix(SharedString::from("$"))
                                            .separator(',')
                                            .text_size(px(32.0))
                                            .font_weight(FontWeight::BOLD),
                                    ),
                            )
                            .child(
                                col()
                                    .child(demo_label("Users", theme))
                                    .child(
                                        NumberTicker::new("ticker-users", ticker_val / 3)
                                            .separator(',')
                                            .text_size(px(32.0))
                                            .font_weight(FontWeight::BOLD),
                                    ),
                            )
                            .child(
                                col()
                                    .child(demo_label("Downloads", theme))
                                    .child(
                                        NumberTicker::new("ticker-downloads", ticker_val * 7)
                                            .separator(',')
                                            .suffix(SharedString::from("k"))
                                            .text_size(px(32.0))
                                            .font_weight(FontWeight::BOLD),
                                    ),
                            ),
                    ),
            )
            .child(
                section(theme)
                    .child(h2("Animated Progress"))
                    .child(caption("Smooth animated progress bars with shimmer effect"))
                    .child(
                        col()
                            .gap(px(12.0))
                            .child(
                                col()
                                    .child(demo_label(
                                        &format!("Default — {:.0}%", progress_val * 100.0),
                                        theme,
                                    ))
                                    .child(
                                        AnimatedProgress::new("progress-default")
                                            .value(progress_val)
                                            .w_full(),
                                    ),
                            )
                            .child(
                                col()
                                    .child(demo_label("With Shimmer", theme))
                                    .child(
                                        AnimatedProgress::new("progress-shimmer")
                                            .value(0.65)
                                            .shimmer(true)
                                            .color(hsla(0.55, 0.8, 0.5, 1.0))
                                            .w_full(),
                                    ),
                            )
                            .child(
                                col()
                                    .child(demo_label("Success", theme))
                                    .child(
                                        AnimatedProgress::new("progress-success")
                                            .value(1.0)
                                            .color(hsla(0.35, 0.7, 0.5, 1.0))
                                            .w_full(),
                                    ),
                            ),
                    ),
            )
            .child(
                section(theme)
                    .child(h2("Gauge & Donut Chart"))
                    .child(caption("Real-time data visualization"))
                    .child(
                        row()
                            .gap(px(32.0))
                            .child(
                                col()
                                    .items_center()
                                    .child(demo_label("CPU Usage", theme))
                                    .child(
                                        Gauge::new("gauge-cpu")
                                            .value(gauge_val)
                                            .label(SharedString::from("CPU"))
                                            .color(hsla(0.55, 0.8, 0.5, 1.0)),
                                    ),
                            )
                            .child(
                                col()
                                    .items_center()
                                    .child(demo_label("Memory", theme))
                                    .child(
                                        Gauge::new("gauge-memory")
                                            .value(0.58)
                                            .label(SharedString::from("RAM"))
                                            .color(hsla(0.08, 0.9, 0.55, 1.0)),
                                    ),
                            )
                            .child(
                                col()
                                    .items_center()
                                    .child(demo_label("Storage Breakdown", theme))
                                    .child(
                                        DonutChart::new()
                                            .segment(PieChartSegment {
                                                label: "Documents".into(),
                                                value: 35.0,
                                                color: Some(hsla(0.6, 0.7, 0.5, 1.0)),
                                            })
                                            .segment(PieChartSegment {
                                                label: "Photos".into(),
                                                value: 28.0,
                                                color: Some(hsla(0.35, 0.7, 0.5, 1.0)),
                                            })
                                            .segment(PieChartSegment {
                                                label: "Apps".into(),
                                                value: 22.0,
                                                color: Some(hsla(0.08, 0.9, 0.55, 1.0)),
                                            })
                                            .segment(PieChartSegment {
                                                label: "Other".into(),
                                                value: 15.0,
                                                color: Some(hsla(0.75, 0.6, 0.5, 1.0)),
                                            })
                                            .center_label(SharedString::from("128 GB"))
                                            .center_value(SharedString::from("Total"))
                                            .show_legend(true),
                                    ),
                            ),
                    ),
            )
            .child(
                section(theme)
                    .child(h2("TreeMap"))
                    .child(caption("Hierarchical data as proportional rectangles"))
                    .child(
                        TreeMap::new()
                            .data(vec![
                                TreeMapNode::new("Frontend", 40.0)
                                    .color(hsla(0.6, 0.7, 0.5, 1.0)),
                                TreeMapNode::new("Backend", 30.0)
                                    .color(hsla(0.35, 0.7, 0.5, 1.0)),
                                TreeMapNode::new("DevOps", 15.0)
                                    .color(hsla(0.08, 0.9, 0.55, 1.0)),
                                TreeMapNode::new("Design", 10.0)
                                    .color(hsla(0.75, 0.6, 0.5, 1.0)),
                                TreeMapNode::new("QA", 5.0)
                                    .color(hsla(0.0, 0.7, 0.5, 1.0)),
                            ])
                            .show_labels(true)
                            .w(px(600.0))
                            .h(px(200.0)),
                    ),
            )
    }

    fn render_interactive_section(&self, theme: &Theme, cx: &mut Context<Self>) -> Div {
        let copy_state = self.copy_state.clone();
        let expandable_1 = self.expandable_state_1.clone();
        let expandable_2 = self.expandable_state_2.clone();
        let fab_state = self.fab_state.clone();
        let switch_key = self.switch_key;
        let entity = cx.entity();

        div()
            .flex()
            .flex_col()
            .gap(px(24.0))
            .child(
                section(theme)
                    .child(h2("Animated Switch"))
                    .child(caption("Smooth content transitions between views"))
                    .child(
                        col()
                            .child(
                                row()
                                    .child(
                                        div()
                                            .id("switch-btn-0")
                                            .px(px(12.0))
                                            .py(px(6.0))
                                            .rounded(px(6.0))
                                            .cursor_pointer()
                                            .when(switch_key == 0, |el| {
                                                el.bg(theme.tokens.primary)
                                                    .text_color(theme.tokens.primary_foreground)
                                            })
                                            .when(switch_key != 0, |el| {
                                                el.bg(theme.tokens.muted)
                                                    .text_color(theme.tokens.muted_foreground)
                                            })
                                            .on_mouse_down(MouseButton::Left, {
                                                let entity = entity.clone();
                                                move |_, _, cx| {
                                                    entity.update(cx, |s, cx| {
                                                        s.switch_key = 0;
                                                        cx.notify();
                                                    });
                                                }
                                            })
                                            .child("Dashboard"),
                                    )
                                    .child(
                                        div()
                                            .id("switch-btn-1")
                                            .px(px(12.0))
                                            .py(px(6.0))
                                            .rounded(px(6.0))
                                            .cursor_pointer()
                                            .when(switch_key == 1, |el| {
                                                el.bg(theme.tokens.primary)
                                                    .text_color(theme.tokens.primary_foreground)
                                            })
                                            .when(switch_key != 1, |el| {
                                                el.bg(theme.tokens.muted)
                                                    .text_color(theme.tokens.muted_foreground)
                                            })
                                            .on_mouse_down(MouseButton::Left, {
                                                let entity = entity.clone();
                                                move |_, _, cx| {
                                                    entity.update(cx, |s, cx| {
                                                        s.switch_key = 1;
                                                        cx.notify();
                                                    });
                                                }
                                            })
                                            .child("Settings"),
                                    )
                                    .child(
                                        div()
                                            .id("switch-btn-2")
                                            .px(px(12.0))
                                            .py(px(6.0))
                                            .rounded(px(6.0))
                                            .cursor_pointer()
                                            .when(switch_key == 2, |el| {
                                                el.bg(theme.tokens.primary)
                                                    .text_color(theme.tokens.primary_foreground)
                                            })
                                            .when(switch_key != 2, |el| {
                                                el.bg(theme.tokens.muted)
                                                    .text_color(theme.tokens.muted_foreground)
                                            })
                                            .on_mouse_down(MouseButton::Left, {
                                                let entity = entity.clone();
                                                move |_, _, cx| {
                                                    entity.update(cx, |s, cx| {
                                                        s.switch_key = 2;
                                                        cx.notify();
                                                    });
                                                }
                                            })
                                            .child("Profile"),
                                    ),
                            )
                            .child(
                                div().h(px(60.0)).child(
                                    AnimatedSwitch::new("content-switch")
                                        .active(switch_key)
                                        .transition(AnimatedSwitchTransition::SlideLeft)
                                        .duration(Duration::from_millis(250))
                                        .child(
                                            0,
                                            div()
                                                .p(px(16.0))
                                                .bg(theme.tokens.muted)
                                                .rounded(px(8.0))
                                                .child(body("Dashboard view — charts, metrics, and analytics")),
                                        )
                                        .child(
                                            1,
                                            div()
                                                .p(px(16.0))
                                                .bg(theme.tokens.muted)
                                                .rounded(px(8.0))
                                                .child(body("Settings view — preferences and configuration")),
                                        )
                                        .child(
                                            2,
                                            div()
                                                .p(px(16.0))
                                                .bg(theme.tokens.muted)
                                                .rounded(px(8.0))
                                                .child(body("Profile view — user information and avatar")),
                                        ),
                                ),
                            ),
                    ),
            )
            .child(
                section(theme)
                    .child(h2("Keyboard Shortcuts & Copy"))
                    .child(caption("KBD badges and clipboard copy button"))
                    .child(
                        row()
                            .gap(px(24.0))
                            .child(
                                row()
                                    .child(KBD::new("Cmd"))
                                    .child(body("+"))
                                    .child(KBD::new("K"))
                                    .child(muted("Command Palette")),
                            )
                            .child(
                                row()
                                    .child(KBD::new("Ctrl"))
                                    .child(body("+"))
                                    .child(KBD::new("Shift"))
                                    .child(body("+"))
                                    .child(KBD::new("P"))
                                    .child(muted("Quick Open")),
                            ),
                    )
                    .child(
                        row()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(8.0))
                                    .px(px(12.0))
                                    .py(px(8.0))
                                    .bg(theme.tokens.muted)
                                    .rounded(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(13.0))
                                            .font_family("monospace")
                                            .child("cargo add adabraka-ui"),
                                    )
                                    .child(CopyButton::new("copy-btn", copy_state)),
                            ),
                    ),
            )
            .child(
                section(theme)
                    .child(h2("Expandable Cards"))
                    .child(caption("Click to expand/collapse with animated transitions"))
                    .child(
                        col()
                            .gap(px(12.0))
                            .child(
                                ExpandableCard::new("expand-1", expandable_1)
                                    .collapsed(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap(px(8.0))
                                            .child(h3("What is adabraka-ui?"))
                                            .child(muted("Click to learn more")),
                                    )
                                    .expanded(
                                        col()
                                            .gap(px(8.0))
                                            .child(h3("What is adabraka-ui?"))
                                            .child(body(
                                                "adabraka-ui is a comprehensive component library for building \
                                                 beautiful desktop applications with GPUI. It provides 85+ \
                                                 components including buttons, inputs, charts, overlays, \
                                                 animations, and much more.",
                                            ))
                                            .child(muted("Click to collapse")),
                                    ),
                            )
                            .child(
                                ExpandableCard::new("expand-2", expandable_2)
                                    .collapsed(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap(px(8.0))
                                            .child(h3("Getting Started"))
                                            .child(muted("Click to expand")),
                                    )
                                    .expanded(
                                        col()
                                            .gap(px(8.0))
                                            .child(h3("Getting Started"))
                                            .child(body(
                                                "Add adabraka-ui to your Cargo.toml, call init() in your app \
                                                 setup, install a theme, and start building with the builder \
                                                 pattern API.",
                                            ))
                                            .child(
                                                div()
                                                    .flex()
                                                    .gap(px(4.0))
                                                    .child(KBD::new("1"))
                                                    .child(muted("Add dep"))
                                                    .child(KBD::new("2"))
                                                    .child(muted("Init"))
                                                    .child(KBD::new("3"))
                                                    .child(muted("Build")),
                                            )
                                            .child(muted("Click to collapse")),
                                    ),
                            ),
                    ),
            )
            .child(
                section(theme)
                    .child(h2("Floating Action Button"))
                    .child(caption("Expandable FAB with animated action items"))
                    .child(
                        div()
                            .h(px(180.0))
                            .w_full()
                            .relative()
                            .bg(theme.tokens.muted)
                            .rounded(px(12.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(muted("Click the FAB in the corner"))
                            .child(
                                div()
                                    .absolute()
                                    .bottom(px(16.0))
                                    .right(px(16.0))
                                    .child(
                                        FloatingActionButton::new("fab-demo", fab_state)
                                            .icon(SharedString::from("+"))
                                            .size(FABSize::Md)
                                            .action("new-file", "F", |_, _| {})
                                            .action("new-folder", "D", |_, _| {})
                                            .action("upload", "U", |_, _| {}),
                                    ),
                            ),
                    ),
            )
    }

    fn render_effects_section(&self, theme: &Theme, _cx: &mut Context<Self>) -> Div {
        let confetti_state = self.confetti_state.clone();

        div()
            .flex()
            .flex_col()
            .gap(px(24.0))
            .child(
                section(theme)
                    .child(h2("Pulse Indicator"))
                    .child(caption("Animated status indicators"))
                    .child(
                        row()
                            .gap(px(24.0))
                            .child(
                                row()
                                    .child(
                                        PulseIndicator::new("pulse-green")
                                            .color(hsla(0.35, 0.8, 0.5, 1.0))
                                            .size(px(10.0)),
                                    )
                                    .child(body("Online")),
                            )
                            .child(
                                row()
                                    .child(
                                        PulseIndicator::new("pulse-yellow")
                                            .color(hsla(0.15, 0.9, 0.55, 1.0))
                                            .size(px(10.0)),
                                    )
                                    .child(body("Away")),
                            )
                            .child(
                                row()
                                    .child(
                                        PulseIndicator::new("pulse-red")
                                            .color(hsla(0.0, 0.8, 0.55, 1.0))
                                            .size(px(10.0)),
                                    )
                                    .child(body("Recording")),
                            )
                            .child(
                                row()
                                    .child(
                                        PulseIndicator::new("pulse-blue")
                                            .color(hsla(0.6, 0.8, 0.55, 1.0))
                                            .size(px(10.0)),
                                    )
                                    .child(body("Syncing")),
                            ),
                    ),
            )
            .child(
                section(theme)
                    .child(h2("Shimmer"))
                    .child(caption("Loading placeholder effect"))
                    .child(
                        col()
                            .gap(px(12.0))
                            .child(
                                row()
                                    .gap(px(12.0))
                                    .child(
                                        Shimmer::new()
                                            .w(px(48.0))
                                            .h(px(48.0))
                                            .rounded(px(24.0)),
                                    )
                                    .child(
                                        col()
                                            .gap(px(6.0))
                                            .child(Shimmer::new().w(px(160.0)).h(px(16.0)).rounded(px(4.0)))
                                            .child(Shimmer::new().w(px(120.0)).h(px(12.0)).rounded(px(4.0))),
                                    ),
                            )
                            .child(Shimmer::new().w_full().h(px(100.0)).rounded(px(8.0))),
                    ),
            )
            .child(
                section(theme)
                    .child(h2("Confetti"))
                    .child(caption("Celebration particle burst — click the button"))
                    .child(
                        div()
                            .h(px(200.0))
                            .w_full()
                            .relative()
                            .overflow_hidden()
                            .bg(theme.tokens.muted)
                            .rounded(px(12.0))
                            .child(
                                Confetti::new("confetti-demo", confetti_state.clone())
                                    .size_full(),
                            )
                            .child(
                                div()
                                    .absolute()
                                    .inset_0()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child(
                                        div()
                                            .id("confetti-trigger")
                                            .px(px(20.0))
                                            .py(px(10.0))
                                            .bg(theme.tokens.primary)
                                            .text_color(theme.tokens.primary_foreground)
                                            .rounded(px(8.0))
                                            .cursor_pointer()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .on_mouse_down(MouseButton::Left, {
                                                let state = confetti_state.clone();
                                                move |_, _, cx| {
                                                    state.update(cx, |s, cx| s.burst(cx));
                                                }
                                            })
                                            .child("Celebrate!"),
                                    ),
                            ),
                    ),
            )
            .child(
                section(theme)
                    .child(h2("Aurora Background"))
                    .child(caption("Animated blob background effect"))
                    .child(
                        Aurora::new()
                            .colors(vec![
                                hsla(0.75, 0.7, 0.5, 0.3),
                                hsla(0.55, 0.7, 0.5, 0.3),
                                hsla(0.6, 0.8, 0.4, 0.3),
                            ])
                            .speed(0.8)
                            .w_full()
                            .h(px(200.0))
                            .rounded(px(12.0))
                            .overflow_hidden()
                            .child(
                                div()
                                    .size_full()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child(
                                        div()
                                            .text_size(px(24.0))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(white())
                                            .child("Aurora Effect"),
                                    ),
                            ),
                    ),
            )
            .child(
                section(theme)
                    .child(h2("QR Code"))
                    .child(caption("Generate QR codes from any text"))
                    .child(
                        row()
                            .gap(px(24.0))
                            .child(
                                col()
                                    .items_center()
                                    .child(
                                        QRCodeComponent::new("https://github.com/augani/adabraka-ui")
                                            .size(px(150.0))
                                            .fg_color(theme.tokens.foreground)
                                            .bg_color(theme.tokens.card),
                                    )
                                    .child(demo_label("GitHub Repo", theme)),
                            )
                            .child(
                                col()
                                    .items_center()
                                    .child(
                                        QRCodeComponent::new("adabraka-ui: 85+ GPUI components")
                                            .size(px(150.0))
                                            .fg_color(hsla(0.6, 0.7, 0.4, 1.0))
                                            .bg_color(theme.tokens.card),
                                    )
                                    .child(demo_label("Custom Colors", theme)),
                            ),
                    ),
            )
    }
}

fn main() {
    Application::new().run(move |cx| {
        let bounds = Bounds::centered(None, size(px(960.0), px(800.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("adabraka-ui Component Showcase".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_, cx| {
                install_theme(cx, Theme::dark());
                cx.new(ShowcaseApp::new)
            },
        )
        .unwrap();
    });
}
