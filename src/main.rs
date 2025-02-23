use chrono::{DateTime, Local};
use directories::UserDirs;
use eframe::egui;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use rust_xlsxwriter::*;
use regex;
use std::time::{Duration, Instant};
use calamine;
use calamine::{Reader, Xlsx};

#[derive(Serialize, Deserialize, Debug)]
struct AppConfig {
    window_pos_x: f32,
    window_pos_y: f32,
}

impl Default for AppConfig {
    fn default() -> Self {
        let pos = get_initial_window_position();
        Self {
            window_pos_x: pos.x,
            window_pos_y: pos.y,
        }
    }
}

fn get_initial_window_position() -> egui::Pos2 {
    let ctx = egui::Context::default();
    let screen_rect = ctx.input(|i| i.screen_rect);
    let window_width = 100.0;  // 144 * 0.7 ≈ 100
    let window_height = 161.0;  // (190 * 0.7) + 28 ≈ 161
    
    // Position am rechten Rand, vertikal zentriert
    let x = screen_rect.max.x - window_width;
    let y = (screen_rect.max.y - window_height) / 2.0;
    
    println!("Fenster wird positioniert am rechten Rand: ({}, {})", x, y);
    egui::pos2(x, y)
}

fn get_config_path() -> PathBuf {
    if let Some(user_dirs) = UserDirs::new() {
        if let Some(doc_dir) = user_dirs.document_dir() {
            return doc_dir.join("tickets.config");
        }
    }
    PathBuf::from("tickets.config")
}

fn save_config(config: &AppConfig) {
    let config_path = get_config_path();
    if let Ok(toml) = toml::to_string(config) {
        if let Err(e) = fs::write(&config_path, toml) {
            eprintln!("Fehler beim Speichern der Konfiguration: {}", e);
        }
    }
}

fn load_config() -> AppConfig {
    let config_path = get_config_path();
    if config_path.exists() {
        if let Ok(content) = fs::read_to_string(&config_path) {
            if let Ok(config) = toml::from_str(&content) {
                return config;
            }
        }
    }
    AppConfig::default()
}

struct MyApp {
    face_textures: Vec<egui::TextureHandle>,
    last_ticket_time: Option<DateTime<Local>>,
    ticket_input: String,
    csv_path: PathBuf,
    config: AppConfig,
    celebration_start: Option<Instant>,
    success_texture: Option<egui::TextureHandle>,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut face_textures = Vec::new();
        let face_images = vec![
            include_bytes!("../assets/face_0.png").as_ref(),
            include_bytes!("../assets/face_1.png").as_ref(),
            include_bytes!("../assets/face_2.png").as_ref(),
            include_bytes!("../assets/face_3.png").as_ref(),
            include_bytes!("../assets/face_4.png").as_ref(),
            include_bytes!("../assets/face_5.png").as_ref(),
            include_bytes!("../assets/face_6.png").as_ref(),
            include_bytes!("../assets/face_7.png").as_ref(),
        ];

        // Lade die eingebetteten Bilder
        for (i, image_bytes) in face_images.iter().enumerate() {
            if let Ok(img) = image::load_from_memory(image_bytes) {
                let img = img.to_rgba8();
                let size = [img.width() as usize, img.height() as usize];
                let pixels = img.into_raw();
                let texture = cc.egui_ctx.load_texture(
                    format!("face_{}", i),
                    egui::ColorImage::from_rgba_unmultiplied(size, &pixels),
                    Default::default(),
                );
                face_textures.push(texture);
            } else {
                eprintln!("Fehler beim Dekodieren von Bild {}", i);
            }
        }
        if face_textures.is_empty() {
            // Fallback: falls kein Bild geladen werden konnte, verwende einen Dummy-Text
            face_textures.push(cc.egui_ctx.load_texture(
                "dummy",
                egui::ColorImage::example(),
                Default::default(),
            ));
        }
        // CSV wird im typischen Dokumente-Ordner abgelegt
        let csv_path = if let Some(user_dirs) = UserDirs::new() {
            if let Some(doc_dir) = user_dirs.document_dir() {
                let path = doc_dir.join("tickets.csv");
                println!("CSV wird gespeichert unter: {}", path.display());
                path
            } else {
                println!("Konnte Dokumente-Ordner nicht finden, verwende lokales Verzeichnis");
                PathBuf::from("tickets.csv")
            }
        } else {
            println!("Konnte User-Verzeichnis nicht finden, verwende lokales Verzeichnis");
            PathBuf::from("tickets.csv")
        };

