#[cfg(target_os = "macos")]
pub fn play_sound() {
    println!("Sound-Effekt (nicht verfügbar auf macOS)");
}

#[cfg(target_os = "macos")]
pub fn install_autostart() -> Result<(), Box<dyn std::error::Error>> {
    if let Some(home_dir) = dirs::home_dir() {
        let launch_agents_dir = home_dir.join("Library/LaunchAgents");
        std::fs::create_dir_all(&launch_agents_dir)?;
        
        let plist_file = launch_agents_dir.join("com.ticket-ticker.plist");
        let executable_path = std::env::current_exe()?;
        
        let plist_content = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
            <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
            <plist version="1.0">
            <dict>
                <key>Label</key>
                <string>com.ticket-ticker</string>
                <key>ProgramArguments</key>
                <array>
                    <string>{}</string>
                </array>
                <key>RunAtLoad</key>
                <true/>
                <key>KeepAlive</key>
                <false/>
            </dict>
            </plist>"#,
            executable_path.display()
        );
        
        std::fs::write(plist_file, plist_content)?;
    }
    Ok(())
}

#[cfg(target_os = "macos")]
pub fn uninstall_autostart() -> Result<(), Box<dyn std::error::Error>> {
    if let Some(home_dir) = dirs::home_dir() {
        let plist_file = home_dir.join("Library/LaunchAgents/com.ticket-ticker.plist");
        if plist_file.exists() {
            std::fs::remove_file(plist_file)?;
        }
    }
    Ok(())
}
