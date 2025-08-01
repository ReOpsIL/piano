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
        let ledger_line_length = 18.0; // Slightly longer than note head
        let ledger_half_length = ledger_line_length / 2.0;
        
        // Determine if note is above or below staff
        if y < staff_top - line_spacing / 4.0 {
            // Note is above the staff - draw ledger lines above
            self.draw_ledger_lines_above(painter, x, y, staff_top, line_spacing, ledger_half_length, stroke);
        } else if y > staff_bottom + line_spacing / 4.0 {
            // Note is below the staff - draw ledger lines below  
            self.draw_ledger_lines_below(painter, x, y, staff_bottom, line_spacing, ledger_half_length, stroke);
        }
        // If note is within staff range, no ledger lines needed
    }
    
    fn draw_ledger_lines_above(&self, painter: &Painter, x: f32, note_y: f32, staff_top: f32, line_spacing: f32, ledger_half_length: f32, stroke: Stroke) {
        // Calculate the first ledger line position above the staff
        let first_ledger_y = staff_top - line_spacing;
        
        // Find which ledger line position the note is closest to
        let mut current_ledger_y = first_ledger_y;
        
        // Continue drawing ledger lines until we've covered the note's position
        while current_ledger_y >= note_y - line_spacing {
            // Check if we need a ledger line at this position
            if self.note_requires_ledger_line_at(note_y, current_ledger_y, line_spacing) {
                painter.line_segment(
                    [
                        Pos2::new(x - ledger_half_length, current_ledger_y),
                        Pos2::new(x + ledger_half_length, current_ledger_y),
                    ],
                    stroke,
                );
            }
            current_ledger_y -= line_spacing;
        }
    }
    
    fn draw_ledger_lines_below(&self, painter: &Painter, x: f32, note_y: f32, staff_bottom: f32, line_spacing: f32, ledger_half_length: f32, stroke: Stroke) {
        // Calculate the first ledger line position below the staff
        let first_ledger_y = staff_bottom + line_spacing;
        
        // Find which ledger line position the note is closest to
        let mut current_ledger_y = first_ledger_y;
        
        // Continue drawing ledger lines until we've covered the note's position
        while current_ledger_y <= note_y + line_spacing {
            // Check if we need a ledger line at this position
            if self.note_requires_ledger_line_at(note_y, current_ledger_y, line_spacing) {
                painter.line_segment(
                    [
                        Pos2::new(x - ledger_half_length, current_ledger_y),
                        Pos2::new(x + ledger_half_length, current_ledger_y),
                    ],
                    stroke,
                );
            }
            current_ledger_y += line_spacing;
        }
    }
    
    fn note_requires_ledger_line_at(&self, note_y: f32, ledger_y: f32, line_spacing: f32) -> bool {
        let half_space = line_spacing / 2.0;
        let tolerance = line_spacing / 8.0; // Small tolerance for floating point comparison
        
        // Check if note is positioned on this ledger line
        if (note_y - ledger_y).abs() < tolerance {
            return true;
        }
        
        // Check if note is in the space immediately above or below this ledger line
        // Notes in spaces also need the bounding ledger lines
        let space_above = ledger_y - half_space;
        let space_below = ledger_y + half_space;
        
        if (note_y - space_above).abs() < tolerance || (note_y - space_below).abs() < tolerance {
            return true;
        }
        
        false
    }
}