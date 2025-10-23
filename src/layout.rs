//! Layout components - High-level layout abstractions for common UI patterns.

use gpui::*;
use crate::components::scrollbar::{ScrollbarState, ScrollbarAxis, Scrollbar};
use std::panic::Location;
use std::sync::atomic::AtomicUsize;

static SCROLL_CONTAINER_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Align {
    Start,
    Center,
    End,
    Stretch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Justify {
    Start,
    Center,
    End,
    Between,
    Around,
    Evenly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlowDirection {
    Horizontal,
    Vertical,
}

pub struct VStack {
    base: Div,
    spacing: Option<Pixels>,
    align: Option<Align>,
}

impl VStack {
    pub fn new() -> Self {
        Self {
            base: div().flex().flex_col(),
            spacing: None,
            align: None,
        }
    }

    pub fn spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        self.spacing = Some(spacing.into());
        self
    }

    pub fn gap(self, gap: impl Into<Pixels>) -> Self {
        self.spacing(gap)
    }

    pub fn align(mut self, align: Align) -> Self {
        self.align = Some(align);
        self
    }

    pub fn fill(mut self) -> Self {
        self.base = self.base.size_full();
        self
    }

    pub fn fill_width(mut self) -> Self {
        self.base = self.base.w_full();
        self
    }

    pub fn fill_height(mut self) -> Self {
        self.base = self.base.h_full();
        self
    }

    pub fn grow(mut self) -> Self {
        self.base = self.base.flex_1();
        self
    }

    pub fn padding(mut self, padding: impl Into<Pixels>) -> Self {
        self.base = self.base.p(padding.into());
        self
    }

    pub fn items_center(mut self) -> Self {
        self.align = Some(Align::Center);
        self
    }
}

impl ParentElement for VStack {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.base.extend(elements);
    }
}

impl Styled for VStack {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl InteractiveElement for VStack {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl StatefulInteractiveElement for VStack {}

impl IntoElement for VStack {
    type Element = Div;

    fn into_element(mut self) -> Self::Element {
        if let Some(spacing) = self.spacing {
            self.base = self.base.gap(spacing);
        }

        if let Some(align) = self.align {
            self.base = match align {
                Align::Start => self.base.items_start(),
                Align::Center => self.base.items_center(),
                Align::End => self.base.items_end(),
                Align::Stretch => self.base,
            };
        }

        self.base
    }
}

pub struct HStack {
    base: Div,
    spacing: Option<Pixels>,
    align: Option<Align>,
    justify: Option<Justify>,
}

impl HStack {
    pub fn new() -> Self {
        Self {
            base: div().flex().flex_row(),
            spacing: None,
            align: None,
            justify: None,
        }
    }

    pub fn spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        self.spacing = Some(spacing.into());
        self
    }

    pub fn gap(self, gap: impl Into<Pixels>) -> Self {
        self.spacing(gap)
    }

    pub fn align(mut self, align: Align) -> Self {
        self.align = Some(align);
        self
    }

    pub fn justify(mut self, justify: Justify) -> Self {
        self.justify = Some(justify);
        self
    }

    pub fn fill(mut self) -> Self {
        self.base = self.base.size_full();
        self
    }

    pub fn fill_width(mut self) -> Self {
        self.base = self.base.w_full();
        self
    }

    pub fn fill_height(mut self) -> Self {
        self.base = self.base.h_full();
        self
    }

    pub fn grow(mut self) -> Self {
        self.base = self.base.flex_1();
        self
    }

    pub fn padding(mut self, padding: impl Into<Pixels>) -> Self {
        self.base = self.base.p(padding.into());
        self
    }

    pub fn items_center(mut self) -> Self {
        self.align = Some(Align::Center);
        self
    }

    pub fn space_between(mut self) -> Self {
        self.justify = Some(Justify::Between);
        self
    }
}

impl ParentElement for HStack {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.base.extend(elements);
    }
}

impl Styled for HStack {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl InteractiveElement for HStack {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl StatefulInteractiveElement for HStack {}

impl IntoElement for HStack {
    type Element = Div;

