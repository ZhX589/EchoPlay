# EchoPlay 项目 AI 协作指南

## 项目简介

EchoPlay 是一个 Rust 编写的终端音乐播放器，采用插件化架构，支持本地音乐和网络音源（QQ音乐、网易云等）。

## 技术栈

- 语言：Rust (latest stable)
- TUI：ratatui + crossterm
- 音频：rodio (起步) → symphonia + cpal (进阶)
- 异步：tokio
- 插件：libloading + 或 RPC

## 开发原则

1. **小步迭代**：每次只实现一个最小功能点
2. **可运行优先**：每一步完成后必须能编译运行
3. **测试驱动**：核心逻辑应有单元测试
4. **文档同步**：代码改动同步更新注释

## 代码规范

- 使用 `cargo fmt` 格式化
- 使用 `cargo clippy` 检查
- 公共 API 必须有文档注释
- 错误使用 `thiserror` + `anyhow`

## 提交规范

```

feat: 添加xxx功能
fix: 修复xxx问题
docs: 更新文档
refactor: 重构xxx
test: 添加xxx测试

```

## 当前阶段

请查看 `STEPS.md` 了解当前进度，每次只完成一个 Step，完成后等待人工验证。

## 回答格式

完成每个 Step 时，输出：

1. 新增/修改的文件列表
2. 关键代码片段（核心逻辑）
3. 如何验证功能
4. 下一步预告
