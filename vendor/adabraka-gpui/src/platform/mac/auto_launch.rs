use anyhow::Result;
use cocoa::base::{id, nil};
use objc::{class, msg_send, sel, sel_impl};

pub fn set_auto_launch(_app_id: &str, enabled: bool) -> Result<()> {
    unsafe {
        let service: id = msg_send![class!(SMAppService), mainApp];
        if service == nil {
            return Err(anyhow::anyhow!(
                "SMAppService not available (requires macOS 13+)"
            ));
        }

        if enabled {
            let error: id = nil;
            let success: bool = msg_send![service, registerAndReturnError: &error];
            if !success {
                return Err(anyhow::anyhow!("Failed to register auto-launch"));
            }
        } else {
            let error: id = nil;
            let success: bool = msg_send![service, unregisterAndReturnError: &error];
            if !success {
                return Err(anyhow::anyhow!("Failed to unregister auto-launch"));
            }
        }

        Ok(())
    }
}

pub fn is_auto_launch_enabled(_app_id: &str) -> bool {
    unsafe {
        let service: id = msg_send![class!(SMAppService), mainApp];
        if service == nil {
            return false;
        }
        let status: isize = msg_send![service, status];
        status == 1
    }
}
