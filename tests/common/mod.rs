use std::any::Any;
use std::collections::HashMap;

use async_trait::async_trait;

use EchoPlay::error::{EchoError, MediaNotFoundReason, NotFoundReason};
use EchoPlay::media::{AudioQuality, Media};
use EchoPlay::model::*;
use EchoPlay::provider::Provider;

/// Mock provider for offline integration testing.
pub struct MockProvider {
    pub id: String,
    pub songs: HashMap<String, Song>,
    pub albums: HashMap<String, Album>,
    pub artists: HashMap<String, Artist>,
    pub playlists: HashMap<String, Playlist>,
    pub lyrics: HashMap<String, Lyric>,
    pub media_urls: HashMap<String, Media>,
}

impl MockProvider {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            songs: HashMap::new(),
            albums: HashMap::new(),
            artists: HashMap::new(),
            playlists: HashMap::new(),
            lyrics: HashMap::new(),
            media_urls: HashMap::new(),
        }
    }

    pub fn add_song(&mut self, song: Song) {
        self.songs.insert(song.identifier.clone(), song);
    }

    pub fn add_album(&mut self, album: Album) {
        self.albums.insert(album.identifier.clone(), album);
    }

    pub fn add_artist(&mut self, artist: Artist) {
        self.artists.insert(artist.identifier.clone(), artist);
    }

    pub fn add_lyric(&mut self, lyric: Lyric) {
        self.lyrics.insert(lyric.identifier.clone(), lyric);
    }

    pub fn add_media(&mut self, song_id: &str, media: Media) {
        self.media_urls.insert(song_id.to_string(), media);
    }

    /// Build a sample provider with test data.
    pub fn sample() -> Self {
        let mut p = Self::new("mock");

        p.add_song(Song {
            identifier: "s1".into(),
            source: "mock".into(),
            title: "Test Song".into(),
            artists_name: "Artist A".into(),
            album_name: "Album X".into(),
            duration_ms: "240000".into(),
            album: Some(BriefAlbum {
                identifier: "a1".into(),
                source: "mock".into(),
                name: "Album X".into(),
                artists_name: "Artist A".into(),
            }),
            artists: vec![BriefArtist {
                identifier: "ar1".into(),
                source: "mock".into(),
                name: "Artist A".into(),
            }],
            duration: 240000,
            genre: "Pop".into(),
            date: "2024-01-01".into(),
            track: "1".into(),
            disc: "1".into(),
            pic_url: String::new(),
        });

        p.add_song(Song {
            identifier: "s2".into(),
            source: "mock".into(),
            title: "Another Song".into(),
            artists_name: "Artist B".into(),
            album_name: "Album Y".into(),
            duration_ms: "180000".into(),
            album: None,
            artists: vec![],
            duration: 180000,
            genre: "Rock".into(),
            date: "2024-06-01".into(),
            track: "3".into(),
            disc: "1".into(),
            pic_url: String::new(),
        });

        p.add_album(Album {
            identifier: "a1".into(),
            source: "mock".into(),
            name: "Album X".into(),
            artists_name: "Artist A".into(),
            cover: String::new(),
            album_type: "studio".into(),
            artists: vec![BriefArtist {
                identifier: "ar1".into(),
                source: "mock".into(),
                name: "Artist A".into(),
            }],
            songs: vec![BriefSong {
                identifier: "s1".into(),
                source: "mock".into(),
                title: "Test Song".into(),
                artists_name: "Artist A".into(),
                album_name: "Album X".into(),
                duration_ms: "240000".into(),
            }],
            song_count: 1,
            description: "A test album".into(),
            released: "2024-01-01".into(),
        });

        p.add_artist(Artist {
            identifier: "ar1".into(),
            source: "mock".into(),
            name: "Artist A".into(),
            pic_url: String::new(),
            aliases: vec![],
            hot_songs: vec![BriefSong {
                identifier: "s1".into(),
                source: "mock".into(),
                title: "Test Song".into(),
                artists_name: "Artist A".into(),
                album_name: "Album X".into(),
                duration_ms: "240000".into(),
            }],
            description: "A test artist".into(),
            song_count: 10,
            album_count: 3,
        });

        p.add_lyric(Lyric {
            identifier: "s1".into(),
            source: "mock".into(),
            content: "[00:00.00]Test lyric line 1\n[00:05.00]Test lyric line 2\n".into(),
            trans_content: String::new(),
        });

        p.add_media("s1", Media::audio("https://example.com/s1.mp3"));
        p.add_media("s2", Media::audio("https://example.com/s2.flac"));

        p
    }
}

#[async_trait]
impl Provider for MockProvider {
    fn identifier(&self) -> &str {
        &self.id
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

    async fn song_list_quality(&self, song: &BriefSong) -> Result<Vec<AudioQuality>, EchoError> {
        if self.media_urls.contains_key(&song.identifier) {
            Ok(vec![AudioQuality::Hq, AudioQuality::Sq, AudioQuality::Lq])
        } else {
            Err(EchoError::MediaNotFound {
                reason: MediaNotFoundReason::NotFound,
            })
        }
    }

    async fn song_get_media(
        &self,
        song: &BriefSong,
        quality: AudioQuality,
    ) -> Result<Option<Media>, EchoError> {
        if let Some(media) = self.media_urls.get(&song.identifier) {
            let _ = quality;
            Ok(Some(media.clone()))
        } else {
            Ok(None)
        }
    }

    async fn song_get_lyric(&self, song: &BriefSong) -> Result<Option<Lyric>, EchoError> {
        Ok(self.lyrics.get(&song.identifier).cloned())
    }

    async fn album_get(&self, identifier: &str) -> Result<Album, EchoError> {
        self.albums
            .get(identifier)
            .cloned()
            .ok_or(EchoError::ModelNotFound {
                reason: NotFoundReason::NotFound,
            })
    }

    async fn album_list_songs(&self, album: &BriefAlbum) -> Result<Vec<Song>, EchoError> {
        if let Some(a) = self.albums.get(&album.identifier) {
            let mut songs = Vec::new();
            for brief in &a.songs {
                if let Some(song) = self.songs.get(&brief.identifier) {
                    songs.push(song.clone());
                }
            }
            Ok(songs)
        } else {
            Err(EchoError::ModelNotFound {
                reason: NotFoundReason::NotFound,
            })
        }
    }

    async fn artist_get(&self, identifier: &str) -> Result<Artist, EchoError> {
        self.artists
            .get(identifier)
            .cloned()
            .ok_or(EchoError::ModelNotFound {
                reason: NotFoundReason::NotFound,
            })
    }

    async fn playlist_get(&self, identifier: &str) -> Result<Playlist, EchoError> {
        self.playlists
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
        let mut result = SearchResult::empty(&self.id, keyword, SearchType::Song);
        let kw = keyword.to_lowercase();

        for song in self.songs.values() {
            if song.title.to_lowercase().contains(&kw)
                || song.artists_name.to_lowercase().contains(&kw)
            {
                result.songs.push(song.brief());
            }
        }
        for album in self.albums.values() {
            if album.name.to_lowercase().contains(&kw) {
                result.albums.push(BriefAlbum {
                    identifier: album.identifier.clone(),
                    source: album.source.clone(),
                    name: album.name.clone(),
                    artists_name: album.artists_name.clone(),
                });
            }
        }
        for artist in self.artists.values() {
            if artist.name.to_lowercase().contains(&kw) {
                result.artists.push(BriefArtist {
                    identifier: artist.identifier.clone(),
                    source: artist.source.clone(),
                    name: artist.name.clone(),
                });
            }
        }

        Ok(result)
    }
}
