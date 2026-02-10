use crate::terminal_state::CellStyle;
use gpui::Rgba;

pub const ANSI_COLORS: [Rgba; 16] = [
    Rgba {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    },
    Rgba {
        r: 0.8,
        g: 0.2,
        b: 0.2,
        a: 1.0,
    },
    Rgba {
        r: 0.2,
        g: 0.8,
        b: 0.2,
        a: 1.0,
    },
    Rgba {
        r: 0.8,
        g: 0.8,
        b: 0.2,
        a: 1.0,
    },
    Rgba {
        r: 0.2,
        g: 0.4,
        b: 0.8,
        a: 1.0,
    },
    Rgba {
        r: 0.8,
        g: 0.2,
        b: 0.8,
        a: 1.0,
    },
    Rgba {
        r: 0.2,
        g: 0.8,
        b: 0.8,
        a: 1.0,
    },
    Rgba {
        r: 0.8,
        g: 0.8,
        b: 0.8,
        a: 1.0,
    },
    Rgba {
        r: 0.4,
        g: 0.4,
        b: 0.4,
        a: 1.0,
    },
    Rgba {
        r: 1.0,
        g: 0.4,
        b: 0.4,
        a: 1.0,
    },
    Rgba {
        r: 0.4,
        g: 1.0,
        b: 0.4,
        a: 1.0,
    },
    Rgba {
        r: 1.0,
        g: 1.0,
        b: 0.4,
        a: 1.0,
    },
    Rgba {
        r: 0.4,
        g: 0.6,
        b: 1.0,
        a: 1.0,
    },
    Rgba {
        r: 1.0,
        g: 0.4,
        b: 1.0,
        a: 1.0,
    },
    Rgba {
        r: 0.4,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    },
    Rgba {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    },
];

