use std::sync::Arc;

use crate::library::Library;
use crate::model::*;
use crate::player::Player;
use crate::player::playlist::Playlist;
use crate::player::state::PlayerState;
use crate::tui::theme::{ResolvedTheme, Theme};
use crate::tui::widgets::lyrics::LyricLine;

/// Current UI view.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    PlayView,
    Library,
    Search,
}

/// Application state, bridging TUI and backend.
pub struct App {
    pub library: Arc<Library>,
    pub player: Arc<Player>,
    pub playlist: Playlist,
    pub theme: ResolvedTheme,

    /// Current view mode.
    pub view_mode: ViewMode,
    /// Whether the tab bar is visible (auto-hides after selection).
    pub tab_bar_visible: bool,
    /// Whether the app should quit.
    pub should_quit: bool,

    /// Search query text.
    pub search_query: String,
    /// Current search results.
    pub search_results: Vec<BriefSong>,
    /// Selected index in the current list view.
    pub selected_index: usize,

    /// Current song lyrics (parsed).
    pub current_lyrics: Vec<LyricLine>,
    /// Current playback position in milliseconds.
    pub position_ms: u64,
}

impl App {
    pub fn new(library: Arc<Library>, player: Arc<Player>, theme: Theme) -> Self {
        Self {
            library,
            player,
            playlist: Playlist::new(),
            theme: ResolvedTheme::resolve(&theme),
            view_mode: ViewMode::PlayView,
            tab_bar_visible: false,
            should_quit: false,
            search_query: String::new(),
            search_results: Vec::new(),
            selected_index: 0,
            current_lyrics: Vec::new(),
            position_ms: 0,
        }
    }

    /// Set the playlist songs.
    pub fn set_playlist(&mut self, songs: Vec<BriefSong>) {
        self.playlist = Playlist::with_songs(songs);
        self.selected_index = 0;
    }

    /// Set lyrics for the current song.
    pub fn set_lyrics(&mut self, lrc_content: &str) {
        self.current_lyrics = crate::tui::widgets::lyrics::parse_lrc(lrc_content);
    }

    /// Clear lyrics.
    pub fn clear_lyrics(&mut self) {
        self.current_lyrics.clear();
    }

    /// Current player state.
    pub fn player_state(&self) -> PlayerState {
        self.player.state()
    }

    /// Volume as percentage (0-100).
    pub fn volume_pct(&self) -> u8 {
        (self.player.volume() * 100.0).round() as u8
    }

    /// Toggle tab bar visibility.
    pub fn toggle_tab_bar(&mut self) {
        self.tab_bar_visible = !self.tab_bar_visible;
    }

    /// Switch to a view and hide the tab bar.
    pub fn switch_view(&mut self, mode: ViewMode) {
        self.view_mode = mode;
        self.tab_bar_visible = false;
        self.selected_index = 0;
    }

    /// Cycle to next tab.
    pub fn next_tab(&mut self) {
        self.view_mode = match self.view_mode {
            ViewMode::PlayView => ViewMode::Library,
            ViewMode::Library => ViewMode::Search,
            ViewMode::Search => ViewMode::PlayView,
        };
        self.selected_index = 0;
    }

    /// Cycle to previous tab.
    pub fn prev_tab(&mut self) {
        self.view_mode = match self.view_mode {
            ViewMode::PlayView => ViewMode::Search,
            ViewMode::Library => ViewMode::PlayView,
            ViewMode::Search => ViewMode::Library,
        };
        self.selected_index = 0;
    }

    /// Toggle play/pause.
    pub fn toggle_play(&mut self) {
        match self.player.state() {
            PlayerState::Playing => self.player.pause(),
            PlayerState::Paused => self.player.resume(),
            PlayerState::Stopped => {
                if let Some(song) = self.playlist.current().cloned() {
                    let media = crate::media::Media::audio(&song.identifier);
                    let _ = self.player.play(&media);
                    self.position_ms = 0;
                }
            }
        }
    }

    /// Play the next song.
    pub fn next_song(&mut self) {
        if let Some(song) = self.playlist.next().cloned() {
            let media = crate::media::Media::audio(&song.identifier);
            let _ = self.player.play(&media);
            self.position_ms = 0;
        }
    }

    /// Play the previous song.
    pub fn prev_song(&mut self) {
        if let Some(song) = self.playlist.prev().cloned() {
            let media = crate::media::Media::audio(&song.identifier);
            let _ = self.player.play(&media);
            self.position_ms = 0;
        }
    }

    /// Adjust volume by delta.
    pub fn adjust_volume(&self, delta: f32) {
        let new_vol = (self.player.volume() + delta).clamp(0.0, 1.0);
        self.player.set_volume(new_vol);
    }

    /// Play the song at the given index in the current view.
    pub fn play_at_index(&mut self, index: usize) {
        let songs = match self.view_mode {
            ViewMode::Search => &self.search_results,
            ViewMode::Library | ViewMode::PlayView => self.playlist.songs(),
        };

        if let Some(song) = songs.get(index) {
            let media = crate::media::Media::audio(&song.identifier);
            let _ = self.player.play(&media);
            self.position_ms = 0;

            if self.view_mode == ViewMode::PlayView || self.view_mode == ViewMode::Library {
                self.playlist.set_current(index);
            }
        }
    }

    /// Move selection up.
    pub fn select_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    /// Move selection down.
    pub fn select_down(&mut self) {
        let max = match self.view_mode {
            ViewMode::Search => self.search_results.len(),
            ViewMode::Library | ViewMode::PlayView => self.playlist.len(),
        };
        if max > 0 && self.selected_index + 1 < max {
            self.selected_index += 1;
        }
    }

    /// Cycle play mode.
    pub fn cycle_play_mode(&mut self) {
        self.playlist.cycle_mode();
    }

    /// Perform a search.
    pub async fn do_search(&mut self) {
        if self.search_query.is_empty() {
            return;
        }

        let results = self
            .library
            .search(&self.search_query, SearchType::Song, None)
            .await;

        self.search_results.clear();
        for r in results.into_iter().flatten() {
            self.search_results.extend(r.songs);
        }

        self.selected_index = 0;
    }

    /// Update playback position (called on tick).
    pub fn update_position(&mut self) {
        if self.player.state() == PlayerState::Playing {
            self.position_ms += 100;

            // Clamp to duration to prevent overshooting
            if let Some(song) = self.playlist.current()
                && let Ok(duration_ms) = song.duration_ms.parse::<u64>()
                && duration_ms > 0
                && self.position_ms > duration_ms
            {
                self.position_ms = duration_ms;
            }
        }
    }
}
