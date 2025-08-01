use midly::{Smf, Track, TrackEventKind, MidiMessage};
use crate::notation::{Note, NoteType};
use std::collections::HashMap;

pub struct MidiParser;

impl MidiParser {
    pub fn parse_midi_file(data: &[u8]) -> Result<Vec<Note>, Box<dyn std::error::Error>> {
        let smf = Smf::parse(data)?;
        let mut notes = Vec::new();
        let ticks_per_beat = match smf.header.timing {
            midly::Timing::Metrical(tpb) => tpb.as_int(),
            midly::Timing::Timecode(_, _) => 96, // Default fallback
        };
        
        for track in &smf.tracks {
            let track_notes = Self::parse_track(track, ticks_per_beat)?;
            notes.extend(track_notes);
        }
        
        // Sort notes by time position
        notes.sort_by(|a, b| a.position.partial_cmp(&b.position).unwrap());
        
        Ok(notes)
    }
    
    fn parse_track(track: &Track, ticks_per_beat: u16) -> Result<Vec<Note>, Box<dyn std::error::Error>> {
        let mut notes = Vec::new();
        let mut current_time = 0u32;
        let mut note_on_events: HashMap<u8, u32> = HashMap::new();
        
        for event in track {
            current_time += event.delta.as_int();
            
            if let TrackEventKind::Midi { channel: _, message } = &event.kind {
                match message {
                    MidiMessage::NoteOn { key, vel } => {
                        if *vel > 0 {
                            note_on_events.insert((*key).into(), current_time);
                        } else {
                            // Note off (velocity 0)
                            let key_u8: u8 = (*key).into();
                            if let Some(start_time) = note_on_events.remove(&key_u8) {
                                let duration_ticks = current_time - start_time;
                                let duration_beats = duration_ticks as f32 / ticks_per_beat as f32;
                                let note_type = Self::duration_to_note_type(duration_beats);
                                let position = start_time as f32 / ticks_per_beat as f32;
                                
                                notes.push(Note::new(key_u8, note_type, position));
                            }
                        }
                    }
                    MidiMessage::NoteOff { key, vel: _ } => {
                        let key_u8: u8 = (*key).into();
                        if let Some(start_time) = note_on_events.remove(&key_u8) {
                            let duration_ticks = current_time - start_time;
                            let duration_beats = duration_ticks as f32 / ticks_per_beat as f32;
                            let note_type = Self::duration_to_note_type(duration_beats);
                            let position = start_time as f32 / ticks_per_beat as f32;
                            
                            notes.push(Note::new(key_u8, note_type, position));
                        }
                    }
                    _ => {}
                }
            }
        }
        
        Ok(notes)
    }
    
    fn duration_to_note_type(duration_beats: f32) -> NoteType {
        if duration_beats >= 3.5 {
            NoteType::Whole
        } else if duration_beats >= 1.5 {
            NoteType::Half
        } else if duration_beats >= 0.75 {
            NoteType::Quarter
        } else {
            NoteType::Eighth
        }
    }
}