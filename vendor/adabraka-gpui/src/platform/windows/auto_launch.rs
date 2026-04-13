use anyhow::Result;

const RUN_KEY: &str = r"SOFTWARE\Microsoft\Windows\CurrentVersion\Run";

pub(crate) fn set_auto_launch(app_id: &str, enabled: bool) -> Result<()> {
    let key = windows_registry::CURRENT_USER.create(RUN_KEY)?;
    if enabled {
        let exe_path = std::env::current_exe()?;
        key.set_string(app_id, &exe_path.to_string_lossy())?;
    } else {
        let _ = key.remove_value(app_id);
    }
    Ok(())
}

pub(crate) fn is_auto_launch_enabled(app_id: &str) -> bool {
    let Ok(key) = windows_registry::CURRENT_USER.open(RUN_KEY) else {
        return false;
    };
    key.get_string(app_id).is_ok()
}
