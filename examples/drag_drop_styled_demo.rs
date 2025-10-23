use adabraka_ui::{
    prelude::*,
    components::{
        drag_drop::{DragData, Draggable, DropZone, DropZoneStyle},
        scrollable::scrollable_vertical,
    },
};
use gpui::{prelude::FluentBuilder as _, *};
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
                        title: Some("Drag & Drop Styled Trait Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(1100.0), px(900.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|_| DragDropStyledDemo::new()),
            )
            .unwrap();
        });
}

#[derive(Clone, Debug)]
struct Task {
    id: usize,
    title: String,
    priority: Priority,
}

#[derive(Clone, Debug, PartialEq)]
enum Priority {
    Low,
    Medium,
    High,
    Urgent,
}

impl Priority {
    fn color(&self) -> Hsla {
        match self {
            Priority::Low => rgb(0x10b981).into(),      // green
            Priority::Medium => rgb(0x3b82f6).into(),   // blue
            Priority::High => rgb(0xf59e0b).into(),     // amber
            Priority::Urgent => rgb(0xef4444).into(),   // red
        }
    }

    fn label(&self) -> &'static str {
        match self {
            Priority::Low => "Low",
            Priority::Medium => "Medium",
            Priority::High => "High",
            Priority::Urgent => "Urgent",
        }
    }
}

struct DragDropStyledDemo {
    todos: Vec<Task>,
    in_progress: Vec<Task>,
    completed: Vec<Task>,
}

impl DragDropStyledDemo {
    fn new() -> Self {
        Self {
            todos: vec![
                Task {
                    id: 1,
                    title: "Design new landing page".to_string(),
                    priority: Priority::High,
                },
                Task {
                    id: 2,
                    title: "Fix responsive layout".to_string(),
                    priority: Priority::Urgent,
                },
                Task {
                    id: 3,
                    title: "Update documentation".to_string(),
                    priority: Priority::Low,
                },
            ],
            in_progress: vec![
                Task {
                    id: 4,
                    title: "Implement dark mode".to_string(),
                    priority: Priority::Medium,
                },
            ],
            completed: vec![
                Task {
                    id: 5,
                    title: "Set up CI/CD pipeline".to_string(),
                    priority: Priority::High,
                },
            ],
        }
    }
}

impl Render for DragDropStyledDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
                                        .text_size(px(28.0))
                                        .font_weight(FontWeight::BOLD)
                                        .child("Drag & Drop Styled Trait Demo")
                                )
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child("Customize draggable items and drop zones with styling variations")
                                )
                        )
                        .child(
                            div()
                                .flex()
                                .gap(px(24.0))
                                .w_full()
                                .child(
                                    // Todo column
                                    self.render_column(
                                        "To Do",
                                        &self.todos,
                                        "todo",
                                        theme.tokens.muted,
                                        DropZoneStyle::Dashed,
                                        cx
                                    )
                                )
                                .child(
                                    // In Progress column
                                    self.render_column(
                                        "In Progress",
                                        &self.in_progress,
                                        "progress",
                                        rgb(0x3b82f6).into(),
                                        DropZoneStyle::Solid,
                                        cx
                                    )
                                )
                                .child(
                                    // Completed column
                                    self.render_column(
                                        "Completed",
                                        &self.completed,
                                        "completed",
                                        rgb(0x10b981).into(),
                                        DropZoneStyle::Filled,
                                        cx
                                    )
                                )
                        )
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(20.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("Styling Variations Showcase")
                                )
                                .child(self.render_styling_variations())
                        )
                )
            )
    }
}

