use crate::commands::plugin::is_builtin_changqing_enabled;
use crate::core::search::aggregator::search_all;
use crate::types::search::{SearchRequest, SearchResult};
use crate::AppState;

#[tauri::command]
pub async fn search_music(
    request: SearchRequest,
    state: tauri::State<'_, AppState>,
) -> Result<SearchResult, String> {
    let builtin_changqing_enabled = is_builtin_changqing_enabled(&state)?;
    Ok(search_all(&request, builtin_changqing_enabled))
}
