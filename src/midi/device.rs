use midir::{MidiInput, MidiInputPort};

#[derive(Debug, Clone, PartialEq)]
pub struct MidiDevice {
    pub name: String,
    pub port_index: usize,
}

impl MidiDevice {
    pub fn list_available() -> Vec<MidiDevice> {
        let midi_in = match MidiInput::new("Piano Device Scanner") {
            Ok(input) => input,
            Err(_) => return Vec::new(),
        };
        
        let ports = midi_in.ports();
        let mut devices = Vec::new();
        
        for (index, port) in ports.iter().enumerate() {
            if let Ok(name) = midi_in.port_name(port) {
                devices.push(MidiDevice {
                    name,
                    port_index: index,
                });
            }
        }
        
        devices
    }
    
    pub fn get_display_name(&self) -> String {
        if self.name.is_empty() {
            format!("MIDI Device {}", self.port_index)
        } else {
            self.name.clone()
        }
    }
}