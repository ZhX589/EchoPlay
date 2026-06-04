use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};

use crate::tui::app::{App, ViewMode};
use crate::tui::tab_bar;
use crate::tui::views;

/// Draw the entire UI.
pub fn draw(f: &mut Frame, app: &App) {
    let area = f.area();

    // Layout: optional tab bar + main content
    let chunks = if app.tab_bar_visible {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(area)
    } else {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0)])
            .split(area)
    };

    // Tab bar (if visible)
    if app.tab_bar_visible {
        tab_bar::render_tab_bar(f, chunks[0], &app.theme, app.view_mode, true);
    }

    // Main content area
    let content_area = if app.tab_bar_visible {
        chunks[1]
    } else {
        chunks[0]
    };

    // Apply background color if set
    if let Some(bg) = app.theme.bg {
        let bg_block =
            ratatui::widgets::Block::default().style(ratatui::style::Style::default().bg(bg));
        f.render_widget(bg_block, content_area);
    }

    // Route to the appropriate view
    match app.view_mode {
        ViewMode::PlayView => {
            views::playback::render_playback_view(f, content_area, app, &app.theme);
        }
        ViewMode::Library => {
            let songs = app.playlist.songs();
            let playing = app.playlist.current_index();
            views::library::render_library_view(
                f,
                content_area,
                &app.theme,
                songs,
                app.selected_index,
                &format!(" Library ({}) ", songs.len()),
                playing,
            );
        }
        ViewMode::Search => {
            views::library::render_search_view(
                f,
                content_area,
                &app.theme,
                &app.search_results,
                app.selected_index,
                &app.search_query,
            );
        }
    }
}
