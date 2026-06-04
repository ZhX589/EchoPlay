use ratatui::style::{Color, Modifier, Style};
use serde::{Deserialize, Serialize};

/// Application theme. All visual styling is centralized here.
/// Values are configurable via TOML; nothing is hardcoded in widget code.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub colors: ThemeColors,
    pub borders: BorderStyle,
    pub progress_bar: ProgressBarStyle,
    pub lyrics: LyricsStyle,
    pub cover: CoverStyle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeColors {
    /// Accent color — progress bar fill, current lyric line, highlights.
    pub accent: String,
    /// Highlight background for selected items.
    pub highlight: String,
    /// Primary text color.
    pub text: String,
    /// Secondary text (timestamps, artist names, labels).
    pub secondary: String,
    /// Inactive elements (unfilled progress, past/future lyrics).
    pub inactive: String,
    /// Optional background color. None = terminal default.
    pub bg: Option<String>,
    /// Tab bar active tab color.
    pub tab_active: String,
    /// Tab bar inactive tab color.
    pub tab_inactive: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BorderStyle {
    /// Preset: "plain", "rounded", "double", "thick".
    pub preset: String,
    /// Border color.
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressBarStyle {
    /// Filled portion symbol.
    pub filled: String,
    /// Unfilled portion symbol.
    pub unfilled: String,
    /// Show timestamps on left/right.
    pub show_timestamps: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LyricsStyle {
    /// Number of lines to show before the current line.
    pub lines_before: usize,
    /// Number of lines to show after the current line.
    pub lines_after: usize,
    /// Current line color.
    pub active_color: String,
    /// Inactive lines color.
    pub inactive_color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverStyle {
    /// Enable cover art rendering.
    pub enabled: bool,
    /// Max width in terminal columns.
    pub max_width: u16,
    /// Max height in terminal rows.
    pub max_height: u16,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            colors: ThemeColors {
                accent: "cyan".into(),
                highlight: "dark_gray".into(),
                text: "white".into(),
                secondary: "gray".into(),
                inactive: "dark_gray".into(),
                bg: None,
                tab_active: "cyan".into(),
                tab_inactive: "gray".into(),
            },
            borders: BorderStyle {
                preset: "rounded".into(),
                color: "gray".into(),
            },
            progress_bar: ProgressBarStyle {
                filled: "━".into(),
                unfilled: "━".into(),
                show_timestamps: true,
            },
            lyrics: LyricsStyle {
                lines_before: 2,
                lines_after: 2,
                active_color: "cyan".into(),
                inactive_color: "dark_gray".into(),
            },
            cover: CoverStyle {
                enabled: true,
                max_width: 32,
                max_height: 16,
            },
        }
    }
}

// ---------------------------------------------------------------------------
// Resolved styles (runtime, not serialized)
// ---------------------------------------------------------------------------

/// Resolved theme with ratatui Style objects ready for use.
pub struct ResolvedTheme {
    pub accent: Style,
    pub highlight: Style,
    pub text: Style,
    pub secondary: Style,
    pub inactive: Style,
    pub bg: Option<Color>,
    pub tab_active: Style,
    pub tab_inactive: Style,
    pub border_color: Color,
    pub border_set: ratatui::widgets::Borders,
    pub progress_filled: Style,
    pub progress_unfilled: Style,
    pub progress_filled_symbol: String,
    pub progress_unfilled_symbol: String,
    pub show_timestamps: bool,
    pub lyrics_before: usize,
    pub lyrics_after: usize,
    pub lyrics_active: Style,
    pub lyrics_inactive: Style,
    pub cover_enabled: bool,
    pub cover_max_width: u16,
    pub cover_max_height: u16,
}

impl ResolvedTheme {
    pub fn resolve(theme: &Theme) -> Self {
        let accent = parse_color(&theme.colors.accent);
        let highlight = parse_color(&theme.colors.highlight);
        let text = parse_color(&theme.colors.text);
        let secondary = parse_color(&theme.colors.secondary);
        let inactive = parse_color(&theme.colors.inactive);
        let bg = theme.colors.bg.as_deref().map(parse_color);
        let border_color = parse_color(&theme.borders.color);

        Self {
            accent: Style::default().fg(accent),
            highlight: Style::default().fg(highlight).bg(accent),
            text: Style::default().fg(text),
            secondary: Style::default().fg(secondary),
            inactive: Style::default().fg(inactive),
            bg,
            tab_active: Style::default().fg(accent).add_modifier(Modifier::BOLD),
            tab_inactive: Style::default().fg(parse_color(&theme.colors.tab_inactive)),
            border_color,
            border_set: match theme.borders.preset.as_str() {
                "double" => ratatui::widgets::Borders::ALL,
                "thick" => ratatui::widgets::Borders::ALL,
                _ => ratatui::widgets::Borders::ALL,
            },
            progress_filled: Style::default().fg(accent),
            progress_unfilled: Style::default().fg(inactive),
            progress_filled_symbol: theme.progress_bar.filled.clone(),
            progress_unfilled_symbol: theme.progress_bar.unfilled.clone(),
            show_timestamps: theme.progress_bar.show_timestamps,
            lyrics_before: theme.lyrics.lines_before,
            lyrics_after: theme.lyrics.lines_after,
            lyrics_active: Style::default()
                .fg(parse_color(&theme.lyrics.active_color))
                .add_modifier(Modifier::BOLD),
            lyrics_inactive: Style::default().fg(parse_color(&theme.lyrics.inactive_color)),
            cover_enabled: theme.cover.enabled,
            cover_max_width: theme.cover.max_width,
            cover_max_height: theme.cover.max_height,
        }
    }
}

fn parse_color(s: &str) -> Color {
    match s.to_lowercase().as_str() {
        "black" => Color::Black,
        "red" => Color::Red,
        "green" => Color::Green,
        "yellow" => Color::Yellow,
        "blue" => Color::Blue,
        "magenta" => Color::Magenta,
        "cyan" => Color::Cyan,
        "gray" | "grey" => Color::Gray,
        "dark_gray" | "dark_grey" => Color::DarkGray,
        "white" => Color::White,
        "light_red" => Color::LightRed,
        "light_green" => Color::LightGreen,
        "light_yellow" => Color::LightYellow,
        "light_blue" => Color::LightBlue,
        "light_magenta" => Color::LightMagenta,
        "light_cyan" => Color::LightCyan,
        s if s.starts_with('#') && s.len() == 7 => {
            let r = u8::from_str_radix(&s[1..3], 16).unwrap_or(0);
            let g = u8::from_str_radix(&s[3..5], 16).unwrap_or(0);
            let b = u8::from_str_radix(&s[5..7], 16).unwrap_or(0);
            Color::Rgb(r, g, b)
        }
        _ => Color::White,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_theme() {
        let theme = Theme::default();
        assert_eq!(theme.colors.accent, "cyan");
        assert_eq!(theme.borders.preset, "rounded");
        assert_eq!(theme.lyrics.lines_before, 2);
    }

    #[test]
    fn test_resolve_theme() {
        let theme = Theme::default();
        let resolved = ResolvedTheme::resolve(&theme);
        assert_eq!(resolved.lyrics_before, 2);
        assert!(resolved.cover_enabled);
    }

    #[test]
    fn test_parse_hex_color() {
        let c = parse_color("#ff8800");
        assert_eq!(c, Color::Rgb(255, 136, 0));
    }

    #[test]
    fn test_parse_named_colors() {
        assert_eq!(parse_color("red"), Color::Red);
        assert_eq!(parse_color("Cyan"), Color::Cyan);
        assert_eq!(parse_color("dark_gray"), Color::DarkGray);
    }

    #[test]
    fn test_theme_roundtrip() {
        let theme = Theme::default();
        let toml_str = toml::to_string(&theme).unwrap();
        let parsed: Theme = toml::from_str(&toml_str).unwrap();
        assert_eq!(parsed.colors.accent, theme.colors.accent);
    }
}
