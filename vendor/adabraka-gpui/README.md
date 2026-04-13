# Adabraka GPUI

A GPU-accelerated UI framework for Rust, forked from [Zed's GPUI](https://github.com/zed-industries/zed). Adabraka GPUI extends the original framework with daemon-mode capabilities, system tray integration, global hotkeys, native notifications, and more — making it suitable for background apps, menu bar utilities, and overlay tools.

## Getting Started

Add the following to your `Cargo.toml`:

```toml
adabraka-gpui = "0.4"
```

### Platform Support

| Feature | macOS | Linux (X11) | Linux (Wayland) | Windows |
|---|---|---|---|---|
| GPU-accelerated rendering | Metal | Vulkan/OpenGL | Vulkan/OpenGL | DirectX |
| System tray icon & menu | Yes | Yes (DBus/SNI) | Yes (DBus/SNI) | Yes (Shell_NotifyIcon) |
| Tray menu actions | Yes | Yes | Yes | Yes |
| Global hotkeys | Yes | Yes (XGrabKey) | No (protocol limitation) | Yes (RegisterHotKey) |
| Native notifications | Yes (UNUserNotification) | Yes (notify-rust) | Yes (notify-rust) | Yes (Shell balloon) |
| Overlay windows (always-on-top) | Yes | Yes | Partial (no layer-shell) | Yes |
| Click-through windows | Yes | Yes (Shape ext) | Yes (wl_region) | Yes (WS_EX_TRANSPARENT) |
| Window show/hide | Yes | Yes | Yes | Yes |
| Auto-launch at login | Yes (SMAppService) | Yes (XDG autostart) | Yes (XDG autostart) | Yes (Registry) |
| Single instance lock | Yes (Unix socket) | Yes (Unix socket) | Yes (Unix socket) | Yes (Named mutex) |
| Focused window info | Yes (Accessibility) | Yes (EWMH) | No | Yes (Win32) |
| Permission queries | Yes (Accessibility, Mic) | No | No | No |
| Daemon mode (no dock icon) | Yes (Accessory policy) | Yes | Yes | Yes |

## Features

### Core UI Framework (inherited from GPUI)
- Hybrid immediate/retained mode rendering
- GPU-accelerated with Metal, Vulkan, OpenGL, and DirectX backends
- Tailwind-style layout and styling API
- Entity-based state management
- Declarative views with the `Render` trait
- Low-level `Element` API for custom rendering
- Async executor integrated with the platform event loop
- Action system for keyboard shortcuts
- Test framework with `#[gpui::test]`

### Daemon & Background App Support (new in Adabraka)
- **System tray** — icon, tooltip, and nested menus with action callbacks
- **Global hotkeys** — register system-wide keyboard shortcuts
- **Native notifications** — OS-level notifications on all platforms
- **Overlay windows** — always-on-top transparent windows
- **Click-through windows** — mouse events pass through to windows below
- **Window show/hide** — programmatic visibility control
- **Auto-launch** — register your app to start at login
- **Single instance** — prevent multiple copies with activation signaling
- **Keep alive without windows** — app runs with no visible windows
- **Focused window info** — query which window the user is focused on
- **Permission status** — check accessibility and microphone permissions
- **In-app toast notifications** — stackable, auto-dismissing toast component

## Quick Example

```rust
use gpui::{App, Application, TrayMenuItem};

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.set_keep_alive_without_windows(true);
        cx.set_tray_tooltip("My App");

        cx.set_tray_menu(vec![
            TrayMenuItem::Action {
                label: "Settings".into(),
                id: "settings".into(),
            },
            TrayMenuItem::Separator,
            TrayMenuItem::Action {
                label: "Quit".into(),
                id: "quit".into(),
            },
        ]);

        cx.on_tray_menu_action(|id, cx| match id.as_ref() {
            "quit" => cx.quit(),
            _ => {}
        });
    });
}
```

## Dependencies

### macOS

- [Xcode](https://apps.apple.com/us/app/xcode/id497799835?mt=12) with macOS components
- Xcode command line tools: `xcode-select --install`

### Linux

- Standard development packages for your distro
- For X11: `libxcb`, `libxkbcommon`
- For Wayland: `libwayland-client`, `libxkbcommon`
- D-Bus (for system tray via StatusNotifierItem)

### Windows

- Visual Studio Build Tools with C++ workload
- Windows SDK

## License

Apache-2.0
