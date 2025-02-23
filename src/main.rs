mod app;
mod platform;
mod config;
mod ui;
mod utils;

use app::ticket_tracker::TicketTracker;

fn main() -> Result<(), eframe::Error> {
    // process command line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "--install" => {
                if let Err(e) = TicketTracker::install_autostart() {
                    eprintln!("Installation failed: {}", e);
                    std::process::exit(1);
                }
                println!("Installation successful!");
                std::process::exit(0);
            }
            "--uninstall" => {
                if let Err(e) = TicketTracker::uninstall_autostart() {
                    eprintln!("Uninstallation failed: {}", e);
                    std::process::exit(1);
                }
                println!("Uninstallation successful!");
                std::process::exit(0);
            }
            "--quit" => platform::process::quit_all(),
            "--help" | "-h" => {
                println!("Ticket Tracker - A tool to track JIRA tickets

USAGE:
    ticket-ticker [OPTIONS]

OPTIONS:
    -h, --help       shows this help
    --install        installs the app for autostart
    --uninstall      removes the app from autostart
    --quit           quits all running instances of the app

Without options, the app will start normally.");
                std::process::exit(0);
            }
            _ => {}
        }
    }

    let native_options = ui::window::create_window_options();
    
    eframe::run_native(
        "Ticket App",
        native_options,
        Box::new(|cc| Box::new(TicketTracker::new(cc))),
    )
}