use crate::model::BriefSong;

/// Playback mode for the playlist.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayMode {
    /// Play songs in order.
    Sequential,
    /// Repeat current song.
    SingleLoop,
    /// Shuffle play.
    Random,
    /// Loop the entire playlist.
    LoopAll,
}

/// Manages a list of songs and the current playback position.
pub struct Playlist {
    songs: Vec<BriefSong>,
    current_index: Option<usize>,
    mode: PlayMode,
}

impl Playlist {
    pub fn new() -> Self {
        Self {
            songs: Vec::new(),
            current_index: None,
            mode: PlayMode::Sequential,
        }
    }

    pub fn with_songs(songs: Vec<BriefSong>) -> Self {
        let current = if songs.is_empty() { None } else { Some(0) };
        Self {
            songs,
            current_index: current,
            mode: PlayMode::Sequential,
        }
    }

    /// Current song.
    pub fn current(&self) -> Option<&BriefSong> {
        self.current_index.and_then(|i| self.songs.get(i))
    }

    /// All songs in the playlist.
    pub fn songs(&self) -> &[BriefSong] {
        &self.songs
    }

    /// Number of songs.
    pub fn len(&self) -> usize {
        self.songs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.songs.is_empty()
    }

    /// Current index.
    pub fn current_index(&self) -> Option<usize> {
        self.current_index
    }

    /// Set the current song by index.
    pub fn set_current(&mut self, index: usize) {
        if index < self.songs.len() {
            self.current_index = Some(index);
        }
    }

    /// Go to the next song. Returns the new current song, or None if at end.
    pub fn next(&mut self) -> Option<&BriefSong> {
        if self.songs.is_empty() {
            return None;
        }

        let next_idx = match self.current_index {
            None => 0,
            Some(i) => match self.mode {
                PlayMode::Sequential | PlayMode::LoopAll => {
                    if i + 1 >= self.songs.len() {
                        if self.mode == PlayMode::LoopAll {
                            0
                        } else {
                            return None;
                        }
                    } else {
                        i + 1
                    }
                }
                PlayMode::SingleLoop => i,
                PlayMode::Random => {
                    use rand::Rng;
                    let mut rng = rand::thread_rng();
                    if self.songs.len() == 1 {
                        0
                    } else {
                        rng.gen_range(0..self.songs.len())
                    }
                }
            },
        };

        self.current_index = Some(next_idx);
        self.songs.get(next_idx)
    }

    /// Go to the previous song.
    pub fn prev(&mut self) -> Option<&BriefSong> {
        if self.songs.is_empty() {
            return None;
        }

        let prev_idx = match self.current_index {
            None => 0,
            Some(i) => match self.mode {
                PlayMode::LoopAll => {
                    if i == 0 {
                        self.songs.len() - 1
                    } else {
                        i - 1
                    }
                }
                _ => {
                    if i == 0 {
                        0
                    } else {
                        i - 1
                    }
                }
            },
        };

        self.current_index = Some(prev_idx);
        self.songs.get(prev_idx)
    }

    pub fn mode(&self) -> PlayMode {
        self.mode
    }

    pub fn set_mode(&mut self, mode: PlayMode) {
        self.mode = mode;
    }

    pub fn cycle_mode(&mut self) {
        self.mode = match self.mode {
            PlayMode::Sequential => PlayMode::SingleLoop,
            PlayMode::SingleLoop => PlayMode::Random,
            PlayMode::Random => PlayMode::LoopAll,
            PlayMode::LoopAll => PlayMode::Sequential,
        };
    }
}

impl Default for Playlist {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_song(id: &str, title: &str) -> BriefSong {
        BriefSong {
            identifier: id.into(),
            source: "test".into(),
            title: title.into(),
            artists_name: "A".into(),
            album_name: "B".into(),
            duration_ms: "1000".into(),
        }
    }

    #[test]
    fn test_empty_playlist() {
        let mut pl = Playlist::new();
        assert!(pl.current().is_none());
        assert!(pl.next().is_none());
        assert!(pl.prev().is_none());
    }

    #[test]
    fn test_sequential_navigation() {
        let songs = vec![
            make_song("1", "A"),
            make_song("2", "B"),
            make_song("3", "C"),
        ];
        let mut pl = Playlist::with_songs(songs);
        assert_eq!(pl.current().unwrap().title, "A");

        assert_eq!(pl.next().unwrap().title, "B");
        assert_eq!(pl.next().unwrap().title, "C");
        assert!(pl.next().is_none()); // end of list

        assert_eq!(pl.prev().unwrap().title, "B");
        assert_eq!(pl.prev().unwrap().title, "A");
        assert_eq!(pl.prev().unwrap().title, "A"); // stays at 0
    }

    #[test]
    fn test_loop_all() {
        let songs = vec![make_song("1", "A"), make_song("2", "B")];
        let mut pl = Playlist::with_songs(songs);
        pl.set_mode(PlayMode::LoopAll);

        assert_eq!(pl.next().unwrap().title, "B");
        assert_eq!(pl.next().unwrap().title, "A"); // wraps around
    }

    #[test]
    fn test_single_loop() {
        let songs = vec![make_song("1", "A"), make_song("2", "B")];
        let mut pl = Playlist::with_songs(songs);
        pl.set_mode(PlayMode::SingleLoop);

        assert_eq!(pl.next().unwrap().title, "A"); // stays
        assert_eq!(pl.next().unwrap().title, "A");
    }

    #[test]
    fn test_cycle_mode() {
        let mut pl = Playlist::new();
        assert_eq!(pl.mode(), PlayMode::Sequential);
        pl.cycle_mode();
        assert_eq!(pl.mode(), PlayMode::SingleLoop);
        pl.cycle_mode();
        assert_eq!(pl.mode(), PlayMode::Random);
        pl.cycle_mode();
        assert_eq!(pl.mode(), PlayMode::LoopAll);
        pl.cycle_mode();
        assert_eq!(pl.mode(), PlayMode::Sequential);
    }

    #[test]
    fn test_set_current() {
        let songs = vec![
            make_song("1", "A"),
            make_song("2", "B"),
            make_song("3", "C"),
        ];
        let mut pl = Playlist::with_songs(songs);
        pl.set_current(2);
        assert_eq!(pl.current().unwrap().title, "C");
    }
}
