use gpui::{
    div, px, App, ClipboardItem, Context, FocusHandle, Focusable, InteractiveElement, IntoElement,
    KeyDownEvent, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, ParentElement, Render,
    ScrollWheelEvent, StatefulInteractiveElement, Styled, Subscription, Timer, Window,
};
use std::path::PathBuf;
use std::time::{Duration, Instant};

use adabraka_ui::theme::use_theme;

use crate::ansi_parser::{AnsiParser, ClearMode, ParsedSegment};
use crate::pty_service::{key_codes, PtyService};
use crate::terminal_state::{Charset, CursorStyle, TerminalLine, TerminalState};

const LINE_HEIGHT: f32 = 18.0;
const CHAR_WIDTH: f32 = 8.6;
const TERMINAL_PADDING: f32 = 8.0;
const CURSOR_BLINK_INTERVAL_MS: u64 = 530;

pub struct TerminalView {
    state: TerminalState,
    parser: AnsiParser,
    pty: Option<PtyService>,
    focus_handle: FocusHandle,
    cursor_blink: bool,
    cursor_blink_state: bool,
    last_blink_time: Instant,
    selection_start: Option<(usize, usize)>,
    selection_end: Option<(usize, usize)>,
    is_selecting: bool,
    viewport_height: f32,
    viewport_width: f32,
    polling_started: bool,
    last_resize: Option<(usize, usize)>,
    #[allow(dead_code)]
    focus_subscriptions: Vec<Subscription>,
    pending_clipboard: Option<String>,
    pending_notification: Option<(String, Option<String>)>,
    last_click_time: Instant,
    last_click_pos: Option<(usize, usize)>,
    click_count: u8,
}

