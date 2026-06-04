use std::sync::Arc;

use crate::error::EchoError;
use crate::media::Media;
use crate::model::*;
use crate::provider::Provider;
use crate::provider::registry::ProviderRegistry;
use crate::standby::{StandbyCandidate, StandbyMatcher};

/// Central facade for all music operations.
///
/// The Library holds a [`ProviderRegistry`] and dispatches calls to the
/// correct provider based on `model.source`. It also provides cross-provider
/// standby resolution when a song's media is unavailable.
pub struct Library {
    registry: ProviderRegistry,
    standby_matcher: StandbyMatcher,
}

impl Library {
    pub fn new(registry: ProviderRegistry) -> Self {
        Self {
            registry,
            standby_matcher: StandbyMatcher::default(),
        }
    }

    pub fn with_standby_score(mut self, min_score: f64) -> Self {
        self.standby_matcher = StandbyMatcher::new(min_score);
        self
    }

    /// Access the underlying provider registry.
    pub fn registry(&self) -> &ProviderRegistry {
        &self.registry
    }

    /// Get provider by source identifier.
    fn get_provider(&self, source: &str) -> Result<&Arc<dyn Provider>, EchoError> {
        self.registry
            .get(source)
            .ok_or_else(|| EchoError::ProviderNotFound(source.to_string()))
    }

    // -----------------------------------------------------------------------
    // Song operations
    // -----------------------------------------------------------------------

    /// Upgrade a BriefSong to a full Song.
    pub async fn song_upgrade(&self, song: &BriefSong) -> Result<Song, EchoError> {
        let provider = self.get_provider(&song.source)?;
        provider.song_get(&song.identifier).await
    }

    /// Prepare media for playback with quality policy.
    pub async fn song_prepare_media(
        &self,
        song: &BriefSong,
        policy: Option<&str>,
    ) -> Result<Media, EchoError> {
        let provider = self.get_provider(&song.source)?;
        let (media, _) = provider.song_select_media(song, policy).await?;
        Ok(media)
    }

    /// Get lyrics for a song.
    pub async fn song_get_lyric(&self, song: &BriefSong) -> Result<Option<Lyric>, EchoError> {
        let provider = self.get_provider(&song.source)?;
        provider.song_get_lyric(song).await
    }

    // -----------------------------------------------------------------------
    // Album operations
    // -----------------------------------------------------------------------

    /// Upgrade a BriefAlbum to a full Album.
    pub async fn album_upgrade(&self, album: &BriefAlbum) -> Result<Album, EchoError> {
        let provider = self.get_provider(&album.source)?;
        provider.album_get(&album.identifier).await
    }

    /// List songs in an album.
    pub async fn album_list_songs(&self, album: &BriefAlbum) -> Result<Vec<Song>, EchoError> {
        let provider = self.get_provider(&album.source)?;
        provider.album_list_songs(album).await
    }

    // -----------------------------------------------------------------------
    // Artist operations
    // -----------------------------------------------------------------------

    /// Upgrade a BriefArtist to a full Artist.
    pub async fn artist_upgrade(&self, artist: &BriefArtist) -> Result<Artist, EchoError> {
        let provider = self.get_provider(&artist.source)?;
        provider.artist_get(&artist.identifier).await
    }

    /// List songs by an artist.
    pub async fn artist_list_songs(&self, artist: &BriefArtist) -> Result<Vec<Song>, EchoError> {
        let provider = self.get_provider(&artist.source)?;
        provider.artist_list_songs(artist).await
    }

    /// List albums by an artist.
    pub async fn artist_list_albums(&self, artist: &BriefArtist) -> Result<Vec<Album>, EchoError> {
        let provider = self.get_provider(&artist.source)?;
        provider.artist_list_albums(artist).await
    }

    // -----------------------------------------------------------------------
    // Playlist operations
    // -----------------------------------------------------------------------

    /// Upgrade a BriefPlaylist to a full Playlist.
    pub async fn playlist_upgrade(&self, playlist: &BriefPlaylist) -> Result<Playlist, EchoError> {
        let provider = self.get_provider(&playlist.source)?;
        provider.playlist_get(&playlist.identifier).await
    }

    /// List songs in a playlist.
    pub async fn playlist_list_songs(
        &self,
        playlist: &BriefPlaylist,
    ) -> Result<Vec<Song>, EchoError> {
        let provider = self.get_provider(&playlist.source)?;
        provider.playlist_list_songs(playlist).await
    }

    // -----------------------------------------------------------------------
    // Search
    // -----------------------------------------------------------------------

    /// Search across multiple providers.
    ///
    /// If `source_in` is specified, only search those providers.
    /// Returns one `SearchResult` per provider that responded.
    pub async fn search(
        &self,
        keyword: &str,
        search_type: SearchType,
        source_in: Option<&[&str]>,
    ) -> Vec<Result<SearchResult, EchoError>> {
        let providers: Vec<_> = self
            .registry
            .list()
            .into_iter()
            .filter(|p| {
                source_in
                    .map(|sources| sources.contains(&p.identifier()))
                    .unwrap_or(true)
            })
            .collect();

        let mut results = Vec::new();
        for provider in providers {
            results.push(provider.search(keyword, search_type).await);
        }
        results
    }

    // -----------------------------------------------------------------------
    // Standby (cross-provider fallback)
    // -----------------------------------------------------------------------

