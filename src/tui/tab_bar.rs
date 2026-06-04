use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Tabs};

use crate::tui::app::ViewMode;
use crate::tui::theme::ResolvedTheme;

// Nerd Font icons
const ICON_MUSIC: &str = "\u{F001}"; // nf-fa-music
const ICON_LIST: &str = "\u{039E}"; // Greek capital xi (list-like)
const ICON_SEARCH: &str = "\u{F002}"; // nf-fa-magnifying_glass

/// Tab definitions with icons.
pub const TABS: &[(&str, &str)] = &[
    ("Play", ICON_MUSIC),
    ("Library", ICON_LIST),
    ("Search", ICON_SEARCH),
];

/// Render the tab bar at the top of the screen.
pub fn render_tab_bar(
    f: &mut Frame,
    area: Rect,
    theme: &ResolvedTheme,
    current: ViewMode,
    visible: bool,
) {
    if !visible {
        return;
    }

    let selected = match current {
        ViewMode::PlayView => 0,
        ViewMode::Library => 1,
        ViewMode::Search => 2,
    };

    let titles: Vec<Line> = TABS
        .iter()
        .enumerate()
        .map(|(i, (name, icon))| {
            let style = if i == selected {
                theme.tab_active
            } else {
                theme.tab_inactive
            };
            Line::from(Span::styled(format!(" {} {} ", icon, name), style))
        })
        .collect();

    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme.secondary)
                .title(" EchoPlay "),
        )
        .select(selected)
        .style(theme.text)
        .highlight_style(theme.tab_active);

    f.render_widget(tabs, area);
}

pub fn tab_to_viewmode(index: usize) -> ViewMode {
    match index {
        0 => ViewMode::PlayView,
        1 => ViewMode::Library,
        2 => ViewMode::Search,
        _ => ViewMode::PlayView,
    }
}
