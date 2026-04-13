use std::collections::HashMap;
use std::time::Duration;

use windows::Win32::{
    System::Power::{
        SetThreadExecutionState, ES_CONTINUOUS, ES_DISPLAY_REQUIRED,
        ES_SYSTEM_REQUIRED, EXECUTION_STATE,
    },
    System::SystemInformation::GetTickCount,
    UI::Input::KeyboardAndMouse::{GetLastInputInfo, LASTINPUTINFO},
};

use crate::PowerSaveBlockerKind;

pub(crate) fn power_save_flags(kind: PowerSaveBlockerKind) -> EXECUTION_STATE {
    match kind {
        PowerSaveBlockerKind::PreventAppSuspension => ES_CONTINUOUS | ES_SYSTEM_REQUIRED,
        PowerSaveBlockerKind::PreventDisplaySleep => {
            ES_CONTINUOUS | ES_SYSTEM_REQUIRED | ES_DISPLAY_REQUIRED
        }
    }
}

pub(crate) fn apply_combined_power_state(blockers: &HashMap<u32, EXECUTION_STATE>) {
    let combined = blockers
        .values()
        .fold(ES_CONTINUOUS, |acc, &flags| acc | flags);
    unsafe {
        SetThreadExecutionState(combined);
    }
}

pub(crate) fn system_idle_time() -> Option<Duration> {
    let mut info = LASTINPUTINFO {
        cbSize: std::mem::size_of::<LASTINPUTINFO>() as u32,
        dwTime: 0,
    };
    let success = unsafe { GetLastInputInfo(&mut info) };
    if success.as_bool() {
        let now = unsafe { GetTickCount() };
        let idle_ms = now.wrapping_sub(info.dwTime);
        Some(Duration::from_millis(idle_ms as u64))
    } else {
        None
    }
}
