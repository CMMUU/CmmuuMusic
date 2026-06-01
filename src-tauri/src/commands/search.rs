use tauri::State;

use crate::types::search::{SearchRequest, SearchResult};
use crate::AppState;

/// 搜索音乐（骨架：搜索阶段接入插件音源聚合）。
#[tauri::command]
pub async fn search_music(
    request: SearchRequest,
    _state: State<'_, AppState>,
) -> Result<SearchResult, String> {
    Ok(SearchResult {
        songs: Vec::new(),
        total: 0,
        page: request.page,
        has_more: false,
    })
}
