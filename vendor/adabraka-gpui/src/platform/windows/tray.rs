use std::collections::HashMap;

use anyhow::Result;
use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::*,
        UI::{
            Shell::{
                Shell_NotifyIconW, NIF_ICON, NIF_INFO, NIF_MESSAGE, NIF_SHOWTIP, NIF_TIP, NIM_ADD,
                NIM_DELETE, NIM_MODIFY, NOTIFYICONDATAW,
            },
            WindowsAndMessaging::*,
        },
    },
};

use crate::{SharedString, TrayMenuItem, WM_GPUI_TRAY_ICON};

const TRAY_ICON_ID: u32 = 1;

pub(crate) struct WindowsTray {
    icon_added: bool,
    hwnd: HWND,
    current_icon: Option<HICON>,
    pub(crate) menu_items: Vec<TrayMenuItem>,
    pub(crate) command_id_map: HashMap<u32, SharedString>,
}

impl WindowsTray {
    pub fn new(hwnd: HWND) -> Self {
        let mut tray = Self {
            icon_added: false,
            hwnd,
            current_icon: None,
            menu_items: Vec::new(),
            command_id_map: HashMap::new(),
        };
        tray.ensure_icon(hwnd);
        tray
    }

    fn ensure_icon(&mut self, hwnd: HWND) {
        if self.icon_added {
            return;
        }
        let mut nid = NOTIFYICONDATAW {
            cbSize: std::mem::size_of::<NOTIFYICONDATAW>() as u32,
            hWnd: hwnd,
            uID: TRAY_ICON_ID,
            uFlags: NIF_MESSAGE | NIF_SHOWTIP,
            uCallbackMessage: WM_GPUI_TRAY_ICON,
            ..Default::default()
        };
        unsafe {
            let _ = Shell_NotifyIconW(NIM_ADD, &nid);
        }
        self.icon_added = true;
    }

    pub fn set_icon(&mut self, icon_data: Option<&[u8]>, hwnd: HWND) {
        self.ensure_icon(hwnd);
        if let Some(old_icon) = self.current_icon.take() {
            unsafe {
                let _ = DestroyIcon(old_icon);
            }
        }
        let hicon = match icon_data {
            Some(data) => create_hicon_from_bytes(data),
            None => None,
        };
        self.current_icon = hicon;
        let mut nid = NOTIFYICONDATAW {
            cbSize: std::mem::size_of::<NOTIFYICONDATAW>() as u32,
            hWnd: hwnd,
            uID: TRAY_ICON_ID,
            uFlags: NIF_ICON,
            hIcon: hicon.unwrap_or_default(),
            ..Default::default()
        };
        unsafe {
            let _ = Shell_NotifyIconW(NIM_MODIFY, &nid);
        }
    }

    pub fn set_tooltip(&mut self, tooltip: &str, hwnd: HWND) {
        self.ensure_icon(hwnd);
        let mut nid = NOTIFYICONDATAW {
            cbSize: std::mem::size_of::<NOTIFYICONDATAW>() as u32,
            hWnd: hwnd,
            uID: TRAY_ICON_ID,
            uFlags: NIF_TIP,
            ..Default::default()
        };
        let wide: Vec<u16> = tooltip.encode_utf16().collect();
        let len = wide.len().min(nid.szTip.len() - 1);
        nid.szTip[..len].copy_from_slice(&wide[..len]);
        unsafe {
            let _ = Shell_NotifyIconW(NIM_MODIFY, &nid);
        }
    }

    pub fn show_balloon(&self, title: &str, body: &str, hwnd: HWND) -> Result<()> {
        let mut nid = NOTIFYICONDATAW {
            cbSize: std::mem::size_of::<NOTIFYICONDATAW>() as u32,
            hWnd: hwnd,
            uID: TRAY_ICON_ID,
            uFlags: NIF_INFO,
            ..Default::default()
        };

        let title_wide: Vec<u16> = title.encode_utf16().collect();
        let title_len = title_wide.len().min(nid.szInfoTitle.len() - 1);
        nid.szInfoTitle[..title_len].copy_from_slice(&title_wide[..title_len]);

        let body_wide: Vec<u16> = body.encode_utf16().collect();
        let body_len = body_wide.len().min(nid.szInfo.len() - 1);
        nid.szInfo[..body_len].copy_from_slice(&body_wide[..body_len]);

        unsafe {
            Shell_NotifyIconW(NIM_MODIFY, &nid)
                .ok()
                .map_err(|e| anyhow::anyhow!("Failed to show balloon notification: {}", e))
        }
    }

