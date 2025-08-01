use eframe::egui;

#[derive(Debug, Clone)]
pub struct AppSettings {
    pub midi_latency_compensation: f32,
    pub visual_feedback_duration: f32,
    pub auto_advance: bool,
    pub show_note_names: bool,
    pub metronome_enabled: bool,
    pub metronome_bpm: u32,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            midi_latency_compensation: 10.0,
            visual_feedback_duration: 1.0,
            auto_advance: true,
            show_note_names: false,
            metronome_enabled: false,
            metronome_bpm: 120,
        }
    }
}

pub struct SettingsWindow {
    settings: AppSettings,
}

impl SettingsWindow {
    pub fn new() -> Self {
        Self {
            settings: AppSettings::default(),
        }
    }
    
    pub fn show(&mut self, ctx: &egui::Context) {
        egui::Window::new("Settings")
            .default_size([400.0, 300.0])
            .show(ctx, |ui| {
                ui.heading("Application Settings");
                
                ui.separator();
                
                // MIDI Settings
                ui.collapsing("MIDI Settings", |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Latency Compensation (ms):");
                        ui.add(egui::Slider::new(&mut self.settings.midi_latency_compensation, 0.0..=50.0));
                    });
                });
                
                ui.separator();
                
                // Visual Settings
                ui.collapsing("Visual Settings", |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Feedback Duration (s):");
                        ui.add(egui::Slider::new(&mut self.settings.visual_feedback_duration, 0.5..=3.0));
                    });
                    
                    ui.checkbox(&mut self.settings.show_note_names, "Show note names");
                });
                
                ui.separator();
                
                // Gameplay Settings
                ui.collapsing("Gameplay Settings", |ui| {
                    ui.checkbox(&mut self.settings.auto_advance, "Auto-advance to next note");
                });
                
                ui.separator();
                
                // Metronome Settings
                ui.collapsing("Metronome Settings", |ui| {
                    ui.checkbox(&mut self.settings.metronome_enabled, "Enable metronome");
                    
                    if self.settings.metronome_enabled {
                        ui.horizontal(|ui| {
                            ui.label("BPM:");
                            ui.add(egui::Slider::new(&mut self.settings.metronome_bpm, 60..=200));
                        });
                    }
                });
                
                ui.separator();
                
                ui.horizontal(|ui| {
                    if ui.button("Reset to Defaults").clicked() {
                        self.settings = AppSettings::default();
                    }
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Save").clicked() {
                            // TODO: Save settings to file
                        }
                    });
                });
            });
    }
    
    pub fn get_settings(&self) -> &AppSettings {
        &self.settings
    }
}