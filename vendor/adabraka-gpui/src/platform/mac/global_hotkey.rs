use crate::Keystroke;
use cocoa::{
    appkit::{NSEvent, NSEventModifierFlags, NSEventType},
    base::{id, nil},
};
use objc::{msg_send, sel, sel_impl};
use std::collections::HashMap;

pub(crate) fn keystroke_matches_event(keystroke: &Keystroke, event: id) -> bool {
    unsafe {
        let event_type = event.eventType();
        if event_type != NSEventType::NSKeyDown {
            return false;
        }

        let modifiers = event.modifierFlags();
        let event_cmd = modifiers.contains(NSEventModifierFlags::NSCommandKeyMask);
        let event_ctrl = modifiers.contains(NSEventModifierFlags::NSControlKeyMask);
        let event_alt = modifiers.contains(NSEventModifierFlags::NSAlternateKeyMask);
        let event_shift = modifiers.contains(NSEventModifierFlags::NSShiftKeyMask);

        if keystroke.modifiers.platform != event_cmd
            || keystroke.modifiers.control != event_ctrl
            || keystroke.modifiers.alt != event_alt
            || keystroke.modifiers.shift != event_shift
        {
            return false;
        }

        let chars_ignoring: id = msg_send![event, charactersIgnoringModifiers];
        if chars_ignoring == nil {
            return false;
        }
        let chars_str: *const std::ffi::c_char = msg_send![chars_ignoring, UTF8String];
        if chars_str.is_null() {
            return false;
        }
        let chars = std::ffi::CStr::from_ptr(chars_str).to_str().unwrap_or("");

        let event_key = chars.to_lowercase();
        let target_key = keystroke.key.to_lowercase();

        event_key == target_key || normalize_key_name(&event_key) == target_key
    }
}

fn normalize_key_name(char_key: &str) -> &str {
    match char_key {
        " " => "space",
        "\t" => "tab",
        "\r" | "\n" => "enter",
        "\u{1b}" => "escape",
        "\u{7f}" => "backspace",
        "\u{f700}" => "up",
        "\u{f701}" => "down",
        "\u{f702}" => "left",
        "\u{f703}" => "right",
        "\u{f728}" => "delete",
        "\u{f729}" => "home",
        "\u{f72b}" => "end",
        "\u{f72c}" => "pageup",
        "\u{f72d}" => "pagedown",
        other => other,
    }
}

pub(crate) fn find_matching_hotkey(
    registrations: &HashMap<u32, Keystroke>,
    event: id,
) -> Option<u32> {
    for (id, keystroke) in registrations {
        if keystroke_matches_event(keystroke, event) {
            return Some(*id);
        }
    }
    None
}
