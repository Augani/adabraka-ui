use adabraka_ui::{
    prelude::*,
    components::{scrollable::scrollable_vertical, pagination::Pagination},
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
                        title: Some("Pagination Styled Trait Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(900.0), px(800.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|_| PaginationStyledDemo::new()),
            )
            .unwrap();
        });
}

struct PaginationStyledDemo {
    current_page_1: usize,
    current_page_2: usize,
    current_page_3: usize,
    current_page_4: usize,
    current_page_5: usize,
    current_page_6: usize,
    current_page_7: usize,
}

impl PaginationStyledDemo {
    fn new() -> Self {
        Self {
            current_page_1: 1,
            current_page_2: 1,
            current_page_3: 1,
            current_page_4: 1,
            current_page_5: 1,
            current_page_6: 1,
            current_page_7: 1,
        }
    }
}

impl Render for PaginationStyledDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();
        let view_handle = cx.entity().downgrade();

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
                            .child("Pagination Styled Trait Customization Demo")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("Demonstrating full GPUI styling control via Styled trait on Pagination")
                    )
            )
            // 1. Custom Padding Examples
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("1. Custom Padding (via Styled trait)")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child(format!("Current page: {}", self.current_page_1))
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(
                                div()
                                    .child(div().text_size(px(13.0)).child("Default padding:"))
                                    .child({
                                        let view = view_handle.clone();
                                        Pagination::new()
                                            .current_page(self.current_page_1)
                                            .total_pages(10)
                                            .on_page_change(move |page: usize, _: &mut Window, cx: &mut App| {
                                                if let Some(view) = view.upgrade() {
                                                    view.update(cx, |view, cx: &mut Context<'_, PaginationStyledDemo>| {
                                                        view.current_page_1 = page;
                                                        cx.notify();
                                                    });
                                                }
                                            })
                                    })
                            )
                            .child(
                                div()
                                    .child(div().text_size(px(13.0)).child("Custom p_4():"))
                                    .child({
                                        let view = view_handle.clone();
                                        Pagination::new()
                                            .current_page(self.current_page_1)
                                            .total_pages(10)
                                            .p_4()  // <- Styled trait method
                                            .on_page_change(move |page: usize, _: &mut Window, cx: &mut App| {
                                                if let Some(view) = view.upgrade() {
                                                    view.update(cx, |view, cx: &mut Context<'_, PaginationStyledDemo>| {
                                                        view.current_page_1 = page;
                                                        cx.notify();
                                                    });
                                                }
                                            })
                                    })
                            )
                            .child(
                                div()
                                    .child(div().text_size(px(13.0)).child("Custom p_8():"))
                                    .child({
                                        let view = view_handle.clone();
                                        Pagination::new()
                                            .current_page(self.current_page_1)
                                            .total_pages(10)
                                            .p_8()  // <- Styled trait method
                                            .on_page_change(move |page: usize, _: &mut Window, cx: &mut App| {
                                                if let Some(view) = view.upgrade() {
                                                    view.update(cx, |view, cx: &mut Context<'_, PaginationStyledDemo>| {
                                                        view.current_page_1 = page;
                                                        cx.notify();
                                                    });
                                                }
                                            })
                                    })
                            )
                    )
            )
            // 2. Custom Background Colors
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("2. Custom Background Colors")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child(format!("Current page: {}", self.current_page_2))
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(
                                div()
                                    .child(div().text_size(px(13.0)).child("Blue background with padding:"))
                                    .child({
                                        let view = view_handle.clone();
                                        Pagination::new()
                                            .current_page(self.current_page_2)
                                            .total_pages(10)
                                            .bg(rgb(0x1e3a8a))  // <- Styled trait
                                            .p_4()
                                            .rounded(px(8.0))
                                            .on_page_change(move |page: usize, _: &mut Window, cx: &mut App| {
                                                if let Some(view) = view.upgrade() {
                                                    view.update(cx, |view, cx: &mut Context<'_, PaginationStyledDemo>| {
                                                        view.current_page_2 = page;
                                                        cx.notify();
                                                    });
                                                }
                                            })
                                    })
                            )
                            .child(
                                div()
                                    .child(div().text_size(px(13.0)).child("Purple background with padding:"))
                                    .child({
                                        let view = view_handle.clone();
                                        Pagination::new()
                                            .current_page(self.current_page_2)
                                            .total_pages(10)
                                            .bg(rgb(0x581c87))  // <- Styled trait
                                            .p_4()
                                            .rounded(px(8.0))
                                            .on_page_change(move |page: usize, _: &mut Window, cx: &mut App| {
                                                if let Some(view) = view.upgrade() {
                                                    view.update(cx, |view, cx: &mut Context<'_, PaginationStyledDemo>| {
                                                        view.current_page_2 = page;
                                                        cx.notify();
                                                    });
                                                }
                                            })
                                    })
                            )
                    )
            )
            // 3. Custom Borders
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("3. Custom Borders")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child(format!("Current page: {}", self.current_page_3))
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(
                                div()
                                    .child(div().text_size(px(13.0)).child("Blue 2px border:"))
                                    .child({
                                        let view = view_handle.clone();
                                        Pagination::new()
                                            .current_page(self.current_page_3)
                                            .total_pages(10)
                                            .border_2()  // <- Styled trait
                                            .border_color(rgb(0x3b82f6))
                                            .rounded(px(8.0))
                                            .p_4()
                                            .on_page_change(move |page: usize, _: &mut Window, cx: &mut App| {
                                                if let Some(view) = view.upgrade() {
                                                    view.update(cx, |view, cx: &mut Context<'_, PaginationStyledDemo>| {
                                                        view.current_page_3 = page;
                                                        cx.notify();
                                                    });
                                                }
                                            })
                                    })
                            )
                            .child(
                                div()
                                    .child(div().text_size(px(13.0)).child("Red 2px border:"))
                                    .child({
                                        let view = view_handle.clone();
                                        Pagination::new()
                                            .current_page(self.current_page_3)
                                            .total_pages(10)
                                            .border_2()  // <- Styled trait
                                            .border_color(rgb(0xef4444))
                                            .rounded(px(8.0))
                                            .p_4()
                                            .on_page_change(move |page: usize, _: &mut Window, cx: &mut App| {
                                                if let Some(view) = view.upgrade() {
                                                    view.update(cx, |view, cx: &mut Context<'_, PaginationStyledDemo>| {
                                                        view.current_page_3 = page;
                                                        cx.notify();
                                                    });
                                                }
                                            })
                                    })
                            )
                    )
            )
            // 4. Custom Border Radius
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("4. Custom Border Radius")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child(format!("Current page: {}", self.current_page_4))
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(
                                div()
                                    .child(div().text_size(px(13.0)).child("No radius (sharp corners):"))
                                    .child({
                                        let view = view_handle.clone();
                                        Pagination::new()
                                            .current_page(self.current_page_4)
                                            .total_pages(10)
                                            .rounded(px(0.0))  // <- Styled trait
                                            .bg(theme.tokens.muted)
                                            .p_4()
                                            .on_page_change(move |page: usize, _: &mut Window, cx: &mut App| {
                                                if let Some(view) = view.upgrade() {
                                                    view.update(cx, |view, cx: &mut Context<'_, PaginationStyledDemo>| {
                                                        view.current_page_4 = page;
                                                        cx.notify();
                                                    });
                                                }
                                            })
                                    })
                            )
                            .child(
                                div()
                                    .child(div().text_size(px(13.0)).child("Large radius (16px):"))
                                    .child({
                                        let view = view_handle.clone();
                                        Pagination::new()
                                            .current_page(self.current_page_4)
                                            .total_pages(10)
                                            .rounded(px(16.0))  // <- Styled trait
                                            .bg(theme.tokens.muted)
                                            .p_4()
                                            .on_page_change(move |page: usize, _: &mut Window, cx: &mut App| {
                                                if let Some(view) = view.upgrade() {
                                                    view.update(cx, |view, cx: &mut Context<'_, PaginationStyledDemo>| {
                                                        view.current_page_4 = page;
                                                        cx.notify();
                                                    });
                                                }
                                            })
                                    })
                            )
                            .child(
                                div()
                                    .child(div().text_size(px(13.0)).child("Pill shape (999px):"))
                                    .child({
                                        let view = view_handle.clone();
                                        Pagination::new()
                                            .current_page(self.current_page_4)
                                            .total_pages(10)
                                            .rounded(px(999.0))  // <- Styled trait
                                            .bg(theme.tokens.muted)
                                            .p_4()
                                            .on_page_change(move |page: usize, _: &mut Window, cx: &mut App| {
                                                if let Some(view) = view.upgrade() {
                                                    view.update(cx, |view, cx: &mut Context<'_, PaginationStyledDemo>| {
                                                        view.current_page_4 = page;
                                                        cx.notify();
                                                    });
                                                }
                                            })
                                    })
                            )
                    )
            )
            // 5. Width Control
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("5. Width Control")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child(format!("Current page: {}", self.current_page_5))
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(
                                div()
                                    .child(div().text_size(px(13.0)).child("Full width:"))
                                    .child({
                                        let view = view_handle.clone();
                                        Pagination::new()
                                            .current_page(self.current_page_5)
                                            .total_pages(10)
                                            .w_full()  // <- Styled trait
                                            .justify_center()
                                            .bg(theme.tokens.muted)
                                            .p_4()
                                            .rounded(px(8.0))
                                            .on_page_change(move |page: usize, _: &mut Window, cx: &mut App| {
                                                if let Some(view) = view.upgrade() {
                                                    view.update(cx, |view, cx: &mut Context<'_, PaginationStyledDemo>| {
                                                        view.current_page_5 = page;
                                                        cx.notify();
                                                    });
                                                }
                                            })
                                    })
                            )
                            .child(
                                div()
                                    .child(div().text_size(px(13.0)).child("Custom width (600px):"))
                                    .child({
                                        let view = view_handle.clone();
                                        Pagination::new()
                                            .current_page(self.current_page_5)
                                            .total_pages(10)
                                            .w(px(600.0))  // <- Styled trait
                                            .justify_center()
                                            .bg(theme.tokens.muted)
                                            .p_4()
                                            .rounded(px(8.0))
                                            .on_page_change(move |page: usize, _: &mut Window, cx: &mut App| {
                                                if let Some(view) = view.upgrade() {
                                                    view.update(cx, |view, cx: &mut Context<'_, PaginationStyledDemo>| {
                                                        view.current_page_5 = page;
                                                        cx.notify();
                                                    });
                                                }
                                            })
                                    })
                            )
                    )
            )
            // 6. Shadow Effects
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("6. Shadow Effects")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child(format!("Current page: {}", self.current_page_6))
                    )
                    .child(
                        VStack::new()
                            .gap(px(16.0))
                            .child(
                                div()
                                    .child(div().text_size(px(13.0)).child("Shadow small:"))
                                    .child({
                                        let view = view_handle.clone();
                                        Pagination::new()
                                            .current_page(self.current_page_6)
                                            .total_pages(10)
                                            .shadow_sm()  // <- Styled trait
                                            .bg(theme.tokens.card)
                                            .p_4()
                                            .rounded(px(8.0))
                                            .on_page_change(move |page: usize, _: &mut Window, cx: &mut App| {
                                                if let Some(view) = view.upgrade() {
                                                    view.update(cx, |view, cx: &mut Context<'_, PaginationStyledDemo>| {
                                                        view.current_page_6 = page;
                                                        cx.notify();
                                                    });
                                                }
                                            })
                                    })
                            )
                            .child(
                                div()
                                    .child(div().text_size(px(13.0)).child("Shadow medium:"))
                                    .child({
                                        let view = view_handle.clone();
                                        Pagination::new()
                                            .current_page(self.current_page_6)
                                            .total_pages(10)
                                            .shadow_md()  // <- Styled trait
                                            .bg(theme.tokens.card)
                                            .p_4()
                                            .rounded(px(8.0))
                                            .on_page_change(move |page: usize, _: &mut Window, cx: &mut App| {
                                                if let Some(view) = view.upgrade() {
                                                    view.update(cx, |view, cx: &mut Context<'_, PaginationStyledDemo>| {
                                                        view.current_page_6 = page;
                                                        cx.notify();
                                                    });
                                                }
                                            })
                                    })
                            )
                            .child(
                                div()
                                    .child(div().text_size(px(13.0)).child("Shadow large:"))
                                    .child({
                                        let view = view_handle.clone();
                                        Pagination::new()
                                            .current_page(self.current_page_6)
                                            .total_pages(10)
                                            .shadow_lg()  // <- Styled trait
                                            .bg(theme.tokens.card)
                                            .p_4()
                                            .rounded(px(8.0))
                                            .on_page_change(move |page: usize, _: &mut Window, cx: &mut App| {
                                                if let Some(view) = view.upgrade() {
                                                    view.update(cx, |view, cx: &mut Context<'_, PaginationStyledDemo>| {
                                                        view.current_page_6 = page;
                                                        cx.notify();
                                                    });
                                                }
                                            })
                                    })
                            )
                    )
            )
            // 7. Combined Styling
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("7. Combined Styling (Multiple Styled Trait Methods)")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child(format!("Current page: {}", self.current_page_7))
                    )
                    .child(
                        VStack::new()
                            .gap(px(16.0))
                            .child(
                                div()
                                    .child(div().text_size(px(13.0)).child("Blue box with shadow and border:"))
                                    .child({
                                        let view = view_handle.clone();
                                        Pagination::new()
                                            .current_page(self.current_page_7)
                                            .total_pages(10)
                                            .bg(rgb(0x1e3a8a))  // <- Styled trait
                                            .p_8()  // <- Styled trait
                                            .rounded(px(12.0))  // <- Styled trait
                                            .shadow_lg()  // <- Styled trait
                                            .border_2()  // <- Styled trait
                                            .border_color(rgb(0x3b82f6))
                                            .on_page_change(move |page: usize, _: &mut Window, cx: &mut App| {
                                                if let Some(view) = view.upgrade() {
                                                    view.update(cx, |view, cx: &mut Context<'_, PaginationStyledDemo>| {
                                                        view.current_page_7 = page;
                                                        cx.notify();
                                                    });
                                                }
                                            })
                                    })
                            )
                            .child(
                                div()
                                    .child(div().text_size(px(13.0)).child("Full width with custom styling:"))
                                    .child({
                                        let view = view_handle.clone();
                                        Pagination::new()
                                            .current_page(self.current_page_7)
                                            .total_pages(10)
                                            .w_full()  // <- Styled trait
                                            .justify_center()
                                            .p(px(20.0))  // <- Styled trait
                                            .bg(rgb(0x581c87))  // <- Styled trait
                                            .rounded(px(16.0))  // <- Styled trait
                                            .shadow_md()  // <- Styled trait
                                            .border_2()  // <- Styled trait
                                            .border_color(rgb(0x8b5cf6))
                                            .on_page_change(move |page: usize, _: &mut Window, cx: &mut App| {
                                                if let Some(view) = view.upgrade() {
                                                    view.update(cx, |view, cx: &mut Context<'_, PaginationStyledDemo>| {
                                                        view.current_page_7 = page;
                                                        cx.notify();
                                                    });
                                                }
                                            })
                                    })
                            )
                            .child(
                                div()
                                    .child(div().text_size(px(13.0)).child("Ultra custom with pill shape:"))
                                    .child({
                                        let view = view_handle.clone();
                                        Pagination::new()
                                            .current_page(self.current_page_7)
                                            .total_pages(10)
                                            .bg(rgb(0x065f46))  // <- Styled trait
                                            .px(px(32.0))  // <- Styled trait
                                            .py(px(16.0))  // <- Styled trait
                                            .rounded(px(999.0))  // <- Styled trait
                                            .shadow_lg()  // <- Styled trait
                                            .border_2()  // <- Styled trait
                                            .border_color(rgb(0x10b981))
                                            .on_page_change(move |page: usize, _: &mut Window, cx: &mut App| {
                                                if let Some(view) = view.upgrade() {
                                                    view.update(cx, |view, cx: &mut Context<'_, PaginationStyledDemo>| {
                                                        view.current_page_7 = page;
                                                        cx.notify();
                                                    });
                                                }
                                            })
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
                            .child("All customizations above use the Styled trait for full GPUI styling control!")
                    )
                    .child(
                        div()
                            .mt(px(8.0))
                            .text_size(px(12.0))
                            .text_color(theme.tokens.accent_foreground)
                            .child("Methods used: .p_4(), .p_8(), .p(), .px(), .py(), .bg(), .border_2(), .border_color(), .rounded(), .w_full(), .w(), .shadow_sm/md/lg()")
                    )
            )
                )
            )
    }
}
