# Cmmuu Music

[English](README.md) | [简体中文](docs/README.zh-CN.md) | [日本語](docs/README.ja.md) | [한국어](docs/README.ko.md)

Cmmuu Music is a cross-platform desktop music player built with Tauri v2, Vue 3, TypeScript, Pinia, and Vite. It provides a lightweight desktop shell, a Rust backend, and a plugin-oriented architecture for compliant music data access.

> This project does not provide, host, or distribute music source files. Users are responsible for using legal and compliant plugins and content sources.

## Features

- Cross-platform desktop support for Linux, macOS, and Windows
- Tauri v2 backend with a Vue 3 frontend
- TypeScript-based frontend development
- Pinia state management and Vue Router routing
- Plugin-oriented architecture for music data access
- Local development, testing, and desktop bundling scripts

## Tech Stack

- Desktop runtime: Tauri v2
- Backend: Rust
- Frontend: Vue 3 + TypeScript
- Build tool: Vite
- State management: Pinia
- Router: Vue Router
- Package manager: pnpm
- Testing: Vitest

## Requirements

Install the required toolchains before running the project:

- Node.js LTS
- pnpm 10+
- Rust stable toolchain
- Tauri v2 system dependencies for your operating system

For platform-specific Tauri prerequisites, follow the official Tauri v2 setup guide for your OS.

## Getting Started

Clone the repository and install dependencies:

```bash
git clone https://github.com/CMMUU/CmmuuMusic.git
cd CmmuuMusic
pnpm install
```

Start the frontend-only development server:

```bash
pnpm dev
```

Start the Tauri desktop development app:

```bash
pnpm tauri:dev
```

Run tests:

```bash
pnpm test
```

Build the frontend:

```bash
pnpm build
```

Build the desktop app:

```bash
pnpm tauri:build
```

## Linux Usage

1. Install Node.js LTS, pnpm, Rust, and the Linux packages required by Tauri v2.
2. Run `pnpm install` in the repository root.
3. Use `pnpm tauri:dev` for local desktop development.
4. Use `pnpm tauri:build` to generate Linux desktop bundles.

The generated packages are produced by Tauri under `src-tauri/target/release/bundle/`.

## macOS Usage

1. Install Node.js LTS, pnpm, Rust, and Xcode Command Line Tools.
2. Run `pnpm install` in the repository root.
3. Use `pnpm tauri:dev` for local desktop development.
4. Use `pnpm tauri:build` to generate macOS bundles.

The generated application bundles are produced by Tauri under `src-tauri/target/release/bundle/`.

## Windows Usage

1. Install Node.js LTS, pnpm, Rust, and the Windows build tools required by Tauri v2.
2. Run `pnpm install` in the repository root.
3. Use `pnpm tauri:dev` for local desktop development.
4. Use `pnpm tauri:build` to generate Windows bundles.

The generated installers and packages are produced by Tauri under `src-tauri/target/release/bundle/`.

## Common Commands

| Command | Description |
| --- | --- |
| `pnpm dev` | Start the Vite frontend development server |
| `pnpm build` | Type-check and build the frontend |
| `pnpm preview` | Preview the built frontend |
| `pnpm tauri:dev` | Start the Tauri desktop development app |
| `pnpm tauri:build` | Build desktop bundles with Tauri |
| `pnpm test` | Run the Vitest test suite |

## Documentation

- [Software Design Document](CmmuuMusic_TauriV2_SDD.md)
- [简体中文使用文档](docs/README.zh-CN.md)
- [日本語ドキュメント](docs/README.ja.md)
- [한국어 문서](docs/README.ko.md)

## License

See [LICENSE](LICENSE).
