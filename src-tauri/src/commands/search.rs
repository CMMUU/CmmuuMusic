use crate::commands::plugin::is_builtin_changqing_enabled;
use crate::core::search::aggregator::{list_source_playlist_songs as list_provider_playlist_songs, search_all};
use crate::types::music::Song;
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

#[tauri::command]
pub async fn list_source_playlist_songs(
    source: String,
    playlist_id: String,
    state: tauri::State<'_, AppState>,
) -> Result<Vec<Song>, String> {
    let builtin_changqing_enabled = is_builtin_changqing_enabled(&state)?;
    Ok(list_provider_playlist_songs(
        &source,
        &playlist_id,
        builtin_changqing_enabled,
    ))
}
