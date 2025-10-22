use gpui::{prelude::FluentBuilder as _, *};
use adabraka_ui::{
    theme::{install_theme, Theme, use_theme},
    components::{
        scrollable::scrollable_vertical,
        text::*,
    },
};

struct TextDemo;

impl Render for TextDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(theme.tokens.background)
            .child(
                // Header
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .p(px(24.0))
                    .border_b_1()
                    .border_color(theme.tokens.border)
                    .child(h1("Text Component System"))
                    .child(caption("Typography made simple with built-in theming and variants"))
            )
            .child(
                scrollable_vertical(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(32.0))
                        .p(px(24.0))
                    // Heading Variants
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(h2("Heading Variants"))
                            .child(body("Six semantic heading sizes with appropriate font weights"))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(12.0))
                                    .p(px(16.0))
                                    .rounded(theme.tokens.radius_md)
                                    .bg(theme.tokens.muted.opacity(0.3))
                                    .child(h1("H1: Extra Large Heading (32px, Bold)"))
                                    .child(h2("H2: Large Heading (28px, Semibold)"))
                                    .child(h3("H3: Medium Heading (24px, Semibold)"))
                                    .child(h4("H4: Small Heading (20px, Semibold)"))
                                    .child(h5("H5: Extra Small Heading (18px, Medium)"))
                                    .child(h6("H6: Tiny Heading (16px, Medium)"))
                            )
                    )
                    // Body Text Variants
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(h2("Body Text Variants"))
                            .child(body("Different sizes for various content needs"))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(12.0))
                                    .p(px(16.0))
                                    .rounded(theme.tokens.radius_md)
                                    .bg(theme.tokens.muted.opacity(0.3))
                                    .child(body_large("Body Large: Perfect for lead paragraphs and important content (16px)"))
                                    .child(body("Body: Default text size for paragraphs and regular content (14px)"))
                                    .child(body_small("Body Small: Compact text for secondary information (13px)"))
                                    .child(caption("Caption: Small text for hints and metadata (12px)"))
                            )
                    )
                    // Label Variants
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(h2("Label Variants"))
                            .child(body("Medium weight text for form labels and UI elements"))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(12.0))
                                    .p(px(16.0))
                                    .rounded(theme.tokens.radius_md)
                                    .bg(theme.tokens.muted.opacity(0.3))
                                    .child(label("Label: Standard form labels (14px, Medium)"))
                                    .child(label_small("Label Small: Compact labels (12px, Medium)"))
                            )
                    )
                    // Code Variants
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(h2("Code/Monospace Variants"))
                            .child(body("Monospace font for code and technical content"))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(12.0))
                                    .p(px(16.0))
                                    .rounded(theme.tokens.radius_md)
                                    .bg(theme.tokens.muted.opacity(0.3))
                                    .child(code("fn main() { println!(\"Hello, World!\"); }"))
                                    .child(code_small("const API_URL = \"https://api.example.com\""))
                            )
                    )
                    // Muted Text
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(h2("Muted Text"))
                            .child(body("Secondary text color for less important content"))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(12.0))
                                    .p(px(16.0))
                                    .rounded(theme.tokens.radius_md)
                                    .bg(theme.tokens.muted.opacity(0.3))
                                    .child(body("This is regular text with default color"))
                                    .child(muted("This is muted text with secondary color"))
                                    .child(muted_small("This is small muted text"))
                            )
                    )
                    // Text Decorations
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(h2("Text Decorations"))
                            .child(body("Text decoration support (underline is currently supported)"))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(12.0))
                                    .p(px(16.0))
                                    .rounded(theme.tokens.radius_md)
                                    .bg(theme.tokens.muted.opacity(0.3))
                                    .child(body("Normal text without decorations"))
                                    .child(Text::new("Underlined text").underline())
                                    .child(caption("Note: Italic and strikethrough will be supported in future GPUI updates"))
                            )
                    )
                    // Custom Styling
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(h2("Custom Styling"))
                            .child(body("Override size, weight, color, and font"))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(12.0))
                                    .p(px(16.0))
                                    .rounded(theme.tokens.radius_md)
                                    .bg(theme.tokens.muted.opacity(0.3))
                                    .child(
                                        Text::new("Custom size text")
                                            .size(px(20.0))
                                    )
                                    .child(
                                        Text::new("Custom weight text")
                                            .weight(FontWeight::BLACK)
                                    )
                                    .child(
                                        Text::new("Custom color text")
                                            .color(rgb(0x10b981).into())
                                    )
                                    .child(
                                        Text::new("All custom: large, bold, colored")
                                            .size(px(18.0))
                                            .weight(FontWeight::BOLD)
                                            .color(rgb(0x3b82f6).into())
                                    )
                            )
                    )
                    // Text Wrapping
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(h2("Text Wrapping & Truncation"))
                            .child(body("Control how text handles overflow"))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(12.0))
                                    .child(
                                        div()
                                            .w(px(400.0))
                                            .p(px(16.0))
                                            .rounded(theme.tokens.radius_md)
                                            .bg(theme.tokens.muted.opacity(0.3))
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .gap(px(12.0))
                                                    .child(label("Normal wrapping:"))
                                                    .child(body("This is a long text that will wrap to multiple lines when it reaches the edge of its container. It demonstrates the default wrapping behavior."))
                                                    .child(label("No wrap:"))
                                                    .child(
                                                        Text::new("This text will not wrap and will overflow the container boundaries")
                                                            .no_wrap()
                                                    )
                                                    .child(label("Truncated with ellipsis:"))
                                                    .child(
                                                        Text::new("This text is truncated with an ellipsis when it's too long to fit")
                                                            .truncate()
                                                    )
                                            )
                                    )
                            )
                    )
                    // Color Palette Examples
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(h2("Theme Colors"))
                            .child(body("Using theme colors for semantic meaning"))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(12.0))
                                    .p(px(16.0))
                                    .rounded(theme.tokens.radius_md)
                                    .bg(theme.tokens.muted.opacity(0.3))
                                    .child(
                                        Text::new("Primary: Main brand color")
                                            .color(theme.tokens.primary)
                                            .weight(FontWeight::SEMIBOLD)
                                    )
                                    .child(
                                        Text::new("Accent: Highlighted elements")
                                            .color(theme.tokens.accent_foreground)
                                            .weight(FontWeight::SEMIBOLD)
                                    )
                                    .child(
                                        Text::new("Destructive: Errors and warnings")
                                            .color(theme.tokens.destructive)
                                            .weight(FontWeight::SEMIBOLD)
                                    )
                                    .child(
                                        Text::new("Muted: Secondary information")
                                            .color(theme.tokens.muted_foreground)
                                    )
                            )
                    )
                    // Usage Examples
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(h2("Real-World Examples"))
                            .child(body("Common UI patterns using the text component"))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(20.0))
                                    // Article Header
                                    .child(
                                        div()
                                            .p(px(16.0))
                                            .rounded(theme.tokens.radius_md)
                                            .bg(theme.tokens.card)
                                            .border_1()
                                            .border_color(theme.tokens.border)
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .child(h3("Article Title"))
                                            .child(muted_small("Published on January 15, 2025 by John Doe"))
                                            .child(body("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."))
                                    )
                                    // Form Field
                                    .child(
                                        div()
                                            .p(px(16.0))
                                            .rounded(theme.tokens.radius_md)
                                            .bg(theme.tokens.card)
                                            .border_1()
                                            .border_color(theme.tokens.border)
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .child(label("Email Address"))
                                            .child(caption("We'll never share your email with anyone else"))
                                            .child(
                                                div()
                                                    .h(px(40.0))
                                                    .px(px(12.0))
                                                    .rounded(theme.tokens.radius_sm)
                                                    .border_1()
                                                    .border_color(theme.tokens.input)
                                                    .flex()
                                                    .items_center()
                                                    .child(muted("user@example.com"))
                                            )
                                    )
                                    // Status Message
                                    .child(
                                        div()
                                            .p(px(16.0))
                                            .rounded(theme.tokens.radius_md)
                                            .bg(hsla(0.45, 0.84, 0.53, 0.1)) // green with 10% opacity
                                            .border_1()
                                            .border_color(hsla(0.45, 0.84, 0.53, 1.0)) // green
                                            .flex()
                                            .flex_col()
                                            .gap(px(4.0))
                                            .child(
                                                Text::new("Success!")
                                                    .weight(FontWeight::SEMIBOLD)
                                                    .color(hsla(0.45, 0.84, 0.53, 1.0))
                                            )
                                            .child(
                                                Text::new("Your changes have been saved successfully.")
                                                    .color(hsla(0.45, 0.88, 0.40, 1.0)) // darker green
                                                    .size(px(13.0))
                                            )
                                    )
                            )
                    )
                    // Benefits
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .p(px(16.0))
                            .rounded(theme.tokens.radius_lg)
                            .bg(theme.tokens.primary.opacity(0.1))
                            .border_1()
                            .border_color(theme.tokens.primary)
                            .child(h2("Benefits"))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.0))
                                    .child(body("✓ No need to manually apply font family on every text element"))
                                    .child(body("✓ Consistent typography across your application"))
                                    .child(body("✓ Easy to change fonts globally by updating theme"))
                                    .child(body("✓ Semantic variants for proper hierarchy"))
                                    .child(body("✓ Builder pattern for custom styling"))
                                    .child(body("✓ Automatic theme color integration"))
                            )
                    )
                )
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        // Initialize the UI library
        adabraka_ui::init(cx);

        // Install dark theme
        install_theme(cx, Theme::dark());

        let bounds = Bounds::centered(None, size(px(900.0), px(800.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("Text Component Demo".into()),
                    appears_transparent: false,
                    traffic_light_position: None,
                }),
                ..Default::default()
            },
            |_, cx| cx.new(|_| TextDemo),
        )
        .unwrap();

        cx.activate(true);
    });
}
