use crate::{DialogKind, DialogOptions};
use cocoa::base::{id, nil};
use cocoa::foundation::{NSAutoreleasePool, NSString};
use futures::channel::oneshot;
use objc::{class, msg_send, sel, sel_impl};

const NS_ALERT_STYLE_WARNING: u64 = 0;
const NS_ALERT_STYLE_INFORMATIONAL: u64 = 1;
const NS_ALERT_STYLE_CRITICAL: u64 = 2;
const NS_ALERT_FIRST_BUTTON_RETURN: i64 = 1000;

pub fn show_dialog(options: DialogOptions) -> oneshot::Receiver<usize> {
    let (tx, rx) = oneshot::channel();
    unsafe {
        let pool = NSAutoreleasePool::new(nil);

        let alert: id = msg_send![class!(NSAlert), new];
        let style = match options.kind {
            DialogKind::Info => NS_ALERT_STYLE_INFORMATIONAL,
            DialogKind::Warning => NS_ALERT_STYLE_WARNING,
            DialogKind::Error => NS_ALERT_STYLE_CRITICAL,
        };
        let _: () = msg_send![alert, setAlertStyle: style];

        let title = NSString::alloc(nil).init_str(options.title.as_ref());
        let _: id = msg_send![title, autorelease];
        let _: () = msg_send![alert, setMessageText: title];

        let informative = if let Some(detail) = &options.detail {
            format!("{}\n\n{}", options.message.as_ref(), detail.as_ref())
        } else {
            options.message.as_ref().to_string()
        };
        let message = NSString::alloc(nil).init_str(&informative);
        let _: id = msg_send![message, autorelease];
        let _: () = msg_send![alert, setInformativeText: message];

        for button_label in &options.buttons {
            let label = NSString::alloc(nil).init_str(button_label.as_ref());
            let _: id = msg_send![label, autorelease];
            let _: () = msg_send![alert, addButtonWithTitle: label];
        }
        if options.buttons.is_empty() {
            let ok_label = NSString::alloc(nil).init_str("OK");
            let _: id = msg_send![ok_label, autorelease];
            let _: () = msg_send![alert, addButtonWithTitle: ok_label];
        }

        let response: i64 = msg_send![alert, runModal];
        let index = (response - NS_ALERT_FIRST_BUTTON_RETURN) as usize;
        tx.send(index).ok();

        let _: () = msg_send![alert, release];
        let _: () = msg_send![pool, drain];
    }
    rx
}
