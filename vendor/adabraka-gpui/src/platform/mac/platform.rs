use super::tray::MacTray;
use super::{
    BoolExt, MacKeyboardLayout, MacKeyboardMapper,
    attributed_string::{NSAttributedString, NSMutableAttributedString},
    events::key_to_native,
    renderer,
};
use crate::{
    Action, AnyWindowHandle, BackgroundExecutor, ClipboardEntry, ClipboardItem, ClipboardString,
    CursorStyle, ForegroundExecutor, Image, ImageFormat, KeyContext, Keymap, MacDispatcher,
    MacDisplay, MacWindow, Menu, MenuItem, OsMenu, OwnedMenu, PathPromptOptions, Platform,
    PlatformDisplay, PlatformKeyboardLayout, PlatformKeyboardMapper, PlatformTextSystem,
    PlatformWindow, Result, SemanticVersion, SharedString, SystemMenuType, Task, TrayIconEvent,
    TrayMenuItem, WindowAppearance, WindowParams, hash,
};
use anyhow::{Context as _, anyhow};
use block::ConcreteBlock;
use cocoa::{
    appkit::{
        NSApplication, NSApplicationActivationPolicy::NSApplicationActivationPolicyAccessory,
        NSApplicationActivationPolicy::NSApplicationActivationPolicyRegular, NSEventModifierFlags,
        NSMenu, NSMenuItem, NSModalResponse, NSOpenPanel, NSPasteboard, NSPasteboardTypePNG,
        NSPasteboardTypeRTF, NSPasteboardTypeRTFD, NSPasteboardTypeString, NSPasteboardTypeTIFF,
        NSSavePanel, NSWindow,
    },
    base::{BOOL, NO, YES, id, nil, selector},
    foundation::{
        NSArray, NSAutoreleasePool, NSBundle, NSData, NSInteger, NSProcessInfo, NSRange, NSSize,
        NSString, NSUInteger, NSURL,
    },
};
use core_foundation::{
    base::{CFRelease, CFType, CFTypeRef, OSStatus, TCFType},
    boolean::CFBoolean,
    data::CFData,
    dictionary::{CFDictionary, CFDictionaryRef, CFMutableDictionary},
    runloop::CFRunLoopRun,
    string::{CFString, CFStringRef},
};
use ctor::ctor;
use futures::channel::oneshot;
use itertools::Itertools;
use objc::{
    class,
    declare::ClassDecl,
    msg_send,
    runtime::{Class, Object, Sel},
    sel, sel_impl,
};
use parking_lot::Mutex;
use ptr::null_mut;
use std::{
    cell::Cell,
    convert::TryInto,
    ffi::{CStr, OsStr, c_void},
    os::{raw::c_char, unix::ffi::OsStrExt},
    path::{Path, PathBuf},
    process::Command,
    ptr,
    rc::Rc,
    slice, str,
    sync::{Arc, OnceLock},
};
use strum::IntoEnumIterator;
use util::ResultExt;

#[allow(non_upper_case_globals)]
const NSUTF8StringEncoding: NSUInteger = 4;

const MAC_PLATFORM_IVAR: &str = "platform";
static mut APP_CLASS: *const Class = ptr::null();
static mut APP_DELEGATE_CLASS: *const Class = ptr::null();

#[ctor]
unsafe fn build_classes() {
    unsafe {
        APP_CLASS = {
            let mut decl = ClassDecl::new("GPUIApplication", class!(NSApplication)).unwrap();
            decl.add_ivar::<*mut c_void>(MAC_PLATFORM_IVAR);
            decl.register()
        }
    };
    unsafe {
        APP_DELEGATE_CLASS = unsafe {
            let mut decl = ClassDecl::new("GPUIApplicationDelegate", class!(NSResponder)).unwrap();
            decl.add_ivar::<*mut c_void>(MAC_PLATFORM_IVAR);
            decl.add_method(
                sel!(applicationWillFinishLaunching:),
                will_finish_launching as extern "C" fn(&mut Object, Sel, id),
            );
            decl.add_method(
                sel!(applicationDidFinishLaunching:),
                did_finish_launching as extern "C" fn(&mut Object, Sel, id),
            );
            decl.add_method(
                sel!(applicationShouldHandleReopen:hasVisibleWindows:),
                should_handle_reopen as extern "C" fn(&mut Object, Sel, id, bool),
            );
            decl.add_method(
                sel!(applicationWillTerminate:),
                will_terminate as extern "C" fn(&mut Object, Sel, id),
            );
            decl.add_method(
                sel!(handleGPUIMenuItem:),
                handle_menu_item as extern "C" fn(&mut Object, Sel, id),
            );
            decl.add_method(
                sel!(handleTrayMenuItem:),
                handle_tray_menu_item as extern "C" fn(&mut Object, Sel, id),
            );
            decl.add_method(
                sel!(handleTrayPanelClick:),
                handle_tray_panel_click as extern "C" fn(&mut Object, Sel, id),
            );
            // Add menu item handlers so that OS save panels have the correct key commands
            decl.add_method(
                sel!(cut:),
                handle_menu_item as extern "C" fn(&mut Object, Sel, id),
            );
            decl.add_method(
                sel!(copy:),
                handle_menu_item as extern "C" fn(&mut Object, Sel, id),
            );
            decl.add_method(
                sel!(paste:),
                handle_menu_item as extern "C" fn(&mut Object, Sel, id),
            );
            decl.add_method(
                sel!(selectAll:),
                handle_menu_item as extern "C" fn(&mut Object, Sel, id),
            );
            decl.add_method(
                sel!(undo:),
                handle_menu_item as extern "C" fn(&mut Object, Sel, id),
            );
            decl.add_method(
                sel!(redo:),
                handle_menu_item as extern "C" fn(&mut Object, Sel, id),
            );
            decl.add_method(
                sel!(validateMenuItem:),
                validate_menu_item as extern "C" fn(&mut Object, Sel, id) -> bool,
            );
            decl.add_method(
                sel!(menuWillOpen:),
                menu_will_open as extern "C" fn(&mut Object, Sel, id),
            );
            decl.add_method(
                sel!(applicationDockMenu:),
                handle_dock_menu as extern "C" fn(&mut Object, Sel, id) -> id,
            );
            decl.add_method(
                sel!(application:openURLs:),
                open_urls as extern "C" fn(&mut Object, Sel, id, id),
            );

            decl.add_method(
                sel!(onKeyboardLayoutChange:),
                on_keyboard_layout_change as extern "C" fn(&mut Object, Sel, id),
            );

            decl.add_method(
                sel!(applicationShouldTerminateAfterLastWindowClosed:),
                should_terminate_after_last_window_closed
                    as extern "C" fn(&mut Object, Sel, id) -> BOOL,
            );

            decl.add_method(
                sel!(handleSystemPowerEvent:),
                handle_system_power_event as extern "C" fn(&mut Object, Sel, id),
            );

            decl.add_method(
                sel!(handleContextMenuItem:),
                handle_context_menu_item as extern "C" fn(&mut Object, Sel, id),
            );

            decl.register()
        }
    }
}

pub(crate) struct MacPlatform(Mutex<MacPlatformState>);

pub(crate) struct MacPlatformState {
    background_executor: BackgroundExecutor,
    foreground_executor: ForegroundExecutor,
    text_system: Arc<dyn PlatformTextSystem>,
    renderer_context: renderer::Context,
    headless: bool,
    pasteboard: id,
    text_hash_pasteboard_type: id,
    metadata_pasteboard_type: id,
    reopen: Option<Box<dyn FnMut()>>,
    on_keyboard_layout_change: Option<Box<dyn FnMut()>>,
    quit: Option<Box<dyn FnMut()>>,
    menu_command: Option<Box<dyn FnMut(&dyn Action)>>,
    validate_menu_command: Option<Box<dyn FnMut(&dyn Action) -> bool>>,
    will_open_menu: Option<Box<dyn FnMut()>>,
    menu_actions: Vec<Box<dyn Action>>,
    open_urls: Option<Box<dyn FnMut(Vec<String>)>>,
    finish_launching: Option<Box<dyn FnOnce()>>,
    dock_menu: Option<id>,
    menus: Option<Vec<OwnedMenu>>,
    keyboard_mapper: Rc<MacKeyboardMapper>,
    keep_alive_without_windows: bool,
    tray: Option<MacTray>,
    tray_icon_callback: Option<Box<dyn FnMut(TrayIconEvent)>>,
    tray_menu_callback: Option<Box<dyn FnMut(SharedString)>>,
    global_hotkey_callback: Option<Box<dyn FnMut(u32)>>,
    global_hotkey_monitors: Vec<id>,
    global_hotkey_registrations: std::collections::HashMap<u32, crate::Keystroke>,
    system_power_callback: Option<Box<dyn FnMut(crate::SystemPowerEvent)>>,
    network_change_callback: Option<Box<dyn FnMut(crate::NetworkStatus)>>,
    media_key_callback: Option<Box<dyn FnMut(crate::MediaKeyEvent)>>,
    media_key_monitor: Option<id>,
    network_monitor: Option<*const c_void>,
    attention_request_id: isize,
    context_menu_callback: Option<Box<dyn FnMut(crate::SharedString)>>,
}

impl Default for MacPlatform {
    fn default() -> Self {
        Self::new(false)
    }
}

impl MacPlatform {
    pub(crate) fn new(headless: bool) -> Self {
        let dispatcher = Arc::new(MacDispatcher::new());

        #[cfg(feature = "font-kit")]
        let text_system = Arc::new(crate::MacTextSystem::new());

        #[cfg(not(feature = "font-kit"))]
        let text_system = Arc::new(crate::NoopTextSystem::new());

        let keyboard_layout = MacKeyboardLayout::new();
        let keyboard_mapper = Rc::new(MacKeyboardMapper::new(keyboard_layout.id()));

        Self(Mutex::new(MacPlatformState {
            headless,
            text_system,
            background_executor: BackgroundExecutor::new(dispatcher.clone()),
            foreground_executor: ForegroundExecutor::new(dispatcher),
            renderer_context: renderer::Context::default(),
            pasteboard: unsafe { NSPasteboard::generalPasteboard(nil) },
            text_hash_pasteboard_type: unsafe { ns_string("zed-text-hash") },
            metadata_pasteboard_type: unsafe { ns_string("zed-metadata") },
            reopen: None,
            quit: None,
            menu_command: None,
            validate_menu_command: None,
            will_open_menu: None,
            menu_actions: Default::default(),
            open_urls: None,
            finish_launching: None,
            dock_menu: None,
            on_keyboard_layout_change: None,
            menus: None,
            keyboard_mapper,
            keep_alive_without_windows: false,
            tray: None,
            tray_icon_callback: None,
            tray_menu_callback: None,
            global_hotkey_callback: None,
            global_hotkey_monitors: Vec::new(),
            global_hotkey_registrations: std::collections::HashMap::new(),
            system_power_callback: None,
            network_change_callback: None,
            media_key_callback: None,
            media_key_monitor: None,
            network_monitor: None,
            attention_request_id: 0,
            context_menu_callback: None,
        }))
    }

