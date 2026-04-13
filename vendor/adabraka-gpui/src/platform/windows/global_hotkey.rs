use anyhow::Result;
use windows::Win32::{
    Foundation::HWND,
    UI::Input::KeyboardAndMouse::{
        HOT_KEY_MODIFIERS, MOD_ALT, MOD_CONTROL, MOD_SHIFT, MOD_WIN, RegisterHotKey,
        UnregisterHotKey, VIRTUAL_KEY, VK_BACK, VK_DELETE, VK_DOWN, VK_END, VK_ESCAPE, VK_F1,
        VK_F2, VK_F3, VK_F4, VK_F5, VK_F6, VK_F7, VK_F8, VK_F9, VK_F10, VK_F11, VK_F12, VK_HOME,
        VK_INSERT, VK_LEFT, VK_NEXT, VK_PRIOR, VK_RETURN, VK_RIGHT, VK_SPACE, VK_TAB, VK_UP,
    },
};

use crate::Keystroke;

pub(crate) fn register(hwnd: HWND, id: u32, keystroke: &Keystroke) -> Result<()> {
    let modifiers = map_modifiers(keystroke);
    let vk = map_key_to_vk(&keystroke.key)?;

    unsafe {
        RegisterHotKey(Some(hwnd), id as i32, modifiers, vk.0 as u32)
            .map_err(|e| anyhow::anyhow!("Failed to register hotkey {}: {}", id, e))
    }
}

pub(crate) fn unregister(hwnd: HWND, id: u32) {
    unsafe {
        let _ = UnregisterHotKey(Some(hwnd), id as i32);
    }
}

fn map_modifiers(keystroke: &Keystroke) -> HOT_KEY_MODIFIERS {
    let mut mods = HOT_KEY_MODIFIERS(0);
    if keystroke.modifiers.alt {
        mods |= MOD_ALT;
    }
    if keystroke.modifiers.control {
        mods |= MOD_CONTROL;
    }
    if keystroke.modifiers.shift {
        mods |= MOD_SHIFT;
    }
    if keystroke.modifiers.platform {
        mods |= MOD_WIN;
    }
    mods
}

fn map_key_to_vk(key: &str) -> Result<VIRTUAL_KEY> {
    let vk = match key.to_lowercase().as_str() {
        "a" => VIRTUAL_KEY(0x41),
        "b" => VIRTUAL_KEY(0x42),
        "c" => VIRTUAL_KEY(0x43),
        "d" => VIRTUAL_KEY(0x44),
        "e" => VIRTUAL_KEY(0x45),
        "f" => VIRTUAL_KEY(0x46),
        "g" => VIRTUAL_KEY(0x47),
        "h" => VIRTUAL_KEY(0x48),
        "i" => VIRTUAL_KEY(0x49),
        "j" => VIRTUAL_KEY(0x4A),
        "k" => VIRTUAL_KEY(0x4B),
        "l" => VIRTUAL_KEY(0x4C),
        "m" => VIRTUAL_KEY(0x4D),
        "n" => VIRTUAL_KEY(0x4E),
        "o" => VIRTUAL_KEY(0x4F),
        "p" => VIRTUAL_KEY(0x50),
        "q" => VIRTUAL_KEY(0x51),
        "r" => VIRTUAL_KEY(0x52),
        "s" => VIRTUAL_KEY(0x53),
        "t" => VIRTUAL_KEY(0x54),
        "u" => VIRTUAL_KEY(0x55),
        "v" => VIRTUAL_KEY(0x56),
        "w" => VIRTUAL_KEY(0x57),
        "x" => VIRTUAL_KEY(0x58),
        "y" => VIRTUAL_KEY(0x59),
        "z" => VIRTUAL_KEY(0x5A),
        "0" => VIRTUAL_KEY(0x30),
        "1" => VIRTUAL_KEY(0x31),
        "2" => VIRTUAL_KEY(0x32),
        "3" => VIRTUAL_KEY(0x33),
        "4" => VIRTUAL_KEY(0x34),
        "5" => VIRTUAL_KEY(0x35),
        "6" => VIRTUAL_KEY(0x36),
        "7" => VIRTUAL_KEY(0x37),
        "8" => VIRTUAL_KEY(0x38),
        "9" => VIRTUAL_KEY(0x39),
        "f1" => VK_F1,
        "f2" => VK_F2,
        "f3" => VK_F3,
        "f4" => VK_F4,
        "f5" => VK_F5,
        "f6" => VK_F6,
        "f7" => VK_F7,
        "f8" => VK_F8,
        "f9" => VK_F9,
        "f10" => VK_F10,
        "f11" => VK_F11,
        "f12" => VK_F12,
        "space" | " " => VK_SPACE,
        "enter" | "return" => VK_RETURN,
        "tab" => VK_TAB,
        "escape" | "esc" => VK_ESCAPE,
        "backspace" => VK_BACK,
        "delete" => VK_DELETE,
        "insert" => VK_INSERT,
        "home" => VK_HOME,
        "end" => VK_END,
        "pageup" => VK_PRIOR,
        "pagedown" => VK_NEXT,
        "up" => VK_UP,
        "down" => VK_DOWN,
        "left" => VK_LEFT,
        "right" => VK_RIGHT,
        other => {
            if other.len() == 1 {
                let ch = other.chars().next().unwrap().to_ascii_uppercase();
                VIRTUAL_KEY(ch as u16)
            } else {
                return Err(anyhow::anyhow!(
                    "Unsupported key for global hotkey: {}",
                    other
                ));
            }
        }
    };
    Ok(vk)
}
