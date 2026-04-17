# EchoPlay 🎵

> 终端里的音乐播放器，插件化架构，支持本地音乐和各大平台音源

## 特性

- 🖥️ 纯终端 TUI 界面，轻量快捷
- 🔌 插件化设计，按需加载音源
- 📁 本地音乐播放 (mp3/flac/wav/ogg)
- 🌐 网络音源支持 (QQ音乐、网易云等，开发中)
- 🎨 可定制主题

## 快速开始

### 安装

```bash
cargo install --git https://github.com/yourname/echoplay
```

### 使用

```bash
echoplay                    # 启动TUI
echoplay --play ~/music     # 播放指定目录
echoplay --help             # 查看帮助
```

## 快捷键

| 按键  | 功能          |
| ----- | ------------- |
| Space | 播放/暂停     |
| ←/→   | 后退/前进5秒  |
| ↑/↓   | 音量调节      |
| n/p   | 下一首/上一首 |
| q     | 退出          |

## 配置文件

`~/.config/echoplay/config.toml`

```toml
[audio]
volume = 80
device = "default"

[library]
scan_dirs = ["~/Music"]

[ui]
theme = "default"
```

## 插件开发

详见 [PLUGINS.md](docs/PLUGINS.md)

## 许可证

MIT © ZhX589