    unsafe fn read_from_pasteboard(&self, pasteboard: *mut Object, kind: id) -> Option<&[u8]> {
        unsafe {
            let data = pasteboard.dataForType(kind);
            if data == nil {
                None
            } else {
                Some(slice::from_raw_parts(
                    data.bytes() as *mut u8,
                    data.length() as usize,
                ))
            }
        }
    }

    unsafe fn create_menu_bar(
        &self,
        menus: &Vec<Menu>,
        delegate: id,
        actions: &mut Vec<Box<dyn Action>>,
        keymap: &Keymap,
    ) -> id {
        unsafe {
            let application_menu = NSMenu::new(nil).autorelease();
            application_menu.setDelegate_(delegate);

            for menu_config in menus {
                let menu = NSMenu::new(nil).autorelease();
                let menu_title = ns_string(&menu_config.name);
                menu.setTitle_(menu_title);
                menu.setDelegate_(delegate);

                for item_config in &menu_config.items {
                    menu.addItem_(Self::create_menu_item(
                        item_config,
                        delegate,
                        actions,
                        keymap,
                    ));
                }

                let menu_item = NSMenuItem::new(nil).autorelease();
                menu_item.setTitle_(menu_title);
                menu_item.setSubmenu_(menu);

                if let Some(icon_bytes) = &menu_config.icon {
                    let ns_data: id = NSData::dataWithBytes_length_(
                        nil,
                        icon_bytes.as_ptr() as *const std::ffi::c_void,
                        icon_bytes.len() as u64,
                    );
                    let image: id = msg_send![class!(NSImage), alloc];
                    let image: id = msg_send![image, initWithData: ns_data];
                    if image != nil {
                        let image: id = msg_send![image, autorelease];
                        let _: () = msg_send![image, setSize: NSSize::new(16.0, 16.0)];
                        let _: () = msg_send![image, setTemplate: YES];
                        let _: () = msg_send![menu_item, setImage: image];
                    }
                }

                application_menu.addItem_(menu_item);

                if menu_config.name == "Window" {
                    let app: id = msg_send![APP_CLASS, sharedApplication];
                    app.setWindowsMenu_(menu);
                }
            }

            application_menu
        }
    }

    unsafe fn create_dock_menu(
        &self,
        menu_items: Vec<MenuItem>,
        delegate: id,
        actions: &mut Vec<Box<dyn Action>>,
        keymap: &Keymap,
    ) -> id {
        unsafe {
            let dock_menu = NSMenu::new(nil);
            dock_menu.setDelegate_(delegate);
            for item_config in menu_items {
                dock_menu.addItem_(Self::create_menu_item(
                    &item_config,
                    delegate,
                    actions,
                    keymap,
                ));
            }

            dock_menu
        }
    }

    unsafe fn create_menu_item(
        item: &MenuItem,
        delegate: id,
        actions: &mut Vec<Box<dyn Action>>,
        keymap: &Keymap,
    ) -> id {
        static DEFAULT_CONTEXT: OnceLock<Vec<KeyContext>> = OnceLock::new();

        unsafe {
            match item {
                MenuItem::Separator => NSMenuItem::separatorItem(nil),
                MenuItem::Action {
                    name,
                    action,
                    os_action,
                } => {
                    // Note that this is intentionally using earlier bindings, whereas typically
                    // later ones take display precedence. See the discussion on
                    // https://github.com/zed-industries/zed/issues/23621
                    let keystrokes = keymap
                        .bindings_for_action(action.as_ref())
                        .find_or_first(|binding| {
                            binding.predicate().is_none_or(|predicate| {
                                predicate.eval(DEFAULT_CONTEXT.get_or_init(|| {
                                    let mut workspace_context = KeyContext::new_with_defaults();
                                    workspace_context.add("Workspace");
                                    let mut pane_context = KeyContext::new_with_defaults();
                                    pane_context.add("Pane");
                                    let mut editor_context = KeyContext::new_with_defaults();
                                    editor_context.add("Editor");

                                    pane_context.extend(&editor_context);
                                    workspace_context.extend(&pane_context);
                                    vec![workspace_context]
                                }))
                            })
                        })
                        .map(|binding| binding.keystrokes());

                    let selector = match os_action {
                        Some(crate::OsAction::Cut) => selector("cut:"),
                        Some(crate::OsAction::Copy) => selector("copy:"),
                        Some(crate::OsAction::Paste) => selector("paste:"),
                        Some(crate::OsAction::SelectAll) => selector("selectAll:"),
                        // "undo:" and "redo:" are always disabled in our case, as
                        // we don't have a NSTextView/NSTextField to enable them on.
                        Some(crate::OsAction::Undo) => selector("handleGPUIMenuItem:"),
                        Some(crate::OsAction::Redo) => selector("handleGPUIMenuItem:"),
                        None => selector("handleGPUIMenuItem:"),
                    };

                    let item;
                    if let Some(keystrokes) = keystrokes {
                        if keystrokes.len() == 1 {
                            let keystroke = &keystrokes[0];
                            let mut mask = NSEventModifierFlags::empty();
                            for (modifier, flag) in &[
                                (
                                    keystroke.modifiers().platform,
                                    NSEventModifierFlags::NSCommandKeyMask,
                                ),
                                (
                                    keystroke.modifiers().control,
                                    NSEventModifierFlags::NSControlKeyMask,
                                ),
                                (
                                    keystroke.modifiers().alt,
                                    NSEventModifierFlags::NSAlternateKeyMask,
                                ),
                                (
                                    keystroke.modifiers().shift,
                                    NSEventModifierFlags::NSShiftKeyMask,
                                ),
                            ] {
                                if *modifier {
                                    mask |= *flag;
                                }
                            }

                            item = NSMenuItem::alloc(nil)
                                .initWithTitle_action_keyEquivalent_(
                                    ns_string(name),
                                    selector,
                                    ns_string(key_to_native(keystroke.key()).as_ref()),
                                )
                                .autorelease();
                            if Self::os_version() >= SemanticVersion::new(12, 0, 0) {
                                let _: () = msg_send![item, setAllowsAutomaticKeyEquivalentLocalization: NO];
                            }
                            item.setKeyEquivalentModifierMask_(mask);
                        } else {
                            item = NSMenuItem::alloc(nil)
                                .initWithTitle_action_keyEquivalent_(
                                    ns_string(name),
                                    selector,
                                    ns_string(""),
                                )
                                .autorelease();
                        }
                    } else {
                        item = NSMenuItem::alloc(nil)
                            .initWithTitle_action_keyEquivalent_(
                                ns_string(name),
                                selector,
                                ns_string(""),
                            )
                            .autorelease();
                    }

                    let tag = actions.len() as NSInteger;
                    let _: () = msg_send![item, setTag: tag];
                    actions.push(action.boxed_clone());
                    item
                }
                MenuItem::Submenu(Menu { name, icon, items }) => {
                    let item = NSMenuItem::new(nil).autorelease();
                    let submenu = NSMenu::new(nil).autorelease();
                    submenu.setDelegate_(delegate);
                    for item in items {
                        submenu.addItem_(Self::create_menu_item(item, delegate, actions, keymap));
                    }
                    item.setSubmenu_(submenu);
                    item.setTitle_(ns_string(name));

                    if let Some(icon_bytes) = icon {
                        let ns_data: id = NSData::dataWithBytes_length_(
                            nil,
                            icon_bytes.as_ptr() as *const std::ffi::c_void,
                            icon_bytes.len() as u64,
                        );
                        let image: id = msg_send![class!(NSImage), alloc];
                        let image: id = msg_send![image, initWithData: ns_data];
                        if image != nil {
                            let image: id = msg_send![image, autorelease];
                            let _: () = msg_send![image, setSize: NSSize::new(16.0, 16.0)];
                            let _: () = msg_send![image, setTemplate: YES];
                            let _: () = msg_send![item, setImage: image];
                        }
                    }

                    item
                }
                MenuItem::SystemMenu(OsMenu { name, menu_type }) => {
                    let item = NSMenuItem::new(nil).autorelease();
                    let submenu = NSMenu::new(nil).autorelease();
                    submenu.setDelegate_(delegate);
                    item.setSubmenu_(submenu);
                    item.setTitle_(ns_string(name));

                    match menu_type {
                        SystemMenuType::Services => {
                            let app: id = msg_send![APP_CLASS, sharedApplication];
                            app.setServicesMenu_(item);
                        }
                    }

                    item
                }
            }
        }
    }

    fn os_version() -> SemanticVersion {
        let version = unsafe {
            let process_info = NSProcessInfo::processInfo(nil);
            process_info.operatingSystemVersion()
        };
        SemanticVersion::new(
            version.majorVersion as usize,
            version.minorVersion as usize,
            version.patchVersion as usize,
        )
    }
}

impl Platform for MacPlatform {
    fn background_executor(&self) -> BackgroundExecutor {
        self.0.lock().background_executor.clone()
    }

    fn foreground_executor(&self) -> crate::ForegroundExecutor {
        self.0.lock().foreground_executor.clone()
    }

    fn text_system(&self) -> Arc<dyn PlatformTextSystem> {
        self.0.lock().text_system.clone()
    }

