use eframe::egui::{self, Ui, Rect, Pos2};
use crate::game::GameEngine;
use super::{Staff, Clef};

pub struct NotationRenderer {
    treble_staff: Staff,
    bass_staff: Staff,
}

impl NotationRenderer {
    pub fn new() -> Self {
        Self {
            treble_staff: Staff::new(Clef::Treble, Pos2::ZERO, 800.0),
            bass_staff: Staff::new(Clef::Bass, Pos2::ZERO, 800.0),
        }
    }
    
    pub fn render(&mut self, ui: &mut Ui, rect: Rect, game_engine: &GameEngine) {
        let painter = ui.painter();
        
        // Update staff positions based on available rect
        let staff_width = rect.width() - 40.0;
        let treble_y = rect.min.y + 20.0;
        let bass_y = rect.min.y + 120.0;
        
        self.treble_staff.position = Pos2::new(rect.min.x + 20.0, treble_y);
        self.treble_staff.width = staff_width;
        
        self.bass_staff.position = Pos2::new(rect.min.x + 20.0, bass_y);
        self.bass_staff.width = staff_width;
        
        // Draw staves
        self.treble_staff.draw(&painter);
        self.bass_staff.draw(&painter);
        
        // Draw current notes from game engine
        self.draw_current_notes(&painter, game_engine);
    }
    
    fn draw_current_notes(&self, painter: &egui::Painter, game_engine: &GameEngine) {
        let current_notes = game_engine.get_current_notes();
        
        for (i, note) in current_notes.iter().enumerate() {
            let staff = if note.pitch >= 60 { &self.treble_staff } else { &self.bass_staff };
            let x = staff.position.x + 80.0 + (i as f32 * 60.0);
            let y = staff.note_y_position(note.pitch);
            
            note.draw(painter, x, y);
        }
    }
}