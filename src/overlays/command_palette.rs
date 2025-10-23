//! Command palette component with fuzzy search.

use gpui::{prelude::FluentBuilder as _, InteractiveElement, *};
use std::rc::Rc;
use crate::{
    theme::use_theme,
    components::{
        text::{body, caption, label_small},
        icon::Icon,
        icon_source::IconSource,
        input::Input,
        input_state::InputState,
        scrollable::scrollable_vertical,
    },
};

actions!(command_palette, [NavigateUp, NavigateDown, SelectCommand, CloseCommand]);

#[derive(Clone)]
pub struct Command {
    pub id: SharedString,
    pub name: SharedString,
    pub description: Option<SharedString>,
    pub icon: Option<IconSource>,
    pub category: Option<SharedString>,
    pub shortcut: Option<SharedString>,
    pub on_select: Option<Rc<dyn Fn(&mut Window, &mut App)>>,
    search_text: String,
}

impl Command {
    pub fn new(id: impl Into<SharedString>, name: impl Into<SharedString>) -> Self {
        let id = id.into();
        let name = name.into();
        let search_text = name.to_string().to_lowercase();

        Self {
            id,
            name,
            description: None,
            icon: None,
            category: None,
            shortcut: None,
            on_select: None,
            search_text,
        }
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        let desc = description.into();
        self.search_text = format!("{} {}", self.name, desc).to_lowercase();
        self.description = Some(desc);
        self
    }

    pub fn icon(mut self, icon: impl Into<IconSource>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn category(mut self, category: impl Into<SharedString>) -> Self {
        self.category = Some(category.into());
        self
    }

    pub fn shortcut(mut self, shortcut: impl Into<SharedString>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    pub fn on_select<F>(mut self, handler: F) -> Self
    where
        F: Fn(&mut Window, &mut App) + 'static,
    {
        self.on_select = Some(Rc::new(handler));
        self
    }

    pub fn matches(&self, query: &str) -> bool {
        if query.is_empty() {
            return true;
        }

        let query = query.to_lowercase();
        self.search_text.contains(&query)
    }

    pub fn match_score(&self, query: &str) -> i32 {
        if query.is_empty() {
            return 0;
        }

        let query = query.to_lowercase();
        let name_lower = self.name.to_string().to_lowercase();

        if name_lower == query {
            return 1000;
        }

        if name_lower.starts_with(&query) {
            return 500;
        }

        if name_lower.contains(&query) {
            return 100;
        }

        if self.search_text.contains(&query) {
            return 50;
        }

        0
    }
}

pub struct CommandPaletteState {
    commands: Vec<Command>,
    search_query: String,
    filtered_commands: Vec<Command>,
    selected_index: usize,
    recent_commands: Vec<SharedString>,
}

impl CommandPaletteState {
    pub fn new(commands: Vec<Command>) -> Self {
        let filtered_commands = commands.clone();

        Self {
            commands,
            search_query: String::new(),
            filtered_commands,
            selected_index: 0,
            recent_commands: Vec::new(),
        }
    }

    pub fn update_search(&mut self, query: String) {
        self.search_query = query.clone();

        if query.is_empty() {
            self.filtered_commands = self.commands.clone();
        } else {
            let mut matches: Vec<(Command, i32)> = self
                .commands
                .iter()
                .filter(|cmd| cmd.matches(&query))
                .map(|cmd| (cmd.clone(), cmd.match_score(&query)))
                .collect();

            matches.sort_by(|a, b| b.1.cmp(&a.1));
            self.filtered_commands = matches.into_iter().map(|(cmd, _)| cmd).collect();
        }

        self.selected_index = 0;
    }

    pub fn select_previous(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub fn select_next(&mut self) {
        if self.selected_index < self.filtered_commands.len().saturating_sub(1) {
            self.selected_index += 1;
        }
    }

    pub fn execute_selected(&mut self, window: &mut Window, cx: &mut App) -> bool {
        if let Some(command) = self.filtered_commands.get(self.selected_index) {
            if let Some(handler) = &command.on_select {
                handler(window, cx);
                self.recent_commands.push(command.id.clone());
                if self.recent_commands.len() > 10 {
                    self.recent_commands.remove(0);
                }
                return true;
            }
        }
        false
    }

    pub fn filtered_commands(&self) -> &[Command] {
        &self.filtered_commands
    }

    pub fn selected_index(&self) -> usize {
        self.selected_index
    }
}

pub struct CommandPalette {
    state: Entity<CommandPaletteState>,
    search_input: Entity<InputState>,
    on_close: Option<Rc<dyn Fn(&mut Window, &mut App)>>,
    focus_handle: FocusHandle,
    style: StyleRefinement,
}

impl CommandPalette {
    pub fn new(_window: &mut Window, cx: &mut Context<Self>, commands: Vec<Command>) -> Self {
        let state = cx.new(|_| CommandPaletteState::new(commands));
        let search_input = cx.new(|cx| {
            InputState::new(cx).placeholder("Type a command or search...")
        });
        let focus_handle = cx.focus_handle();

        cx.subscribe(&search_input, |this, _input, event, cx| {
            use crate::components::input_state::InputEvent;
            match event {
                InputEvent::Change => {
                    let query = this.search_input.read(cx).content().to_string();
                    this.state.update(cx, |state, _cx| {
                        state.update_search(query);
                    });
                    cx.notify();
                }
                _ => {}
            }
        }).detach();

        Self {
            state,
            search_input,
            on_close: None,
            focus_handle,
            style: StyleRefinement::default(),
        }
    }

    pub fn on_close<F>(mut self, handler: F) -> Self
    where
        F: Fn(&mut Window, &mut App) + 'static,
    {
        self.on_close = Some(Rc::new(handler));
        self
    }
}

impl Styled for CommandPalette {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl Focusable for CommandPalette {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for CommandPalette {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();
        let state = self.state.read(cx);
        let filtered = state.filtered_commands();
        let selected_idx = state.selected_index();
        let user_style = self.style.clone();

        div()
            .absolute()
            .inset_0()
            .flex()
            .items_center()
            .justify_center()
            .bg(gpui::rgba(0x00000088))
            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, window, cx| {
                if let Some(handler) = &this.on_close {
                    handler(window, cx);
                }
            }))
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(|this, _: &NavigateUp, _window, cx| {
                this.state.update(cx, |state, _cx| {
                    state.select_previous();
                });
                cx.notify();
            }))
            .on_action(cx.listener(|this, _: &NavigateDown, _window, cx| {
                this.state.update(cx, |state, _cx| {
                    state.select_next();
                });
                cx.notify();
            }))
            .on_action(cx.listener(|this, _: &SelectCommand, window, cx| {
                let executed = this.state.update(cx, |state, app_cx| {
                    state.execute_selected(window, app_cx)
                });
                if executed {
                    if let Some(handler) = &this.on_close {
                        handler(window, cx);
                    }
                }
            }))
            .on_action(cx.listener(|this, _: &CloseCommand, window, cx| {
                if let Some(handler) = &this.on_close {
                    handler(window, cx);
                }
            }))
            .child(
                div()
                    .w(px(600.0))
                    .max_h(px(500.0))
                    .flex()
                    .flex_col()
                    .bg(theme.tokens.card)
                    .border_1()
                    .border_color(theme.tokens.border)
                    .rounded(theme.tokens.radius_lg)
                    .shadow_lg()
                    .overflow_hidden()
                    .on_mouse_down(MouseButton::Left, |_event, _window, _cx| {})
                    .map(|this| {
                        let mut div = this;
                        div.style().refine(&user_style);
                        div
                    })
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .px(px(16.0))
                            .py(px(12.0))
                            .border_b_1()
                            .border_color(theme.tokens.border)
                            .child(
                                Input::new(&self.search_input)
                                    .placeholder("Type a command or search...")
                            )
                    )
                    .child(
                        div()
                            .flex_1()
                            .overflow_hidden()
                            .child(
                                scrollable_vertical(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .p(px(8.0))
                                        .children(
                                            filtered.is_empty().then(|| {
                                                div()
                                                    .flex()
                                                    .items_center()
                                                    .justify_center()
                                                    .h(px(200.0))
                                                    .child(caption("No commands found").color(theme.tokens.muted_foreground))
                                                    .into_any_element()
                                            })
                                        )
                                        .children(filtered.iter().enumerate().map(|(idx, command)| {
                                            let is_selected = idx == selected_idx;
                                            render_command_item(command.clone(), is_selected, cx)
                                        }))
                                )
                            )
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .px(px(16.0))
                            .py(px(8.0))
                            .border_t_1()
                            .border_color(theme.tokens.border)
                            .bg(theme.tokens.muted.opacity(0.3))
                            .child(
                                div()
                                    .flex()
                                    .gap(px(16.0))
                                    .child(label_small("↑↓ Navigate").color(theme.tokens.muted_foreground))
                                    .child(label_small("↵ Select").color(theme.tokens.muted_foreground))
                                    .child(label_small("Esc Close").color(theme.tokens.muted_foreground))
                            )
                    )
            )
    }
}

