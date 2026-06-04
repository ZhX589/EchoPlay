use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::tui::theme::ResolvedTheme;

/// Parsed lyric line with timestamp.
#[derive(Debug, Clone)]
pub struct LyricLine {
    /// Timestamp in milliseconds.
    pub timestamp_ms: u64,
    /// Lyric text.
    pub text: String,
}

/// Parse LRC format lyrics into a list of LyricLine.
pub fn parse_lrc(content: &str) -> Vec<LyricLine> {
    let mut lines = Vec::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Parse [mm:ss.xx] or [mm:ss] format
        if let Some(end) = line.find(']') {
            let tag = &line[1..end];
            let text = line[end + 1..].trim().to_string();

            if text.is_empty() {
                continue; // Skip metadata tags like [ti:], [ar:]
            }

            if let Some(ms) = parse_timestamp(tag) {
                lines.push(LyricLine {
                    timestamp_ms: ms,
                    text,
                });
            }
        }
    }

    lines.sort_by_key(|l| l.timestamp_ms);
    lines
}

/// Parse a timestamp like "01:23.45" or "01:23" into milliseconds.
fn parse_timestamp(s: &str) -> Option<u64> {
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() != 2 {
        return None;
    }

    let minutes: u64 = parts[0].parse().ok()?;
    let sec_parts: Vec<&str> = parts[1].split('.').collect();
    let seconds: u64 = sec_parts[0].parse().ok()?;
    let millis: u64 = if sec_parts.len() > 1 {
        let ms_str = sec_parts[1];
        // Handle both .xx (centiseconds) and .xxx (milliseconds)
        match ms_str.len() {
            1 => ms_str.parse::<u64>().ok()? * 100,
            2 => ms_str.parse::<u64>().ok()? * 10,
            3 => ms_str.parse::<u64>().ok()?,
            _ => 0,
        }
    } else {
        0
    };

    Some(minutes * 60_000 + seconds * 1000 + millis)
}

/// Find the index of the current lyric line based on playback position.
pub fn current_lyric_index(lines: &[LyricLine], position_ms: u64) -> Option<usize> {
    if lines.is_empty() {
        return None;
    }

    // Binary search for the last line whose timestamp <= position
    let idx = lines.partition_point(|l| l.timestamp_ms <= position_ms);
    if idx == 0 { Some(0) } else { Some(idx - 1) }
}

/// Render lyrics with scrolling viewport.
pub fn render_lyrics(
    f: &mut Frame,
    area: Rect,
    theme: &ResolvedTheme,
    lyrics: &[LyricLine],
    position_ms: u64,
) {
    let block = Block::default()
        .title(" Lyrics ")
        .borders(Borders::ALL)
        .border_style(theme.secondary);

    let inner = block.inner(area);
    f.render_widget(block, area);

    if lyrics.is_empty() {
        let placeholder = Paragraph::new("No lyrics available")
            .style(theme.inactive)
            .alignment(ratatui::layout::Alignment::Center);
        f.render_widget(placeholder, inner);
        return;
    }

    let current = current_lyric_index(lyrics, position_ms).unwrap_or(0);
    let before = theme.lyrics_before;
    let after = theme.lyrics_after;
    let _total_visible = (before + 1 + after).min(inner.height as usize);

    // Calculate the range of lyric lines to display
    let start = current.saturating_sub(before);
    let end = (current + after + 1).min(lyrics.len());

    // Build lines with padding for vertical centering
    let content_lines = end - start;
    let padding = (inner.height as usize).saturating_sub(content_lines) / 2;

    let mut display_lines: Vec<Line> = Vec::new();

    // Top padding
    for _ in 0..padding {
        display_lines.push(Line::from(""));
    }

    // Lyric lines
    for (i, lyric) in lyrics[start..end].iter().enumerate() {
        let idx = start + i;
        let style = if idx == current {
            theme.lyrics_active
        } else {
            theme.lyrics_inactive
        };

        let text = lyric_text(&lyric.text);
        let span = Span::styled(text, style);
        display_lines.push(Line::from(span).alignment(ratatui::layout::Alignment::Center));
    }

    let paragraph = Paragraph::new(display_lines);
    f.render_widget(paragraph, inner);
}

fn lyric_text(text: &str) -> String {
    // Truncate if too long, add ellipsis
    if text.len() > 80 {
        format!("{}…", &text[..79])
    } else {
        text.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_lrc() {
        let lrc = "[00:00.00]First line\n[00:05.50]Second line\n[00:10.00]Third line\n";
        let lines = parse_lrc(lrc);
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0].timestamp_ms, 0);
        assert_eq!(lines[1].timestamp_ms, 5500);
        assert_eq!(lines[2].text, "Third line");
    }

    #[test]
    fn test_parse_lrc_with_metadata() {
        let lrc = "[ti:Song Title]\n[ar:Artist]\n[00:00.00]Lyrics\n";
        let lines = parse_lrc(lrc);
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].text, "Lyrics");
    }

    #[test]
    fn test_parse_timestamp() {
        assert_eq!(parse_timestamp("01:30.50"), Some(90_500));
        assert_eq!(parse_timestamp("00:05.00"), Some(5_000));
        assert_eq!(parse_timestamp("02:00"), Some(120_000));
        assert_eq!(parse_timestamp("invalid"), None);
    }

    #[test]
    fn test_current_lyric_index() {
        let lines = vec![
            LyricLine {
                timestamp_ms: 0,
                text: "A".into(),
            },
            LyricLine {
                timestamp_ms: 5000,
                text: "B".into(),
            },
            LyricLine {
                timestamp_ms: 10000,
                text: "C".into(),
            },
        ];
        assert_eq!(current_lyric_index(&lines, 0), Some(0));
        assert_eq!(current_lyric_index(&lines, 3000), Some(0));
        assert_eq!(current_lyric_index(&lines, 5000), Some(1));
        assert_eq!(current_lyric_index(&lines, 7000), Some(1));
        assert_eq!(current_lyric_index(&lines, 15000), Some(2));
    }

    #[test]
    fn test_empty_lyrics() {
        assert!(parse_lrc("").is_empty());
        assert!(current_lyric_index(&[], 0).is_none());
    }
}
