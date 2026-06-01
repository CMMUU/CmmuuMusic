pub mod decoder;
pub mod engine;
pub mod pipeline;
pub mod visualizer;

pub use engine::CpalAudioEngine;

use std::path::Path;

use serde::Serialize;
use thiserror::Error;

/// 音频子系统错误。
#[derive(Debug, Error)]
pub enum AudioError {
    #[error("未找到可用的音频输出设备")]
    NoOutputDevice,

    #[error("不支持的音频流配置: {0}")]
    UnsupportedConfig(String),

    #[error("解码失败: {0}")]
    Decode(String),

    #[error("文件 I/O 错误: {0}")]
    Io(#[from] std::io::Error),

    #[error("音频流构建失败: {0}")]
    Stream(String),

    #[error("功能尚未实现: {0}")]
    NotImplemented(&'static str),
}

/// 播放状态。对齐 SDD §5.1.2。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PlaybackState {
    Idle,
    Loading,
    Playing,
    Paused,
    Ended,
    Error,
}

impl PlaybackState {
    pub(crate) fn from_u8(v: u8) -> Self {
        match v {
            1 => Self::Loading,
            2 => Self::Playing,
            3 => Self::Paused,
            4 => Self::Ended,
            5 => Self::Error,
            _ => Self::Idle,
        }
    }

    pub(crate) fn as_u8(self) -> u8 {
        match self {
            Self::Idle => 0,
            Self::Loading => 1,
            Self::Playing => 2,
            Self::Paused => 3,
            Self::Ended => 4,
            Self::Error => 5,
        }
    }
}

/// 推送到前端的播放状态快照。
#[derive(Debug, Clone, Serialize)]
pub struct PlaybackStatus {
    pub state: PlaybackState,
    /// 当前播放位置（秒）
    pub position: f64,
    /// 总时长（秒），未知时为 None
    pub duration: Option<f64>,
    /// 音量 0.0 - 1.0
    pub volume: f32,
}

/// 音频引擎核心接口。对齐 SDD §5.1.2（POC 阶段为同步接口）。
pub trait AudioEngine: Send + Sync {
    /// 播放本地文件
    fn play_file(&self, path: &Path) -> Result<(), AudioError>;

    /// 播放已下载的音频字节（URL POC 阶段由 command 层负责异步下载）
    fn play_bytes(&self, bytes: Vec<u8>, hint_ext: Option<&str>) -> Result<(), AudioError>;

    /// 暂停 / 恢复切换
    fn toggle_pause(&self);

    /// 停止并释放当前播放
    fn stop(&self);

    /// 跳转到指定位置（秒）
    fn seek(&self, position_secs: f64) -> Result<(), AudioError>;

    /// 设置音量 (0.0 - 1.0)
    fn set_volume(&self, volume: f32);

    /// 获取当前播放状态快照
    fn status(&self) -> PlaybackStatus;
}
