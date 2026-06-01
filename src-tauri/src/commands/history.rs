use tauri::State;

use crate::types::music::{PlayHistoryRecord, Song};
use crate::AppState;

#[tauri::command]
pub async fn record_play_history(
    song: Song,
    duration_played: Option<f64>,
    state: State<'_, AppState>,
) -> Result<PlayHistoryRecord, String> {
    state
        .db
        .record_play_history(&song, duration_played)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_play_history(
    limit: Option<u32>,
    state: State<'_, AppState>,
) -> Result<Vec<PlayHistoryRecord>, String> {
    state
        .db
        .list_play_history(limit.unwrap_or(100))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_recent_songs(
    limit: Option<u32>,
    state: State<'_, AppState>,
) -> Result<Vec<Song>, String> {
    state
        .db
        .list_recent_songs(limit.unwrap_or(50))
        .map_err(|e| e.to_string())
}
