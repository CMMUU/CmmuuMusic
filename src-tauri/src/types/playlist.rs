use serde::{Deserialize, Serialize};

use super::music::Song;

/// 播放列表。对齐 SDD §8.1 playlists 表。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Playlist {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub cover_url: Option<String>,
    #[serde(default)]
    pub songs: Vec<Song>,
    pub created_at: String,
    pub updated_at: String,
}

/// 播放模式。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PlayMode {
    /// 顺序播放（播完即停）
    Sequential,
    /// 列表循环
    Loop,
    /// 随机播放
    Shuffle,
    /// 单曲循环
    Single,
}

impl Default for PlayMode {
    fn default() -> Self {
        Self::Sequential
    }
}
