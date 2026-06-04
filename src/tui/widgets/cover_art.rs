use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::tui::theme::ResolvedTheme;

const ICON_MUSIC: &str = "\u{F001}"; // nf-fa-music

/// Render cover art using Unicode half-block characters.
pub fn render_cover_art(f: &mut Frame, area: Rect, theme: &ResolvedTheme, has_song: bool) {
    if !theme.cover_enabled {
        return;
    }

    let width = area.width.min(theme.cover_max_width);
    let height = area.height.min(theme.cover_max_height);

    let art_area = centered_rect(width, height, area);

    let title = if has_song {
        format!(" {} ", ICON_MUSIC)
    } else {
        " EchoPlay ".to_string()
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme.secondary)
        .title(title);

    let inner = block.inner(art_area);
    f.render_widget(block, art_area);

    if !has_song {
        // Show centered "No song" message
        let msg = Paragraph::new("No song playing")
            .style(theme.inactive)
            .alignment(ratatui::layout::Alignment::Center);
        f.render_widget(msg, inner);
        return;
    }

    // Fill with a gradient-like pattern using half-blocks
    let lines: Vec<Line> = (0..inner.height)
        .map(|row| {
            let spans: Vec<Span> = (0..inner.width)
                .map(|col| {
                    let is_pattern = (row + col) % 3 == 0;
                    if is_pattern {
                        Span::styled(
                            "▀",
                            Style::default()
                                .fg(theme.accent.fg.unwrap_or(Color::Cyan))
                                .bg(Color::DarkGray),
                        )
                    } else {
                        Span::styled("▀", Style::default().fg(Color::DarkGray).bg(Color::Black))
                    }
                })
                .collect();
            Line::from(spans)
        })
        .collect();

    let paragraph = Paragraph::new(lines);
    f.render_widget(paragraph, inner);
}

pub fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;
    Rect {
        x,
        y,
        width: width.min(area.width),
        height: height.min(area.height),
    }
}
