# Cmmuu Music 使用文档

[English](../README.md) | [简体中文](README.zh-CN.md) | [日本語](README.ja.md) | [한국어](README.ko.md)

Cmmuu Music 是一款基于 Tauri v2、Vue 3、TypeScript、Pinia 和 Vite 构建的跨平台桌面音乐播放器。项目提供轻量级桌面壳、Rust 后端能力，以及面向插件的音乐数据访问架构。

> 本项目不提供、托管或分发任何音乐源文件。用户需要自行确保所使用的插件与内容来源合法合规。

## 功能特性

- 支持 Linux、macOS、Windows 桌面平台
- 基于 Tauri v2 后端与 Vue 3 前端
- 使用 TypeScript 开发前端
- 使用 Pinia 管理状态，使用 Vue Router 管理路由
- 面向插件的音乐数据访问架构
- 支持本地开发、测试和桌面应用打包命令

## 技术栈

- 桌面运行时：Tauri v2
- 后端：Rust
- 前端：Vue 3 + TypeScript
- 构建工具：Vite
- 状态管理：Pinia
- 路由：Vue Router
- 包管理器：pnpm
- 测试：Vitest

## 环境要求

运行项目之前，请先安装以下工具链：

- Node.js LTS
- pnpm 10+
- Rust stable 工具链
- 当前操作系统所需的 Tauri v2 系统依赖

不同平台的 Tauri 依赖略有差异，请按照 Tauri v2 官方安装指南配置对应系统环境。

## 快速开始

克隆仓库并安装依赖：

```bash
git clone https://github.com/CMMUU/CmmuuMusic.git
cd CmmuuMusic
pnpm install
```

启动仅前端开发服务：

```bash
pnpm dev
```

启动 Tauri 桌面开发应用：

```bash
pnpm tauri:dev
```

运行测试：

```bash
pnpm test
```

构建前端：

```bash
pnpm build
```

构建桌面应用：

```bash
pnpm tauri:build
```

## Linux 使用说明

1. 安装 Node.js LTS、pnpm、Rust，以及 Tauri v2 在 Linux 上所需的系统包。
2. 在仓库根目录执行 `pnpm install`。
3. 使用 `pnpm tauri:dev` 启动本地桌面开发应用。
4. 使用 `pnpm tauri:build` 生成 Linux 桌面安装包或应用包。

Tauri 生成的打包产物通常位于 `src-tauri/target/release/bundle/`。

## macOS 使用说明

1. 安装 Node.js LTS、pnpm、Rust 和 Xcode Command Line Tools。
2. 在仓库根目录执行 `pnpm install`。
3. 使用 `pnpm tauri:dev` 启动本地桌面开发应用。
4. 使用 `pnpm tauri:build` 生成 macOS 应用包。

Tauri 生成的应用包通常位于 `src-tauri/target/release/bundle/`。

## Windows 使用说明

1. 安装 Node.js LTS、pnpm、Rust，以及 Tauri v2 在 Windows 上所需的构建工具。
2. 在仓库根目录执行 `pnpm install`。
3. 使用 `pnpm tauri:dev` 启动本地桌面开发应用。
4. 使用 `pnpm tauri:build` 生成 Windows 安装包或应用包。

Tauri 生成的安装包通常位于 `src-tauri/target/release/bundle/`。

## 常用命令

| 命令 | 说明 |
| --- | --- |
| `pnpm dev` | 启动 Vite 前端开发服务 |
| `pnpm build` | 类型检查并构建前端 |
| `pnpm preview` | 预览前端构建产物 |
| `pnpm tauri:dev` | 启动 Tauri 桌面开发应用 |
| `pnpm tauri:build` | 使用 Tauri 构建桌面应用包 |
| `pnpm test` | 运行 Vitest 测试 |

## 相关文档

- [软件设计文档](../CmmuuMusic_TauriV2_SDD.md)
- [English README](../README.md)
- [日本語ドキュメント](README.ja.md)
- [한국어 문서](README.ko.md)

## 许可证

请查看 [LICENSE](../LICENSE)。
