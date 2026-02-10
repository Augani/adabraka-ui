use gpui::Rgba;
use std::path::PathBuf;

pub const DEFAULT_COLS: usize = 80;
pub const DEFAULT_ROWS: usize = 24;
pub const DEFAULT_SCROLLBACK: usize = 10000;

#[derive(Clone, Debug, PartialEq)]
pub struct CellStyle {
    pub foreground: Rgba,
    pub background: Rgba,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strikethrough: bool,
    pub dim: bool,
    pub inverse: bool,
    pub blink: bool,
    pub hidden: bool,
}

impl Default for CellStyle {
    fn default() -> Self {
        Self {
            foreground: Rgba {
                r: 0.93,
                g: 0.93,
                b: 0.93,
                a: 1.0,
            },
            background: Rgba {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            },
            bold: false,
            italic: false,
            underline: false,
            strikethrough: false,
            dim: false,
            inverse: false,
            blink: false,
            hidden: false,
        }
    }
}

impl CellStyle {
    pub fn effective_fg(&self) -> Rgba {
        if self.inverse {
            if self.background.a < 0.01 {
                Rgba {
                    r: 0.1,
                    g: 0.1,
                    b: 0.1,
                    a: 1.0,
                }
            } else {
                self.background
            }
        } else if self.hidden {
            self.background
        } else {
            let mut fg = self.foreground;
            if self.dim {
                fg.r *= 0.6;
                fg.g *= 0.6;
                fg.b *= 0.6;
            }
            fg
        }
    }

    pub fn effective_bg(&self) -> Rgba {
        if self.inverse {
            self.foreground
        } else {
            self.background
        }
    }
}

#[derive(Clone, Debug)]
pub struct TerminalCell {
    pub char: char,
    pub style: CellStyle,
    pub width: u8,
}

impl Default for TerminalCell {
    fn default() -> Self {
        Self {
            char: ' ',
            style: CellStyle::default(),
            width: 1,
        }
    }
}

impl TerminalCell {
    pub fn new(char: char, style: CellStyle) -> Self {
        let width = if char.is_ascii() {
            1
        } else {
            unicode_width::UnicodeWidthChar::width(char).unwrap_or(1) as u8
        };
        Self { char, style, width }
    }
}

#[derive(Clone, Debug)]
pub struct TerminalLine {
    pub cells: Vec<TerminalCell>,
    pub wrapped: bool,
}

impl TerminalLine {
    pub fn new(cols: usize) -> Self {
        Self {
            cells: vec![TerminalCell::default(); cols],
            wrapped: false,
        }
    }

