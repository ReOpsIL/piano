use crate::notation::{Note, NoteType};
use super::{DifficultyLevel, DifficultyClassifier};

#[derive(Debug, Clone)]
pub struct Song {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub difficulty: DifficultyLevel,
    pub notes: Vec<Note>,
    pub duration: f32, // in seconds
}

pub struct MusicLibrary {
    songs: Vec<Song>,
    current_song_index: Option<usize>,
}

impl MusicLibrary {
    pub fn new() -> Self {
        let mut library = Self {
            songs: Vec::new(),
            current_song_index: None,
        };
        
        library.load_default_songs();
        library
    }
    
    fn load_default_songs(&mut self) {
        // Create some basic practice songs
        self.songs.push(Song {
            id: "c_scale".to_string(),
            title: "C Major Scale".to_string(),
            artist: "Practice".to_string(),
            difficulty: DifficultyLevel::Beginner,
            notes: self.create_c_major_scale(),
            duration: 8.0,
        });
        
        self.songs.push(Song {
            id: "twinkle".to_string(),
            title: "Twinkle Twinkle Little Star".to_string(),
            artist: "Traditional".to_string(),
            difficulty: DifficultyLevel::Beginner,
            notes: self.create_twinkle_twinkle(),
            duration: 12.0,
        });
        
        self.songs.push(Song {
            id: "mary_lamb".to_string(),
            title: "Mary Had a Little Lamb".to_string(),
            artist: "Traditional".to_string(),
            difficulty: DifficultyLevel::Beginner,
            notes: self.create_mary_had_a_little_lamb(),
            duration: 10.0,
        });
    }
    
    fn create_c_major_scale(&self) -> Vec<Note> {
        vec![
            Note::new(60, NoteType::Quarter, 0.0), // C
            Note::new(62, NoteType::Quarter, 1.0), // D
            Note::new(64, NoteType::Quarter, 2.0), // E
            Note::new(65, NoteType::Quarter, 3.0), // F
            Note::new(67, NoteType::Quarter, 4.0), // G
            Note::new(69, NoteType::Quarter, 5.0), // A
            Note::new(71, NoteType::Quarter, 6.0), // B
            Note::new(72, NoteType::Quarter, 7.0), // C
        ]
    }
    
    fn create_twinkle_twinkle(&self) -> Vec<Note> {
        vec![
            Note::new(60, NoteType::Quarter, 0.0), // C
            Note::new(60, NoteType::Quarter, 1.0), // C
            Note::new(67, NoteType::Quarter, 2.0), // G
            Note::new(67, NoteType::Quarter, 3.0), // G
            Note::new(69, NoteType::Quarter, 4.0), // A
            Note::new(69, NoteType::Quarter, 5.0), // A
            Note::new(67, NoteType::Half, 6.0),    // G
            Note::new(65, NoteType::Quarter, 8.0), // F
            Note::new(65, NoteType::Quarter, 9.0), // F
            Note::new(64, NoteType::Quarter, 10.0), // E
            Note::new(64, NoteType::Quarter, 11.0), // E
            Note::new(62, NoteType::Quarter, 12.0), // D
            Note::new(62, NoteType::Quarter, 13.0), // D
            Note::new(60, NoteType::Half, 14.0),    // C
        ]
    }
    
    fn create_mary_had_a_little_lamb(&self) -> Vec<Note> {
        vec![
            Note::new(64, NoteType::Quarter, 0.0), // E
            Note::new(62, NoteType::Quarter, 1.0), // D
            Note::new(60, NoteType::Quarter, 2.0), // C
            Note::new(62, NoteType::Quarter, 3.0), // D
            Note::new(64, NoteType::Quarter, 4.0), // E
            Note::new(64, NoteType::Quarter, 5.0), // E
            Note::new(64, NoteType::Half, 6.0),    // E
            Note::new(62, NoteType::Quarter, 8.0), // D
            Note::new(62, NoteType::Quarter, 9.0), // D
            Note::new(62, NoteType::Half, 10.0),   // D
            Note::new(64, NoteType::Quarter, 12.0), // E
            Note::new(67, NoteType::Quarter, 13.0), // G
            Note::new(67, NoteType::Half, 14.0),    // G
        ]
    }
    
    pub fn get_songs(&self) -> &[Song] {
        &self.songs
    }
    
    pub fn get_songs_by_difficulty(&self, difficulty: DifficultyLevel) -> Vec<&Song> {
        self.songs.iter()
            .filter(|song| song.difficulty == difficulty)
            .collect()
    }
    
    pub fn get_song_by_id(&self, id: &str) -> Option<&Song> {
        self.songs.iter().find(|song| song.id == id)
    }
    
    pub fn select_song(&mut self, index: usize) -> Option<&Song> {
        if index < self.songs.len() {
            self.current_song_index = Some(index);
            Some(&self.songs[index])
        } else {
            None
        }
    }
    
    pub fn get_current_song(&self) -> Option<&Song> {
        self.current_song_index.map(|index| &self.songs[index])
    }
}