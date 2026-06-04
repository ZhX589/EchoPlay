# EchoPlay 开发路线图

## 阶段 A：基础类型与模型

- [x] Step 0: 创建 STEPS.md 路线图
- [x] Step 1: `Cargo.toml` 添加依赖
- [x] Step 2: `src/error.rs` 错误类型
- [x] Step 3: `src/model/base.rs` 枚举类型 (ModelType, SearchType)
- [x] Step 4: `src/media.rs` 音质和媒体类型
- [x] Step 5: `src/model/` 模型结构体 (Song, Album, Artist, Playlist, Lyric, SearchResult)
- [x] Step 6: `src/model/uri.rs` URI 解析与格式化
- [x] Step 7: `src/model/mod.rs` 模块导出

## 阶段 B：Provider 抽象层

- [x] Step 8: `src/provider/mod.rs` Provider trait
- [x] Step 9: `src/provider/capabilities.rs` 能力 trait (SongGet, SongMultiQuality, Search 等)
- [x] Step 10: `src/provider/registry.rs` ProviderRegistry 注册表
- [x] Step 11: `src/standby.rs` 跨源备用匹配评分
- [x] Step 12: `src/library.rs` Library 门面

## 阶段 C：QQ 音乐 Provider

- [x] Step 13: `src/providers/mod.rs` + `src/providers/qq/mod.rs` 模块骨架
- [x] Step 14: `src/providers/qq/model.rs` QQ 音乐 JSON 响应结构体
- [x] Step 15: `src/providers/qq/api.rs` HTTP 客户端
- [x] Step 16: `src/providers/qq/provider.rs` QQProvider 实现
- [x] Step 17: `src/main.rs` 串联验证

## 阶段 D：测试

- [x] Step 18: `tests/common/mod.rs` MockProvider 离线测试
- [x] Step 19: `tests/library_integration.rs` 集成测试 (18 cases)

## 阶段 E：音频播放引擎

- [x] Step 20: `Cargo.toml` 添加 rodio/symphonia/clap/toml/dirs 依赖
- [x] Step 21: `src/player/mod.rs` Player 结构体 (play/pause/resume/stop/set_volume)
- [x] Step 22: `src/player/state.rs` PlayerState/PlayerInfo
- [x] Step 23: `src/player/playlist.rs` Playlist + PlayMode (顺序/单曲循环/随机/列表循环)
- [x] Step 24: `src/config.rs` Config 结构体 (audio/library/ui) + TOML 序列化

## 阶段 F：本地音乐 Provider

- [x] Step 25: `Cargo.toml` 添加 lofty + tempfile 依赖
- [x] Step 26: `src/providers/local/mod.rs` 模块骨架
- [x] Step 27: `src/providers/local/scanner.rs` DirScanner 递归扫描目录
- [x] Step 28: `src/providers/local/tag_reader.rs` TagReader 读取音频元数据
- [x] Step 29: `src/providers/local/provider.rs` LocalProvider 实现 Provider trait

## 阶段 G：TUI 框架

- [x] Step 30: `Cargo.toml` 添加 ratatui + crossterm 依赖
- [x] Step 31: `src/tui/mod.rs` 终端初始化/恢复
- [x] Step 32: `src/tui/event.rs` 事件处理 (键盘映射)
- [x] Step 33: `src/tui/app.rs` App 结构体 (Library + Player + 视图状态)
- [x] Step 34: `src/tui/ui.rs` 绘制函数 (header/main/now_playing/search)

## 阶段 H：配置系统

- [x] Step 35: `src/config.rs` Config 加载/保存 + 默认值

## 阶段 I：串联与 CLI

- [x] Step 36: `src/main.rs` clap CLI 参数解析 (--play 目录, --search 搜索)
- [x] Step 37: 加载 Config → 初始化 Library (LocalProvider + QQProvider) → Player → TUI
- [x] Step 38: TUI 事件循环 (键盘→App命令→Player/Library操作)

## 阶段 K：TUI 样式优化

- [x] Step 42: 替换 Emoji 为 Nerd Font 图标 (FontAwesome set)
- [x] Step 43: Theme 结构体 (5色+边框+进度条+歌词+封面样式)
- [x] Step 44: widgets (progress_bar/controls/cover_art/lyrics)
- [x] Step 45: views (playback Xero风格 / library列表 / search搜索)
- [x] Step 46: Tab 栏 (默认隐藏, Tab展开, 1/2/3切换)
- [x] Step 47: InputMode (Normal/Search) 分离事件路由
- [x] Step 48: 11轮迭代修复20+个bug
- [x] Step 49: 最终测试 (56 unit + 18 integration = 74 tests)
