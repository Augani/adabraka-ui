use crate::terminal_state::{CellStyle, UnderlineStyle};
use gpui::Rgba;

#[derive(Clone, Debug, PartialEq)]
pub struct InlineImageData {
    pub data: Vec<u8>,
    pub width: ImageDimension,
    pub height: ImageDimension,
    pub preserve_aspect: bool,
    pub source_width: Option<u32>,
    pub source_height: Option<u32>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ImageDimension {
    Auto,
    Cells(usize),
    Pixels(u32),
    Percent(u8),
}

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
    ApcString,
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
    MouseTracking(u16, bool),
    FocusTracking(bool),
    OriginMode(bool),
    AutoWrap(bool),
    InsertMode(bool),
    ApplicationCursorKeys(bool),
    SetG0Charset(u8),
    SetG1Charset(u8),
    ShiftIn,
    ShiftOut,
    SyncUpdate(bool),
    SetHyperlink(Option<String>),
    SetClipboard(String),
    Notification(String, Option<String>),
    InlineImage(InlineImageData),
    DeviceAttributes(u8),
    CursorPositionReport,
    SetWorkingDirectory(String),
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
    is_sub_param: Vec<bool>,
    intermediate: Vec<u8>,
    osc_string: String,
    current_style: CellStyle,
    default_fg: Rgba,
    default_bg: Rgba,
    utf8_buffer: Vec<u8>,
    utf8_remaining: usize,
    private_marker: Option<u8>,
    apc_string: Vec<u8>,
    kitty_chunks: Vec<u8>,
    kitty_params: String,
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
            is_sub_param: Vec::with_capacity(16),
            intermediate: Vec::with_capacity(4),
            osc_string: String::new(),
            current_style: CellStyle::default(),
            default_fg: DEFAULT_FG,
            default_bg: DEFAULT_BG,
            utf8_buffer: Vec::with_capacity(4),
            utf8_remaining: 0,
            private_marker: None,
            apc_string: Vec::new(),
            kitty_chunks: Vec::new(),
            kitty_params: String::new(),
        }
    }

    pub fn reset(&mut self) {
        self.state = ParserState::Ground;
        self.params.clear();
        self.is_sub_param.clear();
        self.intermediate.clear();
        self.osc_string.clear();
        self.utf8_buffer.clear();
        self.utf8_remaining = 0;
        self.private_marker = None;
        self.apc_string.clear();
        self.kitty_chunks.clear();
        self.kitty_params.clear();
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
                ParserState::ApcString => {
                    self.handle_apc(byte, &mut segments);
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
            0x0E => {
                self.flush_text(text_buffer, segments);
                segments.push(ParsedSegment::ShiftOut);
            }
            0x0F => {
                self.flush_text(text_buffer, segments);
                segments.push(ParsedSegment::ShiftIn);
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
                self.is_sub_param.clear();
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
            b'_' => {
                self.state = ParserState::ApcString;
                self.apc_string.clear();
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
        segments: &mut Vec<ParsedSegment>,
    ) {
        match byte {
            b' '..=b'/' => {
                self.intermediate.push(byte);
            }
            0x30..=0x7E => {
                if !self.intermediate.is_empty() {
                    let designator = self.intermediate[0];
                    match designator {
                        b'(' => {
                            segments.push(ParsedSegment::SetG0Charset(byte));
                        }
                        b')' => {
                            segments.push(ParsedSegment::SetG1Charset(byte));
                        }
                        _ => {}
                    }
                }
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
                    self.is_sub_param.push(false);
                }
            }
            b';' => {
                self.state = ParserState::CsiParam;
                if self.params.is_empty() {
                    self.params.push(0);
                    self.is_sub_param.push(false);
                }
                self.params.push(0);
                self.is_sub_param.push(false);
            }
            b':' => {
                self.state = ParserState::CsiParam;
                if self.params.is_empty() {
                    self.params.push(0);
                    self.is_sub_param.push(false);
                }
                self.params.push(0);
                self.is_sub_param.push(true);
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

    fn handle_apc(&mut self, byte: u8, segments: &mut Vec<ParsedSegment>) {
        match byte {
            0x07 => {
                self.execute_apc(segments);
                self.state = ParserState::Ground;
            }
            0x1B => {
                self.execute_apc(segments);
                self.state = ParserState::Escape;
            }
            0x9C => {
                self.execute_apc(segments);
                self.state = ParserState::Ground;
            }
            _ => {
                if self.apc_string.len() < 16 * 1024 * 1024 {
                    self.apc_string.push(byte);
                }
            }
        }
    }

    fn execute_apc(&mut self, segments: &mut Vec<ParsedSegment>) {
        if self.apc_string.is_empty() || self.apc_string[0] != b'G' {
            self.apc_string.clear();
            return;
        }

        let payload = std::mem::take(&mut self.apc_string);
        let payload = &payload[1..];

        let semi_pos = payload.iter().position(|&b| b == b';');
        let (params_str, base64_data) = if let Some(pos) = semi_pos {
            (
                std::str::from_utf8(&payload[..pos]).unwrap_or(""),
                &payload[pos + 1..],
            )
        } else {
            (std::str::from_utf8(payload).unwrap_or(""), &[] as &[u8])
        };

        let mut action = b't';
        let mut format: u32 = 32;
        let mut more_chunks: u32 = 0;
        let mut source_width: u32 = 0;
        let mut source_height: u32 = 0;
        let mut display_cols: usize = 0;
        let mut display_rows: usize = 0;

        for kv in params_str.split(',') {
            if let Some((key, val)) = kv.split_once('=') {
                match key {
                    "a" => {
                        if let Some(b) = val.bytes().next() {
                            action = b;
                        }
                    }
                    "f" => format = val.parse().unwrap_or(32),
                    "m" => more_chunks = val.parse().unwrap_or(0),
                    "s" => source_width = val.parse().unwrap_or(0),
                    "v" => source_height = val.parse().unwrap_or(0),
                    "c" => display_cols = val.parse().unwrap_or(0),
                    "r" => display_rows = val.parse().unwrap_or(0),
                    _ => {}
                }
            }
        }

        if action != b't' && action != b'T' && action != b'p' {
            self.kitty_chunks.clear();
            self.kitty_params.clear();
            return;
        }

        let decoded = if let Ok(d) = base64_decode(std::str::from_utf8(base64_data).unwrap_or("")) {
            d
        } else {
            self.kitty_chunks.clear();
            self.kitty_params.clear();
            return;
        };

        if more_chunks == 1 {
            if self.kitty_chunks.is_empty() {
                self.kitty_params = params_str.to_string();
            }
            self.kitty_chunks.extend_from_slice(&decoded);
            return;
        }

        let final_data = if !self.kitty_chunks.is_empty() {
            self.kitty_chunks.extend_from_slice(&decoded);

            if !self.kitty_params.is_empty() {
                for kv in self.kitty_params.split(',') {
                    if let Some((key, val)) = kv.split_once('=') {
                        match key {
                            "f" => format = val.parse().unwrap_or(format),
                            "s" => source_width = val.parse().unwrap_or(source_width),
                            "v" => source_height = val.parse().unwrap_or(source_height),
                            "c" => display_cols = val.parse().unwrap_or(display_cols),
                            "r" => display_rows = val.parse().unwrap_or(display_rows),
                            _ => {}
                        }
                    }
                }
            }

            std::mem::take(&mut self.kitty_chunks)
        } else {
            decoded
        };
        self.kitty_params.clear();

        let image_data = match format {
            100 => final_data,
            32 => {
                if source_width == 0 || source_height == 0 {
                    return;
                }
                match encode_rgba_as_png(&final_data, source_width, source_height) {
                    Some(png) => png,
                    None => return,
                }
            }
            24 => {
                if source_width == 0 || source_height == 0 {
                    return;
                }
                match encode_rgb_as_png(&final_data, source_width, source_height) {
                    Some(png) => png,
                    None => return,
                }
            }
            _ => return,
        };

        let width = if display_cols > 0 {
            ImageDimension::Cells(display_cols)
        } else {
            ImageDimension::Auto
        };
        let height = if display_rows > 0 {
            ImageDimension::Cells(display_rows)
        } else {
            ImageDimension::Auto
        };

        segments.push(ParsedSegment::InlineImage(InlineImageData {
            data: image_data,
            width,
            height,
            preserve_aspect: true,
            source_width: if source_width > 0 {
                Some(source_width)
            } else {
                None
            },
            source_height: if source_height > 0 {
                Some(source_height)
            } else {
                None
            },
        }));
    }

    fn execute_osc(&mut self, segments: &mut Vec<ParsedSegment>) {
        if let Some(idx) = self.osc_string.find(';') {
            let cmd = &self.osc_string[..idx];
            let arg = &self.osc_string[idx + 1..];
            match cmd {
                "0" | "1" | "2" => {
                    segments.push(ParsedSegment::SetTitle(arg.to_string()));
                }
                "7" => {
                    let path = if let Some(stripped) = arg.strip_prefix("file://") {
                        if let Some(slash) = stripped.find('/') {
                            &stripped[slash..]
                        } else {
                            stripped
                        }
                    } else {
                        arg
                    };
                    if !path.is_empty() {
                        segments.push(ParsedSegment::SetWorkingDirectory(path.to_string()));
                    }
                }
                "8" => {
                    if let Some(url_start) = arg.find(';') {
                        let url = &arg[url_start + 1..];
                        if url.is_empty() {
                            segments.push(ParsedSegment::SetHyperlink(None));
                        } else {
                            segments.push(ParsedSegment::SetHyperlink(Some(url.to_string())));
                        }
                    }
                }
                "9" => {
                    segments.push(ParsedSegment::Notification(arg.to_string(), None));
                }
                "52" => {
                    if let Some(data_start) = arg.find(';') {
                        let base64_data = &arg[data_start + 1..];
                        if !base64_data.is_empty() && base64_data != "?" {
                            if let Ok(decoded) = base64_decode(base64_data) {
                                if let Ok(text) = String::from_utf8(decoded) {
                                    segments.push(ParsedSegment::SetClipboard(text));
                                }
                            }
                        }
                    }
                }
                "777" => {
                    let parts: Vec<&str> = arg.splitn(3, ';').collect();
                    if parts.len() >= 2 && parts[0] == "notify" {
                        let title = parts[1].to_string();
                        let body = parts.get(2).map(|s| s.to_string());
                        segments.push(ParsedSegment::Notification(title, body));
                    }
                }
                "1337" => {
                    self.parse_iterm2_image(arg, segments);
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
            b'c' => {
                segments.push(ParsedSegment::DeviceAttributes(0));
            }
            b'n' => {
                let p = param_or(&self.params, 0, 0);
                if p == 6 {
                    segments.push(ParsedSegment::CursorPositionReport);
                }
            }
            b't' => {}
            b'q' => {
                // DECSCUSR - Set Cursor Style (ignore, handled via private modes)
            }
            _ => {}
        }
    }

    fn execute_private_mode(&mut self, final_byte: u8, segments: &mut Vec<ParsedSegment>) {
        if final_byte == b'c' {
            if self.private_marker == Some(b'>') {
                segments.push(ParsedSegment::DeviceAttributes(1));
            }
            return;
        }

        let enabled = final_byte == b'h';

        for &param in &self.params {
            match param {
                1 => {
                    segments.push(ParsedSegment::ApplicationCursorKeys(enabled));
                }
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
                    segments.push(ParsedSegment::MouseTracking(param, enabled));
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
                2026 => {
                    segments.push(ParsedSegment::SyncUpdate(enabled));
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
                        underline_style: UnderlineStyle::None,
                        underline_color: None,
                        ..CellStyle::default()
                    };
                }
                1 => self.current_style.bold = true,
                2 => self.current_style.dim = true,
                3 => self.current_style.italic = true,
                4 => {
                    if i + 1 < self.params.len()
                        && self.is_sub_param.get(i + 1).copied().unwrap_or(false)
                    {
                        let sub = self.params[i + 1];
                        i += 1;
                        match sub {
                            0 => {
                                self.current_style.underline = false;
                                self.current_style.underline_style = UnderlineStyle::None;
                            }
                            1 => {
                                self.current_style.underline = true;
                                self.current_style.underline_style = UnderlineStyle::Single;
                            }
                            2 => {
                                self.current_style.underline = true;
                                self.current_style.underline_style = UnderlineStyle::Double;
                            }
                            3 => {
                                self.current_style.underline = true;
                                self.current_style.underline_style = UnderlineStyle::Curly;
                            }
                            4 => {
                                self.current_style.underline = true;
                                self.current_style.underline_style = UnderlineStyle::Dotted;
                            }
                            5 => {
                                self.current_style.underline = true;
                                self.current_style.underline_style = UnderlineStyle::Dashed;
                            }
                            _ => {
                                self.current_style.underline = true;
                                self.current_style.underline_style = UnderlineStyle::Single;
                            }
                        }
                    } else {
                        self.current_style.underline = true;
                        self.current_style.underline_style = UnderlineStyle::Single;
                    }
                }
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
                24 => {
                    self.current_style.underline = false;
                    self.current_style.underline_style = UnderlineStyle::None;
                    self.current_style.underline_color = None;
                }
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
                58 => {
                    if let Some(color) = self.parse_extended_color(&mut i) {
                        self.current_style.underline_color = Some(color);
                    }
                }
                59 => {
                    self.current_style.underline_color = None;
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

    fn parse_iterm2_image(&self, arg: &str, segments: &mut Vec<ParsedSegment>) {
        if !arg.starts_with("File=") {
            return;
        }

        let rest = &arg[5..];
        let colon_pos = match rest.find(':') {
            Some(p) => p,
            None => return,
        };

        let params_str = &rest[..colon_pos];
        let base64_data = &rest[colon_pos + 1..];

        let mut is_inline = false;
        let mut width = ImageDimension::Auto;
        let mut height = ImageDimension::Auto;
        let mut preserve_aspect = true;

        for kv in params_str.split(';') {
            if let Some((key, val)) = kv.split_once('=') {
                match key {
                    "inline" => is_inline = val == "1",
                    "width" => width = parse_iterm2_dimension(val),
                    "height" => height = parse_iterm2_dimension(val),
                    "preserveAspectRatio" => preserve_aspect = val != "0",
                    _ => {}
                }
            }
        }

        if !is_inline {
            return;
        }

        let decoded = match base64_decode(base64_data) {
            Ok(d) if !d.is_empty() => d,
            _ => return,
        };

        segments.push(ParsedSegment::InlineImage(InlineImageData {
            data: decoded,
            width,
            height,
            preserve_aspect,
            source_width: None,
            source_height: None,
        }));
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

fn parse_iterm2_dimension(val: &str) -> ImageDimension {
    if val == "auto" || val.is_empty() {
        return ImageDimension::Auto;
    }
    if let Some(stripped) = val.strip_suffix("px") {
        if let Ok(n) = stripped.parse::<u32>() {
            return ImageDimension::Pixels(n);
        }
    }
    if let Some(stripped) = val.strip_suffix('%') {
        if let Ok(n) = stripped.parse::<u8>() {
            return ImageDimension::Percent(n);
        }
    }
    if let Ok(n) = val.parse::<usize>() {
        return ImageDimension::Cells(n);
    }
    ImageDimension::Auto
}

fn encode_rgba_as_png(data: &[u8], width: u32, height: u32) -> Option<Vec<u8>> {
    let expected = (width * height * 4) as usize;
    if data.len() < expected {
        return None;
    }
    let buf = image::ImageBuffer::<image::Rgba<u8>, _>::from_raw(width, height, &data[..expected])?;
    let mut output = Vec::new();
    let encoder = image::codecs::png::PngEncoder::new(&mut output);
    image::ImageEncoder::write_image(
        encoder,
        buf.as_raw(),
        width,
        height,
        image::ExtendedColorType::Rgba8,
    )
    .ok()?;
    Some(output)
}

fn encode_rgb_as_png(data: &[u8], width: u32, height: u32) -> Option<Vec<u8>> {
    let expected = (width * height * 3) as usize;
    if data.len() < expected {
        return None;
    }
    let buf = image::ImageBuffer::<image::Rgb<u8>, _>::from_raw(width, height, &data[..expected])?;
    let mut output = Vec::new();
    let encoder = image::codecs::png::PngEncoder::new(&mut output);
    image::ImageEncoder::write_image(
        encoder,
        buf.as_raw(),
        width,
        height,
        image::ExtendedColorType::Rgb8,
    )
    .ok()?;
    Some(output)
}

fn base64_decode(input: &str) -> Result<Vec<u8>, ()> {
    const DECODE_TABLE: [i8; 128] = [
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 62, -1, -1,
        -1, 63, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, -1, -1, -1, -1, -1, -1, -1, 0, 1, 2, 3, 4,
        5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, -1, -1, -1,
        -1, -1, -1, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45,
        46, 47, 48, 49, 50, 51, -1, -1, -1, -1, -1,
    ];

    let input = input.trim_end_matches('=');
    let mut output = Vec::with_capacity(input.len() * 3 / 4);
    let mut buffer = 0u32;
    let mut bits = 0;

    for &byte in input.as_bytes() {
        if byte >= 128 {
            return Err(());
        }
        let val = DECODE_TABLE[byte as usize];
        if val < 0 {
            continue;
        }
        buffer = (buffer << 6) | (val as u32);
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            output.push((buffer >> bits) as u8);
            buffer &= (1 << bits) - 1;
        }
    }
    Ok(output)
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

#[cfg(test)]
mod tests {
    use super::*;

    fn base64_encode(data: &[u8]) -> String {
        const TABLE: &[u8; 64] =
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut out = String::new();
        for chunk in data.chunks(3) {
            let b0 = chunk[0] as u32;
            let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
            let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
            let n = (b0 << 16) | (b1 << 8) | b2;
            out.push(TABLE[((n >> 18) & 63) as usize] as char);
            out.push(TABLE[((n >> 12) & 63) as usize] as char);
            if chunk.len() > 1 {
                out.push(TABLE[((n >> 6) & 63) as usize] as char);
            } else {
                out.push('=');
            }
            if chunk.len() > 2 {
                out.push(TABLE[(n & 63) as usize] as char);
            } else {
                out.push('=');
            }
        }
        out
    }

    fn minimal_png() -> Vec<u8> {
        let mut png = Vec::new();
        png.extend_from_slice(&[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A]);
        png.extend_from_slice(&[0, 0, 0, 13]); // IHDR length
        png.extend_from_slice(b"IHDR");
        png.extend_from_slice(&[0, 0, 0, 2]); // width=2
        png.extend_from_slice(&[0, 0, 0, 2]); // height=2
        png.push(8); // bit depth
        png.push(2); // color type RGB
        png.extend_from_slice(&[0, 0, 0]); // compression, filter, interlace
        png.extend_from_slice(&[0, 0, 0, 0]); // CRC placeholder
        png.extend_from_slice(&[0, 0, 0, 0]); // IEND length
        png.extend_from_slice(b"IEND");
        png.extend_from_slice(&[0, 0, 0, 0]); // CRC placeholder
        png
    }

    #[test]
    fn test_iterm2_inline_image() {
        let mut parser = AnsiParser::new();
        let png_data = minimal_png();
        let b64 = base64_encode(&png_data);
        let seq = format!("\x1b]1337;File=inline=1:{}\x07", b64);

        let segments = parser.parse(seq.as_bytes());

        let img = segments
            .iter()
            .find_map(|s| {
                if let ParsedSegment::InlineImage(d) = s {
                    Some(d)
                } else {
                    None
                }
            })
            .expect("should produce InlineImage segment");

        assert_eq!(img.data, png_data);
        assert_eq!(img.width, ImageDimension::Auto);
        assert_eq!(img.height, ImageDimension::Auto);
        assert!(img.preserve_aspect);
    }

    #[test]
    fn test_iterm2_with_dimensions() {
        let mut parser = AnsiParser::new();
        let png_data = minimal_png();
        let b64 = base64_encode(&png_data);
        let seq = format!(
            "\x1b]1337;File=inline=1;width=20;height=10px;preserveAspectRatio=0:{}\x07",
            b64
        );

        let segments = parser.parse(seq.as_bytes());
        let img = segments
            .iter()
            .find_map(|s| {
                if let ParsedSegment::InlineImage(d) = s {
                    Some(d)
                } else {
                    None
                }
            })
            .expect("should produce InlineImage");

        assert_eq!(img.width, ImageDimension::Cells(20));
        assert_eq!(img.height, ImageDimension::Pixels(10));
        assert!(!img.preserve_aspect);
    }

    #[test]
    fn test_iterm2_non_inline_ignored() {
        let mut parser = AnsiParser::new();
        let b64 = base64_encode(&minimal_png());
        let seq = format!("\x1b]1337;File=inline=0:{}\x07", b64);

        let segments = parser.parse(seq.as_bytes());
        assert!(!segments
            .iter()
            .any(|s| matches!(s, ParsedSegment::InlineImage(_))));
    }

    #[test]
    fn test_iterm2_percent_dimension() {
        assert_eq!(parse_iterm2_dimension("50%"), ImageDimension::Percent(50));
        assert_eq!(parse_iterm2_dimension("100px"), ImageDimension::Pixels(100));
        assert_eq!(parse_iterm2_dimension("auto"), ImageDimension::Auto);
        assert_eq!(parse_iterm2_dimension("15"), ImageDimension::Cells(15));
        assert_eq!(parse_iterm2_dimension(""), ImageDimension::Auto);
    }

    #[test]
    fn test_kitty_single_chunk_png() {
        let mut parser = AnsiParser::new();
        let png_data = minimal_png();
        let b64 = base64_encode(&png_data);
        let seq = format!("\x1b_Ga=T,f=100;{}\x1b\\", b64);

        let segments = parser.parse(seq.as_bytes());
        let img = segments
            .iter()
            .find_map(|s| {
                if let ParsedSegment::InlineImage(d) = s {
                    Some(d)
                } else {
                    None
                }
            })
            .expect("should produce InlineImage from Kitty");

        assert_eq!(img.data, png_data);
    }

    #[test]
    fn test_kitty_with_cell_size() {
        let mut parser = AnsiParser::new();
        let png_data = minimal_png();
        let b64 = base64_encode(&png_data);
        let seq = format!("\x1b_Ga=T,f=100,c=10,r=5;{}\x1b\\", b64);

        let segments = parser.parse(seq.as_bytes());
        let img = segments
            .iter()
            .find_map(|s| {
                if let ParsedSegment::InlineImage(d) = s {
                    Some(d)
                } else {
                    None
                }
            })
            .expect("should produce InlineImage");

        assert_eq!(img.width, ImageDimension::Cells(10));
        assert_eq!(img.height, ImageDimension::Cells(5));
    }

    #[test]
    fn test_kitty_chunked_transfer() {
        let mut parser = AnsiParser::new();
        let png_data = minimal_png();
        let b64 = base64_encode(&png_data);

        let mid = (b64.len() / 2) & !3;
        let chunk1 = &b64[..mid];
        let chunk2 = &b64[mid..];

        let seq1 = format!("\x1b_Ga=T,f=100,m=1;{}\x1b\\", chunk1);
        let segments1 = parser.parse(seq1.as_bytes());
        assert!(
            !segments1
                .iter()
                .any(|s| matches!(s, ParsedSegment::InlineImage(_))),
            "first chunk should not emit image"
        );

        let seq2 = format!("\x1b_Gm=0;{}\x1b\\", chunk2);
        let segments2 = parser.parse(seq2.as_bytes());
        let img = segments2
            .iter()
            .find_map(|s| {
                if let ParsedSegment::InlineImage(d) = s {
                    Some(d)
                } else {
                    None
                }
            })
            .expect("final chunk should emit InlineImage");

        assert_eq!(img.data, png_data);
    }

    #[test]
    fn test_kitty_unsupported_action_ignored() {
        let mut parser = AnsiParser::new();
        let b64 = base64_encode(&minimal_png());
        let seq = format!("\x1b_Ga=d,f=100;{}\x1b\\", b64);

        let segments = parser.parse(seq.as_bytes());
        assert!(!segments
            .iter()
            .any(|s| matches!(s, ParsedSegment::InlineImage(_))));
    }

    #[test]
    fn test_base64_roundtrip() {
        let data = b"Hello, World!";
        let encoded = base64_encode(data);
        let decoded = base64_decode(&encoded).unwrap();
        assert_eq!(decoded, data);
    }
}
