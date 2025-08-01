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
        // Create a simple practice sequence
        let practice_notes = vec![
            Note::new(60, NoteType::Quarter, 0.0), // C4
            Note::new(62, NoteType::Quarter, 1.0), // D4
            Note::new(64, NoteType::Quarter, 2.0), // E4
            Note::new(65, NoteType::Quarter, 3.0), // F4
            Note::new(67, NoteType::Quarter, 4.0), // G4
        ];
        
        Self {
            state: GameState::Stopped,
            current_notes: practice_notes,
            current_position: 0,
            pressed_keys: HashSet::new(),
            correct_notes: 0,
            total_notes: 5,
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