use adabraka_ui::{
    prelude::*,
    components::scrollable::scrollable_vertical,
    components::slider::{Slider, SliderState, SliderSize, SliderAxis},
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
                        title: Some("Slider Styled Trait Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(900.0), px(800.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|cx| SliderStyledDemo::new(cx)),
            )
            .unwrap();
        });
}

struct SliderStyledDemo {
    slider1: Entity<SliderState>,
    slider2: Entity<SliderState>,
    slider3: Entity<SliderState>,
    slider4: Entity<SliderState>,
    slider5: Entity<SliderState>,
    slider6: Entity<SliderState>,
    slider7: Entity<SliderState>,
    slider8: Entity<SliderState>,
    slider9: Entity<SliderState>,
    slider10: Entity<SliderState>,
}

impl SliderStyledDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        let slider1 = cx.new(|cx| {
            let mut state = SliderState::new(cx);
            state.set_value(50.0, cx);
            state
        });

        let slider2 = cx.new(|cx| {
            let mut state = SliderState::new(cx);
            state.set_value(30.0, cx);
            state
        });

        let slider3 = cx.new(|cx| {
            let mut state = SliderState::new(cx);
            state.set_value(70.0, cx);
            state
        });

        let slider4 = cx.new(|cx| {
            let mut state = SliderState::new(cx);
            state.set_value(45.0, cx);
            state
        });

        let slider5 = cx.new(|cx| {
            let mut state = SliderState::new(cx);
            state.set_value(25.0, cx);
            state
        });

        let slider6 = cx.new(|cx| {
            let mut state = SliderState::new(cx);
            state.set_value(60.0, cx);
            state
        });

        let slider7 = cx.new(|cx| {
            let mut state = SliderState::new(cx);
            state.set_value(80.0, cx);
            state
        });

        let slider8 = cx.new(|cx| {
            let mut state = SliderState::new(cx);
            state.set_value(40.0, cx);
            state
        });

        let slider9 = cx.new(|cx| {
            let mut state = SliderState::new(cx);
            state.set_value(65.0, cx);
            state
        });

        let slider10 = cx.new(|cx| {
            let mut state = SliderState::new(cx);
            state.set_value(90.0, cx);
            state
        });

        Self {
            slider1,
            slider2,
            slider3,
            slider4,
            slider5,
            slider6,
            slider7,
            slider8,
            slider9,
            slider10,
        }
    }
}

