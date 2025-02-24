use crate::{
    config::settings::Settings,
    platform,
    ui::components::TicketInput,
    utils::{
        helpers::validate_ticket_format,
        excel::ExcelHandler,
    },
};
use chrono::{DateTime, Local};
use eframe::egui;
use std::time::{Duration, Instant};
use image;

pub struct TicketTracker {
    face_textures: Vec<egui::TextureHandle>,
    success_texture: Option<egui::TextureHandle>,
    last_ticket_time: Option<DateTime<Local>>,
    input: TicketInput,
    settings: Settings,
    celebration_start: Option<Instant>,
    excel_handler: ExcelHandler,
}

impl TicketTracker {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut face_textures = Vec::new();
        let face_images = vec![
            include_bytes!("../../assets/face_0.png").as_ref(),
            include_bytes!("../../assets/face_1.png").as_ref(),
            include_bytes!("../../assets/face_2.png").as_ref(),
            include_bytes!("../../assets/face_3.png").as_ref(),
            include_bytes!("../../assets/face_4.png").as_ref(),
            include_bytes!("../../assets/face_5.png").as_ref(),
            include_bytes!("../../assets/face_6.png").as_ref(),
            include_bytes!("../../assets/face_7.png").as_ref(),
        ];

        // Load embedded images
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
            }
        }

        // Load success face
        let success_texture = if let Ok(img) = image::load_from_memory(include_bytes!("../../assets/face_100.png")) {
            let img = img.to_rgba8();
            let size = [img.width() as usize, img.height() as usize];
            let pixels = img.into_raw();
            Some(cc.egui_ctx.load_texture(
                "success_face",
                egui::ColorImage::from_rgba_unmultiplied(size, &pixels),
                Default::default(),
            ))
        } else {
            eprintln!("Error loading success face!");
            None
        };

        let settings = Settings::load().unwrap_or_default();
        let excel_handler = ExcelHandler::new(settings.excel_path.clone());
        
        // GUI-Stil konfigurieren
        let mut style = (*cc.egui_ctx.style()).clone();
        style.spacing.item_spacing = egui::vec2(1.0, 1.0);
        style.visuals.window_fill = egui::Color32::from_rgba_unmultiplied(255, 255, 255, 230);
        cc.egui_ctx.set_style(style);

        Self {
            face_textures,
            success_texture,
            last_ticket_time: Some(Local::now()),
            input: TicketInput::new(),
            settings,
            celebration_start: None,
            excel_handler,
        }
    }

    fn calculate_face_index(&self) -> usize {
        let hours_since_last_ticket = self.last_ticket_time.map_or(0.0, |last_time| {
            let duration = Local::now() - last_time;
            duration.num_minutes() as f64 / 60.0
        });
        let index = (hours_since_last_ticket / 14.0 * 7.0).floor() as usize;
        index.min(7)
    }

    fn submit_ticket(&mut self, ctx: &egui::Context) -> Result<(), String> {
        if self.input.text.trim().is_empty() {
            return Ok(());
        }

        if !validate_ticket_format(&self.input.text) {
            return Ok(());
        }

        // KRITISCH: Celebration SOFORT setzen
        self.celebration_start = Some(Instant::now());
        
        // SOFORT neu zeichnen
        ctx.request_repaint();
        
        // DANN erst den Sound
        if self.settings.sound_enabled {
            platform::play_sound();
        }

        // Rest der Verarbeitung
        let ticket = self.input.text.trim().to_uppercase();
        self.input.text.clear();
        
        let now = Local::now();
        self.last_ticket_time = Some(now);

        // Excel im Hintergrund
        let excel_handler = self.excel_handler.clone();
        std::thread::spawn(move || {
            if let Err(e) = excel_handler.save_ticket(&ticket, now) {
                eprintln!("Error saving ticket: {}", e);
            }
        });

        Ok(())
    }

    pub fn install_autostart() -> Result<(), Box<dyn std::error::Error>> {
        platform::install_autostart()
    }

    pub fn uninstall_autostart() -> Result<(), Box<dyn std::error::Error>> {
        platform::uninstall_autostart()
    }
}

impl eframe::App for TicketTracker {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let mut submit_requested = false;

        // Fenster verschiebbar machen
        frame.set_window_pos(self.settings.window_pos);
        frame.set_always_on_top(true);
        frame.drag_window();  // Aktiviert das Verschieben des Fensters

        let display_texture = if let Some(start_time) = self.celebration_start {
            if start_time.elapsed() < Duration::from_secs(1) {
                self.success_texture.as_ref().expect("Success-Face fehlt!")
            } else {
                self.celebration_start = None;
                &self.face_textures[self.calculate_face_index()]
            }
        } else {
            &self.face_textures[self.calculate_face_index()]
        };

        // Fester Container für den Inhalt
        egui::CentralPanel::default()
            .frame(egui::Frame::none())
            .show(ctx, |ui| {
                // WICHTIG: Spacing auf 0 setzen
                ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);
                ui.spacing_mut().window_margin = egui::style::Margin::same(0.0);
                ui.set_min_size(egui::vec2(80.0, 127.0));

                ui.vertical(|ui| {
                    // Face direkt zeichnen
                    ui.add(egui::Image::new(display_texture, egui::vec2(80.0, 106.0)));
                    
                    // Input ohne Abstand zum Face
                    if self.input.show(ui).lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        submit_requested = true;
                    }
                });
            });

        // Submit NACH dem Rendering
        if submit_requested {
            if let Err(e) = self.submit_ticket(ctx) {
                eprintln!("Fehler: {}", e);
            }
        }

        // WICHTIG: IMMER neu zeichnen während Celebration
        if self.celebration_start.is_some() {
            ctx.request_repaint();
        }
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        if let Err(e) = self.settings.save() {
            eprintln!("Fehler beim Speichern der Einstellungen: {}", e);
        }
    }
} 