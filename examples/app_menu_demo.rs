use gpui::{prelude::FluentBuilder as _, *};
use adabraka_ui::{
    navigation::app_menu::*,
    layout::VStack,
    theme::use_theme,
};

// Define all the actions our app will use
actions!(
    app_menu_demo,
    [
        // File menu actions
        NewFile,
        OpenFile,
        SaveFile,
        SaveFileAs,
        CloseFile,
        Quit,
        // Edit menu actions
        Undo,
        Redo,
        Cut,
        Copy,
        Paste,
        SelectAll,
        Find,
        Replace,
        // View menu actions
        ToggleSidebar,
        ToggleStatusBar,
        ZoomIn,
        ZoomOut,
        ZoomReset,
        FullScreen,
        // Window menu actions
        Minimize,
        NewWindow,
        // Help menu actions
        Documentation,
        ReportIssue,
        About,
    ]
);

struct AppMenuDemo {
    focus: FocusHandle,
    last_action: String,
    sidebar_visible: bool,
    status_bar_visible: bool,
    zoom_level: i32,
}

impl AppMenuDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            focus: cx.focus_handle(),
            last_action: "None".to_string(),
            sidebar_visible: true,
            status_bar_visible: true,
            zoom_level: 100,
        }
    }

    fn handle_action(&mut self, action_name: &str, cx: &mut Context<Self>) {
        self.last_action = action_name.to_string();

        // Handle specific actions that change state
        match action_name {
            "Toggle Sidebar" => {
                self.sidebar_visible = !self.sidebar_visible;
            }
            "Toggle Status Bar" => {
                self.status_bar_visible = !self.status_bar_visible;
            }
            "Zoom In" => {
                self.zoom_level = (self.zoom_level + 10).min(200);
            }
            "Zoom Out" => {
                self.zoom_level = (self.zoom_level - 10).max(50);
            }
            "Zoom Reset" => {
                self.zoom_level = 100;
            }
            _ => {}
        }

        cx.notify();
    }
}

