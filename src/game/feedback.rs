use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct FeedbackEvent {
    pub message: String,
    pub color: egui::Color32,
    pub timestamp: Instant,
    pub duration: Duration,
}

pub struct FeedbackSystem {
    active_events: Vec<FeedbackEvent>,
}

impl FeedbackSystem {
    pub fn new() -> Self {
        Self {
            active_events: Vec::new(),
        }
    }
    
    pub fn add_correct_note_feedback(&mut self) {
        self.active_events.push(FeedbackEvent {
            message: "Correct!".to_string(),
            color: egui::Color32::from_rgb(0, 150, 0),
            timestamp: Instant::now(),
            duration: Duration::from_secs(1),
        });
    }
    
    pub fn add_incorrect_note_feedback(&mut self) {
        self.active_events.push(FeedbackEvent {
            message: "Try again".to_string(),
            color: egui::Color32::from_rgb(200, 0, 0),
            timestamp: Instant::now(),
            duration: Duration::from_secs(1),
        });
    }
    
    pub fn update(&mut self) {
        let now = Instant::now();
        self.active_events.retain(|event| {
            now.duration_since(event.timestamp) < event.duration
        });
    }
    
    pub fn render(&self, ui: &mut egui::Ui) {
        for event in &self.active_events {
            ui.colored_label(event.color, &event.message);
        }
    }
}