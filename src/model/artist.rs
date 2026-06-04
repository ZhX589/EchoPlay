use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BriefArtist {
    pub identifier: String,
    pub source: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Artist {
    // --- Brief fields ---
    pub identifier: String,
    pub source: String,
    pub name: String,
    // --- Normal fields ---
    pub pic_url: String,
    pub aliases: Vec<String>,
    pub hot_songs: Vec<super::BriefSong>,
    pub description: String,
    pub song_count: usize,
    pub album_count: usize,
}

impl BriefArtist {
    pub fn into_artist(self) -> Artist {
        Artist {
            identifier: self.identifier,
            source: self.source,
            name: self.name,
            pic_url: String::new(),
            aliases: Vec::new(),
            hot_songs: Vec::new(),
            description: String::new(),
            song_count: 0,
            album_count: 0,
        }
    }
}
