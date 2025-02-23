use chrono::{DateTime, Local};
use rust_xlsxwriter::*;
use std::path::PathBuf;
use calamine::{Reader, open_workbook_auto};

#[derive(Clone)]
pub struct ExcelHandler {
    path: PathBuf,
}

impl ExcelHandler {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn save_ticket(&self, ticket: &str, timestamp: DateTime<Local>) -> Result<(), Box<dyn std::error::Error>> {
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
        if self.path.exists() {
            if let Ok(mut excel) = open_workbook_auto(&self.path) {
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
        }

        // Neuen Eintrag anhängen
        worksheet.write_string(row, 0, &timestamp.format("%d-%m-%Y %H:%M").to_string())?;
        worksheet.write_string(row, 1, &ticket.to_uppercase())?;

        // Auto-filter setzen
        worksheet.autofilter(0, 0, row, 1)?;
        
        // Spaltenbreiten anpassen
        worksheet.set_column_width(0, 20.0)?; // Timestamp
        worksheet.set_column_width(1, 50.0)?; // Ticket

        // Speichern
        workbook.save(&self.path)?;
        
        Ok(())
    }
} 