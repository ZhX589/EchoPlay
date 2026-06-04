use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString, EnumIter, Serialize, Deserialize,
)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ModelType {
    Song,
    Album,
    Artist,
    Playlist,
    Lyric,
    Video,
    User,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString, EnumIter, Serialize, Deserialize,
)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum SearchType {
    Song,
    Album,
    Artist,
    Playlist,
    Video,
}

impl SearchType {
    pub fn from_model_type(mt: ModelType) -> Self {
        match mt {
            ModelType::Song => Self::Song,
            ModelType::Album => Self::Album,
            ModelType::Artist => Self::Artist,
            ModelType::Playlist => Self::Playlist,
            ModelType::Video => Self::Video,
            _ => Self::Song,
        }
    }
}

impl ModelType {
    pub fn plural(&self) -> &'static str {
        match self {
            Self::Song => "songs",
            Self::Album => "albums",
            Self::Artist => "artists",
            Self::Playlist => "playlists",
            Self::Lyric => "lyrics",
            Self::Video => "videos",
            Self::User => "users",
        }
    }

    pub fn from_plural(s: &str) -> Option<Self> {
        match s {
            "songs" => Some(Self::Song),
            "albums" => Some(Self::Album),
            "artists" => Some(Self::Artist),
            "playlists" => Some(Self::Playlist),
            "lyrics" => Some(Self::Lyric),
            "videos" => Some(Self::Video),
            "users" => Some(Self::User),
            _ => None,
        }
    }
}