    fn run(&self, on_finish_launching: Box<dyn FnOnce()>) {
        let mut state = self.0.lock();
        if state.headless {
            drop(state);
            on_finish_launching();
            unsafe { CFRunLoopRun() };
        } else {
            state.finish_launching = Some(on_finish_launching);
            drop(state);
        }

        unsafe {
            let app: id = msg_send![APP_CLASS, sharedApplication];
            let app_delegate: id = msg_send![APP_DELEGATE_CLASS, new];
            app.setDelegate_(app_delegate);

            let self_ptr = self as *const Self as *const c_void;
            (*app).set_ivar(MAC_PLATFORM_IVAR, self_ptr);
            (*app_delegate).set_ivar(MAC_PLATFORM_IVAR, self_ptr);

            let pool = NSAutoreleasePool::new(nil);
            app.run();
            pool.drain();

            (*app).set_ivar(MAC_PLATFORM_IVAR, null_mut::<c_void>());
            (*NSWindow::delegate(app)).set_ivar(MAC_PLATFORM_IVAR, null_mut::<c_void>());
        }
    }

    fn quit(&self) {
        // Quitting the app causes us to close windows, which invokes `Window::on_close` callbacks
        // synchronously before this method terminates. If we call `Platform::quit` while holding a
        // borrow of the app state (which most of the time we will do), we will end up
        // double-borrowing the app state in the `on_close` callbacks for our open windows. To solve
        // this, we make quitting the application asynchronous so that we aren't holding borrows to
        // the app state on the stack when we actually terminate the app.

        use super::dispatcher::{dispatch_get_main_queue, dispatch_sys::dispatch_async_f};

        unsafe {
            dispatch_async_f(dispatch_get_main_queue(), ptr::null_mut(), Some(quit));
        }

        unsafe extern "C" fn quit(_: *mut c_void) {
            unsafe {
                let app = NSApplication::sharedApplication(nil);
                let _: () = msg_send![app, terminate: nil];
            }
        }
    }

