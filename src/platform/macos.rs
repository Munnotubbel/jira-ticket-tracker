#[cfg(target_os = "macos")]
use std::io::Cursor;
#[cfg(target_os = "macos")]
use rodio::{Decoder, OutputStream, Sink};

#[cfg(target_os = "macos")]
pub fn play_sound() {
    let sound_bytes = include_bytes!("../../assets/yeah.wav");
    
    std::thread::spawn(move || {
        if let Ok((_stream, stream_handle)) = OutputStream::try_default() {
            if let Ok(sink) = Sink::try_new(&stream_handle) {
                let cursor = Cursor::new(sound_bytes.to_vec());
                if let Ok(source) = Decoder::new(cursor) {
                    sink.append(source);
                    sink.sleep_until_end();
                }
            }
        }
    });
}

#[cfg(target_os = "macos")]
pub fn install_autostart() -> Result<(), Box<dyn std::error::Error>> {
    if let Some(home_dir) = dirs::home_dir() {
        let launch_agents_dir = home_dir.join("Library/LaunchAgents");
        std::fs::create_dir_all(&launch_agents_dir)?;
        
        let plist_file = launch_agents_dir.join("com.ticket-tracker.plist");
        let executable_path = std::env::current_exe()?;
        
        let plist_content = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
            <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
            <plist version="1.0">
            <dict>
                <key>Label</key>
                <string>com.ticket-tracker</string>
                <key>ProgramArguments</key>
                <array>
                    <string>{}</string>
                </array>
                <key>RunAtLoad</key>
                <true/>
                <key>KeepAlive</key>
                <false/>
                <key>ProcessType</key>
                <string>Interactive</string>
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
        let plist_file = home_dir.join("Library/LaunchAgents/com.ticket-tracker.plist");
        if plist_file.exists() {
            std::fs::remove_file(plist_file)?;
        }
    }
    Ok(())
}
