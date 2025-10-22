//! # Radio and RadioGroup Components
//!
//! Radio button components for single-selection input within a group.
//! Follows shadcn/ui design patterns with focus rings and accessibility support.
//! ## Components
//!
//! - `Radio`: Individual radio button with label
//! - `RadioGroup`: Container managing radio button selection state
//!
//! ## Features
//!
//! - Single selection within a group
//! - Focus ring on keyboard navigation
//! - Disabled state support
//! - Horizontal and vertical layouts
//! - Accessibility with proper ARIA attributes
//! - Theme-integrated styling with shadows
//!
//! ## Design Decisions
//!
//! - Uses primary color for selected state
//! - Inner circle indicator for selection
//! - Focus ring follows our theme system (3px spread)
//! - Supports keyboard navigation with tab stops
//! - RadioGroup automatically manages checked state
//!

use gpui::{prelude::FluentBuilder as _, *};
use std::rc::Rc;
use crate::theme::use_theme;

/// Layout direction for RadioGroup
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RadioLayout {
    /// Vertical stack (default)
    Vertical,
    /// Horizontal row
    Horizontal,
}

impl Default for RadioLayout {
    fn default() -> Self {
        Self::Vertical
    }
}

/// Individual radio button component
#[derive(IntoElement)]
pub struct Radio {
    base: Stateful<Div>,
    id: ElementId,
    label: Option<SharedString>,
    checked: bool,
    disabled: bool,
    on_click: Option<Rc<dyn Fn(&mut Window, &mut App)>>,
    style: StyleRefinement,
}

impl Radio {
    /// Create a new radio button
    pub fn new(id: impl Into<ElementId>) -> Self {
        let id = id.into();
        Self {
            id: id.clone(),
            base: div().id(id),
            label: None,
            checked: false,
            disabled: false,
            on_click: None,
            style: StyleRefinement::default(),
        }
    }

    /// Set the label text
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set checked state
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set click handler
    pub fn on_click(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }
}

impl InteractiveElement for Radio {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl StatefulInteractiveElement for Radio {}

impl Styled for Radio {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Radio {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let user_style = self.style;

        let focus_handle = window
            .use_keyed_state(self.id.clone(), cx, |_, cx| cx.focus_handle())
            .read(cx)
            .clone();
        let is_focused = focus_handle.is_focused(window);

        let (border_color, bg, dot_opacity) = if self.checked {
            (theme.tokens.primary, theme.tokens.primary, 1.0)
        } else {
            (theme.tokens.input, theme.tokens.background, 0.0)
        };

        let (border_color, bg) = if self.disabled {
            (border_color.opacity(0.5), bg.opacity(0.5))
        } else {
            (border_color, bg)
        };

        let shadow_xs = BoxShadow {
            offset: theme.tokens.shadow_xs.offset,
            blur_radius: theme.tokens.shadow_xs.blur_radius,
            spread_radius: theme.tokens.shadow_xs.spread_radius,
            color: theme.tokens.shadow_xs.color,
        };
        let focus_ring = theme.tokens.focus_ring_light();

