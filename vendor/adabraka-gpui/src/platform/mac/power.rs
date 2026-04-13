use core_foundation::base::TCFType;
use core_foundation::string::CFString;
use std::ffi::c_void;

#[link(name = "IOKit", kind = "framework")]
unsafe extern "C" {
    fn IOPMAssertionCreateWithName(
        assertion_type: *const c_void,
        assertion_level: u32,
        reason_for_activity: *const c_void,
        assertion_id: *mut u32,
    ) -> i32;
    fn IOPMAssertionRelease(assertion_id: u32) -> i32;
}

const IOPM_ASSERTION_LEVEL_ON: u32 = 255;

pub fn start_power_save_blocker(kind: crate::PowerSaveBlockerKind) -> Option<u32> {
    unsafe {
        let assertion_type = match kind {
            crate::PowerSaveBlockerKind::PreventAppSuspension => {
                CFString::new("PreventUserIdleSystemSleep")
            }
            crate::PowerSaveBlockerKind::PreventDisplaySleep => {
                CFString::new("PreventUserIdleDisplaySleep")
            }
        };
        let reason = CFString::new("Application requested power save blocker");
        let mut assertion_id: u32 = 0;

        let result = IOPMAssertionCreateWithName(
            assertion_type.as_concrete_TypeRef() as *const c_void,
            IOPM_ASSERTION_LEVEL_ON,
            reason.as_concrete_TypeRef() as *const c_void,
            &mut assertion_id,
        );

        if result == 0 {
            Some(assertion_id)
        } else {
            None
        }
    }
}

pub fn stop_power_save_blocker(id: u32) {
    unsafe {
        IOPMAssertionRelease(id);
    }
}

pub fn system_idle_time() -> Option<std::time::Duration> {
    unsafe {
        let seconds = CGEventSourceSecondsSinceLastEventType(0, u32::MAX);
        if seconds >= 0.0 {
            Some(std::time::Duration::from_secs_f64(seconds))
        } else {
            None
        }
    }
}

#[link(name = "ApplicationServices", kind = "framework")]
unsafe extern "C" {
    fn CGEventSourceSecondsSinceLastEventType(source_state: u32, event_type: u32) -> f64;
}
