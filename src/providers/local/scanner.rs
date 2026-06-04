use std::path::{Path, PathBuf};

/// Supported audio file extensions.
const AUDIO_EXTENSIONS: &[&str] = &["mp3", "flac", "wav", "ogg", "m4a", "aac", "wma"];

/// Recursively scan a directory for audio files.
pub struct DirScanner;

impl DirScanner {
    /// Scan a directory recursively and return all audio file paths.
    pub fn scan(dir: &Path) -> Vec<PathBuf> {
        let mut files = Vec::new();
        if dir.is_dir() {
            Self::scan_recursive(dir, &mut files);
        }
        files
    }

    fn scan_recursive(dir: &Path, files: &mut Vec<PathBuf>) {
        let entries = match std::fs::read_dir(dir) {
            Ok(e) => e,
            Err(_) => return,
        };

        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                Self::scan_recursive(&path, files);
            } else if Self::is_audio(&path) {
                files.push(path);
            }
        }
    }

    fn is_audio(path: &Path) -> bool {
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| AUDIO_EXTENSIONS.contains(&ext.to_lowercase().as_str()))
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_is_audio() {
        assert!(DirScanner::is_audio(Path::new("song.mp3")));
        assert!(DirScanner::is_audio(Path::new("song.flac")));
        assert!(DirScanner::is_audio(Path::new("song.wav")));
        assert!(DirScanner::is_audio(Path::new("song.ogg")));
        assert!(DirScanner::is_audio(Path::new("song.MP3")));
        assert!(!DirScanner::is_audio(Path::new("readme.txt")));
        assert!(!DirScanner::is_audio(Path::new("noext")));
    }

    #[test]
    fn test_scan_empty_dir() {
        let dir = tempfile::tempdir().unwrap();
        let files = DirScanner::scan(dir.path());
        assert!(files.is_empty());
    }

    #[test]
    fn test_scan_with_files() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("song.mp3"), b"fake").unwrap();
        fs::write(dir.path().join("track.flac"), b"fake").unwrap();
        fs::write(dir.path().join("readme.txt"), b"not audio").unwrap();
        fs::create_dir(dir.path().join("sub")).unwrap();
        fs::write(dir.path().join("sub/nested.ogg"), b"fake").unwrap();

        let files = DirScanner::scan(dir.path());
        assert_eq!(files.len(), 3);
    }
}
