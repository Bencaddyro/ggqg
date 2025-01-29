use chrono::Local;
use std::fs;

pub fn write_to_file(content: &str, label: Option<&str>) -> std::io::Result<()> {
    let time = Local::now().timestamp_millis().to_string();
    let name = label.unwrap_or(&time);
    fs::write(format!("{name}.dot"), content)?;
    Ok(())
}
