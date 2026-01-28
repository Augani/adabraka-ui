use adabraka_ui::{
    components::scrollable::scrollable_vertical,
    components::stepper::{StepItem, StepStatus, Stepper, StepperSize, StepperState},
    prelude::*,
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
                        title: Some("Stepper Component Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(1000.0), px(900.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|cx| StepperDemo::new(cx)),
            )
            .unwrap();
        });
}

struct StepperDemo {
    horizontal_stepper: Entity<StepperState>,
    vertical_stepper: Entity<StepperState>,
    small_stepper: Entity<StepperState>,
    large_stepper: Entity<StepperState>,
    non_linear_stepper: Entity<StepperState>,
    icons_stepper: Entity<StepperState>,
    error_stepper: Entity<StepperState>,
}

impl StepperDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        let horizontal_stepper = cx.new(|cx| {
            StepperState::new(cx).with_steps(vec![
                StepItem::new("Account").description("Create your account"),
                StepItem::new("Profile").description("Set up your profile"),
                StepItem::new("Settings").description("Configure preferences"),
                StepItem::new("Complete").description("Finish setup"),
            ])
        });

        let vertical_stepper = cx.new(|cx| {
            StepperState::new(cx).with_steps(vec![
                StepItem::new("Select Plan").description("Choose a subscription plan"),
                StepItem::new("Payment").description("Enter payment details"),
                StepItem::new("Review").description("Review your order"),
                StepItem::new("Confirmation").description("Order confirmed"),
            ])
        });

        let small_stepper = cx.new(|cx| {
            StepperState::new(cx).with_steps(vec![
                StepItem::new("Step 1"),
                StepItem::new("Step 2"),
                StepItem::new("Step 3"),
            ])
        });

        let large_stepper = cx.new(|cx| {
            StepperState::new(cx).with_steps(vec![
                StepItem::new("Introduction").description("Get started with the basics"),
                StepItem::new("Configuration").description("Set up your environment"),
                StepItem::new("Deployment").description("Deploy your application"),
            ])
        });

        let non_linear_stepper = cx.new(|cx| {
            StepperState::new(cx).with_linear(false).with_steps(vec![
                StepItem::new("Overview"),
                StepItem::new("Details"),
                StepItem::new("Attachments"),
                StepItem::new("Submit"),
            ])
        });

        let icons_stepper = cx.new(|cx| {
            StepperState::new(cx).with_steps(vec![
                StepItem::new("Cart")
                    .description("Review items")
                    .icon("shopping-cart"),
                StepItem::new("Shipping")
                    .description("Enter address")
                    .icon("truck"),
                StepItem::new("Payment")
                    .description("Pay securely")
                    .icon("credit-card"),
                StepItem::new("Done")
                    .description("Order placed")
                    .icon("check-circle"),
            ])
        });

        let error_stepper = cx.new(|cx| {
            let mut state = StepperState::new(cx).with_steps(vec![
                StepItem::new("Upload").description("Upload your files"),
                StepItem::new("Process").description("Processing files"),
                StepItem::new("Validate")
                    .description("Validation failed")
                    .status(StepStatus::Error),
                StepItem::new("Complete").description("Ready to go"),
            ]);
            state.mark_completed(0, cx);
            state.mark_completed(1, cx);
            state.set_current_step(2, cx);
            state
        });

        Self {
            horizontal_stepper,
            vertical_stepper,
            small_stepper,
            large_stepper,
            non_linear_stepper,
            icons_stepper,
            error_stepper,
        }
    }
}