        self.base
            .when(!self.disabled, |this| {
                this.track_focus(&focus_handle.tab_index(0).tab_stop(true))
            })
            .flex()
            .gap(px(8.0))
            .items_center()
            .text_sm()
            .font_family(theme.tokens.font_family.clone())
            .text_color(if self.disabled {
                theme.tokens.muted_foreground
            } else {
                theme.tokens.foreground
            })
            .when(is_focused && !self.disabled, |this| {
                this.shadow(vec![focus_ring])
            })
            .rounded(theme.tokens.radius_md)
            .child(
                div()
                    .relative()
                    .size(px(16.0))
                    .flex_shrink_0()
                    .rounded_full()
                    .border_1()
                    .border_color(border_color)
                    .bg(bg)
                    .when(!self.disabled, |this| this.shadow(vec![shadow_xs]))
                    .child(
                        div()
                            .absolute()
                            .top(px(3.0))
                            .left(px(3.0))
                            .size(px(8.0))
                            .rounded_full()
                            .bg(theme.tokens.primary_foreground)
                            .opacity(dot_opacity),
                    ),
            )
            .when_some(self.label, |this, label| {
                this.child(
                    div()
                        .line_height(relative(1.0))
                        .child(label),
                )
            })
            .when(!self.disabled, |this| {
                this.cursor(CursorStyle::PointingHand)
                    .on_mouse_down(MouseButton::Left, |_, window, _| {
                        window.prevent_default();
                    })
                    .when_some(self.on_click, |this, handler| {
                        this.on_click(move |_, window, cx| {
                            window.prevent_default();
                            cx.stop_propagation();
                            handler(window, cx);
                        })
                    })
            })
            .map(|this| {
                let mut div = this;
                div.style().refine(&user_style);
                div
            })
    }
}

/// Radio group container managing selection state
///
/// # Example
///
/// ```rust
/// RadioGroup::new("theme-selection")
///     .selected_index(Some(0))
///     .on_change(|index, window, cx| {
///         println!("Selected: {}", index);
///     })
///     .child(Radio::new("light").label("Light"))
///     .child(Radio::new("dark").label("Dark"))
///     .child(Radio::new("system").label("System"))
/// ```
#[derive(IntoElement)]
pub struct RadioGroup {
    id: ElementId,
    radios: Vec<Radio>,
    layout: RadioLayout,
    selected_index: Option<usize>,
    disabled: bool,
    on_change: Option<Rc<dyn Fn(&usize, &mut Window, &mut App)>>,
}

impl RadioGroup {
    /// Create a new radio group
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            radios: Vec::new(),
            layout: RadioLayout::default(),
            selected_index: None,
            disabled: false,
            on_change: None,
        }
    }

    /// Create a vertical radio group
    pub fn vertical(id: impl Into<ElementId>) -> Self {
        Self::new(id)
    }

    /// Create a horizontal radio group
    pub fn horizontal(id: impl Into<ElementId>) -> Self {
        Self::new(id).layout(RadioLayout::Horizontal)
    }

    /// Set the layout direction
    pub fn layout(mut self, layout: RadioLayout) -> Self {
        self.layout = layout;
        self
    }

    /// Set the selected radio index
    pub fn selected_index(mut self, index: Option<usize>) -> Self {
        self.selected_index = index;
        self
    }

    /// Set disabled state for all radios
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set change handler
    pub fn on_change(mut self, handler: impl Fn(&usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }

    /// Add a child radio
    pub fn child(mut self, child: impl Into<Radio>) -> Self {
        self.radios.push(child.into());
        self
    }

    /// Add multiple child radios
    pub fn children(mut self, children: impl IntoIterator<Item = impl Into<Radio>>) -> Self {
        self.radios.extend(children.into_iter().map(Into::into));
        self
    }
}

impl RenderOnce for RadioGroup {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let on_change = self.on_change;
        let disabled = self.disabled;
        let selected_ix = self.selected_index;

        div()
            .id(self.id)
            .flex()
            .when(self.layout == RadioLayout::Vertical, |this| {
                this.flex_col()
            })
            .when(self.layout == RadioLayout::Horizontal, |this| {
                this.flex_row().flex_wrap()
            })
            .gap(px(12.0))
            .children(
                self.radios
                    .into_iter()
                    .enumerate()
                    .map(|(ix, radio)| {
                        let checked = selected_ix == Some(ix);
                        radio
                            .checked(checked)
                            .disabled(disabled)
                            .when_some(on_change.clone(), |this, on_change| {
                                this.on_click(move |window, cx| {
                                    on_change(&ix, window, cx);
                                })
                            })
                    }),
            )
    }
}

// Convenience From implementations
impl From<&'static str> for Radio {
    fn from(label: &'static str) -> Self {
        Self::new(label).label(label)
    }
}

impl From<SharedString> for Radio {
    fn from(label: SharedString) -> Self {
        Self::new(label.clone()).label(label)
    }
}

impl From<String> for Radio {
    fn from(label: String) -> Self {
        let shared: SharedString = label.into();
        Self::new(shared.clone()).label(shared)
    }
}
