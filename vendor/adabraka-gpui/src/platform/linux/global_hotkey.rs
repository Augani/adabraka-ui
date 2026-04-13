#![allow(dead_code)]

use anyhow::Result;
use collections::HashMap;

use crate::Keystroke;

pub struct LinuxGlobalHotkey {
    registered: HashMap<u32, Keystroke>,
}

impl LinuxGlobalHotkey {
    pub fn new() -> Self {
        Self {
            registered: HashMap::default(),
        }
    }

    pub fn register(&mut self, id: u32, keystroke: &Keystroke) -> Result<()> {
        self.registered.insert(id, keystroke.clone());
        Ok(())
    }

    pub fn unregister(&mut self, id: u32) {
        self.registered.remove(&id);
    }
}

#[cfg(feature = "x11")]
pub mod x11 {
    use super::*;
    use std::rc::Rc;
    use x11rb::connection::Connection as _;
    use x11rb::protocol::xproto::{self, ConnectionExt as _, GrabMode, ModMask};
    use x11rb::xcb_ffi::XCBConnection;

    fn keystroke_to_x11_modmask(keystroke: &Keystroke) -> u16 {
        let mut mask = 0u16;
        if keystroke.modifiers.control {
            mask |= u16::from(ModMask::CONTROL);
        }
        if keystroke.modifiers.alt {
            mask |= u16::from(ModMask::M1);
        }
        if keystroke.modifiers.shift {
            mask |= u16::from(ModMask::SHIFT);
        }
        if keystroke.modifiers.platform {
            mask |= u16::from(ModMask::M4);
        }
        mask
    }

    fn keystroke_to_x11_keycode(keystroke: &Keystroke, xcb: &XCBConnection) -> Option<u8> {
        let setup = xcb.setup();
        let min_keycode = setup.min_keycode;
        let max_keycode = setup.max_keycode;

        let keysym = key_name_to_keysym(&keystroke.key)?;

        let reply = xcb
            .get_keyboard_mapping(min_keycode, max_keycode - min_keycode + 1)
            .ok()?
            .reply()
            .ok()?;

        let keysyms_per_keycode = reply.keysyms_per_keycode as usize;
        if keysyms_per_keycode == 0 {
            return None;
        }

        for i in 0..((max_keycode - min_keycode + 1) as usize) {
            let base = i * keysyms_per_keycode;
            for j in 0..keysyms_per_keycode {
                if reply.keysyms[base + j] == keysym {
                    return Some(min_keycode + i as u8);
                }
            }
        }

        None
    }

    fn key_name_to_keysym(key: &str) -> Option<u32> {
        match key.to_lowercase().as_str() {
            "a" => Some(0x61),
            "b" => Some(0x62),
            "c" => Some(0x63),
            "d" => Some(0x64),
            "e" => Some(0x65),
            "f" => Some(0x66),
            "g" => Some(0x67),
            "h" => Some(0x68),
            "i" => Some(0x69),
            "j" => Some(0x6a),
            "k" => Some(0x6b),
            "l" => Some(0x6c),
            "m" => Some(0x6d),
            "n" => Some(0x6e),
            "o" => Some(0x6f),
            "p" => Some(0x70),
            "q" => Some(0x71),
            "r" => Some(0x72),
            "s" => Some(0x73),
            "t" => Some(0x74),
            "u" => Some(0x75),
            "v" => Some(0x76),
            "w" => Some(0x77),
            "x" => Some(0x78),
            "y" => Some(0x79),
            "z" => Some(0x7a),
            "0" => Some(0x30),
            "1" => Some(0x31),
            "2" => Some(0x32),
            "3" => Some(0x33),
            "4" => Some(0x34),
            "5" => Some(0x35),
            "6" => Some(0x36),
            "7" => Some(0x37),
            "8" => Some(0x38),
            "9" => Some(0x39),
            "space" => Some(0x20),
            "enter" | "return" => Some(0xff0d),
            "tab" => Some(0xff09),
            "escape" => Some(0xff1b),
            "backspace" => Some(0xff08),
            "delete" => Some(0xffff),
            "insert" => Some(0xff63),
            "home" => Some(0xff50),
            "end" => Some(0xff57),
            "pageup" => Some(0xff55),
            "pagedown" => Some(0xff56),
            "left" => Some(0xff51),
            "up" => Some(0xff52),
            "right" => Some(0xff53),
            "down" => Some(0xff54),
            "f1" => Some(0xffbe),
            "f2" => Some(0xffbf),
            "f3" => Some(0xffc0),
            "f4" => Some(0xffc1),
            "f5" => Some(0xffc2),
            "f6" => Some(0xffc3),
            "f7" => Some(0xffc4),
            "f8" => Some(0xffc5),
            "f9" => Some(0xffc6),
            "f10" => Some(0xffc7),
            "f11" => Some(0xffc8),
            "f12" => Some(0xffc9),
            "-" => Some(0x2d),
            "=" => Some(0x3d),
            "[" => Some(0x5b),
            "]" => Some(0x5d),
            "\\" => Some(0x5c),
            ";" => Some(0x3b),
            "'" => Some(0x27),
            "`" => Some(0x60),
            "," => Some(0x2c),
            "." => Some(0x2e),
            "/" => Some(0x2f),
            _ => None,
        }
    }

    pub struct X11GlobalHotkey {
        inner: LinuxGlobalHotkey,
        keycodes: HashMap<u32, (u8, u16)>,
    }

    impl X11GlobalHotkey {
        pub fn new() -> Self {
            Self {
                inner: LinuxGlobalHotkey::new(),
                keycodes: HashMap::default(),
            }
        }

        pub fn register(
            &mut self,
            id: u32,
            keystroke: &Keystroke,
            xcb: &Rc<XCBConnection>,
            root_window: xproto::Window,
        ) -> Result<()> {
            let keycode = keystroke_to_x11_keycode(keystroke, xcb).ok_or_else(|| {
                anyhow::anyhow!("Could not resolve keycode for key: {}", keystroke.key)
            })?;
            let modmask = keystroke_to_x11_modmask(keystroke);

            xcb.grab_key(
                false,
                root_window,
                modmask.into(),
                keycode,
                GrabMode::ASYNC,
                GrabMode::ASYNC,
            )?
            .check()?;

            self.keycodes.insert(id, (keycode, modmask));
            self.inner.register(id, keystroke)
        }

        pub fn unregister(
            &mut self,
            id: u32,
            xcb: &Rc<XCBConnection>,
            root_window: xproto::Window,
        ) {
            if let Some((keycode, modmask)) = self.keycodes.remove(&id) {
                let _ = xcb.ungrab_key(keycode, root_window, modmask.into());
            }
            self.inner.unregister(id);
        }
    }
}

#[cfg(feature = "wayland")]
pub mod wayland {
    use super::*;

    pub struct WaylandGlobalHotkey {
        _inner: LinuxGlobalHotkey,
    }

    impl WaylandGlobalHotkey {
        pub fn new() -> Self {
            Self {
                _inner: LinuxGlobalHotkey::new(),
            }
        }

        pub fn register(&mut self, _id: u32, _keystroke: &Keystroke) -> Result<()> {
            Err(anyhow::anyhow!("Global hotkeys not supported on Wayland"))
        }

        pub fn unregister(&mut self, _id: u32) {}
    }
}
