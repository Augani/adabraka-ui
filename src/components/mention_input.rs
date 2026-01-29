use crate::components::avatar::{Avatar, AvatarSize};
use crate::components::scrollable::scrollable_vertical;
use crate::theme::use_theme;
use gpui::{prelude::*, *};
use std::ops::Range;
use std::rc::Rc;
use unicode_segmentation::*;

actions!(
    mention_input,
    [
        MentionUp,
        MentionDown,
        MentionConfirm,
        MentionCancel,
        MentionBackspace,
        MentionDelete,
        MentionLeft,
        MentionRight,
        MentionHome,
        MentionEnd,
        MentionSelectAll,
        MentionCopy,
        MentionCut,
        MentionPaste,
        MentionEnter,
    ]
);

const DROPDOWN_MARGIN: Pixels = px(4.0);

#[derive(Clone, Debug, PartialEq)]
pub struct MentionItem {
    pub id: SharedString,
    pub name: SharedString,
    pub avatar_url: Option<SharedString>,
}

impl MentionItem {
    pub fn new(id: impl Into<SharedString>, name: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            avatar_url: None,
        }
    }

    pub fn with_avatar(mut self, url: impl Into<SharedString>) -> Self {
        self.avatar_url = Some(url.into());
        self
    }
}

#[derive(Clone, Debug)]
pub struct Mention {
    pub item: MentionItem,
    pub range: Range<usize>,
}

#[derive(Clone, Debug)]
pub enum MentionInputEvent {
    Change,
    MentionSelected(MentionItem),
    Enter,
    Focus,
    Blur,
}

pub struct MentionInputState {
    focus_handle: FocusHandle,
    pub content: String,
    pub placeholder: SharedString,
    pub disabled: bool,
    selected_range: Range<usize>,
    selection_reversed: bool,
    last_layout: Option<gpui::ShapedLine>,
    last_bounds: Option<Bounds<Pixels>>,
    is_selecting: bool,

    pub trigger_char: char,
    pub mentions: Vec<Mention>,
    pub dropdown_open: bool,
    pub dropdown_filter: String,
    pub dropdown_index: usize,
    trigger_start: Option<usize>,
}

impl EventEmitter<MentionInputEvent> for MentionInputState {}

impl MentionInputState {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            content: String::new(),
            placeholder: "Type @ to mention someone...".into(),
            disabled: false,
            selected_range: 0..0,
            selection_reversed: false,
            last_layout: None,
            last_bounds: None,
            is_selecting: false,

            trigger_char: '@',
            mentions: Vec::new(),
            dropdown_open: false,
            dropdown_filter: String::new(),
            dropdown_index: 0,
            trigger_start: None,
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn mentions(&self) -> &[Mention] {
        &self.mentions
    }

    pub fn set_value(
        &mut self,
        value: impl Into<String>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.content = value.into();
        self.mentions.clear();
        self.selected_range = self.content.len()..self.content.len();
        cx.emit(MentionInputEvent::Change);
        cx.notify();
    }

    pub fn clear(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.content.clear();
        self.mentions.clear();
        self.selected_range = 0..0;
        self.close_dropdown(cx);
        cx.emit(MentionInputEvent::Change);
        cx.notify();
    }

    fn open_dropdown(&mut self, start: usize, cx: &mut Context<Self>) {
        self.dropdown_open = true;
        self.trigger_start = Some(start);
        self.dropdown_filter.clear();
        self.dropdown_index = 0;
        cx.notify();
    }

    fn close_dropdown(&mut self, cx: &mut Context<Self>) {
        self.dropdown_open = false;
        self.trigger_start = None;
        self.dropdown_filter.clear();
        self.dropdown_index = 0;
        cx.notify();
    }

    fn insert_mention(&mut self, item: MentionItem, cx: &mut Context<Self>) {
        let Some(trigger_start) = self.trigger_start else {
            return;
        };

        let mention_text = format!("@{} ", item.name);
        let end_pos = self.selected_range.end;

        let before = &self.content[..trigger_start];
        let after = &self.content[end_pos..];
        self.content = format!("{}{}{}", before, mention_text, after);

        let mention_range = trigger_start..trigger_start + mention_text.len() - 1;
        self.mentions.push(Mention {
            item: item.clone(),
            range: mention_range,
        });

        self.selected_range =
            trigger_start + mention_text.len()..trigger_start + mention_text.len();
        self.close_dropdown(cx);

        cx.emit(MentionInputEvent::MentionSelected(item));
        cx.emit(MentionInputEvent::Change);
        cx.notify();
    }

