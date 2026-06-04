use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BriefPlaylist {
    pub identifier: String,
    pub source: String,
    pub name: String,
    pub creator_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Playlist {
    // --- Brief fields ---
    pub identifier: String,
    pub source: String,
    pub name: String,
    pub creator_name: String,
    // --- Normal fields ---
    pub cover: String,
    pub description: String,
    pub play_count: u64,
    pub songs: Vec<super::BriefSong>,
    pub song_count: usize,
    pub created: String,
    pub updated: String,
}

impl BriefPlaylist {
    pub fn into_playlist(self) -> Playlist {
        Playlist {
            identifier: self.identifier,
            source: self.source,
            name: self.name,
            creator_name: self.creator_name,
            cover: String::new(),
            description: String::new(),
            play_count: 0,
            songs: Vec::new(),
            song_count: 0,
            created: String::new(),
            updated: String::new(),
        }
    }
}