impl TerminalView {
    pub fn title(&self) -> String {
        self.state
            .title()
            .map(|s| s.to_string())
            .unwrap_or_else(|| {
                self.state
                    .working_directory()
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| {
                        self.state.working_directory().to_string_lossy().to_string()
                    })
            })
    }

    pub fn new(cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        Self {
            state: TerminalState::default(),
            parser: AnsiParser::new(),
            pty: None,
            focus_handle,
            cursor_blink: true,
            cursor_blink_state: true,
            last_blink_time: Instant::now(),
            selection_start: None,
            selection_end: None,
            is_selecting: false,
            viewport_height: 400.0,
            viewport_width: 800.0,
            polling_started: false,
            last_resize: None,
            focus_subscriptions: Vec::new(),
            pending_clipboard: None,
            pending_notification: None,
            last_click_time: Instant::now(),
            last_click_pos: None,
            click_count: 0,
        }
    }

    pub fn with_working_directory(mut self, path: PathBuf) -> Self {
        self.state = self.state.with_working_directory(path);
        self
    }

    pub fn focus(&self, window: &mut Window) {
        window.focus(&self.focus_handle);
    }

    pub fn is_running(&self) -> bool {
        self.pty.as_ref().map(|p| p.is_running()).unwrap_or(false)
    }

    pub fn start_with_polling(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Result<(), String> {
        if self.is_running() {
            return Ok(());
        }

        let cols = self.calculate_cols();
        let rows = self.calculate_rows();
        self.state.resize(cols, rows);

        let mut pty = PtyService::new()
            .with_working_directory(self.state.working_directory().clone())
            .with_size(cols as u16, rows as u16);

        pty.start().map_err(|e| e.to_string())?;
        self.pty = Some(pty);
        self.state.set_running(true);

        let focus_in_sub = cx.on_focus_in(&self.focus_handle, window, |this, _, _cx| {
            this.send_focus_in();
        });
        let focus_out_sub = cx.on_focus_out(&self.focus_handle, window, |this, _, _, _cx| {
            this.send_focus_out();
        });
        self.focus_subscriptions = vec![focus_in_sub, focus_out_sub];

        self.polling_started = true;
        cx.spawn_in(window, async move |this, cx| loop {
            Timer::after(Duration::from_millis(8)).await;

            let should_continue = this
                .update(cx, |view, cx| {
                    if !view.is_running() {
                        view.polling_started = false;
                        return false;
                    }
                    view.process_output();
                    cx.notify();
                    true
                })
                .unwrap_or(false);

            if !should_continue {
                break;
            }
        })
        .detach();

        window.focus(&self.focus_handle);
        Ok(())
    }

    pub fn stop(&mut self) {
        if let Some(mut pty) = self.pty.take() {
            pty.stop();
        }
        self.state.set_running(false);
    }

    pub fn process_output(&mut self) {
        if let Some(pty) = &self.pty {
            let output = pty.drain_output();
            if !output.is_empty() {
                self.process_bytes(&output);
            }
        }
    }

    fn process_bytes(&mut self, bytes: &[u8]) {
        let segments = self.parser.parse(bytes);
        for segment in segments {
            self.apply_segment(segment);
        }
    }

    fn apply_segment(&mut self, segment: ParsedSegment) {
        match segment {
            ParsedSegment::Text(text, style) => {
                self.state.set_current_style(style);
                self.state.write_str(&text);
            }
            ParsedSegment::CursorUp(n) => self.state.cursor_up(n),
            ParsedSegment::CursorDown(n) => self.state.cursor_down(n),
            ParsedSegment::CursorForward(n) => self.state.cursor_forward(n),
            ParsedSegment::CursorBackward(n) => self.state.cursor_backward(n),
            ParsedSegment::CursorPosition(row, col) => self.state.move_cursor_to(row, col),
            ParsedSegment::CursorToColumn(col) => self.state.cursor_to_column(col),
            ParsedSegment::CursorNextLine(n) => self.state.cursor_next_line(n),
            ParsedSegment::CursorPrevLine(n) => self.state.cursor_prev_line(n),
            ParsedSegment::CursorSave => self.state.save_cursor(),
            ParsedSegment::CursorRestore => self.state.restore_cursor(),
            ParsedSegment::CursorVisible(v) => self.state.set_cursor_visible(v),
            ParsedSegment::CursorStyle(s) => {
                let style = match s {
                    0 | 1 | 2 => CursorStyle::Block,
                    3 | 4 => CursorStyle::Underline,
                    5 | 6 => CursorStyle::Bar,
                    _ => CursorStyle::Block,
                };
                self.state.set_cursor_style(style);
            }
            ParsedSegment::ClearScreen(mode) => match mode {
                ClearMode::ToEnd => self.state.clear_to_end_of_screen(),
                ClearMode::ToStart => self.state.clear_to_start_of_screen(),
                ClearMode::All | ClearMode::Scrollback => self.state.clear_screen(),
            },
            ParsedSegment::ClearLine(mode) => match mode {
                ClearMode::ToEnd => self.state.clear_to_end_of_line(),
                ClearMode::ToStart => self.state.clear_to_start_of_line(),
                ClearMode::All | ClearMode::Scrollback => self.state.clear_line(),
            },
            ParsedSegment::EraseChars(n) => self.state.erase_chars(n),
            ParsedSegment::InsertLines(n) => self.state.insert_lines(n),
            ParsedSegment::DeleteLines(n) => self.state.delete_lines(n),
            ParsedSegment::InsertChars(n) => self.state.insert_chars(n),
            ParsedSegment::DeleteChars(n) => self.state.delete_chars(n),
            ParsedSegment::ScrollUp(n) => self.state.scroll_up_n(n),
            ParsedSegment::ScrollDown(n) => self.state.scroll_down_n(n),
            ParsedSegment::SetScrollRegion(top, bottom) => self.state.set_scroll_region(top, bottom),
            ParsedSegment::ResetScrollRegion => self.state.reset_scroll_region(),
            ParsedSegment::SetTitle(title) => self.state.set_title(Some(title)),
            ParsedSegment::Bell => {}
            ParsedSegment::Backspace => self.state.backspace(),
            ParsedSegment::Tab => self.state.tab(),
            ParsedSegment::LineFeed => self.state.line_feed(),
            ParsedSegment::CarriageReturn => self.state.carriage_return(),
            ParsedSegment::ReverseIndex => self.state.reverse_index(),
            ParsedSegment::AltScreenEnter => self.state.enter_alt_screen(),
            ParsedSegment::AltScreenExit => self.state.exit_alt_screen(),
            ParsedSegment::BracketedPasteMode(enabled) => self.state.set_bracketed_paste(enabled),
            ParsedSegment::MouseTracking(enabled) => self.state.set_mouse_tracking(enabled),
            ParsedSegment::FocusTracking(enabled) => self.state.set_focus_tracking(enabled),
            ParsedSegment::OriginMode(enabled) => self.state.set_origin_mode(enabled),
            ParsedSegment::AutoWrap(enabled) => self.state.set_autowrap(enabled),
            ParsedSegment::InsertMode(enabled) => self.state.set_insert_mode(enabled),
            ParsedSegment::ApplicationCursorKeys(enabled) => {
                self.state.set_application_cursor_keys(enabled)
            }
            ParsedSegment::SetG0Charset(c) => {
                let charset = match c {
                    b'0' => Charset::DecSpecialGraphics,
                    _ => Charset::Ascii,
                };
                self.state.set_g0_charset(charset);
            }
            ParsedSegment::SetG1Charset(c) => {
                let charset = match c {
                    b'0' => Charset::DecSpecialGraphics,
                    _ => Charset::Ascii,
                };
                self.state.set_g1_charset(charset);
            }
            ParsedSegment::ShiftIn => self.state.shift_in(),
            ParsedSegment::ShiftOut => self.state.shift_out(),
            ParsedSegment::SyncUpdate(_) => {}
            ParsedSegment::SetHyperlink(url) => self.state.set_hyperlink(url),
            ParsedSegment::SetClipboard(text) => {
                self.pending_clipboard = Some(text);
            }
            ParsedSegment::Notification(title, body) => {
                self.pending_notification = Some((title, body));
            }
            ParsedSegment::Reset => self.state.reset(),
        }
    }

    pub fn send_input(&mut self, data: &[u8]) {
        if let Some(pty) = &mut self.pty {
            let _ = pty.write(data);
        }
    }

    pub fn send_str(&mut self, s: &str) {
        self.send_input(s.as_bytes());
    }

    fn calculate_cols(&self) -> usize {
        let available = self.viewport_width - TERMINAL_PADDING * 2.0 - 12.0;
        ((available / CHAR_WIDTH).floor() as usize).max(20)
    }

    fn calculate_rows(&self) -> usize {
        let available = self.viewport_height - TERMINAL_PADDING * 2.0;
        ((available / LINE_HEIGHT).floor() as usize).max(5)
    }

    pub fn update_viewport(&mut self, width: f32, height: f32) {
        self.viewport_width = width;
        self.viewport_height = height;

        let cols = self.calculate_cols();
        let rows = self.calculate_rows();

        if self.last_resize != Some((cols, rows)) {
            self.last_resize = Some((cols, rows));
            self.state.resize(cols, rows);
            if let Some(pty) = &mut self.pty {
                let _ = pty.resize(cols as u16, rows as u16);
            }
        }
    }

    pub fn scroll_up(&mut self, lines: usize) {
        self.state.scroll_viewport_up(lines);
    }

    pub fn scroll_down(&mut self, lines: usize) {
        self.state.scroll_viewport_down(lines);
    }

    pub fn scroll_to_bottom(&mut self) {
        self.state.scroll_to_bottom();
    }

    pub fn handle_key_down(
        &mut self,
        event: &KeyDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.is_running() {
            return;
        }

        if event.keystroke.modifiers.control || event.keystroke.modifiers.platform {
        } else {
            self.scroll_to_bottom();
        }

        let key = event.keystroke.key.as_str();

        let app_cursor = self.state.application_cursor_keys();
        let handled = match key {
            "enter" => { self.send_input(key_codes::ENTER); true }
            "tab" => { self.send_input(key_codes::TAB); true }
            "backspace" => { self.send_input(key_codes::BACKSPACE); true }
            "escape" => { self.send_input(key_codes::ESCAPE); true }
            "delete" => { self.send_input(key_codes::DELETE); true }
            "up" => {
                if app_cursor { self.send_input(b"\x1bOA"); } else { self.send_input(key_codes::UP); }
                true
            }
            "down" => {
                if app_cursor { self.send_input(b"\x1bOB"); } else { self.send_input(key_codes::DOWN); }
                true
            }
            "right" => {
                if app_cursor { self.send_input(b"\x1bOC"); } else { self.send_input(key_codes::RIGHT); }
                true
            }
            "left" => {
                if app_cursor { self.send_input(b"\x1bOD"); } else { self.send_input(key_codes::LEFT); }
                true
            }
            "home" => {
                if app_cursor { self.send_input(b"\x1bOH"); } else { self.send_input(key_codes::HOME); }
                true
            }
            "end" => {
                if app_cursor { self.send_input(b"\x1bOF"); } else { self.send_input(key_codes::END); }
                true
            }
            "pageup" => { self.send_input(key_codes::PAGE_UP); true }
            "pagedown" => { self.send_input(key_codes::PAGE_DOWN); true }
            "insert" => { self.send_input(b"\x1b[2~"); true }
            "f1" => { self.send_input(b"\x1bOP"); true }
            "f2" => { self.send_input(b"\x1bOQ"); true }
            "f3" => { self.send_input(b"\x1bOR"); true }
            "f4" => { self.send_input(b"\x1bOS"); true }
            "f5" => { self.send_input(b"\x1b[15~"); true }
            "f6" => { self.send_input(b"\x1b[17~"); true }
            "f7" => { self.send_input(b"\x1b[18~"); true }
            "f8" => { self.send_input(b"\x1b[19~"); true }
            "f9" => { self.send_input(b"\x1b[20~"); true }
            "f10" => { self.send_input(b"\x1b[21~"); true }
            "f11" => { self.send_input(b"\x1b[23~"); true }
            "f12" => { self.send_input(b"\x1b[24~"); true }
            "space" => { self.send_input(b" "); true }
            _ => false,
        };

        if !handled {
            if let Some(key_char) = &event.keystroke.key_char {
                if event.keystroke.modifiers.platform {
                    let c = key_char.chars().next().unwrap_or('\0').to_ascii_lowercase();
                    match c {
                        'c' => {
                            if self.has_selection() {
                                self.copy_selection(cx);
                            } else {
                                self.send_input(&[0x03]);
                            }
                        }
                        'v' => {
                            self.paste_from_clipboard(cx);
                        }
                        'd' => {
                            self.send_input(&[0x04]);
                        }
                        _ => {}
                    }
                } else if event.keystroke.modifiers.control {
                    let c = key_char.chars().next().unwrap_or('\0');
                    if c.is_ascii_alphabetic() {
                        let ctrl_code = (c.to_ascii_lowercase() as u8) - b'a' + 1;
                        self.send_input(&[ctrl_code]);
                    }
                } else if event.keystroke.modifiers.alt {
                    let mut data = vec![0x1b];
                    data.extend(key_char.as_bytes());
                    self.send_input(&data);
                } else {
                    self.send_str(key_char);
                    self.clear_selection();
                }
            } else if !event.keystroke.modifiers.platform
                && !event.keystroke.modifiers.control
                && !event.keystroke.modifiers.alt
            {
                if key == "space" {
                    self.send_input(b" ");
                    self.clear_selection();
                } else if key.len() == 1 {
                    self.send_str(key);
                    self.clear_selection();
                }
            }
        }

        self.reset_cursor_blink();
    }

    pub fn handle_scroll(
        &mut self,
        event: &ScrollWheelEvent,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        let delta_y = match event.delta {
            gpui::ScrollDelta::Lines(lines) => lines.y,
            gpui::ScrollDelta::Pixels(pixels) => f32::from(pixels.y) / LINE_HEIGHT,
        };

        let lines = delta_y.abs().ceil() as usize;
        let lines = lines.max(1);

        if delta_y > 0.0 {
            self.scroll_up(lines);
        } else {
            self.scroll_down(lines);
        }
    }

    pub fn handle_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        window.focus(&self.focus_handle);

        if event.button == MouseButton::Left {
            let (line, col) = self.position_from_mouse(event.position);

            if event.modifiers.platform {
                if let Some(url) = self.hyperlink_at(line, col) {
                    let _ = open::that(&url);
                    return;
                }
            }

            let now = Instant::now();
            let same_pos = self.last_click_pos == Some((line, col));
            let quick_click = now.duration_since(self.last_click_time) < Duration::from_millis(400);

            if same_pos && quick_click {
                self.click_count = (self.click_count % 3) + 1;
            } else {
                self.click_count = 1;
            }

            self.last_click_time = now;
            self.last_click_pos = Some((line, col));

            match self.click_count {
                2 => {
                    let (start, end) = self.word_bounds_at(line, col);
                    self.selection_start = Some((line, start));
                    self.selection_end = Some((line, end));
                    self.is_selecting = false;
                }
                3 => {
                    let line_len = self.state.line(line).map(|l| l.len()).unwrap_or(0);
                    self.selection_start = Some((line, 0));
                    self.selection_end = Some((line, line_len));
                    self.is_selecting = false;
                }
                _ => {
                    self.selection_start = Some((line, col));
                    self.selection_end = Some((line, col));
                    self.is_selecting = true;
                }
            }
            cx.notify();
        }
    }

    fn hyperlink_at(&self, line_idx: usize, col: usize) -> Option<String> {
        let line = self.state.line(line_idx)?;
        let cell = line.get(col)?;
        cell.hyperlink.clone()
    }

    fn word_bounds_at(&self, line_idx: usize, col: usize) -> (usize, usize) {
        let line = match self.state.line(line_idx) {
            Some(l) => l,
            None => return (col, col),
        };

        let chars: Vec<char> = line.cells.iter().map(|c| c.char).collect();
        if col >= chars.len() {
            return (col, col);
        }

        let is_word_char = |c: char| c.is_alphanumeric() || c == '_';
        let target_is_word = is_word_char(chars[col]);

        let mut start = col;
        while start > 0 {
            let prev = chars[start - 1];
            if target_is_word {
                if !is_word_char(prev) {
                    break;
                }
            } else if prev.is_whitespace() != chars[col].is_whitespace() {
                break;
            }
            start -= 1;
        }

        let mut end = col;
        while end < chars.len() {
            let curr = chars[end];
            if target_is_word {
                if !is_word_char(curr) {
                    break;
                }
            } else if curr.is_whitespace() != chars[col].is_whitespace() {
                break;
            }
            end += 1;
        }

        (start, end)
    }

    pub fn handle_mouse_move(
        &mut self,
        event: &MouseMoveEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.is_selecting {
            let (line, col) = self.position_from_mouse(event.position);
            self.selection_end = Some((line, col));
            cx.notify();
        }
    }

    pub fn handle_mouse_up(
        &mut self,
        event: &MouseUpEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if event.button == MouseButton::Left {
            self.is_selecting = false;
            cx.notify();
        }
    }

    fn position_from_mouse(&self, position: gpui::Point<gpui::Pixels>) -> (usize, usize) {
        let x = f32::from(position.x) - TERMINAL_PADDING;
        let y = f32::from(position.y) - TERMINAL_PADDING;

        let col = (x / CHAR_WIDTH).max(0.0) as usize;
        let row = (y / LINE_HEIGHT).max(0.0) as usize;

        let total = self.state.total_lines();
        let rows = self.state.rows();
        let scroll_offset = self.state.scroll_offset();
        let viewport_start = total.saturating_sub(rows + scroll_offset);

        let line = viewport_start + row;

        (line, col)
    }

    fn is_position_selected(&self, line: usize, col: usize) -> bool {
        if let (Some(start), Some(end)) = (self.selection_start, self.selection_end) {
            let (start_line, start_col, end_line, end_col) =
                if start.0 > end.0 || (start.0 == end.0 && start.1 > end.1) {
                    (end.0, end.1, start.0, start.1)
                } else {
                    (start.0, start.1, end.0, end.1)
                };

            if line < start_line || line > end_line {
                return false;
            }

            if line == start_line && line == end_line {
                col >= start_col && col < end_col
            } else if line == start_line {
                col >= start_col
            } else if line == end_line {
                col < end_col
            } else {
                true
            }
        } else {
            false
        }
    }

    pub fn get_selected_text(&self) -> Option<String> {
        let (start, end) = (self.selection_start?, self.selection_end?);

        let (start_line, start_col, end_line, end_col) =
            if start.0 > end.0 || (start.0 == end.0 && start.1 > end.1) {
                (end.0, end.1, start.0, start.1)
            } else {
                (start.0, start.1, end.0, end.1)
            };

        if start_line == end_line && start_col == end_col {
            return None;
        }

        let mut result = String::new();

        for line_idx in start_line..=end_line {
            if let Some(line) = self.state.line(line_idx) {
                let line_text: String = line.cells.iter().map(|c| c.char).collect();
                let line_text = line_text.trim_end();

                let col_start = if line_idx == start_line { start_col } else { 0 };
                let col_end = if line_idx == end_line {
                    end_col.min(line_text.len())
                } else {
                    line_text.len()
                };

                if col_start < line_text.len() {
                    let chars: Vec<char> = line_text.chars().collect();
                    let selected: String =
                        chars[col_start..col_end.min(chars.len())].iter().collect();
                    result.push_str(&selected);
                }

                if line_idx < end_line {
                    result.push('\n');
                }
            }
        }

        if result.is_empty() { None } else { Some(result) }
    }

    pub fn copy_selection(&self, cx: &mut Context<Self>) {
        if let Some(text) = self.get_selected_text() {
            cx.write_to_clipboard(ClipboardItem::new_string(text));
        }
    }

    pub fn paste_from_clipboard(&mut self, cx: &mut Context<Self>) {
        if let Some(item) = cx.read_from_clipboard() {
            if let Some(text) = item.text() {
                if self.state.bracketed_paste() {
                    self.send_input(b"\x1b[200~");
                    self.send_str(&text);
                    self.send_input(b"\x1b[201~");
                } else {
                    self.send_str(&text);
                }
            }
        }
    }

    pub fn clear_selection(&mut self) {
        self.selection_start = None;
        self.selection_end = None;
        self.is_selecting = false;
    }

    pub fn has_selection(&self) -> bool {
        self.selection_start.is_some()
            && self.selection_end.is_some()
            && self.selection_start != self.selection_end
    }

    pub fn update_cursor_blink(&mut self) {
        let now = Instant::now();
        if now.duration_since(self.last_blink_time)
            >= Duration::from_millis(CURSOR_BLINK_INTERVAL_MS)
        {
            self.cursor_blink_state = !self.cursor_blink_state;
            self.last_blink_time = now;
        }
    }

    pub fn reset_cursor_blink(&mut self) {
        self.cursor_blink_state = true;
        self.last_blink_time = Instant::now();
    }

    pub fn send_focus_in(&mut self) {
        if self.state.focus_tracking() {
            self.send_input(b"\x1b[I");
        }
    }

    pub fn send_focus_out(&mut self) {
        if self.state.focus_tracking() {
            self.send_input(b"\x1b[O");
        }
    }

    fn render_line(
        &self,
        idx: usize,
        line: &TerminalLine,
        is_cursor_line: bool,
    ) -> impl IntoElement {
        let cursor_col = self.state.cursor().col;
        let show_cursor = is_cursor_line
            && self.cursor_blink
            && self.cursor_blink_state
            && self.state.cursor_visible()
            && self.state.is_at_bottom();

        let theme = use_theme();
        let cursor_bg = theme.tokens.primary;
        let cursor_fg = theme.tokens.background;
        let selection_bg = theme.tokens.accent.opacity(0.3);

        let has_selection = self.has_selection() && {
            let (start, end) = (self.selection_start.unwrap(), self.selection_end.unwrap());
            let (s_line, e_line) = if start.0 <= end.0 {
                (start.0, end.0)
            } else {
                (end.0, start.0)
            };
            idx >= s_line && idx <= e_line
        };

        let cols = self.state.cols();
        let mut spans: Vec<gpui::AnyElement> = Vec::new();
        let mut current_text = String::new();
        let mut current_style: Option<&crate::terminal_state::CellStyle> = None;
        let mut current_selected = false;
        let mut current_has_link = false;

        for col in 0..cols {
            let cell = line.get(col);
            let is_cursor_pos = show_cursor && col == cursor_col;
            let is_selected = has_selection && self.is_position_selected(idx, col);
            let has_link = cell.map(|c| c.hyperlink.is_some()).unwrap_or(false);

            if is_cursor_pos {
                if !current_text.is_empty() {
                    let style = current_style.cloned().unwrap_or_default();
                    spans.push(self.make_span(&current_text, &style, current_selected, current_has_link, &theme, &selection_bg));
                    current_text.clear();
                }

                let c = cell.map(|c| c.char).unwrap_or(' ');
                let cursor_span = match self.state.cursor_style() {
                    CursorStyle::Block => {
                        div()
                            .bg(cursor_bg)
                            .text_color(cursor_fg)
                            .child(c.to_string())
                    }
                    CursorStyle::Underline => {
                        div()
                            .border_b_2()
                            .border_color(cursor_bg)
                            .text_color(cell.map(|c| c.style.effective_fg()).unwrap_or(theme.tokens.foreground.into()))
                            .child(c.to_string())
                    }
                    CursorStyle::Bar => {
                        div()
                            .border_l_2()
                            .border_color(cursor_bg)
                            .text_color(cell.map(|c| c.style.effective_fg()).unwrap_or(theme.tokens.foreground.into()))
                            .child(c.to_string())
                    }
                };
                spans.push(cursor_span.into_any_element());
                current_style = cell.map(|c| &c.style);
                current_selected = is_selected;
                current_has_link = has_link;
            } else if let Some(cell) = cell {
                let cell_has_link = cell.hyperlink.is_some();
                let needs_flush = current_style.map(|s| s != &cell.style).unwrap_or(true)
                    || is_selected != current_selected
                    || cell_has_link != current_has_link;

                if needs_flush && !current_text.is_empty() {
                    let style = current_style.cloned().unwrap_or_default();
                    spans.push(self.make_span(&current_text, &style, current_selected, current_has_link, &theme, &selection_bg));
                    current_text.clear();
                }

                current_style = Some(&cell.style);
                current_selected = is_selected;
                current_has_link = cell_has_link;
                current_text.push(cell.char);
            }
        }

        if !current_text.is_empty() {
            let style = current_style.cloned().unwrap_or_default();
            spans.push(self.make_span(&current_text, &style, current_selected, current_has_link, &theme, &selection_bg));
        }

        if show_cursor && cursor_col >= cols {
            let cursor_span = div().bg(cursor_bg).text_color(cursor_fg).child(" ");
            spans.push(cursor_span.into_any_element());
        }

        div()
            .h(px(LINE_HEIGHT))
            .w_full()
            .flex()
            .font_family("JetBrains Mono")
            .text_size(px(13.0))
            .children(spans)
    }

    fn make_span(
        &self,
        text: &str,
        style: &crate::terminal_state::CellStyle,
        selected: bool,
        has_hyperlink: bool,
        _theme: &adabraka_ui::theme::Theme,
        selection_bg: &gpui::Hsla,
    ) -> gpui::AnyElement {
        let fg = style.effective_fg();
        let bg = style.effective_bg();

        let mut el = div().text_color(fg);

        if selected {
            el = el.bg(*selection_bg);
        } else if bg.a > 0.01 {
            el = el.bg(bg);
        }

        if style.bold {
            el = el.font_weight(gpui::FontWeight::BOLD);
        }

        if style.italic {
            el = el.italic();
        }

        if style.underline || has_hyperlink {
            el = el.underline();
        }

        if style.strikethrough {
            el = el.line_through();
        }

        el.child(text.to_string()).into_any_element()
    }

    fn cursor_absolute_line(&self) -> usize {
        let total = self.state.total_lines();
        let rows = self.state.rows();
        total.saturating_sub(rows) + self.state.cursor().row
    }
}