    fn restart(&self, _binary_path: Option<PathBuf>) {
        use std::os::unix::process::CommandExt as _;

        let app_pid = std::process::id().to_string();
        let app_path = self
            .app_path()
            .ok()
            // When the app is not bundled, `app_path` returns the
            // directory containing the executable. Disregard this
            // and get the path to the executable itself.
            .and_then(|path| (path.extension()?.to_str()? == "app").then_some(path))
            .unwrap_or_else(|| std::env::current_exe().unwrap());

        // Wait until this process has exited and then re-open this path.
        let script = r#"
            while kill -0 $0 2> /dev/null; do
                sleep 0.1
            done
            open "$1"
        "#;

        #[allow(
            clippy::disallowed_methods,
            reason = "We are restarting ourselves, using std command thus is fine"
        )]
        let restart_process = Command::new("/bin/bash")
            .arg("-c")
            .arg(script)
            .arg(app_pid)
            .arg(app_path)
            .process_group(0)
            .spawn();

        match restart_process {
            Ok(_) => self.quit(),
            Err(e) => log::error!("failed to spawn restart script: {:?}", e),
        }
    }

    fn activate(&self, ignoring_other_apps: bool) {
        unsafe {
            let app = NSApplication::sharedApplication(nil);
            app.activateIgnoringOtherApps_(ignoring_other_apps.to_objc());
        }
    }

    fn hide(&self) {
        unsafe {
            let app = NSApplication::sharedApplication(nil);
            let _: () = msg_send![app, hide: nil];
        }
    }

    fn hide_other_apps(&self) {
        unsafe {
            let app = NSApplication::sharedApplication(nil);
            let _: () = msg_send![app, hideOtherApplications: nil];
        }
    }

    fn unhide_other_apps(&self) {
        unsafe {
            let app = NSApplication::sharedApplication(nil);
            let _: () = msg_send![app, unhideAllApplications: nil];
        }
    }

    fn primary_display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        Some(Rc::new(MacDisplay::primary()))
    }

    fn displays(&self) -> Vec<Rc<dyn PlatformDisplay>> {
        MacDisplay::all()
            .map(|screen| Rc::new(screen) as Rc<_>)
            .collect()
    }

    #[cfg(feature = "screen-capture")]
    fn is_screen_capture_supported(&self) -> bool {
        let min_version = cocoa::foundation::NSOperatingSystemVersion::new(12, 3, 0);
        super::is_macos_version_at_least(min_version)
    }

    #[cfg(feature = "screen-capture")]
    fn screen_capture_sources(
        &self,
    ) -> oneshot::Receiver<Result<Vec<Rc<dyn crate::ScreenCaptureSource>>>> {
        super::screen_capture::get_sources()
    }

    fn active_window(&self) -> Option<AnyWindowHandle> {
        MacWindow::active_window()
    }

    // Returns the windows ordered front-to-back, meaning that the active
    // window is the first one in the returned vec.
    fn window_stack(&self) -> Option<Vec<AnyWindowHandle>> {
        Some(MacWindow::ordered_windows())
    }

    fn open_window(
        &self,
        handle: AnyWindowHandle,
        options: WindowParams,
    ) -> Result<Box<dyn PlatformWindow>> {
        let renderer_context = self.0.lock().renderer_context.clone();
        Ok(Box::new(MacWindow::open(
            handle,
            options,
            self.foreground_executor(),
            renderer_context,
        )))
    }

    fn window_appearance(&self) -> WindowAppearance {
        unsafe {
            let app = NSApplication::sharedApplication(nil);
            let appearance: id = msg_send![app, effectiveAppearance];
            WindowAppearance::from_native(appearance)
        }
    }

    fn open_url(&self, url: &str) {
        unsafe {
            let url = NSURL::alloc(nil)
                .initWithString_(ns_string(url))
                .autorelease();
            let workspace: id = msg_send![class!(NSWorkspace), sharedWorkspace];
            msg_send![workspace, openURL: url]
        }
    }

    fn register_url_scheme(&self, scheme: &str) -> Task<anyhow::Result<()>> {
        // API only available post Monterey
        // https://developer.apple.com/documentation/appkit/nsworkspace/3753004-setdefaultapplicationaturl
        let (done_tx, done_rx) = oneshot::channel();
        if Self::os_version() < SemanticVersion::new(12, 0, 0) {
            return Task::ready(Err(anyhow!(
                "macOS 12.0 or later is required to register URL schemes"
            )));
        }

        let bundle_id = unsafe {
            let bundle: id = msg_send![class!(NSBundle), mainBundle];
            let bundle_id: id = msg_send![bundle, bundleIdentifier];
            if bundle_id == nil {
                return Task::ready(Err(anyhow!("Can only register URL scheme in bundled apps")));
            }
            bundle_id
        };

        unsafe {
            let workspace: id = msg_send![class!(NSWorkspace), sharedWorkspace];
            let scheme: id = ns_string(scheme);
            let app: id = msg_send![workspace, URLForApplicationWithBundleIdentifier: bundle_id];
            if app == nil {
                return Task::ready(Err(anyhow!(
                    "Cannot register URL scheme until app is installed"
                )));
            }
            let done_tx = Cell::new(Some(done_tx));
            let block = ConcreteBlock::new(move |error: id| {
                let result = if error == nil {
                    Ok(())
                } else {
                    let msg: id = msg_send![error, localizedDescription];
                    Err(anyhow!("Failed to register: {msg:?}"))
                };

                if let Some(done_tx) = done_tx.take() {
                    let _ = done_tx.send(result);
                }
            });
            let block = block.copy();
            let _: () = msg_send![workspace, setDefaultApplicationAtURL: app toOpenURLsWithScheme: scheme completionHandler: block];
        }

        self.background_executor()
            .spawn(async { crate::Flatten::flatten(done_rx.await.map_err(|e| anyhow!(e))) })
    }

    fn on_open_urls(&self, callback: Box<dyn FnMut(Vec<String>)>) {
        self.0.lock().open_urls = Some(callback);
    }

    fn prompt_for_paths(
        &self,
        options: PathPromptOptions,
    ) -> oneshot::Receiver<Result<Option<Vec<PathBuf>>>> {
        let (done_tx, done_rx) = oneshot::channel();
        self.foreground_executor()
            .spawn(async move {
                unsafe {
                    let panel = NSOpenPanel::openPanel(nil);
                    panel.setCanChooseDirectories_(options.directories.to_objc());
                    panel.setCanChooseFiles_(options.files.to_objc());
                    panel.setAllowsMultipleSelection_(options.multiple.to_objc());

                    panel.setCanCreateDirectories(true.to_objc());
                    panel.setResolvesAliases_(false.to_objc());
                    let done_tx = Cell::new(Some(done_tx));
                    let block = ConcreteBlock::new(move |response: NSModalResponse| {
                        let result = if response == NSModalResponse::NSModalResponseOk {
                            let mut result = Vec::new();
                            let urls = panel.URLs();
                            for i in 0..urls.count() {
                                let url = urls.objectAtIndex(i);
                                if url.isFileURL() == YES
                                    && let Ok(path) = ns_url_to_path(url)
                                {
                                    result.push(path)
                                }
                            }
                            Some(result)
                        } else {
                            None
                        };

                        if let Some(done_tx) = done_tx.take() {
                            let _ = done_tx.send(Ok(result));
                        }
                    });
                    let block = block.copy();

                    if let Some(prompt) = options.prompt {
                        let _: () = msg_send![panel, setPrompt: ns_string(&prompt)];
                    }

                    let _: () = msg_send![panel, beginWithCompletionHandler: block];
                }
            })
            .detach();
        done_rx
    }

    fn prompt_for_new_path(
        &self,
        directory: &Path,
        suggested_name: Option<&str>,
    ) -> oneshot::Receiver<Result<Option<PathBuf>>> {
        let directory = directory.to_owned();
        let suggested_name = suggested_name.map(|s| s.to_owned());
        let (done_tx, done_rx) = oneshot::channel();
        self.foreground_executor()
            .spawn(async move {
                unsafe {
                    let panel = NSSavePanel::savePanel(nil);
                    let path = ns_string(directory.to_string_lossy().as_ref());
                    let url = NSURL::fileURLWithPath_isDirectory_(nil, path, true.to_objc());
                    panel.setDirectoryURL(url);

                    if let Some(suggested_name) = suggested_name {
                        let name_string = ns_string(&suggested_name);
                        let _: () = msg_send![panel, setNameFieldStringValue: name_string];
                    }

                    let done_tx = Cell::new(Some(done_tx));
                    let block = ConcreteBlock::new(move |response: NSModalResponse| {
                        let mut result = None;
                        if response == NSModalResponse::NSModalResponseOk {
                            let url = panel.URL();
                            if url.isFileURL() == YES {
                                result = ns_url_to_path(panel.URL()).ok().map(|mut result| {
                                    let Some(filename) = result.file_name() else {
                                        return result;
                                    };
                                    let chunks = filename
                                        .as_bytes()
                                        .split(|&b| b == b'.')
                                        .collect::<Vec<_>>();

                                    // https://github.com/zed-industries/zed/issues/16969
                                    // Workaround a bug in macOS Sequoia that adds an extra file-extension
                                    // sometimes. e.g. `a.sql` becomes `a.sql.s` or `a.txtx` becomes `a.txtx.txt`
                                    //
                                    // This is conditional on OS version because I'd like to get rid of it, so that
                                    // you can manually create a file called `a.sql.s`. That said it seems better
                                    // to break that use-case than breaking `a.sql`.
                                    if chunks.len() == 3
                                        && chunks[1].starts_with(chunks[2])
                                        && Self::os_version() >= SemanticVersion::new(15, 0, 0)
                                    {
                                        let new_filename = OsStr::from_bytes(
                                            &filename.as_bytes()
                                                [..chunks[0].len() + 1 + chunks[1].len()],
                                        )
                                        .to_owned();
                                        result.set_file_name(&new_filename);
                                    }
                                    result
                                })
                            }
                        }

                        if let Some(done_tx) = done_tx.take() {
                            let _ = done_tx.send(Ok(result));
                        }
                    });
                    let block = block.copy();
                    let _: () = msg_send![panel, beginWithCompletionHandler: block];
                }
            })
            .detach();

        done_rx
    }

    fn can_select_mixed_files_and_dirs(&self) -> bool {
        true
    }

    fn reveal_path(&self, path: &Path) {
        unsafe {
            let path = path.to_path_buf();
            self.0
                .lock()
                .background_executor
                .spawn(async move {
                    let full_path = ns_string(path.to_str().unwrap_or(""));
                    let root_full_path = ns_string("");
                    let workspace: id = msg_send![class!(NSWorkspace), sharedWorkspace];
                    let _: BOOL = msg_send![
                        workspace,
                        selectFile: full_path
                        inFileViewerRootedAtPath: root_full_path
                    ];
                })
                .detach();
        }
    }

    fn open_with_system(&self, path: &Path) {
        let path = path.to_owned();
        self.0
            .lock()
            .background_executor
            .spawn(async move {
                if let Some(mut child) = smol::process::Command::new("open")
                    .arg(path)
                    .spawn()
                    .context("invoking open command")
                    .log_err()
                {
                    child.status().await.log_err();
                }
            })
            .detach();
    }

    fn on_quit(&self, callback: Box<dyn FnMut()>) {
        self.0.lock().quit = Some(callback);
    }

    fn on_reopen(&self, callback: Box<dyn FnMut()>) {
        self.0.lock().reopen = Some(callback);
    }

    fn on_keyboard_layout_change(&self, callback: Box<dyn FnMut()>) {
        self.0.lock().on_keyboard_layout_change = Some(callback);
    }

    fn on_app_menu_action(&self, callback: Box<dyn FnMut(&dyn Action)>) {
        self.0.lock().menu_command = Some(callback);
    }

    fn on_will_open_app_menu(&self, callback: Box<dyn FnMut()>) {
        self.0.lock().will_open_menu = Some(callback);
    }

    fn on_validate_app_menu_command(&self, callback: Box<dyn FnMut(&dyn Action) -> bool>) {
        self.0.lock().validate_menu_command = Some(callback);
    }

    fn keyboard_layout(&self) -> Box<dyn PlatformKeyboardLayout> {
        Box::new(MacKeyboardLayout::new())
    }

    fn keyboard_mapper(&self) -> Rc<dyn PlatformKeyboardMapper> {
        self.0.lock().keyboard_mapper.clone()
    }

    fn app_path(&self) -> Result<PathBuf> {
        unsafe {
            let bundle: id = NSBundle::mainBundle();
            anyhow::ensure!(!bundle.is_null(), "app is not running inside a bundle");
            Ok(path_from_objc(msg_send![bundle, bundlePath]))
        }
    }

    fn set_menus(&self, menus: Vec<Menu>, keymap: &Keymap) {
        unsafe {
            let app: id = msg_send![APP_CLASS, sharedApplication];
            let mut state = self.0.lock();
            let actions = &mut state.menu_actions;
            let menu = self.create_menu_bar(&menus, NSWindow::delegate(app), actions, keymap);
            drop(state);
            app.setMainMenu_(menu);
        }
        self.0.lock().menus = Some(menus.into_iter().map(|menu| menu.owned()).collect());
    }

    fn get_menus(&self) -> Option<Vec<OwnedMenu>> {
        self.0.lock().menus.clone()
    }

    fn set_dock_menu(&self, menu: Vec<MenuItem>, keymap: &Keymap) {
        unsafe {
            let app: id = msg_send![APP_CLASS, sharedApplication];
            let mut state = self.0.lock();
            let actions = &mut state.menu_actions;
            let new = self.create_dock_menu(menu, NSWindow::delegate(app), actions, keymap);
            if let Some(old) = state.dock_menu.replace(new) {
                CFRelease(old as _)
            }
        }
    }

    fn add_recent_document(&self, path: &Path) {
        if let Some(path_str) = path.to_str() {
            unsafe {
                let document_controller: id =
                    msg_send![class!(NSDocumentController), sharedDocumentController];
                let url: id = NSURL::fileURLWithPath_(nil, ns_string(path_str));
                let _: () = msg_send![document_controller, noteNewRecentDocumentURL:url];
            }
        }
    }

    fn path_for_auxiliary_executable(&self, name: &str) -> Result<PathBuf> {
        unsafe {
            let bundle: id = NSBundle::mainBundle();
            anyhow::ensure!(!bundle.is_null(), "app is not running inside a bundle");
            let name = ns_string(name);
            let url: id = msg_send![bundle, URLForAuxiliaryExecutable: name];
            anyhow::ensure!(!url.is_null(), "resource not found");
            ns_url_to_path(url)
        }
    }

    /// Match cursor style to one of the styles available
    /// in macOS's [NSCursor](https://developer.apple.com/documentation/appkit/nscursor).
    fn set_cursor_style(&self, style: CursorStyle) {
        unsafe {
            if style == CursorStyle::None {
                let _: () = msg_send![class!(NSCursor), setHiddenUntilMouseMoves:YES];
                return;
            }

            let new_cursor: id = match style {
                CursorStyle::Arrow => msg_send![class!(NSCursor), arrowCursor],
                CursorStyle::IBeam => msg_send![class!(NSCursor), IBeamCursor],
                CursorStyle::Crosshair => msg_send![class!(NSCursor), crosshairCursor],
                CursorStyle::ClosedHand => msg_send![class!(NSCursor), closedHandCursor],
                CursorStyle::OpenHand => msg_send![class!(NSCursor), openHandCursor],
                CursorStyle::PointingHand => msg_send![class!(NSCursor), pointingHandCursor],
                CursorStyle::ResizeLeftRight => msg_send![class!(NSCursor), resizeLeftRightCursor],
                CursorStyle::ResizeUpDown => msg_send![class!(NSCursor), resizeUpDownCursor],
                CursorStyle::ResizeLeft => msg_send![class!(NSCursor), resizeLeftCursor],
                CursorStyle::ResizeRight => msg_send![class!(NSCursor), resizeRightCursor],
                CursorStyle::ResizeColumn => msg_send![class!(NSCursor), resizeLeftRightCursor],
                CursorStyle::ResizeRow => msg_send![class!(NSCursor), resizeUpDownCursor],
                CursorStyle::ResizeUp => msg_send![class!(NSCursor), resizeUpCursor],
                CursorStyle::ResizeDown => msg_send![class!(NSCursor), resizeDownCursor],

                // Undocumented, private class methods:
                // https://stackoverflow.com/questions/27242353/cocoa-predefined-resize-mouse-cursor
                CursorStyle::ResizeUpLeftDownRight => {
                    msg_send![class!(NSCursor), _windowResizeNorthWestSouthEastCursor]
                }
                CursorStyle::ResizeUpRightDownLeft => {
                    msg_send![class!(NSCursor), _windowResizeNorthEastSouthWestCursor]
                }

                CursorStyle::IBeamCursorForVerticalLayout => {
                    msg_send![class!(NSCursor), IBeamCursorForVerticalLayout]
                }
                CursorStyle::OperationNotAllowed => {
                    msg_send![class!(NSCursor), operationNotAllowedCursor]
                }
                CursorStyle::DragLink => msg_send![class!(NSCursor), dragLinkCursor],
                CursorStyle::DragCopy => msg_send![class!(NSCursor), dragCopyCursor],
                CursorStyle::ContextualMenu => msg_send![class!(NSCursor), contextualMenuCursor],
                CursorStyle::None => unreachable!(),
            };

            let old_cursor: id = msg_send![class!(NSCursor), currentCursor];
            if new_cursor != old_cursor {
                let _: () = msg_send![new_cursor, set];
            }
        }
    }

    fn should_auto_hide_scrollbars(&self) -> bool {
        #[allow(non_upper_case_globals)]
        const NSScrollerStyleOverlay: NSInteger = 1;

        unsafe {
            let style: NSInteger = msg_send![class!(NSScroller), preferredScrollerStyle];
            style == NSScrollerStyleOverlay
        }
    }

    fn write_to_clipboard(&self, item: ClipboardItem) {
        use crate::ClipboardEntry;

        unsafe {
            // We only want to use NSAttributedString if there are multiple entries to write.
            if item.entries.len() <= 1 {
                match item.entries.first() {
                    Some(entry) => match entry {
                        ClipboardEntry::String(string) => {
                            self.write_plaintext_to_clipboard(string);
                        }
                        ClipboardEntry::Image(image) => {
                            self.write_image_to_clipboard(image);
                        }
                    },
                    None => {
                        // Writing an empty list of entries just clears the clipboard.
                        let state = self.0.lock();
                        state.pasteboard.clearContents();
                    }
                }
            } else {
                let mut any_images = false;
                let attributed_string = {
                    let mut buf = NSMutableAttributedString::alloc(nil)
                        // TODO can we skip this? Or at least part of it?
                        .init_attributed_string(NSString::alloc(nil).init_str(""));

                    for entry in item.entries {
                        if let ClipboardEntry::String(ClipboardString { text, metadata: _ }) = entry
                        {
                            let to_append = NSAttributedString::alloc(nil)
                                .init_attributed_string(NSString::alloc(nil).init_str(&text));

                            buf.appendAttributedString_(to_append);
                        }
                    }

                    buf
                };

                let state = self.0.lock();
                state.pasteboard.clearContents();

                // Only set rich text clipboard types if we actually have 1+ images to include.
                if any_images {
                    let rtfd_data = attributed_string.RTFDFromRange_documentAttributes_(
                        NSRange::new(0, msg_send![attributed_string, length]),
                        nil,
                    );
                    if rtfd_data != nil {
                        state
                            .pasteboard
                            .setData_forType(rtfd_data, NSPasteboardTypeRTFD);
                    }

                    let rtf_data = attributed_string.RTFFromRange_documentAttributes_(
                        NSRange::new(0, attributed_string.length()),
                        nil,
                    );
                    if rtf_data != nil {
                        state
                            .pasteboard
                            .setData_forType(rtf_data, NSPasteboardTypeRTF);
                    }
                }

                let plain_text = attributed_string.string();
                state
                    .pasteboard
                    .setString_forType(plain_text, NSPasteboardTypeString);
            }
        }
    }

    fn read_from_clipboard(&self) -> Option<ClipboardItem> {
        let state = self.0.lock();
        let pasteboard = state.pasteboard;

        // First, see if it's a string.
        unsafe {
            let types: id = pasteboard.types();
            let string_type: id = ns_string("public.utf8-plain-text");

            if msg_send![types, containsObject: string_type] {
                let data = pasteboard.dataForType(string_type);
                if data == nil {
                    return None;
                } else if data.bytes().is_null() {
                    // https://developer.apple.com/documentation/foundation/nsdata/1410616-bytes?language=objc
                    // "If the length of the NSData object is 0, this property returns nil."
                    return Some(self.read_string_from_clipboard(&state, &[]));
                } else {
                    let bytes =
                        slice::from_raw_parts(data.bytes() as *mut u8, data.length() as usize);

                    return Some(self.read_string_from_clipboard(&state, bytes));
                }
            }

            // If it wasn't a string, try the various supported image types.
            for format in ImageFormat::iter() {
                if let Some(item) = try_clipboard_image(pasteboard, format) {
                    return Some(item);
                }
            }
        }

        // If it wasn't a string or a supported image type, give up.
        None
    }

    fn write_credentials(&self, url: &str, username: &str, password: &[u8]) -> Task<Result<()>> {
        let url = url.to_string();
        let username = username.to_string();
        let password = password.to_vec();
        self.background_executor().spawn(async move {
            unsafe {
                use security::*;

                let url = CFString::from(url.as_str());
                let username = CFString::from(username.as_str());
                let password = CFData::from_buffer(&password);

                // First, check if there are already credentials for the given server. If so, then
                // update the username and password.
                let mut verb = "updating";
                let mut query_attrs = CFMutableDictionary::with_capacity(2);
                query_attrs.set(kSecClass as *const _, kSecClassInternetPassword as *const _);
                query_attrs.set(kSecAttrServer as *const _, url.as_CFTypeRef());

                let mut attrs = CFMutableDictionary::with_capacity(4);
                attrs.set(kSecClass as *const _, kSecClassInternetPassword as *const _);
                attrs.set(kSecAttrServer as *const _, url.as_CFTypeRef());
                attrs.set(kSecAttrAccount as *const _, username.as_CFTypeRef());
                attrs.set(kSecValueData as *const _, password.as_CFTypeRef());

                let mut status = SecItemUpdate(
                    query_attrs.as_concrete_TypeRef(),
                    attrs.as_concrete_TypeRef(),
                );

                // If there were no existing credentials for the given server, then create them.
                if status == errSecItemNotFound {
                    verb = "creating";
                    status = SecItemAdd(attrs.as_concrete_TypeRef(), ptr::null_mut());
                }
                anyhow::ensure!(status == errSecSuccess, "{verb} password failed: {status}");
            }
            Ok(())
        })
    }

    fn read_credentials(&self, url: &str) -> Task<Result<Option<(String, Vec<u8>)>>> {
        let url = url.to_string();
        self.background_executor().spawn(async move {
            let url = CFString::from(url.as_str());
            let cf_true = CFBoolean::true_value().as_CFTypeRef();

            unsafe {
                use security::*;

                // Find any credentials for the given server URL.
                let mut attrs = CFMutableDictionary::with_capacity(5);
                attrs.set(kSecClass as *const _, kSecClassInternetPassword as *const _);
                attrs.set(kSecAttrServer as *const _, url.as_CFTypeRef());
                attrs.set(kSecReturnAttributes as *const _, cf_true);
                attrs.set(kSecReturnData as *const _, cf_true);

                let mut result = CFTypeRef::from(ptr::null());
                let status = SecItemCopyMatching(attrs.as_concrete_TypeRef(), &mut result);
                match status {
                    security::errSecSuccess => {}
                    security::errSecItemNotFound | security::errSecUserCanceled => return Ok(None),
                    _ => anyhow::bail!("reading password failed: {status}"),
                }

                let result = CFType::wrap_under_create_rule(result)
                    .downcast::<CFDictionary>()
                    .context("keychain item was not a dictionary")?;
                let username = result
                    .find(kSecAttrAccount as *const _)
                    .context("account was missing from keychain item")?;
                let username = CFType::wrap_under_get_rule(*username)
                    .downcast::<CFString>()
                    .context("account was not a string")?;
                let password = result
                    .find(kSecValueData as *const _)
                    .context("password was missing from keychain item")?;
                let password = CFType::wrap_under_get_rule(*password)
                    .downcast::<CFData>()
                    .context("password was not a string")?;

                Ok(Some((username.to_string(), password.bytes().to_vec())))
            }
        })
    }

    fn set_keep_alive_without_windows(&self, keep_alive: bool) {
        self.0.lock().keep_alive_without_windows = keep_alive;
    }

    fn set_tray_icon(&self, icon: Option<&[u8]>) {
        let mut state = self.0.lock();
        if state.tray.is_none() {
            state.tray = Some(MacTray::new());
        }
        if let Some(tray) = &state.tray {
            tray.set_icon(icon);
        }
    }

    fn set_tray_menu(&self, menu: Vec<TrayMenuItem>) {
        let mut state = self.0.lock();
        if state.tray.is_none() {
            state.tray = Some(MacTray::new());
        }
        if let Some(tray) = &state.tray {
            tray.set_menu(menu);
        }
    }

    fn set_tray_tooltip(&self, tooltip: &str) {
        let mut state = self.0.lock();
        if state.tray.is_none() {
            state.tray = Some(MacTray::new());
        }
        if let Some(tray) = &state.tray {
            tray.set_tooltip(tooltip);
        }
    }

    fn set_tray_panel_mode(&self, enabled: bool) {
        let mut state = self.0.lock();
        if state.tray.is_none() {
            state.tray = Some(MacTray::new());
        }
        if let Some(tray) = &state.tray {
            tray.set_panel_mode(enabled);
        }
    }

    fn get_tray_icon_bounds(&self) -> Option<crate::Bounds<crate::Pixels>> {
        let state = self.0.lock();
        state.tray.as_ref().and_then(|tray| tray.get_icon_bounds())
    }

    fn on_tray_icon_event(&self, callback: Box<dyn FnMut(TrayIconEvent)>) {
        self.0.lock().tray_icon_callback = Some(callback);
    }

    fn on_tray_menu_action(&self, callback: Box<dyn FnMut(SharedString)>) {
        self.0.lock().tray_menu_callback = Some(callback);
    }

    fn register_global_hotkey(&self, id: u32, keystroke: &crate::Keystroke) -> Result<()> {
        let mut state = self.0.lock();
        state
            .global_hotkey_registrations
            .insert(id, keystroke.clone());

        if state.global_hotkey_monitors.is_empty() {
            let platform_ptr = &self.0 as *const Mutex<MacPlatformState> as *const c_void;

            unsafe {
                let mask: u64 = 1 << 10;

                let global_block = ConcreteBlock::new(move |event: id| {
                    let platform_state = &*(platform_ptr as *const Mutex<MacPlatformState>);
                    let mut lock = platform_state.lock();
                    if let Some(hotkey_id) = super::global_hotkey::find_matching_hotkey(
                        &lock.global_hotkey_registrations,
                        event,
                    ) {
                        if let Some(mut callback) = lock.global_hotkey_callback.take() {
                            drop(lock);
                            callback(hotkey_id);
                            platform_state.lock().global_hotkey_callback = Some(callback);
                        }
                    }
                });
                let global_block = global_block.copy();
                let global_monitor: id = msg_send![
                    class!(NSEvent),
                    addGlobalMonitorForEventsMatchingMask: mask
                    handler: &*global_block
                ];
                std::mem::forget(global_block);

                let local_block = ConcreteBlock::new(move |event: id| -> id {
                    let platform_state = &*(platform_ptr as *const Mutex<MacPlatformState>);
                    let mut lock = platform_state.lock();
                    if let Some(hotkey_id) = super::global_hotkey::find_matching_hotkey(
                        &lock.global_hotkey_registrations,
                        event,
                    ) {
                        if let Some(mut callback) = lock.global_hotkey_callback.take() {
                            drop(lock);
                            callback(hotkey_id);
                            platform_state.lock().global_hotkey_callback = Some(callback);
                        }
                    }
                    event
                });
                let local_block = local_block.copy();
                let local_monitor: id = msg_send![
                    class!(NSEvent),
                    addLocalMonitorForEventsMatchingMask: mask
                    handler: &*local_block
                ];
                std::mem::forget(local_block);

                state.global_hotkey_monitors.push(global_monitor);
                state.global_hotkey_monitors.push(local_monitor);
            }
        }

        Ok(())
    }

    fn unregister_global_hotkey(&self, id: u32) {
        let mut state = self.0.lock();
        state.global_hotkey_registrations.remove(&id);
    }

    fn on_global_hotkey(&self, callback: Box<dyn FnMut(u32)>) {
        self.0.lock().global_hotkey_callback = Some(callback);
    }

    fn focused_window_info(&self) -> Option<crate::FocusedWindowInfo> {
        super::active_window::get_focused_window_info()
    }

    fn accessibility_status(&self) -> crate::PermissionStatus {
        super::permissions::accessibility_status()
    }

    fn request_accessibility_permission(&self) {
        super::permissions::request_accessibility_permission();
    }

    fn set_auto_launch(&self, app_id: &str, enabled: bool) -> Result<()> {
        super::auto_launch::set_auto_launch(app_id, enabled)
    }

    fn is_auto_launch_enabled(&self, app_id: &str) -> bool {
        super::auto_launch::is_auto_launch_enabled(app_id)
    }

    fn show_notification(&self, title: &str, body: &str) -> Result<()> {
        unsafe {
            let bundle: id = msg_send![class!(NSBundle), mainBundle];
            let bundle_id: id = msg_send![bundle, bundleIdentifier];
            if bundle_id == nil {
                return Err(anyhow!(
                    "Notifications require an app bundle (bundleIdentifier is nil)"
                ));
            }

            let center: id = msg_send![class!(UNUserNotificationCenter), currentNotificationCenter];
            if center == nil {
                return Err(anyhow!("UNUserNotificationCenter not available"));
            }
            let content: id = msg_send![class!(UNMutableNotificationContent), new];
            let ns_title = cocoa::foundation::NSString::alloc(nil).init_str(title);
            let _: () = msg_send![content, setTitle: ns_title];
            let ns_body = cocoa::foundation::NSString::alloc(nil).init_str(body);
            let _: () = msg_send![content, setBody: ns_body];

            let uuid_str = uuid::Uuid::new_v4().to_string();
            let ns_id = cocoa::foundation::NSString::alloc(nil).init_str(&uuid_str);
            let request: id = msg_send![
                class!(UNNotificationRequest),
                requestWithIdentifier: ns_id
                content: content
                trigger: nil
            ];
            let _: () =
                msg_send![center, addNotificationRequest: request withCompletionHandler: nil];
        }
        Ok(())
    }

    fn delete_credentials(&self, url: &str) -> Task<Result<()>> {
        let url = url.to_string();

        self.background_executor().spawn(async move {
            unsafe {
                use security::*;

                let url = CFString::from(url.as_str());
                let mut query_attrs = CFMutableDictionary::with_capacity(2);
                query_attrs.set(kSecClass as *const _, kSecClassInternetPassword as *const _);
                query_attrs.set(kSecAttrServer as *const _, url.as_CFTypeRef());

                let status = SecItemDelete(query_attrs.as_concrete_TypeRef());
                anyhow::ensure!(status == errSecSuccess, "delete password failed: {status}");
            }
            Ok(())
        })
    }

    fn on_system_power_event(&self, callback: Box<dyn FnMut(crate::SystemPowerEvent)>) {
        self.0.lock().system_power_callback = Some(callback);
    }

    fn start_power_save_blocker(&self, kind: crate::PowerSaveBlockerKind) -> Option<u32> {
        super::power::start_power_save_blocker(kind)
    }

    fn stop_power_save_blocker(&self, id: u32) {
        super::power::stop_power_save_blocker(id);
    }

    fn system_idle_time(&self) -> Option<std::time::Duration> {
        super::power::system_idle_time()
    }

    fn network_status(&self) -> crate::NetworkStatus {
        super::network::network_status()
    }

    fn on_network_status_change(&self, callback: Box<dyn FnMut(crate::NetworkStatus)>) {
        let mut state = self.0.lock();

        if let Some(old_monitor) = state.network_monitor.take() {
            unsafe { super::network::cancel_path_monitor(old_monitor) };
        }

        state.network_change_callback = Some(callback);

        let platform_ptr = &self.0 as *const Mutex<MacPlatformState> as *const c_void;

        unsafe {
            let monitor = super::network::create_path_monitor();
            if monitor.is_null() {
                return;
            }

            let block = ConcreteBlock::new(move |path: *const c_void| {
                let status = super::network::path_status_to_network_status(path);

                struct NetworkChangeCtx {
                    platform: *const c_void,
                    status: crate::NetworkStatus,
                }

                let ctx = Box::into_raw(Box::new(NetworkChangeCtx {
                    platform: platform_ptr,
                    status,
                }));

                use super::dispatcher::{dispatch_get_main_queue, dispatch_sys::dispatch_async_f};

                unsafe extern "C" fn invoke(ctx_ptr: *mut c_void) {
                    let ctx = unsafe { Box::from_raw(ctx_ptr as *mut NetworkChangeCtx) };
                    let platform_state =
                        unsafe { &*(ctx.platform as *const Mutex<MacPlatformState>) };
                    let mut lock = platform_state.lock();
                    if let Some(mut callback) = lock.network_change_callback.take() {
                        drop(lock);
                        callback(ctx.status);
                        platform_state.lock().network_change_callback = Some(callback);
                    }
                }

                dispatch_async_f(dispatch_get_main_queue(), ctx as *mut c_void, Some(invoke));
            });
            let block = block.copy();

            let queue = super::dispatcher::dispatch_get_main_queue();
            super::network::start_path_monitor(
                monitor,
                &*block as *const _ as *const c_void,
                queue as *const c_void,
            );
            std::mem::forget(block);

            state.network_monitor = Some(monitor);
        }
    }

    fn on_media_key_event(&self, callback: Box<dyn FnMut(crate::MediaKeyEvent)>) {
        let mut state = self.0.lock();
        state.media_key_callback = Some(callback);

        if state.media_key_monitor.is_some() {
            return;
        }

        let platform_ptr = &self.0 as *const Mutex<MacPlatformState> as *const c_void;

        unsafe {
            let mask: u64 = 1 << 14; // NSSystemDefinedMask

            let block = ConcreteBlock::new(move |event: id| {
                let subtype: i16 = msg_send![event, subtype];
                if subtype != 8 {
                    return;
                }

                let data1: isize = msg_send![event, data1];
                let key_code = (data1 >> 16) & 0xFF;
                let flags = (data1 >> 8) & 0xFF;
                let is_down = (flags & 0x1) == 0;

                if !is_down {
                    return;
                }

                let media_event = match key_code {
                    16 => crate::MediaKeyEvent::PlayPause,
                    17 => crate::MediaKeyEvent::NextTrack,
                    18 => crate::MediaKeyEvent::PreviousTrack,
                    19 => crate::MediaKeyEvent::Stop,
                    20 => crate::MediaKeyEvent::Play,
                    _ => return,
                };

                let platform_state = &*(platform_ptr as *const Mutex<MacPlatformState>);
                let mut lock = platform_state.lock();
                if let Some(mut callback) = lock.media_key_callback.take() {
                    drop(lock);
                    callback(media_event);
                    platform_state.lock().media_key_callback = Some(callback);
                }
            });
            let block = block.copy();
            let monitor: id = msg_send![
                class!(NSEvent),
                addGlobalMonitorForEventsMatchingMask: mask
                handler: &*block
            ];
            std::mem::forget(block);

            state.media_key_monitor = Some(monitor);
        }
    }

    fn request_user_attention(&self, attention_type: crate::AttentionType) {
        let id = super::dock::request_user_attention(attention_type);
        self.0.lock().attention_request_id = id;
    }

    fn cancel_user_attention(&self) {
        let id = self.0.lock().attention_request_id;
        super::dock::cancel_user_attention(id);
    }

    fn set_dock_badge(&self, label: Option<&str>) {
        super::dock::set_dock_badge(label);
    }

    fn show_context_menu(
        &self,
        position: crate::Point<crate::Pixels>,
        items: Vec<crate::TrayMenuItem>,
        callback: Box<dyn FnMut(crate::SharedString)>,
    ) {
        self.0.lock().context_menu_callback = Some(callback);

        unsafe {
            let menu: id = msg_send![class!(NSMenu), new];
            let _: () = msg_send![menu, setAutoenablesItems: NO];
            super::tray::build_menu_with_selector(
                menu,
                &items,
                sel!(handleContextMenuItem:),
            );

            let main_screen: id = cocoa::appkit::NSScreen::mainScreen(nil);
            let screen_height = if main_screen != nil {
                cocoa::appkit::NSScreen::frame(main_screen).size.height
            } else {
                0.0
            };

            let point = cocoa::foundation::NSPoint::new(
                position.x.0 as f64,
                screen_height - position.y.0 as f64,
            );

            let _: () = msg_send![menu, popUpMenuPositioningItem: nil atLocation: point inView: nil];
            let _: () = msg_send![menu, release];
        }
    }

    fn show_dialog(
        &self,
        options: crate::DialogOptions,
    ) -> futures::channel::oneshot::Receiver<usize> {
        super::dialog::show_dialog(options)
    }

    fn os_info(&self) -> crate::OsInfo {
        super::os_info::get_os_info()
    }

    fn biometric_status(&self) -> crate::BiometricStatus {
        super::biometric::biometric_status()
    }

    fn authenticate_biometric(&self, reason: &str, callback: Box<dyn FnOnce(bool) + Send>) {
        super::biometric::authenticate_biometric(reason, callback);
    }
}

