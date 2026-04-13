use crate::{DialogKind, DialogOptions};

pub fn show_dialog(options: &DialogOptions) -> usize {
    if let Some(idx) = try_zenity(options) {
        return idx;
    }
    if let Some(idx) = try_kdialog(options) {
        return idx;
    }
    0
}

fn try_zenity(options: &DialogOptions) -> Option<usize> {
    let message = build_message(options);
    let button_count = options.buttons.len();

    if button_count <= 1 {
        let zenity_type = match options.kind {
            DialogKind::Error => "--error",
            DialogKind::Warning => "--warning",
            DialogKind::Info => "--info",
        };
        let _ = std::process::Command::new("zenity")
            .args([zenity_type, "--title", &options.title, "--text", &message])
            .output()
            .ok()?;
        return Some(0);
    }

    let mut cmd = std::process::Command::new("zenity");
    cmd.args(["--question", "--title", &options.title, "--text", &message]);

    cmd.args(["--ok-label", &options.buttons[0]]);
    cmd.args(["--cancel-label", &options.buttons[1]]);

    for button in options.buttons.iter().skip(2) {
        cmd.args(["--extra-button", button]);
    }

    let output = cmd.output().ok()?;

    if output.status.success() {
        return Some(0);
    }

    if button_count > 2 {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stdout_trimmed = stdout.trim();
        for (idx, button) in options.buttons.iter().enumerate().skip(2) {
            if stdout_trimmed == button.as_ref() {
                return Some(idx);
            }
        }
    }

    Some(1)
}

fn try_kdialog(options: &DialogOptions) -> Option<usize> {
    let message = build_message(options);
    let button_count = options.buttons.len();

    if button_count <= 1 {
        let kdialog_type = match options.kind {
            DialogKind::Error => "--error",
            DialogKind::Warning => "--sorry",
            DialogKind::Info => "--msgbox",
        };
        let _ = std::process::Command::new("kdialog")
            .args([kdialog_type, &message, "--title", &options.title])
            .output()
            .ok()?;
        return Some(0);
    }

    let mut args = vec!["--warningyesno".to_string(), message.clone()];
    if button_count >= 3 {
        args[0] = "--warningyesnocancel".to_string();
    }

    args.push("--title".to_string());
    args.push(options.title.to_string());

    if button_count >= 1 {
        args.push("--yes-label".to_string());
        args.push(options.buttons[0].to_string());
    }
    if button_count >= 2 {
        args.push("--no-label".to_string());
        args.push(options.buttons[1].to_string());
    }
    if button_count >= 3 {
        args.push("--cancel-label".to_string());
        args.push(options.buttons[2].to_string());
    }

    let output = std::process::Command::new("kdialog")
        .args(&args)
        .output()
        .ok()?;

    match output.status.code() {
        Some(0) => Some(0),
        Some(1) => Some(1),
        Some(2) => Some(2),
        _ => Some(0),
    }
}

fn build_message(options: &DialogOptions) -> String {
    match &options.detail {
        Some(detail) if !options.message.is_empty() => {
            format!("{}\n\n{}", options.message, detail)
        }
        Some(detail) => detail.to_string(),
        None => options.message.to_string(),
    }
}