impl DragDropStyledDemo {
    fn render_column(
        &self,
        title: &'static str,
        tasks: &[Task],
        zone_id: &'static str,
        accent_color: Hsla,
        zone_style: DropZoneStyle,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme = use_theme();

        div()
            .flex()
            .flex_col()
            .flex_1()
            .min_w(px(280.0))
            .child(
                // Column header
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .mb(px(12.0))
                    .px(px(8.0))
                    .child(
                        div()
                            .text_size(px(16.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child(title)
                    )
                    .child(
                        div()
                            .px(px(8.0))
                            .py(px(2.0))
                            .rounded(px(12.0))
                            .bg(accent_color.opacity(0.2))
                            .text_color(accent_color)
                            .text_size(px(12.0))
                            .font_weight(FontWeight::MEDIUM)
                            .child(format!("{}", tasks.len()))
                    )
            )
            .child(
                // Drop zone with custom styling
                DropZone::<Task>::new(zone_id)
                    .drop_zone_style(zone_style)
                    .min_h(px(400.0))
                    // Custom border and background using Styled trait
                    .border_2()
                    .border_color(accent_color.opacity(0.3))
                    .bg(accent_color.opacity(0.05))
                    .shadow(vec![BoxShadow {
                        color: accent_color.opacity(0.1),
                        offset: point(px(0.0), px(2.0)),
                        blur_radius: px(8.0),
                        spread_radius: px(0.0),
                    }])
                    .on_drop(cx.listener(move |this, data: &DragData<Task>, _, cx| {
                        match zone_id {
                            "todo" => this.todos.push(data.data.clone()),
                            "progress" => this.in_progress.push(data.data.clone()),
                            "completed" => this.completed.push(data.data.clone()),
                            _ => {}
                        }
                        cx.notify();
                    }))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(8.0))
                            .w_full()
                            .when(tasks.is_empty(), |this| {
                                this.child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .items_center()
                                        .justify_center()
                                        .gap(px(8.0))
                                        .py(px(32.0))
                                        .child(
                                            div()
                                                .text_size(px(14.0))
                                                .text_color(theme.tokens.muted_foreground)
                                                .child("Drop tasks here")
                                        )
                                )
                            })
                            .children(tasks.iter().enumerate().map(|(ix, task)| {
                                let task_clone = task.clone();
                                let drag_data = DragData::new(task_clone.clone())
                                    .with_label(task.title.clone());

                                Draggable::new((zone_id, ix), drag_data)
                                    // Custom styling for draggable items
                                    .rounded(px(8.0))
                                    .shadow(vec![BoxShadow {
                                        color: hsla(0.0, 0.0, 0.0, 0.05),
                                        offset: point(px(0.0), px(1.0)),
                                        blur_radius: px(3.0),
                                        spread_radius: px(0.0),
                                    }])
                                    .hover_bg(theme.tokens.muted.opacity(0.2))
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(6.0))
                                            .p(px(12.0))
                                            .rounded(px(8.0))
                                            .bg(theme.tokens.card)
                                            .border_1()
                                            .border_color(theme.tokens.border)
                                            .child(
                                                div()
                                                    .text_size(px(14.0))
                                                    .font_weight(FontWeight::MEDIUM)
                                                    .child(task.title.clone())
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .items_center()
                                                    .gap(px(6.0))
                                                    .child(
                                                        div()
                                                            .w(px(8.0))
                                                            .h(px(8.0))
                                                            .rounded(px(4.0))
                                                            .bg(task.priority.color())
                                                    )
                                                    .child(
                                                        div()
                                                            .text_size(px(12.0))
                                                            .text_color(theme.tokens.muted_foreground)
                                                            .child(task.priority.label())
                                                    )
                                            )
                                    )
                            }))
                    )
            )
    }

    fn render_styling_variations(&self) -> impl IntoElement {
        let theme = use_theme();

        VStack::new()
            .gap(px(24.0))
            .w_full()
            // Variation 1: Gradient background draggables
            .child(
                VStack::new()
                    .gap(px(8.0))
                    .child(
                        div()
                            .text_size(px(16.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("1. Gradient Background Draggables")
                    )
                    .child(
                        HStack::new()
                            .gap(px(12.0))
                            .child(
                                Draggable::new("gradient-1", DragData::new("Item 1".to_string()))
                                    .bg(Hsla::from(rgb(0x8b5cf6)))
                                    .rounded(px(12.0))
                                    .shadow(vec![BoxShadow {
                                        color: Hsla::from(rgb(0x8b5cf6)).opacity(0.4),
                                        offset: point(px(0.0), px(4.0)),
                                        blur_radius: px(12.0),
                                        spread_radius: px(0.0),
                                    }])
                                    .child(
                                        div()
                                            .px(px(20.0))
                                            .py(px(12.0))
                                            .text_color(gpui::white())
                                            .font_weight(FontWeight::MEDIUM)
                                            .child("Purple Gradient")
                                    )
                            )
                            .child(
                                Draggable::new("gradient-2", DragData::new("Item 2".to_string()))
                                    .bg(Hsla::from(rgb(0xec4899)))
                                    .rounded(px(12.0))
                                    .shadow(vec![BoxShadow {
                                        color: Hsla::from(rgb(0xec4899)).opacity(0.4),
                                        offset: point(px(0.0), px(4.0)),
                                        blur_radius: px(12.0),
                                        spread_radius: px(0.0),
                                    }])
                                    .child(
                                        div()
                                            .px(px(20.0))
                                            .py(px(12.0))
                                            .text_color(gpui::white())
                                            .font_weight(FontWeight::MEDIUM)
                                            .child("Pink Gradient")
                                    )
                            )
                            .child(
                                Draggable::new("gradient-3", DragData::new("Item 3".to_string()))
                                    .bg(Hsla::from(rgb(0x06b6d4)))
                                    .rounded(px(12.0))
                                    .shadow(vec![BoxShadow {
                                        color: Hsla::from(rgb(0x06b6d4)).opacity(0.4),
                                        offset: point(px(0.0), px(4.0)),
                                        blur_radius: px(12.0),
                                        spread_radius: px(0.0),
                                    }])
                                    .child(
                                        div()
                                            .px(px(20.0))
                                            .py(px(12.0))
                                            .text_color(gpui::white())
                                            .font_weight(FontWeight::MEDIUM)
                                            .child("Cyan Gradient")
                                    )
                            )
                    )
            )
            // Variation 2: Large shadow drop zones
            .child(
                VStack::new()
                    .gap(px(8.0))
                    .child(
                        div()
                            .text_size(px(16.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("2. Large Shadow Drop Zones")
                    )
                    .child(
                        HStack::new()
                            .gap(px(12.0))
                            .child(
                                DropZone::<String>::new("shadow-zone-1")
                                    .drop_zone_style(DropZoneStyle::Solid)
                                    .min_h(px(120.0))
                                    .w(px(200.0))
                                    .shadow(vec![BoxShadow {
                                        color: hsla(0.0, 0.0, 0.0, 0.15),
                                        offset: point(px(0.0), px(8.0)),
                                        blur_radius: px(24.0),
                                        spread_radius: px(0.0),
                                    }])
                                    .child(
                                        div()
                                            .text_size(px(13.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("Drop Zone 1")
                                    )
                            )
                            .child(
                                DropZone::<String>::new("shadow-zone-2")
                                    .drop_zone_style(DropZoneStyle::Filled)
                                    .min_h(px(120.0))
                                    .w(px(200.0))
                                    .shadow(vec![BoxShadow {
                                        color: hsla(0.0, 0.0, 0.0, 0.15),
                                        offset: point(px(0.0), px(8.0)),
                                        blur_radius: px(24.0),
                                        spread_radius: px(0.0),
                                    }])
                                    .child(
                                        div()
                                            .text_size(px(13.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("Drop Zone 2")
                                    )
                            )
                    )
            )
            // Variation 3: Rounded and bordered draggables
            .child(
                VStack::new()
                    .gap(px(8.0))
                    .child(
                        div()
                            .text_size(px(16.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("3. Rounded and Bordered Draggables")
                    )
                    .child(
                        HStack::new()
                            .gap(px(12.0))
                            .child(
                                Draggable::new("border-1", DragData::new("Tag".to_string()))
                                    .rounded(px(20.0))
                                    .border_2()
                                    .border_color(Hsla::from(rgb(0x10b981)))
                                    .bg(Hsla::from(rgb(0x10b981)).opacity(0.1))
                                    .child(
                                        div()
                                            .px(px(16.0))
                                            .py(px(8.0))
                                            .text_color(Hsla::from(rgb(0x10b981)))
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_size(px(13.0))
                                            .child("Success Tag")
                                    )
                            )
                            .child(
                                Draggable::new("border-2", DragData::new("Tag".to_string()))
                                    .rounded(px(20.0))
                                    .border_2()
                                    .border_color(Hsla::from(rgb(0xf59e0b)))
                                    .bg(Hsla::from(rgb(0xf59e0b)).opacity(0.1))
                                    .child(
                                        div()
                                            .px(px(16.0))
                                            .py(px(8.0))
                                            .text_color(Hsla::from(rgb(0xf59e0b)))
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_size(px(13.0))
                                            .child("Warning Tag")
                                    )
                            )
                            .child(
                                Draggable::new("border-3", DragData::new("Tag".to_string()))
                                    .rounded(px(20.0))
                                    .border_2()
                                    .border_color(Hsla::from(rgb(0xef4444)))
                                    .bg(Hsla::from(rgb(0xef4444)).opacity(0.1))
                                    .child(
                                        div()
                                            .px(px(16.0))
                                            .py(px(8.0))
                                            .text_color(Hsla::from(rgb(0xef4444)))
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_size(px(13.0))
                                            .child("Error Tag")
                                    )
                            )
                    )
            )
            // Variation 4: Custom padding and margins
            .child(
                VStack::new()
                    .gap(px(8.0))
                    .child(
                        div()
                            .text_size(px(16.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("4. Custom Padding & Margins")
                    )
                    .child(
                        HStack::new()
                            .gap(px(16.0))
                            .child(
                                Draggable::new("padding-1", DragData::new("Compact".to_string()))
                                    .p(px(4.0))
                                    .bg(theme.tokens.card)
                                    .rounded(px(6.0))
                                    .child(
                                        div()
                                            .px(px(12.0))
                                            .py(px(6.0))
                                            .bg(theme.tokens.primary)
                                            .text_color(theme.tokens.primary_foreground)
                                            .rounded(px(4.0))
                                            .text_size(px(13.0))
                                            .child("Compact")
                                    )
                            )
                            .child(
                                Draggable::new("padding-2", DragData::new("Regular".to_string()))
                                    .p(px(8.0))
                                    .bg(theme.tokens.card)
                                    .rounded(px(8.0))
                                    .child(
                                        div()
                                            .px(px(16.0))
                                            .py(px(10.0))
                                            .bg(theme.tokens.primary)
                                            .text_color(theme.tokens.primary_foreground)
                                            .rounded(px(6.0))
                                            .text_size(px(14.0))
                                            .child("Regular")
                                    )
                            )
                            .child(
                                Draggable::new("padding-3", DragData::new("Spacious".to_string()))
                                    .p(px(12.0))
                                    .bg(theme.tokens.card)
                                    .rounded(px(10.0))
                                    .child(
                                        div()
                                            .px(px(20.0))
                                            .py(px(14.0))
                                            .bg(theme.tokens.primary)
                                            .text_color(theme.tokens.primary_foreground)
                                            .rounded(px(8.0))
                                            .text_size(px(15.0))
                                            .child("Spacious")
                                    )
                            )
                    )
            )
            // Variation 5: Different opacity levels
            .child(
                VStack::new()
                    .gap(px(8.0))
                    .child(
                        div()
                            .text_size(px(16.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("5. Opacity Variations")
                    )
                    .child(
                        HStack::new()
                            .gap(px(12.0))
                            .child(
                                Draggable::new("opacity-1", DragData::new("100%".to_string()))
                                    .bg(theme.tokens.primary)
                                    .opacity(1.0)
                                    .rounded(px(8.0))
                                    .child(
                                        div()
                                            .px(px(20.0))
                                            .py(px(12.0))
                                            .text_color(theme.tokens.primary_foreground)
                                            .font_weight(FontWeight::MEDIUM)
                                            .child("100% Opacity")
                                    )
                            )
                            .child(
                                Draggable::new("opacity-2", DragData::new("70%".to_string()))
                                    .bg(theme.tokens.primary)
                                    .opacity(0.7)
                                    .rounded(px(8.0))
                                    .child(
                                        div()
                                            .px(px(20.0))
                                            .py(px(12.0))
                                            .text_color(theme.tokens.primary_foreground)
                                            .font_weight(FontWeight::MEDIUM)
                                            .child("70% Opacity")
                                    )
                            )
                            .child(
                                Draggable::new("opacity-3", DragData::new("40%".to_string()))
                                    .bg(theme.tokens.primary)
                                    .opacity(0.4)
                                    .rounded(px(8.0))
                                    .child(
                                        div()
                                            .px(px(20.0))
                                            .py(px(12.0))
                                            .text_color(theme.tokens.primary_foreground)
                                            .font_weight(FontWeight::MEDIUM)
                                            .child("40% Opacity")
                                    )
                            )
                    )
            )
            // Variation 6: Custom border styles for drop zones
            .child(
                VStack::new()
                    .gap(px(8.0))
                    .child(
                        div()
                            .text_size(px(16.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("6. Custom Border Styles for Drop Zones")
                    )
                    .child(
                        HStack::new()
                            .gap(px(12.0))
                            .child(
                                DropZone::<String>::new("border-zone-1")
                                    .drop_zone_style(DropZoneStyle::Dashed)
                                    .min_h(px(100.0))
                                    .w(px(180.0))
                                    .border_4()
                                    .border_color(Hsla::from(rgb(0x8b5cf6)))
                                    .rounded(px(12.0))
                                    .child(
                                        div()
                                            .text_size(px(13.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("Thick Border")
                                    )
                            )
                            .child(
                                DropZone::<String>::new("border-zone-2")
                                    .drop_zone_style(DropZoneStyle::Solid)
                                    .min_h(px(100.0))
                                    .w(px(180.0))
                                    .border_2()
                                    .border_color(Hsla::from(rgb(0x06b6d4)))
                                    .rounded(px(20.0))
                                    .bg(Hsla::from(rgb(0x06b6d4)).opacity(0.05))
                                    .child(
                                        div()
                                            .text_size(px(13.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("Rounded Border")
                                    )
                            )
                            .child(
                                DropZone::<String>::new("border-zone-3")
                                    .drop_zone_style(DropZoneStyle::Filled)
                                    .min_h(px(100.0))
                                    .w(px(180.0))
                                    .border_1()
                                    .border_color(Hsla::from(rgb(0xf59e0b)))
                                    .rounded(px(8.0))
                                    .bg(Hsla::from(rgb(0xf59e0b)).opacity(0.1))
                                    .child(
                                        div()
                                            .text_size(px(13.0))
                                            .text_color(theme.tokens.muted_foreground)
                                            .child("Filled + Border")
                                    )
                            )
                    )
            )
    }
}
