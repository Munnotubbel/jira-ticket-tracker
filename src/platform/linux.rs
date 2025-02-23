use std::io::Write;

pub fn install_autostart() -> Result<(), Box<dyn std::error::Error>> {
    let home = std::env::var("HOME")?;
    let autostart_dir = format!("{}/.config/autostart", home);
    std::fs::create_dir_all(&autostart_dir)?;

    let desktop_file = format!("{}/ticket-ticker.desktop", autostart_dir);
    let mut file = std::fs::File::create(desktop_file)?;
    write!(file, r#"[Desktop Entry]
Type=Application
Name=TicketTicker
Exec=ticket-ticker
Terminal=false
Categories=Utility;
"#)?;

    Ok(())
}

pub fn uninstall_autostart() -> Result<(), Box<dyn std::error::Error>> {
    let home = std::env::var("HOME")?;
    let desktop_file = format!("{}/.config/autostart/ticket-ticker.desktop", home);
    if std::path::Path::new(&desktop_file).exists() {
        std::fs::remove_file(desktop_file)?;
    }
    Ok(())
}
