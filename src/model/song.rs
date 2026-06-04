use serde::{Deserialize, Serialize};

/// Lightweight song reference, suitable for display in lists.
/// Requires no network request to obtain.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BriefSong {
    pub identifier: String,
    pub source: String,
    pub title: String,
    pub artists_name: String,
    pub album_name: String,
    /// Duration in milliseconds, as string (may be empty for unknown).
    pub duration_ms: String,
}

/// Full song model with all details populated.
/// Requires a network request (upgrade) to obtain.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Song {
    // --- Brief fields (always present) ---
    pub identifier: String,
    pub source: String,
    pub title: String,
    pub artists_name: String,
    pub album_name: String,
    pub duration_ms: String,
    // --- Normal fields (populated on upgrade) ---
    pub album: Option<super::BriefAlbum>,
    pub artists: Vec<super::BriefArtist>,
    /// Duration in milliseconds.
    pub duration: i64,
    pub genre: String,
    pub date: String,
    pub track: String,
    pub disc: String,
    pub pic_url: String,
}

impl BriefSong {
    pub fn into_song(self) -> Song {
        Song {
            identifier: self.identifier,
            source: self.source,
            title: self.title,
            artists_name: self.artists_name,
            album_name: self.album_name,
            duration_ms: self.duration_ms,
            album: None,
            artists: Vec::new(),
            duration: 0,
            genre: String::new(),
            date: String::new(),
            track: String::new(),
            disc: String::new(),
            pic_url: String::new(),
        }
    }
}

impl Song {
    pub fn brief(&self) -> BriefSong {
        BriefSong {
            identifier: self.identifier.clone(),
            source: self.source.clone(),
            title: self.title.clone(),
            artists_name: self.artists_name.clone(),
            album_name: self.album_name.clone(),
            duration_ms: self.duration_ms.clone(),
        }
    }
}
