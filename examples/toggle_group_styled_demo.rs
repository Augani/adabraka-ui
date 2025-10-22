use adabraka_ui::{
    prelude::*,
    components::{
        scrollable::scrollable_vertical,
        toggle_group::{ToggleGroup, ToggleGroupItem},
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
                        title: Some("ToggleGroup Styled Trait Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(900.0), px(800.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|_| ToggleGroupStyledDemo::new()),
            )
            .unwrap();
        });
}

struct ToggleGroupStyledDemo {
    selected_view: SharedString,
    selected_format: SharedString,
    selected_multiple: Vec<SharedString>,
}

impl ToggleGroupStyledDemo {
    fn new() -> Self {
        Self {
            selected_view: "list".into(),
            selected_format: "left".into(),
            selected_multiple: vec!["bold".into()],
        }
    }
}

impl Render for ToggleGroupStyledDemo {
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
                            .text_size(px(24.0))
                            .font_weight(FontWeight::BOLD)
                            .child("ToggleGroup Styled Trait Customization Demo")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("Demonstrating full GPUI styling control via Styled trait")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.accent)
                            .child(format!("Current view: {} | Format: {} | Multiple: {:?}",
                                self.selected_view, self.selected_format, self.selected_multiple))
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
                        VStack::new()
                            .gap(px(12.0))
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Default padding:")
                            )
                            .child(
                                ToggleGroup::new()
                                    .item(ToggleGroupItem::new("list", "List"))
                                    .item(ToggleGroupItem::new("grid", "Grid"))
                                    .item(ToggleGroupItem::new("table", "Table"))
                                    .value(self.selected_view.clone())
                                    .on_change(cx.listener(|view, value: &SharedString, _, cx| {
                                        view.selected_view = value.clone();
                                        cx.notify();
                                    }))
                            )
                            .child(
                                div()
                                    .mt(px(12.0))
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Custom p_4():")
                            )
                            .child(
                                ToggleGroup::new()
                                    .item(ToggleGroupItem::new("list", "List"))
                                    .item(ToggleGroupItem::new("grid", "Grid"))
                                    .item(ToggleGroupItem::new("table", "Table"))
                                    .value(self.selected_view.clone())
                                    .p_4()  // ← Styled trait method
                                    .on_change(cx.listener(|view, value: &SharedString, _, cx| {
                                        view.selected_view = value.clone();
                                        cx.notify();
                                    }))
                            )
                            .child(
                                div()
                                    .mt(px(12.0))
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Custom p_8():")
                            )
                            .child(
                                ToggleGroup::new()
                                    .item(ToggleGroupItem::new("list", "List"))
                                    .item(ToggleGroupItem::new("grid", "Grid"))
                                    .item(ToggleGroupItem::new("table", "Table"))
                                    .value(self.selected_view.clone())
                                    .p_8()  // ← Styled trait method
                                    .on_change(cx.listener(|view, value: &SharedString, _, cx| {
                                        view.selected_view = value.clone();
                                        cx.notify();
                                    }))
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
                        VStack::new()
                            .gap(px(12.0))
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Blue background:")
                            )
                            .child(
                                ToggleGroup::new()
                                    .item(ToggleGroupItem::new("left", "Left"))
                                    .item(ToggleGroupItem::new("center", "Center"))
                                    .item(ToggleGroupItem::new("right", "Right"))
                                    .value(self.selected_format.clone())
                                    .bg(hsla(217.0/360.0, 0.91, 0.60, 0.2))  // ← Styled trait (blue)
                                    .on_change(cx.listener(|view, value: &SharedString, _, cx| {
                                        view.selected_format = value.clone();
                                        cx.notify();
                                    }))
                            )
                            .child(
                                div()
                                    .mt(px(12.0))
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Purple background:")
                            )
                            .child(
                                ToggleGroup::new()
                                    .item(ToggleGroupItem::new("left", "Left"))
                                    .item(ToggleGroupItem::new("center", "Center"))
                                    .item(ToggleGroupItem::new("right", "Right"))
                                    .value(self.selected_format.clone())
                                    .bg(hsla(258.0/360.0, 0.90, 0.66, 0.2))  // ← Styled trait (purple)
                                    .on_change(cx.listener(|view, value: &SharedString, _, cx| {
                                        view.selected_format = value.clone();
                                        cx.notify();
                                    }))
                            )
                            .child(
                                div()
                                    .mt(px(12.0))
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Green background:")
                            )
                            .child(
                                ToggleGroup::new()
                                    .item(ToggleGroupItem::new("left", "Left"))
                                    .item(ToggleGroupItem::new("center", "Center"))
                                    .item(ToggleGroupItem::new("right", "Right"))
                                    .value(self.selected_format.clone())
                                    .bg(hsla(160.0/360.0, 0.84, 0.39, 0.2))  // ← Styled trait (green)
                                    .on_change(cx.listener(|view, value: &SharedString, _, cx| {
                                        view.selected_format = value.clone();
                                        cx.notify();
                                    }))
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
                        VStack::new()
                            .gap(px(12.0))
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Blue border 2px:")
                            )
                            .child(
                                ToggleGroup::new()
                                    .item(ToggleGroupItem::new("left", "Left"))
                                    .item(ToggleGroupItem::new("center", "Center"))
                                    .item(ToggleGroupItem::new("right", "Right"))
                                    .value(self.selected_format.clone())
                                    .border_2()  // ← Styled trait
                                    .border_color(rgb(0x3b82f6))
                                    .on_change(cx.listener(|view, value: &SharedString, _, cx| {
                                        view.selected_format = value.clone();
                                        cx.notify();
                                    }))
                            )
                            .child(
                                div()
                                    .mt(px(12.0))
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Red border 2px:")
                            )
                            .child(
                                ToggleGroup::new()
                                    .item(ToggleGroupItem::new("left", "Left"))
                                    .item(ToggleGroupItem::new("center", "Center"))
                                    .item(ToggleGroupItem::new("right", "Right"))
                                    .value(self.selected_format.clone())
                                    .border_2()  // ← Styled trait
                                    .border_color(rgb(0xef4444))
                                    .on_change(cx.listener(|view, value: &SharedString, _, cx| {
                                        view.selected_format = value.clone();
                                        cx.notify();
                                    }))
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
                        VStack::new()
                            .gap(px(12.0))
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("No radius:")
                            )
                            .child(
                                ToggleGroup::new()
                                    .item(ToggleGroupItem::new("left", "Left"))
                                    .item(ToggleGroupItem::new("center", "Center"))
                                    .item(ToggleGroupItem::new("right", "Right"))
                                    .value(self.selected_format.clone())
                                    .rounded(px(0.0))  // ← Styled trait
                                    .on_change(cx.listener(|view, value: &SharedString, _, cx| {
                                        view.selected_format = value.clone();
                                        cx.notify();
                                    }))
                            )
                            .child(
                                div()
                                    .mt(px(12.0))
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Large radius:")
                            )
                            .child(
                                ToggleGroup::new()
                                    .item(ToggleGroupItem::new("left", "Left"))
                                    .item(ToggleGroupItem::new("center", "Center"))
                                    .item(ToggleGroupItem::new("right", "Right"))
                                    .value(self.selected_format.clone())
                                    .rounded(px(16.0))  // ← Styled trait
                                    .on_change(cx.listener(|view, value: &SharedString, _, cx| {
                                        view.selected_format = value.clone();
                                        cx.notify();
                                    }))
                            )
                            .child(
                                div()
                                    .mt(px(12.0))
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Pill shape:")
                            )
                            .child(
                                ToggleGroup::new()
                                    .item(ToggleGroupItem::new("left", "Left"))
                                    .item(ToggleGroupItem::new("center", "Center"))
                                    .item(ToggleGroupItem::new("right", "Right"))
                                    .value(self.selected_format.clone())
                                    .rounded(px(999.0))  // ← Styled trait
                                    .on_change(cx.listener(|view, value: &SharedString, _, cx| {
                                        view.selected_format = value.clone();
                                        cx.notify();
                                    }))
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
                        VStack::new()
                            .gap(px(12.0))
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Full width:")
                            )
                            .child(
                                ToggleGroup::new()
                                    .item(ToggleGroupItem::new("left", "Left"))
                                    .item(ToggleGroupItem::new("center", "Center"))
                                    .item(ToggleGroupItem::new("right", "Right"))
                                    .value(self.selected_format.clone())
                                    .w_full()  // ← Styled trait
                                    .on_change(cx.listener(|view, value: &SharedString, _, cx| {
                                        view.selected_format = value.clone();
                                        cx.notify();
                                    }))
                            )
                            .child(
                                div()
                                    .mt(px(12.0))
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Custom width (400px):")
                            )
                            .child(
                                ToggleGroup::new()
                                    .item(ToggleGroupItem::new("left", "Left"))
                                    .item(ToggleGroupItem::new("center", "Center"))
                                    .item(ToggleGroupItem::new("right", "Right"))
                                    .value(self.selected_format.clone())
                                    .w(px(400.0))  // ← Styled trait
                                    .on_change(cx.listener(|view, value: &SharedString, _, cx| {
                                        view.selected_format = value.clone();
                                        cx.notify();
                                    }))
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
                        VStack::new()
                            .gap(px(12.0))
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Shadow small:")
                            )
                            .child(
                                ToggleGroup::new()
                                    .item(ToggleGroupItem::new("left", "Left"))
                                    .item(ToggleGroupItem::new("center", "Center"))
                                    .item(ToggleGroupItem::new("right", "Right"))
                                    .value(self.selected_format.clone())
                                    .shadow_sm()  // ← Styled trait
                                    .on_change(cx.listener(|view, value: &SharedString, _, cx| {
                                        view.selected_format = value.clone();
                                        cx.notify();
                                    }))
                            )
                            .child(
                                div()
                                    .mt(px(12.0))
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Shadow medium:")
                            )
                            .child(
                                ToggleGroup::new()
                                    .item(ToggleGroupItem::new("left", "Left"))
                                    .item(ToggleGroupItem::new("center", "Center"))
                                    .item(ToggleGroupItem::new("right", "Right"))
                                    .value(self.selected_format.clone())
                                    .shadow_md()  // ← Styled trait
                                    .on_change(cx.listener(|view, value: &SharedString, _, cx| {
                                        view.selected_format = value.clone();
                                        cx.notify();
                                    }))
                            )
                            .child(
                                div()
                                    .mt(px(12.0))
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Shadow large:")
                            )
                            .child(
                                ToggleGroup::new()
                                    .item(ToggleGroupItem::new("left", "Left"))
                                    .item(ToggleGroupItem::new("center", "Center"))
                                    .item(ToggleGroupItem::new("right", "Right"))
                                    .value(self.selected_format.clone())
                                    .shadow_lg()  // ← Styled trait
                                    .on_change(cx.listener(|view, value: &SharedString, _, cx| {
                                        view.selected_format = value.clone();
                                        cx.notify();
                                    }))
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
                        VStack::new()
                            .gap(px(12.0))
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Purple background with shadow and custom padding:")
                            )
                            .child(
                                ToggleGroup::new()
                                    .item(ToggleGroupItem::new("left", "Left"))
                                    .item(ToggleGroupItem::new("center", "Center"))
                                    .item(ToggleGroupItem::new("right", "Right"))
                                    .value(self.selected_format.clone())
                                    .p_8()  // ← Styled trait
                                    .rounded(px(999.0))  // ← Styled trait
                                    .bg(hsla(258.0/360.0, 0.90, 0.66, 0.2))  // ← Styled trait (purple)
                                    .shadow_lg()  // ← Styled trait
                                    .on_change(cx.listener(|view, value: &SharedString, _, cx| {
                                        view.selected_format = value.clone();
                                        cx.notify();
                                    }))
                            )
                            .child(
                                div()
                                    .mt(px(12.0))
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Full width with border and custom radius:")
                            )
                            .child(
                                ToggleGroup::new()
                                    .item(ToggleGroupItem::new("left", "Left"))
                                    .item(ToggleGroupItem::new("center", "Center"))
                                    .item(ToggleGroupItem::new("right", "Right"))
                                    .value(self.selected_format.clone())
                                    .w_full()  // ← Styled trait
                                    .p(px(6.0))  // ← Styled trait
                                    .border_2()  // ← Styled trait
                                    .border_color(rgb(0x10b981))
                                    .rounded(px(12.0))  // ← Styled trait
                                    .on_change(cx.listener(|view, value: &SharedString, _, cx| {
                                        view.selected_format = value.clone();
                                        cx.notify();
                                    }))
                            )
                            .child(
                                div()
                                    .mt(px(12.0))
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Ultra custom: orange background, shadow, custom padding:")
                            )
                            .child(
                                ToggleGroup::new()
                                    .item(ToggleGroupItem::new("left", "Left"))
                                    .item(ToggleGroupItem::new("center", "Center"))
                                    .item(ToggleGroupItem::new("right", "Right"))
                                    .value(self.selected_format.clone())
                                    .px(px(24.0))  // ← Styled trait
                                    .py(px(12.0))  // ← Styled trait
                                    .bg(hsla(38.0/360.0, 0.92, 0.50, 0.2))  // ← Styled trait (orange)
                                    .rounded(px(8.0))  // ← Styled trait
                                    .shadow_md()  // ← Styled trait
                                    .w(px(500.0))  // ← Styled trait
                                    .on_change(cx.listener(|view, value: &SharedString, _, cx| {
                                        view.selected_format = value.clone();
                                        cx.notify();
                                    }))
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
                            .child("Methods used: .p_4(), .p_8(), .px(), .py(), .bg(), .border_2(), .rounded(), .w_full(), .w(), .shadow_sm/md/lg()")
                    )
            )
                )
            )
    }
}
