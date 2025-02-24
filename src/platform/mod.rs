#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::{play_sound, install_autostart, uninstall_autostart};

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::{play_sound, install_autostart, uninstall_autostart};

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::{play_sound, install_autostart, uninstall_autostart};

pub mod process;
