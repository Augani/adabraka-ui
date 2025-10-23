use gpui::*;
use adabraka_ui::{
    navigation::app_menu::*,
    layout::VStack,
    theme::use_theme,
};

// Define all the actions our app will use
actions!(
    app_menu_styled_demo,
    [
        // File menu actions
        NewFile,
        OpenFile,
        SaveFile,
        Quit,
        // Edit menu actions
        Undo,
        Redo,
        Cut,
        Copy,
        Paste,
        // View menu actions
        ToggleSidebar,
        ZoomIn,
        ZoomOut,
    ]
);

struct AppMenuStyledDemo {
    focus: FocusHandle,
    last_action: String,
}

impl AppMenuStyledDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            focus: cx.focus_handle(),
            last_action: "None".to_string(),
        }
    }

    fn handle_action(&mut self, action_name: &str, cx: &mut Context<Self>) {
        self.last_action = action_name.to_string();
        cx.notify();
    }
}

impl Render for AppMenuStyledDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        div()
            .track_focus(&self.focus)
            .size_full()
            .bg(theme.tokens.background)
            .text_color(theme.tokens.foreground)
            // Register all action handlers
            .on_action(cx.listener(|this, _: &NewFile, _window, cx| {
                this.handle_action("New File", cx);
            }))
            .on_action(cx.listener(|this, _: &OpenFile, _window, cx| {
                this.handle_action("Open File", cx);
            }))
            .on_action(cx.listener(|this, _: &SaveFile, _window, cx| {
                this.handle_action("Save File", cx);
            }))
            .on_action(cx.listener(|this, _: &Undo, _window, cx| {
                this.handle_action("Undo", cx);
            }))
            .on_action(cx.listener(|this, _: &Redo, _window, cx| {
                this.handle_action("Redo", cx);
            }))
            .on_action(cx.listener(|this, _: &Cut, _window, cx| {
                this.handle_action("Cut", cx);
            }))
            .on_action(cx.listener(|this, _: &Copy, _window, cx| {
                this.handle_action("Copy", cx);
            }))
            .on_action(cx.listener(|this, _: &Paste, _window, cx| {
                this.handle_action("Paste", cx);
            }))
            .on_action(cx.listener(|this, _: &ToggleSidebar, _window, cx| {
                this.handle_action("Toggle Sidebar", cx);
            }))
            .on_action(cx.listener(|this, _: &ZoomIn, _window, cx| {
                this.handle_action("Zoom In", cx);
            }))
            .on_action(cx.listener(|this, _: &ZoomOut, _window, cx| {
                this.handle_action("Zoom Out", cx);
            }))
            .child(
                VStack::new()
                    .p(px(32.0))
                    .gap(px(32.0))
                    .size_full()
                    // Header
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .text_size(px(32.0))
                                    .font_weight(FontWeight::BOLD)
                                    .child("AppMenu Styled Trait Demo")
                            )
                            .child(
                                div()
                                    .text_size(px(16.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Demonstrating Styled trait implementation on AppMenu")
                            )
                    )
                    // Important Note
                    .child(
                        div()
                            .p(px(16.0))
                            .bg(theme.tokens.accent.opacity(0.2))
                            .rounded(theme.tokens.radius_md)
                            .border_1()
                            .border_color(theme.tokens.accent)
                            .flex()
                            .flex_col()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .text_size(px(16.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(theme.tokens.accent_foreground)
                                    .child("Important Note About Styled Trait on AppMenu")
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .child("AppMenu now implements the Styled trait, allowing you to call GPUI styling methods like .p_4(), .bg(), .rounded(), etc.")
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .child("However, AppMenu builds native OS menu bars (macOS menu bar, Windows/Linux top menu). Native OS menus are rendered by the operating system and do not support custom styling.")
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .child("The Styled trait implementation provides API consistency across all components, but the style settings will not affect the visual appearance of native menus.")
                            )
                    )
                    // API Examples
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(
                                div()
                                    .text_size(px(20.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("Styled Trait API Examples")
                            )
                            .child(
                                div()
                                    .p(px(16.0))
                                    .bg(theme.tokens.card)
                                    .rounded(theme.tokens.radius_md)
                                    .border_1()
                                    .border_color(theme.tokens.border)
                                    .flex()
                                    .flex_col()
                                    .gap(px(12.0))
                                    .child(
                                        div()
                                            .font_family("monospace")
                                            .text_size(px(13.0))
                                            .text_color(theme.tokens.accent)
                                            .child("// You can now use Styled trait methods on AppMenu:")
                                    )
                                    .child(
                                        div()
                                            .font_family("monospace")
                                            .text_size(px(13.0))
                                            .child("file_menu()")
                                    )
                                    .child(
                                        div()
                                            .font_family("monospace")
                                            .text_size(px(13.0))
                                            .ml(px(16.0))
                                            .child(".p_4()          // Padding")
                                    )
                                    .child(
                                        div()
                                            .font_family("monospace")
                                            .text_size(px(13.0))
                                            .ml(px(16.0))
                                            .child(".bg(color)      // Background color")
                                    )
                                    .child(
                                        div()
                                            .font_family("monospace")
                                            .text_size(px(13.0))
                                            .ml(px(16.0))
                                            .child(".rounded(px)    // Border radius")
                                    )
                                    .child(
                                        div()
                                            .font_family("monospace")
                                            .text_size(px(13.0))
                                            .ml(px(16.0))
                                            .child(".shadow_sm()    // Shadow effects")
                                    )
                                    .child(
                                        div()
                                            .font_family("monospace")
                                            .text_size(px(13.0))
                                            .ml(px(16.0))
                                            .child(".action(\"New\", NewFile)")
                                    )
                                    .child(
                                        div()
                                            .font_family("monospace")
                                            .text_size(px(13.0))
                                            .ml(px(16.0))
                                            .child(".build()")
                                    )
                            )
                    )
                    // Current State
                    .child(
                        div()
                            .p(px(16.0))
                            .bg(theme.tokens.card)
                            .rounded(theme.tokens.radius_md)
                            .border_1()
                            .border_color(theme.tokens.border)
                            .flex()
                            .flex_col()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .text_size(px(16.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("Menu Interaction Status")
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .child(format!("Last Action: {}", self.last_action))
                            )
                            .child(
                                div()
                                    .mt(px(8.0))
                                    .text_size(px(12.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Try using the menu bar at the top or keyboard shortcuts:")
                            )
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Cmd+N (New), Cmd+O (Open), Cmd+S (Save), Cmd+Z (Undo), etc.")
                            )
                    )
                    // Use Cases
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(
                                div()
                                    .text_size(px(20.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("Why Implement Styled on AppMenu?")
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .flex()
                                            .gap(px(8.0))
                                            .child(
                                                div()
                                                    .text_color(theme.tokens.accent)
                                                    .child("1.")
                                            )
                                            .child("API Consistency: All components in the library follow the same pattern")
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .gap(px(8.0))
                                            .child(
                                                div()
                                                    .text_color(theme.tokens.accent)
                                                    .child("2.")
                                            )
                                            .child("Future-Proofing: If GPUI adds custom menu styling in the future, it's ready")
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .gap(px(8.0))
                                            .child(
                                                div()
                                                    .text_color(theme.tokens.accent)
                                                    .child("3.")
                                            )
                                            .child("Metadata: Style settings can be used as metadata for documentation or testing")
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .gap(px(8.0))
                                            .child(
                                                div()
                                                    .text_color(theme.tokens.accent)
                                                    .child("4.")
                                            )
                                            .child("Chainable API: Maintains the fluent builder pattern across all components")
                                    )
                            )
                    )
            )
    }
}

fn main() {
    Application::new().run(move |cx: &mut App| {
        // Install dark theme
        adabraka_ui::theme::install_theme(cx, adabraka_ui::theme::Theme::dark());

        // Initialize UI system
        adabraka_ui::init(cx);

        // Register Quit action at app level
        cx.on_action(|_: &Quit, cx| {
            println!("Quitting application...");
            cx.quit();
        });

        // Bind keyboard shortcuts
        cx.bind_keys([
            // File
            KeyBinding::new("cmd-n", NewFile, None),
            KeyBinding::new("cmd-o", OpenFile, None),
            KeyBinding::new("cmd-s", SaveFile, None),
            KeyBinding::new("cmd-q", Quit, None),
            // Edit
            KeyBinding::new("cmd-z", Undo, None),
            KeyBinding::new("cmd-shift-z", Redo, None),
            KeyBinding::new("cmd-x", Cut, None),
            KeyBinding::new("cmd-c", Copy, None),
            KeyBinding::new("cmd-v", Paste, None),
            // View
            KeyBinding::new("cmd-b", ToggleSidebar, None),
            KeyBinding::new("cmd-=", ZoomIn, None),
            KeyBinding::new("cmd--", ZoomOut, None),
        ]);

        // Build menu bar demonstrating Styled trait API
        // Note: Styling methods can be called but won't affect native OS menus
        let menu_bar = AppMenuBar::new()
            // File menu with Styled trait methods (for API demonstration)
            .menu(
                file_menu()
                    // These Styled trait methods can be called, maintaining API consistency
                    .p_4()          // Styled trait: padding
                    .rounded(px(8.0))  // Styled trait: border radius
                    .action("New File", NewFile)
                    .action("Open File", OpenFile)
                    .separator()
                    .action("Save", SaveFile)
                    .separator()
                    .action("Quit", Quit)
            )
            // Edit menu
            .menu(
                edit_menu()
                    .bg(rgb(0x3b82f6))  // Styled trait: background (demonstrates API, not visual)
                    .action("Undo", Undo)
                    .action("Redo", Redo)
                    .separator()
                    .action("Cut", Cut)
                    .action("Copy", Copy)
                    .action("Paste", Paste)
            )
            // View menu
            .menu(
                view_menu()
                    .shadow_sm()  // Styled trait: shadow
                    .action("Toggle Sidebar", ToggleSidebar)
                    .separator()
                    .action("Zoom In", ZoomIn)
                    .action("Zoom Out", ZoomOut)
            );

        // Set the menus on the application
        cx.set_menus(menu_bar.build());

        // Activate the menu bar
        cx.activate(true);

        // Create window
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(1000.0), px(800.0)),
                    cx,
                ))),
                titlebar: Some(TitlebarOptions {
                    title: Some("AppMenu Styled Trait Demo".into()),
                    appears_transparent: false,
                    ..Default::default()
                }),
                ..Default::default()
            },
            |window, cx| cx.new(|cx| {
                let view = AppMenuStyledDemo::new(cx);
                window.focus(&view.focus);
                view
            }),
        )
        .unwrap();
    });
}
