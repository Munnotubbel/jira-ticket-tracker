use std::process::Command;

pub fn quit_all() {
    #[cfg(target_os = "linux")]
    {
        Command::new("pkill")
            .arg("-f")
            .arg("ticket-ticker")
            .output()
            .ok();
    }

    #[cfg(target_os = "windows")]
    {
        Command::new("taskkill")
            .arg("/F")
            .arg("/IM")
            .arg("ticket-ticker.exe")
            .output()
            .ok();
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("pkill")
            .arg("-f")
            .arg("ticket-ticker")
            .output()
            .ok();
    }

    std::process::exit(0);
} 