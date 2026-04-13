use crate::OsInfo;

pub fn get_os_info() -> OsInfo {
    OsInfo {
        name: "linux".into(),
        version: read_os_version().into(),
        arch: std::env::consts::ARCH.into(),
        locale: read_locale().into(),
        hostname: read_hostname().into(),
    }
}

fn read_os_version() -> String {
    let Ok(contents) = std::fs::read_to_string("/etc/os-release") else {
        return String::new();
    };
    for line in contents.lines() {
        if let Some(value) = line.strip_prefix("PRETTY_NAME=") {
            return value.trim_matches('"').to_string();
        }
    }
    String::new()
}

fn read_hostname() -> String {
    std::fs::read_to_string("/etc/hostname")
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|_| "unknown".to_string())
}

fn read_locale() -> String {
    std::env::var("LC_ALL")
        .or_else(|_| std::env::var("LANG"))
        .map(|locale| {
            locale
                .split('.')
                .next()
                .unwrap_or(&locale)
                .replace('_', "-")
        })
        .unwrap_or_else(|_| "en-US".to_string())
}
