use adabraka_ui::{
    components::{
        alert::Alert,
        scrollable::scrollable_vertical,
        text::{body, caption, h1, h2},
    },
    theme::{install_theme, use_theme, Theme},
};
use gpui::{prelude::FluentBuilder as _, *};

struct AlertDemo;

impl AlertDemo {
    fn new() -> Self {
        Self
    }
}

impl Render for AlertDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(theme.tokens.background)
            .overflow_hidden()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .p(px(24.0))
                    .border_b_1()
                    .border_color(theme.tokens.border)
                    .child(h1("Alert / Banner"))
                    .child(caption(
                        "Alert components for displaying important messages with different severity levels",
                    )),
            )
            .child(scrollable_vertical(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(32.0))
                    .p(px(24.0))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(h2("Basic Variants"))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(12.0))
                                    .child(
                                        Alert::info()
                                            .title("Information")
                                            .description("This is an informational alert message."),
                                    )
                                    .child(
                                        Alert::success()
                                            .title("Success")
                                            .description("Your changes have been saved successfully."),
                                    )
                                    .child(
                                        Alert::warning()
                                            .title("Warning")
                                            .description("Please review your input before proceeding."),
                                    )
                                    .child(
                                        Alert::error()
                                            .title("Error")
                                            .description("An error occurred while processing your request."),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(h2("Title Only"))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(12.0))
                                    .child(Alert::info().title("New update available"))
                                    .child(Alert::success().title("Payment processed"))
                                    .child(Alert::warning().title("Low disk space"))
                                    .child(Alert::error().title("Connection failed")),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(h2("Description Only"))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(12.0))
                                    .child(Alert::info().description(
                                        "Your session will expire in 5 minutes.",
                                    ))
                                    .child(Alert::success().description(
                                        "All files have been uploaded to the server.",
                                    )),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(h2("Without Icon"))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(12.0))
                                    .child(
                                        Alert::info()
                                            .show_icon(false)
                                            .title("Minimal Alert")
                                            .description("This alert has no icon displayed."),
                                    )
                                    .child(
                                        Alert::warning()
                                            .show_icon(false)
                                            .title("Clean Design")
                                            .description("Sometimes less is more."),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(h2("With Action Button"))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(12.0))
                                    .child(
                                        Alert::info()
                                            .title("New Version Available")
                                            .description("Version 2.0 is now available with new features.")
                                            .action("Update Now", |_, _| {
                                                println!("Update clicked!");
                                            }),
                                    )
                                    .child(
                                        Alert::warning()
                                            .title("Subscription Expiring")
                                            .description("Your subscription will expire in 3 days.")
                                            .action("Renew Subscription", |_, _| {
                                                println!("Renew clicked!");
                                            }),
                                    )
                                    .child(
                                        Alert::error()
                                            .title("Authentication Failed")
                                            .description("Your session has expired.")
                                            .action("Sign In Again", |_, _| {
                                                println!("Sign in clicked!");
                                            }),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(h2("Dismissible Alerts"))
                            .child(body("Click the X button to dismiss (prints to console)"))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(12.0))
                                    .child(
                                        Alert::info()
                                            .title("Dismissible Info")
                                            .description("You can close this alert.")
                                            .on_dismiss(|_, _| {
                                                println!("Info alert dismissed!");
                                            }),
                                    )
                                    .child(
                                        Alert::success()
                                            .title("Dismissible Success")
                                            .description("Task completed! Dismiss when ready.")
                                            .on_dismiss(|_, _| {
                                                println!("Success alert dismissed!");
                                            }),
                                    )
                                    .child(
                                        Alert::warning()
                                            .title("Dismissible Warning")
                                            .description("Review and dismiss.")
                                            .on_dismiss(|_, _| {
                                                println!("Warning alert dismissed!");
                                            }),
                                    )
                                    .child(
                                        Alert::error()
                                            .title("Dismissible Error")
                                            .description("Acknowledge and dismiss.")
                                            .on_dismiss(|_, _| {
                                                println!("Error alert dismissed!");
                                            }),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(h2("Combined Features"))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(12.0))
                                    .child(
                                        Alert::info()
                                            .title("Complete Alert")
                                            .description(
                                                "This alert has all features: title, description, action, and dismiss.",
                                            )
                                            .action("Learn More", |_, _| {
                                                println!("Learn more clicked!");
                                            })
                                            .dismissible(true),
                                    )
                                    .child(
                                        Alert::success()
                                            .title("Deployment Complete")
                                            .description("Your application has been deployed to production.")
                                            .action("View Logs", |_, _| {
                                                println!("View logs clicked!");
                                            })
                                            .dismissible(true),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(h2("Custom Icon"))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(12.0))
                                    .child(
                                        Alert::info()
                                            .icon("bell")
                                            .title("Notification")
                                            .description("You have new notifications waiting."),
                                    )
                                    .child(
                                        Alert::success()
                                            .icon("download")
                                            .title("Download Complete")
                                            .description("Your file has been downloaded successfully."),
                                    )
                                    .child(
                                        Alert::warning()
                                            .icon("clock")
                                            .title("Session Timeout")
                                            .description("Your session will timeout in 10 minutes."),
                                    )
                                    .child(
                                        Alert::error()
                                            .icon("wifi-off")
                                            .title("Connection Lost")
                                            .description("Please check your internet connection."),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.0))
                            .child(h2("Usage Examples"))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(12.0))
                                    .p(px(16.0))
                                    .rounded(theme.tokens.radius_md)
                                    .bg(theme.tokens.muted.opacity(0.3))
                                    .child(caption("// Basic alert"))
                                    .child(caption("Alert::info()"))
                                    .child(caption("    .title(\"Information\")"))
                                    .child(caption("    .description(\"Message here\")"))
                                    .child(caption(""))
                                    .child(caption("// With action"))
                                    .child(caption("Alert::warning()"))
                                    .child(caption("    .title(\"Warning\")"))
                                    .child(caption("    .action(\"Fix Now\", |_, _| { ... })"))
                                    .child(caption(""))
                                    .child(caption("// Dismissible"))
                                    .child(caption("Alert::success()"))
                                    .child(caption("    .title(\"Success\")"))
                                    .child(caption("    .on_dismiss(|_, _| { ... })"))
                                    .child(caption(""))
                                    .child(caption("// Custom icon"))
                                    .child(caption("Alert::error()"))
                                    .child(caption("    .icon(\"wifi-off\")"))
                                    .child(caption("    .title(\"No Connection\")")),
                            ),
                    ),
            ))
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        adabraka_ui::init(cx);
        install_theme(cx, Theme::dark());

        let bounds = Bounds::centered(None, size(px(800.0), px(900.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("Alert Demo".into()),
                    appears_transparent: false,
                    traffic_light_position: None,
                }),
                ..Default::default()
            },
            |_, cx| cx.new(|_| AlertDemo::new()),
        )
        .unwrap();

        cx.activate(true);
    });
}
