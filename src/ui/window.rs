use eframe::NativeOptions;
use egui::{Pos2, Vec2};

pub fn create_window_options() -> NativeOptions {
    NativeOptions {
        always_on_top: true,
        initial_window_size: Some(Vec2::new(80.0, 127.0)),
        initial_window_pos: Some(Pos2::new(get_initial_window_position().x, get_initial_window_position().y)),
        centered: false,
        decorated: false,
        transparent: true,
        default_theme: eframe::Theme::Light,
        follow_system_theme: false,
        hardware_acceleration: eframe::HardwareAcceleration::Required,
        active: true,
        ..Default::default()
    }
}

fn get_initial_window_position() -> Vec2 {
    #[cfg(target_os = "windows")]
    {
        use winapi::um::winuser::{GetSystemMetrics, SM_CXSCREEN};
        let screen_width = unsafe { GetSystemMetrics(SM_CXSCREEN) };
        Vec2::new((screen_width - 80) as f32, 0.0)
    }

    #[cfg(not(target_os = "windows"))]
    {
        Vec2::new(1840.0, 0.0) // Standardposition für andere Betriebssysteme
    }
} 