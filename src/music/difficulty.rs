#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DifficultyLevel {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

impl DifficultyLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            DifficultyLevel::Beginner => "Beginner",
            DifficultyLevel::Intermediate => "Intermediate",
            DifficultyLevel::Advanced => "Advanced",
            DifficultyLevel::Expert => "Expert",
        }
    }
    
    pub fn color(&self) -> egui::Color32 {
        match self {
            DifficultyLevel::Beginner => egui::Color32::from_rgb(0, 150, 0),
            DifficultyLevel::Intermediate => egui::Color32::from_rgb(255, 165, 0),
            DifficultyLevel::Advanced => egui::Color32::from_rgb(255, 0, 0),
            DifficultyLevel::Expert => egui::Color32::from_rgb(128, 0, 128),
        }
    }
}

pub struct DifficultyClassifier;

impl DifficultyClassifier {
    pub fn classify_song(notes: &[crate::notation::Note]) -> DifficultyLevel {
        let note_count = notes.len();
        let unique_pitches = notes.iter()
            .map(|n| n.pitch)
            .collect::<std::collections::HashSet<_>>();
        let pitch_range = unique_pitches.len();
        
        // Simple classification based on note count and pitch range
        match (note_count, pitch_range) {
            (0..=8, 1..=5) => DifficultyLevel::Beginner,
            (9..=16, 1..=8) => DifficultyLevel::Intermediate,
            (17..=32, 1..=12) => DifficultyLevel::Advanced,
            _ => DifficultyLevel::Expert,
        }
    }
    
    pub fn estimate_practice_time(difficulty: DifficultyLevel) -> &'static str {
        match difficulty {
            DifficultyLevel::Beginner => "5-10 minutes",
            DifficultyLevel::Intermediate => "10-20 minutes",
            DifficultyLevel::Advanced => "20-45 minutes",
            DifficultyLevel::Expert => "45+ minutes",
        }
    }
}