use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::Paragraph;

use crate::model::BriefSong;
use crate::tui::app::App;
use crate::tui::theme::ResolvedTheme;
use crate::tui::widgets::{controls, cover_art, lyrics, progress_bar};

/// Render the playback view (Xero-style layout).
pub fn render_playback_view(f: &mut Frame, area: Rect, app: &App, theme: &ResolvedTheme) {
    let song = app.playlist.current();
    let has_song = song.is_some();
    let position = app.position_ms as f64 / 1000.0;
    let duration = song
        .and_then(|s| s.duration_ms.parse::<f64>().ok())
        .map(|ms| ms / 1000.0)
        .unwrap_or(0.0);

    let has_lyrics = !app.current_lyrics.is_empty();
    let cover_height = theme.cover_max_height.min(12);
    let metadata_height = 3; // title + artist + album
    let progress_height = 1;
    let controls_height = 1;
    let min_lyrics_height = 5;

    let remaining = area
        .height
        .saturating_sub(cover_height + metadata_height + progress_height + controls_height + 2);

    let constraints = if has_lyrics && remaining >= min_lyrics_height {
        vec![
            Constraint::Length(cover_height),
            Constraint::Length(metadata_height),
            Constraint::Length(progress_height),
            Constraint::Length(controls_height),
            Constraint::Min(min_lyrics_height),
        ]
    } else {
        vec![
            Constraint::Length(cover_height),
            Constraint::Length(metadata_height),
            Constraint::Length(progress_height),
            Constraint::Length(controls_height),
        ]
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(area);

    // 1. Cover art
    cover_art::render_cover_art(f, chunks[0], theme, has_song);

    // 2. Song metadata
    render_metadata(f, chunks[1], theme, song);

    // 3. Progress bar
    progress_bar::render_progress_bar(f, chunks[2], theme, position, duration);

    // 4. Controls
    let play_mode_str = match app.playlist.mode() {
        crate::player::playlist::PlayMode::Sequential => "sequential",
        crate::player::playlist::PlayMode::SingleLoop => "single_loop",
        crate::player::playlist::PlayMode::Random => "random",
        crate::player::playlist::PlayMode::LoopAll => "loop_all",
    };
    controls::render_controls(
        f,
        chunks[3],
        theme,
        app.player_state(),
        play_mode_str,
        app.volume_pct(),
    );

    // 5. Lyrics
    if has_lyrics && chunks.len() > 4 {
        let position_ms = app.position_ms;
        lyrics::render_lyrics(f, chunks[4], theme, &app.current_lyrics, position_ms);
    }
}

fn render_metadata(f: &mut Frame, area: Rect, theme: &ResolvedTheme, song: Option<&BriefSong>) {
    let (title, artist, album) = match song {
        Some(s) => (
            s.title.as_str(),
            s.artists_name.as_str(),
            s.album_name.as_str(),
        ),
        None => ("No song playing", "", ""),
    };

    let title_line = ratatui::text::Line::from(ratatui::text::Span::styled(
        title,
        theme.text.add_modifier(ratatui::style::Modifier::BOLD),
    ))
    .alignment(ratatui::layout::Alignment::Center);

    let artist_line =
        ratatui::text::Line::from(ratatui::text::Span::styled(artist, theme.secondary))
            .alignment(ratatui::layout::Alignment::Center);

    let album_line = ratatui::text::Line::from(ratatui::text::Span::styled(album, theme.inactive))
        .alignment(ratatui::layout::Alignment::Center);

    let metadata = ratatui::text::Text::from(vec![title_line, artist_line, album_line]);
    let paragraph = Paragraph::new(metadata).alignment(ratatui::layout::Alignment::Center);
    f.render_widget(paragraph, area);
}
