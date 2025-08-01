use eframe::egui;
use crate::music::{MusicLibrary, DifficultyLevel};

pub struct SongBrowser {
    selected_difficulty: Option<DifficultyLevel>,
    selected_song_index: Option<usize>,
}

impl SongBrowser {
    pub fn new() -> Self {
        Self {
            selected_difficulty: None,
            selected_song_index: None,
        }
    }
    
    pub fn show(&mut self, ctx: &egui::Context, music_library: &mut MusicLibrary) -> Option<String> {
        let mut selected_song_id = None;
        
        egui::Window::new("Song Browser")
            .default_size([600.0, 400.0])
            .show(ctx, |ui| {
                ui.heading("Choose a Song to Practice");
                
                ui.separator();
                
                // Difficulty filter
                ui.horizontal(|ui| {
                    ui.label("Filter by difficulty:");
                    
                    if ui.selectable_label(self.selected_difficulty.is_none(), "All").clicked() {
                        self.selected_difficulty = None;
                    }
                    
                    for difficulty in [DifficultyLevel::Beginner, DifficultyLevel::Intermediate, DifficultyLevel::Advanced, DifficultyLevel::Expert] {
                        if ui.selectable_label(
                            self.selected_difficulty == Some(difficulty),
                            difficulty.as_str()
                        ).clicked() {
                            self.selected_difficulty = Some(difficulty);
                        }
                    }
                });
                
                ui.separator();
                
                // Song list
                egui::ScrollArea::vertical().show(ui, |ui| {
                    let songs = if let Some(difficulty) = self.selected_difficulty {
                        music_library.get_songs_by_difficulty(difficulty)
                    } else {
                        music_library.get_songs().iter().collect()
                    };
                    
                    for (index, song) in songs.iter().enumerate() {
                        let is_selected = self.selected_song_index == Some(index);
                        
                        ui.horizontal(|ui| {                            
                            if ui.selectable_label(is_selected, &song.title).clicked() {
                                self.selected_song_index = Some(index);
                            }
                            
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.colored_label(song.difficulty.color(), song.difficulty.as_str());
                                ui.label(&song.artist);
                            });
                        });
                        
                        if is_selected {
                            ui.indent("song_details", |ui| {
                                ui.small(&format!("Duration: {:.1}s", song.duration));
                                ui.small(&format!("Notes: {}", song.notes.len()));
                                
                                if ui.button("Start Practice").clicked() {
                                    selected_song_id = Some(song.id.clone());
                                }
                            });
                        }
                        
                        ui.separator();
                    }
                });
            });
        
        selected_song_id
    }
}