# Cmmuu Music   Tauri v2 版软件设计文档 (SDD)


---

## 目录

1. [文档概述](#1-文档概述)
2. [架构总览](#2-架构总览)
3. [技术栈选型](#3-技术栈选型)
4. [系统架构设计](#4-系统架构设计)
5. [核心模块设计](#5-核心模块设计)
6. [插件系统设计](#6-插件系统设计)
7. [音频管线设计](#7-音频管线设计)
8. [数据层设计](#8-数据层设计)
9. [安全模型](#9-安全模型)
10. [性能优化策略](#10-性能优化策略)
11. [UI/UX 设计规范](#11-uiux-设计规范)
12. [测试策略](#12-测试策略)
13. [构建与部署](#13-构建与部署)
14. [Electron → Tauri 迁移对照](#14-electron--tauri-迁移对照)
15. [风险与缓解](#15-风险与缓解)
16. [附录](#16-附录)

---

## 1. 文档概述

### 1.1 项目背景

Cmmuu Music 是一款跨平台桌面音乐播放器框架，现基于 **Electron + Vue 3** 开发。项目不直接存储/提供音乐源文件，仅提供插件运行框架与播放功能，用户通过合规插件获取音乐数据。

**核心痛点驱动重写：**
- Electron 内存占用大（通常 250MB+），启动慢
- Node.js 运行时开销高，音频处理性能受限
- 插件沙箱基于 Node.js VM，隔离性不够强
- 安装包体积大（macOS 292MB）

### 1.2 重写目标

| 维度 | 目标 |
|------|------|
| **包体积** | macOS < 30MB，Windows < 25MB，Linux < 20MB |
| **内存占用** | 空闲 < 80MB，播放中 < 150MB |
| **启动速度** | 冷启动 < 1.5s |
| **音频性能** | 零拷贝音频管线，延迟 < 10ms |
| **插件安全** | 基于 Deno/QuickJS 的进程级隔离沙箱 |
| **跨平台** | macOS（ARM64/x64）、Windows（x64/ARM64）、Linux（x64） |
| **兼容性** | 支持现有 LX Music 插件生态（事件驱动模式） |

### 1.3 术语定义

| 术语 | 定义 |
|------|------|
| **CmmuuPlugin** | 本项目的原生插件格式，基于方法导出 |
| **LX Plugin** | 洛雪音乐兼容插件格式，基于事件驱动 |
| **音源 (Source)** | 音乐数据提供方标识，如 `kw`、`wy`、`tx` |
| **宿主 (Host)** | Tauri 后端 Rust 进程，负责插件生命周期管理 |
| **沙箱 (Sandbox)** | 插件执行的隔离环境 |
| **音频管线 (Audio Pipeline)** | 从获取音源 URL 到扬声器输出的完整链路 |

---

## 2. 架构总览

### 2.1 四层架构

```
┌────────────────────────────────────────────────────┐
│                 Presentation Layer                  │
│        Vue 3 + TypeScript + Pinia + Vite           │
│   UI Components / Router / State Management        │
├────────────────────────────────────────────────────┤
│                 Bridge Layer (Tauri v2)             │
│    Commands / Events / IPC / Permissions            │
├────────────────────────────────────────────────────┤
│                  Core Layer (Rust)                  │
│  ┌──────────┬──────────┬──────────┬──────────┐    │
│  │ Plugin   │  Audio   │  Cache   │  Config  │    │
│  │ Manager  │  Engine  │  Manager │  Manager │    │
│  ├──────────┼──────────┼──────────┼──────────┤    │
│  │ Search   │ Playlist │  Lyrics  │ Metadata │    │
│  │ Engine   │ Manager  │  Engine  │ Fetcher  │    │
│  └──────────┴──────────┴──────────┴──────────┘    │
├────────────────────────────────────────────────────┤
│                 Sandbox Layer                       │
│    Plugin Runtime (Deno/QuickJS)                   │
│    Process Isolation / Resource Limits              │
└────────────────────────────────────────────────────┘
```

### 2.2 进程模型

```
Main Process (Rust/Tauri)
├── Audio Thread (real-time, high priority)
├── Plugin Manager Thread
├── Download Thread Pool (tokio)
├── Cache I/O Thread Pool
└── WebView (Vue 3 frontend, GPU accelerated)

Sandbox Process (per-plugin)
├── Plugin Executor (Deno/QuickJS)
├── IPC Bridge → Main Process
└── Resource Limiter (CPU/Memory/Network)
```

**对比 Electron 的优势**：Electron 每个 WebView 都是完整 Chromium 实例，Tauri 使用系统原生 WebView（macOS WKWebView ~15MB，Windows WebView2 ~30MB 共享），插件不再跑在主进程的 Node.js VM 中，而是独立沙箱进程。

---

## 3. 技术栈选型

### 3.1 核心技术栈

| 层级 | 技术 | 版本 | 选型理由 |
|------|------|------|----------|
| **桌面框架** | Tauri | v2.x | 原生性能、小体积、Rust 安全性 |
| **后端语言** | Rust | 1.80+ (Edition 2024) | 零成本抽象、内存安全、高性能音频处理 |
| **前端框架** | Vue 3 | 3.5+ | 保持原项目生态、Composition API |
| **前端语言** | TypeScript | 5.5+ | 类型安全 |
| **构建工具** | Vite | 6.x | 极速 HMR |
| **状态管理** | Pinia | 2.x | Vue 3 官方推荐 |
| **异步运行时** | Tokio | 1.x | Rust 异步标准 |
| **音频引擎** | Symphonia + cpal | 0.5 / 0.15 | 纯 Rust 解码 + 跨平台音频输出 |
| **HTTP 客户端** | Reqwest | 0.12 | 异步 HTTP、TLS、连接池 |
| **序列化** | Serde | 1.x | Rust 事实标准 |
| **数据库** | SQLite (rusqlite) | 0.32 | 嵌入式、零配置 |
| **插件沙箱** | Deno Core / QuickJS | - | 安全 JS 执行环境 |

### 3.2 前端依赖（保持与原项目一致）

| 库 | 用途 |
|----|------|
| `vue-router` | 路由管理 |
| `pinia` | 状态管理 |
| `@tauri-apps/api` | Tauri 前端桥接 |
| `@vueuse/core` | 组合式工具 |
| `applemusic-like-lyrics` | 歌词组件（AGPL-3.0，保留） |

### 3.3 Rust Crate 依赖

```toml
[dependencies]
tauri = { version = "2", features = ["image-png", "image-ico"] }
tauri-plugin-shell = "2"
tauri-plugin-dialog = "2"
tauri-plugin-fs = "2"
tauri-plugin-notification = "2"
tauri-plugin-updater = "2"
tauri-plugin-process = "2"

# Async runtime
tokio = { version = "1", features = ["full"] }

# Audio
symphonia = { version = "0.5", features = ["all"] }
cpal = "0.15"
rubato = "1"  # sample rate conversion

# HTTP
reqwest = { version = "0.12", features = ["json", "stream", "native-tls-vendored"] }

# DB
rusqlite = { version = "0.32", features = ["bundled", "serde"] }

# Sandbox
deno_core = "0.300"  # or quickjs-rs for lighter weight

# Serialization
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# Crypto (for plugins)
md-5 = "0.10"
aes = "0.8"
rsa = "0.9"
base64 = "0.22"

# Other
uuid = { version = "1", features = ["v4"] }
chrono = { version = "0.4", features = ["serde"] }
log = "0.4"
env_logger = "0.11"
parking_lot = "0.12"  # faster mutex
thiserror = "2"
```

---

## 4. 系统架构设计

### 4.1 模块依赖关系图

```
┌─────────────────────────────────────────────┐
│                  App Entry                   │
│              (main.rs / lib.rs)              │
└──────────────────┬──────────────────────────┘
                   │
    ┌──────────────┼──────────────┐
    ▼              ▼              ▼
┌────────┐  ┌────────────┐  ┌──────────┐
│ Tauri  │  │   Config   │  │ Database │
│ Setup  │  │   Manager  │  │  Manager │
└───┬────┘  └────────────┘  └────┬─────┘
    │                            │
    ├────────────────────────────┤
    │                            │
    ▼              ▼             ▼          ▼
┌────────┐  ┌──────────┐  ┌──────────┐ ┌───────┐
│ Plugin │  │  Audio   │  │ Playlist │ │ Cache │
│Manager │  │  Engine  │  │ Manager  │ │  Mgr  │
└───┬────┘  └────┬─────┘  └────┬─────┘ └───┬───┘
    │            │              │           │
    ▼            ▼              ▼           ▼
┌────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐
│Sandbox │ │ Symphonia│ │  SQLite  │ │  File    │
│Runtime │ │  + cpal  │ │  Store   │ │  System  │
└────────┘ └──────────┘ └──────────┘ └──────────┘
```

### 4.2 IPC 通信设计

Tauri v2 通过 `#[tauri::command]` 宏定义前后端通信接口。

**通信模式：**

| 方向 | 机制 | 场景 |
|------|------|------|
| 前端 → 后端 | `invoke("cmd", args)` | 用户操作触发 |
| 后端 → 前端 | `window.emit("event", payload)` | 状态变更推送 |
| 流式 | Channel API | 下载进度、搜索分页 |
| 原生菜单 | Tauri Menu Event | 系统托盘、快捷键 |

### 4.3 目录结构

```
cmmuu-music-tauri/
├── src-tauri/                    # Rust 后端
│   ├── src/
│   │   ├── main.rs               # 入口
│   │   ├── lib.rs                # 库根，模块注册
│   │   ├── commands/             # Tauri 命令（IPC 接口）
│   │   │   ├── mod.rs
│   │   │   ├── player.rs         # 播放控制命令
│   │   │   ├── playlist.rs       # 播放列表命令
│   │   │   ├── plugin.rs         # 插件管理命令
│   │   │   ├── search.rs         # 搜索命令
│   │   │   ├── lyrics.rs         # 歌词命令
│   │   │   └── settings.rs       # 设置命令
│   │   ├── core/                 # 核心业务逻辑
│   │   │   ├── mod.rs
│   │   │   ├── audio/            # 音频引擎
│   │   │   │   ├── mod.rs
│   │   │   │   ├── engine.rs     # 音频引擎核心
│   │   │   │   ├── decoder.rs    # 解码器（Symphonia）
│   │   │   │   ├── pipeline.rs   # 音频管线
│   │   │   │   └── visualizer.rs # 可视化数据提取
│   │   │   ├── plugin/           # 插件系统
│   │   │   │   ├── mod.rs
│   │   │   │   ├── manager.rs    # 插件管理器
│   │   │   │   ├── sandbox.rs    # 沙箱运行时
│   │   │   │   ├── loader.rs     # 插件加载器
│   │   │   │   ├── converter.rs  # LX→Cmmuu 转换器
│   │   │   │   └── api.rs        # 宿主 API 实现
│   │   │   ├── playlist/         # 播放列表
│   │   │   │   ├── mod.rs
│   │   │   │   └── manager.rs
│   │   │   ├── search/           # 搜索聚合
│   │   │   │   ├── mod.rs
│   │   │   │   └── aggregator.rs
│   │   │   ├── lyrics/           # 歌词引擎
│   │   │   │   ├── mod.rs
│   │   │   │   ├── parser.rs     # LRC/KRC 解析
│   │   │   │   └── engine.rs
│   │   │   ├── cache/            # 缓存管理
│   │   │   │   ├── mod.rs
│   │   │   │   ├── music.rs      # 音乐缓存
│   │   │   │   └── metadata.rs   # 元数据缓存
│   │   │   ├── config/           # 配置管理
│   │   │   │   ├── mod.rs
│   │   │   │   └── manager.rs
│   │   │   └── db/               # 数据库
│   │   │       ├── mod.rs
│   │   │       ├── models.rs     # 数据模型
│   │   │       └── migrations.rs # 数据库迁移
│   │   ├── types/                # 共享类型
│   │   │   ├── mod.rs
│   │   │   ├── music.rs          # 歌曲/专辑/艺人
│   │   │   ├── playlist.rs       # 播放列表
│   │   │   ├── plugin.rs         # 插件相关
│   │   │   └── search.rs         # 搜索相关
│   │   └── utils/                # 工具函数
│   │       ├── mod.rs
│   │       ├── crypto.rs         # 加密工具（供插件使用）
│   │       ├── request.rs        # HTTP 请求封装
│   │       └── path.rs           # 路径处理
│   ├── Cargo.toml
│   ├── tauri.conf.json           # Tauri 配置
│   ├── capabilities/             # Tauri v2 权限声明
│   │   └── default.json
│   └── icons/                    # 应用图标
├── src/                          # Vue 3 前端
│   ├── components/
│   │   ├── player/               # 播放器组件
│   │   │   ├── AudioVisualizer.vue
│   │   │   ├── PlayerBar.vue
│   │   │   ├── PlayerControls.vue
│   │   │   ├── ProgressBar.vue
│   │   │   └── VolumeControl.vue
│   │   ├── playlist/             # 播放列表
│   │   │   ├── PlaylistPanel.vue
│   │   │   └── SongVirtualList.vue
│   │   ├── search/               # 搜索
│   │   │   └── SearchPanel.vue
│   │   ├── lyrics/               # 歌词
│   │   │   └── LyricsDisplay.vue
│   │   ├── settings/             # 设置
│   │   │   ├── PluginSettings.vue
│   │   │   ├── AudioSettings.vue
│   │   │   ├── ThemeSettings.vue
│   │   │   └── CacheSettings.vue
│   │   ├── layout/               # 布局
│   │   │   ├── MainLayout.vue
│   │   │   └── TitleBar.vue
│   │   └── common/               # 通用组件
│   │       ├── ContextMenu.vue
│   │       └── ThemeSelector.vue
│   ├── views/
│   │   ├── HomeView.vue
│   │   ├── SearchView.vue
│   │   ├── LocalView.vue
│   │   ├── RecentView.vue
│   │   └── SettingsView.vue
│   ├── stores/
│   │   ├── player.ts             # 播放状态
│   │   ├── playlist.ts           # 播放列表状态
│   │   ├── search.ts             # 搜索状态
│   │   ├── settings.ts           # 设置状态
│   │   └── plugin.ts             # 插件状态
│   ├── composables/
│   │   ├── useAudio.ts
│   │   ├── usePlaylist.ts
│   │   ├── useSearch.ts
│   │   └── useTheme.ts
│   ├── api/
│   │   └── commands.ts           # Tauri invoke 封装
│   ├── router/
│   │   └── index.ts
│   ├── types/
│   │   └── index.ts
│   ├── App.vue
│   └── main.ts
├── package.json
├── vite.config.ts
├── tsconfig.json
└── README.md
```

---

## 5. 核心模块设计

### 5.1 音频引擎 (Audio Engine)

这是迁移到 Rust 后**收益最大**的模块。

#### 5.1.1 架构

```
┌──────────────┐    ┌──────────────┐    ┌──────────────┐
│  URL Source  │───▶│   Decoder    │───▶│  Resampler   │
│ (HTTP/File)  │    │ (Symphonia)  │    │  (Rubato)    │
└──────────────┘    └──────────────┘    └──────┬───────┘
                                               │
                    ┌──────────────┐            │
   ┌───────────┐    │  Visualizer  │◀───────────┤
   │ Speakers  │◀───│ (FFT/Scope)  │            │
   └───────────┘    └──────────────┘    ┌───────┴──────┐
                                        │ Audio Output │
                                        │   (cpal)     │
                                        └──────────────┘
```

#### 5.1.2 核心接口

```rust
/// 音频引擎核心 trait
#[async_trait]
pub trait AudioEngine: Send + Sync {
    /// 加载并播放 URL
    async fn play_url(&self, url: &str) -> Result<(), AudioError>;
    
    /// 播放本地文件
    async fn play_file(&self, path: &Path) -> Result<(), AudioError>;
    
    /// 暂停/恢复
    fn toggle_pause(&self);
    
    /// 跳转到指定位置（秒）
    fn seek(&self, position_secs: f64) -> Result<(), AudioError>;
    
    /// 设置音量 (0.0 - 1.0)
    fn set_volume(&self, volume: f32);
    
    /// 获取当前播放进度
    fn position(&self) -> Duration;
    
    /// 获取总时长
    fn duration(&self) -> Option<Duration>;
    
    /// 获取频谱数据（用于可视化）
    fn spectrum_data(&self) -> Vec<f32>;
    
    /// 订阅播放状态变更事件
    fn on_state_change(&self, callback: Box<dyn Fn(PlaybackState) + Send>);
}

/// 播放状态
#[derive(Debug, Clone, PartialEq)]
pub enum PlaybackState {
    Idle,
    Loading,
    Playing,
    Paused,
    Ended,
    Error(String),
}

/// 解码器
pub struct SymphoniaDecoder {
    format: Box<dyn FormatReader>,
    decoder: Box<dyn Decoder>,
    track_id: u32,
    sample_rate: u32,
    channels: u32,
}

impl SymphoniaDecoder {
    /// 从 URL 流式解码
    pub async fn from_url(url: &str) -> Result<Self, AudioError>;
    
    /// 从文件解码
    pub fn from_file(path: &Path) -> Result<Self, AudioError>;
    
    /// 读取下一帧 PCM 数据
    pub fn next_frame(&mut self) -> Option<AudioFrame>;
}

/// PCM 音频帧
pub struct AudioFrame {
    pub data: Vec<f32>,
    pub sample_rate: u32,
    pub channels: u8,
}
```

#### 5.1.3 支持的音频格式

| 格式 | 通过 Symphonia 原生支持 |
|------|------------------------|
| MP3 | ✅ |
| FLAC | ✅ |
| AAC (M4A) | ✅ |
| WAV | ✅ |
| OGG Vorbis | ✅ |
| OPUS | ✅ |
| WMA | ✅ |
| ALAC | ✅ |

**对比 Electron**：原版本依赖 Chromium 的 `<audio>` 元素解码，受浏览器能力限制。Rust 版使用 Symphonia 可直接解码所有主流格式，性能远超 Web Audio API。

#### 5.1.4 音频线程优先级

```rust
// 音频线程设置为实时优先级（仅 macOS/Linux）
#[cfg(not(target_os = "windows"))]
fn set_realtime_priority() {
    unsafe {
        let policy = libc::SCHED_RR;
        let param = libc::sched_param { sched_priority: 50 };
        libc::pthread_setschedparam(
            libc::pthread_self(),
            policy,
            &param as *const libc::sched_param
        );
    }
}
```

---

### 5.2 插件管理器 (Plugin Manager)

#### 5.2.1 插件生命周期

```
┌──────────┐    ┌──────────┐    ┌──────────┐    ┌──────────┐
│  Install │───▶│ Validate │───▶│  Load    │───▶│  Ready   │
└──────────┘    └──────────┘    └──────────┘    └────┬─────┘
                                                     │
                    ┌──────────┐    ┌──────────┐     │
                    │ Disabled │◀───│  Error   │◀────┤
                    └──────────┘    └──────────┘     │
                                                     │
                    ┌──────────┐    ┌──────────┐     │
                    │  Removed │◀───│ Uninstall │◀────┘
                    └──────────┘    └──────────┘
```

#### 5.2.2 核心结构

```rust
/// 插件管理器
pub struct PluginManager {
    plugins: HashMap<PluginId, LoadedPlugin>,
    sandbox_runtime: Arc<SandboxRuntime>,
    event_bus: Arc<EventBus>,
}

/// 已加载的插件
pub struct LoadedPlugin {
    pub info: PluginInfo,
    pub sources: Vec<MusicSource>,
    pub sandbox_instance: SandboxInstance,
    pub status: PluginStatus,
    pub installed_at: DateTime<Utc>,
}

/// 插件元信息（从文件头注释解析）
pub struct PluginInfo {
    pub id: PluginId,
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: String,
    pub homepage: String,
    pub plugin_type: PluginType, // CmmuuNative 或 LX
}

/// 音源定义
pub struct MusicSource {
    pub id: String,           // kw, wy, tx, kg, mg
    pub name: String,         // 酷我音乐, 网易云音乐, ...
    pub qualities: Vec<Quality>,  // ["128k", "320k", "flac", ...]
}

#[derive(Debug, Clone)]
pub enum Quality {
    K128,
    K320,
    Flac,
    Flac24bit,
    HiRes,
    Atmos,
    Master,
}

impl Quality {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "128k" => Some(Self::K128),
            "320k" => Some(Self::K320),
            "flac" => Some(Self::Flac),
            "flac24bit" => Some(Self::Flac24bit),
            "hires" => Some(Self::HiRes),
            "atmos" => Some(Self::Atmos),
            "master" => Some(Self::Master),
            _ => None,
        }
    }
}
```

#### 5.2.3 插件 API 协议

```rust
/// 插件调用的标准请求
#[derive(Serialize, Deserialize)]
pub struct PluginRequest {
    pub action: PluginAction,
    pub source: String,
    pub music_info: MusicInfo,
    pub quality: Option<Quality>,
}

#[derive(Serialize, Deserialize)]
pub enum PluginAction {
    MusicUrl,
    GetPic,
    GetLyric,
}

/// 音乐信息（兼容原版字段）
#[derive(Serialize, Deserialize)]
pub struct MusicInfo {
    pub id: Option<String>,
    #[serde(alias = "songmid")]
    pub song_mid: Option<String>,
    pub hash: Option<String>,
    pub name: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub duration: Option<f64>,
}
```

#### 5.2.4 LX 插件兼容层

保留对现有洛雪音乐插件生态的完整兼容：

```rust
/// LX 事件驱动插件适配器
pub struct LxPluginAdapter {
    instance: SandboxInstance,
}

impl LxPluginAdapter {
    /// 注册事件监听器
    pub fn on_event(&self, event: &str, handler: impl Fn(Value) -> Value + 'static);
    
    /// 发送事件
    pub fn emit(&self, event: &str, data: Value);
    
    /// 转换为 Cmmuu 原生接口（自动适配）
    pub fn into_native(self) -> impl CmmuuPlugin;
}

// LX 事件名称常量（与 globalThis.lx.EVENT_NAMES 对齐）
const LX_EVENT_INITED: &str = "inited";
const LX_EVENT_REQUEST: &str = "request";
const LX_EVENT_UPDATE_ALERT: &str = "updateAlert";
```

---

### 5.3 播放列表管理器 (Playlist Manager)

```rust
/// 播放列表管理器
pub struct PlaylistManager {
    db: Arc<Database>,
    current_playlist: RwLock<Option<Playlist>>,
    play_queue: RwLock<PlayQueue>,
}

/// 播放列表
#[derive(Serialize, Deserialize)]
pub struct Playlist {
    pub id: String,
    pub name: String,
    pub songs: Vec<Song>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 播放队列
pub struct PlayQueue {
    pub items: VecDeque<QueueItem>,
    pub current_index: usize,
    pub play_mode: PlayMode,
}

#[derive(Clone)]
pub enum PlayMode {
    Sequential,  // 顺序播放
    Loop,        // 列表循环
    Shuffle,     // 随机播放
    Single,      // 单曲循环
}
```

---

## 6. 插件系统设计

### 6.1 沙箱架构深度设计

**核心原则：进程级隔离，最小权限，资源限制。**

```
┌───────────────────────────────────────────────┐
│              Main Process (Rust)               │
│                                                │
│  ┌─────────────────────────────┐               │
│  │     Plugin Host API          │               │
│  │  - request(url, options)    │               │
│  │  - utils.crypto.*           │               │
│  │  - NoticeCenter(type,data)  │               │
│  └──────────┬──────────────────┘               │
│             │ IPC (Unix Socket / Named Pipe)    │
├─────────────┼──────────────────────────────────┤
│             ▼                                  │
│  ┌──────────────────────────────┐              │
│  │   Sandbox Process (Deno)     │              │
│  │                              │              │
│  │  CPU: max 50%                │              │
│  │  Memory: max 128MB           │              │
│  │  Network: whitelist only     │              │
│  │  FS: read-only plugin dir    │              │
│  │  Subprocess: denied          │              │
│  │                              │              │
│  │  ┌────────────────────────┐  │              │
│  │  │  Plugin JS Code        │  │              │
│  │  │  (executed in sandbox) │  │              │
│  │  └────────────────────────┘  │              │
│  └──────────────────────────────┘              │
└───────────────────────────────────────────────┘
```

#### 沙箱方案对比

| 方案 | 隔离性 | 性能 | 体积 | 兼容性 |
|------|--------|------|------|--------|
| **Deno Core** | ⭐⭐⭐⭐⭐ 进程级 | ⭐⭐⭐⭐ | +15MB | JS 100% |
| **QuickJS** | ⭐⭐⭐ 线程级 | ⭐⭐⭐⭐⭐ | +2MB | ES2020 |
| **Boa (JS in Rust)** | ⭐⭐⭐⭐ 同进程 | ⭐⭐⭐ | +5MB | ES2022 |
| **V8 Isolate** | ⭐⭐⭐⭐ 线程级 | ⭐⭐⭐⭐ | +10MB | JS 100% |

**推荐方案：QuickJS**（首次发布）+ **Deno Core**（高级用户可选）。

理由：
- QuickJS 体积小（2MB），集成简单，性能足够处理插件逻辑（插件本身不做重计算）
- 可加 feature flag 支持 Deno Core 以获得进程级隔离和完整 JS 特性
- 对 LX 插件兼容性好（LX 插件不依赖复杂 Node.js API）

#### QuickJS 沙箱实现概要

```rust
use quickjs_rs::{Context, Runtime};

pub struct QuickJsSandbox {
    runtime: Runtime,
    context: Context,
    host_api: HostApi,
}

impl QuickJsSandbox {
    pub fn new() -> Result<Self, SandboxError> {
        let runtime = Runtime::new();
        let context = Context::new(&runtime)?;
        
        // 注入宿主 API
        let host_api = HostApi::default();
        context.add_global("cmmuumusic", host_api.clone())?;
        
        // 注入安全受限的全局对象
        context.add_global("console", SandboxConsole::new())?;
        context.add_global("JSON", JsonApi)?;
        context.add_global("Buffer", BufferApi)?;
        
        // 定时器在沙箱内限制
        context.add_global("setTimeout", TimeoutApi::new(Duration::from_secs(30)))?;
        
        // 禁止 require
        context.eval("var require = undefined")?;
        
        Ok(Self { runtime, context, host_api })
    }
    
    pub fn execute(&self, code: &str) -> Result<Value, SandboxError> {
        // 设置 CPU 时间限制，防止死循环
        self.context.set_max_duration(Duration::from_secs(10));
        self.context.eval(code).map_err(SandboxError::from)
    }
    
    pub fn call_method(&self, name: &str, args: Vec<Value>) -> Result<Value, SandboxError> {
        self.context.call_function(name, args).map_err(SandboxError::from)
    }
}
```

#### 宿主 API 实现

```rust
/// 注入到沙箱的 cmmuumusic 宿主对象
pub struct HostApi {
    http_client: reqwest::Client,
    event_sender: mpsc::Sender<HostEvent>,
}

impl HostApi {
    /// cmmuumusic.request(url, options) — HTTP 请求
    async fn request(&self, url: String, options: RequestOptions) -> Result<Response, RequestError> {
        // URL 白名单检查（防止 SSRF）
        self.validate_url(&url)?;
        
        let req = self.build_request(url, options)?;
        let resp = self.http_client.execute(req).await?;
        
        Ok(Response {
            status_code: resp.status().as_u16(),
            headers: resp.headers().clone(),
            body: resp.text().await,
        })
    }
    
    /// cmmuumusic.NoticeCenter(type, data) — 通知宿主
    fn notice(&self, notice_type: NoticeType, data: NoticeData) {
        self.event_sender.send(HostEvent::Notice(notice_type, data));
    }
    
    /// cmmuumusic.utils.crypto — 加密工具
    fn crypto_md5(&self, input: &str) -> String {
        format!("{:x}", md5::compute(input.as_bytes()))
    }
    
    fn crypto_aes_encrypt(&self, data: &[u8], mode: &str, key: &[u8], iv: &[u8]) -> Result<Vec<u8>>;
    
    fn crypto_rsa_encrypt(&self, data: &[u8], public_key_pem: &str) -> Result<Vec<u8>>;
    
    // URL 安全检查（防 SSRF、防内网攻击）
    fn validate_url(&self, url: &str) -> Result<(), RequestError> {
        let parsed = url::Url::parse(url)?;
        let host = parsed.host_str().unwrap_or("");
        
        // 禁止内网、本地地址
        let blocked = ["127.0.0.1", "localhost", "0.0.0.0", "10.", "172.16.", "192.168."];
        for blocked_prefix in &blocked {
            if host.contains(blocked_prefix) {
                return Err(RequestError::BlockedUrl);
            }
        }
        
        // 只允许 HTTPS
        if parsed.scheme() != "https" {
            return Err(RequestError::InsecureHttp);
        }
        
        Ok(())
    }
}
```

---

### 6.2 插件安装流程

```
用户选择 .js 文件
     │
     ▼
┌─────────────────┐
│ 1. 文件校验      │  检查扩展名、编码、大小
└───────┬─────────┘
        │
        ▼
┌─────────────────┐
│ 2. 安全检查      │  静态分析，禁止危险 API 调用
└───────┬─────────┘
        │
        ▼
┌─────────────────┐
│ 3. 沙箱预览      │  在沙箱中执行，验证导出完整性
└───────┬─────────┘
        │
        ▼
┌─────────────────┐
│ 4. 元数据提取    │  解析 pluginInfo、sources
└───────┬─────────┘
        │
        ▼
┌─────────────────┐
│ 5. 去重检查      │  同名同版本拒绝安装
└───────┬─────────┘
        │
        ▼
┌─────────────────┐
│ 6. 持久化存储    │  保存到 AppDir/plugins/
└───────┬─────────┘
        │
        ▼
┌─────────────────┐
│ 7. 注册到管理器  │  加入 PluginManager，可用状态
└─────────────────┘
```

---

## 7. 音频管线设计

### 7.1 完整数据流

```
用户点击播放
     │
     ▼
┌──────────────────┐
│ 1. 插件获取 URL   │  plugin.musicUrl(source, info, quality)
└──────┬───────────┘
       │ 返回 HTTP(S) URL
       ▼
┌──────────────────┐
│ 2. URL 解析       │  判断直链/重定向/M3U8 等
└──────┬───────────┘
       │
       ▼
┌──────────────────┐
│ 3. 缓存检查       │  查询本地缓存，命中则跳过下载
└──────┬───────────┘
       │
       ▼
┌──────────────────┐
│ 4. 流式下载       │  tokio + reqwest streaming
│    (HTTP Range)   │  支持断点续传、Seek 时 Range 请求
└──────┬───────────┘
       │
       ▼
┌──────────────────┐
│ 5. 解码           │  Symphonia 流式解码
│                   │  支持 Container → Codec → PCM
└──────┬───────────┘
       │ f32 PCM
       ▼
┌──────────────────┐
│ 6. 音频处理       │  音量/均衡器/采样率转换
└──────┬───────────┘
       │
       ▼
┌──────────────────┐
│ 7. 输出 + 可视化  │  cpal 音频输出 + FFT 数据推前端
└──────────────────┘
```

### 7.2 内存管理策略

```rust
/// 音频缓冲区 — 零拷贝环形缓冲区
pub struct AudioRingBuffer {
    buffer: Vec<AtomicF32>,  // 原子 f32 数组，支持无锁读写
    write_pos: AtomicUsize,
    read_pos: AtomicUsize,
    capacity: usize,
}

impl AudioRingBuffer {
    /// 生产者写入（音频解码线程）
    pub fn push_samples(&self, samples: &[f32]) -> usize;
    
    /// 消费者读取（音频输出线程）
    pub fn pull_samples(&self, buf: &mut [f32]) -> usize;
    
    /// 可读取的样本数
    pub fn available(&self) -> usize;
}
```

### 7.3 可视化引擎

```rust
/// 实时频谱分析器
pub struct SpectrumAnalyzer {
    fft: RealFftPlanner<f32>,
    window: Vec<f32>,
    output: Vec<f32>,
}

impl SpectrumAnalyzer {
    pub fn new(window_size: usize) -> Self;
    
    /// 输入 PCM 数据，输出频谱
    pub fn analyze(&mut self, samples: &[f32]) -> SpectrumData;
}

/// 频谱数据（推送到前端的格式）
#[derive(Serialize)]
pub struct SpectrumData {
    pub bands: Vec<f32>,         // 频率带幅度
    pub waveform: Vec<f32>,      // 波形数据（降采样）
    pub peak: f32,               // 峰值
    pub rms: f32,                // 均方根
}
```

### 7.4 音频特效管线（可扩展）

```rust
/// 音频特效链
pub struct AudioEffectChain {
    effects: Vec<Box<dyn AudioEffect>>,
}

pub trait AudioEffect: Send + Sync {
    fn name(&self) -> &str;
    fn process(&mut self, input: &[f32], output: &mut [f32]);
    fn reset(&mut self);
}

// 内置效果器
pub struct Equalizer { /* 10-band EQ */ }
pub struct Compressor { /* 动态压缩 */ }
pub struct Reverb { /* 混响 */ }
pub struct PitchShifter { /* 变调 */ }
```

---

## 8. 数据层设计

### 8.1 数据库 Schema

```sql
-- 播放列表
CREATE TABLE playlists (
    id          TEXT PRIMARY KEY,
    name        TEXT NOT NULL,
    description TEXT,
    cover_url   TEXT,
    created_at  TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at  TEXT NOT NULL DEFAULT (datetime('now'))
);

-- 歌曲
CREATE TABLE songs (
    id          TEXT PRIMARY KEY,           -- 格式: {source}:{id}
    source      TEXT NOT NULL,              -- kw, wy, tx, etc.
    title       TEXT NOT NULL,
    artist      TEXT,
    album       TEXT,
    cover_url   TEXT,
    duration    REAL,                       -- 秒
    lyric_text  TEXT,                       -- 完整歌词
    created_at  TEXT NOT NULL DEFAULT (datetime('now'))
);

-- 播放列表-歌曲关联
CREATE TABLE playlist_songs (
    playlist_id TEXT NOT NULL REFERENCES playlists(id) ON DELETE CASCADE,
    song_id     TEXT NOT NULL REFERENCES songs(id),
    sort_order  INTEGER NOT NULL,
    added_at    TEXT NOT NULL DEFAULT (datetime('now')),
    PRIMARY KEY (playlist_id, song_id)
);

-- 播放历史
CREATE TABLE play_history (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    song_id     TEXT NOT NULL REFERENCES songs(id),
    played_at   TEXT NOT NULL DEFAULT (datetime('now')),
    duration_played REAL                    -- 实际播放时长
);

-- 插件注册表
CREATE TABLE plugins (
    id          TEXT PRIMARY KEY,
    name        TEXT NOT NULL,
    version     TEXT NOT NULL,
    author      TEXT,
    plugin_type TEXT NOT NULL,              -- 'cmmuu' | 'lx'
    file_path   TEXT NOT NULL,
    enabled     INTEGER NOT NULL DEFAULT 1,
    installed_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at  TEXT NOT NULL DEFAULT (datetime('now'))
);

-- 用户设置 (JSON blob，灵活扩展)
CREATE TABLE settings (
    key         TEXT PRIMARY KEY,
    value       TEXT NOT NULL               -- JSON
);

-- 音乐缓存索引
CREATE TABLE cache_index (
    url_hash    TEXT PRIMARY KEY,           -- URL 的 SHA-256 前 16 字节
    original_url TEXT NOT NULL,
    local_path  TEXT NOT NULL,
    file_size   INTEGER,
    expires_at  TEXT,
    created_at  TEXT NOT NULL DEFAULT (datetime('now'))
);

-- 索引
CREATE INDEX idx_play_history_played_at ON play_history(played_at);
CREATE INDEX idx_cache_expires ON cache_index(expires_at);
CREATE INDEX idx_playlist_songs_order ON playlist_songs(playlist_id, sort_order);
CREATE INDEX idx_songs_source ON songs(source);
```

### 8.2 数据模型 (Rust)

```rust
/// 歌曲
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Song {
    pub id: String,
    pub source: String,
    pub title: String,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub cover_url: Option<String>,
    pub duration: Option<f64>,
    pub lyric_text: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// 播放列表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Playlist {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub cover_url: Option<String>,
    pub songs: Vec<Song>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 播放历史条目
#[derive(Debug, Serialize, Deserialize)]
pub struct PlayHistoryEntry {
    pub song: Song,
    pub played_at: DateTime<Utc>,
    pub duration_played: Option<f64>,
}

/// 用户设置
#[derive(Debug, Serialize, Deserialize)]
pub struct AppSettings {
    pub theme: Theme,
    pub audio_quality: Quality,
    pub volume: f32,
    pub equalizer: EqualizerSettings,
    pub cache_limit_mb: u64,
    pub auto_update: bool,
    pub language: String,
    pub lyrics: LyricsSettings,
    pub hotkeys: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Theme {
    pub mode: ThemeMode,         // Light / Dark / System
    pub primary_color: String,
    pub background_image: Option<String>,
    pub blur_effect: bool,
}
```

---

## 9. 安全模型

### 9.1 安全架构

```
┌──────────────────────────────────────────────────────────┐
│                    威胁模型矩阵                            │
├──────────────┬───────────────┬──────────────┬────────────┤
│   威胁向量    │   风险等级     │   缓解措施    │   验证方式  │
├──────────────┼───────────────┼──────────────┼────────────┤
│ 恶意插件      │     HIGH      │ 沙箱隔离      │ 安全审计   │
│ 网络 SSRF    │     HIGH      │ URL 白名单    │ 渗透测试   │
│ 数据泄露      │     MEDIUM    │ 本地存储      │ 代码审查   │
│ 供应链攻击    │     MEDIUM    │ Cargo.lock   │ CI 扫描    │
│ IPC 劫持     │     LOW       │ Unix Socket   │ 端口扫描   │
│ 内存破坏      │     LOW       │ Rust 安全     │ Miri + ASAN│
└──────────────┴───────────────┴──────────────┴────────────┘
```

### 9.2 Tauri v2 安全配置

```json
// tauri.conf.json 安全相关配置
{
  "app": {
    "security": {
      "csp": "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' https: data:; media-src 'self' https:; connect-src 'self' https:",
      "freezePrototype": true,
      "pattern": {
        "use": "isolation"
      }
    }
  }
}
```

### 9.3 Capabilities 权限声明 (Tauri v2)

```json
// capabilities/default.json
{
  "identifier": "default",
  "description": "默认权限集",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "shell:allow-open",
    "dialog:default",
    "fs:allow-read-text-file",
    "fs:allow-write-text-file",
    "fs:allow-exists",
    {
      "identifier": "fs:scope",
      "allow": [
        { "path": "$APPDATA/**" },
        { "path": "$DOWNLOAD/**" }
      ]
    },
    "notification:default",
    "updater:default",
    "process:allow-exit"
  ]
}
```

---

## 10. 性能优化策略

### 10.1 关键指标与目标

| 指标 | 目标 | 测量方法 |
|------|------|----------|
| 冷启动时间 | < 1.5s | `tauri dev --bench` |
| 热启动时间 | < 0.5s | 从系统托盘恢复 |
| 首帧渲染 | < 300ms | Performance API |
| 包体积 (macOS) | < 30MB | `du -sh .app` |
| 空闲内存 | < 80MB | Activity Monitor |
| 播放内存 | < 150MB | 播放 FLAC 44.1kHz/16bit |
| 搜索响应 | < 500ms | 插件返回 + UI 渲染 |
| 歌曲切换延迟 | < 100ms | URL → 首帧音频 |

### 10.2 前端性能优化

1. **虚拟滚动**：歌单列表使用 `vue-virtual-scroller`，10000+ 曲目不卡顿
2. **组件懒加载**：`defineAsyncComponent` 拆分 Chunk
3. **图片懒加载**：封面图使用 Intersection Observer
4. **Web Worker**：歌词解析、搜索结果去重在 Worker 中执行
5. **CSS Containment**：`contain: content` 减少重绘范围
6. **字体子集化**：仅打包所需字符

### 10.3 Rust 后端优化

1. **零拷贝**：音频缓冲区使用 `bytes` crate 的 `Bytes` 共享引用
2. **连接池**：Reqwest `Client` 全局单例，连接复用
3. **无锁数据结构**：`parking_lot::RwLock` 替代标准库 `RwLock`
4. **SIMD**：`symphonia` 自动启用 SIMD 加速解码
5. **内存池**：音频帧对象池，减少分配
6. **编译优化**：
   ```toml
   [profile.release]
   opt-level = 3
   lto = "fat"
   codegen-units = 1
   strip = true
   panic = "abort"
   ```

### 10.4 音频预加载 (Audio Preloading)

```rust
/// 预加载管理器 — 在歌曲切换前预加载下 N 首
pub struct PreloadManager {
    queue: VecDeque<PreloadTask>,
    max_preload: usize, // 默认 3
}

impl PreloadManager {
    /// 根据播放模式预加载后续歌曲
    pub async fn preload_next(&self, queue: &PlayQueue, mode: PlayMode);
    
    /// 取消未开始的预加载任务
    pub fn cancel_pending(&self);
}
```

---

## 11. UI/UX 设计规范

### 11.1 布局结构

```
┌───────────────────────────────────────────────────┐
│  Title Bar: [Logo] Cmmuu Music  ─  [最小化] [最大化] [关闭] │
├───────────┬───────────────────────────────────────┤
│           │                                       │
│  Sidebar  │          Content Area                 │
│           │                                       │
│  ├ 首页    │  ┌───────────────────────────────┐   │
│  ├ 发现    │  │                               │   │
│  ├ 歌单    │  │    (根据 Tab 切换内容)          │   │
│  ├ 本地    │  │                               │   │
│  ├ 最近    │  └───────────────────────────────┘   │
│  ├────     │                                       │
│  ├ 设置    │                                       │
│  └ 关于    │                                       │
├───────────┴───────────────────────────────────────┤
│  Player Bar: [封面] 歌曲信息 ── 进度条 ── [播放控制] │
└───────────────────────────────────────────────────┘
```

### 11.2 主题系统

```typescript
// 主题定义
interface ThemeConfig {
  mode: 'light' | 'dark' | 'system';
  colors: {
    // 背景
    bgPrimary: string;       // 主背景
    bgSecondary: string;     // 次背景（卡片、侧栏）
    bgTertiary: string;      // 悬浮态
    
    // 文字
    textPrimary: string;
    textSecondary: string;
    textTertiary: string;
    
    // 主题色
    accent: string;
    accentHover: string;
    
    // 功能色
    success: string;
    warning: string;
    error: string;
    
    // 播放器
    playerBg: string;       // 播放条背景
    progressBar: string;    // 进度条已播放
    progressBg: string;     // 进度条背景
  };
  
  // 圆角
  borderRadius: {
    sm: number;    // 4px
    md: number;    // 8px
    lg: number;    // 12px
    xl: number;    // 16px
    full: number;  // 9999px
  };
  
  // 阴影
  shadows: {
    sm: string;
    md: string;
    lg: string;
  };
  
  // 模糊效果
  blur: {
    light: string;
    heavy: string;
  };
}
```

### 11.3 动画规范

| 动画 | 时长 | 缓动 | 说明 |
|------|------|------|------|
| 侧栏展开 | 250ms | `cubic-bezier(0.4,0,0.2,1)` | 带模糊背景 |
| 列表项悬浮 | 150ms | ease-out | 微缩放 1.02x |
| 播放器展开 | 300ms | spring(1, 0.8, 20) | 弹性动画 |
| 进度条更新 | RAF | linear | 60fps |
| 封面旋转 | 20s/圈 | linear | 播放时旋转，暂停停止 |
| 频谱条 | RAF | - | 音频线程直推数据 |

---

## 12. 测试策略

### 12.1 测试金字塔

```
          ╱─────╲
         ╱  E2E  ╲          Playwright + Tauri Driver
        ╱─────────╲
       ╱ Integration╲        cargo test --test *
      ╱───────────────╲
     ╱   Unit Tests    ╲      cargo test + vitest
    ╱───────────────────╲
   ╱  Static Analysis    ╲    clippy + eslint + cargo audit
  ╱───────────────────────╲
```

### 12.2 Rust 后端测试

```rust
// 单元测试
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_plugin_validate_valid() {
        let manager = PluginManager::new_test();
        let result = manager.validate_plugin("tests/fixtures/valid_plugin.js");
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_plugin_validate_no_exports() {
        let manager = PluginManager::new_test();
        let result = manager.validate_plugin("tests/fixtures/no_exports.js");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_audio_buffer_push_pull() {
        let buf = AudioRingBuffer::new(1024);
        let samples = vec![0.5f32; 512];
        assert_eq!(buf.push_samples(&samples), 512);
        assert_eq!(buf.available(), 512);
        
        let mut out = vec![0.0f32; 512];
        assert_eq!(buf.pull_samples(&mut out), 512);
    }
    
    #[test]
    fn test_music_info_deserialize_legacy() {
        let json = r#"{"songmid": "abc123", "name": "test"}"#;
        let info: MusicInfo = serde_json::from_str(json).unwrap();
        assert_eq!(info.song_mid, Some("abc123".to_string()));
        assert_eq!(info.id, None); // legacy field, not id
    }
}
```

### 12.3 前端测试 (Vitest)

```typescript
// stores/player.test.ts
import { describe, it, expect, beforeEach } from 'vitest';
import { usePlayerStore } from '@/stores/player';

describe('PlayerStore', () => {
  beforeEach(() => {
    const store = usePlayerStore();
    store.$reset();
  });
  
  it('should start in idle state', () => {
    const store = usePlayerStore();
    expect(store.state).toBe('idle');
    expect(store.currentSong).toBeNull();
  });
  
  it('should update progress on tick', () => {
    const store = usePlayerStore();
    store.state = 'playing';
    store.duration = 100;
    store.tick(1.0);
    expect(store.currentTime).toBeCloseTo(1.0);
  });
  
  it('should toggle shuffle mode', () => {
    const store = usePlayerStore();
    expect(store.playMode).toBe('sequential');
    store.toggleShuffle();
    expect(store.playMode).toBe('shuffle');
  });
});
```

### 12.4 E2E 测试 (Playwright)

```typescript
// e2e/basic.spec.ts
import { test, expect } from '@playwright/test';

test('app launches and shows home page', async ({ page }) => {
  await page.goto('tauri://localhost');
  await expect(page.locator('[data-testid="app-title"]')).toHaveText('Cmmuu Music');
});

test('can search for music', async ({ page }) => {
  await page.goto('tauri://localhost');
  await page.click('[data-testid="search-tab"]');
  await page.fill('[data-testid="search-input"]', '周杰伦');
  await page.click('[data-testid="search-button"]');
  await expect(page.locator('[data-testid="search-results"]')).toBeVisible();
});
```

---

## 13. 构建与部署

### 13.1 构建流程

```
Source Code
     │
     ▼
┌──────────────┐
│  cargo build  │  Rust 编译（--release）
│  vite build   │  前端打包
└──────┬───────┘
       │
       ▼
┌──────────────┐
│ tauri build   │  资源打包 + 代码签名 + 安装包生成
└──────┬───────┘
       │
       ├──▶ macOS:    .dmg + .app bundle (签名+公证)
       ├──▶ Windows:  .msi + .exe installer (签名)
       └──▶ Linux:    .deb + .AppImage
```

### 13.2 macOS 代码签名与公证

```bash
# 1. 设置签名身份
export APPLE_SIGNING_IDENTITY="Developer ID Application: Your Name (TEAMID)"

# 2. 构建并签名
cargo tauri build --target aarch64-apple-darwin

# 3. 公证（Notarization）
xcrun notarytool submit "target/release/bundle/dmg/CmmuuMusic_*.dmg" \
  --apple-id "your@email.com" \
  --team-id "TEAMID" \
  --password "@keychain:AC_PASSWORD" \
  --wait

# 4. 装订票据（Staple）
xcrun stapler staple "target/release/bundle/dmg/CmmuuMusic_*.dmg"
```

### 13.3 CI/CD 配置 (GitHub Actions)

```yaml
name: Release

on:
  push:
    tags: ['v*']

jobs:
  build:
    strategy:
      matrix:
        include:
          - os: macos-latest
            target: aarch64-apple-darwin
            artifact: dmg
          - os: macos-latest
            target: x86_64-apple-darwin
            artifact: dmg
          - os: windows-latest
            target: x86_64-pc-windows-msvc
            artifact: msi
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
            artifact: deb

    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}
      
      - name: Setup Node
        uses: actions/setup-node@v4
        with:
          node-version: 22
          cache: 'yarn'
      
      - name: Install dependencies
        run: yarn install --frozen-lockfile
      
      # Linux 需要系统依赖
      - name: Install Linux deps
        if: runner.os == 'Linux'
        run: |
          sudo apt-get update
          sudo apt-get install -y \
            libwebkit2gtk-4.1-dev \
            libappindicator3-dev \
            librsvg2-dev \
            patchelf \
            libasound2-dev
      
      - name: Build
        run: cargo tauri build --target ${{ matrix.target }}
      
      - name: Upload artifacts
        uses: actions/upload-artifact@v4
        with:
          name: cmmuu-music-${{ matrix.target }}
          path: src-tauri/target/${{ matrix.target }}/release/bundle/
```

### 13.4 自动更新

使用 Tauri v2 内置的 `tauri-plugin-updater`：

```rust
// src-tauri/src/main.rs
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

```json
// tauri.conf.json
{
  "plugins": {
    "updater": {
      "endpoints": [
        "https://releases.cmmuu.shiqianjiang.cn/{{target}}/latest.json"
      ],
      "pubkey": "YOUR_PUBLIC_KEY_BASE64"
    }
  }
}
```

---

## 14. Electron → Tauri 迁移对照

### 14.1 模块迁移映射

| Electron (原版) | Tauri v2 (新版) | 说明 |
|-----------------|-----------------|------|
| `src/main/index.ts` | `src-tauri/src/main.rs` | 主进程 → Rust |
| `electron-builder.yml` | `tauri.conf.json` | 打包配置 |
| `ipcMain.handle()` | `#[tauri::command]` | IPC 接口 |
| `ipcRenderer.invoke()` | `invoke()` from `@tauri-apps/api` | 前端调用 |
| `BrowserWindow` | Tauri WebviewWindow | 窗口管理 |
| `autoUpdater` | `tauri-plugin-updater` | 自动更新 |
| `Notification` | `tauri-plugin-notification` | 系统通知 |
| `dialog` | `tauri-plugin-dialog` | 文件对话框 |
| `app.getPath('userData')` | `app_data_dir()` | 数据目录 |
| `globalShortcut` | `tauri-plugin-global-shortcut` | 全局快捷键 |
| `tray` | `tauri::tray::TrayIconBuilder` | 系统托盘 |
| `Menu` | `tauri::menu::MenuBuilder` | 原生菜单 |
| `shell.openPath()` | `tauri-plugin-shell` | 打开外部链接 |
| `fs` (Node.js) | `tauri-plugin-fs` | 文件系统 |
| `<audio>` Web API | Symphonia + cpal | 音频播放 |
| `nodeIntegration` | N/A (不需要) | 无 Node.js 上下文 |

### 14.2 代码迁移示例

**原版 (Electron + Node.js)：**
```typescript
// main/events/songList.ts
import { ipcMain } from 'electron';
import { playlistService } from '../services/songList/';

ipcMain.handle('playlist:create', async (event, name) => {
  return await playlistService.createPlaylist(name);
});
```

**新版 (Tauri + Rust)：**
```rust
// src-tauri/src/commands/playlist.rs
use tauri::State;
use crate::core::playlist::PlaylistManager;

#[tauri::command]
pub async fn create_playlist(
    name: String,
    manager: State<'_, PlaylistManager>,
) -> Result<Playlist, String> {
    manager.create_playlist(&name).await.map_err(|e| e.to_string())
}
```

**前端调用（相同）：**
```typescript
// api/commands.ts
import { invoke } from '@tauri-apps/api/core';

export const createPlaylist = (name: string) => 
  invoke<Playlist>('create_playlist', { name });
```

### 14.3 前端代码复用率

| 类别 | 复用率 | 说明 |
|------|--------|------|
| Vue 组件 | **90%** | 大部分可直接迁移，仅需替换 IPC 调用 |
| Pinia Store | **85%** | actions 中的 `invoke` 替换 `ipcRenderer` |
| 样式 (SCSS) | **95%** | 完全可复用 |
| 路由配置 | **100%** | vue-router 不变 |
| 工具函数 | **90%** | 移除 Node.js 依赖即可 |
| 类型定义 | **80%** | 需对齐 Rust 端类型 |
| `applemusic-like-lyrics` | **100%** | 纯前端库，直接复用 |

---

## 15. 风险与缓解

| 风险 | 严重性 | 概率 | 缓解措施 |
|------|--------|------|----------|
| **插件兼容性** LX 插件工作原理可能不兼容 | HIGH | MEDIUM | 完整测试现有 LX 插件生态，提供兼容层和迁移指南 |
| **QuickJS 性能不足** 插件复杂逻辑执行慢 | MEDIUM | LOW | 提供 Deno Core 备选方案（feature flag） |
| **WebView 差异** macOS WKWebView vs Windows WebView2 行为不同 | MEDIUM | MEDIUM | Tauri v2 已处理大部分差异，CI 双平台测试 |
| **音频驱动兼容** Linux ALSA/PulseAudio 配置多样 | LOW | MEDIUM | cpal 已处理抽象，文档说明推荐配置 |
| **社区迁移阻力** 用户习惯 Electron 版本 | LOW | HIGH | 双版本维护期，渐进迁移 |
| **Tauri v2 不稳定** 框架仍在小版本迭代 | LOW | MEDIUM | 锁定 Tauri 小版本，定期更新测试 |

---

## 16. 附录

### A. 参考资源

| 资源 | 地址 |
|------|------|
| Cmmuu Music 仓库 | https://github.com/timeshiftsauce/CmmuuMusic |
| 项目文档 | https://cmmuu.docs.shiqianjiang.cn |
| Tauri v2 文档 | https://v2.tauri.app |
| Symphonia 音频库 | https://github.com/pdeljanov/Symphonia |
| cpal 音频输出 | https://github.com/RustAudio/cpal |
| applemusic-like-lyrics | https://github.com/Steve-xmh/applemusic-like-lyrics |
| QuickJS | https://bellard.org/quickjs/ |
| Deno Core | https://crates.io/crates/deno_core |

### B. 开发路线图

| 阶段 | 内容 | 预估 |
|------|------|------|
| **Phase 0: 原型验证** | Tauri + Vue 骨架搭建，音频播放 POC | 2 周 |
| **Phase 1: 核心重写** | 音频引擎、插件沙箱、数据库、IPC | 6 周 |
| **Phase 2: UI 迁移** | 前端组件迁移，主题系统，动画 | 3 周 |
| **Phase 3: 插件生态** | LX 兼容层，插件商店，开发者工具 | 3 周 |
| **Phase 4: 完善** | 测试、文档、CI/CD、签名公证 | 2 周 |
| **Phase 5: 发布** | Beta → RC → Stable | 2 周 |

### C. 变更记录

| 版本 | 日期 | 变更 |
|------|------|------|
| v1.0.0 | 2026-05-31 | 初版 SDD 完成 |

---

> **本文档基于 Cmmuu Music 现有架构与功能，结合 Rust + Tauri v2 技术栈进行全面现代化重设计。所有设计决策均考虑了与原版插件的兼容性、性能优化、安全性和用户体验。**
