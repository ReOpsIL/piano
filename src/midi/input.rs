use midir::{MidiInput as MidirInput, MidiInputConnection, MidiInputPort};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use super::MidiDevice;

#[derive(Debug, Clone)]
pub enum EventType {
    NoteOn,
    NoteOff,
}

#[derive(Debug, Clone)]
pub struct MidiEvent {
    pub note: u8,
    pub velocity: u8,
    pub timestamp: u64,
    pub event_type: EventType,
}

pub struct MidiInput {
    _connection: Option<MidiInputConnection<()>>,
    events: Arc<Mutex<Vec<MidiEvent>>>,
    connected: bool,
    current_device: Option<MidiDevice>,
}

impl MidiInput {
    pub fn new(events: Arc<Mutex<Vec<MidiEvent>>>) -> Self {
        let mut midi_input = Self {
            _connection: None,
            events,
            connected: false,
            current_device: None,
        };
        
        midi_input.connect_to_first_available();
        midi_input
    }
    
    pub fn is_connected(&self) -> bool {
        self.connected
    }
    
    pub fn get_current_device(&self) -> Option<&MidiDevice> {
        self.current_device.as_ref()
    }
    
    pub fn connect_to_device(&mut self, device: &MidiDevice) -> Result<(), String> {
        // Disconnect current connection
        self.disconnect();
        
        let midi_in = match MidirInput::new("Piano App") {
            Ok(input) => input,
            Err(e) => return Err(format!("Failed to create MIDI input: {}", e)),
        };
        
        let ports = midi_in.ports();
        if device.port_index >= ports.len() {
            return Err("Device port index out of range".to_string());
        }
        
        let port = &ports[device.port_index];
        let port_name = midi_in.port_name(port).unwrap_or_else(|_| "Unknown".to_string());
        log::info!("Connecting to MIDI port: {}", port_name);
        
        let events = self.events.clone();
        let connection = midi_in.connect(
            port,
            "piano-input",
            move |timestamp, message, _| {
                if let Some(event) = Self::parse_midi_message(message, timestamp) {
                    if let Ok(mut events) = events.lock() {
                        events.push(event);
                    }
                }
            },
            (),
        );
        
        match connection {
            Ok(conn) => {
                self._connection = Some(conn);
                self.connected = true;
                self.current_device = Some(device.clone());
                log::info!("Successfully connected to MIDI device: {}", port_name);
                Ok(())
            }
            Err(e) => {
                log::error!("Failed to connect to MIDI device: {}", e);
                Err(format!("Connection failed: {}", e))
            }
        }
    }
    
    pub fn disconnect(&mut self) {
        if let Some(_conn) = self._connection.take() {
            self.connected = false;
            self.current_device = None;
            log::info!("Disconnected from MIDI device");
        }
    }
    
    fn connect_to_first_available(&mut self) {
        let devices = MidiDevice::list_available();
        if let Some(device) = devices.first() {
            let _ = self.connect_to_device(device);
        }
    }
    
    fn parse_midi_message(message: &[u8], _timestamp: u64) -> Option<MidiEvent> {
        if message.len() < 3 {
            return None;
        }
        
        let status = message[0];
        let note = message[1];
        let velocity = message[2];
        
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        
        match status & 0xF0 {
            0x90 if velocity > 0 => Some(MidiEvent {
                note,
                velocity,
                timestamp,
                event_type: EventType::NoteOn,
            }),
            0x80 | 0x90 => Some(MidiEvent {
                note,
                velocity,
                timestamp,
                event_type: EventType::NoteOff,
            }),
            _ => None,
        }
    }
}