impl Render for AppMenuDemo {
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
            .on_action(cx.listener(|this, _: &SaveFileAs, _window, cx| {
                this.handle_action("Save File As", cx);
            }))
            .on_action(cx.listener(|this, _: &CloseFile, _window, cx| {
                this.handle_action("Close File", cx);
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
            .on_action(cx.listener(|this, _: &SelectAll, _window, cx| {
                this.handle_action("Select All", cx);
            }))
            .on_action(cx.listener(|this, _: &Find, _window, cx| {
                this.handle_action("Find", cx);
            }))
            .on_action(cx.listener(|this, _: &Replace, _window, cx| {
                this.handle_action("Replace", cx);
            }))
            .on_action(cx.listener(|this, _: &ToggleSidebar, _window, cx| {
                this.handle_action("Toggle Sidebar", cx);
            }))
            .on_action(cx.listener(|this, _: &ToggleStatusBar, _window, cx| {
                this.handle_action("Toggle Status Bar", cx);
            }))
            .on_action(cx.listener(|this, _: &ZoomIn, _window, cx| {
                this.handle_action("Zoom In", cx);
            }))
            .on_action(cx.listener(|this, _: &ZoomOut, _window, cx| {
                this.handle_action("Zoom Out", cx);
            }))
            .on_action(cx.listener(|this, _: &ZoomReset, _window, cx| {
                this.handle_action("Zoom Reset", cx);
            }))
            .on_action(cx.listener(|this, _: &FullScreen, _window, cx| {
                this.handle_action("Full Screen", cx);
            }))
            .on_action(cx.listener(|this, _: &Minimize, _window, cx| {
                this.handle_action("Minimize", cx);
            }))
            .on_action(cx.listener(|this, _: &NewWindow, _window, cx| {
                this.handle_action("New Window", cx);
            }))
            .on_action(cx.listener(|this, _: &Documentation, _window, cx| {
                this.handle_action("Documentation", cx);
            }))
            .on_action(cx.listener(|this, _: &ReportIssue, _window, cx| {
                this.handle_action("Report Issue", cx);
            }))
            .on_action(cx.listener(|this, _: &About, _window, cx| {
                this.handle_action("About", cx);
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
                                    .child("Application Menu Demo")
                            )
                            .child(
                                div()
                                    .text_size(px(16.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Native OS menu bar at the top of the screen")
                            )
                    )
                    // Instructions
                    .child(
                        div()
                            .p(px(16.0))
                            .bg(theme.tokens.muted.opacity(0.3))
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
                                    .child("How to Use")
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .child("• Look at the top of your screen (macOS) or top of the window (Windows/Linux)")
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .child("• Click on File, Edit, View, Window, or Help menus")
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .child("• Select any menu item to see it reflected in the status below")
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .child("• Try keyboard shortcuts (Cmd+N, Cmd+S, Cmd+Z, etc.)")
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
                                    .child("Current State")
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .child(format!("Last Action: {}", self.last_action))
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .child(format!("Sidebar: {}", if self.sidebar_visible { "Visible" } else { "Hidden" }))
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .child(format!("Status Bar: {}", if self.status_bar_visible { "Visible" } else { "Hidden" }))
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .child(format!("Zoom Level: {}%", self.zoom_level))
                            )
                    )
                    // Features
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(
                                div()
                                    .text_size(px(20.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("Menu Features Demonstrated")
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
                                                    .child("✓")
                                            )
                                            .child("Native OS menu bar integration")
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .gap(px(8.0))
                                            .child(
                                                div()
                                                    .text_color(theme.tokens.accent)
                                                    .child("✓")
                                            )
                                            .child("Standard menus: File, Edit, View, Window, Help")
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .gap(px(8.0))
                                            .child(
                                                div()
                                                    .text_color(theme.tokens.accent)
                                                    .child("✓")
                                            )
                                            .child("Separators between menu sections")
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .gap(px(8.0))
                                            .child(
                                                div()
                                                    .text_color(theme.tokens.accent)
                                                    .child("✓")
                                            )
                                            .child("Submenus (Recent Files under File menu)")
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .gap(px(8.0))
                                            .child(
                                                div()
                                                    .text_color(theme.tokens.accent)
                                                    .child("✓")
                                            )
                                            .child("OS-specific menus (Services on macOS)")
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
            KeyBinding::new("cmd-shift-s", SaveFileAs, None),
            KeyBinding::new("cmd-w", CloseFile, None),
            KeyBinding::new("cmd-q", Quit, None),
            // Edit
            KeyBinding::new("cmd-z", Undo, None),
            KeyBinding::new("cmd-shift-z", Redo, None),
            KeyBinding::new("cmd-x", Cut, None),
            KeyBinding::new("cmd-c", Copy, None),
            KeyBinding::new("cmd-v", Paste, None),
            KeyBinding::new("cmd-a", SelectAll, None),
            KeyBinding::new("cmd-f", Find, None),
            KeyBinding::new("cmd-shift-f", Replace, None),
            // View
            KeyBinding::new("cmd-b", ToggleSidebar, None),
            KeyBinding::new("cmd-=", ZoomIn, None),
            KeyBinding::new("cmd--", ZoomOut, None),
            KeyBinding::new("cmd-0", ZoomReset, None),
            KeyBinding::new("cmd-ctrl-f", FullScreen, None),
            // Window
            KeyBinding::new("cmd-m", Minimize, None),
            KeyBinding::new("cmd-shift-n", NewWindow, None),
        ]);

        // Build the menu bar using our builder API
        let menu_bar = AppMenuBar::new()
            // File menu
            .menu(
                file_menu()
                    .action("New File", NewFile)
                    .action("Open File", OpenFile)
                    .separator()
                    .submenu(
                        AppMenu::new("Recent Files")
                            .action("project1.txt", OpenFile)
                            .action("project2.txt", OpenFile)
                            .action("project3.txt", OpenFile)
                    )
                    .separator()
                    .action("Save", SaveFile)
                    .action("Save As...", SaveFileAs)
                    .separator()
                    .action("Close File", CloseFile)
                    .separator()
                    .action("Quit", Quit)
            )
            // Edit menu
            .menu(
                edit_menu()
                    .action("Undo", Undo)
                    .action("Redo", Redo)
                    .separator()
                    .action("Cut", Cut)
                    .action("Copy", Copy)
                    .action("Paste", Paste)
                    .separator()
                    .action("Select All", SelectAll)
                    .separator()
                    .action("Find", Find)
                    .action("Replace", Replace)
            )
            // View menu
            .menu(
                view_menu()
                    .action("Toggle Sidebar", ToggleSidebar)
                    .action("Toggle Status Bar", ToggleStatusBar)
                    .separator()
                    .action("Zoom In", ZoomIn)
                    .action("Zoom Out", ZoomOut)
                    .action("Reset Zoom", ZoomReset)
                    .separator()
                    .action("Full Screen", FullScreen)
            )
            // Window menu
            .menu(
                window_menu()
                    .action("Minimize", Minimize)
                    .action("New Window", NewWindow)
            )
            // Help menu
            .menu(
                help_menu()
                    .action("Documentation", Documentation)
                    .action("Report an Issue", ReportIssue)
                    .separator()
                    .action("About", About)
            );

        // Set the menus on the application
        cx.set_menus(menu_bar.build());

        // Activate the menu bar so it's visible
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
                    title: Some("Application Menu Demo".into()),
                    appears_transparent: false,
                    ..Default::default()
                }),
                ..Default::default()
            },
            |window, cx| cx.new(|cx| {
                let view = AppMenuDemo::new(cx);
                window.focus(&view.focus);
                view
            }),
        )
        .unwrap();
    });
}