impl MacPlatform {
    unsafe fn read_string_from_clipboard(
        &self,
        state: &MacPlatformState,
        text_bytes: &[u8],
    ) -> ClipboardItem {
        unsafe {
            let text = String::from_utf8_lossy(text_bytes).to_string();
            let metadata = self
                .read_from_pasteboard(state.pasteboard, state.text_hash_pasteboard_type)
                .and_then(|hash_bytes| {
                    let hash_bytes = hash_bytes.try_into().ok()?;
                    let hash = u64::from_be_bytes(hash_bytes);
                    let metadata = self
                        .read_from_pasteboard(state.pasteboard, state.metadata_pasteboard_type)?;

                    if hash == ClipboardString::text_hash(&text) {
                        String::from_utf8(metadata.to_vec()).ok()
                    } else {
                        None
                    }
                });

            ClipboardItem {
                entries: vec![ClipboardEntry::String(ClipboardString { text, metadata })],
            }
        }
    }

    unsafe fn write_plaintext_to_clipboard(&self, string: &ClipboardString) {
        unsafe {
            let state = self.0.lock();
            state.pasteboard.clearContents();

            let text_bytes = NSData::dataWithBytes_length_(
                nil,
                string.text.as_ptr() as *const c_void,
                string.text.len() as u64,
            );
            state
                .pasteboard
                .setData_forType(text_bytes, NSPasteboardTypeString);

            if let Some(metadata) = string.metadata.as_ref() {
                let hash_bytes = ClipboardString::text_hash(&string.text).to_be_bytes();
                let hash_bytes = NSData::dataWithBytes_length_(
                    nil,
                    hash_bytes.as_ptr() as *const c_void,
                    hash_bytes.len() as u64,
                );
                state
                    .pasteboard
                    .setData_forType(hash_bytes, state.text_hash_pasteboard_type);

                let metadata_bytes = NSData::dataWithBytes_length_(
                    nil,
                    metadata.as_ptr() as *const c_void,
                    metadata.len() as u64,
                );
                state
                    .pasteboard
                    .setData_forType(metadata_bytes, state.metadata_pasteboard_type);
            }
        }
    }

