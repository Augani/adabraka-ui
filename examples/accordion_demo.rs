use gpui::*;
use adabraka_ui::display::accordion::Accordion;
use std::collections::HashSet;

fn main() {
    Application::new().run(|cx| {
        // Initialize adabraka-ui
        adabraka_ui::init(cx);

        cx.open_window(
            WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some("Accordion Demo".into()),
                    ..Default::default()
                }),
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(800.0), px(900.0)),
                    cx,
                ))),
                ..Default::default()
            },
            |window, cx| cx.new(|cx| AccordionDemo::new(window, cx)),
        )
        .unwrap();
    });
}

struct AccordionDemo {
    single_open: HashSet<usize>,
    multiple_open: HashSet<usize>,
    borderless_open: HashSet<usize>,
}

impl AccordionDemo {
    fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        let mut single_open = HashSet::new();
        single_open.insert(1); // Second item starts open

        let mut multiple_open = HashSet::new();
        multiple_open.insert(1);
        multiple_open.insert(2);

        Self {
            single_open,
            multiple_open,
            borderless_open: HashSet::new(),
        }
    }
}

impl Render for AccordionDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .p(px(32.0))
            .gap(px(32.0))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .child(
                        div()
                            .text_size(px(24.0))
                            .font_weight(FontWeight::BOLD)
                            .child("Accordion Component")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .child("Vertically stacked set of interactive headings that reveal content")
                    )
            )
            // Single mode accordion
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(12.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("Single Mode (Default)")
                    )
                    .child(
                        Accordion::new("single-accordion")
                            .item(|item| {
                                item.title("What is GPUI?")
                                    .icon("info")
                                    .content(
                                        div()
                                            .child("GPUI is a GPU-accelerated UI framework built by Zed Industries. ")
                                            .child("It provides high-performance rendering and is used in the Zed editor.")
                                    )
                                    .open(self.single_open.contains(&0))
                            })
                            .item(|item| {
                                item.title("How does it work?")
                                    .icon("code")
                                    .content(
                                        div()
                                            .child("GPUI uses Metal (on macOS), DirectX (on Windows), and Vulkan (on Linux) ")
                                            .child("to render UI elements directly on the GPU for optimal performance.")
                                    )
                                    .open(self.single_open.contains(&1))
                            })
                            .item(|item| {
                                item.title("Why use it?")
                                    .icon("zap")
                                    .content(
                                        div()
                                            .child("GPUI offers exceptional performance, ")
                                            .child("a declarative API similar to SwiftUI/React, ")
                                            .child("and native OS integration.")
                                    )
                                    .open(self.single_open.contains(&2))
                            })
                            .on_change(_cx.listener(|this, indices: &[usize], _window, _cx| {
                                println!("[Single Accordion] Open items: {:?}", indices);
                                this.single_open = indices.iter().copied().collect();
                                _cx.notify();
                            }))
                    )
            )
            // Multiple mode accordion
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(12.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("Multiple Mode")
                    )
                    .child(
                        Accordion::new("multiple-accordion")
                            .multiple(true)
                            .item(|item| {
                                item.title("TypeScript Support")
                                    .content(
                                        div()
                                            .child("GPUI has Rust bindings and can be used from Rust code.")
                                    )
                                    .open(self.multiple_open.contains(&0))
                            })
                            .item(|item| {
                                item.title("Component Library")
                                    .content(
                                        div()
                                            .child("This is adabraka-ui, a comprehensive component library for GPUI ")
                                            .child("with components like buttons, inputs, tables, and more.")
                                    )
                                    .open(self.multiple_open.contains(&1))
                            })
                            .item(|item| {
                                item.title("Theme System")
                                    .content(
                                        div()
                                            .child("Built-in theme system with light and dark modes, ")
                                            .child("design tokens, and shadcn-inspired color palettes.")
                                    )
                                    .open(self.multiple_open.contains(&2))
                            })
                            .on_change(_cx.listener(|this, indices: &[usize], _window, _cx| {
                                println!("[Multiple Accordion] Open items: {:?}", indices);
                                this.multiple_open = indices.iter().copied().collect();
                                _cx.notify();
                            }))
                    )
            )
            // Borderless mode
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(12.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("Borderless Mode")
                    )
                    .child(
                        Accordion::new("borderless-accordion")
                            .bordered(false)
                            .item(|item| {
                                item.title("Getting Started")
                                    .content(
                                        div()
                                            .child("Install adabraka-ui and add it to your dependencies.")
                                    )
                                    .open(self.borderless_open.contains(&0))
                            })
                            .item(|item| {
                                item.title("Documentation")
                                    .content(
                                        div()
                                            .child("Check out the examples directory for usage examples.")
                                    )
                                    .open(self.borderless_open.contains(&1))
                            })
                            .on_change(_cx.listener(|this, indices: &[usize], _window, _cx| {
                                println!("[Borderless Accordion] Open items: {:?}", indices);
                                this.borderless_open = indices.iter().copied().collect();
                                _cx.notify();
                            }))
                    )
            )
    }
}