        // Lade das Success-Face
        let success_texture = if let Ok(img) = image::load_from_memory(include_bytes!("../assets/face_100.png")) {
            let img = img.to_rgba8();
            let size = [img.width() as usize, img.height() as usize];
            let pixels = img.into_raw();
            Some(cc.egui_ctx.load_texture(
                "success_face",
                egui::ColorImage::from_rgba_unmultiplied(size, &pixels),
                Default::default(),
            ))
        } else {
            None
        };

        Self {
            face_textures,
            last_ticket_time: None,
            ticket_input: String::new(),
            csv_path,
            config: load_config(),
            celebration_start: None,
            success_texture,
        }
    }

    fn calculate_face_index(&self) -> usize {
        let hours_since_last_ticket = self.last_ticket_time.map_or(0.0, |last_time| {
            let duration = Local::now() - last_time;
            duration.num_minutes() as f64 / 60.0
        });
        // 0 Stunden → Index 0; 14 Stunden → Index 7
        let index = (hours_since_last_ticket / 14.0 * 7.0).floor() as usize;
        index.min(7)  // Maximum Index ist jetzt 7
    }

    fn validate_jira_ticket(&self, ticket: &str) -> bool {
        // Format: 2-10 Buchstaben, Bindestrich, 1-6 Ziffern
        let re = regex::Regex::new(r"^[A-Za-z]{2,10}-[0-9]{1,6}$").unwrap();
        re.is_match(ticket.trim())
    }

    fn play_success_sound() {
        #[cfg(target_os = "windows")]
        {
            use winapi::um::mmsystem::PlaySoundW;
            use std::os::windows::ffi::OsStrExt;
            use std::ffi::OsStr;
            
            let sound_bytes = include_bytes!("../assets/yeah.wav");
            // Temporäre Datei erstellen
            if let Ok(mut temp_file) = std::fs::File::create("temp_sound.wav") {
                if std::io::Write::write_all(&mut temp_file, sound_bytes).is_ok() {
                    let wide: Vec<u16> = OsStr::new("temp_sound.wav")
                        .encode_wide()
                        .chain(std::iter::once(0))
                        .collect();
                    
                    unsafe {
                        PlaySoundW(
                            wide.as_ptr(),
                            std::ptr::null_mut(),
                            0x00020000 | 0x00000001, // SND_ASYNC | SND_FILENAME
                        );
                    }
                    // Kurz warten und dann aufräumen
                    std::thread::spawn(|| {
                        std::thread::sleep(std::time::Duration::from_secs(2));
                        let _ = std::fs::remove_file("temp_sound.wav");
                    });
                }
            }
        }

        #[cfg(target_os = "linux")]
        {
            use std::process::Command;
            // Sound-Bytes in temporäre Datei schreiben
            let sound_bytes = include_bytes!("../assets/yeah.wav");
            if let Ok(mut temp_file) = std::fs::File::create("temp_sound.wav") {
                if std::io::Write::write_all(&mut temp_file, sound_bytes).is_ok() {
                    let _ = Command::new("paplay")
                        .arg("temp_sound.wav")
                        .spawn();
                    // Kurz warten und dann aufräumen
                    std::thread::spawn(|| {
                        std::thread::sleep(std::time::Duration::from_secs(2));
                        let _ = std::fs::remove_file("temp_sound.wav");
                    });
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            // Sound-Bytes in temporäre Datei schreiben
            let sound_bytes = include_bytes!("../assets/yeah.wav");
            if let Ok(mut temp_file) = std::fs::File::create("temp_sound.wav") {
                if std::io::Write::write_all(&mut temp_file, sound_bytes).is_ok() {
                    let _ = Command::new("afplay")
                        .arg("temp_sound.wav")
                        .spawn();
                    // Kurz warten und dann aufräumen
                    std::thread::spawn(|| {
                        std::thread::sleep(std::time::Duration::from_secs(2));
                        let _ = std::fs::remove_file("temp_sound.wav");
                    });
                }
            }
        }
    }

    fn submit_ticket(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.ticket_input.trim().is_empty() {
            return Ok(());
        }

        if !self.validate_jira_ticket(&self.ticket_input) {
            println!("Ungültiges JIRA-Ticket Format. Bitte verwende das Format: PROJ-123");
            return Ok(());
        }

        // Starte Celebration
        self.celebration_start = Some(Instant::now());
        Self::play_success_sound();

        let now = Local::now();
        self.last_ticket_time = Some(now);
        
        // Konvertiere Ticket zu Großbuchstaben
        let ticket = self.ticket_input.trim().to_uppercase();
        
        // XLSX Handling
        let xlsx_path = self.csv_path.with_extension("xlsx");
        let mut workbook = Workbook::new();
        let worksheet = workbook.add_worksheet();
        
        // Header formatieren
        let header_format = Format::new()
            .set_bold()
            .set_border(FormatBorder::Thin);

        // Header schreiben
        worksheet.write_string_with_format(0, 0, "Timestamp", &header_format)?;
        worksheet.write_string_with_format(0, 1, "Ticket", &header_format)?;

        // Existierende Einträge laden
        let mut row = 1;
        if xlsx_path.exists() {
            let mut excel: Xlsx<_> = calamine::open_workbook(&xlsx_path)?;
            if let Some(Ok(range)) = excel.worksheet_range("Sheet1") {
                for row_index in 1..range.height() {  // Skip header row
                    if let Some(timestamp) = range.get((row_index, 0)) {
                        if let Some(old_ticket) = range.get((row_index, 1)) {
                            worksheet.write_string(row, 0, &timestamp.to_string())?;
                            worksheet.write_string(row, 1, &old_ticket.to_string())?;
                            row += 1;
                        }
                    }
                }
            }
        }

        // Neuen Eintrag anhängen
        worksheet.write_string(row, 0, &now.format("%d-%m-%Y %H:%M").to_string())?;
        worksheet.write_string(row, 1, &ticket)?;

        // Auto-filter setzen
        worksheet.autofilter(0, 0, row, 1)?;
        
        // Spaltenbreiten anpassen
        worksheet.set_column_width(0, 20.0)?; // Timestamp
        worksheet.set_column_width(1, 50.0)?; // Ticket

        // Speichern
        if let Err(e) = workbook.save(&xlsx_path) {
            eprintln!("Fehler beim Speichern der Excel-Datei: {}", e);
        }

        self.ticket_input.clear();
        Ok(())
    }

    fn install_autostart() -> Result<(), Box<dyn std::error::Error>> {
        let executable_path = std::env::current_exe()?;
        
        #[cfg(target_os = "windows")]
        {
            use std::os::windows::fs::symlink_file;
            let startup_folder = if let Some(appdata) = std::env::var_os("APPDATA") {
                PathBuf::from(appdata).join("Microsoft\\Windows\\Start Menu\\Programs\\Startup")
            } else {
                return Err("Could not find startup folder".into());
            };
            let link_path = startup_folder.join("TicketTicker.lnk");
            
            // Windows Shortcut erstellen
            let mut shortcut = std::fs::File::create(&link_path)?;
            writeln!(shortcut, "[InternetShortcut]\nURL=file:///{}", executable_path.display())?;
        }

        #[cfg(target_os = "linux")]
        {
            let home = std::env::var("HOME")?;
            let autostart_dir = PathBuf::from(&home).join(".config/autostart");
            std::fs::create_dir_all(&autostart_dir)?;
            
            let desktop_entry = format!(
                "[Desktop Entry]\n\
                Type=Application\n\
                Name=TicketTicker\n\
                Exec={}\n\
                Terminal=false\n\
                Categories=Utility;\n\
                X-GNOME-Autostart-enabled=true",
                executable_path.display()
            );
            
            std::fs::write(
                autostart_dir.join("ticket-ticker.desktop"),
                desktop_entry
            )?;
        }

        #[cfg(target_os = "macos")]
        {
            let home = std::env::var("HOME")?;
            let launch_agents = PathBuf::from(&home).join("Library/LaunchAgents");
            std::fs::create_dir_all(&launch_agents)?;
            
            let plist = format!(
                "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
                <!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">\n\
                <plist version=\"1.0\">\n\
                <dict>\n\
                    <key>Label</key>\n\
                    <string>com.ticketticker.app</string>\n\
                    <key>ProgramArguments</key>\n\
                    <array>\n\
                        <string>{}</string>\n\
                    </array>\n\
                    <key>RunAtLoad</key>\n\
                    <true/>\n\
                </dict>\n\
                </plist>",
                executable_path.display()
            );
            
            std::fs::write(
                launch_agents.join("com.ticketticker.app.plist"),
                plist
            )?;
        }

        Ok(())
    }

    fn uninstall_autostart() -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(target_os = "windows")]
        {
            if let Some(appdata) = std::env::var_os("APPDATA") {
                let link_path = PathBuf::from(appdata)
                    .join("Microsoft\\Windows\\Start Menu\\Programs\\Startup")
                    .join("TicketTicker.lnk");
                if link_path.exists() {
                    std::fs::remove_file(link_path)?;
                }
            }
        }

        #[cfg(target_os = "linux")]
        {
            if let Ok(home) = std::env::var("HOME") {
                let desktop_file = PathBuf::from(home)
                    .join(".config/autostart")
                    .join("ticket-ticker.desktop");
                if desktop_file.exists() {
                    std::fs::remove_file(desktop_file)?;
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            if let Ok(home) = std::env::var("HOME") {
                let plist_file = PathBuf::from(home)
                    .join("Library/LaunchAgents")
                    .join("com.ticketticker.app.plist");
                if plist_file.exists() {
                    std::fs::remove_file(plist_file)?;
                }
            }
        }

        Ok(())
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let window_pos = egui::pos2(self.config.window_pos_x, self.config.window_pos_y);
        let mut submit_requested = false;

        // UI-Logik in separater Funktion
        let response = {
            // Bestimme welches Gesicht angezeigt werden soll
            let current_face_index = self.calculate_face_index();
            let display_texture = if let Some(celebration_start) = self.celebration_start {
                if celebration_start.elapsed() < Duration::from_secs(1) {
                    // Zeige Success-Face während der Celebration
                    self.success_texture.as_ref().unwrap_or(&self.face_textures[0])
                } else {
                    // Celebration vorbei
                    self.celebration_start = None;
                    &self.face_textures[current_face_index]
                }
            } else {
                &self.face_textures[current_face_index]
            };

            egui::Area::new("draggable_area")
                .movable(true)
                .interactable(true)
                .default_pos(window_pos)
                .show(ctx, |ui| {
                    ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);
                    ui.spacing_mut().window_margin = egui::style::Margin::same(0.0);
                    ui.set_min_size(egui::vec2(80.0, 127.0));

                    ui.vertical(|ui| {
                        // Bildanzeige
                        let image_size = egui::vec2(80.0, 106.0);
                        ui.add(egui::Image::new(display_texture, image_size));
                        
                        // Weißes Eingabefeld
                        egui::Frame::none()
                            .fill(egui::Color32::WHITE)
                            .inner_margin(egui::style::Margin::symmetric(4.0, 2.0))
                            .outer_margin(egui::style::Margin::same(0.0))
                            .show(ui, |ui| {
                                ui.set_min_size(egui::vec2(80.0, 21.0));
                                
                                // Placeholder Text
                                if self.ticket_input.is_empty() {
                                    let placeholder_text = "PROJ-123";
                                    let placeholder_pos = ui.cursor().min;
                                    ui.painter().text(
                                        placeholder_pos + egui::vec2(4.0, 2.0),
                                        egui::Align2::LEFT_TOP,
                                        placeholder_text,
                                        egui::FontId::monospace(12.0),
                                        egui::Color32::from_gray(180),
                                    );
                                }

                                let text_edit = egui::TextEdit::singleline(&mut self.ticket_input)
                                    .desired_width(80.0)
                                    .font(egui::FontId::monospace(12.0))
                                    .text_color(egui::Color32::BLACK)
                                    .margin(egui::vec2(4.0, 0.0));

                                let response = ui.add(text_edit);
                                
                                // Schwarze Unterlinie
                                let line_rect = response.rect;
                                ui.painter().line_segment(
                                    [
                                        line_rect.left_bottom() + egui::vec2(0.0, -2.0),
                                        line_rect.right_bottom() + egui::vec2(0.0, -2.0)
                                    ],
                                    egui::Stroke::new(1.0, egui::Color32::BLACK)
                                );

                                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                                    submit_requested = true;
                                }
                            });
                    });
                })
        };

        // Handle submit outside of the UI closure
        if submit_requested {
            if let Err(e) = self.submit_ticket() {
                eprintln!("Fehler beim Speichern des Tickets: {}", e);
            }
        }

        // Position bei jedem Drag & Drop aktualisieren
        if response.response.dragged() {
            frame.drag_window();
        } else if response.response.drag_released() {
            if let Some(current_pos) = frame.info().window_info.position {
                self.config.window_pos_x = current_pos.x;
                self.config.window_pos_y = current_pos.y;
                save_config(&self.config);
            }
        }

        // Request repaint wenn celebration aktiv ist
        if self.celebration_start.is_some() {
            ctx.request_repaint();
        }
    }

    fn on_close_event(&mut self) -> bool {
        // Finale Speicherung der Position beim Beenden
        save_config(&self.config);
        true
    }
}

