use serde::{Deserialize, Serialize};

/// 歌曲音质等级。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Quality {
    #[serde(rename = "128k")]
    K128,
    #[serde(rename = "320k")]
    K320,
    Flac,
    Flac24bit,
    Hires,
    Atmos,
    Master,
}

impl Quality {
    pub fn from_tag(s: &str) -> Option<Self> {
        match s {
            "128k" => Some(Self::K128),
            "320k" => Some(Self::K320),
            "flac" => Some(Self::Flac),
            "flac24bit" => Some(Self::Flac24bit),
            "hires" => Some(Self::Hires),
            "atmos" => Some(Self::Atmos),
            "master" => Some(Self::Master),
            _ => None,
        }
    }

    pub fn as_tag(&self) -> &'static str {
        match self {
            Self::K128 => "128k",
            Self::K320 => "320k",
            Self::Flac => "flac",
            Self::Flac24bit => "flac24bit",
            Self::Hires => "hires",
            Self::Atmos => "atmos",
            Self::Master => "master",
        }
    }
}

/// 歌曲。对齐 SDD §8.2 数据模型与 §8.1 songs 表。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Song {
    /// 格式: {source}:{id}
    pub id: String,
    pub source: String,
    pub title: String,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub cover_url: Option<String>,
    /// 时长（秒）
    pub duration: Option<f64>,
    pub lyric_text: Option<String>,
    /// 临时可播放 URL，仅用于搜索结果/播放请求，不持久化到 songs 表。
    pub play_url: Option<String>,
}

/// 插件请求中携带的音乐信息（兼容原版字段命名）。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