    fn cursor_offset(&self) -> usize {
        if self.selection_reversed {
            self.selected_range.start
        } else {
            self.selected_range.end
        }
    }

    fn move_to(&mut self, offset: usize, cx: &mut Context<Self>) {
        self.selected_range = offset..offset;
        cx.notify()
    }

    fn select_to(&mut self, offset: usize, cx: &mut Context<Self>) {
        if self.selection_reversed {
            self.selected_range.start = offset
        } else {
            self.selected_range.end = offset
        };
        if self.selected_range.end < self.selected_range.start {
            self.selection_reversed = !self.selection_reversed;
            self.selected_range = self.selected_range.end..self.selected_range.start;
        }
        cx.notify()
    }

    fn previous_boundary(&self, offset: usize) -> usize {
        self.content
            .grapheme_indices(true)
            .rev()
            .find_map(|(idx, _)| (idx < offset).then_some(idx))
            .unwrap_or(0)
    }

    fn next_boundary(&self, offset: usize) -> usize {
        self.content
            .grapheme_indices(true)
            .find_map(|(idx, _)| (idx > offset).then_some(idx))
            .unwrap_or(self.content.len())
    }

    fn offset_from_utf16(&self, offset: usize) -> usize {
        let mut utf8_offset = 0;
        let mut utf16_count = 0;
        for ch in self.content.chars() {
            if utf16_count >= offset {
                break;
            }
            utf16_count += ch.len_utf16();
            utf8_offset += ch.len_utf8();
        }
        utf8_offset
    }

    fn offset_to_utf16(&self, offset: usize) -> usize {
        let mut utf16_offset = 0;
        let mut utf8_count = 0;
        for ch in self.content.chars() {
            if utf8_count >= offset {
                break;
            }
            utf8_count += ch.len_utf8();
            utf16_offset += ch.len_utf16();
        }
        utf16_offset
    }

    fn range_to_utf16(&self, range: &Range<usize>) -> Range<usize> {
        self.offset_to_utf16(range.start)..self.offset_to_utf16(range.end)
    }

    fn range_from_utf16(&self, range_utf16: &Range<usize>) -> Range<usize> {
        self.offset_from_utf16(range_utf16.start)..self.offset_from_utf16(range_utf16.end)
    }

    fn index_for_mouse_position(&self, position: Point<Pixels>) -> usize {
        if self.content.is_empty() {
            return 0;
        }
        let (Some(bounds), Some(line)) = (self.last_bounds.as_ref(), self.last_layout.as_ref())
        else {
            return 0;
        };
        if position.y < bounds.top() {
            return 0;
        }
        if position.y > bounds.bottom() {
            return self.content.len();
        }
        line.closest_index_for_x(position.x - bounds.left())
    }

    fn check_for_trigger(&mut self, cx: &mut Context<Self>) {
        let cursor = self.cursor_offset();
        if cursor == 0 {
            if self.dropdown_open {
                self.close_dropdown(cx);
            }
            return;
        }

        let trigger_char = self.trigger_char;
        let content = self.content.clone();
        let text_before = &content[..cursor];

        if let Some(trigger_pos) = text_before.rfind(trigger_char) {
            let between = text_before[trigger_pos + 1..].to_string();
            let has_space = between.contains(' ');
            let at_word_boundary = trigger_pos == 0
                || text_before
                    .chars()
                    .nth(trigger_pos.saturating_sub(1))
                    .map_or(true, |c| c.is_whitespace());

            if !has_space && at_word_boundary {
                if !self.dropdown_open || self.trigger_start != Some(trigger_pos) {
                    self.open_dropdown(trigger_pos, cx);
                }
                self.dropdown_filter = between;
                self.dropdown_index = 0;
                cx.notify();
            } else if self.dropdown_open {
                self.close_dropdown(cx);
            }
        } else if self.dropdown_open {
            self.close_dropdown(cx);
        }
    }