    unsafe fn write_image_to_clipboard(&self, image: &Image) {
        unsafe {
            let state = self.0.lock();
            state.pasteboard.clearContents();

            let bytes = NSData::dataWithBytes_length_(
                nil,
                image.bytes.as_ptr() as *const c_void,
                image.bytes.len() as u64,
            );

            state
                .pasteboard
                .setData_forType(bytes, Into::<UTType>::into(image.format).inner_mut());
        }
    }
}

fn try_clipboard_image(pasteboard: id, format: ImageFormat) -> Option<ClipboardItem> {
    let mut ut_type: UTType = format.into();

    unsafe {
        let types: id = pasteboard.types();
        if msg_send![types, containsObject: ut_type.inner()] {
            let data = pasteboard.dataForType(ut_type.inner_mut());
            if data == nil {
                None
            } else {
                let bytes = Vec::from(slice::from_raw_parts(
                    data.bytes() as *mut u8,
                    data.length() as usize,
                ));
                let id = hash(&bytes);

                Some(ClipboardItem {
                    entries: vec![ClipboardEntry::Image(Image { format, bytes, id })],
                })
            }
        } else {
            None
        }
    }
}

unsafe fn path_from_objc(path: id) -> PathBuf {
    let len = msg_send![path, lengthOfBytesUsingEncoding: NSUTF8StringEncoding];
    let bytes = unsafe { path.UTF8String() as *const u8 };
    let path = str::from_utf8(unsafe { slice::from_raw_parts(bytes, len) }).unwrap();
    PathBuf::from(path)
}

