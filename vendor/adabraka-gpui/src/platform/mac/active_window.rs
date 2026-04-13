use crate::platform::FocusedWindowInfo;
use cocoa::base::{id, nil};
use core_foundation::base::TCFType;
use core_foundation::string::CFString;
use objc::{class, msg_send, sel, sel_impl};
use std::ffi::c_void;

#[link(name = "ApplicationServices", kind = "framework")]
unsafe extern "C" {
    fn AXUIElementCreateApplication(pid: i32) -> *mut c_void;
    fn AXUIElementCopyAttributeValue(
        element: *mut c_void,
        attribute: core_foundation::string::CFStringRef,
        value: *mut *mut c_void,
    ) -> i32;
}

pub fn get_focused_window_info() -> Option<FocusedWindowInfo> {
    unsafe {
        let workspace: id = msg_send![class!(NSWorkspace), sharedWorkspace];
        let frontmost_app: id = msg_send![workspace, frontmostApplication];
        if frontmost_app == nil {
            return None;
        }

        let app_name_ns: id = msg_send![frontmost_app, localizedName];
        let app_name = nsstring_to_string(app_name_ns)?;

        let bundle_id_ns: id = msg_send![frontmost_app, bundleIdentifier];
        let bundle_id = nsstring_to_string(bundle_id_ns);

        let pid: i32 = msg_send![frontmost_app, processIdentifier];

        let window_title = get_window_title_via_accessibility(pid).unwrap_or_default();

        Some(FocusedWindowInfo {
            app_name,
            window_title,
            bundle_id,
            pid: Some(pid as u32),
        })
    }
}

fn get_window_title_via_accessibility(pid: i32) -> Option<String> {
    unsafe {
        let app_element = AXUIElementCreateApplication(pid);
        if app_element.is_null() {
            return None;
        }

        let focused_window_attr = CFString::new("AXFocusedWindow");
        let mut window_value: *mut c_void = std::ptr::null_mut();
        let result = AXUIElementCopyAttributeValue(
            app_element,
            focused_window_attr.as_concrete_TypeRef(),
            &mut window_value,
        );
        core_foundation::base::CFRelease(app_element as _);

        if result != 0 || window_value.is_null() {
            return None;
        }

        let title_attr = CFString::new("AXTitle");
        let mut title_value: *mut c_void = std::ptr::null_mut();
        let result = AXUIElementCopyAttributeValue(
            window_value,
            title_attr.as_concrete_TypeRef(),
            &mut title_value,
        );
        core_foundation::base::CFRelease(window_value as _);

        if result != 0 || title_value.is_null() {
            return None;
        }

        let cf_title = core_foundation::string::CFString::wrap_under_create_rule(
            title_value as core_foundation::string::CFStringRef,
        );
        Some(cf_title.to_string())
    }
}

unsafe fn nsstring_to_string(nsstring: id) -> Option<String> {
    unsafe {
        if nsstring == nil {
            return None;
        }
        let bytes: *const std::ffi::c_char = msg_send![nsstring, UTF8String];
        if bytes.is_null() {
            return None;
        }
        Some(
            std::ffi::CStr::from_ptr(bytes)
                .to_string_lossy()
                .into_owned(),
        )
    }
}
