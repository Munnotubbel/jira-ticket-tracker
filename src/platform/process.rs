use std::process::Command;

pub fn quit_all() {
    #[cfg(target_os = "linux")]
    {
        Command::new("pkill")
            .arg("-f")
            .arg("ticket-tracker")
            .output()
            .ok();
    }

    #[cfg(target_os = "windows")]
    {
        Command::new("taskkill")
            .arg("/F")
            .arg("/IM")
            .arg("ticket-tracker.exe")
            .output()
            .ok();
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("pkill")
            .arg("-f")
            .arg("ticket-tracker")
            .output()
            .ok();
    }

    std::process::exit(0);
} 