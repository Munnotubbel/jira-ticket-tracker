use egui::{Response, Ui};

pub struct TicketInput {
    pub text: String,
}

impl TicketInput {
    pub fn new() -> Self {
        Self {
            text: String::new(),
        }
    }

    pub fn show(&mut self, ui: &mut Ui) -> Response {
        egui::Frame::none()
            .fill(egui::Color32::WHITE)
            .inner_margin(egui::style::Margin::symmetric(4.0, 2.0))
            .outer_margin(egui::style::Margin::same(0.0))
            .show(ui, |ui| {
                ui.set_min_size(egui::vec2(80.0, 21.0));
                
                // Placeholder Text
                if self.text.is_empty() {
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

                let text_edit = egui::TextEdit::singleline(&mut self.text)
                    .desired_width(80.0)
                    .font(egui::FontId::monospace(12.0))
                    .text_color(egui::Color32::BLACK)
                    .margin(egui::vec2(4.0, 0.0));

                let response = ui.add(text_edit);
                
                // Nur die untere schwarze Linie zeichnen
                let line_rect = response.rect;
                ui.painter().line_segment(
                    [
                        line_rect.left_bottom() + egui::vec2(0.0, -2.0),
                        line_rect.right_bottom() + egui::vec2(0.0, -2.0)
                    ],
                    egui::Stroke::new(1.0, egui::Color32::BLACK)
                );

                response
            })
            .inner
    }
}
