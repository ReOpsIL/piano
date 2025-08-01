use eframe::egui;
use std::sync::{Arc, Mutex};

use crate::midi::{MidiInput, MidiEvent, MidiDevice};
use crate::notation::NotationRenderer;
use crate::game::GameEngine;
use crate::music::MusicLibrary;

pub struct PianoApp {
    midi_input: Arc<Mutex<MidiInput>>,
    notation_renderer: NotationRenderer,
    game_engine: GameEngine,
    music_library: MusicLibrary,
    midi_events: Arc<Mutex<Vec<MidiEvent>>>,
    available_devices: Vec<MidiDevice>,
    selected_device_index: Option<usize>,
    show_device_selector: bool,
}

impl PianoApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let midi_events = Arc::new(Mutex::new(Vec::new()));
        let midi_input = Arc::new(Mutex::new(MidiInput::new(midi_events.clone())));
        let available_devices = MidiDevice::list_available();
        
        Self {
            midi_input,
            notation_renderer: NotationRenderer::new(),
            game_engine: GameEngine::new(),
            music_library: MusicLibrary::new(),
            midi_events,
            available_devices,
            selected_device_index: None,
            show_device_selector: false,
        }
    }
    
    fn refresh_devices(&mut self) {
        self.available_devices = MidiDevice::list_available();
        self.selected_device_index = None;
    }
    
    fn connect_to_selected_device(&mut self) {
        if let Some(index) = self.selected_device_index {
            if index < self.available_devices.len() {
                let device = &self.available_devices[index];
                if let Ok(mut midi_input) = self.midi_input.lock() {
                    match midi_input.connect_to_device(device) {
                        Ok(()) => {
                            log::info!("Connected to device: {}", device.get_display_name());
                        }
                        Err(e) => {
                            log::error!("Failed to connect to device: {}", e);
                        }
                    }
                }
            }
        }
    }
}

impl eframe::App for PianoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Process MIDI events
        if let Ok(mut events) = self.midi_events.lock() {
            for event in events.drain(..) {
                self.game_engine.process_midi_event(&event);
            }
        }

        // Main application UI
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Piano Sight Reading");
            
            // MIDI device selection and status
            ui.horizontal(|ui| {
                ui.label("MIDI Device:");
                
                // Device selector button
                let current_device_name = if let Ok(midi) = self.midi_input.lock() {
                    if let Some(device) = midi.get_current_device() {
                        device.get_display_name()
                    } else {
                        "No device selected".to_string()
                    }
                } else {
                    "Error".to_string()
                };
                
                if ui.button(&current_device_name).clicked() {
                    self.show_device_selector = true;
                    self.refresh_devices();
                }
                
                // Connection status
                if let Ok(midi) = self.midi_input.lock() {
                    let connected = midi.is_connected();
                    ui.colored_label(
                        if connected { egui::Color32::GREEN } else { egui::Color32::RED },
                        if connected { "Connected" } else { "Disconnected" }
                    );
                }
                
                // Refresh button
                if ui.button("🔄 Refresh").clicked() {
                    self.refresh_devices();
                }
            });
            
            ui.separator();
            
            // Music notation area
            let notation_rect = ui.available_rect_before_wrap();
            if notation_rect.height() > 200.0 {
                let notation_response = ui.allocate_rect(
                    egui::Rect::from_min_size(
                        notation_rect.min,
                        egui::Vec2::new(notation_rect.width(), 300.0)
                    ),
                    egui::Sense::hover()
                );
                
                self.notation_renderer.render(ui, notation_response.rect, &self.game_engine);
            }
            
            ui.separator();
            
            // Game controls
            ui.horizontal(|ui| {
                if ui.button("Start Practice").clicked() {
                    self.game_engine.start_practice();
                }
                
                if ui.button("Pause").clicked() {
                    self.game_engine.pause();
                }
                
                if ui.button("Reset").clicked() {
                    self.game_engine.reset();
                }
            });
            
            // Progress display
            ui.horizontal(|ui| {
                ui.label("Progress:");
                let progress = self.game_engine.get_progress();
                ui.add(egui::ProgressBar::new(progress).show_percentage());
            });
        });

        // Device selector popup
        if self.show_device_selector {
            egui::Window::new("Select MIDI Device")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label("Available MIDI devices:");
                    ui.separator();
                    
                    if self.available_devices.is_empty() {
                        ui.label("No MIDI devices found");
                        ui.small("Make sure your MIDI device is connected and try refreshing.");
                    } else {
                        for (index, device) in self.available_devices.iter().enumerate() {
                            let is_selected = self.selected_device_index == Some(index);
                            if ui.selectable_label(is_selected, device.get_display_name()).clicked() {
                                self.selected_device_index = Some(index);
                            }
                        }
                    }
                    
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("Connect").clicked() {
                            if self.selected_device_index.is_some() {
                                self.connect_to_selected_device();
                                self.show_device_selector = false;
                            }
                        }
                        
                        if ui.button("Refresh").clicked() {
                            self.refresh_devices();
                        }
                        
                        if ui.button("Cancel").clicked() {
                            self.show_device_selector = false;
                        }
                    });
                });
        }

        // Request repaint for real-time updates
        ctx.request_repaint();
    }
}