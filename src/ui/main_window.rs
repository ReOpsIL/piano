use eframe::egui;

pub struct MainWindow {
    show_song_browser: bool,
    show_settings: bool,
}

impl MainWindow {
    pub fn new() -> Self {
        Self {
            show_song_browser: false,
            show_settings: false,
        }
    }
    
    pub fn show_menu_bar(&mut self, ui: &mut egui::Ui) {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Browse Songs").clicked() {
                    self.show_song_browser = true;
                    ui.close_menu();
                }
                
                if ui.button("Import MIDI").clicked() {
                    // TODO: Implement MIDI file import
                    ui.close_menu();
                }
                
                ui.separator();
                
                if ui.button("Exit").clicked() {
                    std::process::exit(0);
                }
            });
            
            ui.menu_button("Settings", |ui| {
                if ui.button("Preferences").clicked() {
                    self.show_settings = true;
                    ui.close_menu();
                }
                
                if ui.button("MIDI Devices").clicked() {
                    // TODO: Show MIDI device selection
                    ui.close_menu();
                }
            });
            
            ui.menu_button("Help", |ui| {
                if ui.button("About").clicked() {
                    // TODO: Show about dialog
                    ui.close_menu();
                }
            });
        });
    }
    
    pub fn should_show_song_browser(&self) -> bool {
        self.show_song_browser
    }
    
    pub fn should_show_settings(&self) -> bool {
        self.show_settings
    }
    
    pub fn close_song_browser(&mut self) {
        self.show_song_browser = false;
    }
    
    pub fn close_settings(&mut self) {
        self.show_settings = false;
    }
}