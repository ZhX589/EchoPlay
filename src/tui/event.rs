use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

/// User input events mapped from crossterm.
#[derive(Debug, Clone)]
pub enum AppEvent {
    Quit,
    ToggleTabBar,
    NextTab,
    PrevTab,
    SwitchToTab(usize),
    TogglePlay,
    Next,
    Previous,
    VolumeUp,
    VolumeDown,
    Up,
    Down,
    PlaySelected,
    CyclePlayMode,
    FocusSearch,
    ConfirmSearch,
    Escape,
    Char(char),
    Backspace,
    None,
}

/// Current input mode for proper key routing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    Search,
}

/// Poll crossterm events and convert to AppEvent.
pub fn poll_event(tick_rate: Duration, input_mode: InputMode) -> Option<AppEvent> {
    if event::poll(tick_rate).ok()? {
        match event::read().ok()? {
            Event::Key(key) => Some(map_key(key, input_mode)),
            _ => None,
        }
    } else {
        None
    }
}

fn map_key(key: KeyEvent, input_mode: InputMode) -> AppEvent {
    // In search mode, most keys are text input
    if input_mode == InputMode::Search {
        return match (key.modifiers, key.code) {
            (KeyModifiers::CONTROL, KeyCode::Char('c')) => AppEvent::Quit,
            (_, KeyCode::Esc) => AppEvent::Escape,
            (_, KeyCode::Enter) => AppEvent::ConfirmSearch,
            (_, KeyCode::Backspace) => AppEvent::Backspace,
            (_, KeyCode::Up) => AppEvent::Up,
            (_, KeyCode::Down) => AppEvent::Down,
            (_, KeyCode::Char(c)) => AppEvent::Char(c),
            _ => AppEvent::None,
        };
    }

    // Normal mode
    match (key.modifiers, key.code) {
        (KeyModifiers::CONTROL, KeyCode::Char('c')) => AppEvent::Quit,

        // Tab navigation
        (_, KeyCode::Tab) => AppEvent::ToggleTabBar,
        (_, KeyCode::BackTab) => AppEvent::PrevTab,

        // Escape
        (_, KeyCode::Esc) => AppEvent::Escape,

        // Arrow keys
        (_, KeyCode::Up) => AppEvent::Up,
        (_, KeyCode::Down) => AppEvent::Down,

        // Enter — context dependent
        (_, KeyCode::Enter) => AppEvent::PlaySelected,

        // Number keys for tab switching
        (_, KeyCode::Char('1')) => AppEvent::SwitchToTab(0),
        (_, KeyCode::Char('2')) => AppEvent::SwitchToTab(1),
        (_, KeyCode::Char('3')) => AppEvent::SwitchToTab(2),

        // Playback controls
        (_, KeyCode::Char(' ')) => AppEvent::TogglePlay,
        (_, KeyCode::Char('n')) => AppEvent::Next,
        (_, KeyCode::Char('p')) => AppEvent::Previous,
        (_, KeyCode::Char('+')) | (_, KeyCode::Char('=')) => AppEvent::VolumeUp,
        (_, KeyCode::Char('-')) => AppEvent::VolumeDown,
        (_, KeyCode::Char('m')) => AppEvent::CyclePlayMode,

        // Search
        (_, KeyCode::Char('/')) => AppEvent::FocusSearch,

        _ => AppEvent::None,
    }
}