    pub fn show_context_menu(&mut self, hwnd: HWND) {
        if self.menu_items.is_empty() {
            return;
        }
        self.command_id_map.clear();
        unsafe {
            let hmenu = CreatePopupMenu();
            if let Ok(hmenu) = hmenu {
                let mut counter: u32 = 1;
                Self::build_menu(
                    hmenu,
                    &self.menu_items,
                    &mut counter,
                    &mut self.command_id_map,
                );
                let mut point = POINT::default();
                let _ = GetCursorPos(&mut point);
                let _ = SetForegroundWindow(hwnd);
                let _ = TrackPopupMenu(
                    hmenu,
                    TPM_LEFTALIGN | TPM_BOTTOMALIGN,
                    point.x,
                    point.y,
                    None,
                    hwnd,
                    None,
                );
                let _ = DestroyMenu(hmenu);
            }
        }
    }

    pub(crate) unsafe fn build_menu(
        hmenu: HMENU,
        items: &[TrayMenuItem],
        counter: &mut u32,
        id_map: &mut HashMap<u32, SharedString>,
    ) {
        for item in items.iter() {
            match item {
                TrayMenuItem::Action { label, id } => {
                    let cmd_id = *counter;
                    *counter += 1;
                    id_map.insert(cmd_id, id.clone());
                    let wide: Vec<u16> = label.encode_utf16().chain(Some(0)).collect();
                    unsafe {
                        let _ =
                            AppendMenuW(hmenu, MF_STRING, cmd_id as usize, PCWSTR(wide.as_ptr()));
                    }
                }
                TrayMenuItem::Separator => unsafe {
                    let _ = AppendMenuW(hmenu, MF_SEPARATOR, 0, None);
                },
                TrayMenuItem::Submenu {
                    label,
                    items: sub_items,
                } => {
                    if let Ok(submenu) = unsafe { CreatePopupMenu() } {
                        unsafe { Self::build_menu(submenu, sub_items, counter, id_map) };
                        let wide: Vec<u16> = label.encode_utf16().chain(Some(0)).collect();
                        unsafe {
                            let _ = AppendMenuW(
                                hmenu,
                                MF_POPUP,
                                submenu.0 as usize,
                                PCWSTR(wide.as_ptr()),
                            );
                        }
                    }
                }
                TrayMenuItem::Toggle {
                    label, checked, id, ..
                } => {
                    let cmd_id = *counter;
                    *counter += 1;
                    id_map.insert(cmd_id, id.clone());
                    let flags = if *checked {
                        MF_STRING | MF_CHECKED
                    } else {
                        MF_STRING
                    };
                    let wide: Vec<u16> = label.encode_utf16().chain(Some(0)).collect();
                    unsafe {
                        let _ = AppendMenuW(hmenu, flags, cmd_id as usize, PCWSTR(wide.as_ptr()));
                    }
                }
            }
        }
    }
}

impl Drop for WindowsTray {
    fn drop(&mut self) {
        if self.icon_added {
            let mut nid = NOTIFYICONDATAW {
                cbSize: std::mem::size_of::<NOTIFYICONDATAW>() as u32,
                hWnd: self.hwnd,
                uID: TRAY_ICON_ID,
                ..Default::default()
            };
            unsafe {
                let _ = Shell_NotifyIconW(NIM_DELETE, &nid);
            }
        }
        if let Some(icon) = self.current_icon.take() {
            unsafe {
                let _ = DestroyIcon(icon);
            }
        }
    }
}

fn create_hicon_from_bytes(data: &[u8]) -> Option<HICON> {
    unsafe {
        let offset = LookupIconIdFromDirectoryEx(data.as_ptr(), true, 0, 0, LR_DEFAULTCOLOR);
        if offset <= 0 {
            return None;
        }
        if (offset as usize) >= data.len() {
            return None;
        }
        let icon_data = &data[offset as usize..];
        let hicon = CreateIconFromResourceEx(icon_data, true, 0x00030000, 0, 0, LR_DEFAULTCOLOR);
        hicon.ok()
    }
}
