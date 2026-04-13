use crate::platform::TrayMenuItem;
use crate::{point, px, size, Bounds, Pixels};
use cocoa::{
    appkit::NSScreen,
    base::{id, nil, NO, YES},
    foundation::{NSData, NSSize, NSString},
};
use objc::{class, msg_send, rc::StrongPtr, sel, sel_impl};
use std::cell::Cell;
use std::ffi::c_void;

pub(crate) struct MacTray {
    status_item: StrongPtr,
    panel_mode: Cell<bool>,
    stored_menu: Cell<id>,
}

impl MacTray {
    pub fn new() -> Self {
        unsafe {
            let status_bar: id = msg_send![class!(NSStatusBar), systemStatusBar];
            let length: f64 = -1.0;
            let status_item: id = msg_send![status_bar, statusItemWithLength: length];
            let status_item = StrongPtr::retain(status_item);
            let _: () = msg_send![*status_item, setVisible: YES];

            let button: id = msg_send![*status_item, button];
            if button != nil {
                let default_title = NSString::alloc(nil).init_str("App");
                let _: () = msg_send![button, setTitle: default_title];
            }

            Self {
                status_item,
                panel_mode: Cell::new(false),
                stored_menu: Cell::new(nil),
            }
        }
    }

    pub fn set_icon(&self, icon_data: Option<&[u8]>) {
        unsafe {
            let button: id = msg_send![*self.status_item, button];
            if button == nil {
                return;
            }
            match icon_data {
                Some(data) => {
                    let ns_data: id = NSData::dataWithBytes_length_(
                        nil,
                        data.as_ptr() as *const c_void,
                        data.len() as u64,
                    );
                    let image: id = msg_send![class!(NSImage), alloc];
                    let image: id = msg_send![image, initWithData: ns_data];
                    if image != nil {
                        let _: () = msg_send![image, setSize: NSSize::new(18.0, 18.0)];
                        let _: () = msg_send![image, setTemplate: YES];
                        let _: () = msg_send![button, setImage: image];
                        let empty = NSString::alloc(nil).init_str("");
                        let _: () = msg_send![button, setTitle: empty];
                    }
                }
                None => {
                    let _: () = msg_send![button, setImage: nil];
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn set_title(&self, title: &str) {
        unsafe {
            let button: id = msg_send![*self.status_item, button];
            if button == nil {
                return;
            }
            let ns_title = NSString::alloc(nil).init_str(title);
            let _: () = msg_send![button, setTitle: ns_title];
        }
    }

    pub fn set_tooltip(&self, tooltip: &str) {
        unsafe {
            let button: id = msg_send![*self.status_item, button];
            if button == nil {
                return;
            }
            let ns_tooltip = NSString::alloc(nil).init_str(tooltip);
            let _: () = msg_send![button, setToolTip: ns_tooltip];
        }
    }

    pub fn set_menu(&self, items: Vec<TrayMenuItem>) {
        unsafe {
            let old_menu = self.stored_menu.get();
            if old_menu != nil {
                let _: () = msg_send![old_menu, release];
            }

            let menu: id = msg_send![class!(NSMenu), new];
            let _: () = msg_send![menu, setAutoenablesItems: NO];
            build_menu_with_selector(menu, &items, sel!(handleTrayMenuItem:));

            self.stored_menu.set(menu);

            if !self.panel_mode.get() {
                let _: () = msg_send![*self.status_item, setMenu: menu];
            }
        }
    }

    pub fn set_panel_mode(&self, enabled: bool) {
        self.panel_mode.set(enabled);
        unsafe {
            if enabled {
                let _: () = msg_send![*self.status_item, setMenu: nil];

                let button: id = msg_send![*self.status_item, button];
                if button != nil {
                    let delegate = get_app_delegate();
                    if delegate != nil {
                        let _: () = msg_send![button, setTarget: delegate];
                        let _: () = msg_send![button, setAction: sel!(handleTrayPanelClick:)];
                    }
                }
            } else {
                let button: id = msg_send![*self.status_item, button];
                if button != nil {
                    let null_sel: *const std::ffi::c_void = std::ptr::null();
                    let _: () = msg_send![button, setTarget: nil];
                    let _: () = msg_send![button, setAction: null_sel];
                }

                let stored = self.stored_menu.get();
                if stored != nil {
                    let _: () = msg_send![*self.status_item, setMenu: stored];
                }
            }
        }
    }

    pub fn get_icon_bounds(&self) -> Option<Bounds<Pixels>> {
        unsafe {
            let button: id = msg_send![*self.status_item, button];
            if button == nil {
                return None;
            }

            let button_window: id = msg_send![button, window];
            if button_window == nil {
                return None;
            }

            let frame: cocoa::foundation::NSRect = msg_send![button_window, frame];

            let main_screen: id = NSScreen::mainScreen(nil);
            if main_screen == nil {
                return None;
            }
            let screen_frame = NSScreen::frame(main_screen);

            let flipped_y = screen_frame.size.height - frame.origin.y - frame.size.height;

            Some(Bounds::new(
                point(px(frame.origin.x as f32), px(flipped_y as f32)),
                size(px(frame.size.width as f32), px(frame.size.height as f32)),
            ))
        }
    }
}

impl Drop for MacTray {
    fn drop(&mut self) {
        unsafe {
            let stored = self.stored_menu.get();
            if stored != nil {
                let _: () = msg_send![stored, release];
            }
            let status_bar: id = msg_send![class!(NSStatusBar), systemStatusBar];
            let _: () = msg_send![status_bar, removeStatusItem: *self.status_item];
        }
    }
}

unsafe fn get_app_delegate() -> id {
    let app: id = msg_send![class!(NSApplication), sharedApplication];
    msg_send![app, delegate]
}

pub(crate) unsafe fn configure_actionable_item_with_selector(
    menu_item: id,
    item_id: &str,
    selector: objc::runtime::Sel,
) {
    unsafe {
        let delegate = get_app_delegate();
        if delegate != nil {
            let _: () = msg_send![menu_item, setTarget: delegate];
            let _: () = msg_send![menu_item, setAction: selector];
            let represented = NSString::alloc(nil).init_str(item_id);
            let _: () = msg_send![menu_item, setRepresentedObject: represented];
            let _: () = msg_send![menu_item, setEnabled: YES];
        }
    }
}

pub(crate) unsafe fn build_menu_with_selector(
    menu: id,
    items: &[TrayMenuItem],
    selector: objc::runtime::Sel,
) {
    unsafe {
        for item in items {
            match item {
                TrayMenuItem::Action { label, id } => {
                    let title = NSString::alloc(nil).init_str(label.as_ref());
                    let menu_item: id = msg_send![class!(NSMenuItem), alloc];
                    let empty = NSString::alloc(nil).init_str("");
                    let menu_item: id =
                        msg_send![menu_item, initWithTitle:title action:nil keyEquivalent:empty];
                    configure_actionable_item_with_selector(menu_item, id.as_ref(), selector);
                    let _: () = msg_send![menu, addItem: menu_item];
                }
                TrayMenuItem::Separator => {
                    let separator: id = msg_send![class!(NSMenuItem), separatorItem];
                    let _: () = msg_send![menu, addItem: separator];
                }
                TrayMenuItem::Submenu {
                    label,
                    items: sub_items,
                } => {
                    let title = NSString::alloc(nil).init_str(label.as_ref());
                    let menu_item: id = msg_send![class!(NSMenuItem), alloc];
                    let empty = NSString::alloc(nil).init_str("");
                    let menu_item: id =
                        msg_send![menu_item, initWithTitle:title action:nil keyEquivalent:empty];
                    let submenu: id = msg_send![class!(NSMenu), new];
                    build_menu_with_selector(submenu, sub_items, selector);
                    let _: () = msg_send![menu_item, setSubmenu: submenu];
                    let _: () = msg_send![menu, addItem: menu_item];
                }
                TrayMenuItem::Toggle { label, checked, id } => {
                    let title = NSString::alloc(nil).init_str(label.as_ref());
                    let menu_item: id = msg_send![class!(NSMenuItem), alloc];
                    let empty = NSString::alloc(nil).init_str("");
                    let menu_item: id =
                        msg_send![menu_item, initWithTitle:title action:nil keyEquivalent:empty];
                    configure_actionable_item_with_selector(menu_item, id.as_ref(), selector);
                    let state: isize = if *checked { 1 } else { 0 };
                    let _: () = msg_send![menu_item, setState: state];
                    let _: () = msg_send![menu, addItem: menu_item];
                }
            }
        }
    }
}
