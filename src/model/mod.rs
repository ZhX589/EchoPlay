pub mod album;
pub mod artist;
pub mod base;
pub mod lyric;
pub mod playlist;
pub mod search;
pub mod song;
pub mod uri;
pub mod user;

// Re-export core types at model level for convenience.
pub use album::{Album, BriefAlbum};
pub use artist::{Artist, BriefArtist};
pub use base::{ModelType, SearchType};
pub use lyric::Lyric;
pub use playlist::{BriefPlaylist, Playlist};
pub use search::SearchResult;
pub use song::{BriefSong, Song};
pub use uri::Uri;
pub use user::{BriefUser, User};