unsafe fn get_mac_platform(object: &mut Object) -> &MacPlatform {
    unsafe {
        let platform_ptr: *mut c_void = *object.get_ivar(MAC_PLATFORM_IVAR);
        assert!(!platform_ptr.is_null());
        &*(platform_ptr as *const MacPlatform)
    }
}

extern "C" fn will_finish_launching(_this: &mut Object, _: Sel, _: id) {
    unsafe {
        let user_defaults: id = msg_send![class!(NSUserDefaults), standardUserDefaults];

        // The autofill heuristic controller causes slowdown and high CPU usage.
        // We don't know exactly why. This disables the full heuristic controller.
        //
        // Adapted from: https://github.com/ghostty-org/ghostty/pull/8625
        let name = ns_string("NSAutoFillHeuristicControllerEnabled");
        let existing_value: id = msg_send![user_defaults, objectForKey: name];
        if existing_value == nil {
            let false_value: id = msg_send![class!(NSNumber), numberWithBool:false];
            let _: () = msg_send![user_defaults, setObject: false_value forKey: name];
        }
    }
}

extern "C" fn did_finish_launching(this: &mut Object, _: Sel, _: id) {
    unsafe {
        let app: id = msg_send![APP_CLASS, sharedApplication];

        let notification_center: *mut Object =
            msg_send![class!(NSNotificationCenter), defaultCenter];
        let name = ns_string("NSTextInputContextKeyboardSelectionDidChangeNotification");
        let _: () = msg_send![notification_center, addObserver: this as id
            selector: sel!(onKeyboardLayoutChange:)
            name: name
            object: nil
        ];

        let workspace: id = msg_send![class!(NSWorkspace), sharedWorkspace];
        let ws_notification_center: id = msg_send![workspace, notificationCenter];

        let power_notifications = [
            "NSWorkspaceWillSleepNotification",
            "NSWorkspaceDidWakeNotification",
            "NSWorkspaceSessionDidResignActiveNotification",
            "NSWorkspaceSessionDidBecomeActiveNotification",
            "NSWorkspaceWillPowerOffNotification",
        ];
        for name in &power_notifications {
            let ns_name = ns_string(name);
            let _: () = msg_send![ws_notification_center, addObserver: this as id
                selector: sel!(handleSystemPowerEvent:)
                name: ns_name
                object: nil
            ];
        }

        let platform = get_mac_platform(this);
        let callback = platform.0.lock().finish_launching.take();
        if let Some(callback) = callback {
            callback();
        }

        let keep_alive = platform.0.lock().keep_alive_without_windows;
        if keep_alive {
            app.setActivationPolicy_(NSApplicationActivationPolicyAccessory);
        } else {
            app.setActivationPolicy_(NSApplicationActivationPolicyRegular);
        }
    }
}

extern "C" fn should_handle_reopen(this: &mut Object, _: Sel, _: id, has_open_windows: bool) {
    if !has_open_windows {
        let platform = unsafe { get_mac_platform(this) };
        let mut lock = platform.0.lock();
        if let Some(mut callback) = lock.reopen.take() {
            drop(lock);
            callback();
            platform.0.lock().reopen.get_or_insert(callback);
        }
    }
}

extern "C" fn will_terminate(this: &mut Object, _: Sel, _: id) {
    let platform = unsafe { get_mac_platform(this) };
    let mut lock = platform.0.lock();
    if let Some(mut callback) = lock.quit.take() {
        drop(lock);
        callback();
        platform.0.lock().quit.get_or_insert(callback);
    }
}

extern "C" fn on_keyboard_layout_change(this: &mut Object, _: Sel, _: id) {
    let platform = unsafe { get_mac_platform(this) };
    let mut lock = platform.0.lock();
    let keyboard_layout = MacKeyboardLayout::new();
    lock.keyboard_mapper = Rc::new(MacKeyboardMapper::new(keyboard_layout.id()));
    if let Some(mut callback) = lock.on_keyboard_layout_change.take() {
        drop(lock);
        callback();
        platform
            .0
            .lock()
            .on_keyboard_layout_change
            .get_or_insert(callback);
    }
}

extern "C" fn should_terminate_after_last_window_closed(this: &mut Object, _: Sel, _: id) -> BOOL {
    let platform = unsafe { get_mac_platform(this) };
    let lock = platform.0.lock();
    if lock.keep_alive_without_windows {
        NO
    } else {
        YES
    }
}

extern "C" fn open_urls(this: &mut Object, _: Sel, _: id, urls: id) {
    let urls = unsafe {
        (0..urls.count())
            .filter_map(|i| {
                let url = urls.objectAtIndex(i);
                match CStr::from_ptr(url.absoluteString().UTF8String() as *mut c_char).to_str() {
                    Ok(string) => Some(string.to_string()),
                    Err(err) => {
                        log::error!("error converting path to string: {}", err);
                        None
                    }
                }
            })
            .collect::<Vec<_>>()
    };
    let platform = unsafe { get_mac_platform(this) };
    let mut lock = platform.0.lock();
    if let Some(mut callback) = lock.open_urls.take() {
        drop(lock);
        callback(urls);
        platform.0.lock().open_urls.get_or_insert(callback);
    }
}

extern "C" fn handle_menu_item(this: &mut Object, _: Sel, item: id) {
    unsafe {
        let platform = get_mac_platform(this);
        let mut lock = platform.0.lock();
        if let Some(mut callback) = lock.menu_command.take() {
            let tag: NSInteger = msg_send![item, tag];
            let index = tag as usize;
            if let Some(action) = lock.menu_actions.get(index) {
                let action = action.boxed_clone();
                drop(lock);
                callback(&*action);
            }
            platform.0.lock().menu_command.get_or_insert(callback);
        }
    }
}

extern "C" fn handle_tray_menu_item(this: &mut Object, _: Sel, item: id) {
    unsafe {
        let platform = get_mac_platform(this);
        let represented: id = msg_send![item, representedObject];
        if represented == nil {
            return;
        }
        let len: usize = msg_send![represented, lengthOfBytesUsingEncoding: NSUTF8StringEncoding];
        let bytes: *const u8 = msg_send![represented, UTF8String];
        let id_str = std::str::from_utf8(slice::from_raw_parts(bytes, len)).unwrap_or("");
        let shared_id: SharedString = id_str.to_string().into();

        let platform_ptr = platform as *const MacPlatform;

        use super::dispatcher::{dispatch_get_main_queue, dispatch_sys::dispatch_async_f};

        struct TrayActionCtx {
            platform: *const MacPlatform,
            id: SharedString,
        }

        let ctx = Box::into_raw(Box::new(TrayActionCtx {
            platform: platform_ptr,
            id: shared_id,
        }));

        unsafe extern "C" fn invoke(ctx_ptr: *mut c_void) {
            let ctx = unsafe { Box::from_raw(ctx_ptr as *mut TrayActionCtx) };
            let platform = unsafe { &*ctx.platform };
            let mut lock = platform.0.lock();
            if let Some(mut callback) = lock.tray_menu_callback.take() {
                drop(lock);
                callback(ctx.id);
                platform.0.lock().tray_menu_callback = Some(callback);
            }
        }

        dispatch_async_f(dispatch_get_main_queue(), ctx as *mut c_void, Some(invoke));
    }
}

extern "C" fn handle_tray_panel_click(this: &mut Object, _: Sel, _sender: id) {
    unsafe {
        let platform = get_mac_platform(this);
        let platform_ptr = platform as *const MacPlatform;

        use super::dispatcher::{dispatch_get_main_queue, dispatch_sys::dispatch_async_f};

        unsafe extern "C" fn invoke(ctx_ptr: *mut c_void) {
            let platform = unsafe { &*(ctx_ptr as *const MacPlatform) };
            let mut lock = platform.0.lock();
            if let Some(mut callback) = lock.tray_icon_callback.take() {
                drop(lock);
                callback(TrayIconEvent::LeftClick);
                platform.0.lock().tray_icon_callback = Some(callback);
            }
        }

        dispatch_async_f(
            dispatch_get_main_queue(),
            platform_ptr as *mut c_void,
            Some(invoke),
        );
    }
}

