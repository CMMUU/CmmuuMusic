use crate::core::search::aggregator::search_all;
use crate::types::search::{SearchRequest, SearchResult};
use crate::AppState;

#[tauri::command]
pub async fn search_music(
    request: SearchRequest,
    _state: tauri::State<'_, AppState>,
) -> Result<SearchResult, String> {
    Ok(search_all(&request))
}
