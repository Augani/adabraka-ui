//! # Complex Layout Demo
//!
//! Demonstrates building complex desktop application layouts using the layout
//! components from adabraka-ui. Shows how to combine ScrollContainer, Panel,
//! Container, VStack, HStack, and other layout primitives to create
//! sophisticated UIs with minimal boilerplate.
//!
//! This example showcases:
//! - Auto-scrolling lists with unique IDs
//! - Nested layouts (sidebar + main content)
//! - Responsive containers with max-width
//! - Card-style panels
//! - Programmatic scroll control

use gpui::*;
use adabraka_ui::layout::*;

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.activate(true);
        cx.open_window(WindowOptions::default(), |_, cx| {
            cx.new(|_| ComplexLayoutDemo::new())
        })
        .unwrap();
    });
}

// ============================================================================
// Demo Application
// ============================================================================

struct ComplexLayoutDemo {
    scroll_handle: ScrollHandle,
    messages: Vec<Message>,
    sidebar_items: Vec<SidebarItem>,
}

#[derive(Clone)]
struct Message {
    id: usize,
    author: SharedString,
    content: SharedString,
    timestamp: SharedString,
}

#[derive(Clone)]
struct SidebarItem {
    icon: SharedString,
    label: SharedString,
    count: Option<usize>,
}

impl ComplexLayoutDemo {
    fn new() -> Self {
        Self {
            scroll_handle: ScrollHandle::new(),
            messages: Self::sample_messages(),
            sidebar_items: Self::sample_sidebar_items(),
        }
    }

    fn sample_messages() -> Vec<Message> {
        (0..50)
            .map(|i| Message {
                id: i,
                author: format!("User {}", i % 5).into(),
                content: format!("This is message number {}. Lorem ipsum dolor sit amet, consectetur adipiscing elit.", i).into(),
                timestamp: format!("{}:{}0", 10 + (i / 60), i % 60).into(),
            })
            .collect()
    }

    fn sample_sidebar_items() -> Vec<SidebarItem> {
        vec![
            SidebarItem {
                icon: "📥".into(),
                label: "Inbox".into(),
                count: Some(42),
            },
            SidebarItem {
                icon: "📤".into(),
                label: "Sent".into(),
                count: None,
            },
            SidebarItem {
                icon: "⭐".into(),
                label: "Favorites".into(),
                count: Some(7),
            },
            SidebarItem {
                icon: "🗑️".into(),
                label: "Trash".into(),
                count: Some(3),
            },
        ]
    }

    fn render_sidebar(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        Panel::new()
            .border()
            .w(px(240.0))
            .h_full()
            .bg(rgb(0xf5f5f5))
            .child(
                VStack::new()
                    .fill()
                    .child(
                        // Sidebar header
                        Panel::new()
                            .section()
                            .child(
                                div()
                                    .text_lg()
                                    .font_weight(FontWeight::BOLD)
                                    .child("Navigation")
                            )
                    )
                    .child(
                        // Sidebar items (scrollable) - auto-sizes to fill remaining space!
                        ScrollList::new()
                            .spacing(4.0)
                            .px(px(12.0))
                            .children(
                                self.sidebar_items.iter().map(|item| {
                                    self.render_sidebar_item(item)
                                })
                            )
                    )
            )
    }

    fn render_sidebar_item(&self, item: &SidebarItem) -> impl IntoElement {
        let base = div()
            .flex()
            .items_center()
            .justify_between()
            .px(px(12.0))
            .py(px(8.0))
            .rounded(px(6.0))
            .bg(rgb(0xffffff))
            .hover(|style| style.bg(rgb(0xe0e0e0)))
            .cursor_pointer()
            .child(
                HStack::new()
                    .spacing(8.0)
                    .items_center()
                    .child(div().text_lg().child(item.icon.clone()))
                    .child(div().child(item.label.clone()))
            );
        
        if let Some(count) = item.count {
            base.child(
                div()
                    .px(px(8.0))
                    .py(px(2.0))
                    .rounded(px(12.0))
                    .bg(rgb(0x3b82f6))
                    .text_color(rgb(0xffffff))
                    .text_sm()
                    .child(format!("{}", count))
            )
        } else {
            base
        }
    }

