use serde::{Deserialize, Serialize};

use super::base::SearchType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub source: String,
    pub query: String,
    pub search_type: SearchType,
    pub songs: Vec<super::BriefSong>,
    pub albums: Vec<super::BriefAlbum>,
    pub artists: Vec<super::BriefArtist>,
    pub playlists: Vec<super::BriefPlaylist>,
    pub err_msg: String,
}

impl SearchResult {
    pub fn empty(source: &str, query: &str, search_type: SearchType) -> Self {
        Self {
            source: source.to_string(),
            query: query.to_string(),
            search_type,
            songs: Vec::new(),
            albums: Vec::new(),
            artists: Vec::new(),
            playlists: Vec::new(),
            err_msg: String::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.songs.is_empty()
            && self.albums.is_empty()
            && self.artists.is_empty()
            && self.playlists.is_empty()
    }
}
