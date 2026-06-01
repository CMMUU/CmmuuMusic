use crate::core::search::demo::search_demo;
use crate::types::search::{SearchRequest, SearchResult};
use crate::AppState;

/// 搜索音乐（当前接入内置 demo 音源，后续阶段替换为插件音源聚合）。
#[tauri::command]
pub async fn search_music(
    request: SearchRequest,
    _state: tauri::State<'_, AppState>,
) -> Result<SearchResult, String> {
    Ok(search_demo(&request))
}
