# Cmmuu Music 利用ドキュメント

[English](../README.md) | [简体中文](README.zh-CN.md) | [日本語](README.ja.md) | [한국어](README.ko.md)

Cmmuu Music は、Tauri v2、Vue 3、TypeScript、Pinia、Vite を使用して構築されたクロスプラットフォームのデスクトップ音楽プレーヤーです。軽量なデスクトップシェル、Rust バックエンド、そしてプラグイン指向の音楽データアクセス構成を提供します。

> このプロジェクトは音楽ソースファイルを提供、ホスト、配布しません。ユーザーは、合法かつ適切なプラグインとコンテンツソースを利用する責任があります。

## 主な機能

- Linux、macOS、Windows のデスクトップ環境をサポート
- Tauri v2 バックエンドと Vue 3 フロントエンド
- TypeScript によるフロントエンド開発
- Pinia による状態管理と Vue Router によるルーティング
- プラグイン指向の音楽データアクセス構成
- ローカル開発、テスト、デスクトップアプリのビルド用スクリプト

## 技術スタック

- デスクトップランタイム: Tauri v2
- バックエンド: Rust
- フロントエンド: Vue 3 + TypeScript
- ビルドツール: Vite
- 状態管理: Pinia
- ルーター: Vue Router
- パッケージマネージャー: pnpm
- テスト: Vitest

## 必要環境

プロジェクトを実行する前に、以下のツールチェーンをインストールしてください。

- Node.js LTS
- pnpm 10+
- Rust stable toolchain
- 利用する OS に対応した Tauri v2 のシステム依存関係

OS ごとの Tauri 依存関係については、Tauri v2 の公式セットアップガイドに従ってください。

## はじめに

リポジトリをクローンし、依存関係をインストールします。

```bash
git clone https://github.com/CMMUU/CmmuuMusic.git
cd CmmuuMusic
pnpm install
```

フロントエンドのみの開発サーバーを起動します。

```bash
pnpm dev
```

Tauri デスクトップ開発アプリを起動します。

```bash
pnpm tauri:dev
```

テストを実行します。

```bash
pnpm test
```

フロントエンドをビルドします。

```bash
pnpm build
```

デスクトップアプリをビルドします。

```bash
pnpm tauri:build
```

## Linux での利用

1. Node.js LTS、pnpm、Rust、Linux 向けの Tauri v2 必須パッケージをインストールします。
2. リポジトリのルートで `pnpm install` を実行します。
3. `pnpm tauri:dev` でローカルのデスクトップ開発アプリを起動します。
4. `pnpm tauri:build` で Linux 向けデスクトップバンドルを生成します。

生成されたパッケージは通常 `src-tauri/target/release/bundle/` に出力されます。

## macOS での利用

1. Node.js LTS、pnpm、Rust、Xcode Command Line Tools をインストールします。
2. リポジトリのルートで `pnpm install` を実行します。
3. `pnpm tauri:dev` でローカルのデスクトップ開発アプリを起動します。
4. `pnpm tauri:build` で macOS 向けアプリバンドルを生成します。

生成されたアプリバンドルは通常 `src-tauri/target/release/bundle/` に出力されます。

## Windows での利用

1. Node.js LTS、pnpm、Rust、Windows 向けの Tauri v2 ビルドツールをインストールします。
2. リポジトリのルートで `pnpm install` を実行します。
3. `pnpm tauri:dev` でローカルのデスクトップ開発アプリを起動します。
4. `pnpm tauri:build` で Windows 向けインストーラーまたはパッケージを生成します。

生成されたインストーラーやパッケージは通常 `src-tauri/target/release/bundle/` に出力されます。

## よく使うコマンド

| コマンド | 説明 |
| --- | --- |
| `pnpm dev` | Vite フロントエンド開発サーバーを起動 |
| `pnpm build` | 型チェックを行い、フロントエンドをビルド |
| `pnpm preview` | ビルド済みフロントエンドをプレビュー |
| `pnpm tauri:dev` | Tauri デスクトップ開発アプリを起動 |
| `pnpm tauri:build` | Tauri でデスクトップバンドルをビルド |
| `pnpm test` | Vitest テストを実行 |

## 関連ドキュメント

- [ソフトウェア設計ドキュメント](../CmmuuMusic_TauriV2_SDD.md)
- [English README](../README.md)
- [简体中文文档](README.zh-CN.md)
- [한국어 문서](README.ko.md)

## ライセンス

[LICENSE](../LICENSE) を参照してください。