    fn into_element(mut self) -> Self::Element {
        if let Some(spacing) = self.spacing {
            self.base = self.base.gap(spacing);
        }

        if let Some(align) = self.align {
            self.base = match align {
                Align::Start => self.base.items_start(),
                Align::Center => self.base.items_center(),
                Align::End => self.base.items_end(),
                Align::Stretch => self.base,
            };
        }

        if let Some(justify) = self.justify {
            self.base = match justify {
                Justify::Start => self.base.justify_start(),
                Justify::Center => self.base.justify_center(),
                Justify::End => self.base.justify_end(),
                Justify::Between => self.base.justify_between(),
                Justify::Around => self.base.justify_around(),
                Justify::Evenly => self.base.justify_around(),
            };
        }

        self.base
    }
}

pub struct Flow {
    base: Div,
    direction: FlowDirection,
    spacing: Option<Pixels>,
    align: Option<Align>,
}

impl Flow {
    pub fn new() -> Self {
        Self {
            base: div().flex().flex_wrap(),
            direction: FlowDirection::Horizontal,
            spacing: None,
            align: None,
        }
    }

    pub fn direction(mut self, direction: FlowDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        self.spacing = Some(spacing.into());
        self
    }

    pub fn align(mut self, align: Align) -> Self {
        self.align = Some(align);
        self
    }
}

impl ParentElement for Flow {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.base.extend(elements);
    }
}

impl Styled for Flow {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl InteractiveElement for Flow {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl StatefulInteractiveElement for Flow {}

impl IntoElement for Flow {
    type Element = Div;

    fn into_element(mut self) -> Self::Element {
        self.base = match self.direction {
            FlowDirection::Horizontal => self.base.flex_row(),
            FlowDirection::Vertical => self.base.flex_col(),
        };

        if let Some(spacing) = self.spacing {
            self.base = self.base.gap(spacing);
        }

        if let Some(align) = self.align {
            self.base = match align {
                Align::Start => self.base.items_start(),
                Align::Center => self.base.items_center(),
                Align::End => self.base.items_end(),
                Align::Stretch => self.base,
            };
        }

        self.base
    }
}

pub struct Grid {
    base: Div,
    columns: usize,
    gap: Option<Pixels>,
    grid_children: Vec<AnyElement>,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            base: div().flex().flex_col(),
            columns: 1,
            gap: None,
            grid_children: vec![],
        }
    }

    pub fn columns(mut self, columns: usize) -> Self {
        self.columns = columns.max(1);
        self
    }

    pub fn gap(mut self, gap: impl Into<Pixels>) -> Self {
        self.gap = Some(gap.into());
        self
    }
}

impl ParentElement for Grid {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.grid_children.extend(elements);
    }
}

impl Styled for Grid {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl InteractiveElement for Grid {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl StatefulInteractiveElement for Grid {}

impl IntoElement for Grid {
    type Element = Div;

    fn into_element(mut self) -> Self::Element {
        if let Some(gap) = self.gap {
            self.base = self.base.gap(gap);
        }

        let total_children = self.grid_children.len();
        let mut rows = vec![];
        let mut current_row = vec![];

        for (i, child) in self.grid_children.into_iter().enumerate() {
            current_row.push(child);
            if (i + 1) % self.columns == 0 || i == total_children - 1 {
                rows.push(current_row);
                current_row = vec![];
            }
        }

        for row_children in rows {
            let mut row = div().flex().flex_row().w_full();

            if let Some(gap) = self.gap {
                row = row.gap(gap);
            }

            for child in row_children {
                row = row.child(div().flex_1().child(child));
            }

            self.base = self.base.child(row);
        }

        self.base
    }
}

pub struct Cluster {
    base: Div,
    spacing: Option<Pixels>,
    align: Option<Align>,
}

impl Cluster {
    pub fn new() -> Self {
        Self {
            base: div().flex().flex_row(),
            spacing: None,
            align: None,
        }
    }

    pub fn spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        self.spacing = Some(spacing.into());
        self
    }

    pub fn align(mut self, align: Align) -> Self {
        self.align = Some(align);
        self
    }
}

impl ParentElement for Cluster {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.base.extend(elements);
    }
}

impl Styled for Cluster {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl InteractiveElement for Cluster {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl StatefulInteractiveElement for Cluster {}

impl IntoElement for Cluster {
    type Element = Div;

    fn into_element(mut self) -> Self::Element {
        if let Some(spacing) = self.spacing {
            self.base = self.base.gap(spacing);
        }

        if let Some(align) = self.align {
            self.base = match align {
                Align::Start => self.base.items_start(),
                Align::Center => self.base.items_center(),
                Align::End => self.base.items_end(),
                Align::Stretch => self.base,
            };
        }

        self.base
    }
}

pub struct Spacer {
    size: Option<Pixels>,
}

impl Spacer {
    pub fn new() -> Self {
        Self { size: None }
    }

