<div align="center">

# EchoPlay

**Terminal Music Player** · Plugin Architecture · Local Playback · Online Sources

**English** | [简体中文](README.zh.md)

[![Rust](https://img.shields.io/badge/Rust-2024-edition-orange?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Tests](https://img.shields.io/badge/Tests-74%20passed-brightgreen)](#development)
[![Platform](https://img.shields.io/badge/Platform-Linux%20%7C%20macOS%20%7C%20Windows-lightgrey)](#installation)

</div>

---

## Features

- Terminal TUI (ratatui + crossterm) with Nerd Font icons
- Local music playback: mp3 / flac / wav / ogg / m4a / aac
- Online music sources: QQ Music (search + lyrics)
- Plugin-based provider system — add new sources by implementing one trait
- Cross-provider standby: auto-search other sources when one fails
- Xero-style playback view: cover art, progress bar, controls, synced lyrics
- Configurable theme (colors, borders, progress bar, lyrics)
- TOML config file

## Installation

```bash
git clone <repo-url> && cd EchoPlay
cargo build --release
```

Binary at `target/release/EchoPlay`.

## Usage

```bash
# Launch TUI (empty)
EchoPlay

# Scan a local directory and play
EchoPlay --play ~/Music

# Search QQ Music from CLI (prints results, exits)
EchoPlay --search "周杰伦"
```

## Keybindings

### Navigation

| Key | Action |
|-----|--------|
| `Tab` | Show / hide tab bar |
| `1` | Switch to Play view |
| `2` | Switch to Library view |
| `3` | Switch to Search view |
| `Esc` | Back / clear search / hide tabs |
| `q` | Quit |

### Playback

| Key | Action |
|-----|--------|
| `Space` | Play / Pause |
| `n` | Next song |
| `p` | Previous song |
| `+` / `=` | Volume up |
| `-` | Volume down |
| `m` | Cycle play mode (sequential → single loop → random → loop all) |

### List Navigation

| Key | Action |
|-----|--------|
| `Up` | Move selection up |
| `Down` | Move selection down |
| `Enter` | Play selected song |

### Search

| Key | Action |
|-----|--------|
| `/` | Focus search input |
| `Enter` | Execute search |
| `Esc` | Clear search (1st) / exit search (2nd) |

In search mode, all keys are text input — `n`, `p`, `1`, `2`, `3` type characters instead of triggering shortcuts.

## Views

**Play View** (default) — Xero-style layout:
- Centered cover art (ASCII half-block rendering)
- Song title / artist / album
- Progress bar with timestamps
- Control buttons (Nerd Font icons)
- Synced lyrics (scrolling viewport, current line highlighted)

**Library View** — song list from local scan or playlist. Currently playing song marked with music icon.

**Search View** — search input + results list. Searches QQ Music.

## Tab Bar

Hidden by default. Press `Tab` to show. Select with `1`/`2`/`3` or arrow keys. Auto-hides after selection.

## Configuration

File path: `~/.config/echoplay/config.toml`

```toml
[audio]
volume = 80          # Volume 0-100
device = "default"   # Audio device

[library]
scan_dirs = [        # Local scan directories
    "~/Music",
    "/home/user/Downloads",
]

[ui]
theme = "default"    # Theme name
```

## Theme

Theme is defined in `src/tui/theme.rs`. All colors support named values or hex codes.

| Key | Default | Description |
|-----|---------|-------------|
| `colors.accent` | `cyan` | Progress bar, current lyric, highlights |
| `colors.highlight` | `dark_gray` | Selected item background |
| `colors.text` | `white` | Primary text |
| `colors.secondary` | `gray` | Timestamps, artist names |
| `colors.inactive` | `dark_gray` | Inactive elements |
| `borders.preset` | `rounded` | Border style: plain / rounded / double / thick |
| `lyrics.lines_before` | `2` | Lyric lines before current |
| `lyrics.lines_after` | `2` | Lyric lines after current |
| `cover.max_width` | `32` | Cover art max width (columns) |
| `cover.max_height` | `16` | Cover art max height (rows) |

Supported color names: `black`, `red`, `green`, `yellow`, `blue`, `magenta`, `cyan`, `gray`, `dark_gray`, `white`, `light_red`, `light_green`, `light_yellow`, `light_blue`, `light_magenta`, `light_cyan`, or hex `#ff8800`.

## Architecture

```
src/
├── main.rs              # CLI entry (clap) + TUI event loop
├── config.rs            # Config (TOML load/save)
├── error.rs             # Error types
├── media.rs             # AudioQuality, Media, QualitySortPolicy
├── library.rs           # Library facade (dispatches to providers)
├── standby.rs           # Cross-provider standby matching
├── model/               # Brief/Normal dual-layer models + URI
├── player/
│   ├── mod.rs           # Player (rodio: play/pause/stop/volume)
│   ├── state.rs         # PlayerState / PlayerInfo
│   └── playlist.rs      # Playlist + PlayMode
├── provider/
│   ├── mod.rs           # Provider trait (15 async methods)
│   └── registry.rs      # ProviderRegistry
├── providers/
│   ├── qq/              # QQ Music (RPC gateway, MD5 signing)
│   └── local/           # Local files (DirScanner + TagReader)
└── tui/
    ├── theme.rs         # Theme + ResolvedTheme
    ├── app.rs           # App state + ViewMode routing
    ├── event.rs         # InputMode + key mapping
    ├── tab_bar.rs       # Tab bar
    ├── ui.rs            # Top-level draw dispatch
    ├── views/           # Playback / Library / Search views
    └── widgets/         # Progress bar / Controls / Cover / Lyrics
```

### Adding a Provider

Implement the `Provider` trait to add a new music source:

```rust
#[async_trait]
impl Provider for MyProvider {
    fn identifier(&self) -> &str { "mysource" }
    fn name(&self) -> &str { "My Music Source" }
    fn as_any(&self) -> &dyn Any { self }

    async fn search(&self, keyword: &str, search_type: SearchType)
        -> Result<SearchResult, EchoError> { ... }

    async fn song_get(&self, identifier: &str)
        -> Result<Song, EchoError> { ... }

    async fn song_get_media(&self, song: &BriefSong, quality: AudioQuality)
        -> Result<Option<Media>, EchoError> { ... }

    // Other methods have default implementations returning NotSupported
}
```

Register in `main.rs`:
```rust
registry.register(Arc::new(MyProvider::new()))?;
```

### Model System

Every model has a `Brief` variant (lightweight, for display) and a `Normal` variant (full details, requires network):

| Brief | Normal |
|-------|--------|
| `BriefSong` | `Song` |
| `BriefAlbum` | `Album` |
| `BriefArtist` | `Artist` |
| `BriefPlaylist` | `Playlist` |

URI format: `echoplay://{source}/songs/{identifier}`

## Dependencies

| Crate | Purpose |
|-------|---------|
| rodio | Audio playback (mp3/flac/wav/ogg) |
| ratatui | TUI framework |
| crossterm | Terminal events |
| lofty | Audio file metadata (tags) |
| reqwest | HTTP client (QQ Music API) |
| tokio | Async runtime |
| serde / toml | Config serialization |
| clap | CLI argument parsing |

## Development

```bash
cargo build
cargo test        # 56 unit + 18 integration tests
cargo fmt
cargo clippy
```

## License

[MIT](LICENSE)
