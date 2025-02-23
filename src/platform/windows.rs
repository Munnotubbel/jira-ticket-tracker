#[cfg(target_os = "windows")]
use std::ptr::null_mut;
#[cfg(target_os = "windows")]
use winapi::shared::minwindef::{DWORD, HMODULE};

#[cfg(target_os = "windows")]
pub fn play_sound() {
    const SND_ASYNC: DWORD = 0x0001;
    const SND_MEMORY: DWORD = 0x0004;
    const SND_NODEFAULT: DWORD = 0x0002;  // Verhindert Standard-Lautstärke
    
    let sound_bytes = include_bytes!("../../assets/yeah.wav");
    
    extern "system" {
        fn PlaySoundA(
            pszSound: *const i8,
            hmod: HMODULE,
            fdwSound: DWORD,
        ) -> i32;
        
        fn waveOutSetVolume(
            hwo: HMODULE,
            dwVolume: DWORD,
        ) -> i32;
    }
    
    unsafe {
        // Setze Lautstärke auf 50%
        waveOutSetVolume(null_mut(), 0x40004000);  // 50% für beide Kanäle
        
        PlaySoundA(
            sound_bytes.as_ptr() as *const i8,
            null_mut(),
            SND_MEMORY | SND_ASYNC | SND_NODEFAULT
        );
    }
}

#[cfg(target_os = "windows")]
pub fn install_autostart() -> Result<(), Box<dyn std::error::Error>> {
    // windows-specific autostart code
    Ok(())
}

#[cfg(target_os = "windows")]
pub fn uninstall_autostart() -> Result<(), Box<dyn std::error::Error>> {
    // windows-specific uninstall code
    Ok(())
}
