<div align="center">

# EchoPlay

**终端音乐播放器** · 插件化架构 · 本地播放 · 在线音源

[English](README.en.md) | **简体中文**

[![Rust](https://img.shields.io/badge/Rust-2024-edition-orange?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Tests](https://img.shields.io/badge/Tests-74%20passed-brightgreen)](#开发)
[![Platform](https://img.shields.io/badge/Platform-Linux%20%7C%20macOS%20%7C%20Windows-lightgrey)](#安装)

</div>

---

## 功能

- 纯终端 TUI 界面，Nerd Font 图标，轻量快捷
- 本地音乐播放：mp3 / flac / wav / ogg / m4a / aac
- 在线音源：QQ 音乐（搜索 + 歌词）
- 插件化 Provider 系统 — 实现一个 trait 即可接入新音源
- 跨源备用：某源获取失败时自动搜索其他源
- Xero 风格播放界面：封面 / 进度条 / 控制按钮 / 同步歌词
- 可配置主题（颜色 / 边框 / 进度条 / 歌词样式）
- TOML 配置文件

## 安装

```bash
git clone <repo-url> && cd EchoPlay
cargo build --release
```

编译产物位于 `target/release/EchoPlay`。

## 使用

```bash
# 启动 TUI（空界面）
EchoPlay

# 扫描本地目录并播放
EchoPlay --play ~/Music

# 搜索 QQ 音乐（输出结果后退出）
EchoPlay --search "周杰伦"
```

## 快捷键

### 导航

| 按键 | 功能 |
|------|------|
| `Tab` | 显示 / 隐藏标签栏 |
| `1` | 切换到播放视图 |
| `2` | 切换到曲库视图 |
| `3` | 切换到搜索视图 |
| `Esc` | 返回 / 清除搜索 / 隐藏标签栏 |
| `q` | 退出 |

### 播放控制

| 按键 | 功能 |
|------|------|
| `Space` | 播放 / 暂停 |
| `n` | 下一首 |
| `p` | 上一首 |
| `+` / `=` | 音量增大 |
| `-` | 音量减小 |
| `m` | 切换播放模式（顺序 → 单曲循环 → 随机 → 列表循环） |

### 列表操作

| 按键 | 功能 |
|------|------|
| `↑` | 上移选择 |
| `↓` | 下移选择 |
| `Enter` | 播放选中歌曲 |

### 搜索

| 按键 | 功能 |
|------|------|
| `/` | 聚焦搜索框 |
| `Enter` | 执行搜索 |
| `Esc` | 清除搜索（第1次）/ 退出搜索（第2次） |

搜索模式下所有按键均为文本输入 — `n`、`p`、`1`、`2`、`3` 输入字符而非触发快捷键。

## 视图

**播放视图**（默认）— Xero 风格布局：
- 居中封面（ASCII 半块字符渲染）
- 歌曲标题 / 歌手 / 专辑
- 进度条（带时间戳）
- 控制按钮（Nerd Font 图标）
- 同步歌词（滚动视口，当前行高亮）

**曲库视图** — 本地扫描或播放列表的歌曲列表。当前播放歌曲用音符图标标记。

**搜索视图** — 搜索输入框 + 搜索结果列表。搜索 QQ 音乐。

## 标签栏

默认隐藏。按 `Tab` 显示。用 `1`/`2`/`3` 或方向键选择。选择后自动隐藏。

## 配置

文件路径：`~/.config/echoplay/config.toml`

```toml
[audio]
volume = 80          # 音量 0-100
device = "default"   # 音频设备

[library]
scan_dirs = [        # 本地扫描目录
    "~/Music",
    "/home/user/Downloads",
]

[ui]
theme = "default"    # 主题名称
```

## 主题

主题在 `src/tui/theme.rs` 中定义。所有颜色支持命名值或十六进制码。

| 配置项 | 默认值 | 说明 |
|--------|--------|------|
| `colors.accent` | `cyan` | 进度条、当前歌词行、高亮 |
| `colors.highlight` | `dark_gray` | 选中项背景 |
| `colors.text` | `white` | 主文本 |
| `colors.secondary` | `gray` | 次要文本（时间戳、歌手名） |
| `colors.inactive` | `dark_gray` | 未激活元素 |
| `borders.preset` | `rounded` | 边框样式：plain / rounded / double / thick |
| `lyrics.lines_before` | `2` | 当前行之前显示的歌词行数 |
| `lyrics.lines_after` | `2` | 当前行之后显示的歌词行数 |
| `cover.max_width` | `32` | 封面最大宽度（列） |
| `cover.max_height` | `16` | 封面最大高度（行） |

支持的颜色名：`black`、`red`、`green`、`yellow`、`blue`、`magenta`、`cyan`、`gray`、`dark_gray`、`white`、`light_red`、`light_green`、`light_yellow`、`light_blue`、`light_magenta`、`light_cyan`，或十六进制 `#ff8800`。

## 架构

```
src/
├── main.rs              # CLI 入口 (clap) + TUI 事件循环
├── config.rs            # 配置 (TOML 加载/保存)
├── error.rs             # 错误类型
├── media.rs             # 音质 / 媒体 / 排序策略
├── library.rs           # Library 门面（分发到 Provider）
├── standby.rs           # 跨源备用匹配
├── model/               # Brief/Normal 双层模型 + URI
├── player/
│   ├── mod.rs           # Player (rodio)
│   ├── state.rs         # PlayerState / PlayerInfo
│   └── playlist.rs      # Playlist + PlayMode
├── provider/
│   ├── mod.rs           # Provider trait（15 个 async 方法）
│   └── registry.rs      # ProviderRegistry
├── providers/
│   ├── qq/              # QQ 音乐（RPC 网关，MD5 签名）
│   └── local/           # 本地文件（DirScanner + TagReader）
└── tui/
    ├── theme.rs         # Theme + ResolvedTheme
    ├── app.rs           # App 状态 + 视图路由
    ├── event.rs         # InputMode + 按键映射
    ├── tab_bar.rs       # 标签栏
    ├── ui.rs            # 顶层绘制分发
    ├── views/           # 播放 / 列表 / 搜索视图
    └── widgets/         # 进度条 / 控制 / 封面 / 歌词
```

### 扩展 Provider

实现 `Provider` trait 即可接入新音源：

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

    // 其他方法有默认实现，返回 NotSupported
}
```

在 `main.rs` 中注册：
```rust
registry.register(Arc::new(MyProvider::new()))?;
```

### 模型系统

每个模型有 `Brief`（轻量，用于显示）和 `Normal`（完整详情，需网络请求）两层：

| Brief | Normal |
|-------|--------|
| `BriefSong` | `Song` |
| `BriefAlbum` | `Album` |
| `BriefArtist` | `Artist` |
| `BriefPlaylist` | `Playlist` |

URI 格式：`echoplay://{source}/songs/{identifier}`

## 依赖

| Crate | 用途 |
|-------|------|
| rodio | 音频播放 (mp3/flac/wav/ogg) |
| ratatui | TUI 框架 |
| crossterm | 终端事件 |
| lofty | 音频文件元数据 (tags) |
| reqwest | HTTP 客户端 (QQ 音乐 API) |
| tokio | 异步运行时 |
| serde / toml | 配置序列化 |
| clap | CLI 参数解析 |

## 开发

```bash
cargo build
cargo test        # 56 单元测试 + 18 集成测试
cargo fmt
cargo clippy
```

## 许可证

[MIT](LICENSE)
