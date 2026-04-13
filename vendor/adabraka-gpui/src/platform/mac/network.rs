use crate::NetworkStatus;
use std::ffi::c_void;

#[link(name = "SystemConfiguration", kind = "framework")]
unsafe extern "C" {
    fn SCNetworkReachabilityCreateWithName(
        allocator: *const c_void,
        nodename: *const u8,
    ) -> *const c_void;
    fn SCNetworkReachabilityGetFlags(target: *const c_void, flags: *mut u32) -> u8;
}

#[link(name = "Network", kind = "framework")]
unsafe extern "C" {
    fn nw_path_monitor_create() -> *const c_void;
    fn nw_path_monitor_set_update_handler(monitor: *const c_void, handler: *const c_void);
    fn nw_path_monitor_set_queue(monitor: *const c_void, queue: *const c_void);
    fn nw_path_monitor_start(monitor: *const c_void);
    fn nw_path_monitor_cancel(monitor: *const c_void);
    fn nw_path_get_status(path: *const c_void) -> i32;
}

const K_SC_NETWORK_REACHABILITY_FLAGS_REACHABLE: u32 = 1 << 1;
const NW_PATH_STATUS_SATISFIED: i32 = 1;

pub(crate) fn network_status() -> NetworkStatus {
    unsafe {
        let host = b"captive.apple.com\0";
        let reachability = SCNetworkReachabilityCreateWithName(std::ptr::null(), host.as_ptr());
        if reachability.is_null() {
            return NetworkStatus::Offline;
        }

        let mut flags: u32 = 0;
        let ok = SCNetworkReachabilityGetFlags(reachability, &mut flags);

        core_foundation::base::CFRelease(reachability as core_foundation::base::CFTypeRef);

        if ok != 0 && (flags & K_SC_NETWORK_REACHABILITY_FLAGS_REACHABLE) != 0 {
            NetworkStatus::Online
        } else {
            NetworkStatus::Offline
        }
    }
}

pub(crate) fn path_status_to_network_status(path: *const c_void) -> NetworkStatus {
    let status = unsafe { nw_path_get_status(path) };
    if status == NW_PATH_STATUS_SATISFIED {
        NetworkStatus::Online
    } else {
        NetworkStatus::Offline
    }
}

pub(crate) unsafe fn create_path_monitor() -> *const c_void {
    unsafe { nw_path_monitor_create() }
}

pub(crate) unsafe fn start_path_monitor(
    monitor: *const c_void,
    handler_block: *const c_void,
    queue: *const c_void,
) {
    unsafe {
        nw_path_monitor_set_update_handler(monitor, handler_block);
        nw_path_monitor_set_queue(monitor, queue);
        nw_path_monitor_start(monitor);
    }
}

pub(crate) unsafe fn cancel_path_monitor(monitor: *const c_void) {
    if !monitor.is_null() {
        unsafe {
            nw_path_monitor_cancel(monitor);
        }
    }
}