    fn adjust_mentions_after_edit(&mut self, edit_start: usize, old_len: usize, new_len: usize) {
        let diff = new_len as isize - old_len as isize;

        self.mentions.retain(|mention| {
            !(edit_start < mention.range.end && edit_start + old_len > mention.range.start)
        });

        for mention in &mut self.mentions {
            if mention.range.start >= edit_start + old_len {
                mention.range.start = (mention.range.start as isize + diff) as usize;
                mention.range.end = (mention.range.end as isize + diff) as usize;
            }
        }
    }

    pub fn mention_up(&mut self, _: &MentionUp, _: &mut Window, cx: &mut Context<Self>) {
        if self.dropdown_open && self.dropdown_index > 0 {
            self.dropdown_index -= 1;
            cx.notify();
        }
    }

    pub fn mention_down(
        &mut self,
        _: &MentionDown,
        filtered_count: usize,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.dropdown_open && self.dropdown_index < filtered_count.saturating_sub(1) {
            self.dropdown_index += 1;
            cx.notify();
        }
    }

    pub fn mention_confirm(
        &mut self,
        _: &MentionConfirm,
        items: &[MentionItem],
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.dropdown_open {
            let filter = self.dropdown_filter.to_lowercase();
            let filtered: Vec<_> = items
                .iter()
                .filter(|item| item.name.to_lowercase().contains(&filter))
                .collect();

            if let Some(item) = filtered.get(self.dropdown_index) {
                self.insert_mention((*item).clone(), cx);
            }
        }
    }

    pub fn mention_cancel(&mut self, _: &MentionCancel, _: &mut Window, cx: &mut Context<Self>) {
        if self.dropdown_open {
            self.close_dropdown(cx);
        }
    }

    pub fn mention_backspace(
        &mut self,
        _: &MentionBackspace,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.selected_range.is_empty() {
            self.select_to(self.previous_boundary(self.cursor_offset()), cx)
        }
        self.replace_text_in_range(None, "", window, cx);
        self.check_for_trigger(cx);
    }

    pub fn mention_delete(
        &mut self,
        _: &MentionDelete,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.selected_range.is_empty() {
            self.select_to(self.next_boundary(self.cursor_offset()), cx)
        }
        self.replace_text_in_range(None, "", window, cx);
        self.check_for_trigger(cx);
    }

    pub fn mention_left(&mut self, _: &MentionLeft, _: &mut Window, cx: &mut Context<Self>) {
        if self.dropdown_open {
            return;
        }
        if self.selected_range.is_empty() {
            self.move_to(self.previous_boundary(self.cursor_offset()), cx);
        } else {
            self.move_to(self.selected_range.start, cx)
        }
    }

    pub fn mention_right(&mut self, _: &MentionRight, _: &mut Window, cx: &mut Context<Self>) {
        if self.dropdown_open {
            return;
        }
        if self.selected_range.is_empty() {
            self.move_to(self.next_boundary(self.selected_range.end), cx);
        } else {
            self.move_to(self.selected_range.end, cx)
        }
    }

    pub fn mention_home(&mut self, _: &MentionHome, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(0, cx);
    }

    pub fn mention_end(&mut self, _: &MentionEnd, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(self.content.len(), cx);
    }

    pub fn mention_select_all(
        &mut self,
        _: &MentionSelectAll,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.move_to(0, cx);
        self.select_to(self.content.len(), cx)
    }

    pub fn mention_copy(&mut self, _: &MentionCopy, _: &mut Window, cx: &mut Context<Self>) {
        if !self.selected_range.is_empty() {
            cx.write_to_clipboard(ClipboardItem::new_string(
                self.content[self.selected_range.clone()].to_string(),
            ));
        }
    }

    pub fn mention_cut(&mut self, _: &MentionCut, window: &mut Window, cx: &mut Context<Self>) {
        if !self.selected_range.is_empty() {
            cx.write_to_clipboard(ClipboardItem::new_string(
                self.content[self.selected_range.clone()].to_string(),
            ));
            self.replace_text_in_range(None, "", window, cx);
            self.check_for_trigger(cx);
        }
    }

    pub fn mention_paste(&mut self, _: &MentionPaste, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(text) = cx.read_from_clipboard().and_then(|item| item.text()) {
            let text = text.replace("\n", " ");
            self.replace_text_in_range(None, &text, window, cx);
            self.check_for_trigger(cx);
        }
    }

    pub fn mention_enter(&mut self, _: &MentionEnter, _: &mut Window, cx: &mut Context<Self>) {
        if !self.dropdown_open {
            cx.emit(MentionInputEvent::Enter);
        }
    }

