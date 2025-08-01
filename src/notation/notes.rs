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
        
        // Draw ledger lines if needed
        self.draw_ledger_lines(painter, x, y);
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
    
    fn draw_ledger_lines(&self, _painter: &Painter, _x: f32, _y: f32) {
        // TODO: Implement ledger lines for notes outside the staff
    }
}