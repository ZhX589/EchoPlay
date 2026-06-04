use std::any::Any;
use std::collections::HashMap;

use async_trait::async_trait;

use crate::error::EchoError;
use crate::media::{AudioQuality, Media};
use crate::model::*;
use crate::provider::Provider;

use super::api::QQMusicApi;

/// QQ Music provider.
///
/// Uses the QQ Music RPC gateway for search, song detail, and media URLs.
/// Lyrics are fetched from the legacy REST endpoint.
pub struct QQProvider {
    api: QQMusicApi,
}

impl QQProvider {
    pub fn new() -> Self {
        Self {
            api: QQMusicApi::new(),
        }
    }

    pub fn with_cookies(cookies: HashMap<String, String>) -> Self {
        Self {
            api: QQMusicApi::with_cookies(cookies),
        }
    }
}

#[async_trait]
impl Provider for QQProvider {
    fn identifier(&self) -> &str {
        "qq"
    }

    fn name(&self) -> &str {
        "QQ Music"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    async fn search(
        &self,
        keyword: &str,
        search_type: SearchType,
    ) -> Result<SearchResult, EchoError> {
        self.api.search(keyword, search_type).await
    }

    async fn song_get(&self, identifier: &str) -> Result<Song, EchoError> {
        self.api.get_song_detail(identifier).await
    }

    async fn song_get_lyric(&self, song: &BriefSong) -> Result<Option<Lyric>, EchoError> {
        self.api.get_lyric(&song.identifier).await
    }

    async fn song_get_media(
        &self,
        song: &BriefSong,
        quality: AudioQuality,
    ) -> Result<Option<Media>, EchoError> {
        // media_mid is stored in duration_ms field as a hack, or we need to
        // re-fetch the song detail to get it. For now, use identifier as media_mid
        // (QQ Music songmid == media_mid in many cases).
        // A proper implementation would cache the FileInfo from song_get.
        self.api
            .get_song_media_url(&song.identifier, &song.identifier, quality)
            .await
    }
}

impl Default for QQProvider {
    fn default() -> Self {
        Self::new()
    }
}