    fn on_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.is_selecting = true;
        let click_index = self.index_for_mouse_position(event.position);
        if event.modifiers.shift {
            self.select_to(click_index, cx);
        } else {
            self.move_to(click_index, cx)
        }
    }

    fn on_mouse_up(&mut self, _: &MouseUpEvent, _window: &mut Window, _: &mut Context<Self>) {
        self.is_selecting = false;
    }

    fn on_mouse_move(&mut self, event: &MouseMoveEvent, _: &mut Window, cx: &mut Context<Self>) {
        if self.is_selecting {
            self.select_to(self.index_for_mouse_position(event.position), cx);
        }
    }

    pub fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EntityInputHandler for MentionInputState {
    fn text_for_range(
        &mut self,
        range_utf16: Range<usize>,
        actual_range: &mut Option<Range<usize>>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<String> {
        let range = self.range_from_utf16(&range_utf16);
        actual_range.replace(self.range_to_utf16(&range));
        Some(self.content[range].to_string())
    }

    fn selected_text_range(
        &mut self,
        _ignore_disabled_input: bool,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<UTF16Selection> {
        Some(UTF16Selection {
            range: self.range_to_utf16(&self.selected_range),
            reversed: self.selection_reversed,
        })
    }

    fn marked_text_range(
        &self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Range<usize>> {
        None
    }

    fn unmark_text(&mut self, _window: &mut Window, _cx: &mut Context<Self>) {}

    fn replace_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let range = range_utf16
            .as_ref()
            .map(|range_utf16| self.range_from_utf16(range_utf16))
            .unwrap_or(self.selected_range.clone());

        let old_len = range.end - range.start;
        self.adjust_mentions_after_edit(range.start, old_len, new_text.len());

        self.content = format!(
            "{}{}{}",
            &self.content[0..range.start],
            new_text,
            &self.content[range.end..]
        );
        self.selected_range = range.start + new_text.len()..range.start + new_text.len();

        self.check_for_trigger(cx);
        cx.emit(MentionInputEvent::Change);
        cx.notify();
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        new_selected_range_utf16: Option<Range<usize>>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let range = range_utf16
            .as_ref()
            .map(|range_utf16| self.range_from_utf16(range_utf16))
            .unwrap_or(self.selected_range.clone());

        self.content = format!(
            "{}{}{}",
            &self.content[0..range.start],
            new_text,
            &self.content[range.end..]
        );

        self.selected_range = new_selected_range_utf16
            .as_ref()
            .map(|range_utf16| self.range_from_utf16(range_utf16))
            .map(|new_range| new_range.start + range.start..new_range.end + range.end)
            .unwrap_or_else(|| range.start + new_text.len()..range.start + new_text.len());

        self.check_for_trigger(cx);
        cx.notify();
    }

    fn bounds_for_range(
        &mut self,
        range_utf16: Range<usize>,
        bounds: Bounds<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Bounds<Pixels>> {
        let last_layout = self.last_layout.as_ref()?;
        let range = self.range_from_utf16(&range_utf16);
        Some(Bounds::from_corners(
            point(
                bounds.left() + last_layout.x_for_index(range.start),
                bounds.top(),
            ),
            point(
                bounds.left() + last_layout.x_for_index(range.end),
                bounds.bottom(),
            ),
        ))
    }

    fn character_index_for_point(
        &mut self,
        point: Point<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<usize> {
        let line_point = self.last_bounds?.localize(&point)?;
        let last_layout = self.last_layout.as_ref()?;
        let utf8_index = last_layout.index_for_x(point.x - line_point.x)?;
        Some(self.offset_to_utf16(utf8_index))
    }
}

impl Focusable for MentionInputState {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for MentionInputState {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}

#[derive(IntoElement)]
pub struct MentionInput {
    state: Entity<MentionInputState>,
    items: Vec<MentionItem>,
    placeholder: SharedString,
    disabled: bool,
    trigger_char: char,
    max_dropdown_height: Pixels,

    on_change: Option<Rc<dyn Fn(&str, &mut App)>>,
    on_mention: Option<Rc<dyn Fn(&MentionItem, &mut App)>>,
    on_enter: Option<Rc<dyn Fn(&str, &mut App)>>,

    bounds: Bounds<Pixels>,
    style: StyleRefinement,
}

impl MentionInput {
    pub fn new(state: &Entity<MentionInputState>, items: Vec<MentionItem>) -> Self {
        Self {
            state: state.clone(),
            items,
            placeholder: "Type @ to mention someone...".into(),
            disabled: false,
            trigger_char: '@',
            max_dropdown_height: px(200.0),
            on_change: None,
            on_mention: None,
            on_enter: None,
            bounds: Bounds::default(),
            style: StyleRefinement::default(),
        }
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn trigger_char(mut self, trigger: char) -> Self {
        self.trigger_char = trigger;
        self
    }

    pub fn max_dropdown_height(mut self, height: impl Into<Pixels>) -> Self {
        self.max_dropdown_height = height.into();
        self
    }

    pub fn on_change<F>(mut self, callback: F) -> Self
    where
        F: Fn(&str, &mut App) + 'static,
    {
        self.on_change = Some(Rc::new(callback));
        self
    }

    pub fn on_mention<F>(mut self, callback: F) -> Self
    where
        F: Fn(&MentionItem, &mut App) + 'static,
    {
        self.on_mention = Some(Rc::new(callback));
        self
    }

    pub fn on_enter<F>(mut self, callback: F) -> Self
    where
        F: Fn(&str, &mut App) + 'static,
    {
        self.on_enter = Some(Rc::new(callback));
        self
    }

    fn filtered_items(&self, filter: &str) -> Vec<&MentionItem> {
        let filter_lower = filter.to_lowercase();
        self.items
            .iter()
            .filter(|item| item.name.to_lowercase().contains(&filter_lower))
            .collect()
    }
}

impl Styled for MentionInput {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for MentionInput {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = use_theme();

        let state_entity = self.state.clone();
        let state = self.state.read(cx);
        let is_focused = state.focus_handle(cx).is_focused(window);
        let dropdown_open = state.dropdown_open;
        let dropdown_filter = state.dropdown_filter.clone();
        let dropdown_index = state.dropdown_index;
        let _content = state.content.clone();
        let mentions = state.mentions.clone();
        let _selected_range = state.selected_range.clone();
        let placeholder = self.placeholder.clone();
        let focus_handle = state.focus_handle(cx);

        self.state.update(cx, |s, _| {
            s.trigger_char = self.trigger_char;
            s.placeholder = self.placeholder.clone();
            s.disabled = self.disabled;
        });

        let user_style = self.style.clone();
        let filtered = self.filtered_items(&dropdown_filter);
        let filtered_count = filtered.len();

        let on_change = self.on_change.clone();
        let on_mention = self.on_mention.clone();
        let on_enter = self.on_enter.clone();
        let items_for_confirm = self.items.clone();

        cx.subscribe(
            &state_entity,
            move |_emitter, event: &MentionInputEvent, cx| match event {
                MentionInputEvent::Change => {
                    if let Some(ref callback) = on_change {
                        let content = _emitter.read(cx).content.clone();
                        callback(&content, cx);
                    }
                }
                MentionInputEvent::MentionSelected(item) => {
                    if let Some(ref callback) = on_mention {
                        callback(item, cx);
                    }
                }
                MentionInputEvent::Enter => {
                    if let Some(ref callback) = on_enter {
                        let content = _emitter.read(cx).content.clone();
                        callback(&content, cx);
                    }
                }
                _ => {}
            },
        )
        .detach();

        let _bounds = self.bounds;

        let text_element = MentionTextElement {
            input: self.state.clone(),
            mentions: mentions.clone(),
            placeholder: placeholder.clone(),
        };

        div()
            .relative()
            .w_full()
            .key_context("MentionInput")
            .track_focus(&focus_handle)
            .when(!self.disabled, |this| {
                this.on_action({
                    let state = state_entity.clone();
                    move |action: &MentionBackspace, window, cx| {
                        state.update(cx, |s, cx| s.mention_backspace(action, window, cx));
                    }
                })
                .on_action({
                    let state = state_entity.clone();
                    move |action: &MentionDelete, window, cx| {
                        state.update(cx, |s, cx| s.mention_delete(action, window, cx));
                    }
                })
                .on_action({
                    let state = state_entity.clone();
                    move |action: &MentionLeft, window, cx| {
                        state.update(cx, |s, cx| s.mention_left(action, window, cx));
                    }
                })
                .on_action({
                    let state = state_entity.clone();
                    move |action: &MentionRight, window, cx| {
                        state.update(cx, |s, cx| s.mention_right(action, window, cx));
                    }
                })
                .on_action({
                    let state = state_entity.clone();
                    move |action: &MentionHome, window, cx| {
                        state.update(cx, |s, cx| s.mention_home(action, window, cx));
                    }
                })
                .on_action({
                    let state = state_entity.clone();
                    move |action: &MentionEnd, window, cx| {
                        state.update(cx, |s, cx| s.mention_end(action, window, cx));
                    }
                })
                .on_action({
                    let state = state_entity.clone();
                    move |action: &MentionSelectAll, window, cx| {
                        state.update(cx, |s, cx| s.mention_select_all(action, window, cx));
                    }
                })
                .on_action({
                    let state = state_entity.clone();
                    move |action: &MentionCopy, window, cx| {
                        state.update(cx, |s, cx| s.mention_copy(action, window, cx));
                    }
                })
                .on_action({
                    let state = state_entity.clone();
                    move |action: &MentionCut, window, cx| {
                        state.update(cx, |s, cx| s.mention_cut(action, window, cx));
                    }
                })
                .on_action({
                    let state = state_entity.clone();
                    move |action: &MentionPaste, window, cx| {
                        state.update(cx, |s, cx| s.mention_paste(action, window, cx));
                    }
                })
                .on_action({
                    let state = state_entity.clone();
                    move |action: &MentionUp, window, cx| {
                        state.update(cx, |s, cx| s.mention_up(action, window, cx));
                    }
                })
                .on_action({
                    let state = state_entity.clone();
                    let count = filtered_count;
                    move |action: &MentionDown, window, cx| {
                        state.update(cx, |s, cx| s.mention_down(action, count, window, cx));
                    }
                })
                .on_action({
                    let state = state_entity.clone();
                    let items = items_for_confirm.clone();
                    move |action: &MentionConfirm, window, cx| {
                        state.update(cx, |s, cx| s.mention_confirm(action, &items, window, cx));
                    }
                })
                .on_action({
                    let state = state_entity.clone();
                    move |action: &MentionCancel, window, cx| {
                        state.update(cx, |s, cx| s.mention_cancel(action, window, cx));
                    }
                })
                .on_action({
                    let state = state_entity.clone();
                    move |action: &MentionEnter, window, cx| {
                        state.update(cx, |s, cx| s.mention_enter(action, window, cx));
                    }
                })
            })
            .child(
                div()
                    .id("mention-input-container")
                    .flex()
                    .items_center()
                    .h(px(40.0))
                    .px(px(12.0))
                    .bg(theme.tokens.background)
                    .border_1()
                    .border_color(if is_focused {
                        theme.tokens.ring
                    } else {
                        theme.tokens.input
                    })
                    .rounded(theme.tokens.radius_md)
                    .shadow(vec![theme.tokens.shadow_xs.clone()])
                    .when(!self.disabled, |this| {
                        this.hover(|style| style.border_color(theme.tokens.ring))
                    })
                    .when(self.disabled, |this| this.opacity(0.5))
                    .on_mouse_down(MouseButton::Left, {
                        let state = state_entity.clone();
                        move |event, window, cx| {
                            state.update(cx, |s, cx| s.on_mouse_down(event, window, cx));
                        }
                    })
                    .on_mouse_up(MouseButton::Left, {
                        let state = state_entity.clone();
                        move |event, window, cx| {
                            state.update(cx, |s, cx| s.on_mouse_up(event, window, cx));
                        }
                    })
                    .on_mouse_move({
                        let state = state_entity.clone();
                        move |event, window, cx| {
                            state.update(cx, |s, cx| s.on_mouse_move(event, window, cx));
                        }
                    })
                    .child(
                        div()
                            .flex_1()
                            .overflow_hidden()
                            .text_size(px(14.0))
                            .font_family(theme.tokens.font_family.clone())
                            .text_color(theme.tokens.foreground)
                            .child(text_element),
                    )
                    .child({
                        canvas(
                            move |bounds, _, _cx| {
                                let _ = bounds;
                            },
                            |_, _, _, _| {},
                        )
                        .absolute()
                        .size_full()
                    }),
            )
            .when(dropdown_open && !filtered.is_empty(), |this| {
                let items = self.items.clone();
                let state_for_select = state_entity.clone();

                this.child(
                    deferred(
                        anchored()
                            .snap_to_window_with_margin(Edges::all(DROPDOWN_MARGIN))
                            .child(
                                div().occlude().w(px(300.0)).child(
                                    div()
                                        .occlude()
                                        .mt(DROPDOWN_MARGIN)
                                        .bg(theme.tokens.popover)
                                        .border_1()
                                        .border_color(theme.tokens.border)
                                        .rounded(theme.tokens.radius_md)
                                        .shadow_lg()
                                        .overflow_hidden()
                                        .child(
                                            div().max_h(self.max_dropdown_height).child(
                                                scrollable_vertical(
                                                    div().py(px(4.0)).children(
                                                        items
                                                            .iter()
                                                            .filter(|item| {
                                                                item.name.to_lowercase().contains(
                                                                    &dropdown_filter.to_lowercase(),
                                                                )
                                                            })
                                                            .enumerate()
                                                            .map(|(idx, item)| {
                                                                let is_selected =
                                                                    idx == dropdown_index;
                                                                let item_clone = item.clone();
                                                                let state =
                                                                    state_for_select.clone();

                                                                div()
                                                                .id(SharedString::from(format!(
                                                                    "mention-item-{}",
                                                                    idx
                                                                )))
                                                                .flex()
                                                                .items_center()
                                                                .gap(px(8.0))
                                                                .px(px(12.0))
                                                                .py(px(8.0))
                                                                .cursor_pointer()
                                                                .bg(if is_selected {
                                                                    theme.tokens.accent
                                                                } else {
                                                                    gpui::transparent_black()
                                                                })
                                                                .hover(|style| {
                                                                    style.bg(theme.tokens.accent)
                                                                })
                                                                .on_mouse_down(
                                                                    MouseButton::Left,
                                                                    move |_, _, cx| {
                                                                        state.update(
                                                                            cx,
                                                                            |s, cx| {
                                                                                s.insert_mention(
                                                                                    item_clone
                                                                                        .clone(),
                                                                                    cx,
                                                                                );
                                                                            },
                                                                        );
                                                                    },
                                                                )
                                                                .child(
                                                                    if let Some(ref avatar_url) =
                                                                        item.avatar_url
                                                                    {
                                                                        Avatar::new()
                                                                            .src(avatar_url.clone())
                                                                            .size(AvatarSize::Sm)
                                                                            .into_any_element()
                                                                    } else {
                                                                        Avatar::new()
                                                                            .name(item.name.clone())
                                                                            .size(AvatarSize::Sm)
                                                                            .into_any_element()
                                                                    },
                                                                )
                                                                .child(
                                                                    div()
                                                                        .text_size(px(14.0))
                                                                        .font_family(
                                                                            theme
                                                                                .tokens
                                                                                .font_family
                                                                                .clone(),
                                                                        )
                                                                        .text_color(
                                                                            theme
                                                                                .tokens
                                                                                .popover_foreground,
                                                                        )
                                                                        .child(item.name.clone()),
                                                                )
                                                            }),
                                                    ),
                                                ),
                                            ),
                                        ),
                                ),
                            ),
                    )
                    .with_priority(1),
                )
            })
            .map(|this| {
                let mut div = this;
                div.style().refine(&user_style);
                div
            })
    }
}

struct MentionTextElement {
    input: Entity<MentionInputState>,
    mentions: Vec<Mention>,
    placeholder: SharedString,
}

struct MentionPrepaintState {
    line: Option<gpui::ShapedLine>,
    cursor: Option<PaintQuad>,
    selection: Option<PaintQuad>,
}

impl IntoElement for MentionTextElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl gpui::Element for MentionTextElement {
    type RequestLayoutState = ();
    type PrepaintState = MentionPrepaintState;

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut style = Style::default();
        style.size.width = relative(1.).into();
        style.size.height = window.line_height().into();
        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        let input = self.input.read(cx);
        let content: SharedString = input.content.clone().into();
        let selected_range = input.selected_range.clone();
        let cursor_offset = input.cursor_offset();
        let theme = use_theme();
        let style = window.text_style();

        let (display_text, text_color) = if content.is_empty() {
            (self.placeholder.clone(), theme.tokens.muted_foreground)
        } else {
            (content.clone(), style.color)
        };

        let font_size = style.font_size.to_pixels(window.rem_size());

        let mut runs = Vec::new();
        if !content.is_empty() {
            let mut last_end = 0;
            let mut sorted_mentions = self.mentions.clone();
            sorted_mentions.sort_by_key(|m| m.range.start);

            for mention in &sorted_mentions {
                if mention.range.start > last_end {
                    runs.push(TextRun {
                        len: mention.range.start - last_end,
                        font: style.font(),
                        color: text_color,
                        background_color: None,
                        underline: None,
                        strikethrough: None,
                    });
                }

                let mention_len = mention.range.end.min(content.len()) - mention.range.start;
                if mention_len > 0 {
                    runs.push(TextRun {
                        len: mention_len,
                        font: style.font(),
                        color: theme.tokens.primary,
                        background_color: Some(theme.tokens.primary.opacity(0.1)),
                        underline: None,
                        strikethrough: None,
                    });
                }
                last_end = mention.range.end.min(content.len());
            }

            if last_end < content.len() {
                runs.push(TextRun {
                    len: content.len() - last_end,
                    font: style.font(),
                    color: text_color,
                    background_color: None,
                    underline: None,
                    strikethrough: None,
                });
            }
        }

        if runs.is_empty() {
            runs.push(TextRun {
                len: display_text.len(),
                font: style.font(),
                color: text_color,
                background_color: None,
                underline: None,
                strikethrough: None,
            });
        }

        let line = window
            .text_system()
            .shape_line(display_text, font_size, &runs, None);

        let cursor_pos = line.x_for_index(cursor_offset);
        let (selection, cursor) = if selected_range.is_empty() {
            (
                None,
                Some(fill(
                    Bounds::new(
                        point(bounds.left() + cursor_pos, bounds.top()),
                        size(px(2.), bounds.bottom() - bounds.top()),
                    ),
                    theme.tokens.primary,
                )),
            )
        } else {
            (
                Some(fill(
                    Bounds::from_corners(
                        point(
                            bounds.left() + line.x_for_index(selected_range.start),
                            bounds.top(),
                        ),
                        point(
                            bounds.left() + line.x_for_index(selected_range.end),
                            bounds.bottom(),
                        ),
                    ),
                    theme.tokens.primary.opacity(0.2),
                )),
                None,
            )
        };

        MentionPrepaintState {
            line: Some(line),
            cursor,
            selection,
        }
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let focus_handle = self.input.read(cx).focus_handle.clone();

        window.handle_input(
            &focus_handle,
            ElementInputHandler::new(bounds, self.input.clone()),
            cx,
        );

        if let Some(selection) = prepaint.selection.take() {
            window.paint_quad(selection)
        }

        let Some(line) = prepaint.line.take() else {
            return;
        };

        if line
            .paint(bounds.origin, window.line_height(), window, cx)
            .is_err()
        {
            return;
        }

        if focus_handle.is_focused(window) {
            if let Some(cursor) = prepaint.cursor.take() {
                window.paint_quad(cursor);
            }
        }

        self.input.update(cx, |input, _cx| {
            input.last_layout = Some(line);
            input.last_bounds = Some(bounds);
        });
    }
}

pub fn init_mention_input(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("backspace", MentionBackspace, Some("MentionInput")),
        KeyBinding::new("delete", MentionDelete, Some("MentionInput")),
        KeyBinding::new("left", MentionLeft, Some("MentionInput")),
        KeyBinding::new("right", MentionRight, Some("MentionInput")),
        KeyBinding::new("home", MentionHome, Some("MentionInput")),
        KeyBinding::new("end", MentionEnd, Some("MentionInput")),
        KeyBinding::new("up", MentionUp, Some("MentionInput")),
        KeyBinding::new("down", MentionDown, Some("MentionInput")),
        KeyBinding::new("enter", MentionConfirm, Some("MentionInput")),
        KeyBinding::new("escape", MentionCancel, Some("MentionInput")),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-a", MentionSelectAll, Some("MentionInput")),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-a", MentionSelectAll, Some("MentionInput")),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-c", MentionCopy, Some("MentionInput")),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-c", MentionCopy, Some("MentionInput")),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-x", MentionCut, Some("MentionInput")),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-x", MentionCut, Some("MentionInput")),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-v", MentionPaste, Some("MentionInput")),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-v", MentionPaste, Some("MentionInput")),
    ]);
}
