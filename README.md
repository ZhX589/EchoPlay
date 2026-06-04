<div align="center">

<img src="https://img.shields.io/badge/-EchoPlay-1a1a2e?style=for-the-badge&labelColor=1a1a2e&color=0f3460" alt="EchoPlay">

### Terminal Music Player

<a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/Rust-2024-edition-orange?logo=rust&logoColor=white" alt="Rust"></a>
<a href="LICENSE"><img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="MIT License"></a>
<img src="https://img.shields.io/badge/Tests-74%20passed-brightgreen" alt="Tests">
<img src="https://img.shields.io/badge/Platform-Linux%20%7C%20macOS%20%7C%20Windows-lightgrey" alt="Platform">

---

[**English**](README.en.md) · [**简体中文**](README.zh.md)

</div>

---

## Highlights

```
 Plugin Architecture        Local Playback          Online Sources
┌─────────────────┐    ┌──────────────────┐    ┌──────────────────┐
│  Provider Trait  │    │  mp3 flac wav    │    │    QQ Music      │
│  One Interface   │    │  ogg m4a aac     │    │  Search + Lyrics │
│  Extensible      │    │  Tag Reader      │    │  RPC Gateway     │
└─────────────────┘    └──────────────────┘    └──────────────────┘
```

## Quick Start

```bash
# Build
git clone <repo-url> && cd EchoPlay
cargo build --release

# Run
./target/release/EchoPlay                # Launch TUI
./target/release/EchoPlay --play ~/Music # Play local directory
./target/release/EchoPlay --search "周杰伦" # Search QQ Music
```

## TUI Preview

```
┌─ EchoPlay ─────────────────────────────────────────────┐
│                    ┌────────────┐                       │
│                    │  ▄▄████▄▄  │                       │
│                    │  ▀▀████▀▀  │                       │
│                    │  ▄▄████▄▄  │                       │
│                    └────────────┘                       │
│                   Song Title - Artist                   │
│                        Album Name                       │
│                                                        │
│              1:23 ━━━━━━━━━━━━━━━━━ 4:56               │
│                                                        │
│            ⏮    ▶    ⏭   🔀  🔁  🔊80%                │
│                                                        │
│  ┌─ Lyrics ────────────────────────────────────────┐   │
│  │           Previous lyric line                    │   │
│  │      ★  Current lyric line (highlighted)  ★     │   │
│  │             Next lyric line                      │   │
│  └─────────────────────────────────────────────────┘   │
└────────────────────────────────────────────────────────┘
```

## Keybindings

| Key | Action | Key | Action |
|-----|--------|-----|--------|
| `Tab` | Show/hide tabs | `Space` | Play/Pause |
| `1` `2` `3` | Switch view | `n` / `p` | Next / Previous |
| `↑` `↓` | Navigate list | `+` `-` | Volume |
| `Enter` | Play selected | `m` | Cycle play mode |
| `/` | Search | `Esc` | Back |
| `q` | Quit | | |

## Configuration

`~/.config/echoplay/config.toml`

```toml
[audio]
volume = 80

[library]
scan_dirs = ["~/Music"]

[ui]
theme = "default"
```

## Architecture

```
Provider Trait  →  ProviderRegistry  →  Library (facade)
     │                                        │
     ├── QQProvider (QQ Music)                │
     └── LocalProvider (local files)          │
                                              ▼
                              ┌──────────────────────────┐
                              │        Player (rodio)     │
                              │  play / pause / stop      │
                              │  volume / seek            │
                              └──────────────────────────┘
                                              │
                              ┌──────────────────────────┐
                              │     TUI (ratatui)         │
                              │  views / widgets / theme  │
                              └──────────────────────────┘
```

## Development

```bash
cargo build                              # Build
cargo test                               # Run all tests (74)
cargo run -- --play ~/Music              # Test local playback
cargo run -- --search "test"             # Test QQ Music search
```

## License

[MIT](LICENSE)
