use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::player::state::PlayerState;
use crate::tui::theme::ResolvedTheme;

// Nerd Font icons (Font Awesome set)
const ICON_PREV: &str = "\u{F048}"; // nf-fa-backward_step
const ICON_PLAY: &str = "\u{F04B}"; // nf-fa-play
const ICON_PAUSE: &str = "\u{F04C}"; // nf-fa-pause
const ICON_NEXT: &str = "\u{F051}"; // nf-fa-forward_step
const ICON_SHUFFLE: &str = "\u{F074}"; // nf-fa-shuffle
const ICON_REPEAT: &str = "\u{F01E}"; // nf-fa-repeat
const ICON_REPEAT_ONE: &str = "\u{F01E}"; // nf-fa-repeat (same, we'll add indicator)
const ICON_VOLUME: &str = "\u{F028}"; // nf-fa-volume_high

/// Render playback control buttons centered in the area.
pub fn render_controls(
    f: &mut Frame,
    area: Rect,
    theme: &ResolvedTheme,
    player_state: PlayerState,
    play_mode: &str,
    volume_pct: u8,
) {
    let play_icon = match player_state {
        PlayerState::Playing => ICON_PAUSE,
        PlayerState::Paused | PlayerState::Stopped => ICON_PLAY,
    };

    let mode_icon = match play_mode {
        "sequential" => ICON_REPEAT,
        "single_loop" => ICON_REPEAT_ONE,
        "random" => ICON_SHUFFLE,
        "loop_all" => ICON_REPEAT,
        _ => ICON_REPEAT,
    };

    let spans = vec![
        Span::styled(format!("  {}  ", ICON_PREV), theme.text),
        Span::styled(format!("  {}  ", play_icon), theme.accent),
        Span::styled(format!("  {}  ", ICON_NEXT), theme.text),
        Span::styled(format!(" {} ", mode_icon), theme.secondary),
        Span::styled(format!(" {}{}% ", ICON_VOLUME, volume_pct), theme.secondary),
    ];

    let line = Line::from(spans);
    let paragraph = Paragraph::new(line)
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::NONE));

    f.render_widget(paragraph, area);
}
