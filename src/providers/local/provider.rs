use std::any::Any;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use async_trait::async_trait;

use crate::error::EchoError;
use crate::media::Media;
use crate::model::*;
use crate::provider::Provider;

use super::scanner::DirScanner;
use super::tag_reader::TagReader;

/// Local file system music provider.
///
/// Scans directories for audio files and reads their metadata.
pub struct LocalProvider {
    /// Cached songs indexed by file path.
    songs: Mutex<HashMap<String, BriefSong>>,
    /// Scanned file paths.
    paths: Mutex<Vec<PathBuf>>,
}

impl LocalProvider {
    pub fn new() -> Self {
        Self {
            songs: Mutex::new(HashMap::new()),
            paths: Mutex::new(Vec::new()),
        }
    }

    /// Scan directories and index all audio files.
    pub fn scan(&self, dirs: &[&Path]) {
        let mut all_files = Vec::new();
        for dir in dirs {
            all_files.extend(DirScanner::scan(dir));
        }

        let mut songs = self.songs.lock().unwrap();
        songs.clear();

        for path in &all_files {
            if let Some(brief) = TagReader::read_brief(path) {
                songs.insert(brief.identifier.clone(), brief);
            }
        }

        let mut paths = self.paths.lock().unwrap();
        *paths = all_files;
    }

    /// Number of indexed songs.
    pub fn song_count(&self) -> usize {
        self.songs.lock().unwrap().len()
    }
}

#[async_trait]
impl Provider for LocalProvider {
    fn identifier(&self) -> &str {
        "local"
    }

    fn name(&self) -> &str {
        "Local Files"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    async fn search(
        &self,
        keyword: &str,
        _search_type: SearchType,
    ) -> Result<SearchResult, EchoError> {
        let songs = self.songs.lock().unwrap();
        let mut result = SearchResult::empty("local", keyword, SearchType::Song);
        let kw = keyword.to_lowercase();

        for brief in songs.values() {
            if brief.title.to_lowercase().contains(&kw)
                || brief.artists_name.to_lowercase().contains(&kw)
                || brief.album_name.to_lowercase().contains(&kw)
            {
                result.songs.push(brief.clone());
            }
        }

        Ok(result)
    }

    async fn song_get(&self, identifier: &str) -> Result<Song, EchoError> {
        let path = Path::new(identifier);
        if !path.exists() {
            return Err(EchoError::ModelNotFound {
                reason: crate::error::NotFoundReason::NotFound,
            });
        }

        TagReader::read_full(path).ok_or(EchoError::ModelNotFound {
            reason: crate::error::NotFoundReason::NotFound,
        })
    }

    async fn song_get_media(
        &self,
        song: &BriefSong,
        _quality: crate::media::AudioQuality,
    ) -> Result<Option<Media>, EchoError> {
        let path = Path::new(&song.identifier);
        if path.exists() {
            Ok(Some(Media::audio(&song.identifier)))
        } else {
            Ok(None)
        }
    }
}

impl Default for LocalProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_empty() {
        let provider = LocalProvider::new();
        let dir = tempfile::tempdir().unwrap();
        provider.scan(&[dir.path()]);
        assert_eq!(provider.song_count(), 0);
    }

    #[test]
    fn test_scan_nonexistent_dir() {
        let provider = LocalProvider::new();
        provider.scan(&[Path::new("/nonexistent/path")]);
        assert_eq!(provider.song_count(), 0);
    }

    #[tokio::test]
    async fn test_search_empty() {
        let provider = LocalProvider::new();
        let result = provider.search("test", SearchType::Song).await.unwrap();
        assert!(result.songs.is_empty());
    }

    #[tokio::test]
    async fn test_song_get_nonexistent() {
        let provider = LocalProvider::new();
        let result = provider.song_get("/nonexistent/file.mp3").await;
        assert!(result.is_err());
    }
}
