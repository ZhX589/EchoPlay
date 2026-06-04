use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};

use crate::model::BriefSong;
use crate::tui::theme::ResolvedTheme;

const ICON_MUSIC: &str = "\u{F001}"; // nf-fa-music

/// Render the library/search results list view with proper selection highlight.
pub fn render_library_view(
    f: &mut Frame,
    area: Rect,
    theme: &ResolvedTheme,
    songs: &[BriefSong],
    selected: usize,
    title: &str,
    playing_index: Option<usize>,
) {
    let items: Vec<ListItem> = songs
        .iter()
        .enumerate()
        .map(|(i, song)| {
            let indicator = if Some(i) == playing_index {
                format!("{} ", ICON_MUSIC)
            } else {
                "  ".to_string()
            };

            let content = format!(
                "{}{:3}. {} - {} [{}]",
                indicator,
                i + 1,
                song.title,
                song.artists_name,
                song.album_name
            );
            ListItem::new(content)
        })
        .collect();

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(theme.secondary);

    let list = List::new(items).block(block).highlight_style(
        Style::default()
            .fg(ratatui::style::Color::Black)
            .bg(ratatui::style::Color::Cyan)
            .add_modifier(Modifier::BOLD),
    );

    let mut state = ListState::default();
    if !songs.is_empty() {
        state.select(Some(selected.min(songs.len() - 1)));
    }

    f.render_stateful_widget(list, area, &mut state);
}

/// Render the search view with query input and results.
pub fn render_search_view(
    f: &mut Frame,
    area: Rect,
    theme: &ResolvedTheme,
    songs: &[BriefSong],
    selected: usize,
    query: &str,
) {
    // Split into search input + results
    let chunks = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            ratatui::layout::Constraint::Length(3),
            ratatui::layout::Constraint::Min(0),
        ])
        .split(area);

    // Search input
    let cursor = if query.is_empty() { "" } else { "_" };
    let input_text = format!("{}{}", query, cursor);
    let input_title = if query.is_empty() {
        " Search (press / to focus) "
    } else {
        " Search "
    };

    let input_block = Block::default()
        .title(input_title)
        .borders(Borders::ALL)
        .border_style(if query.is_empty() {
            theme.secondary
        } else {
            theme.accent
        });

    let input_paragraph = Paragraph::new(input_text)
        .block(input_block)
        .style(theme.text);
    f.render_widget(input_paragraph, chunks[0]);

    // Results
    let results_title = if query.is_empty() {
        " Results ".to_string()
    } else {
        format!(" Results ({}) ", songs.len())
    };
    render_library_view(f, chunks[1], theme, songs, selected, &results_title, None);
}
