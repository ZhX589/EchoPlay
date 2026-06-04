pub mod registry;

use std::any::Any;

use async_trait::async_trait;

use crate::error::{EchoError, MediaNotFoundReason, NotFoundReason};
use crate::media::{AudioQuality, Media};
use crate::model::*;

/// Base trait for all music source providers.
///
/// Each method has a default implementation that returns "not supported".
/// Concrete providers override only the capabilities they support.
#[async_trait]
pub trait Provider: Send + Sync + Any {
    /// Unique identifier, e.g. "qq", "netease", "local".
    fn identifier(&self) -> &str;

    /// Human-readable display name, e.g. "QQ Music".
    fn name(&self) -> &str;

    fn as_any(&self) -> &dyn Any;

    // -- Song capabilities --

    async fn song_get(&self, _identifier: &str) -> Result<Song, EchoError> {
        Err(EchoError::ModelNotFound {
            reason: NotFoundReason::NotSupported,
        })
    }

    async fn song_list_quality(&self, _song: &BriefSong) -> Result<Vec<AudioQuality>, EchoError> {
        Err(EchoError::MediaNotFound {
            reason: MediaNotFoundReason::NotFound,
        })
    }

    async fn song_get_media(
        &self,
        _song: &BriefSong,
        _quality: AudioQuality,
    ) -> Result<Option<Media>, EchoError> {
        Err(EchoError::MediaNotFound {
            reason: MediaNotFoundReason::NotFound,
        })
    }

    /// Select the best media by quality sort policy.
    /// Default tries qualities in policy order and returns the first hit.
    async fn song_select_media(
        &self,
        song: &BriefSong,
        policy: Option<&str>,
    ) -> Result<(Media, AudioQuality), EchoError> {
        let qualities = self.song_list_quality(song).await?;
        let sorted = crate::media::QualitySortPolicy::apply_audio(policy.unwrap_or("hq<>"));
        for q in sorted {
            if qualities.contains(&q)
                && let Some(media) = self.song_get_media(song, q).await?
            {
                return Ok((media, q));
            }
        }
        Err(EchoError::MediaNotFound {
            reason: MediaNotFoundReason::NotFound,
        })
    }

    async fn song_get_lyric(&self, _song: &BriefSong) -> Result<Option<Lyric>, EchoError> {
        Ok(None)
    }

    // -- Album capabilities --

    async fn album_get(&self, _identifier: &str) -> Result<Album, EchoError> {
        Err(EchoError::ModelNotFound {
            reason: NotFoundReason::NotSupported,
        })
    }

    async fn album_list_songs(&self, _album: &BriefAlbum) -> Result<Vec<Song>, EchoError> {
        Err(EchoError::ModelNotFound {
            reason: NotFoundReason::NotSupported,
        })
    }

    // -- Artist capabilities --

    async fn artist_get(&self, _identifier: &str) -> Result<Artist, EchoError> {
        Err(EchoError::ModelNotFound {
            reason: NotFoundReason::NotSupported,
        })
    }

    async fn artist_list_songs(&self, _artist: &BriefArtist) -> Result<Vec<Song>, EchoError> {
        Err(EchoError::ModelNotFound {
            reason: NotFoundReason::NotSupported,
        })
    }

    async fn artist_list_albums(&self, _artist: &BriefArtist) -> Result<Vec<Album>, EchoError> {
        Err(EchoError::ModelNotFound {
            reason: NotFoundReason::NotSupported,
        })
    }

    // -- Playlist capabilities --

    async fn playlist_get(&self, _identifier: &str) -> Result<Playlist, EchoError> {
        Err(EchoError::ModelNotFound {
            reason: NotFoundReason::NotSupported,
        })
    }

    async fn playlist_list_songs(&self, _playlist: &BriefPlaylist) -> Result<Vec<Song>, EchoError> {
        Err(EchoError::ModelNotFound {
            reason: NotFoundReason::NotSupported,
        })
    }

    // -- Search --

    async fn search(
        &self,
        _keyword: &str,
        _search_type: SearchType,
    ) -> Result<SearchResult, EchoError> {
        Err(EchoError::ModelNotFound {
            reason: NotFoundReason::NotSupported,
        })
    }
}
