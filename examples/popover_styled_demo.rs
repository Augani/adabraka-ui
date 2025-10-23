use adabraka_ui::{
    prelude::*,
    components::scrollable::scrollable_vertical,
    overlays::popover::{Popover, PopoverContent},
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
                        title: Some("Popover Styled Trait Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(900.0), px(800.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|_| PopoverStyledDemo::new()),
            )
            .unwrap();
        });
}

struct PopoverStyledDemo {
    popover_count: usize,
}

impl PopoverStyledDemo {
    fn new() -> Self {
        Self {
            popover_count: 0,
        }
    }
}

impl Render for PopoverStyledDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

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
                            .child("Popover Styled Trait Customization Demo")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("Demonstrating full GPUI styling control via Styled trait on Popover trigger wrapper")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.accent)
                            .child(format!("Total popover opens: {}", self.popover_count))
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
                        HStack::new()
                            .gap(px(12.0))
                            .items_center()
                            .child(
                                Popover::new("default-padding")
                                    .trigger(Button::new("default-padding-btn", "Default Padding").variant(ButtonVariant::Outline))
                                    .content(|window, cx| {
                                        cx.new(|cx| {
                                            PopoverContent::new(window, cx, |_, _| {
                                                div()
                                                    .child(
                                                        div()
                                                            .text_size(px(14.0))
                                                            .font_weight(FontWeight::SEMIBOLD)
                                                            .child("Default Padding")
                                                    )
                                                    .child(
                                                        div()
                                                            .text_size(px(13.0))
                                                            .child("No custom styling applied")
                                                    )
                                                    .into_any_element()
                                            })
                                        })
                                    })
                            )
                            .child(
                                Popover::new("custom-p4")
                                    .p_4()  // <- Styled trait method
                                    .trigger(Button::new("custom-p4-btn", "Custom p_4()").variant(ButtonVariant::Outline))
                                    .content(|window, cx| {
                                        cx.new(|cx| {
                                            PopoverContent::new(window, cx, |_, _| {
                                                div()
                                                    .child(
                                                        div()
                                                            .text_size(px(14.0))
                                                            .font_weight(FontWeight::SEMIBOLD)
                                                            .child("With p_4()")
                                                    )
                                                    .child(
                                                        div()
                                                            .text_size(px(13.0))
                                                            .child("Wrapper has custom padding")
                                                    )
                                                    .into_any_element()
                                            })
                                        })
                                    })
                            )
                            .child(
                                Popover::new("custom-p8")
                                    .p_8()  // <- Styled trait method
                                    .trigger(Button::new("custom-p8-btn", "Custom p_8()").variant(ButtonVariant::Outline))
                                    .content(|window, cx| {
                                        cx.new(|cx| {
                                            PopoverContent::new(window, cx, |_, _| {
                                                div()
                                                    .child(
                                                        div()
                                                            .text_size(px(14.0))
                                                            .font_weight(FontWeight::SEMIBOLD)
                                                            .child("With p_8()")
                                                    )
                                                    .child(
                                                        div()
                                                            .text_size(px(13.0))
                                                            .child("Even more wrapper padding")
                                                    )
                                                    .into_any_element()
                                            })
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
                        HStack::new()
                            .gap(px(12.0))
                            .child(
                                Popover::new("blue-bg")
                                    .bg(rgb(0x3b82f6))  // <- Styled trait
                                    .rounded(px(8.0))
                                    .p(px(4.0))
                                    .trigger(Button::new("blue-bg-btn", "Blue Wrapper").variant(ButtonVariant::Ghost))
                                    .content(|window, cx| {
                                        cx.new(|cx| {
                                            PopoverContent::new(window, cx, |_, _| {
                                                div()
                                                    .child(
                                                        div()
                                                            .text_size(px(14.0))
                                                            .font_weight(FontWeight::SEMIBOLD)
                                                            .child("Blue Background")
                                                    )
                                                    .child(
                                                        div()
                                                            .text_size(px(13.0))
                                                            .child("The trigger wrapper has a blue background")
                                                    )
                                                    .into_any_element()
                                            })
                                        })
                                    })
                            )
                            .child(
                                Popover::new("purple-bg")
                                    .bg(rgb(0x8b5cf6))  // <- Styled trait
                                    .rounded(px(8.0))
                                    .p(px(4.0))
                                    .trigger(Button::new("purple-bg-btn", "Purple Wrapper").variant(ButtonVariant::Ghost))
                                    .content(|window, cx| {
                                        cx.new(|cx| {
                                            PopoverContent::new(window, cx, |_, _| {
                                                div()
                                                    .child(
                                                        div()
                                                            .text_size(px(14.0))
                                                            .font_weight(FontWeight::SEMIBOLD)
                                                            .child("Purple Background")
                                                    )
                                                    .child(
                                                        div()
                                                            .text_size(px(13.0))
                                                            .child("The trigger wrapper has a purple background")
                                                    )
                                                    .into_any_element()
                                            })
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
                        HStack::new()
                            .gap(px(12.0))
                            .child(
                                Popover::new("border-2")
                                    .border_2()  // <- Styled trait
                                    .border_color(rgb(0x3b82f6))
                                    .rounded(px(8.0))
                                    .p(px(4.0))
                                    .trigger(Button::new("border-2-btn", "Border 2px").variant(ButtonVariant::Ghost))
                                    .content(|window, cx| {
                                        cx.new(|cx| {
                                            PopoverContent::new(window, cx, |_, _| {
                                                div()
                                                    .child(
                                                        div()
                                                            .text_size(px(14.0))
                                                            .font_weight(FontWeight::SEMIBOLD)
                                                            .child("Blue Border")
                                                    )
                                                    .child(
                                                        div()
                                                            .text_size(px(13.0))
                                                            .child("Wrapper has a 2px blue border")
                                                    )
                                                    .into_any_element()
                                            })
                                        })
                                    })
                            )
                            .child(
                                Popover::new("border-red")
                                    .border_2()  // <- Styled trait
                                    .border_color(rgb(0xef4444))
                                    .rounded(px(8.0))
                                    .p(px(4.0))
                                    .trigger(Button::new("border-red-btn", "Red Border").variant(ButtonVariant::Ghost))
                                    .content(|window, cx| {
                                        cx.new(|cx| {
                                            PopoverContent::new(window, cx, |_, _| {
                                                div()
                                                    .child(
                                                        div()
                                                            .text_size(px(14.0))
                                                            .font_weight(FontWeight::SEMIBOLD)
                                                            .child("Red Border")
                                                    )
                                                    .child(
                                                        div()
                                                            .text_size(px(13.0))
                                                            .child("Wrapper has a 2px red border")
                                                    )
                                                    .into_any_element()
                                            })
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
                        HStack::new()
                            .gap(px(12.0))
                            .child(
                                Popover::new("no-radius")
                                    .rounded(px(0.0))  // <- Styled trait
                                    .bg(theme.tokens.accent)
                                    .p(px(4.0))
                                    .trigger(Button::new("no-radius-btn", "No Radius").variant(ButtonVariant::Ghost))
                                    .content(|window, cx| {
                                        cx.new(|cx| {
                                            PopoverContent::new(window, cx, |_, _| {
                                                div()
                                                    .child(
                                                        div()
                                                            .text_size(px(14.0))
                                                            .font_weight(FontWeight::SEMIBOLD)
                                                            .child("Square Wrapper")
                                                    )
                                                    .child(
                                                        div()
                                                            .text_size(px(13.0))
                                                            .child("No border radius on wrapper")
                                                    )
                                                    .into_any_element()
                                            })
                                        })
                                    })
                            )
                            .child(
                                Popover::new("rounded-lg")
                                    .rounded(px(16.0))  // <- Styled trait
                                    .bg(theme.tokens.accent)
                                    .p(px(4.0))
                                    .trigger(Button::new("rounded-lg-btn", "Large Radius").variant(ButtonVariant::Ghost))
                                    .content(|window, cx| {
                                        cx.new(|cx| {
                                            PopoverContent::new(window, cx, |_, _| {
                                                div()
                                                    .child(
                                                        div()
                                                            .text_size(px(14.0))
                                                            .font_weight(FontWeight::SEMIBOLD)
                                                            .child("Large Radius")
                                                    )
                                                    .child(
                                                        div()
                                                            .text_size(px(13.0))
                                                            .child("16px border radius on wrapper")
                                                    )
                                                    .into_any_element()
                                            })
                                        })
                                    })
                            )
                    )
            )
            // 5. Shadow Effects
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("5. Shadow Effects")
                    )
                    .child(
                        HStack::new()
                            .gap(px(12.0))
                            .child(
                                Popover::new("shadow-sm")
                                    .shadow_sm()  // <- Styled trait
                                    .bg(theme.tokens.card)
                                    .rounded(px(8.0))
                                    .p(px(4.0))
                                    .trigger(Button::new("shadow-sm-btn", "Shadow Small").variant(ButtonVariant::Outline))
                                    .content(|window, cx| {
                                        cx.new(|cx| {
                                            PopoverContent::new(window, cx, |_, _| {
                                                div()
                                                    .child(
                                                        div()
                                                            .text_size(px(14.0))
                                                            .font_weight(FontWeight::SEMIBOLD)
                                                            .child("Small Shadow")
                                                    )
                                                    .child(
                                                        div()
                                                            .text_size(px(13.0))
                                                            .child("Wrapper has small shadow")
                                                    )
                                                    .into_any_element()
                                            })
                                        })
                                    })
                            )
                            .child(
                                Popover::new("shadow-md")
                                    .shadow_md()  // <- Styled trait
                                    .bg(theme.tokens.card)
                                    .rounded(px(8.0))
                                    .p(px(4.0))
                                    .trigger(Button::new("shadow-md-btn", "Shadow Medium").variant(ButtonVariant::Outline))
                                    .content(|window, cx| {
                                        cx.new(|cx| {
                                            PopoverContent::new(window, cx, |_, _| {
                                                div()
                                                    .child(
                                                        div()
                                                            .text_size(px(14.0))
                                                            .font_weight(FontWeight::SEMIBOLD)
                                                            .child("Medium Shadow")
                                                    )
                                                    .child(
                                                        div()
                                                            .text_size(px(13.0))
                                                            .child("Wrapper has medium shadow")
                                                    )
                                                    .into_any_element()
                                            })
                                        })
                                    })
                            )
                            .child(
                                Popover::new("shadow-lg")
                                    .shadow_lg()  // <- Styled trait
                                    .bg(theme.tokens.card)
                                    .rounded(px(8.0))
                                    .p(px(4.0))
                                    .trigger(Button::new("shadow-lg-btn", "Shadow Large").variant(ButtonVariant::Outline))
                                    .content(|window, cx| {
                                        cx.new(|cx| {
                                            PopoverContent::new(window, cx, |_, _| {
                                                div()
                                                    .child(
                                                        div()
                                                            .text_size(px(14.0))
                                                            .font_weight(FontWeight::SEMIBOLD)
                                                            .child("Large Shadow")
                                                    )
                                                    .child(
                                                        div()
                                                            .text_size(px(13.0))
                                                            .child("Wrapper has large shadow")
                                                    )
                                                    .into_any_element()
                                            })
                                        })
                                    })
                            )
                    )
            )
            // 6. Combined Styling
            .child(
                VStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("6. Combined Styling (Multiple Styled Trait Methods)")
                    )
                    .child(
                        VStack::new()
                            .gap(px(12.0))
                            .child(
                                Popover::new("combined-1")
                                    .p_8()  // <- Styled trait
                                    .rounded(px(999.0))  // <- Styled trait
                                    .bg(rgb(0x8b5cf6))  // <- Styled trait
                                    .shadow_lg()  // <- Styled trait
                                    .trigger(Button::new("combined-1-btn", "Purple Pill with Shadow").variant(ButtonVariant::Ghost))
                                    .content(|window, cx| {
                                        cx.new(|cx| {
                                            PopoverContent::new(window, cx, |_, _| {
                                                div()
                                                    .child(
                                                        div()
                                                            .text_size(px(14.0))
                                                            .font_weight(FontWeight::SEMIBOLD)
                                                            .child("Combined Styling 1")
                                                    )
                                                    .child(
                                                        div()
                                                            .text_size(px(13.0))
                                                            .child("Purple pill wrapper with large shadow")
                                                    )
                                                    .into_any_element()
                                            })
                                        })
                                    })
                            )
                            .child(
                                Popover::new("combined-2")
                                    .p(px(20.0))  // <- Styled trait
                                    .border_2()  // <- Styled trait
                                    .border_color(rgb(0x10b981))
                                    .rounded(px(12.0))  // <- Styled trait
                                    .trigger(Button::new("combined-2-btn", "Green Border with Padding").variant(ButtonVariant::Outline))
                                    .content(|window, cx| {
                                        cx.new(|cx| {
                                            PopoverContent::new(window, cx, |_, _| {
                                                div()
                                                    .child(
                                                        div()
                                                            .text_size(px(14.0))
                                                            .font_weight(FontWeight::SEMIBOLD)
                                                            .child("Combined Styling 2")
                                                    )
                                                    .child(
                                                        div()
                                                            .text_size(px(13.0))
                                                            .child("Large padding with green border")
                                                    )
                                                    .into_any_element()
                                            })
                                        })
                                    })
                            )
                            .child(
                                Popover::new("combined-3")
                                    .px(px(40.0))  // <- Styled trait
                                    .py(px(16.0))  // <- Styled trait
                                    .bg(rgb(0xf59e0b))  // <- Styled trait
                                    .rounded(px(8.0))  // <- Styled trait
                                    .shadow_md()  // <- Styled trait
                                    .trigger(Button::new("combined-3-btn", "Ultra Custom Wrapper").variant(ButtonVariant::Ghost))
                                    .content(|window, cx| {
                                        cx.new(|cx| {
                                            PopoverContent::new(window, cx, |_, _| {
                                                div()
                                                    .child(
                                                        div()
                                                            .text_size(px(14.0))
                                                            .font_weight(FontWeight::SEMIBOLD)
                                                            .child("Combined Styling 3")
                                                    )
                                                    .child(
                                                        div()
                                                            .text_size(px(13.0))
                                                            .child("Custom padding, orange background, and shadow")
                                                    )
                                                    .into_any_element()
                                            })
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
                            .child("All customizations above use the Styled trait for full GPUI styling control on the popover trigger wrapper!")
                    )
                    .child(
                        div()
                            .mt(px(8.0))
                            .text_size(px(12.0))
                            .text_color(theme.tokens.accent_foreground)
                            .child("Methods used: .p_4(), .p_8(), .p(), .px(), .py(), .bg(), .border_2(), .rounded(), .shadow_sm/md/lg()")
                    )
            )
                )
            )
    }
}