impl Render for StepperDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        let horizontal_state = self.horizontal_stepper.clone();
        let vertical_state = self.vertical_stepper.clone();
        let small_state = self.small_stepper.clone();
        let large_state = self.large_stepper.clone();
        let non_linear_state = self.non_linear_stepper.clone();
        let icons_state = self.icons_stepper.clone();

        div()
            .size_full()
            .bg(theme.tokens.background)
            .overflow_hidden()
            .child(scrollable_vertical(
                div()
                    .flex()
                    .flex_col()
                    .text_color(theme.tokens.foreground)
                    .p(px(32.0))
                    .gap(px(40.0))
                    .child(
                        VStack::new()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .text_size(px(24.0))
                                    .font_weight(FontWeight::BOLD)
                                    .child("Stepper / Wizard Component Demo"),
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("A versatile stepper component for multi-step workflows"),
                            ),
                    )
                    .child(
                        VStack::new()
                            .gap(px(16.0))
                            .child(
                                div()
                                    .text_size(px(18.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("1. Horizontal Stepper (Default)"),
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Click on steps to navigate. Linear mode requires completing steps in order."),
                            )
                            .child(
                                Stepper::new(self.horizontal_stepper.clone())
                                    .on_step_change(|step, _, _| {
                                        println!("Horizontal stepper changed to step: {}", step);
                                    }),
                            )
                            .child(
                                div()
                                    .flex()
                                    .gap(px(8.0))
                                    .mt(px(16.0))
                                    .child(
                                        Button::new("prev-h", "Previous")
                                            .variant(ButtonVariant::Outline)
                                            .on_click({
                                                let state = horizontal_state.clone();
                                                move |_, _, cx| {
                                                    state.update(cx, |s, cx| {
                                                        s.previous(cx);
                                                    });
                                                }
                                            }),
                                    )
                                    .child(
                                        Button::new("next-h", "Next")
                                            .on_click({
                                                let state = horizontal_state.clone();
                                                move |_, _, cx| {
                                                    state.update(cx, |s, cx| {
                                                        s.next(cx);
                                                    });
                                                }
                                            }),
                                    ),
                            ),
                    )
                    .child(
                        VStack::new()
                            .gap(px(16.0))
                            .child(
                                div()
                                    .text_size(px(18.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("2. Vertical Stepper"),
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Vertical orientation is better for forms with more content per step."),
                            )
                            .child(
                                div()
                                    .p(px(16.0))
                                    .bg(theme.tokens.card)
                                    .rounded(theme.tokens.radius_lg)
                                    .child(
                                        Stepper::new(self.vertical_stepper.clone())
                                            .vertical()
                                            .on_step_change(|step, _, _| {
                                                println!("Vertical stepper changed to step: {}", step);
                                            }),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .gap(px(8.0))
                                    .mt(px(16.0))
                                    .child(
                                        Button::new("prev-v", "Previous")
                                            .variant(ButtonVariant::Outline)
                                            .on_click({
                                                let state = vertical_state.clone();
                                                move |_, _, cx| {
                                                    state.update(cx, |s, cx| {
                                                        s.previous(cx);
                                                    });
                                                }
                                            }),
                                    )
                                    .child(
                                        Button::new("next-v", "Next")
                                            .on_click({
                                                let state = vertical_state.clone();
                                                move |_, _, cx| {
                                                    state.update(cx, |s, cx| {
                                                        s.next(cx);
                                                    });
                                                }
                                            }),
                                    ),
                            ),
                    )
                    .child(
                        VStack::new()
                            .gap(px(16.0))
                            .child(
                                div()
                                    .text_size(px(18.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("3. Size Variants"),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(24.0))
                                    .child(
                                        VStack::new()
                                            .gap(px(8.0))
                                            .child(
                                                div()
                                                    .text_size(px(14.0))
                                                    .text_color(theme.tokens.muted_foreground)
                                                    .child("Small (Sm)"),
                                            )
                                            .child(
                                                Stepper::new(self.small_stepper.clone())
                                                    .size(StepperSize::Sm),
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .gap(px(8.0))
                                                    .child(
                                                        Button::new("prev-sm", "Prev")
                                                            .size(ButtonSize::Sm)
                                                            .variant(ButtonVariant::Outline)
                                                            .on_click({
                                                                let state = small_state.clone();
                                                                move |_, _, cx| {
                                                                    state.update(cx, |s, cx| {
                                                                        s.previous(cx);
                                                                    });
                                                                }
                                                            }),
                                                    )
                                                    .child(
                                                        Button::new("next-sm", "Next")
                                                            .size(ButtonSize::Sm)
                                                            .on_click({
                                                                let state = small_state.clone();
                                                                move |_, _, cx| {
                                                                    state.update(cx, |s, cx| {
                                                                        s.next(cx);
                                                                    });
                                                                }
                                                            }),
                                                    ),
                                            ),
                                    )
                                    .child(
                                        VStack::new()
                                            .gap(px(8.0))
                                            .child(
                                                div()
                                                    .text_size(px(14.0))
                                                    .text_color(theme.tokens.muted_foreground)
                                                    .child("Large (Lg)"),
                                            )
                                            .child(
                                                Stepper::new(self.large_stepper.clone())
                                                    .size(StepperSize::Lg),
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .gap(px(8.0))
                                                    .child(
                                                        Button::new("prev-lg", "Previous")
                                                            .size(ButtonSize::Lg)
                                                            .variant(ButtonVariant::Outline)
                                                            .on_click({
                                                                let state = large_state.clone();
                                                                move |_, _, cx| {
                                                                    state.update(cx, |s, cx| {
                                                                        s.previous(cx);
                                                                    });
                                                                }
                                                            }),
                                                    )
                                                    .child(
                                                        Button::new("next-lg", "Next")
                                                            .size(ButtonSize::Lg)
                                                            .on_click({
                                                                let state = large_state.clone();
                                                                move |_, _, cx| {
                                                                    state.update(cx, |s, cx| {
                                                                        s.next(cx);
                                                                    });
                                                                }
                                                            }),
                                                    ),
                                            ),
                                    ),
                            ),
                    )
                    .child(
                        VStack::new()
                            .gap(px(16.0))
                            .child(
                                div()
                                    .text_size(px(18.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("4. Non-Linear Navigation"),
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("All steps are clickable regardless of completion status."),
                            )
                            .child(
                                Stepper::new(self.non_linear_stepper.clone())
                                    .on_step_change(|step, _, _| {
                                        println!("Non-linear stepper changed to step: {}", step);
                                    }),
                            )
                            .child(
                                div()
                                    .flex()
                                    .gap(px(8.0))
                                    .mt(px(8.0))
                                    .child(
                                        Button::new("mark-complete", "Mark Current Complete")
                                            .variant(ButtonVariant::Secondary)
                                            .on_click({
                                                let state = non_linear_state.clone();
                                                move |_, _, cx| {
                                                    state.update(cx, |s, cx| {
                                                        let current = s.current_step();
                                                        s.mark_completed(current, cx);
                                                    });
                                                }
                                            }),
                                    ),
                            ),
                    )
                    .child(
                        VStack::new()
                            .gap(px(16.0))
                            .child(
                                div()
                                    .text_size(px(18.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("5. Custom Icons"),
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Steps can have custom icons instead of numbers."),
                            )
                            .child(
                                Stepper::new(self.icons_stepper.clone())
                                    .on_step_change(|step, _, _| {
                                        println!("Icons stepper changed to step: {}", step);
                                    }),
                            )
                            .child(
                                div()
                                    .flex()
                                    .gap(px(8.0))
                                    .mt(px(8.0))
                                    .child(
                                        Button::new("prev-icons", "Previous")
                                            .variant(ButtonVariant::Outline)
                                            .on_click({
                                                let state = icons_state.clone();
                                                move |_, _, cx| {
                                                    state.update(cx, |s, cx| {
                                                        s.previous(cx);
                                                    });
                                                }
                                            }),
                                    )
                                    .child(
                                        Button::new("next-icons", "Next")
                                            .on_click({
                                                let state = icons_state.clone();
                                                move |_, _, cx| {
                                                    state.update(cx, |s, cx| {
                                                        s.next(cx);
                                                    });
                                                }
                                            }),
                                    ),
                            ),
                    )
                    .child(
                        VStack::new()
                            .gap(px(16.0))
                            .child(
                                div()
                                    .text_size(px(18.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("6. Error State"),
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Steps can show error states when validation fails."),
                            )
                            .child(
                                Stepper::new(self.error_stepper.clone())
                                    .on_step_change(|step, _, _| {
                                        println!("Error stepper changed to step: {}", step);
                                    }),
                            ),
                    )
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
                                    .child("Stepper component features: Horizontal and vertical orientations, linear and non-linear navigation, size variants (Sm, Md, Lg), custom icons, error states, and step change callbacks."),
                            ),
                    ),
            ))
    }
}