fn render_command_item(command: Command, selected: bool, _cx: &App) -> impl IntoElement {
    let theme = use_theme();

    div()
        .flex()
        .items_center()
        .gap(px(12.0))
        .px(px(12.0))
        .py(px(10.0))
        .rounded(theme.tokens.radius_sm)
        .cursor(CursorStyle::PointingHand)
        .when(selected, |div| {
            div.bg(theme.tokens.accent)
        })
        .when(!selected, |div| {
            div.hover(|style| style.bg(theme.tokens.muted))
        })
        .when_some(command.on_select, |div, handler| {
            div.on_mouse_down(MouseButton::Left, move |_event, window, cx| {
                handler(window, cx);
            })
        })
        .when_some(command.icon, |div, icon| {
            div.child(
                Icon::new(icon)
                    .size(px(18.0))
                    .color(if selected {
                        theme.tokens.accent_foreground
                    } else {
                        theme.tokens.foreground
                    })
            )
        })
        .child(
            div()
                .flex_1()
                .flex()
                .flex_col()
                .gap(px(2.0))
                .child(
                    body(command.name).color(if selected {
                        theme.tokens.accent_foreground
                    } else {
                        theme.tokens.foreground
                    })
                )
                .when_some(command.description, |div, desc| {
                    div.child(
                        caption(desc).color(if selected {
                            theme.tokens.accent_foreground.opacity(0.8)
                        } else {
                            theme.tokens.muted_foreground
                        })
                    )
                })
        )
        .children(
            command.shortcut.map(|shortcut| {
                div()
                    .px(px(8.0))
                    .py(px(4.0))
                    .rounded(theme.tokens.radius_sm)
                    .bg(if selected {
                        theme.tokens.accent_foreground.opacity(0.2)
                    } else {
                        theme.tokens.muted
                    })
                    .child(
                        caption(shortcut).color(if selected {
                            theme.tokens.accent_foreground
                        } else {
                            theme.tokens.muted_foreground
                        })
                    )
                    .into_any_element()
            })
        )
}
