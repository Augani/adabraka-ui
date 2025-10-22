use gpui::*;
use adabraka_ui::layout::VStack;
use adabraka_ui::theme::use_theme;

use std::ops::Range;

use adabraka_ui::virtual_list::{
    vlist_uniform, vlist_variable, ItemExtentProvider,
};

struct UniformDemo {
    scroll: ScrollHandle,
}

impl UniformDemo {
    fn new(_cx: &mut Context<Self>) -> Self { Self { scroll: ScrollHandle::new() } }
}

impl Render for UniformDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();
        let item_count: usize = 10_000_000; // 10M
        let item_extent = px(28.0);

        let renderer = move |range: Range<usize>, _window: &mut Window, _cx: &mut App| {
            let mut out = Vec::with_capacity(range.len());
            for i in range {
                out.push(
                    div()
                        .w_full()
                        .h(item_extent)
                        .px(px(12.0))
                        .items_center()
                        .bg(if i % 2 == 0 { theme.tokens.background } else { theme.tokens.muted.opacity(0.3) })
                        .child(format!("Row #{}", i))
                );
            }
            out
        };

        vlist_uniform("uniform-demo", item_count, item_extent, renderer)
            .track_scroll(&self.scroll)
            .overscan(8)
    }
}

struct RandomHeightsProvider;

impl ItemExtentProvider for RandomHeightsProvider {
    fn extent(&self, index: usize) -> Pixels {
        // Simple deterministic pseudo-random: 24..=72 px
        let x = (index as u64).wrapping_mul(6364136223846793005).wrapping_add(1);
        let r = ((x >> 33) as u32) % 49; // 0..=48
        px(24.0 + r as f32)
    }
}

struct VariableDemo {
    scroll: ScrollHandle,
}

impl VariableDemo { fn new(_cx: &mut Context<Self>) -> Self { Self { scroll: ScrollHandle::new() } } }

impl Render for VariableDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();
        let item_count: usize = 1_000_000; // 1M
        let provider = RandomHeightsProvider;

        let renderer = move |range: Range<usize>, _window: &mut Window, _cx: &mut App| {
            let mut out = Vec::with_capacity(range.len());
            for i in range {
                let bg = if i % 2 == 0 { theme.tokens.background } else { theme.tokens.muted.opacity(0.3) };
                out.push(
                    div()
                        .w_full()
                        .px(px(12.0))
                        .bg(bg)
                        .child(format!("Variable row #{}", i))
                );
            }
            out
        };

        vlist_variable("variable-demo", item_count, provider, renderer)
            .track_scroll(&self.scroll)
            .overscan(8)
    }
}

struct VirtualListDemoApp {
    uniform: Entity<UniformDemo>,
    variable: Entity<VariableDemo>,
}

impl VirtualListDemoApp { fn new(cx: &mut Context<Self>) -> Self { Self { uniform: cx.new(UniformDemo::new), variable: cx.new(VariableDemo::new) } } }

impl Render for VirtualListDemoApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();
        VStack::new()
            .size_full()
            .bg(theme.tokens.background)
            .text_color(theme.tokens.foreground)
            .gap(px(24.0))
            .p(px(16.0))
            .child(
                div()
                    .text_size(px(18.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .child("Uniform VirtualList (10M items)")
            )
            .child(
                div()
                    .h(px(320.0))
                    .border_1()
                    .border_color(theme.tokens.border)
                    .rounded(px(8.0))
                    .overflow_hidden()
                    .child(self.uniform.clone())
            )
            .child(
                div()
                    .text_size(px(18.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .child("Variable-size VirtualList (1M items)")
            )
            .child(
                div()
                    .h(px(320.0))
                    .border_1()
                    .border_color(theme.tokens.border)
                    .rounded(px(8.0))
                    .overflow_hidden()
                    .child(self.variable.clone())
            )
    }
}

fn main() {
    struct Assets { base: std::path::PathBuf }
    impl gpui::AssetSource for Assets {
        fn load(&self, path: &str) -> Result<Option<std::borrow::Cow<'static, [u8]>>> {
            std::fs::read(self.base.join(path))
                .map(|data| Some(std::borrow::Cow::Owned(data)))
                .map_err(|err| err.into())
        }
        fn list(&self, path: &str) -> Result<Vec<gpui::SharedString>> {
            std::fs::read_dir(self.base.join(path))
                .map(|entries| {
                    entries
                        .filter_map(|entry| {
                            entry
                                .ok()
                                .and_then(|entry| entry.file_name().into_string().ok())
                                .map(gpui::SharedString::from)
                        })
                        .collect()
                })
                .map_err(|err| err.into())
        }
    }

    Application::new()
        .with_assets(Assets { base: std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")) })
        .run(move |cx: &mut App| {
            adabraka_ui::theme::install_theme(cx, adabraka_ui::theme::Theme::dark());
            adabraka_ui::init(cx);
            adabraka_ui::set_icon_base_path("assets/icons");

            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                        None,
                        size(px(1000.0), px(720.0)),
                        cx,
                    ))),
                    titlebar: Some(TitlebarOptions {
                        title: Some("VirtualList Demo".into()),
                        appears_transparent: false,
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                |_window, cx| cx.new(VirtualListDemoApp::new),
            )
            .unwrap();
        });
}


