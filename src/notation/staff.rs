use eframe::egui::{self, Painter, Pos2, Color32, Stroke};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Clef {
    Treble,
    Bass,
}

#[derive(Debug, Clone)]
pub struct Staff {
    pub clef: Clef,
    pub position: Pos2,
    pub width: f32,
    pub line_spacing: f32,
}

impl Staff {
    pub fn new(clef: Clef, position: Pos2, width: f32) -> Self {
        Self {
            clef,
            position,
            width,
            line_spacing: 12.0,
        }
    }
    
    pub fn draw(&self, painter: &Painter) {
        let stroke = Stroke::new(1.0, Color32::BLACK);
        
        // Draw staff lines
        for i in 0..5 {
            let y = self.position.y + (i as f32) * self.line_spacing;
            painter.line_segment(
                [
                    Pos2::new(self.position.x, y),
                    Pos2::new(self.position.x + self.width, y),
                ],
                stroke,
            );
        }
        
        // Draw clef symbol (simplified)
        self.draw_clef_symbol(painter);
    }
    
    fn draw_clef_symbol(&self, painter: &Painter) {
        let clef_x = self.position.x + 10.0;
        let center_y = self.position.y + 2.0 * self.line_spacing;
        
        match self.clef {
            Clef::Treble => {
                // Simplified treble clef representation
                painter.circle_filled(
                    Pos2::new(clef_x, center_y),
                    8.0,
                    Color32::BLACK,
                );
                painter.text(
                    Pos2::new(clef_x - 4.0, center_y - 6.0),
                    egui::Align2::CENTER_CENTER,
                    "𝄞",
                    egui::FontId::proportional(20.0),
                    Color32::BLACK,
                );
            }
            Clef::Bass => {
                // Simplified bass clef representation
                painter.circle_filled(
                    Pos2::new(clef_x, center_y),
                    8.0,
                    Color32::BLACK,
                );
                painter.text(
                    Pos2::new(clef_x - 4.0, center_y - 6.0),
                    egui::Align2::CENTER_CENTER,
                    "𝄢",
                    egui::FontId::proportional(20.0),
                    Color32::BLACK,
                );
            }
        }
    }
    
    pub fn note_y_position(&self, midi_note: u8) -> f32 {
        match self.clef {
            Clef::Treble => {
                // Middle C (60) is below the staff
                let middle_c_y = self.position.y + 5.0 * self.line_spacing + self.line_spacing / 2.0;
                middle_c_y - ((midi_note as f32 - 60.0) * self.line_spacing / 2.0)
            }
            Clef::Bass => {
                // Middle C (60) is above the staff
                let middle_c_y = self.position.y - self.line_spacing / 2.0;
                middle_c_y - ((midi_note as f32 - 60.0) * self.line_spacing / 2.0)
            }
        }
    }
}