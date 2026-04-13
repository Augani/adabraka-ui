use anyhow::Result;

pub fn show_notification(title: &str, body: &str) -> Result<()> {
    notify_rust::Notification::new()
        .summary(title)
        .body(body)
        .show()
        .map_err(|e| anyhow::anyhow!("Failed to show notification: {}", e))?;
    Ok(())
}