    pub fn fixed(size: impl Into<Pixels>) -> Self {
        Self { size: Some(size.into()) }
    }
}

impl IntoElement for Spacer {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        if let Some(size) = self.size {
            div().size(size)
        } else {
            div().flex_1()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollDirection {
    Vertical,
    Horizontal,
    Both,
}

pub struct ScrollContainer {
    base: Div,
    direction: ScrollDirection,
    scroll_handle: Option<ScrollHandle>,
    auto_size: bool,
    smooth_scroll: bool,
    custom_id: Option<ElementId>,
    auto_id: ElementId,
    show_scrollbar: bool,
    horizontal_top: bool,
    scrollbar_state: ScrollbarState,
}

impl ScrollContainer {
    #[track_caller]
    pub fn new(direction: ScrollDirection) -> Self {
        let location = Location::caller();
        let auto_id = ElementId::Name(
            format!("scroll-container:{}:{}:{}", location.file(), location.line(), location.column())
                .into(),
        );

        Self {
            base: div(),
            direction,
            scroll_handle: None,
            auto_size: false,
            smooth_scroll: true,
            custom_id: None,
            auto_id,
            show_scrollbar: false,
            horizontal_top: false,
            scrollbar_state: ScrollbarState::default(),
        }
    }

    #[track_caller]
    pub fn vertical() -> Self {
        Self::new(ScrollDirection::Vertical)
    }

    #[track_caller]
    pub fn horizontal() -> Self {
        Self::new(ScrollDirection::Horizontal)
    }

    #[track_caller]
    pub fn both() -> Self {
        Self::new(ScrollDirection::Both)
    }

    pub fn track_scroll(mut self, handle: &ScrollHandle) -> Self {
        self.scroll_handle = Some(handle.clone());
        self
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.custom_id = Some(id.into());
        self
    }

    pub fn no_auto_size(mut self) -> Self {
        self.auto_size = false;
        self
    }

    /// Smooth scrolling reduces vibrating effect during scroll by allowing concurrent scroll in both axes
    pub fn smooth(mut self) -> Self {
        self.smooth_scroll = true;
        self
    }

    pub fn no_smooth(mut self) -> Self {
        self.smooth_scroll = false;
        self
    }

    pub fn flex_grow(mut self) -> Self {
        self.base = self.base.flex_1();
        self
    }

    pub fn with_scrollbar(mut self) -> Self {
        self.show_scrollbar = true;
        self
    }

    pub fn horizontal_bar_top(mut self) -> Self {
        self.horizontal_top = true;
        self
    }

    pub fn horizontal_bar_bottom(mut self) -> Self {
        self.horizontal_top = false;
        self
    }
}

impl ParentElement for ScrollContainer {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.base.extend(elements);
    }
}

impl Styled for ScrollContainer {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl InteractiveElement for ScrollContainer {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl StatefulInteractiveElement for ScrollContainer {}

impl IntoElement for ScrollContainer {
    type Element = Stateful<Div>;

    fn into_element(self) -> Self::Element {
        let id_to_use = self.custom_id.clone().unwrap_or(self.auto_id.clone());
        let handle = self.scroll_handle.clone().unwrap_or_else(ScrollHandle::new);

        if !self.show_scrollbar {
            let mut scrollable = self.base.id(id_to_use);

            if let Some(handle) = &self.scroll_handle {
                scrollable = scrollable.track_scroll(handle);
            }

            scrollable = match self.direction {
                ScrollDirection::Vertical => scrollable.overflow_y_scroll(),
                ScrollDirection::Horizontal => scrollable.overflow_x_scroll(),
                ScrollDirection::Both => scrollable.overflow_scroll(),
            };

            return scrollable;
        } else {
            let scrollbar_state = self.scrollbar_state.clone();
            scrollbar_state.init_visible();

            let axis = match self.direction {
                ScrollDirection::Vertical => ScrollbarAxis::Vertical,
                ScrollDirection::Horizontal => ScrollbarAxis::Horizontal,
                ScrollDirection::Both => ScrollbarAxis::Both,
            };
            let mut scrollbar = Scrollbar::both(&scrollbar_state, &handle).axis(axis);
            if self.horizontal_top {
                scrollbar = scrollbar.horizontal_top();
            }

            let mut scrollable = self.base
                .id(id_to_use.clone())
                .track_scroll(&handle);

            scrollable = match self.direction {
                ScrollDirection::Vertical => scrollable.overflow_y_scroll(),
                ScrollDirection::Horizontal => scrollable.overflow_x_scroll(),
                ScrollDirection::Both => scrollable.overflow_scroll(),
            };

            scrollable = scrollable.relative().size_full();

            let wrapper = div()
                .id(ElementId::Name(format!("{}-wrapper", id_to_use).into()))
                .relative()
                .size_full()
                .child(scrollable)
                .child(scrollbar);

            wrapper
        }
    }
}

pub struct Panel {
    base: Div,
}

impl Panel {
    pub fn new() -> Self {
        Self {
            base: div(),
        }
    }