pub const DEFAULT_FG: Rgba = Rgba {
    r: 0.93,
    g: 0.93,
    b: 0.93,
    a: 1.0,
};
pub const DEFAULT_BG: Rgba = Rgba {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 0.0,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ParserState {
    Ground,
    Escape,
    EscapeIntermediate,
    CsiEntry,
    CsiParam,
    CsiIntermediate,
    CsiPrivate,
    OscString,
    DcsEntry,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ParsedSegment {
    Text(String, CellStyle),
    CursorUp(usize),
    CursorDown(usize),
    CursorForward(usize),
    CursorBackward(usize),
    CursorPosition(usize, usize),
    CursorToColumn(usize),
    CursorNextLine(usize),
    CursorPrevLine(usize),
    CursorSave,
    CursorRestore,
    CursorVisible(bool),
    CursorStyle(u8),
    ClearScreen(ClearMode),
    ClearLine(ClearMode),
    EraseChars(usize),
    InsertLines(usize),
    DeleteLines(usize),
    InsertChars(usize),
    DeleteChars(usize),
    ScrollUp(usize),
    ScrollDown(usize),
    SetScrollRegion(usize, usize),
    ResetScrollRegion,
    SetTitle(String),
    Bell,
    Backspace,
    Tab,
    LineFeed,
    CarriageReturn,
    ReverseIndex,
    AltScreenEnter,
    AltScreenExit,
    BracketedPasteMode(bool),
    MouseTracking(bool),
    FocusTracking(bool),
    OriginMode(bool),
    AutoWrap(bool),
    InsertMode(bool),
    Reset,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClearMode {
    ToEnd,
    ToStart,
    All,
    Scrollback,
}

impl ClearMode {
    fn from_param(param: usize) -> Self {
        match param {
            0 => ClearMode::ToEnd,
            1 => ClearMode::ToStart,
            2 => ClearMode::All,
            3 => ClearMode::Scrollback,
            _ => ClearMode::ToEnd,
        }
    }
}

pub struct AnsiParser {
    state: ParserState,
    params: Vec<u16>,
    intermediate: Vec<u8>,
    osc_string: String,
    current_style: CellStyle,
    default_fg: Rgba,
    default_bg: Rgba,
    utf8_buffer: Vec<u8>,
    utf8_remaining: usize,
    private_marker: Option<u8>,
}

impl Default for AnsiParser {
    fn default() -> Self {
        Self::new()
    }
}

impl AnsiParser {
    pub fn new() -> Self {
        Self {
            state: ParserState::Ground,
            params: Vec::with_capacity(16),
            intermediate: Vec::with_capacity(4),
            osc_string: String::new(),
            current_style: CellStyle::default(),
            default_fg: DEFAULT_FG,
            default_bg: DEFAULT_BG,
            utf8_buffer: Vec::with_capacity(4),
            utf8_remaining: 0,
            private_marker: None,
        }
    }

    pub fn reset(&mut self) {
        self.state = ParserState::Ground;
        self.params.clear();
        self.intermediate.clear();
        self.osc_string.clear();
        self.utf8_buffer.clear();
        self.utf8_remaining = 0;
        self.private_marker = None;
        self.current_style = CellStyle {
            foreground: self.default_fg,
            background: self.default_bg,
            ..CellStyle::default()
        };
    }

    pub fn parse(&mut self, input: &[u8]) -> Vec<ParsedSegment> {
        let mut segments = Vec::new();
        let mut text_buffer = String::new();

        for &byte in input {
            match self.state {
                ParserState::Ground => {
                    self.handle_ground(byte, &mut text_buffer, &mut segments);
                }
                ParserState::Escape => {
                    self.handle_escape(byte, &mut text_buffer, &mut segments);
                }
                ParserState::EscapeIntermediate => {
                    self.handle_escape_intermediate(byte, &mut text_buffer, &mut segments);
                }
                ParserState::CsiEntry | ParserState::CsiParam => {
                    self.handle_csi(byte, &mut text_buffer, &mut segments);
                }
                ParserState::CsiIntermediate => {
                    self.handle_csi_intermediate(byte, &mut text_buffer, &mut segments);
                }
                ParserState::CsiPrivate => {
                    self.handle_csi_private(byte, &mut text_buffer, &mut segments);
                }
                ParserState::OscString => {
                    self.handle_osc(byte, &mut segments);
                }
                ParserState::DcsEntry => {
                    self.handle_dcs(byte);
                }
            }
        }

        if !text_buffer.is_empty() {
            segments.push(ParsedSegment::Text(
                std::mem::take(&mut text_buffer),
                self.current_style.clone(),
            ));
        }

        segments
    }

    fn flush_text(&self, text_buffer: &mut String, segments: &mut Vec<ParsedSegment>) {
        if !text_buffer.is_empty() {
            segments.push(ParsedSegment::Text(
                std::mem::take(text_buffer),
                self.current_style.clone(),
            ));
        }
    }

    fn handle_ground(
        &mut self,
        byte: u8,
        text_buffer: &mut String,
        segments: &mut Vec<ParsedSegment>,
    ) {
        if self.utf8_remaining > 0 {
            if (byte & 0xC0) == 0x80 {
                self.utf8_buffer.push(byte);
                self.utf8_remaining -= 1;
                if self.utf8_remaining == 0 {
                    if let Ok(s) = std::str::from_utf8(&self.utf8_buffer) {
                        text_buffer.push_str(s);
                    }
                    self.utf8_buffer.clear();
                }
                return;
            } else {
                self.utf8_buffer.clear();
                self.utf8_remaining = 0;
            }
        }

        match byte {
            0x1B => {
                self.flush_text(text_buffer, segments);
                self.state = ParserState::Escape;
            }
            0x07 => {
                self.flush_text(text_buffer, segments);
                segments.push(ParsedSegment::Bell);
            }
            0x08 => {
                self.flush_text(text_buffer, segments);
                segments.push(ParsedSegment::Backspace);
            }
            0x09 => {
                self.flush_text(text_buffer, segments);
                segments.push(ParsedSegment::Tab);
            }
            0x0A | 0x0B | 0x0C => {
                self.flush_text(text_buffer, segments);
                segments.push(ParsedSegment::LineFeed);
            }
            0x0D => {
                self.flush_text(text_buffer, segments);
                segments.push(ParsedSegment::CarriageReturn);
            }
            0x00..=0x1F => {}
            0x20..=0x7F => {
                text_buffer.push(byte as char);
            }
            0xC0..=0xDF => {
                self.utf8_buffer.clear();
                self.utf8_buffer.push(byte);
                self.utf8_remaining = 1;
            }
            0xE0..=0xEF => {
                self.utf8_buffer.clear();
                self.utf8_buffer.push(byte);
                self.utf8_remaining = 2;
            }
            0xF0..=0xF7 => {
                self.utf8_buffer.clear();
                self.utf8_buffer.push(byte);
                self.utf8_remaining = 3;
            }
            _ => {}
        }
    }

    fn handle_escape(
        &mut self,
        byte: u8,
        _text_buffer: &mut String,
        segments: &mut Vec<ParsedSegment>,
    ) {
        match byte {
            b'[' => {
                self.state = ParserState::CsiEntry;
                self.params.clear();
                self.intermediate.clear();
                self.private_marker = None;
            }
            b']' => {
                self.state = ParserState::OscString;
                self.osc_string.clear();
            }
            b'P' => {
                self.state = ParserState::DcsEntry;
            }
            b'7' => {
                segments.push(ParsedSegment::CursorSave);
                self.state = ParserState::Ground;
            }
            b'8' => {
                segments.push(ParsedSegment::CursorRestore);
                self.state = ParserState::Ground;
            }
            b'D' => {
                segments.push(ParsedSegment::LineFeed);
                self.state = ParserState::Ground;
            }
            b'E' => {
                segments.push(ParsedSegment::LineFeed);
                segments.push(ParsedSegment::CarriageReturn);
                self.state = ParserState::Ground;
            }
            b'M' => {
                segments.push(ParsedSegment::ReverseIndex);
                self.state = ParserState::Ground;
            }
            b'c' => {
                segments.push(ParsedSegment::Reset);
                self.reset();
                self.state = ParserState::Ground;
            }
            b' '..=b'/' => {
                self.intermediate.push(byte);
                self.state = ParserState::EscapeIntermediate;
            }
            _ => {
                self.state = ParserState::Ground;
            }
        }
    }

    fn handle_escape_intermediate(
        &mut self,
        byte: u8,
        _text_buffer: &mut String,
        _segments: &mut Vec<ParsedSegment>,
    ) {
        match byte {
            b' '..=b'/' => {
                self.intermediate.push(byte);
            }
            0x30..=0x7E => {
                self.state = ParserState::Ground;
            }
            _ => {
                self.state = ParserState::Ground;
            }
        }
    }

    fn handle_csi(
        &mut self,
        byte: u8,
        _text_buffer: &mut String,
        segments: &mut Vec<ParsedSegment>,
    ) {
        match byte {
            b'?' | b'>' | b'=' | b'!' | b'<' => {
                self.private_marker = Some(byte);
                self.state = ParserState::CsiPrivate;
            }
            b'0'..=b'9' => {
                self.state = ParserState::CsiParam;
                let digit = (byte - b'0') as u16;
                if let Some(last) = self.params.last_mut() {
                    *last = last.saturating_mul(10).saturating_add(digit);
                } else {
                    self.params.push(digit);
                }
            }
            b';' | b':' => {
                self.state = ParserState::CsiParam;
                if self.params.is_empty() {
                    self.params.push(0);
                }
                self.params.push(0);
            }
            b' '..=b'/' => {
                self.intermediate.push(byte);
                self.state = ParserState::CsiIntermediate;
            }
            b'@'..=b'~' => {
                self.execute_csi(byte, segments);
                self.state = ParserState::Ground;
            }
            _ => {
                self.state = ParserState::Ground;
            }
        }
    }

    fn handle_csi_private(
        &mut self,
        byte: u8,
        _text_buffer: &mut String,
        segments: &mut Vec<ParsedSegment>,
    ) {
        match byte {
            b'0'..=b'9' => {
                let digit = (byte - b'0') as u16;
                if let Some(last) = self.params.last_mut() {
                    *last = last.saturating_mul(10).saturating_add(digit);
                } else {
                    self.params.push(digit);
                }
            }
            b';' => {
                if self.params.is_empty() {
                    self.params.push(0);
                }
                self.params.push(0);
            }
            b'@'..=b'~' => {
                self.execute_private_mode(byte, segments);
                self.state = ParserState::Ground;
            }
            _ => {
                self.state = ParserState::Ground;
            }
        }
    }

    fn handle_csi_intermediate(
        &mut self,
        byte: u8,
        _text_buffer: &mut String,
        segments: &mut Vec<ParsedSegment>,
    ) {
        match byte {
            b' '..=b'/' => {
                self.intermediate.push(byte);
            }
            b'@'..=b'~' => {
                if !self.intermediate.is_empty() && self.intermediate[0] == b' ' {
                    if byte == b'q' {
                        let style = self.params.first().copied().unwrap_or(0) as u8;
                        segments.push(ParsedSegment::CursorStyle(style));
                    }
                }
                self.state = ParserState::Ground;
            }
            _ => {
                self.state = ParserState::Ground;
            }
        }
    }

    fn handle_osc(&mut self, byte: u8, segments: &mut Vec<ParsedSegment>) {
        match byte {
            0x07 => {
                self.execute_osc(segments);
                self.state = ParserState::Ground;
            }
            0x1B => {
                self.state = ParserState::Ground;
                self.execute_osc(segments);
            }
            0x9C => {
                self.execute_osc(segments);
                self.state = ParserState::Ground;
            }
            _ => {
                if byte >= 0x20 && byte <= 0x7E {
                    self.osc_string.push(byte as char);
                }
            }
        }
    }

    fn handle_dcs(&mut self, byte: u8) {
        match byte {
            0x1B | 0x9C => {
                self.state = ParserState::Ground;
            }
            _ => {}
        }
    }

    fn execute_osc(&mut self, segments: &mut Vec<ParsedSegment>) {
        if let Some(idx) = self.osc_string.find(';') {
            let cmd = &self.osc_string[..idx];
            let arg = &self.osc_string[idx + 1..];
            match cmd {
                "0" | "1" | "2" => {
                    segments.push(ParsedSegment::SetTitle(arg.to_string()));
                }
                _ => {}
            }
        }
        self.osc_string.clear();
    }

    fn execute_csi(&mut self, final_byte: u8, segments: &mut Vec<ParsedSegment>) {
        let param_or = |params: &[u16], idx: usize, default: usize| -> usize {
            let val = params.get(idx).copied().unwrap_or(0) as usize;
            if val == 0 {
                default
            } else {
                val
            }
        };

        match final_byte {
            b'A' => {
                segments.push(ParsedSegment::CursorUp(param_or(&self.params, 0, 1)));
            }
            b'B' | b'e' => {
                segments.push(ParsedSegment::CursorDown(param_or(&self.params, 0, 1)));
            }
            b'C' | b'a' => {
                segments.push(ParsedSegment::CursorForward(param_or(&self.params, 0, 1)));
            }
            b'D' => {
                segments.push(ParsedSegment::CursorBackward(param_or(&self.params, 0, 1)));
            }
            b'E' => {
                segments.push(ParsedSegment::CursorNextLine(param_or(&self.params, 0, 1)));
            }
            b'F' => {
                segments.push(ParsedSegment::CursorPrevLine(param_or(&self.params, 0, 1)));
            }
            b'G' | b'`' => {
                segments.push(ParsedSegment::CursorToColumn(param_or(&self.params, 0, 1)));
            }
            b'H' | b'f' => {
                let row = param_or(&self.params, 0, 1).saturating_sub(1);
                let col = param_or(&self.params, 1, 1).saturating_sub(1);
                segments.push(ParsedSegment::CursorPosition(row, col));
            }
            b'd' => {
                let row = param_or(&self.params, 0, 1);
                segments.push(ParsedSegment::CursorPosition(row.saturating_sub(1), 0));
            }
            b'J' => {
                let mode = ClearMode::from_param(param_or(&self.params, 0, 0));
                segments.push(ParsedSegment::ClearScreen(mode));
            }
            b'K' => {
                let mode = ClearMode::from_param(param_or(&self.params, 0, 0));
                segments.push(ParsedSegment::ClearLine(mode));
            }
            b'L' => {
                segments.push(ParsedSegment::InsertLines(param_or(&self.params, 0, 1)));
            }
            b'M' => {
                segments.push(ParsedSegment::DeleteLines(param_or(&self.params, 0, 1)));
            }
            b'@' => {
                segments.push(ParsedSegment::InsertChars(param_or(&self.params, 0, 1)));
            }
            b'P' => {
                segments.push(ParsedSegment::DeleteChars(param_or(&self.params, 0, 1)));
            }
            b'X' => {
                segments.push(ParsedSegment::EraseChars(param_or(&self.params, 0, 1)));
            }
            b'S' => {
                segments.push(ParsedSegment::ScrollUp(param_or(&self.params, 0, 1)));
            }
            b'T' => {
                segments.push(ParsedSegment::ScrollDown(param_or(&self.params, 0, 1)));
            }
            b'r' => {
                let top = param_or(&self.params, 0, 1);
                let bottom = param_or(&self.params, 1, 0);
                if bottom == 0 {
                    segments.push(ParsedSegment::ResetScrollRegion);
                } else {
                    segments.push(ParsedSegment::SetScrollRegion(
                        top.saturating_sub(1),
                        bottom.saturating_sub(1),
                    ));
                }
            }
            b'm' => {
                self.execute_sgr();
            }
            b's' => {
                segments.push(ParsedSegment::CursorSave);
            }
            b'u' => {
                segments.push(ParsedSegment::CursorRestore);
            }
            b'n' => {}
            b't' => {}
            _ => {}
        }
    }

    fn execute_private_mode(&mut self, final_byte: u8, segments: &mut Vec<ParsedSegment>) {
        let enabled = final_byte == b'h';

        for &param in &self.params {
            match param {
                1 => {}
                6 => {
                    segments.push(ParsedSegment::OriginMode(enabled));
                }
                7 => {
                    segments.push(ParsedSegment::AutoWrap(enabled));
                }
                12 => {}
                25 => {
                    segments.push(ParsedSegment::CursorVisible(enabled));
                }
                47 | 1047 => {
                    if enabled {
                        segments.push(ParsedSegment::AltScreenEnter);
                    } else {
                        segments.push(ParsedSegment::AltScreenExit);
                    }
                }
                1000 | 1002 | 1003 | 1006 | 1015 => {
                    segments.push(ParsedSegment::MouseTracking(enabled));
                }
                1004 => {
                    segments.push(ParsedSegment::FocusTracking(enabled));
                }
                1049 => {
                    if enabled {
                        segments.push(ParsedSegment::CursorSave);
                        segments.push(ParsedSegment::AltScreenEnter);
                        segments.push(ParsedSegment::ClearScreen(ClearMode::All));
                    } else {
                        segments.push(ParsedSegment::AltScreenExit);
                        segments.push(ParsedSegment::CursorRestore);
                    }
                }
                2004 => {
                    segments.push(ParsedSegment::BracketedPasteMode(enabled));
                }
                _ => {}
            }
        }
    }

    fn execute_sgr(&mut self) {
        if self.params.is_empty() {
            self.params.push(0);
        }

        let mut i = 0;
        while i < self.params.len() {
            let code = self.params[i] as usize;
            match code {
                0 => {
                    self.current_style = CellStyle {
                        foreground: self.default_fg,
                        background: self.default_bg,
                        ..CellStyle::default()
                    };
                }
                1 => self.current_style.bold = true,
                2 => self.current_style.dim = true,
                3 => self.current_style.italic = true,
                4 => self.current_style.underline = true,
                5 | 6 => self.current_style.blink = true,
                7 => self.current_style.inverse = true,
                8 => self.current_style.hidden = true,
                9 => self.current_style.strikethrough = true,
                21 => self.current_style.bold = false,
                22 => {
                    self.current_style.bold = false;
                    self.current_style.dim = false;
                }
                23 => self.current_style.italic = false,
                24 => self.current_style.underline = false,
                25 => self.current_style.blink = false,
                27 => self.current_style.inverse = false,
                28 => self.current_style.hidden = false,
                29 => self.current_style.strikethrough = false,
                30..=37 => {
                    self.current_style.foreground = ANSI_COLORS[code - 30];
                }
                38 => {
                    if let Some(color) = self.parse_extended_color(&mut i) {
                        self.current_style.foreground = color;
                    }
                }
                39 => {
                    self.current_style.foreground = self.default_fg;
                }
                40..=47 => {
                    self.current_style.background = ANSI_COLORS[code - 40];
                }
                48 => {
                    if let Some(color) = self.parse_extended_color(&mut i) {
                        self.current_style.background = color;
                    }
                }
                49 => {
                    self.current_style.background = self.default_bg;
                }
                90..=97 => {
                    self.current_style.foreground = ANSI_COLORS[code - 90 + 8];
                }
                100..=107 => {
                    self.current_style.background = ANSI_COLORS[code - 100 + 8];
                }
                _ => {}
            }
            i += 1;
        }
    }

    fn parse_extended_color(&self, i: &mut usize) -> Option<Rgba> {
        if *i + 1 >= self.params.len() {
            return None;
        }

        let mode = self.params[*i + 1];
        match mode {
            2 => {
                if *i + 4 >= self.params.len() {
                    return None;
                }
                let r = self.params[*i + 2] as f32 / 255.0;
                let g = self.params[*i + 3] as f32 / 255.0;
                let b = self.params[*i + 4] as f32 / 255.0;
                *i += 4;
                Some(Rgba { r, g, b, a: 1.0 })
            }
            5 => {
                if *i + 2 >= self.params.len() {
                    return None;
                }
                let n = self.params[*i + 2] as usize;
                *i += 2;
                Some(color_from_256(n))
            }
            _ => None,
        }
    }
}

pub fn color_from_256(n: usize) -> Rgba {
    match n {
        0..=15 => ANSI_COLORS[n],
        16..=231 => {
            let n = n - 16;
            let r = (n / 36) % 6;
            let g = (n / 6) % 6;
            let b = n % 6;
            Rgba {
                r: if r == 0 {
                    0.0
                } else {
                    (r * 40 + 55) as f32 / 255.0
                },
                g: if g == 0 {
                    0.0
                } else {
                    (g * 40 + 55) as f32 / 255.0
                },
                b: if b == 0 {
                    0.0
                } else {
                    (b * 40 + 55) as f32 / 255.0
                },
                a: 1.0,
            }
        }
        232..=255 => {
            let gray = ((n - 232) * 10 + 8) as f32 / 255.0;
            Rgba {
                r: gray,
                g: gray,
                b: gray,
                a: 1.0,
            }
        }
        _ => DEFAULT_FG,
    }
}