impl Focusable for TerminalView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for TerminalView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.process_output();

        if let Some(text) = self.pending_clipboard.take() {
            cx.write_to_clipboard(ClipboardItem::new_string(text));
        }

        if let Some((_title, _body)) = self.pending_notification.take() {
            // TODO: Show native notification when GPUI supports it
        }

        if self.is_running() && !self.polling_started {
            self.polling_started = true;
            cx.spawn_in(window, async move |this, cx| loop {
                Timer::after(Duration::from_millis(8)).await;

                let should_continue = this
                    .update(cx, |view, cx| {
                        if !view.is_running() {
                            view.polling_started = false;
                            return false;
                        }
                        view.process_output();
                        cx.notify();
                        true
                    })
                    .unwrap_or(false);

                if !should_continue {
                    break;
                }
            })
            .detach();

            window.focus(&self.focus_handle);
        }

        let _theme = use_theme();
        let terminal_bg = gpui::rgb(0x1a1b26);

        let cursor_line = self.cursor_absolute_line();
        let total = self.state.total_lines();
        let rows = self.state.rows();
        let scroll_offset = self.state.scroll_offset();

        let viewport_start = total.saturating_sub(rows + scroll_offset);
        let viewport_end = total.saturating_sub(scroll_offset);

        self.update_cursor_blink();

        let lines_to_render: Vec<_> = (viewport_start..viewport_end)
            .map(|idx| {
                let line = self.state.line(idx).cloned();
                let is_cursor_line = idx == cursor_line;
                (idx, line, is_cursor_line)
            })
            .collect();

        div()
            .id("terminal-view")
            .key_context("Terminal")
            .track_focus(&self.focus_handle)
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, window, cx| {
                this.handle_key_down(event, window, cx);
                cx.stop_propagation();
            }))
            .on_scroll_wheel(cx.listener(Self::handle_scroll))
            .on_mouse_down(MouseButton::Left, cx.listener(Self::handle_mouse_down))
            .on_mouse_move(cx.listener(Self::handle_mouse_move))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::handle_mouse_up))
            .size_full()
            .bg(terminal_bg)
            .flex()
            .flex_col()
            .child(
                div()
                    .id("terminal-content")
                    .flex_1()
                    .overflow_hidden()
                    .p(px(TERMINAL_PADDING))
                    .on_resize({
                        let this = cx.entity().clone();
                        move |ev, _window, cx| {
                            let width = f32::from(ev.size.width);
                            let height = f32::from(ev.size.height);
                            this.update(cx, |view, _cx| {
                                view.update_viewport(width, height);
                            });
                        }
                    })
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .children(
                                lines_to_render
                                    .into_iter()
                                    .map(|(idx, line, is_cursor_line)| {
                                        if let Some(line) = line {
                                            self.render_line(idx, &line, is_cursor_line)
                                                .into_any_element()
                                        } else {
                                            div().h(px(LINE_HEIGHT)).into_any_element()
                                        }
                                    })
                                    .collect::<Vec<_>>(),
                            ),
                    ),
            )
    }
}
