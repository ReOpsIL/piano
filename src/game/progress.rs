use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongProgress {
    pub song_id: String,
    pub completion_percentage: f32,
    pub best_accuracy: f32,
    pub attempts: u32,
    pub last_played: u64, // timestamp
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerStats {
    pub total_notes_played: u32,
    pub correct_notes: u32,
    pub total_practice_time: u64, // in seconds
    pub songs_completed: u32,
}

pub struct ProgressTracker {
    song_progress: HashMap<String, SongProgress>,
    player_stats: PlayerStats,
}

impl ProgressTracker {
    pub fn new() -> Self {
        Self {
            song_progress: HashMap::new(),
            player_stats: PlayerStats {
                total_notes_played: 0,
                correct_notes: 0,
                total_practice_time: 0,
                songs_completed: 0,
            },
        }
    }
    
    pub fn update_song_progress(&mut self, song_id: String, correct: u32, total: u32) {
        let accuracy = if total > 0 { correct as f32 / total as f32 } else { 0.0 };
        let completion = if total > 0 { (correct as f32 / total as f32) * 100.0 } else { 0.0 };
        
        let progress = self.song_progress.entry(song_id.clone()).or_insert(SongProgress {
            song_id: song_id.clone(),
            completion_percentage: 0.0,
            best_accuracy: 0.0,
            attempts: 0,
            last_played: 0,
        });
        
        progress.completion_percentage = completion.max(progress.completion_percentage);
        progress.best_accuracy = accuracy.max(progress.best_accuracy);
        progress.attempts += 1;
        progress.last_played = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Update player stats
        self.player_stats.total_notes_played += total;
        self.player_stats.correct_notes += correct;
        
        if completion >= 100.0 {
            self.player_stats.songs_completed += 1;
        }
    }
    
    pub fn get_song_progress(&self, song_id: &str) -> Option<&SongProgress> {
        self.song_progress.get(song_id)
    }
    
    pub fn get_player_stats(&self) -> &PlayerStats {
        &self.player_stats
    }
    
    pub fn get_overall_accuracy(&self) -> f32 {
        if self.player_stats.total_notes_played > 0 {
            self.player_stats.correct_notes as f32 / self.player_stats.total_notes_played as f32
        } else {
            0.0
        }
    }
}