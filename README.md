# Claude Code Cleaner

<div align="center">

![Rust](https://img.shields.io/badge/rust-1.70+-orange)
![License](https://img.shields.io/badge/license-MIT-blue)
![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey)

> 🧹 **交互式终端清理工具，释放 Claude Code 占用的磁盘空间**

> **Interactive TUI tool to reclaim disk space from Claude Code**

Claude Code 长期使用后，`~/.claude/` 目录可能膨胀到 **1.8 GB+**、**10000+ 文件** —— 包含旧会话数据、调试日志、遥测数据、孤立的项目缓存等。本工具帮助你直观地查看这些内容，选择要清理的部分，安全地释放磁盘空间。

[English](#english) · [功能](#features) · [安装](#installation) · [使用](#usage)

</div>

## 功能 Features

- **5 步引导式工作流**：扫描 → 选择 → 项目 → 预览 → 清理
- **智能扫描**：检测所有可清理的数据分类，按文件年龄追踪
- **孤立项目检测**：自动识别原始路径已不存在的项目缓存
- **过期过滤**：仅清理超过设定天数（默认 30 天）的文件
- **精准清理 `.claude.json`**：移除孤立的条目、会话指标和缓存，不删除原文件
- **三段式预览栏**：绿色=将清理，黄色=匹配但未选，红色=保留
- **实时进度条**：清理过程中实时显示各分类进度
- **偏好设置持久化**：你的选择会在下次启动时自动恢复
- **干跑模式**：预览将删除的内容，不实际触碰任何文件
- **受保护路径**：`settings.json`、`CLAUDE.md`、`skills/` 等关键文件永不删除

## 可清理分类 Cleanable Categories

| 分类 | 目录/文件 | 说明 |
|---|---|---|
| Projects | `projects/` | 项目会话数据（通常占用最大） |
| Debug Logs | `debug/` | 调试日志文件 |
| File History | `file-history/` | 文件版本快照 |
| Telemetry | `telemetry/` | 遥测数据 |
| Shell Snapshots | `shell-snapshots/` | Shell 环境快照 |
| Plugins | `plugins/` | 插件缓存 |
| Transcripts | `transcripts/` | 旧会话记录 |
| Todos | `todos/` | 待办事项 |
| Plans | `plans/` | 计划文档 |
| Usage Data | `usage-data/` | 使用分析数据 |
| Tasks | `tasks/` | 任务管理数据 |
| Paste Cache | `paste-cache/` | 剪贴板缓存 |
| Config Backups | `~/.claude.json.backup*` | 配置文件备份 |
| History | `history.jsonl` | 命令历史（仅修剪，不删除） |
| Config JSON | `~/.claude.json` | 精准字段清理 |

## 安装 Installation

### 快速安装（Linux / macOS / Windows）

```bash
curl -fsSL https://raw.githubusercontent.com/financia0x00001/claude-code-cleaner/master/install.sh | bash
```

指定版本安装：

```bash
curl -fsSL https://raw.githubusercontent.com/financia0x00001/claude-code-cleaner/master/install.sh | bash -s v0.1.2
```

Windows (Git Bash / WSL) 安装到 `%LOCALAPPDATA%\claude-code-cleaner\bin\`。

### 通过 cargo

```bash
cargo install claude-code-cleaner
```

### 从 Releases 下载

从 [Releases](https://github.com/financia0x00001/claude-code-cleaner/releases) 下载对应平台的二进制文件。

### 从源码编译

```bash
git clone https://github.com/financia0x00001/claude-code-cleaner.git
cd claude-code-cleaner
cargo build --release
```

## 使用 Usage

```bash
claude-code-cleaner
```

启动后自动扫描 `~/.claude/` 目录，进入全屏 TUI 界面。

### 5 个屏幕

1. **仪表盘（扫描）** — 显示总大小、文件数、按分类统计
2. **选择** — 勾选要清理的分类，调整过期天数，开启干跑模式
3. **项目** — 浏览所有项目，区分孤立项目和活跃项目
4. **预览** — 可视化清理计划（绿=将清理，黄=匹配未选，红=保留）
5. **清理** — 执行清理，实时进度条，显示释放空间

### 键盘快捷键

| 按键 | 功能 |
|---|---|
| `1`-`5` | 跳转到指定屏幕 |
| `Tab` / `Shift-Tab` | 上/下一步 |
| `j`/`k` 或 `↑`/`↓` | 列表导航 |
| `Space` | 选择/取消 |
| `Enter` | 确认/继续 |
| `Esc` | 返回 |
| `a` | 全选 |
| `n` | 全不选 |
| `o` | 只选孤立项目 |
| `/` | 搜索过滤 |
| `s` | 重新扫描 |
| `?` | 帮助 |
| `q` | 退出 |

## 安全性 Safety

- **受保护路径** 永不修改或删除
- **命令历史** 仅修剪（保留最后 500 行），不删除
- **配置文件** 以原子写入方式清理
- **活跃项目** 仅删除过期文件，保留目录和近期文件
- **干跑模式** 预览将删除的内容，不实际触碰文件
- 失败删除会被收集报告，不会中断进程

## 系统要求 Requirements

- Rust 1.70+（从源码编译时需要）
- 支持 Unicode 的终端
- **Windows**：Git Bash（运行 install.sh）或 WSL；也可用 `cargo install` 或从 Releases 下载

---

<a name="english"></a>

## English

### Overview

Claude Code Cleaner is an interactive terminal UI (TUI) tool designed to reclaim disk space consumed by Claude Code's `~/.claude/` directory. After extended usage, this directory can accumulate over **1.8 GB** across **10,000+ files**, including old session data, debug logs, telemetry, orphaned project caches, and more.

### What it does

The tool provides a guided 5-screen workflow that helps you:
1. **Scan** — Automatically analyze the `~/.claude/` directory and categorize files by size and age
2. **Select** — Choose which categories to clean, adjust the expiry threshold, and enable dry-run mode
3. **Projects** — Browse detected projects, identify orphans (projects whose original paths no longer exist)
4. **Preview** — Review the full clean plan with a visual 3-segment progress bar
5. **Clean** — Execute the plan with real-time progress tracking

### Why this fork

This is a **Windows-compatible fork** of the original [claude-code-cleaner](https://github.com/GarrickZ2/claude-code-cleaner) by [GarrickZ2](https://github.com/GarrickZ2). The original project only supported Linux and macOS. This fork adds:

- **Full Windows support** — Git Bash, WSL, and native Windows terminal compatibility
- **Windows installer** — `install.sh` now detects Windows and handles `.zip` packages
- **CI/CD** — GitHub Actions builds and publishes Windows binaries (`.exe`) to Releases
- **Chinese documentation** — Bilingual README with Chinese instructions

### Big thanks to the original author

> 🙏 **Special thanks to [GarrickZ2](https://github.com/GarrickZ2) for the excellent original project.**
>
> The core design, TUI implementation, scanning logic, and safety mechanisms were all created by the original author. This fork only adds Windows compatibility and bilingual documentation on top of their outstanding work.

## License

MIT — See [LICENSE](LICENSE) file for details.

> This project is a derivative work based on [claude-code-cleaner](https://github.com/GarrickZ2/claude-code-cleaner) by GarrickZ2, licensed under the MIT License.
