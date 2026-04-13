use super::platform::PowerSaveHandle;

pub fn inhibit_screensaver(app_name: &str, reason: &str) -> Option<PowerSaveHandle> {
    let output = std::process::Command::new("dbus-send")
        .args([
            "--session",
            "--dest=org.freedesktop.ScreenSaver",
            "--type=method_call",
            "--print-reply",
            "/org/freedesktop/ScreenSaver",
            "org.freedesktop.ScreenSaver.Inhibit",
            &format!("string:{}", app_name),
            &format!("string:{}", reason),
        ])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    parse_dbus_uint32(&output.stdout).map(PowerSaveHandle::ScreenSaverCookie)
}

pub fn uninhibit_screensaver(cookie: u32) {
    let _ = std::process::Command::new("dbus-send")
        .args([
            "--session",
            "--dest=org.freedesktop.ScreenSaver",
            "--type=method_call",
            "/org/freedesktop/ScreenSaver",
            "org.freedesktop.ScreenSaver.UnInhibit",
            &format!("uint32:{}", cookie),
        ])
        .output();
}

pub fn inhibit_suspend(app_name: &str, reason: &str) -> Option<PowerSaveHandle> {
    let who_arg = format!("--who={}", app_name);
    let why_arg = format!("--why={}", reason);
    #[allow(clippy::disallowed_methods)]
    let child = std::process::Command::new("systemd-inhibit")
        .args(["--what=sleep", &who_arg, &why_arg, "sleep", "infinity"])
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .ok()?;

    Some(PowerSaveHandle::ChildProcess(child))
}

pub fn release_blocker(handle: PowerSaveHandle) {
    match handle {
        PowerSaveHandle::ScreenSaverCookie(cookie) => uninhibit_screensaver(cookie),
        PowerSaveHandle::ChildProcess(mut child) => {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

fn parse_dbus_uint32(stdout: &[u8]) -> Option<u32> {
    let text = std::str::from_utf8(stdout).ok()?;
    for line in text.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("uint32") {
            return rest.trim().parse::<u32>().ok();
        }
    }
    None
}
