use crate::AttentionType;
use cocoa::base::{id, nil};
use cocoa::foundation::NSString;
use objc::{class, msg_send, sel, sel_impl};

const NS_INFORMATIONAL_REQUEST: isize = 10;
const NS_CRITICAL_REQUEST: isize = 0;

pub fn request_user_attention(attention_type: AttentionType) -> isize {
    unsafe {
        let app: id = msg_send![class!(NSApplication), sharedApplication];
        let request_type = match attention_type {
            AttentionType::Informational => NS_INFORMATIONAL_REQUEST,
            AttentionType::Critical => NS_CRITICAL_REQUEST,
        };
        msg_send![app, requestUserAttention: request_type]
    }
}

pub fn cancel_user_attention(request_id: isize) {
    unsafe {
        let app: id = msg_send![class!(NSApplication), sharedApplication];
        let _: () = msg_send![app, cancelUserAttentionRequest: request_id];
    }
}

pub fn set_dock_badge(label: Option<&str>) {
    unsafe {
        let app: id = msg_send![class!(NSApplication), sharedApplication];
        let dock_tile: id = msg_send![app, dockTile];
        let ns_label: id = match label {
            Some(text) => {
                let s: id = NSString::alloc(nil).init_str(text);
                let _: id = msg_send![s, autorelease];
                s
            }
            None => nil,
        };
        let _: () = msg_send![dock_tile, setBadgeLabel: ns_label];
    }
}
