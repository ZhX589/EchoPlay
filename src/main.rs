use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use clap::Parser;

use EchoPlay::config::Config;
use EchoPlay::library::Library;
use EchoPlay::player::Player;
use EchoPlay::provider::Provider;
use EchoPlay::provider::registry::ProviderRegistry;
use EchoPlay::providers::QQProvider;
use EchoPlay::providers::local::LocalProvider;
use EchoPlay::tui::app::{App, ViewMode};
use EchoPlay::tui::event::{AppEvent, InputMode, poll_event};
use EchoPlay::tui::tab_bar;
use EchoPlay::tui::theme::Theme;
use EchoPlay::tui::ui;

#[derive(Parser)]
#[command(name = "echoplay", about = "Terminal music player")]
struct Cli {
    /// Play music from specified directory
    #[arg(long)]
    play: Option<PathBuf>,

    /// Search and play a song
    #[arg(long)]
    search: Option<String>,
}

#[tokio::main]
async fn main() {
    // Install panic hook to restore terminal on panic
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = EchoPlay::tui::restore_terminal(&mut EchoPlay::tui::init_terminal().unwrap());
        original_hook(info);
    }));
    let cli = Cli::parse();
    let config = Config::load();

    // Initialize providers
    let local = Arc::new(LocalProvider::new());

    // Scan local directories from config
    let scan_paths: Vec<&std::path::Path> = config
        .library
        .scan_dirs
        .iter()
        .map(|s| std::path::Path::new(s.as_str()))
        .collect();

    // If --play directory specified, scan it
    if let Some(ref dir) = cli.play {
        local.scan(&[dir.as_path()]);
    } else if !scan_paths.is_empty() {
        local.scan(&scan_paths);
    }

    let mut registry = ProviderRegistry::new();
    registry.register(local.clone()).unwrap();
    registry.register(Arc::new(QQProvider::new())).unwrap();

    let library = Arc::new(Library::new(registry));

    // Initialize player
    let player = match Player::new() {
        Ok(p) => {
            p.set_volume(config.audio.volume as f32 / 100.0);
            Arc::new(p)
        }
        Err(e) => {
            eprintln!("Failed to initialize audio: {}", e);
            return;
        }
    };

    // If --search specified, search and print results, then exit
    if let Some(ref query) = cli.search {
        let results = library
            .search(query, EchoPlay::model::SearchType::Song, None)
            .await;
        for r in results.into_iter().flatten() {
            for (i, song) in r.songs.iter().take(10).enumerate() {
                println!(
                    "{:3}. {} - {} [{}]",
                    i + 1,
                    song.title,
                    song.artists_name,
                    song.album_name
                );
            }
        }
        return;
    }

    // Start TUI
    let mut terminal = match EchoPlay::tui::init_terminal() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Failed to initialize terminal: {}", e);
            return;
        }
    };

    let theme = Theme::default();
    let mut app = App::new(library, player, theme);

    // If we scanned local files, load them into the playlist
    if local.song_count() > 0 {
        let search_result = local
            .search("", EchoPlay::model::SearchType::Song)
            .await
            .unwrap_or_else(|_| {
                EchoPlay::model::SearchResult::empty("local", "", EchoPlay::model::SearchType::Song)
            });
        app.set_playlist(search_result.songs);
    }

    // Main event loop
    let tick_rate = Duration::from_millis(100);
    loop {
        // Draw UI
        if terminal.draw(|f| ui::draw(f, &app)).is_err() {
            break;
        }

        // Determine input mode
        let input_mode = if app.view_mode == ViewMode::Search {
            InputMode::Search
        } else {
            InputMode::Normal
        };

        // Handle events
        if let Some(event) = poll_event(tick_rate, input_mode) {
            match event {
                AppEvent::Quit => break,

                // Tab navigation
                AppEvent::ToggleTabBar => app.toggle_tab_bar(),
                AppEvent::NextTab => app.next_tab(),
                AppEvent::PrevTab => app.prev_tab(),
                AppEvent::SwitchToTab(idx) => {
                    let mode = tab_bar::tab_to_viewmode(idx);
                    app.switch_view(mode);
                }

                // Playback
                AppEvent::TogglePlay => app.toggle_play(),
                AppEvent::Next => app.next_song(),
                AppEvent::Previous => app.prev_song(),
                AppEvent::VolumeUp => app.adjust_volume(0.05),
                AppEvent::VolumeDown => app.adjust_volume(-0.05),
                AppEvent::CyclePlayMode => app.cycle_play_mode(),

                // Navigation
                AppEvent::Up => app.select_up(),
                AppEvent::Down => app.select_down(),
                AppEvent::PlaySelected => app.play_at_index(app.selected_index),

                // Search
                AppEvent::FocusSearch => {
                    app.view_mode = ViewMode::Search;
                }
                AppEvent::ConfirmSearch => {
                    app.do_search().await;
                    // Stay in Search view to show results
                }
                AppEvent::Escape => {
                    if app.tab_bar_visible {
                        app.tab_bar_visible = false;
                    } else if app.view_mode == ViewMode::Search {
                        if !app.search_query.is_empty() {
                            // First Esc clears search
                            app.search_query.clear();
                            app.search_results.clear();
                            app.selected_index = 0;
                        } else {
                            // Second Esc goes back to play view
                            app.view_mode = ViewMode::PlayView;
                        }
                    } else {
                        app.toggle_tab_bar();
                    }
                }

                // Text input (only in search mode)
                AppEvent::Char(c) => {
                    if app.view_mode == ViewMode::Search {
                        app.search_query.push(c);
                    }
                }
                AppEvent::Backspace => {
                    if app.view_mode == ViewMode::Search {
                        app.search_query.pop();
                    }
                }

                AppEvent::None => {}
            }
        }

        // Update playback position
        app.update_position();

        // Check if playback finished and auto-advance
        if app.player_state() == EchoPlay::player::state::PlayerState::Playing
            && app.player.is_finished()
        {
            app.next_song();
        }

        if app.should_quit {
            break;
        }
    }

    // Restore terminal
    let _ = EchoPlay::tui::restore_terminal(&mut terminal);
}
