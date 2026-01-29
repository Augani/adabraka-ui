use adabraka_ui::{
    components::carousel::{
        Carousel, CarouselSize, CarouselSlide, CarouselState, CarouselTransition,
    },
    components::scrollable::scrollable_vertical,
    prelude::*,
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
                        title: Some("Carousel Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(1000.0), px(900.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|cx| CarouselDemo::new(cx)),
            )
            .unwrap();
        });
}

struct CarouselDemo {
    carousel1: Entity<CarouselState>,
    carousel2: Entity<CarouselState>,
    carousel3: Entity<CarouselState>,
    carousel4: Entity<CarouselState>,
    carousel5: Entity<CarouselState>,
}

impl CarouselDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        let carousel1 = cx.new(|cx| CarouselState::new(cx));
        let carousel2 = cx.new(|cx| CarouselState::new(cx));
        let carousel3 = cx.new(|cx| CarouselState::new(cx));
        let carousel4 = cx.new(|cx| CarouselState::new(cx));
        let carousel5 = cx.new(|cx| CarouselState::new(cx));

        Self {
            carousel1,
            carousel2,
            carousel3,
            carousel4,
            carousel5,
        }
    }
}

fn create_slide(index: usize, color: Hsla, theme: &Theme) -> CarouselSlide {
    CarouselSlide::new(
        div()
            .h(px(200.0))
            .w_full()
            .bg(color)
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .text_size(px(32.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(theme.tokens.background)
                    .child(format!("Slide {}", index + 1)),
            ),
    )
}

fn create_image_slide(index: usize, _theme: &Theme) -> CarouselSlide {
    let images = [
        "assets/images/carousel_1.jpg",
        "assets/images/carousel_2.jpg",
        "assets/images/carousel_3.jpg",
        "assets/images/carousel_4.jpg",
        "assets/images/carousel_5.jpg",
    ];
    let image_path = images[index % images.len()];

    CarouselSlide::new(
        div()
            .h(px(300.0))
            .w_full()
            .overflow_hidden()
            .relative()
            .child(img(image_path).size_full().object_fit(ObjectFit::Cover))
            .child(
                div()
                    .absolute()
                    .bottom_0()
                    .left_0()
                    .right_0()
                    .p(px(16.0))
                    .bg(rgba(0x00000080))
                    .flex()
                    .flex_col()
                    .gap(px(4.0))
                    .child(
                        div()
                            .text_size(px(24.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(white())
                            .child(format!("Image {}", index + 1)),
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(rgba(0xffffffcc))
                            .child("Click arrows or use keyboard to navigate"),
                    ),
            ),
    )
}

impl Render for CarouselDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        let current1 = self.carousel1.read(cx).current_index();
        let current2 = self.carousel2.read(cx).current_index();
        let current3 = self.carousel3.read(cx).current_index();
        let current4 = self.carousel4.read(cx).current_index();
        let current5 = self.carousel5.read(cx).current_index();

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
                        .gap(px(40.0))
                        .child(
                            VStack::new()
                                .gap(px(8.0))
                                .child(
                                    div()
                                        .text_size(px(28.0))
                                        .font_weight(FontWeight::BOLD)
                                        .child("Carousel Component Demo")
                                )
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child("A flexible carousel with navigation arrows, dots, keyboard support, and transitions")
                                )
                        )
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(20.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("1. Basic Carousel")
                                )
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child(format!("Current slide: {} / 5", current1 + 1))
                                )
                                .child(
                                    Carousel::new("carousel-1", self.carousel1.clone())
                                        .slides(vec![
                                            create_slide(0, rgb(0x3b82f6).into(), &theme),
                                            create_slide(1, rgb(0x10b981).into(), &theme),
                                            create_slide(2, rgb(0xf59e0b).into(), &theme),
                                            create_slide(3, rgb(0xef4444).into(), &theme),
                                            create_slide(4, rgb(0x8b5cf6).into(), &theme),
                                        ])
                                        .on_change(|idx, _, _| {
                                            println!("Carousel 1 changed to slide: {}", idx + 1);
                                        })
                                )
                        )
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(20.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("2. Infinite Loop Carousel")
                                )
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child(format!("Current slide: {} / 3 (loops infinitely)", current2 + 1))
                                )
                                .child(
                                    Carousel::new("carousel-2", self.carousel2.clone())
                                        .infinite(true)
                                        .slides(vec![
                                            create_image_slide(0, &theme),
                                            create_image_slide(1, &theme),
                                            create_image_slide(2, &theme),
                                        ])
                                        .on_change(|idx, _, _| {
                                            println!("Carousel 2 (infinite) changed to slide: {}", idx + 1);
                                        })
                                )
                        )
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(20.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("3. Large Size with Fade Transition")
                                )
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child(format!("Current slide: {} / 4", current3 + 1))
                                )
                                .child(
                                    Carousel::new("carousel-3", self.carousel3.clone())
                                        .size(CarouselSize::Lg)
                                        .transition(CarouselTransition::Fade)
                                        .slides(vec![
                                            CarouselSlide::new(
                                                div()
                                                    .h(px(250.0))
                                                    .w_full()
                                                    .bg(rgb(0x1e40af))
                                                    .flex()
                                                    .items_center()
                                                    .justify_center()
                                                    .child(
                                                        div()
                                                            .text_size(px(24.0))
                                                            .text_color(white())
                                                            .child("Fade Transition 1")
                                                    )
                                            ),
                                            CarouselSlide::new(
                                                div()
                                                    .h(px(250.0))
                                                    .w_full()
                                                    .bg(rgb(0x047857))
                                                    .flex()
                                                    .items_center()
                                                    .justify_center()
                                                    .child(
                                                        div()
                                                            .text_size(px(24.0))
                                                            .text_color(white())
                                                            .child("Fade Transition 2")
                                                    )
                                            ),
                                            CarouselSlide::new(
                                                div()
                                                    .h(px(250.0))
                                                    .w_full()
                                                    .bg(rgb(0xb45309))
                                                    .flex()
                                                    .items_center()
                                                    .justify_center()
                                                    .child(
                                                        div()
                                                            .text_size(px(24.0))
                                                            .text_color(white())
                                                            .child("Fade Transition 3")
                                                    )
                                            ),
                                            CarouselSlide::new(
                                                div()
                                                    .h(px(250.0))
                                                    .w_full()
                                                    .bg(rgb(0xb91c1c))
                                                    .flex()
                                                    .items_center()
                                                    .justify_center()
                                                    .child(
                                                        div()
                                                            .text_size(px(24.0))
                                                            .text_color(white())
                                                            .child("Fade Transition 4")
                                                    )
                                            ),
                                        ])
                                )
                        )
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(20.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("4. Small Size - Dots Only")
                                )
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child(format!("Current slide: {} / 3", current4 + 1))
                                )
                                .child(
                                    Carousel::new("carousel-4", self.carousel4.clone())
                                        .size(CarouselSize::Sm)
                                        .show_arrows(false)
                                        .slides(vec![
                                            CarouselSlide::new(
                                                div()
                                                    .h(px(150.0))
                                                    .w_full()
                                                    .bg(rgb(0x6366f1))
                                                    .flex()
                                                    .items_center()
                                                    .justify_center()
                                                    .child(
                                                        div()
                                                            .text_size(px(20.0))
                                                            .text_color(white())
                                                            .child("Click dots to navigate")
                                                    )
                                            ),
                                            CarouselSlide::new(
                                                div()
                                                    .h(px(150.0))
                                                    .w_full()
                                                    .bg(rgb(0xec4899))
                                                    .flex()
                                                    .items_center()
                                                    .justify_center()
                                                    .child(
                                                        div()
                                                            .text_size(px(20.0))
                                                            .text_color(white())
                                                            .child("Slide 2")
                                                    )
                                            ),
                                            CarouselSlide::new(
                                                div()
                                                    .h(px(150.0))
                                                    .w_full()
                                                    .bg(rgb(0x14b8a6))
                                                    .flex()
                                                    .items_center()
                                                    .justify_center()
                                                    .child(
                                                        div()
                                                            .text_size(px(20.0))
                                                            .text_color(white())
                                                            .child("Slide 3")
                                                    )
                                            ),
                                        ])
                                )
                        )
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(20.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("5. Custom Styled Carousel")
                                )
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child(format!("Current slide: {} / 4", current5 + 1))
                                )
                                .child(
                                    Carousel::new("carousel-5", self.carousel5.clone())
                                        .infinite(true)
                                        .border_2()
                                        .border_color(rgb(0x8b5cf6))
                                        .rounded(px(16.0))
                                        .slides(vec![
                                            CarouselSlide::new(
                                                div()
                                                    .h(px(200.0))
                                                    .w_full()
                                                    .bg(rgb(0x0f172a))
                                                    .flex()
                                                    .flex_col()
                                                    .items_center()
                                                    .justify_center()
                                                    .gap(px(12.0))
                                                    .child(
                                                        div()
                                                            .text_size(px(24.0))
                                                            .font_weight(FontWeight::BOLD)
                                                            .text_color(rgb(0x8b5cf6))
                                                            .child("Custom Styled")
                                                    )
                                                    .child(
                                                        div()
                                                            .text_size(px(14.0))
                                                            .text_color(theme.tokens.muted_foreground)
                                                            .child("With purple border and rounded corners")
                                                    )
                                            ),
                                            CarouselSlide::new(
                                                div()
                                                    .h(px(200.0))
                                                    .w_full()
                                                    .bg(rgb(0x0f172a))
                                                    .flex()
                                                    .items_center()
                                                    .justify_center()
                                                    .child(
                                                        div()
                                                            .text_size(px(24.0))
                                                            .text_color(rgb(0x10b981))
                                                            .child("Infinite Loop Enabled")
                                                    )
                                            ),
                                            CarouselSlide::new(
                                                div()
                                                    .h(px(200.0))
                                                    .w_full()
                                                    .bg(rgb(0x0f172a))
                                                    .flex()
                                                    .items_center()
                                                    .justify_center()
                                                    .child(
                                                        div()
                                                            .text_size(px(24.0))
                                                            .text_color(rgb(0xf59e0b))
                                                            .child("Keyboard Navigation")
                                                    )
                                            ),
                                            CarouselSlide::new(
                                                div()
                                                    .h(px(200.0))
                                                    .w_full()
                                                    .bg(rgb(0x0f172a))
                                                    .flex()
                                                    .items_center()
                                                    .justify_center()
                                                    .child(
                                                        div()
                                                            .text_size(px(24.0))
                                                            .text_color(rgb(0xef4444))
                                                            .child("Arrow Keys: Left / Right")
                                                    )
                                            ),
                                        ])
                                        .on_change(|idx, _, _| {
                                            println!("Custom carousel changed to: {}", idx + 1);
                                        })
                                )
                        )
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
                                        .child("Keyboard Navigation: Arrow Left/Right to navigate, Home/End to jump to first/last slide")
                                )
                                .child(
                                    div()
                                        .mt(px(8.0))
                                        .text_size(px(12.0))
                                        .text_color(theme.tokens.accent_foreground)
                                        .child("Features: Slide/Fade transitions, infinite loop, dot indicators, navigation arrows, size variants")
                                )
                        )
                )
            )
    }
}
