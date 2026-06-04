use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BriefAlbum {
    pub identifier: String,
    pub source: String,
    pub name: String,
    pub artists_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Album {
    // --- Brief fields ---
    pub identifier: String,
    pub source: String,
    pub name: String,
    pub artists_name: String,
    // --- Normal fields ---
    pub cover: String,
    pub album_type: String,
    pub artists: Vec<super::BriefArtist>,
    pub songs: Vec<super::BriefSong>,
    pub song_count: usize,
    pub description: String,
    pub released: String,
}

impl BriefAlbum {
    pub fn into_album(self) -> Album {
        Album {
            identifier: self.identifier,
            source: self.source,
            name: self.name,
            artists_name: self.artists_name,
            cover: String::new(),
            album_type: String::new(),
            artists: Vec::new(),
            songs: Vec::new(),
            song_count: 0,
            description: String::new(),
            released: String::new(),
        }
    }
}
