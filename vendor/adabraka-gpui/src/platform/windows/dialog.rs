use windows::Win32::{Foundation::HWND, UI::WindowsAndMessaging::*};

use crate::DialogOptions;

pub(crate) fn show_dialog_sync(hwnd: HWND, options: DialogOptions) -> usize {
    let kind_flag = match options.kind {
        crate::DialogKind::Info => MB_ICONINFORMATION,
        crate::DialogKind::Warning => MB_ICONWARNING,
        crate::DialogKind::Error => MB_ICONERROR,
    };

    let button_flag = match options.buttons.len() {
        0 | 1 => MB_OK,
        2 => MB_OKCANCEL,
        _ => MB_YESNOCANCEL,
    };

    let mut message = options.message.to_string();
    if let Some(ref detail) = options.detail {
        message.push_str("\n\n");
        message.push_str(detail);
    }

    let title_wide: Vec<u16> = options.title.encode_utf16().chain(Some(0)).collect();
    let message_wide: Vec<u16> = message.encode_utf16().chain(Some(0)).collect();

    let result = unsafe {
        MessageBoxW(
            Some(hwnd),
            windows::core::PCWSTR(message_wide.as_ptr()),
            windows::core::PCWSTR(title_wide.as_ptr()),
            MESSAGEBOX_STYLE(kind_flag.0 | button_flag.0),
        )
    };

    match result.0 {
        1 => 0,
        2 => {
            if options.buttons.len() >= 2 {
                options.buttons.len() - 1
            } else {
                0
            }
        }
        6 => 0,
        7 => 1,
        _ => 0,
    }
}