    /// Find standby songs from other providers when the primary source fails.
    ///
    /// Searches providers in `source_in` (or all providers if None) for songs
    /// matching the original by title, artist, album, and duration.
    pub async fn find_standby(
        &self,
        original: &BriefSong,
        source_in: Option<&[&str]>,
    ) -> Vec<StandbyCandidate> {
        let query = format!("{} {}", original.title, original.artists_name);

        let providers: Vec<_> = self
            .registry
            .list()
            .into_iter()
            .filter(|p| {
                // Exclude the original source
                if p.identifier() == original.source {
                    return false;
                }
                source_in
                    .map(|sources| sources.contains(&p.identifier()))
                    .unwrap_or(true)
            })
            .collect();

        let mut all_candidates = Vec::new();

        for provider in providers {
            if let Ok(result) = provider.search(&query, SearchType::Song).await {
                all_candidates.extend(result.songs);
            }
        }

        self.standby_matcher.filter(original, all_candidates)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::NotFoundReason;
    use async_trait::async_trait;
    use std::any::Any;

    // -- Mock provider for testing --

    struct MockProvider {
        songs: std::collections::HashMap<String, Song>,
    }

    impl MockProvider {
        fn new() -> Self {
            let mut songs = std::collections::HashMap::new();
            songs.insert(
                "1".into(),
                Song {
                    identifier: "1".into(),
                    source: "mock".into(),
                    title: "Test Song".into(),
                    artists_name: "Test Artist".into(),
                    album_name: "Test Album".into(),
                    duration_ms: "240000".into(),
                    album: None,
                    artists: vec![],
                    duration: 240000,
                    genre: "Pop".into(),
                    date: "2024-01-01".into(),
                    track: "1".into(),
                    disc: "1".into(),
                    pic_url: String::new(),
                },
            );
            Self { songs }
        }
    }

    #[async_trait]
    impl Provider for MockProvider {
        fn identifier(&self) -> &str {
            "mock"
        }
        fn name(&self) -> &str {
            "Mock"
        }
        fn as_any(&self) -> &dyn Any {
            self
        }

        async fn song_get(&self, identifier: &str) -> Result<Song, EchoError> {
            self.songs
                .get(identifier)
                .cloned()
                .ok_or(EchoError::ModelNotFound {
                    reason: NotFoundReason::NotFound,
                })
        }

        async fn search(
            &self,
            keyword: &str,
            _search_type: SearchType,
        ) -> Result<SearchResult, EchoError> {
            let mut result = SearchResult::empty("mock", keyword, SearchType::Song);
            for song in self.songs.values() {
                if song.title.to_lowercase().contains(&keyword.to_lowercase())
                    || song
                        .artists_name
                        .to_lowercase()
                        .contains(&keyword.to_lowercase())
                {
                    result.songs.push(song.brief());
                }
            }
            Ok(result)
        }
    }

    fn setup_library() -> Library {
        let mut reg = ProviderRegistry::new();
        reg.register(Arc::new(MockProvider::new())).unwrap();
        Library::new(reg)
    }

    #[tokio::test]
    async fn test_song_upgrade() {
        let lib = setup_library();
        let brief = BriefSong {
            identifier: "1".into(),
            source: "mock".into(),
            title: "Test".into(),
            artists_name: "Artist".into(),
            album_name: "Album".into(),
            duration_ms: "240000".into(),
        };
        let song = lib.song_upgrade(&brief).await.unwrap();
        assert_eq!(song.title, "Test Song");
        assert_eq!(song.identifier, "1");
    }

    #[tokio::test]
    async fn test_song_upgrade_not_found() {
        let lib = setup_library();
        let brief = BriefSong {
            identifier: "999".into(),
            source: "mock".into(),
            title: "X".into(),
            artists_name: "X".into(),
            album_name: "X".into(),
            duration_ms: "0".into(),
        };
        let err = lib.song_upgrade(&brief).await.unwrap_err();
        assert!(matches!(
            err,
            EchoError::ModelNotFound {
                reason: NotFoundReason::NotFound
            }
        ));
    }

    #[tokio::test]
    async fn test_provider_not_found() {
        let lib = setup_library();
        let brief = BriefSong {
            identifier: "1".into(),
            source: "nonexistent".into(),
            title: "X".into(),
            artists_name: "X".into(),
            album_name: "X".into(),
            duration_ms: "0".into(),
        };
        let err = lib.song_upgrade(&brief).await.unwrap_err();
        assert!(matches!(err, EchoError::ProviderNotFound(_)));
    }

    #[tokio::test]
    async fn test_search() {
        let lib = setup_library();
        let results = lib.search("Test", SearchType::Song, None).await;
        assert_eq!(results.len(), 1);
        let result = results.into_iter().next().unwrap().unwrap();
        assert!(!result.songs.is_empty());
    }

    #[tokio::test]
    async fn test_unsupported_capability() {
        let lib = setup_library();
        let album = BriefAlbum {
            identifier: "1".into(),
            source: "mock".into(),
            name: "X".into(),
            artists_name: "X".into(),
        };
        let err = lib.album_upgrade(&album).await.unwrap_err();
        assert!(matches!(
            err,
            EchoError::ModelNotFound {
                reason: NotFoundReason::NotSupported
            }
        ));
    }
}