    pub fn len(&self) -> usize {
        self.cells.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    pub fn get(&self, col: usize) -> Option<&TerminalCell> {
        self.cells.get(col)
    }

    pub fn get_mut(&mut self, col: usize) -> Option<&mut TerminalCell> {
        self.cells.get_mut(col)
    }

    pub fn set(&mut self, col: usize, cell: TerminalCell) {
        if col < self.cells.len() {
            self.cells[col] = cell;
        }
    }

    pub fn clear(&mut self) {
        for cell in &mut self.cells {
            *cell = TerminalCell::default();
        }
        self.wrapped = false;
    }

    pub fn clear_with_style(&mut self, style: &CellStyle) {
        for cell in &mut self.cells {
            cell.char = ' ';
            cell.style = style.clone();
            cell.width = 1;
        }
        self.wrapped = false;
    }

    pub fn clear_from(&mut self, col: usize) {
        for i in col..self.cells.len() {
            self.cells[i] = TerminalCell::default();
        }
    }

    pub fn clear_from_with_style(&mut self, col: usize, style: &CellStyle) {
        for i in col..self.cells.len() {
            self.cells[i] = TerminalCell {
                char: ' ',
                style: style.clone(),
                width: 1,
            };
        }
    }

    pub fn clear_to(&mut self, col: usize) {
        let end = col.min(self.cells.len());
        for i in 0..end {
            self.cells[i] = TerminalCell::default();
        }
    }

    pub fn clear_to_with_style(&mut self, col: usize, style: &CellStyle) {
        let end = col.min(self.cells.len());
        for i in 0..end {
            self.cells[i] = TerminalCell {
                char: ' ',
                style: style.clone(),
                width: 1,
            };
        }
    }

    pub fn text(&self) -> String {
        let mut s: String = self.cells.iter().map(|c| c.char).collect();
        s.truncate(s.trim_end().len());
        s
    }

    pub fn resize(&mut self, cols: usize) {
        self.cells.resize_with(cols, TerminalCell::default);
    }

    pub fn insert_cells(&mut self, col: usize, count: usize) {
        let cols = self.cells.len();
        for _ in 0..count {
            if col < cols {
                self.cells.insert(col, TerminalCell::default());
                self.cells.truncate(cols);
            }
        }
    }

    pub fn delete_cells(&mut self, col: usize, count: usize) {
        let cols = self.cells.len();
        for _ in 0..count {
            if col < self.cells.len() {
                self.cells.remove(col);
                self.cells.push(TerminalCell::default());
            }
        }
        self.cells.truncate(cols);
    }

    pub fn erase_chars(&mut self, col: usize, count: usize, style: &CellStyle) {
        let end = (col + count).min(self.cells.len());
        for i in col..end {
            self.cells[i] = TerminalCell {
                char: ' ',
                style: style.clone(),
                width: 1,
            };
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CursorPosition {
    pub row: usize,
    pub col: usize,
}

impl CursorPosition {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    pub fn origin() -> Self {
        Self { row: 0, col: 0 }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CursorStyle {
    Block,
    Underline,
    Bar,
}

impl Default for CursorStyle {
    fn default() -> Self {
        Self::Block
    }
}

#[derive(Clone, Debug)]
struct SavedCursor {
    position: CursorPosition,
    style: CellStyle,
    origin_mode: bool,
    autowrap: bool,
}

#[derive(Clone, Debug)]
pub struct TerminalState {
    lines: Vec<TerminalLine>,
    cursor: CursorPosition,
    cols: usize,
    rows: usize,
    scroll_offset: usize,
    max_scrollback: usize,
    working_directory: PathBuf,
    is_running: bool,
    current_style: CellStyle,
    cursor_visible: bool,
    cursor_style: CursorStyle,
    saved_cursor: Option<SavedCursor>,
    title: Option<String>,

    scroll_region_top: usize,
    scroll_region_bottom: usize,
    origin_mode: bool,
    autowrap: bool,
    insert_mode: bool,

    alt_screen: Option<Box<AltScreenState>>,
    use_alt_screen: bool,

    bracketed_paste: bool,
    mouse_tracking: bool,
    focus_tracking: bool,

    user_scrolled: bool,

    tabs: Vec<usize>,
}

#[derive(Clone, Debug)]
struct AltScreenState {
    lines: Vec<TerminalLine>,
    cursor: CursorPosition,
    saved_cursor: Option<SavedCursor>,
}

impl Default for TerminalState {
    fn default() -> Self {
        Self::new(DEFAULT_COLS, DEFAULT_ROWS)
    }
}

impl TerminalState {
    pub fn new(cols: usize, rows: usize) -> Self {
        let mut lines = Vec::with_capacity(rows);
        for _ in 0..rows {
            lines.push(TerminalLine::new(cols));
        }

        let mut tabs = Vec::new();
        for i in (0..cols).step_by(8) {
            tabs.push(i);
        }

        Self {
            lines,
            cursor: CursorPosition::origin(),
            cols,
            rows,
            scroll_offset: 0,
            max_scrollback: DEFAULT_SCROLLBACK,
            working_directory: std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/")),
            is_running: false,
            current_style: CellStyle::default(),
            cursor_visible: true,
            cursor_style: CursorStyle::default(),
            saved_cursor: None,
            title: None,
            scroll_region_top: 0,
            scroll_region_bottom: rows.saturating_sub(1),
            origin_mode: false,
            autowrap: true,
            insert_mode: false,
            alt_screen: None,
            use_alt_screen: false,
            bracketed_paste: false,
            mouse_tracking: false,
            focus_tracking: false,
            user_scrolled: false,
            tabs,
        }
    }

    pub fn with_working_directory(mut self, path: PathBuf) -> Self {
        self.working_directory = path;
        self
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cursor(&self) -> CursorPosition {
        self.cursor
    }

    pub fn cursor_visible(&self) -> bool {
        self.cursor_visible
    }

    pub fn cursor_style(&self) -> CursorStyle {
        self.cursor_style
    }

    pub fn scroll_offset(&self) -> usize {
        self.scroll_offset
    }

    pub fn working_directory(&self) -> &PathBuf {
        &self.working_directory
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    pub fn current_style(&self) -> &CellStyle {
        &self.current_style
    }

    pub fn total_lines(&self) -> usize {
        self.lines.len()
    }

    pub fn scrollback_lines(&self) -> usize {
        self.lines.len().saturating_sub(self.rows)
    }

    pub fn max_scroll_offset(&self) -> usize {
        self.scrollback_lines()
    }

    pub fn is_at_bottom(&self) -> bool {
        self.scroll_offset == 0
    }

    pub fn user_scrolled(&self) -> bool {
        self.user_scrolled
    }

    pub fn bracketed_paste(&self) -> bool {
        self.bracketed_paste
    }

    pub fn set_running(&mut self, running: bool) {
        self.is_running = running;
    }

    pub fn set_working_directory(&mut self, path: PathBuf) {
        self.working_directory = path;
    }

    pub fn set_title(&mut self, title: Option<String>) {
        self.title = title;
    }

    pub fn set_current_style(&mut self, style: CellStyle) {
        self.current_style = style;
    }

    pub fn set_cursor_visible(&mut self, visible: bool) {
        self.cursor_visible = visible;
    }

    pub fn set_cursor_style(&mut self, style: CursorStyle) {
        self.cursor_style = style;
    }

    pub fn set_bracketed_paste(&mut self, enabled: bool) {
        self.bracketed_paste = enabled;
    }

    pub fn set_mouse_tracking(&mut self, enabled: bool) {
        self.mouse_tracking = enabled;
    }

    pub fn set_focus_tracking(&mut self, enabled: bool) {
        self.focus_tracking = enabled;
    }

    pub fn line(&self, index: usize) -> Option<&TerminalLine> {
        self.lines.get(index)
    }

    pub fn visible_lines(&self) -> impl Iterator<Item = &TerminalLine> {
        let total = self.lines.len();
        let start = total.saturating_sub(self.rows + self.scroll_offset);
        let end = total.saturating_sub(self.scroll_offset);
        self.lines[start..end].iter()
    }

    fn viewport_to_absolute(&self, row: usize) -> usize {
        let total = self.lines.len();
        total.saturating_sub(self.rows) + row
    }

    fn current_line_mut(&mut self) -> &mut TerminalLine {
        let idx = self.viewport_to_absolute(self.cursor.row);
        while self.lines.len() <= idx {
            self.lines.push(TerminalLine::new(self.cols));
        }
        &mut self.lines[idx]
    }

    fn effective_row(&self) -> usize {
        if self.origin_mode {
            self.scroll_region_top + self.cursor.row
        } else {
            self.cursor.row
        }
    }

    pub fn write_char(&mut self, c: char) {
        if self.cursor.col >= self.cols {
            if self.autowrap {
                self.current_line_mut().wrapped = true;
                self.newline();
            } else {
                self.cursor.col = self.cols - 1;
            }
        }

        if self.insert_mode {
            let col = self.cursor.col;
            self.current_line_mut().insert_cells(col, 1);
        }

        let style = self.current_style.clone();
        let col = self.cursor.col;
        let cell = TerminalCell::new(c, style);
        let width = cell.width as usize;

        let line = self.current_line_mut();
        line.set(col, cell);

        for i in 1..width {
            if col + i < line.len() {
                line.set(
                    col + i,
                    TerminalCell {
                        char: ' ',
                        style: CellStyle::default(),
                        width: 0,
                    },
                );
            }
        }

        self.cursor.col += width;
    }

    pub fn write_str(&mut self, s: &str) {
        for c in s.chars() {
            match c {
                '\n' => self.newline(),
                '\r' => self.carriage_return(),
                '\t' => self.tab(),
                '\x08' => self.backspace(),
                '\x07' => {}
                c if c.is_control() => {}
                c => self.write_char(c),
            }
        }
    }

    pub fn newline(&mut self) {
        self.cursor.col = 0;
        if self.cursor.row >= self.scroll_region_bottom {
            self.scroll_up_region();
        } else if self.cursor.row + 1 < self.rows {
            self.cursor.row += 1;
        }

        if !self.user_scrolled {
            self.scroll_offset = 0;
        }
    }

    pub fn line_feed(&mut self) {
        if self.cursor.row >= self.scroll_region_bottom {
            self.scroll_up_region();
        } else if self.cursor.row + 1 < self.rows {
            self.cursor.row += 1;
        }

        if !self.user_scrolled {
            self.scroll_offset = 0;
        }
    }

    pub fn reverse_index(&mut self) {
        if self.cursor.row <= self.scroll_region_top {
            self.scroll_down_region();
        } else if self.cursor.row > 0 {
            self.cursor.row -= 1;
        }
    }

    pub fn carriage_return(&mut self) {
        self.cursor.col = 0;
    }

    pub fn tab(&mut self) {
        let next_tab = self
            .tabs
            .iter()
            .find(|&&t| t > self.cursor.col)
            .copied()
            .unwrap_or(self.cols - 1);
        self.cursor.col = next_tab.min(self.cols - 1);
    }

    pub fn backspace(&mut self) {
        if self.cursor.col > 0 {
            self.cursor.col -= 1;
        }
    }

    fn scroll_up_region(&mut self) {
        if self.use_alt_screen {
            let top = self.scroll_region_top;
            let bottom = self.scroll_region_bottom;
            if bottom > top && bottom < self.rows {
                let idx = self.viewport_to_absolute(top);
                if idx < self.lines.len() {
                    self.lines.remove(idx);
                    let insert_idx = self.viewport_to_absolute(bottom);
                    self.lines.insert(
                        insert_idx.min(self.lines.len()),
                        TerminalLine::new(self.cols),
                    );
                }
            }
        } else {
            self.lines.push(TerminalLine::new(self.cols));
            while self.lines.len() > self.rows + self.max_scrollback {
                self.lines.remove(0);
            }
        }
    }

    fn scroll_down_region(&mut self) {
        let bottom = self.scroll_region_bottom;
        let insert_idx = self.viewport_to_absolute(self.scroll_region_top);

        if insert_idx < self.lines.len() {
            self.lines.insert(insert_idx, TerminalLine::new(self.cols));
            let remove_idx = self.viewport_to_absolute(bottom + 1);
            if remove_idx < self.lines.len() {
                self.lines.remove(remove_idx);
            }
        }
    }

    pub fn scroll_up(&mut self) {
        self.scroll_up_region();
    }

    pub fn scroll_down(&mut self) {
        self.scroll_down_region();
    }

    pub fn scroll_up_n(&mut self, n: usize) {
        for _ in 0..n {
            self.scroll_up_region();
        }
    }

    pub fn scroll_down_n(&mut self, n: usize) {
        for _ in 0..n {
            self.scroll_down_region();
        }
    }

    pub fn move_cursor_to(&mut self, row: usize, col: usize) {
        let max_row = self.rows.saturating_sub(1);
        let max_col = self.cols.saturating_sub(1);

        if self.origin_mode {
            self.cursor.row = (self.scroll_region_top + row).min(self.scroll_region_bottom);
        } else {
            self.cursor.row = row.min(max_row);
        }
        self.cursor.col = col.min(max_col);
    }

    pub fn cursor_up(&mut self, n: usize) {
        let min_row = if self.origin_mode {
            self.scroll_region_top
        } else {
            0
        };
        self.cursor.row = self.cursor.row.saturating_sub(n).max(min_row);
    }

    pub fn cursor_down(&mut self, n: usize) {
        let max_row = if self.origin_mode {
            self.scroll_region_bottom
        } else {
            self.rows.saturating_sub(1)
        };
        self.cursor.row = (self.cursor.row + n).min(max_row);
    }

    pub fn cursor_forward(&mut self, n: usize) {
        self.cursor.col = (self.cursor.col + n).min(self.cols.saturating_sub(1));
    }

    pub fn cursor_backward(&mut self, n: usize) {
        self.cursor.col = self.cursor.col.saturating_sub(n);
    }

    pub fn cursor_to_column(&mut self, col: usize) {
        self.cursor.col = col.saturating_sub(1).min(self.cols.saturating_sub(1));
    }

    pub fn cursor_to_line(&mut self, line: usize) {
        let row = line.saturating_sub(1);
        if self.origin_mode {
            self.cursor.row = (self.scroll_region_top + row).min(self.scroll_region_bottom);
        } else {
            self.cursor.row = row.min(self.rows.saturating_sub(1));
        }
    }

    pub fn cursor_next_line(&mut self, n: usize) {
        self.cursor_down(n);
        self.cursor.col = 0;
    }

    pub fn cursor_prev_line(&mut self, n: usize) {
        self.cursor_up(n);
        self.cursor.col = 0;
    }

    pub fn save_cursor(&mut self) {
        self.saved_cursor = Some(SavedCursor {
            position: self.cursor,
            style: self.current_style.clone(),
            origin_mode: self.origin_mode,
            autowrap: self.autowrap,
        });
    }

    pub fn restore_cursor(&mut self) {
        if let Some(saved) = &self.saved_cursor {
            self.cursor = saved.position;
            self.current_style = saved.style.clone();
            self.origin_mode = saved.origin_mode;
            self.autowrap = saved.autowrap;
        }
    }

    pub fn set_scroll_region(&mut self, top: usize, bottom: usize) {
        let top = top.min(self.rows.saturating_sub(1));
        let bottom = bottom.min(self.rows.saturating_sub(1));

        if top < bottom {
            self.scroll_region_top = top;
            self.scroll_region_bottom = bottom;
            self.move_cursor_to(0, 0);
        }
    }

    pub fn reset_scroll_region(&mut self) {
        self.scroll_region_top = 0;
        self.scroll_region_bottom = self.rows.saturating_sub(1);
    }

    pub fn set_origin_mode(&mut self, enabled: bool) {
        self.origin_mode = enabled;
        self.move_cursor_to(0, 0);
    }

    pub fn set_autowrap(&mut self, enabled: bool) {
        self.autowrap = enabled;
    }

    pub fn set_insert_mode(&mut self, enabled: bool) {
        self.insert_mode = enabled;
    }

    pub fn enter_alt_screen(&mut self) {
        if self.use_alt_screen {
            return;
        }

        self.alt_screen = Some(Box::new(AltScreenState {
            lines: self.lines.clone(),
            cursor: self.cursor,
            saved_cursor: self.saved_cursor.clone(),
        }));

        self.lines.clear();
        for _ in 0..self.rows {
            self.lines.push(TerminalLine::new(self.cols));
        }
        self.cursor = CursorPosition::origin();
        self.saved_cursor = None;
        self.use_alt_screen = true;
        self.scroll_offset = 0;
    }

    pub fn exit_alt_screen(&mut self) {
        if !self.use_alt_screen {
            return;
        }

        if let Some(alt) = self.alt_screen.take() {
            self.lines = alt.lines;
            self.cursor = alt.cursor;
            self.saved_cursor = alt.saved_cursor;
        }

        self.use_alt_screen = false;
    }

    pub fn clear_screen(&mut self) {
        for line in &mut self.lines {
            line.clear_with_style(&self.current_style);
        }
        self.cursor = CursorPosition::origin();
    }

    pub fn clear_screen_above(&mut self) {
        let idx = self.viewport_to_absolute(self.cursor.row);
        let start = self.lines.len().saturating_sub(self.rows);

        for i in start..idx {
            self.lines[i].clear_with_style(&self.current_style);
        }
        if let Some(line) = self.lines.get_mut(idx) {
            line.clear_to_with_style(self.cursor.col + 1, &self.current_style);
        }
    }

    pub fn clear_screen_below(&mut self) {
        let idx = self.viewport_to_absolute(self.cursor.row);

        if let Some(line) = self.lines.get_mut(idx) {
            line.clear_from_with_style(self.cursor.col, &self.current_style);
        }
        for i in (idx + 1)..self.lines.len() {
            self.lines[i].clear_with_style(&self.current_style);
        }
    }

    pub fn clear_to_end_of_screen(&mut self) {
        self.clear_screen_below();
    }

    pub fn clear_to_start_of_screen(&mut self) {
        self.clear_screen_above();
    }

    pub fn clear_line(&mut self) {
        let idx = self.viewport_to_absolute(self.cursor.row);
        if let Some(line) = self.lines.get_mut(idx) {
            line.clear_with_style(&self.current_style);
        }
    }

    pub fn clear_to_end_of_line(&mut self) {
        let idx = self.viewport_to_absolute(self.cursor.row);
        if let Some(line) = self.lines.get_mut(idx) {
            line.clear_from_with_style(self.cursor.col, &self.current_style);
        }
    }

    pub fn clear_to_start_of_line(&mut self) {
        let idx = self.viewport_to_absolute(self.cursor.row);
        if let Some(line) = self.lines.get_mut(idx) {
            line.clear_to_with_style(self.cursor.col + 1, &self.current_style);
        }
    }

    pub fn erase_chars(&mut self, count: usize) {
        let idx = self.viewport_to_absolute(self.cursor.row);
        if let Some(line) = self.lines.get_mut(idx) {
            line.erase_chars(self.cursor.col, count, &self.current_style);
        }
    }

    pub fn insert_lines(&mut self, count: usize) {
        let row = self.cursor.row;
        if row < self.scroll_region_top || row > self.scroll_region_bottom {
            return;
        }

        for _ in 0..count {
            let insert_idx = self.viewport_to_absolute(row);
            self.lines.insert(insert_idx, TerminalLine::new(self.cols));

            let remove_idx = self.viewport_to_absolute(self.scroll_region_bottom + 1);
            if remove_idx < self.lines.len() {
                self.lines.remove(remove_idx);
            }
        }
    }

    pub fn delete_lines(&mut self, count: usize) {
        let row = self.cursor.row;
        if row < self.scroll_region_top || row > self.scroll_region_bottom {
            return;
        }

        for _ in 0..count {
            let remove_idx = self.viewport_to_absolute(row);
            if remove_idx < self.lines.len() {
                self.lines.remove(remove_idx);
            }

            let insert_idx = self.viewport_to_absolute(self.scroll_region_bottom);
            self.lines.insert(
                insert_idx.min(self.lines.len()),
                TerminalLine::new(self.cols),
            );
        }
    }

    pub fn insert_chars(&mut self, count: usize) {
        let idx = self.viewport_to_absolute(self.cursor.row);
        if let Some(line) = self.lines.get_mut(idx) {
            line.insert_cells(self.cursor.col, count);
        }
    }

    pub fn delete_chars(&mut self, count: usize) {
        let idx = self.viewport_to_absolute(self.cursor.row);
        if let Some(line) = self.lines.get_mut(idx) {
            line.delete_cells(self.cursor.col, count);
        }
    }

    pub fn scroll_viewport_up(&mut self, lines: usize) {
        let max = self.max_scroll_offset();
        self.scroll_offset = (self.scroll_offset + lines).min(max);
        self.user_scrolled = self.scroll_offset > 0;
    }

    pub fn scroll_viewport_down(&mut self, lines: usize) {
        self.scroll_offset = self.scroll_offset.saturating_sub(lines);
        if self.scroll_offset == 0 {
            self.user_scrolled = false;
        }
    }

    pub fn scroll_to_bottom(&mut self) {
        self.scroll_offset = 0;
        self.user_scrolled = false;
    }

    pub fn resize(&mut self, cols: usize, rows: usize) {
        if cols == self.cols && rows == self.rows {
            return;
        }

        self.cols = cols;
        self.rows = rows;

        for line in &mut self.lines {
            line.resize(cols);
        }

        while self.lines.len() < rows {
            self.lines.push(TerminalLine::new(cols));
        }

        self.cursor.row = self.cursor.row.min(rows.saturating_sub(1));
        self.cursor.col = self.cursor.col.min(cols.saturating_sub(1));

        self.scroll_region_top = 0;
        self.scroll_region_bottom = rows.saturating_sub(1);

        self.scroll_offset = self.scroll_offset.min(self.max_scroll_offset());

        self.tabs.clear();
        for i in (0..cols).step_by(8) {
            self.tabs.push(i);
        }
    }

    pub fn reset(&mut self) {
        self.lines.clear();
        for _ in 0..self.rows {
            self.lines.push(TerminalLine::new(self.cols));
        }
        self.cursor = CursorPosition::origin();
        self.scroll_offset = 0;
        self.current_style = CellStyle::default();
        self.cursor_visible = true;
        self.cursor_style = CursorStyle::default();
        self.saved_cursor = None;
        self.title = None;
        self.scroll_region_top = 0;
        self.scroll_region_bottom = self.rows.saturating_sub(1);
        self.origin_mode = false;
        self.autowrap = true;
        self.insert_mode = false;
        self.alt_screen = None;
        self.use_alt_screen = false;
        self.bracketed_paste = false;
        self.mouse_tracking = false;
        self.focus_tracking = false;
        self.user_scrolled = false;
    }
}
