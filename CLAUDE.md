# CLAUDE.md

本项目是 **claude-code-cleaner** 的 Windows 兼容版本。

## 项目简介

一个交互式终端 UI（TUI）工具，用于清理 Claude Code 长期运行后积累的 `~/.claude/` 目录占用空间（可达 1.8GB+、10000+ 文件）。

提供 5 步引导式工作流：扫描 → 选择 → 项目 → 预览 → 清理。覆盖 14 个可清理分类（Projects、Debug Logs、File History、Telemetry 等），支持干跑模式、过期过滤、孤立项目检测、偏好设置持久化。

## 技术栈

- **语言**: Rust (edition 2021)
- **核心依赖**: ratatui 0.29 (TUI)、crossterm 0.28 (终端控制)、tokio (异步)、walkdir (目录遍历)、dirs (跨平台用户目录)
- **构建**: `cargo build --release`
- **目标平台**: Linux x64/arm64、macOS x64/arm64、Windows x64

## 项目结构

```
src/
  main.rs             # 入口，终端初始化/清理，事件循环，键盘处理
  app.rs              # 应用状态机，5个 Screen 枚举，偏好设置持久化
  event.rs            # 事件循环：crossterm 输入轮询 + tick + 异步通道
  model/              # 数据类型定义
    category.rs       # 分类枚举、CategoryInfo、受保护路径列表
    project.rs        # 项目信息、过期过滤、目录名编码/解码
    scan_result.rs    # 扫描结果、可回收/可匹配空间计算
    config_json.rs    # 配置文件分析（可安全移除的字段）
    settings.rs       # 清理设置 + 用户偏好持久化
    clean_plan.rs     # 清理计划类型
  scanner/            # 扫描逻辑
    mod.rs            # 异步扫描分发
    categories.rs     # 按分类扫描 + 配置文件分析
    projects.rs       # 项目扫描 + 孤立项目检测
  cleaner/            # 清理执行
    mod.rs            # 异步清理执行器，进度报告
  ui/                 # 渲染层
    mod.rs            # 渲染分发
    dashboard.rs      # 扫描概览屏
    categories.rs     # 统一选择屏（分类+配置+设置）
    projects.rs       # 项目浏览器
    preview.rs        # 清理预览（三段色条）
    cleaning.rs       # 清理进度
    widgets/          # 共享 UI 辅助函数
```

## 关键设计决策

1. **纯跨平台代码**：不使用任何 `#[cfg(unix)]` / `#[cfg(windows)]`，所有路径通过 `dirs::home_dir()` 获取，依赖均为跨平台 crate
2. **安全优先**：受保护路径（settings.json、CLAUDE.md、skills/ 等）永不删除；History 仅修剪保留最后 500 行；Config JSON 原子写入
3. **干跑模式**：默认可开启，预览将删除内容而不实际触碰文件
4. **偏好持久化**：用户选择保存到 `~/.claude/cleaner-preferences.json`，下次启动自动恢复

## Windows 适配（本 fork 新增）

- `install.sh`：新增 Windows 平台检测（MINGW/MSYS/CYGWIN/WSL），使用 `.zip` 包格式，安装到 `%LOCALAPPDATA%\claude-code-cleaner\bin\`
- `.github/workflows/release.yml`：新增 `x86_64-pc-windows-msvc` 构建目标和 CI 矩阵
- `.github/workflows/ci.yml`：新增 `windows-latest` 测试
- `README.md`：中英文双语文档，底部注明二次开发来源

## 常用命令

```bash
# 编译
cargo build --release

# 检查
cargo check
cargo clippy -- -D warnings
cargo fmt --all -- --check

# 运行
cargo run --release

# 安装
cargo install claude-code-cleaner
```

## GitHub 仓库

- 地址：https://github.com/financia0x00001/claude-code-cleaner
- 原始项目：https://github.com/GarrickZ2/claude-code-cleaner（感谢原作者 GarrickZ2）
- License：MIT
