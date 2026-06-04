use std::path::Path;

use lofty::prelude::*;
use lofty::probe::Probe;

use crate::model::{BriefAlbum, BriefArtist, BriefSong, Song};

/// Read audio file metadata using lofty (supports mp3/flac/ogg/wav/m4a).
pub struct TagReader;

impl TagReader {
    /// Read a BriefSong from an audio file's tags.
    pub fn read_brief(path: &Path) -> Option<BriefSong> {
        let tagged_file = Probe::open(path).ok()?.read().ok()?;

        let tag = tagged_file.primary_tag()?;
        let properties = tagged_file.properties();

        let title = tag
            .get_string(&ItemKey::TrackTitle)
            .unwrap_or("Unknown Title")
            .to_string();

        let artist = tag
            .get_string(&ItemKey::TrackArtist)
            .unwrap_or("Unknown Artist")
            .to_string();

        let album = tag
            .get_string(&ItemKey::AlbumTitle)
            .unwrap_or("Unknown Album")
            .to_string();

        let duration_ms = properties.duration().as_millis().to_string();

        let identifier = path.to_string_lossy().to_string();

        Some(BriefSong {
            identifier,
            source: "local".into(),
            title,
            artists_name: artist,
            album_name: album,
            duration_ms,
        })
    }

    /// Read a full Song from an audio file's tags.
    pub fn read_full(path: &Path) -> Option<Song> {
        let tagged_file = Probe::open(path).ok()?.read().ok()?;

        let tag = tagged_file.primary_tag()?;
        let properties = tagged_file.properties();

        let title = tag
            .get_string(&ItemKey::TrackTitle)
            .unwrap_or("Unknown Title")
            .to_string();

        let artist_name = tag
            .get_string(&ItemKey::TrackArtist)
            .unwrap_or("Unknown Artist")
            .to_string();

        let album_name = tag
            .get_string(&ItemKey::AlbumTitle)
            .unwrap_or("Unknown Album")
            .to_string();

        let genre = tag.get_string(&ItemKey::Genre).unwrap_or("").to_string();

        let date = tag.get_string(&ItemKey::Year).unwrap_or("").to_string();

        let track = tag
            .get_string(&ItemKey::TrackNumber)
            .unwrap_or("")
            .to_string();

        let duration = properties.duration().as_millis() as i64;
        let duration_ms = duration.to_string();

        let identifier = path.to_string_lossy().to_string();

        let artist_id = format!("local:artist:{}", artist_name);
        let album_id = format!("local:album:{}", album_name);

        Some(Song {
            identifier,
            source: "local".into(),
            title,
            artists_name: artist_name.clone(),
            album_name: album_name.clone(),
            duration_ms,
            album: Some(BriefAlbum {
                identifier: album_id,
                source: "local".into(),
                name: album_name,
                artists_name: artist_name.clone(),
            }),
            artists: vec![BriefArtist {
                identifier: artist_id,
                source: "local".into(),
                name: artist_name,
            }],
            duration,
            genre,
            date,
            track,
            disc: String::new(),
            pic_url: String::new(),
        })
    }

    /// Get duration in milliseconds from an audio file.
    pub fn read_duration(path: &Path) -> Option<i64> {
        let tagged_file = Probe::open(path).ok()?.read().ok()?;
        Some(tagged_file.properties().duration().as_millis() as i64)
    }
}
