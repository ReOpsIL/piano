use eframe::egui::{self, Painter, Pos2, Color32, Stroke};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NoteType {
    Whole,
    Half,
    Quarter,
    Eighth,
}

#[derive(Debug, Clone)]
pub struct Note {
    pub pitch: u8,
    pub note_type: NoteType,
    pub position: f32,
    pub is_correct: Option<bool>, // None = not played, Some(true) = correct, Some(false) = incorrect
}

impl Note {
    pub fn new(pitch: u8, note_type: NoteType, position: f32) -> Self {
        Self {
            pitch,
            note_type,
            position,
            is_correct: None,
        }
    }
    
    pub fn draw(&self, painter: &Painter, x: f32, y: f32) {
        let color = match self.is_correct {
            None => Color32::BLACK,
            Some(true) => Color32::from_rgb(0, 150, 0),
            Some(false) => Color32::from_rgb(200, 0, 0),
        };
        
        match self.note_type {
            NoteType::Whole => self.draw_whole_note(painter, x, y, color),
            NoteType::Half => self.draw_half_note(painter, x, y, color),
            NoteType::Quarter => self.draw_quarter_note(painter, x, y, color),
            NoteType::Eighth => self.draw_eighth_note(painter, x, y, color),
        }
    }
    
    pub fn draw_with_staff_info(&self, painter: &Painter, x: f32, y: f32, staff_top: f32, staff_bottom: f32, line_spacing: f32) {
        let color = match self.is_correct {
            None => Color32::BLACK,
            Some(true) => Color32::from_rgb(0, 150, 0),
            Some(false) => Color32::from_rgb(200, 0, 0),
        };
        
        // Draw ledger lines first (so they appear behind the note)
        self.draw_ledger_lines(painter, x, y, staff_top, staff_bottom, line_spacing, color);
        
        // Draw the note
        match self.note_type {
            NoteType::Whole => self.draw_whole_note(painter, x, y, color),
            NoteType::Half => self.draw_half_note(painter, x, y, color),
            NoteType::Quarter => self.draw_quarter_note(painter, x, y, color),
            NoteType::Eighth => self.draw_eighth_note(painter, x, y, color),
        }
    }
    
    fn draw_whole_note(&self, painter: &Painter, x: f32, y: f32, color: Color32) {
        painter.circle_stroke(
            Pos2::new(x, y),
            6.0,
            Stroke::new(2.0, color),
        );
    }
    
    fn draw_half_note(&self, painter: &Painter, x: f32, y: f32, color: Color32) {
        painter.circle_stroke(
            Pos2::new(x, y),
            6.0,
            Stroke::new(2.0, color),
        );
        
        // Draw stem
        painter.line_segment(
            [Pos2::new(x + 6.0, y), Pos2::new(x + 6.0, y - 24.0)],
            Stroke::new(1.5, color),
        );
    }
    
    fn draw_quarter_note(&self, painter: &Painter, x: f32, y: f32, color: Color32) {
        painter.circle_filled(
            Pos2::new(x, y),
            6.0,
            color,
        );
        
        // Draw stem
        painter.line_segment(
            [Pos2::new(x + 6.0, y), Pos2::new(x + 6.0, y - 24.0)],
            Stroke::new(1.5, color),
        );
    }
    
    fn draw_eighth_note(&self, painter: &Painter, x: f32, y: f32, color: Color32) {
        painter.circle_filled(
            Pos2::new(x, y),
            6.0,
            color,
        );
        
        // Draw stem
        painter.line_segment(
            [Pos2::new(x + 6.0, y), Pos2::new(x + 6.0, y - 24.0)],
            Stroke::new(1.5, color),
        );
        
        // Draw flag (simplified)
        painter.line_segment(
            [Pos2::new(x + 6.0, y - 24.0), Pos2::new(x + 12.0, y - 18.0)],
            Stroke::new(2.0, color),
        );
    }
    
    fn draw_ledger_lines(&self, painter: &Painter, x: f32, y: f32, staff_top: f32, staff_bottom: f32, line_spacing: f32, color: Color32) {
        let stroke = Stroke::new(1.0, color);
        let ledger_line_length = 16.0; // Length of ledger lines
        let ledger_half_length = ledger_line_length / 2.0;
        
        // Check if note is above the staff (needs ledger lines above)
        if y < staff_top {
            // Calculate how many ledger lines above the staff we need
            let distance_above = staff_top - y;
            let lines_needed = ((distance_above / line_spacing) + 0.5) as i32;
            
            // Draw ledger lines above the staff
            for i in 1..=lines_needed {
                let ledger_y = staff_top - (i as f32 * line_spacing);
                // Only draw ledger line if the note is close to this line
                if (y - ledger_y).abs() < line_spacing / 4.0 {
                    painter.line_segment(
                        [
                            Pos2::new(x - ledger_half_length, ledger_y),
                            Pos2::new(x + ledger_half_length, ledger_y),
                        ],
                        stroke,
                    );
                }
            }
        }
        
        // Check if note is below the staff (needs ledger lines below)
        if y > staff_bottom {
            // Calculate how many ledger lines below the staff we need
            let distance_below = y - staff_bottom;
            let lines_needed = ((distance_below / line_spacing) + 0.5) as i32;
            
            // Draw ledger lines below the staff
            for i in 1..=lines_needed {
                let ledger_y = staff_bottom + (i as f32 * line_spacing);
                // Only draw ledger line if the note is close to this line
                if (y - ledger_y).abs() < line_spacing / 4.0 {
                    painter.line_segment(
                        [
                            Pos2::new(x - ledger_half_length, ledger_y),
                            Pos2::new(x + ledger_half_length, ledger_y),
                        ],
                        stroke,
                    );
                }
            }
        }
    }
}