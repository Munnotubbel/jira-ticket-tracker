#[cfg(target_os = "windows")]
use std::io::Cursor;
#[cfg(target_os = "windows")]
use rodio::{Decoder, OutputStream, Sink};
#[cfg(target_os = "windows")]
use winreg::RegKey;

#[cfg(target_os = "windows")]
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

#[cfg(target_os = "windows")]
pub fn install_autostart() -> Result<(), Box<dyn std::error::Error>> {
    use winreg::enums::HKEY_CURRENT_USER;
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = r"Software\Microsoft\Windows\CurrentVersion\Run";
    let (key, _) = hkcu.create_subkey(path)?;
    
    let exe_path = std::env::current_exe()?;
    key.set_value("TicketTracker", &exe_path.to_str().unwrap())?;
    
    Ok(())
}

#[cfg(target_os = "windows")]
pub fn uninstall_autostart() -> Result<(), Box<dyn std::error::Error>> {
    use winreg::enums::{HKEY_CURRENT_USER, KEY_WRITE};
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = r"Software\Microsoft\Windows\CurrentVersion\Run";
    let key = hkcu.open_subkey_with_flags(path, KEY_WRITE)?;
    key.delete_value("TicketTracker")?;
    
    Ok(())
}
