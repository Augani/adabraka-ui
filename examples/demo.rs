use adabraka_ui::prelude::*;
use gpui::*;

fn main() {
    Application::new().run(|cx| {
        cx.open_window(
            WindowOptions {
                titlebar: Some(gpui::TitlebarOptions {
                    title: Some("adabraka-ui Demo".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |window, cx| {
                cx.new(|cx| DemoApp::new(window, cx))
            },
        )
        .unwrap();
    });
}

struct DemoApp {
    theme: Theme,
    // Move ALL state to root view for interactive components to work
    click_count: usize,
    checkbox1_checked: bool,
    checkbox2_checked: bool,
    checkbox3_checked: bool,
    checkbox3_indeterminate: bool,
    toggle1_checked: bool,
    toggle2_checked: bool,
}

impl DemoApp {
    fn new(_window: &mut Window, cx: &mut App) -> Self {
        let theme = Theme::dark();
        install_theme(cx, theme.clone());

        Self {
            theme,
            click_count: 0,
            checkbox1_checked: false,
            checkbox2_checked: true,
            checkbox3_checked: false,
            checkbox3_indeterminate: true,
            toggle1_checked: false,
            toggle2_checked: true,
        }
    }

    // Helper method to render demo content directly in root view
    fn render_content(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap(px(32.0))
            .child(
                // Section: Buttons
                div()
                    .flex()
                    .flex_col()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(20.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(use_theme().tokens.foreground)
                            .child("Buttons")
                    )
                    .child(
                        div()
                            .flex()
                            .gap(px(12.0))
                            .flex_wrap()
                            .child(
                                Button::new("click-btn", "Click Me!")
                                    .on_click(cx.listener(|view, _event, _window, cx| {
                                        println!("[Demo] User handler called! Incrementing click_count from {} to {}", view.click_count, view.click_count + 1);
                                        view.click_count += 1;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                Button::new("secondary-btn", "Secondary")
                                    .variant(ButtonVariant::Secondary)
                                    .on_click(|_event, _window, _cx| {
                                        println!("Secondary button clicked!");
                                    })
                            )
                            .child(
                                Button::new("destructive-btn", "Destructive")
                                    .variant(ButtonVariant::Destructive)
                                    .on_click(|_event, _window, _cx| {
                                        println!("Destructive clicked!");
                                    })
                            )
                            .child(Button::new("outline-btn", "Outline").variant(ButtonVariant::Outline))
                            .child(Button::new("ghost-btn", "Ghost").variant(ButtonVariant::Ghost))
                            .child(Button::new("link-btn", "Link").variant(ButtonVariant::Link))
                            .child(Button::new("disabled-btn", "Disabled").disabled(true))
                    )
                    .child(
                        div()
                            .flex()
                            .gap(px(12.0))
                            .items_center()
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(use_theme().tokens.muted_foreground)
                                    .child(format!("Button clicked {} times", self.click_count))
                            )
                    )
                    .child(
                        div()
                            .flex()
                            .gap(px(12.0))
                            .flex_wrap()
                            .items_center()
                            .child(Button::new("small-btn", "Small").size(ButtonSize::Sm))
                            .child(Button::new("medium-btn", "Medium").size(ButtonSize::Md))
                            .child(Button::new("large-btn", "Large").size(ButtonSize::Lg))
                    )
            )
            .child(
                // Section: Inputs
                div()
                    .flex()
                    .flex_col()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(20.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(use_theme().tokens.foreground)
                            .child("Inputs")
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(12.0))
                            .child(div().child("TextField: API compatibility issues - placeholder_text not available"))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.0))
                                    .child(
                                        Checkbox::new("checkbox1")
                                            .checked(self.checkbox1_checked)
                                            .label("Checkbox - Unchecked by default")
                                            .on_click(cx.listener(|view, checked, _window, cx| {
                                                println!("[Demo] Checkbox1 user handler called! checked: {}", checked);
                                                view.checkbox1_checked = *checked;
                                                cx.notify();
                                            }))
                                    )
                                    .child(
                                        Checkbox::new("checkbox2")
                                            .checked(self.checkbox2_checked)
                                            .label("Checkbox - Checked by default")
                                            .on_click(cx.listener(|view, checked, _window, cx| {
                                                view.checkbox2_checked = *checked;
                                                cx.notify();
                                            }))
                                    )
                                    .child(
                                        Checkbox::new("checkbox3")
                                            .checked(self.checkbox3_checked)
                                            .indeterminate(self.checkbox3_indeterminate)
                                            .label("Checkbox - Indeterminate state (click to check)")
                                            .on_click(cx.listener(|view, checked, _window, cx| {
                                                // When indeterminate checkbox is clicked, clear indeterminate and set checked
                                                view.checkbox3_indeterminate = false;
                                                view.checkbox3_checked = *checked;
                                                cx.notify();
                                            }))
                                    )
                                    .child(
                                        Checkbox::new("checkbox4")
                                            .checked(true)
                                            .disabled(true)
                                            .label("Checkbox - Disabled")
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .gap(px(8.0))
                                            .child(
                                                Checkbox::new("checkbox5-sm")
                                                    .checked(true)
                                                    .size(CheckboxSize::Sm)
                                                    .label("Small")
                                            )
                                            .child(
                                                Checkbox::new("checkbox6-md")
                                                    .checked(true)
                                                    .size(CheckboxSize::Md)
                                                    .label("Medium")
                                            )
                                    )
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.0))
                                    .child(
                                        Toggle::new("toggle1")
                                            .checked(self.toggle1_checked)
                                            .label("Toggle - Off by default")
                                            .on_click(cx.listener(|view, checked, _window, cx| {
                                                println!("[Demo] Toggle1 user handler called! checked: {}", checked);
                                                view.toggle1_checked = *checked;
                                                cx.notify();
                                            }))
                                    )
                                    .child(
                                        Toggle::new("toggle2")
                                            .checked(self.toggle2_checked)
                                            .label("Toggle - On by default")
                                            .on_click(cx.listener(|view, checked, _window, cx| {
                                                view.toggle2_checked = *checked;
                                                cx.notify();
                                            }))
                                    )
                                    .child(
                                        Toggle::new("toggle3")
                                            .checked(true)
                                            .disabled(true)
                                            .label("Toggle - Disabled")
                                    )
                                    .child(
                                        Toggle::new("toggle4")
                                            .checked(true)
                                            .label("Toggle - Label on left")
                                            .label_side(LabelSide::Left)
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .gap(px(8.0))
                                            .child(
                                                Toggle::new("toggle5-sm")
                                                    .checked(true)
                                                    .size(ToggleSize::Sm)
                                                    .label("Small")
                                            )
                                            .child(
                                                Toggle::new("toggle6-md")
                                                    .checked(true)
                                                    .size(ToggleSize::Md)
                                                    .label("Medium")
                                            )
                                    )
                            )
                            .child(div().child("Select: API compatibility issues - on_click not available"))
                    )
            )
            .child(
                // Section: Tooltip
                div()
                    .flex()
                    .flex_col()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(20.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(use_theme().tokens.foreground)
                            .child("Tooltip")
                    )
                    .child(
                        div()
                            .flex()
                            .gap(px(12.0))
                            .child(div().child("Tooltip: Need to reimplement with proper pattern"))
                    )
            )
            .child(
                // Section: Navigation
                div()
                    .flex()
                    .flex_col()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(20.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(use_theme().tokens.foreground)
                            .child("Navigation")
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(12.0))
                            .child(div().child("Tabs: API compatibility issues - on_click not available"))
                            .child(div().child("Breadcrumbs: API compatibility issues - on_click not available"))
                            .child(div().child("TreeList: API compatibility issues - complex rendering"))
                    )
            )
            .child(
                // Section: Display
                div()
                    .flex()
                    .flex_col()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(20.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(use_theme().tokens.foreground)
                            .child("Display Components")
                    )
                    .child(
                        div()
                            .flex()
                            .gap(px(16.0))
                            .child(Badge::new("Default"))
                            .child(Badge::new("Secondary").variant(BadgeVariant::Secondary))
                            .child(Badge::new("Destructive").variant(BadgeVariant::Destructive))
                            .child(Badge::new("Outline").variant(BadgeVariant::Outline))
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(
                                div()
                                    .text_size(px(16.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(use_theme().tokens.foreground)
                                    .child("Card")
                            )
                            .child(
                                Card::new()
                                    .header(
                                        div()
                                            .text_size(px(18.0))
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .child("Card Title")
                                    )
                                    .content(
                                        div()
                                            .child("This is a card component with header, content, and footer sections. Cards are great for grouping related information.")
                                    )
                                    .footer(
                                        HStack::new()
                                            .justify(Justify::Between)
                                            .child(
                                                div()
                                                    .text_size(px(12.0))
                                                    .text_color(use_theme().tokens.muted_foreground)
                                                    .child("Card footer")
                                            )
                                            .child(
                                                Button::new("action-btn", "Action")
                                                    .size(ButtonSize::Sm)
                                            )
                                    )
                            )
                            .child(
                                Card::new()
                                    .content(
                                        div()
                                            .child("Simple card with just content - no header or footer")
                                    )
                            )
                    )
            )
            .child(
                // Section: Table
                div()
                    .flex()
                    .flex_col()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(20.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(use_theme().tokens.foreground)
                            .child("Table")
                    )
                    .child(
                        Table::new()
                            .columns(vec![
                                TableColumn::new("Name").width(px(200.0)),
                                TableColumn::new("Email").width(px(250.0)),
                                TableColumn::new("Role").width(px(150.0)),
                            ])
                            .rows(vec![
                                TableRow::new(vec![
                                    "John Doe".into(),
                                    "john@example.com".into(),
                                    "Admin".into(),
                                ]).selected(true),
                                TableRow::new(vec![
                                    "Jane Smith".into(),
                                    "jane@example.com".into(),
                                    "Developer".into(),
                                ]),
                                TableRow::new(vec![
                                    "Bob Johnson".into(),
                                    "bob@example.com".into(),
                                    "Designer".into(),
                                ]),
                                TableRow::new(vec![
                                    "Alice Williams".into(),
                                    "alice@example.com".into(),
                                    "Manager".into(),
                                ]),
                            ])
                    )
            )
            .child(
                // Section: Overlays
                div()
                    .flex()
                    .flex_col()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(20.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(use_theme().tokens.foreground)
                            .child("Overlays")
                    )
                    .child(
                        div()
                            .flex()
                            .gap(px(12.0))
                            .child(div().child("Dialog: API compatibility issues - event handling"))
                            .child(div().child("Popover: API compatibility issues - event handling"))
                            .child(div().child("Toast: API compatibility issues - event handling"))
                    )
            )
            .child(
                // Section: Status
                div()
                    .flex()
                    .flex_col()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(20.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(use_theme().tokens.foreground)
                            .child("Component Status")
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(8.0))
                            .text_size(px(14.0))
                            .text_color(use_theme().tokens.muted_foreground)
                            .child(div().child("✅ Scrollable - Fully functional with vertical/horizontal/both modes"))
                            .child(div().child("   • Interactive scrollbar with hover and drag states"))
                            .child(div().child("   • Auto fade-in/fade-out animations"))
                            .child(div().child("   • Click-to-jump and wheel scroll support"))
                            .child(div().child("✅ Button - FULLY WORKING! Clickable with all variants"))
                            .child(div().child("   • Proper event handlers using Rc<dyn Fn>"))
                            .child(div().child("   • Hover states and shadcn/ui styling"))
                            .child(div().child("   • All sizes (sm, md, lg, icon) working"))
                            .child(div().child("   • Disabled state with opacity"))
                            .child(div().child("✅ Badge - Display working"))
                            .child(div().child("✅ Checkbox - FULLY WORKING! Clickable with animations"))
                            .child(div().child("   • Checked, unchecked, and indeterminate states"))
                            .child(div().child("   • Animated fade-in/fade-out check mark"))
                            .child(div().child("   • Hover states and border highlighting"))
                            .child(div().child("   • Disabled state with proper opacity"))
                            .child(div().child("   • Sizes (sm, md) working"))
                            .child(div().child("✅ Toggle/Switch - FULLY WORKING! Clickable with sliding animation"))
                            .child(div().child("   • Smooth sliding thumb animation"))
                            .child(div().child("   • Hover states and color transitions"))
                            .child(div().child("   • Label positioning (left/right)"))
                            .child(div().child("   • Disabled state with proper opacity"))
                            .child(div().child("   • Sizes (sm, md) working"))
                            .child(div().child("⚠️ TextField - Needs Entity<InputState> pattern from gc"))
                            .child(div().child("⚠️ Select - Needs dropdown state management"))
                            .child(div().child("⚠️ Navigation (Tabs, Tree) - Need proper click handlers"))
                            .child(div().child("⚠️ Table - Needs proper implementation"))
                            .child(div().child("⚠️ Overlays (Dialog, Popover, Toast) - Need state management"))
                            .child(div().child("📝 Pattern: Components MUST be in root view, NOT nested entities!"))
                    )
            )
    }
}

impl Render for DemoApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        div()
            .bg(theme.tokens.background)
            .size_full()
            .flex()
            .flex_col()
            .child(
                // Header
                div()
                    .flex()
                    .items_center()
                    .px(px(24.0))
                    .py(px(16.0))
                    .border_b_1()
                    .border_color(theme.tokens.border)
                    .child(
                        div()
                            .text_size(px(24.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(theme.tokens.foreground)
                            .child("adabraka-ui Components Demo")
                    )
                    .child(
                        div()
                            .ml_auto()
                            .text_size(px(14.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("Scrollable showcase - Try clicking!")
                    )
            )
            .child(
                // Main scrollable content area - rendered directly in root view
                div()
                    .flex_1()
                    .w_full()
                    .overflow_hidden()
                    .child(
                        scrollable_vertical(
                            div()
                                .p(px(24.0))
                                .child(self.render_content(cx))
                        )
                    )
            )
    }
}