fn main() -> Result<(), eframe::Error> {
    // Verarbeite Kommandozeilenargumente
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "--install" => {
                if let Err(e) = MyApp::install_autostart() {
                    eprintln!("Installation failed: {}", e);
                    std::process::exit(1);
                }
                println!("Installation successful!");
                std::process::exit(0);
            }
            "--uninstall" => {
                if let Err(e) = MyApp::uninstall_autostart() {
                    eprintln!("Uninstallation failed: {}", e);
                    std::process::exit(1);
                }
                println!("Uninstallation successful!");
                std::process::exit(0);
            }
            "--quit" => {
                // Finde und beende alle laufenden Instanzen
                #[cfg(target_os = "linux")]
                {
                    use std::process::Command;
                    Command::new("pkill")
                        .arg("-f")
                        .arg("ticket-ticker")
                        .output()
                        .ok();
                }
                #[cfg(target_os = "windows")]
                {
                    use std::process::Command;
                    Command::new("taskkill")
                        .arg("/F")
                        .arg("/IM")
                        .arg("ticket-ticker.exe")
                        .output()
                        .ok();
                }
                #[cfg(target_os = "macos")]
                {
                    use std::process::Command;
                    Command::new("pkill")
                        .arg("-f")
                        .arg("ticket-ticker")
                        .output()
                        .ok();
                }
                std::process::exit(0);
            }
            "--help" | "-h" => {
                println!("TicketTicker - Ein Tool zum Tracken von JIRA-Tickets

USAGE:
    ticket-ticker [OPTIONS]

OPTIONS:
    -h, --help       Zeigt diese Hilfe an
    --install        Installiert die App für den Autostart
    --uninstall      Entfernt die App aus dem Autostart
    --quit          Beendet alle laufenden Instanzen der App

Ohne Optionen wird die App normal gestartet.");
                std::process::exit(0);
            }
            _ => {}
        }
    }

    // Normale App-Ausführung
    let initial_pos = get_initial_window_position();
    
    let native_options = eframe::NativeOptions {
        always_on_top: true,
        initial_window_size: Some(egui::vec2(80.0, 127.0)),
        initial_window_pos: Some(initial_pos),
        centered: false,
        decorated: false,
        transparent: true,
        default_theme: eframe::Theme::Light,
        follow_system_theme: false,
        hardware_acceleration: eframe::HardwareAcceleration::Required,
        active: true,
        ..Default::default()
    };

    // Plattformspezifische Anpassungen
    #[cfg(target_os = "macos")]
    {
        native_options.window_level = Some(3); // NSFloatingWindowLevel
    }

    eframe::run_native(
        "Ticket App",
        native_options,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    )
}