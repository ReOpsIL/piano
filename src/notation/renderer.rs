use eframe::egui::{self, Ui, Rect, Pos2};
use crate::game::GameEngine;
use super::{Staff, Clef};

pub struct StaffSystem {
    pub treble_staff: Staff,
    pub bass_staff: Staff,
    pub system_number: usize,
}

pub struct NotationRenderer {
    staff_systems: Vec<StaffSystem>,
    notes_per_system: usize,
    system_height: f32,
    system_spacing: f32,
}

impl NotationRenderer {
    pub fn new() -> Self {
        Self {
            staff_systems: Vec::new(),
            notes_per_system: 8, // Number of notes per staff system
            system_height: 120.0, // Height of each staff system (treble + bass + spacing)
            system_spacing: 40.0, // Spacing between systems
        }
    }
    
    pub fn calculate_content_height(&self, game_engine: &GameEngine) -> f32 {
        let notes = game_engine.get_current_notes();
        let num_systems = ((notes.len() as f32) / (self.notes_per_system as f32)).ceil() as usize;
        let num_systems = num_systems.max(1); // At least one system
        
        (num_systems as f32) * (self.system_height + self.system_spacing) + 40.0
    }

    pub fn render(&mut self, ui: &mut Ui, rect: Rect, game_engine: &GameEngine) {
        let painter = ui.painter();
        let notes = game_engine.get_current_notes();
        
        // Calculate number of systems needed
        let num_systems = ((notes.len() as f32) / (self.notes_per_system as f32)).ceil() as usize;
        let num_systems = num_systems.max(1);
        
        // Create staff systems if needed
        self.update_staff_systems(num_systems, rect);
        
        // Draw all staff systems
        for system in &self.staff_systems {
            system.treble_staff.draw(&painter);
            system.bass_staff.draw(&painter);
        }
        
        // Draw notes across multiple systems
        self.draw_notes_across_systems(&painter, game_engine);
    }
    
    fn update_staff_systems(&mut self, num_systems: usize, rect: Rect) {
        self.staff_systems.clear();
        let staff_width = rect.width() - 40.0;
        
        for i in 0..num_systems {
            let system_y = rect.min.y + 20.0 + (i as f32) * (self.system_height + self.system_spacing);
            let treble_y = system_y;
            let bass_y = system_y + 80.0; // 80 pixels between treble and bass staff
            
            let treble_staff = Staff::new(
                Clef::Treble,
                Pos2::new(rect.min.x + 20.0, treble_y),
                staff_width
            );
            
            let bass_staff = Staff::new(
                Clef::Bass,
                Pos2::new(rect.min.x + 20.0, bass_y),
                staff_width
            );
            
            self.staff_systems.push(StaffSystem {
                treble_staff,
                bass_staff,
                system_number: i,
            });
        }
    }
    
    fn draw_notes_across_systems(&self, painter: &egui::Painter, game_engine: &GameEngine) {
        let current_notes = game_engine.get_current_notes();
        
        for (i, note) in current_notes.iter().enumerate() {
            // Determine which system this note belongs to
            let system_index = i / self.notes_per_system;
            let note_index_in_system = i % self.notes_per_system;
            
            if system_index < self.staff_systems.len() {
                let system = &self.staff_systems[system_index];
                
                // Choose staff based on note pitch
                let staff = if note.pitch >= 60 { &system.treble_staff } else { &system.bass_staff };
                
                // Calculate horizontal position within the system
                let notes_width = staff.width - 160.0; // Leave space for clefs and margins
                let note_spacing = notes_width / (self.notes_per_system as f32);
                let x = staff.position.x + 80.0 + (note_index_in_system as f32 * note_spacing);
                let y = staff.note_y_position(note.pitch);
                
                // Draw note with ledger lines
                note.draw_with_staff_info(
                    painter, 
                    x, 
                    y, 
                    staff.get_staff_top(), 
                    staff.get_staff_bottom(), 
                    staff.get_line_spacing()
                );
            }
        }
    }
}