extern "C" fn handle_system_power_event(this: &mut Object, _: Sel, notification: id) {
    unsafe {
        let name: id = msg_send![notification, name];
        let name_str: *const c_char = msg_send![name, UTF8String];
        let name_cstr = CStr::from_ptr(name_str);
        let name_bytes = name_cstr.to_bytes();

        let event = match name_bytes {
            b"NSWorkspaceWillSleepNotification" => crate::SystemPowerEvent::Suspend,
            b"NSWorkspaceDidWakeNotification" => crate::SystemPowerEvent::Resume,
            b"NSWorkspaceSessionDidResignActiveNotification" => {
                crate::SystemPowerEvent::LockScreen
            }
            b"NSWorkspaceSessionDidBecomeActiveNotification" => {
                crate::SystemPowerEvent::UnlockScreen
            }
            b"NSWorkspaceWillPowerOffNotification" => crate::SystemPowerEvent::Shutdown,
            _ => return,
        };

        let platform = get_mac_platform(this);
        let mut lock = platform.0.lock();
        if let Some(mut callback) = lock.system_power_callback.take() {
            drop(lock);
            callback(event);
            platform.0.lock().system_power_callback = Some(callback);
        }
    }
}

extern "C" fn handle_context_menu_item(this: &mut Object, _: Sel, item: id) {
    unsafe {
        let platform = get_mac_platform(this);
        let represented: id = msg_send![item, representedObject];
        if represented == nil {
            return;
        }
        let len: usize = msg_send![represented, lengthOfBytesUsingEncoding: NSUTF8StringEncoding];
        let bytes: *const u8 = msg_send![represented, UTF8String];
        let id_str = std::str::from_utf8(slice::from_raw_parts(bytes, len)).unwrap_or("");
        let shared_id: SharedString = id_str.to_string().into();

        let mut lock = platform.0.lock();
        if let Some(mut callback) = lock.context_menu_callback.take() {
            drop(lock);
            callback(shared_id);
            platform.0.lock().context_menu_callback = Some(callback);
        }
    }
}

extern "C" fn validate_menu_item(this: &mut Object, _: Sel, item: id) -> bool {
    unsafe {
        let mut result = false;
        let platform = get_mac_platform(this);
        let mut lock = platform.0.lock();
        if let Some(mut callback) = lock.validate_menu_command.take() {
            let tag: NSInteger = msg_send![item, tag];
            let index = tag as usize;
            if let Some(action) = lock.menu_actions.get(index) {
                let action = action.boxed_clone();
                drop(lock);
                result = callback(action.as_ref());
            }
            platform
                .0
                .lock()
                .validate_menu_command
                .get_or_insert(callback);
        }
        result
    }
}

extern "C" fn menu_will_open(this: &mut Object, _: Sel, _: id) {
    unsafe {
        let platform = get_mac_platform(this);
        let mut lock = platform.0.lock();
        if let Some(mut callback) = lock.will_open_menu.take() {
            drop(lock);
            callback();
            platform.0.lock().will_open_menu.get_or_insert(callback);
        }
    }
}

extern "C" fn handle_dock_menu(this: &mut Object, _: Sel, _: id) -> id {
    unsafe {
        let platform = get_mac_platform(this);
        let mut state = platform.0.lock();
        if let Some(id) = state.dock_menu {
            id
        } else {
            nil
        }
    }
}

unsafe fn ns_string(string: &str) -> id {
    unsafe { NSString::alloc(nil).init_str(string).autorelease() }
}

unsafe fn ns_url_to_path(url: id) -> Result<PathBuf> {
    let path: *mut c_char = msg_send![url, fileSystemRepresentation];
    anyhow::ensure!(!path.is_null(), "url is not a file path: {}", unsafe {
        CStr::from_ptr(url.absoluteString().UTF8String()).to_string_lossy()
    });
    Ok(PathBuf::from(OsStr::from_bytes(unsafe {
        CStr::from_ptr(path).to_bytes()
    })))
}

#[link(name = "Carbon", kind = "framework")]
unsafe extern "C" {
    pub(super) fn TISCopyCurrentKeyboardLayoutInputSource() -> *mut Object;
    pub(super) fn TISGetInputSourceProperty(
        inputSource: *mut Object,
        propertyKey: *const c_void,
    ) -> *mut Object;

    pub(super) fn UCKeyTranslate(
        keyLayoutPtr: *const ::std::os::raw::c_void,
        virtualKeyCode: u16,
        keyAction: u16,
        modifierKeyState: u32,
        keyboardType: u32,
        keyTranslateOptions: u32,
        deadKeyState: *mut u32,
        maxStringLength: usize,
        actualStringLength: *mut usize,
        unicodeString: *mut u16,
    ) -> u32;
    pub(super) fn LMGetKbdType() -> u16;
    pub(super) static kTISPropertyUnicodeKeyLayoutData: CFStringRef;
    pub(super) static kTISPropertyInputSourceID: CFStringRef;
    pub(super) static kTISPropertyLocalizedName: CFStringRef;
}

mod security {
    #![allow(non_upper_case_globals)]
    use super::*;

    #[link(name = "Security", kind = "framework")]
    unsafe extern "C" {
        pub static kSecClass: CFStringRef;
        pub static kSecClassInternetPassword: CFStringRef;
        pub static kSecAttrServer: CFStringRef;
        pub static kSecAttrAccount: CFStringRef;
        pub static kSecValueData: CFStringRef;
        pub static kSecReturnAttributes: CFStringRef;
        pub static kSecReturnData: CFStringRef;

        pub fn SecItemAdd(attributes: CFDictionaryRef, result: *mut CFTypeRef) -> OSStatus;
        pub fn SecItemUpdate(query: CFDictionaryRef, attributes: CFDictionaryRef) -> OSStatus;
        pub fn SecItemDelete(query: CFDictionaryRef) -> OSStatus;
        pub fn SecItemCopyMatching(query: CFDictionaryRef, result: *mut CFTypeRef) -> OSStatus;
    }

    pub const errSecSuccess: OSStatus = 0;
    pub const errSecUserCanceled: OSStatus = -128;
    pub const errSecItemNotFound: OSStatus = -25300;
}

impl From<ImageFormat> for UTType {
    fn from(value: ImageFormat) -> Self {
        match value {
            ImageFormat::Png => Self::png(),
            ImageFormat::Jpeg => Self::jpeg(),
            ImageFormat::Tiff => Self::tiff(),
            ImageFormat::Webp => Self::webp(),
            ImageFormat::Gif => Self::gif(),
            ImageFormat::Bmp => Self::bmp(),
            ImageFormat::Svg => Self::svg(),
        }
    }
}

// See https://developer.apple.com/documentation/uniformtypeidentifiers/uttype-swift.struct/
struct UTType(id);

impl UTType {
    pub fn png() -> Self {
        // https://developer.apple.com/documentation/uniformtypeidentifiers/uttype-swift.struct/png
        Self(unsafe { NSPasteboardTypePNG }) // This is a rare case where there's a built-in NSPasteboardType
    }

    pub fn jpeg() -> Self {
        // https://developer.apple.com/documentation/uniformtypeidentifiers/uttype-swift.struct/jpeg
        Self(unsafe { ns_string("public.jpeg") })
    }

    pub fn gif() -> Self {
        // https://developer.apple.com/documentation/uniformtypeidentifiers/uttype-swift.struct/gif
        Self(unsafe { ns_string("com.compuserve.gif") })
    }

    pub fn webp() -> Self {
        // https://developer.apple.com/documentation/uniformtypeidentifiers/uttype-swift.struct/webp
        Self(unsafe { ns_string("org.webmproject.webp") })
    }

    pub fn bmp() -> Self {
        // https://developer.apple.com/documentation/uniformtypeidentifiers/uttype-swift.struct/bmp
        Self(unsafe { ns_string("com.microsoft.bmp") })
    }

    pub fn svg() -> Self {
        // https://developer.apple.com/documentation/uniformtypeidentifiers/uttype-swift.struct/svg
        Self(unsafe { ns_string("public.svg-image") })
    }

    pub fn tiff() -> Self {
        // https://developer.apple.com/documentation/uniformtypeidentifiers/uttype-swift.struct/tiff
        Self(unsafe { NSPasteboardTypeTIFF }) // This is a rare case where there's a built-in NSPasteboardType
    }

    fn inner(&self) -> *const Object {
        self.0
    }

    fn inner_mut(&self) -> *mut Object {
        self.0 as *mut _
    }
}

#[cfg(test)]
mod tests {
    use crate::ClipboardItem;

    use super::*;

    #[test]
    fn test_clipboard() {
        let platform = build_platform();
        assert_eq!(platform.read_from_clipboard(), None);

        let item = ClipboardItem::new_string("1".to_string());
        platform.write_to_clipboard(item.clone());
        assert_eq!(platform.read_from_clipboard(), Some(item));

        let item = ClipboardItem {
            entries: vec![ClipboardEntry::String(
                ClipboardString::new("2".to_string()).with_json_metadata(vec![3, 4]),
            )],
        };
        platform.write_to_clipboard(item.clone());
        assert_eq!(platform.read_from_clipboard(), Some(item));

        let text_from_other_app = "text from other app";
        unsafe {
            let bytes = NSData::dataWithBytes_length_(
                nil,
                text_from_other_app.as_ptr() as *const c_void,
                text_from_other_app.len() as u64,
            );
            platform
                .0
                .lock()
                .pasteboard
                .setData_forType(bytes, NSPasteboardTypeString);
        }
        assert_eq!(
            platform.read_from_clipboard(),
            Some(ClipboardItem::new_string(text_from_other_app.to_string()))
        );
    }

    fn build_platform() -> MacPlatform {
        let platform = MacPlatform::new(false);
        platform.0.lock().pasteboard = unsafe { NSPasteboard::pasteboardWithUniqueName(nil) };
        platform
    }
}