impl Render for SliderStyledDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        let value1 = self.slider1.read(cx).value();
        let value2 = self.slider2.read(cx).value();
        let value3 = self.slider3.read(cx).value();
        let value4 = self.slider4.read(cx).value();
        let value5 = self.slider5.read(cx).value();
        let value6 = self.slider6.read(cx).value();
        let value7 = self.slider7.read(cx).value();
        let value8 = self.slider8.read(cx).value();
        let value9 = self.slider9.read(cx).value();
        let value10 = self.slider10.read(cx).value();

        div()
            .size_full()
            .bg(theme.tokens.background)
            .overflow_hidden()
            .child(
                scrollable_vertical(
                    div()
                        .flex()
                        .flex_col()
                        .text_color(theme.tokens.foreground)
                        .p(px(32.0))
                        .gap(px(32.0))
                        .child(
                            VStack::new()
                                .gap(px(8.0))
                                .child(
                                    div()
                                        .text_size(px(24.0))
                                        .font_weight(FontWeight::BOLD)
                                        .child("Slider Styled Trait Customization Demo")
                                )
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child("Demonstrating interactive sliders with full GPUI styling control")
                                )
                        )
                        // 1. Default Slider with value display
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("1. Default Slider with Value Display")
                                )
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child(format!("Value: {:.0}", value1))
                                )
                                .child(
                                    Slider::new(self.slider1.clone())
                                        .show_value(true)
                                )
                        )
                        // 2. Small Size
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("2. Small Size")
                                )
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child(format!("Value: {:.0}", value2))
                                )
                                .child(
                                    Slider::new(self.slider2.clone())
                                        .size(SliderSize::Sm)
                                )
                        )
                        // 3. Large Size
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("3. Large Size")
                                )
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child(format!("Value: {:.0}", value3))
                                )
                                .child(
                                    Slider::new(self.slider3.clone())
                                        .size(SliderSize::Lg)
                                )
                        )
                        // 4. Custom Width
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("4. Custom Width")
                                )
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child(format!("Value: {:.0}", value4))
                                )
                                .child(
                                    Slider::new(self.slider4.clone())
                                        .w(px(400.0))
                                )
                        )
                        // 5. Custom Background & Padding
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("5. Custom Background & Padding")
                                )
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child(format!("Value: {:.0}", value5))
                                )
                                .child(
                                    Slider::new(self.slider5.clone())
                                        .bg(rgb(0x1e293b))
                                        .p(px(16.0))
                                        .rounded(px(12.0))
                                )
                        )
                        // 6. Custom Border
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("6. Custom Border")
                                )
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child(format!("Value: {:.0}", value6))
                                )
                                .child(
                                    Slider::new(self.slider6.clone())
                                        .border_2()
                                        .border_color(rgb(0x3b82f6))
                                        .p(px(8.0))
                                        .rounded(px(8.0))
                                )
                        )
                        // 7. Combined Styling with onChange
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("7. Combined Styling with onChange Handler")
                                )
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child(format!("Value: {:.0} (drag to see console logs)", value7))
                                )
                                .child({
                                    Slider::new(self.slider7.clone())
                                        .size(SliderSize::Lg)
                                        .show_value(true)
                                        .w(px(600.0))
                                        .bg(rgb(0x0f172a))
                                        .p(px(12.0))
                                        .rounded(px(24.0))
                                        .border_2()
                                        .border_color(rgb(0x8b5cf6))
                                        .on_change(|value, _, _| {
                                            println!("Slider 7 changed to: {:.0}", value);
                                        })
                                })
                        )
                        // Vertical Sliders Section
                        .child(
                            div()
                                .mt(px(32.0))
                                .text_size(px(20.0))
                                .font_weight(FontWeight::BOLD)
                                .child("Vertical Sliders")
                        )
                        // 8. Basic Vertical Slider
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("8. Basic Vertical Slider")
                                )
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child(format!("Value: {:.0}", value8))
                                )
                                .child(
                                    div()
                                        .flex()
                                        .justify_center()
                                        .h(px(200.0))
                                        .child(
                                            Slider::new(self.slider8.clone())
                                                .vertical()
                                        )
                                )
                        )
                        // 9. Large Vertical Slider with Value Display
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("9. Large Vertical Slider with Value Display")
                                )
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child(format!("Value: {:.0}", value9))
                                )
                                .child(
                                    div()
                                        .flex()
                                        .justify_center()
                                        .h(px(250.0))
                                        .child(
                                            Slider::new(self.slider9.clone())
                                                .vertical()
                                                .size(SliderSize::Lg)
                                                .show_value(true)
                                        )
                                )
                        )
                        // 10. Styled Vertical Slider
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("10. Styled Vertical Slider with onChange")
                                )
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child(format!("Value: {:.0} (drag to see console logs)", value10))
                                )
                                .child(
                                    div()
                                        .flex()
                                        .justify_center()
                                        .h(px(300.0))
                                        .child(
                                            Slider::new(self.slider10.clone())
                                                .vertical()
                                                .size(SliderSize::Lg)
                                                .show_value(true)
                                                .bg(rgb(0x0f172a))
                                                .p(px(12.0))
                                                .rounded(px(24.0))
                                                .border_2()
                                                .border_color(rgb(0x10b981))
                                                .on_change(|value, _, _| {
                                                    println!("Vertical slider 10 changed to: {:.0}", value);
                                                })
                                        )
                                )
                        )
                        // Info Box
                        .child(
                            div()
                                .mt(px(16.0))
                                .p(px(16.0))
                                .bg(theme.tokens.accent)
                                .rounded(px(8.0))
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.accent_foreground)
                                        .child("✅ All sliders are fully interactive with drag support!")
                                )
                                .child(
                                    div()
                                        .mt(px(8.0))
                                        .text_size(px(12.0))
                                        .text_color(theme.tokens.accent_foreground)
                                        .child("Features: Horizontal and vertical orientations, drag thumbs to change values, keyboard focus support, size variants, and full Styled trait customization")
                                )
                        )
                )
            )
    }
}
