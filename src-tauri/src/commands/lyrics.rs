use tauri::State;

use crate::AppState;

#[tauri::command]
pub async fn get_lyrics(
    song_id: String,
    state: State<'_, AppState>,
) -> Result<Option<String>, String> {
    state
        .db
        .get_song_lyrics(&song_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_lyrics(
    song_id: String,
    lyric_text: String,
    state: State<'_, AppState>,
) -> Result<bool, String> {
    state
        .db
        .set_song_lyrics(&song_id, &lyric_text)
        .map_err(|e| e.to_string())
}
