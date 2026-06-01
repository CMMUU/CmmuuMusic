use tauri::State;
use uuid::Uuid;

use crate::types::playlist::Playlist;
use crate::AppState;

/// 创建播放列表。
#[tauri::command]
pub async fn create_playlist(
    name: String,
    state: State<'_, AppState>,
) -> Result<Playlist, String> {
    let id = Uuid::new_v4().to_string();
    state
        .db
        .create_playlist(&id, &name)
        .map_err(|e| e.to_string())
}

/// 列出所有播放列表。
#[tauri::command]
pub async fn list_playlists(state: State<'_, AppState>) -> Result<Vec<Playlist>, String> {
    state.db.list_playlists().map_err(|e| e.to_string())
}

/// 删除播放列表。
#[tauri::command]
pub async fn delete_playlist(id: String, state: State<'_, AppState>) -> Result<bool, String> {
    state.db.delete_playlist(&id).map_err(|e| e.to_string())
}