    pub fn card(mut self) -> Self {
        self.base = self.base
            .border_1()
            .rounded(px(8.0))
            .p(px(16.0));
        self
    }

    pub fn elevated(mut self) -> Self {
        self.base = self.base
            .border_1()
            .rounded(px(8.0));
        self
    }

    pub fn section(mut self) -> Self {
        self.base = self.base
            .border_b_1()
            .p(px(12.0));
        self
    }

    pub fn border(mut self) -> Self {
        self.base = self.base.border_1();
        self
    }

    pub fn rounded(mut self) -> Self {
        self.base = self.base.rounded(px(8.0));
        self
    }

    pub fn padded(mut self) -> Self {
        self.base = self.base.p(px(16.0));
        self
    }
}

impl ParentElement for Panel {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.base.extend(elements);
    }
}

impl Styled for Panel {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl InteractiveElement for Panel {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl StatefulInteractiveElement for Panel {}

impl IntoElement for Panel {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        self.base
    }
}

pub struct Container {
    base: Div,
    max_width: Option<Pixels>,
    centered: bool,
}

impl Container {
    pub fn new() -> Self {
        Self {
            base: div().w_full(),
            max_width: None,
            centered: false,
        }
    }

    pub fn max_w(mut self, width: impl Into<Pixels>) -> Self {
        self.max_width = Some(width.into());
        self
    }

    pub fn centered(mut self) -> Self {
        self.centered = true;
        self
    }

    pub fn sm() -> Self {
        Self::new()
            .max_w(px(640.0))
            .centered()
    }

    pub fn md() -> Self {
        Self::new()
            .max_w(px(768.0))
            .centered()
    }

    pub fn lg() -> Self {
        Self::new()
            .max_w(px(1024.0))
            .centered()
    }

    pub fn xl() -> Self {
        Self::new()
            .max_w(px(1280.0))
            .centered()
    }

    pub fn xxl() -> Self {
        Self::new()
            .max_w(px(1536.0))
            .centered()
    }
}

impl ParentElement for Container {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.base.extend(elements);
    }
}

impl Styled for Container {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl InteractiveElement for Container {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl StatefulInteractiveElement for Container {}

impl IntoElement for Container {
    type Element = Div;

    fn into_element(mut self) -> Self::Element {
        if let Some(max_width) = self.max_width {
            self.base = self.base.max_w(max_width);
        }

        if self.centered {
            self.base = self.base.mx_auto();
        }

        self.base
    }
}

pub struct ScrollList {
    scroll_container: ScrollContainer,
    stack: VStack,
}

impl ScrollList {
    pub fn new() -> Self {
        Self {
            scroll_container: ScrollContainer::vertical().flex_grow(),
            stack: VStack::new(),
        }
    }

    pub fn spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        self.stack = self.stack.spacing(spacing);
        self
    }

    pub fn align(mut self, align: Align) -> Self {
        self.stack = self.stack.align(align);
        self
    }

    pub fn track_scroll(mut self, handle: &ScrollHandle) -> Self {
        self.scroll_container = self.scroll_container.track_scroll(handle);
        self
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.scroll_container = self.scroll_container.id(id);
        self
    }

    pub fn no_flex_grow(mut self) -> Self {
        let id = SharedString::from(format!("scroll-list-{}", SCROLL_CONTAINER_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed)));
        self.scroll_container = ScrollContainer::vertical().id(id);
        self
    }
}

impl ParentElement for ScrollList {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.stack.extend(elements);
    }
}

impl Styled for ScrollList {
    fn style(&mut self) -> &mut StyleRefinement {
        self.scroll_container.style()
    }
}

impl InteractiveElement for ScrollList {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.scroll_container.interactivity()
    }
}

impl StatefulInteractiveElement for ScrollList {}

impl IntoElement for ScrollList {
    type Element = Stateful<Div>;

    fn into_element(self) -> Self::Element {
        let mut scroll_container = self.scroll_container;
        scroll_container = scroll_container.child(self.stack);
        scroll_container.into_element()
    }
}
