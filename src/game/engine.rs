use crate::midi::{MidiEvent, EventType};
use crate::notation::{Note, NoteType};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameState {
    Stopped,
    Playing,
    Paused,
}

pub struct GameEngine {
    state: GameState,
    current_notes: Vec<Note>,
    current_position: usize,
    pressed_keys: HashSet<u8>,
    correct_notes: u32,
    total_notes: u32,
}

impl GameEngine {
    pub fn new() -> Self {
        // Create a test sequence specifically for ledger line testing
        let practice_notes = vec![
            // System 1 - Treble clef ledger lines above
            Note::new(72, NoteType::Quarter, 0.0),  // C5 (first line above treble staff)
            Note::new(74, NoteType::Quarter, 1.0),  // D5 (space above treble staff)
            Note::new(76, NoteType::Quarter, 2.0),  // E5 (second line above treble staff)
            Note::new(77, NoteType::Quarter, 3.0),  // F5 (space)
            Note::new(79, NoteType::Quarter, 4.0),  // G5 (third line above)
            Note::new(81, NoteType::Quarter, 5.0),  // A5 (space)
            Note::new(83, NoteType::Quarter, 6.0),  // B5 (fourth line above)
            Note::new(84, NoteType::Quarter, 7.0),  // C6 (space)
            
            // System 2 - Notes on treble staff (no ledger lines needed)
            Note::new(64, NoteType::Quarter, 8.0),  // E4 (top staff line)
            Note::new(65, NoteType::Quarter, 9.0),  // F4 (space)
            Note::new(67, NoteType::Quarter, 10.0), // G4 (line)
            Note::new(69, NoteType::Quarter, 11.0), // A4 (space)
            Note::new(71, NoteType::Quarter, 12.0), // B4 (line)
            Note::new(60, NoteType::Quarter, 13.0), // C4 (Middle C - needs ledger line)
            Note::new(62, NoteType::Quarter, 14.0), // D4 (space below treble staff)
            Note::new(59, NoteType::Quarter, 15.0), // B3 (space)
            
            // System 3 - Bass clef notes and ledger lines below
            Note::new(57, NoteType::Quarter, 16.0), // A3 (top bass staff line)
            Note::new(55, NoteType::Quarter, 17.0), // G3 (space)
            Note::new(53, NoteType::Quarter, 18.0), // F3 (line)
            Note::new(52, NoteType::Quarter, 19.0), // E3 (space)
            Note::new(50, NoteType::Quarter, 20.0), // D3 (line)
            Note::new(48, NoteType::Quarter, 21.0), // C3 (bottom bass staff line)
            Note::new(47, NoteType::Quarter, 22.0), // B2 (first ledger line below bass)
            Note::new(45, NoteType::Quarter, 23.0), // A2 (space below bass)
        ];
        
        Self {
            state: GameState::Stopped,
            current_notes: practice_notes,
            current_position: 0,
            pressed_keys: HashSet::new(),
            correct_notes: 0,
            total_notes: 24,
        }
    }
    
    pub fn start_practice(&mut self) {
        self.state = GameState::Playing;
        self.current_position = 0;
        self.correct_notes = 0;
        self.pressed_keys.clear();
        
        // Reset all note states
        for note in &mut self.current_notes {
            note.is_correct = None;
        }
    }
    
    pub fn pause(&mut self) {
        match self.state {
            GameState::Playing => self.state = GameState::Paused,
            GameState::Paused => self.state = GameState::Playing,
            _ => {}
        }
    }
    
    pub fn reset(&mut self) {
        self.state = GameState::Stopped;
        self.current_position = 0;
        self.correct_notes = 0;
        self.pressed_keys.clear();
        
        for note in &mut self.current_notes {
            note.is_correct = None;
        }
    }
    
    pub fn process_midi_event(&mut self, event: &MidiEvent) {
        if self.state != GameState::Playing {
            return;
        }
        
        match event.event_type {
            EventType::NoteOn => {
                self.pressed_keys.insert(event.note);
                self.check_current_note(event.note);
            }
            EventType::NoteOff => {
                self.pressed_keys.remove(&event.note);
            }
        }
    }
    
    fn check_current_note(&mut self, pressed_note: u8) {
        if self.current_position >= self.current_notes.len() {
            return;
        }
        
        let current_note = &mut self.current_notes[self.current_position];
        
        if current_note.pitch == pressed_note {
            current_note.is_correct = Some(true);
            self.correct_notes += 1;
            self.current_position += 1;
        } else {
            current_note.is_correct = Some(false);
        }
    }
    
    pub fn get_current_notes(&self) -> &[Note] {
        &self.current_notes
    }
    
    pub fn get_progress(&self) -> f32 {
        if self.total_notes == 0 {
            return 0.0;
        }
        self.current_position as f32 / self.total_notes as f32
    }
    
    pub fn get_score(&self) -> (u32, u32) {
        (self.correct_notes, self.total_notes)
    }
}