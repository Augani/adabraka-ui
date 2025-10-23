use adabraka_ui::{
    prelude::*,
    components::{
        resizable::{h_resizable, resizable_panel, ResizableState},
        scrollable::scrollable_vertical,
    },
};
use gpui::*;
use std::path::PathBuf;

struct Assets {
    base: PathBuf,
}

impl gpui::AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<std::borrow::Cow<'static, [u8]>>> {
        std::fs::read(self.base.join(path))
            .map(|data| Some(std::borrow::Cow::Owned(data)))
            .map_err(|err| err.into())
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        std::fs::read_dir(self.base.join(path))
            .map(|entries| {
                entries
                    .filter_map(|entry| {
                        entry
                            .ok()
                            .and_then(|entry| entry.file_name().into_string().ok())
                            .map(SharedString::from)
                    })
                    .collect()
            })
            .map_err(|err| err.into())
    }
}

fn main() {
    Application::new()
        .with_assets(Assets {
            base: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
        })
        .run(|cx| {
            adabraka_ui::init(cx);
            adabraka_ui::set_icon_base_path("assets/icons");
            install_theme(cx, Theme::dark());

            cx.open_window(
                WindowOptions {
                    titlebar: Some(TitlebarOptions {
                        title: Some("Resizable Styled Trait Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(1400.0), px(900.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|_| ResizableStyledDemoView),
            )
            .unwrap();
        });
}

struct ResizableStyledDemoView;

impl Render for ResizableStyledDemoView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let state1 = ResizableState::new(cx);
        let state2 = ResizableState::new(cx);
        let state3 = ResizableState::new(cx);
        let state4 = ResizableState::new(cx);
        let state5 = ResizableState::new(cx);
        let state6 = ResizableState::new(cx);

        div()
            .size_full()
            .flex()
            .flex_col()
            .gap_6()
            .p_8()
            .bg(rgb(0x1a1a1a))
            .child(scrollable_vertical(
                div()
                    .flex()
                    .flex_col()
                    .gap_8()
                    .child(
                        div()
                            .text_2xl()
                            .font_weight(FontWeight::BOLD)
                            .text_color(white())
                            .mb_4()
                            .child("Resizable Panel Styled Trait Demo")
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0xaaaaaa))
                            .mb_4()
                            .child("Drag the handles between panels to resize. The Styled trait allows custom styling.")
                    )
                    // Example 1: Default Resizable Panels
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_3()
                            .child(
                                div()
                                    .text_base()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(0xcccccc))
                                    .child("1. Default Styling")
                            )
                            .child(
                                div()
                                    .h(px(250.0))
                                    .w_full()
                                    .child(
                                        h_resizable("demo1", state1)
                                            .child(
                                                resizable_panel()
                                                    .child(
                                                        div()
                                                            .size_full()
                                                            .flex()
                                                            .items_center()
                                                            .justify_center()
                                                            .text_color(white())
                                                            .child("Panel 1 - Default")
                                                    )
                                            )
                                            .child(
                                                resizable_panel()
                                                    .child(
                                                        div()
                                                            .size_full()
                                                            .flex()
                                                            .items_center()
                                                            .justify_center()
                                                            .text_color(white())
                                                            .child("Panel 2 - Default")
                                                    )
                                            )
                                            .child(
                                                resizable_panel()
                                                    .child(
                                                        div()
                                                            .size_full()
                                                            .flex()
                                                            .items_center()
                                                            .justify_center()
                                                            .text_color(white())
                                                            .child("Panel 3 - Default")
                                                    )
                                            )
                                    )
                            )
                    )
                    // Example 2: Custom Backgrounds
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_3()
                            .child(
                                div()
                                    .text_base()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(0xcccccc))
                                    .child("2. Custom Backgrounds")
                            )
                            .child(
                                div()
                                    .h(px(250.0))
                                    .w_full()
                                    .child(
                                        h_resizable("demo2", state2)
                                            .child(
                                                resizable_panel()
                                                    .bg(rgb(0x1e3a8a))
                                                    .child(
                                                        div()
                                                            .size_full()
                                                            .flex()
                                                            .items_center()
                                                            .justify_center()
                                                            .text_color(white())
                                                            .child("Blue Panel")
                                                    )
                                            )
                                            .child(
                                                resizable_panel()
                                                    .bg(rgb(0x065f46))
                                                    .child(
                                                        div()
                                                            .size_full()
                                                            .flex()
                                                            .items_center()
                                                            .justify_center()
                                                            .text_color(white())
                                                            .child("Green Panel")
                                                    )
                                            )
                                            .child(
                                                resizable_panel()
                                                    .bg(rgb(0x7c2d12))
                                                    .child(
                                                        div()
                                                            .size_full()
                                                            .flex()
                                                            .items_center()
                                                            .justify_center()
                                                            .text_color(white())
                                                            .child("Orange Panel")
                                                    )
                                            )
                                    )
                            )
                    )
                    // Example 3: Custom Borders
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_3()
                            .child(
                                div()
                                    .text_base()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(0xcccccc))
                                    .child("3. Custom Borders")
                            )
                            .child(
                                div()
                                    .h(px(250.0))
                                    .w_full()
                                    .child(
                                        h_resizable("demo3", state3)
                                            .child(
                                                resizable_panel()
                                                    .border_2()
                                                    .border_color(rgb(0x3b82f6))
                                                    .bg(rgb(0x1e1e1e))
                                                    .child(
                                                        div()
                                                            .size_full()
                                                            .flex()
                                                            .items_center()
                                                            .justify_center()
                                                            .text_color(white())
                                                            .child("Blue Border")
                                                    )
                                            )
                                            .child(
                                                resizable_panel()
                                                    .border_2()
                                                    .border_color(rgb(0xf59e0b))
                                                    .bg(rgb(0x1e1e1e))
                                                    .child(
                                                        div()
                                                            .size_full()
                                                            .flex()
                                                            .items_center()
                                                            .justify_center()
                                                            .text_color(white())
                                                            .child("Amber Border")
                                                    )
                                            )
                                            .child(
                                                resizable_panel()
                                                    .border_2()
                                                    .border_color(rgb(0x10b981))
                                                    .bg(rgb(0x1e1e1e))
                                                    .child(
                                                        div()
                                                            .size_full()
                                                            .flex()
                                                            .items_center()
                                                            .justify_center()
                                                            .text_color(white())
                                                            .child("Green Border")
                                                    )
                                            )
                                    )
                            )
                    )
                    // Example 4: Custom Padding & Rounded Corners
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_3()
                            .child(
                                div()
                                    .text_base()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(0xcccccc))
                                    .child("4. Custom Padding & Rounded Corners")
                            )
                            .child(
                                div()
                                    .h(px(250.0))
                                    .w_full()
                                    .child(
                                        h_resizable("demo4", state4)
                                            .child(
                                                resizable_panel()
                                                    .p_6()
                                                    .rounded(px(12.0))
                                                    .bg(rgb(0x831843))
                                                    .child(
                                                        div()
                                                            .size_full()
                                                            .flex()
                                                            .items_center()
                                                            .justify_center()
                                                            .text_color(white())
                                                            .child("Rounded Panel 1")
                                                    )
                                            )
                                            .child(
                                                resizable_panel()
                                                    .p_8()
                                                    .rounded(px(16.0))
                                                    .bg(rgb(0x1e40af))
                                                    .child(
                                                        div()
                                                            .size_full()
                                                            .flex()
                                                            .items_center()
                                                            .justify_center()
                                                            .text_color(white())
                                                            .child("Rounded Panel 2")
                                                    )
                                            )
                                    )
                            )
                    )
                    // Example 5: Combined Styling
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_3()
                            .child(
                                div()
                                    .text_base()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(0xcccccc))
                                    .child("5. Combined Styling")
                            )
                            .child(
                                div()
                                    .h(px(250.0))
                                    .w_full()
                                    .child(
                                        h_resizable("demo5", state5)
                                            .child(
                                                resizable_panel()
                                                    .bg(rgb(0x0f172a))
                                                    .border_2()
                                                    .border_color(rgb(0x6366f1))
                                                    .rounded(px(8.0))
                                                    .p_4()
                                                    .shadow(vec![
                                                        BoxShadow {
                                                            offset: point(px(0.0), px(2.0)),
                                                            blur_radius: px(8.0),
                                                            spread_radius: px(0.0),
                                                            color: hsla(0.0, 0.0, 0.0, 0.3),
                                                        }
                                                    ])
                                                    .child(
                                                        div()
                                                            .size_full()
                                                            .flex()
                                                            .items_center()
                                                            .justify_center()
                                                            .text_color(white())
                                                            .child("Styled Panel 1")
                                                    )
                                            )
                                            .child(
                                                resizable_panel()
                                                    .bg(rgb(0x7c2d12))
                                                    .border_2()
                                                    .border_color(rgb(0xfbbf24))
                                                    .rounded(px(8.0))
                                                    .p_4()
                                                    .shadow(vec![
                                                        BoxShadow {
                                                            offset: point(px(0.0), px(2.0)),
                                                            blur_radius: px(8.0),
                                                            spread_radius: px(0.0),
                                                            color: hsla(0.0, 0.0, 0.0, 0.3),
                                                        }
                                                    ])
                                                    .child(
                                                        div()
                                                            .size_full()
                                                            .flex()
                                                            .items_center()
                                                            .justify_center()
                                                            .text_color(white())
                                                            .child("Styled Panel 2")
                                                    )
                                            )
                                            .child(
                                                resizable_panel()
                                                    .bg(rgb(0x064e3b))
                                                    .border_2()
                                                    .border_color(rgb(0x34d399))
                                                    .rounded(px(8.0))
                                                    .p_4()
                                                    .shadow(vec![
                                                        BoxShadow {
                                                            offset: point(px(0.0), px(2.0)),
                                                            blur_radius: px(8.0),
                                                            spread_radius: px(0.0),
                                                            color: hsla(0.0, 0.0, 0.0, 0.3),
                                                        }
                                                    ])
                                                    .child(
                                                        div()
                                                            .size_full()
                                                            .flex()
                                                            .items_center()
                                                            .justify_center()
                                                            .text_color(white())
                                                            .child("Styled Panel 3")
                                                    )
                                            )
                                    )
                            )
                    )
                    // Example 6: Gradient Backgrounds
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_3()
                            .child(
                                div()
                                    .text_base()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(0xcccccc))
                                    .child("6. Gradient-Style Backgrounds")
                            )
                            .child(
                                div()
                                    .h(px(250.0))
                                    .w_full()
                                    .child(
                                        h_resizable("demo6", state6)
                                            .child(
                                                resizable_panel()
                                                    .bg(rgb(0x581c87))
                                                    .rounded(px(10.0))
                                                    .m_2()
                                                    .child(
                                                        div()
                                                            .size_full()
                                                            .flex()
                                                            .items_center()
                                                            .justify_center()
                                                            .text_color(white())
                                                            .child("Purple")
                                                    )
                                            )
                                            .child(
                                                resizable_panel()
                                                    .bg(rgb(0xbe185d))
                                                    .rounded(px(10.0))
                                                    .m_2()
                                                    .child(
                                                        div()
                                                            .size_full()
                                                            .flex()
                                                            .items_center()
                                                            .justify_center()
                                                            .text_color(white())
                                                            .child("Pink")
                                                    )
                                            )
                                            .child(
                                                resizable_panel()
                                                    .bg(rgb(0x0369a1))
                                                    .rounded(px(10.0))
                                                    .m_2()
                                                    .child(
                                                        div()
                                                            .size_full()
                                                            .flex()
                                                            .items_center()
                                                            .justify_center()
                                                            .text_color(white())
                                                            .child("Sky Blue")
                                                    )
                                            )
                                    )
                            )
                    )
            ))
    }
}
