use std::io::Write;
use std::process::Command;

pub fn play_sound() {
    let sound_bytes = include_bytes!("../../assets/yeah.wav");
    if let Ok(mut temp_file) = std::fs::File::create("temp_sound.wav") {
        if std::io::Write::write_all(&mut temp_file, sound_bytes).is_ok() {
            // Versuche zuerst paplay (PulseAudio)
            let paplay_result = Command::new("paplay")
                .arg("temp_sound.wav")
                .spawn();

            // Wenn paplay fehlschlägt, versuche aplay (ALSA)
            if paplay_result.is_err() {
                let _ = Command::new("aplay")
                    .arg("temp_sound.wav")
                    .spawn();
            }

            // Cleanup nach 2 Sekunden
            std::thread::spawn(|| {
                std::thread::sleep(std::time::Duration::from_secs(2));
                let _ = std::fs::remove_file("temp_sound.wav");
            });
        }
    }
}

pub fn install_autostart() -> Result<(), Box<dyn std::error::Error>> {
    let home = std::env::var("HOME")?;
    let autostart_dir = format!("{}/.config/autostart", home);
    std::fs::create_dir_all(&autostart_dir)?;

    let desktop_file = format!("{}/ticket-tracker.desktop", autostart_dir);
    let mut file = std::fs::File::create(desktop_file)?;
    write!(file, r#"[Desktop Entry]
Type=Application
Name=TicketTracker
Exec=ticket-tracker
Terminal=false
Categories=Utility;
"#)?;

    Ok(())
}

pub fn uninstall_autostart() -> Result<(), Box<dyn std::error::Error>> {
    let home = std::env::var("HOME")?;
    let desktop_file = format!("{}/.config/autostart/ticket-tracker.desktop", home);
    if std::path::Path::new(&desktop_file).exists() {
        std::fs::remove_file(desktop_file)?;
    }
    Ok(())
}
