use serde::{Deserialize, Serialize};

use super::music::Song;

/// 搜索类型。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SearchType {
    Song,
    Album,
    Artist,
    Playlist,
}

impl Default for SearchType {
    fn default() -> Self {
        Self::Song
    }
}

/// 搜索请求。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchRequest {
    pub keyword: String,
    #[serde(default)]
    pub search_type: SearchType,
    /// 指定音源，None 表示聚合所有可用音源
    pub source: Option<String>,
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_page_size")]
    pub page_size: u32,
}

fn default_page() -> u32 {
    1
}

fn default_page_size() -> u32 {
    20
}

/// 音源侧歌单，不等同于本地 SQLite 播放列表。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourcePlaylist {
    pub id: String,
    pub source: String,
    pub name: String,
    pub description: Option<String>,
    pub cover_url: Option<String>,
    pub song_count: Option<u32>,
}

/// 搜索结果。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    pub songs: Vec<Song>,
    pub playlists: Vec<SourcePlaylist>,
    pub total: u32,
    pub page: u32,
    pub has_more: bool,
}
