use std::time::Duration;

/// Playback state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerState {
    Playing,
    Paused,
    Stopped,
}

/// Playback info (position, duration, etc.).
#[derive(Debug, Clone, Copy)]
pub struct PlayerInfo {
    /// Current position — placeholder for now (rodio doesn't expose position easily).
    pub position: Duration,
}

impl Default for PlayerInfo {
    fn default() -> Self {
        Self {
            position: Duration::ZERO,
        }
    }
}