    fn render_toolbar(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        Panel::new()
            .section()
            .bg(rgb(0xffffff))
            .child(
                HStack::new()
                    .fill_width()
                    .items_center()
                    .space_between()
                    .child(
                        div()
                            .text_xl()
                            .font_weight(FontWeight::BOLD)
                            .child("Messages")
                    )
                    .child(
                        HStack::new()
                            .spacing(8.0)
                            .child(self.render_button("Compose", rgb(0x3b82f6).into()))
                            .child(self.render_button("Refresh", rgb(0x6b7280).into()))
                    )
            )
    }

    fn render_button(&self, label: &str, bg_color: Hsla) -> impl IntoElement {
        let label = label.to_string();
        div()
            .px(px(16.0))
            .py(px(8.0))
            .rounded(px(6.0))
            .bg(bg_color)
            .text_color(rgb(0xffffff))
            .hover(|style| style.opacity(0.8))
            .cursor_pointer()
            .child(label)
    }

    fn render_message_list(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        // No manual sizing needed! ScrollList auto-fills parent
        ScrollList::new()
            .spacing(8.0)
            .track_scroll(&self.scroll_handle)
            .id("message-list")
            .px(px(16.0))
            .py(px(16.0))
            .children(
                self.messages.iter().map(|msg| {
                    self.render_message_card(msg)
                })
            )
    }

    fn render_message_card(&self, message: &Message) -> impl IntoElement {
        Panel::new()
            .card()
            .bg(rgb(0xffffff))
            .hover(|style| style.bg(rgb(0xf9fafb)))
            .child(
                VStack::new()
                    .spacing(8.0)
                    .child(
                        // Message header
                        HStack::new()
                            .space_between()
                            .items_center()
                            .child(
                                div()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(0x111827))
                                    .child(message.author.clone())
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(0x6b7280))
                                    .child(message.timestamp.clone())
                            )
                    )
                    .child(
                        // Message content
                        div()
                            .text_color(rgb(0x374151))
                            .child(message.content.clone())
                    )
            )
    }

    fn render_status_bar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .h(px(32.0))
            .px(px(16.0))
            .border_t_1()
            .border_color(rgb(0xe5e7eb))
            .bg(rgb(0xf9fafb))
            .flex()
            .items_center()
            .justify_between()
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(0x6b7280))
                    .child(format!("{} messages", self.messages.len()))
            )
            .child(
                HStack::new()
                    .spacing(12.0)
                    .child(
                        div()
                            .id("scroll-to-top-btn")
                            .text_sm()
                            .text_color(rgb(0x3b82f6))
                            .cursor_pointer()
                            .child("Scroll to Top")
                            .on_click(cx.listener(|this, _, _, _| {
                                this.scroll_handle.set_offset(point(px(0.0), px(0.0)));
                            }))
                    )
                    .child(
                        div()
                            .id("scroll-to-bottom-btn")
                            .text_sm()
                            .text_color(rgb(0x3b82f6))
                            .cursor_pointer()
                            .child("Scroll to Bottom")
                            .on_click(cx.listener(|this, _, _, _| {
                                this.scroll_handle.scroll_to_bottom();
                            }))
                    )
            )
    }
}

impl Render for ComplexLayoutDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(rgb(0xfafafa))
            .child(
                // Main app layout: Sidebar + Content
                HStack::new()
                    .fill()
                    .child(self.render_sidebar(cx))
                    .child(
                        // Main content area
                        VStack::new()
                            .fill()
                            .child(self.render_toolbar(cx))
                            .child(
                                // Message list area (scrollable)
                                // ScrollList auto-fills remaining space in VStack!
                                self.render_message_list(cx)
                            )
                            .child(self.render_status_bar(cx))
                    )
            )
    }
}

