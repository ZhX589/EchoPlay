use std::sync::Arc;

use EchoPlay::error::{EchoError, NotFoundReason};
use EchoPlay::library::Library;
use EchoPlay::model::*;
use EchoPlay::provider::registry::ProviderRegistry;

mod common;
use common::MockProvider;

fn setup_library() -> Library {
    let mut reg = ProviderRegistry::new();
    reg.register(Arc::new(MockProvider::sample())).unwrap();
    Library::new(reg)
}

// ---------------------------------------------------------------------------
// Song operations
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_song_upgrade() {
    let lib = setup_library();
    let brief = BriefSong {
        identifier: "s1".into(),
        source: "mock".into(),
        title: "Test".into(),
        artists_name: "A".into(),
        album_name: "X".into(),
        duration_ms: "240000".into(),
    };
    let song = lib.song_upgrade(&brief).await.unwrap();
    assert_eq!(song.title, "Test Song");
    assert_eq!(song.artists_name, "Artist A");
    assert_eq!(song.duration, 240000);
}

#[tokio::test]
async fn test_song_upgrade_not_found() {
    let lib = setup_library();
    let brief = BriefSong {
        identifier: "nonexistent".into(),
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
async fn test_song_prepare_media() {
    let lib = setup_library();
    let brief = BriefSong {
        identifier: "s1".into(),
        source: "mock".into(),
        title: "Test".into(),
        artists_name: "A".into(),
        album_name: "X".into(),
        duration_ms: "240000".into(),
    };
    let media = lib.song_prepare_media(&brief, None).await.unwrap();
    assert!(media.url.contains("example.com"));
}

#[tokio::test]
async fn test_song_get_lyric() {
    let lib = setup_library();
    let brief = BriefSong {
        identifier: "s1".into(),
        source: "mock".into(),
        title: "Test".into(),
        artists_name: "A".into(),
        album_name: "X".into(),
        duration_ms: "240000".into(),
    };
    let lyric = lib.song_get_lyric(&brief).await.unwrap();
    assert!(lyric.is_some());
    assert!(lyric.unwrap().content.contains("lyric line"));
}

#[tokio::test]
async fn test_song_get_lyric_missing() {
    let lib = setup_library();
    let brief = BriefSong {
        identifier: "s2".into(),
        source: "mock".into(),
        title: "X".into(),
        artists_name: "X".into(),
        album_name: "X".into(),
        duration_ms: "0".into(),
    };
    let lyric = lib.song_get_lyric(&brief).await.unwrap();
    assert!(lyric.is_none());
}

// ---------------------------------------------------------------------------
// Album operations
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_album_upgrade() {
    let lib = setup_library();
    let brief = BriefAlbum {
        identifier: "a1".into(),
        source: "mock".into(),
        name: "Album X".into(),
        artists_name: "Artist A".into(),
    };
    let album = lib.album_upgrade(&brief).await.unwrap();
    assert_eq!(album.name, "Album X");
    assert_eq!(album.song_count, 1);
}

#[tokio::test]
async fn test_album_list_songs() {
    let lib = setup_library();
    let brief = BriefAlbum {
        identifier: "a1".into(),
        source: "mock".into(),
        name: "Album X".into(),
        artists_name: "Artist A".into(),
    };
    let songs = lib.album_list_songs(&brief).await.unwrap();
    assert_eq!(songs.len(), 1);
    assert_eq!(songs[0].title, "Test Song");
}

// ---------------------------------------------------------------------------
// Artist operations
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_artist_upgrade() {
    let lib = setup_library();
    let brief = BriefArtist {
        identifier: "ar1".into(),
        source: "mock".into(),
        name: "Artist A".into(),
    };
    let artist = lib.artist_upgrade(&brief).await.unwrap();
    assert_eq!(artist.name, "Artist A");
    assert_eq!(artist.song_count, 10);
}

// ---------------------------------------------------------------------------
// Search
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_search_songs() {
    let lib = setup_library();
    let results = lib.search("Test", SearchType::Song, None).await;
    assert_eq!(results.len(), 1);
    let result = results.into_iter().next().unwrap().unwrap();
    assert!(!result.songs.is_empty());
    assert_eq!(result.songs[0].title, "Test Song");
}

#[tokio::test]
async fn test_search_artists() {
    let lib = setup_library();
    let results = lib.search("Artist A", SearchType::Artist, None).await;
    assert_eq!(results.len(), 1);
    let result = results.into_iter().next().unwrap().unwrap();
    assert!(!result.artists.is_empty());
}

#[tokio::test]
async fn test_search_albums() {
    let lib = setup_library();
    let results = lib.search("Album", SearchType::Album, None).await;
    assert_eq!(results.len(), 1);
    let result = results.into_iter().next().unwrap().unwrap();
    assert!(!result.albums.is_empty());
}

#[tokio::test]
async fn test_search_no_results() {
    let lib = setup_library();
    let results = lib.search("zzzznonexistent", SearchType::Song, None).await;
    assert_eq!(results.len(), 1);
    let result = results.into_iter().next().unwrap().unwrap();
    assert!(result.is_empty());
}

#[tokio::test]
async fn test_search_specific_source() {
    let lib = setup_library();
    let results = lib.search("Test", SearchType::Song, Some(&["mock"])).await;
    assert_eq!(results.len(), 1);

    // Search a non-existent source
    let results = lib.search("Test", SearchType::Song, Some(&["other"])).await;
    assert_eq!(results.len(), 0);
}

// ---------------------------------------------------------------------------
// Standby (cross-provider fallback)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_find_standby_same_source_excluded() {
    let lib = setup_library();
    let original = BriefSong {
        identifier: "s1".into(),
        source: "mock".into(),
        title: "Test Song".into(),
        artists_name: "Artist A".into(),
        album_name: "Album X".into(),
        duration_ms: "240000".into(),
    };
    // Only one provider (mock), so standby search returns nothing
    // because the original source is excluded.
    let candidates = lib.find_standby(&original, None).await;
    assert!(candidates.is_empty());
}

// ---------------------------------------------------------------------------
// Provider not found
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_provider_not_found() {
    let lib = setup_library();
    let brief = BriefSong {
        identifier: "s1".into(),
        source: "nonexistent".into(),
        title: "X".into(),
        artists_name: "X".into(),
        album_name: "X".into(),
        duration_ms: "0".into(),
    };
    let err = lib.song_upgrade(&brief).await.unwrap_err();
    assert!(matches!(err, EchoError::ProviderNotFound(_)));
}

// ---------------------------------------------------------------------------
// Unsupported capability returns NotSupported
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_unsupported_playlist() {
    let lib = setup_library();
    let brief = BriefPlaylist {
        identifier: "p1".into(),
        source: "mock".into(),
        name: "X".into(),
        creator_name: "X".into(),
    };
    let err = lib.playlist_upgrade(&brief).await.unwrap_err();
    // MockProvider implements playlist_get but returns NotFound (empty map)
    assert!(matches!(
        err,
        EchoError::ModelNotFound {
            reason: NotFoundReason::NotFound
        }
    ));
}

// ---------------------------------------------------------------------------
// Multi-provider scenario
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_multi_provider_search() {
    let mut reg = ProviderRegistry::new();

    let mut p1 = MockProvider::new("source_a");
    p1.add_song(Song {
        identifier: "a1".into(),
        source: "source_a".into(),
        title: "Shared Song".into(),
        artists_name: "Singer".into(),
        album_name: "Album".into(),
        duration_ms: "200000".into(),
        album: None,
        artists: vec![],
        duration: 200000,
        genre: String::new(),
        date: String::new(),
        track: String::new(),
        disc: String::new(),
        pic_url: String::new(),
    });

    let mut p2 = MockProvider::new("source_b");
    p2.add_song(Song {
        identifier: "b1".into(),
        source: "source_b".into(),
        title: "Shared Song".into(),
        artists_name: "Singer".into(),
        album_name: "Album".into(),
        duration_ms: "200000".into(),
        album: None,
        artists: vec![],
        duration: 200000,
        genre: String::new(),
        date: String::new(),
        track: String::new(),
        disc: String::new(),
        pic_url: String::new(),
    });

    reg.register(Arc::new(p1)).unwrap();
    reg.register(Arc::new(p2)).unwrap();

    let lib = Library::new(reg);
    let results = lib.search("Shared", SearchType::Song, None).await;
    assert_eq!(results.len(), 2);
    for r in &results {
        assert_eq!(r.as_ref().unwrap().songs.len(), 1);
    }
}

// ---------------------------------------------------------------------------
// URI round-trip integration
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_uri_to_upgrade() {
    let lib = setup_library();
    let uri = Uri::parse("echoplay://mock/songs/s1").unwrap();
    assert_eq!(uri.source, "mock");
    assert_eq!(uri.model_type, ModelType::Song);

    let brief = BriefSong {
        identifier: uri.identifier.clone(),
        source: uri.source.clone(),
        title: String::new(),
        artists_name: String::new(),
        album_name: String::new(),
        duration_ms: String::new(),
    };
    let song = lib.song_upgrade(&brief).await.unwrap();
    assert_eq!(song.identifier, "s1");
    assert_eq!(song.title, "Test Song");
}
