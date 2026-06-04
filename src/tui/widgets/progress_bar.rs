use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::widgets::Gauge;

use crate::tui::theme::ResolvedTheme;

/// Render a progress bar with optional timestamps.
pub fn render_progress_bar(
    f: &mut Frame,
    area: Rect,
    theme: &ResolvedTheme,
    position_secs: f64,
    duration_secs: f64,
) {
    let ratio = if duration_secs > 0.0 && !duration_secs.is_nan() && !duration_secs.is_infinite() {
        (position_secs / duration_secs).clamp(0.0, 1.0)
    } else {
        0.0
    };

    let label = if theme.show_timestamps {
        format!(
            "{} / {}",
            format_time(position_secs),
            format_time(duration_secs)
        )
    } else {
        String::new()
    };

    let gauge = Gauge::default()
        .gauge_style(theme.progress_filled)
        .ratio(ratio)
        .label(label);

    f.render_widget(gauge, area);
}

/// Format seconds as M:SS or H:MM:SS.
pub fn format_time(secs: f64) -> String {
    if secs.is_nan() || secs.is_infinite() || secs < 0.0 {
        return "0:00".to_string();
    }

    let total_secs = secs as u64;
    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;
    let seconds = total_secs % 60;

    if hours > 0 {
        format!("{}:{:02}:{:02}", hours, minutes, seconds)
    } else {
        format!("{}:{:02}", minutes, seconds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_time() {
        assert_eq!(format_time(0.0), "0:00");
        assert_eq!(format_time(65.0), "1:05");
        assert_eq!(format_time(3661.0), "1:01:01");
    }

    #[test]
    fn test_format_time_edge_cases() {
        assert_eq!(format_time(-1.0), "0:00");
        assert_eq!(format_time(f64::NAN), "0:00");
        assert_eq!(format_time(f64::INFINITY), "0:00");
    }
}
