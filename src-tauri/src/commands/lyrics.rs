use tauri::State;

use crate::AppState;

/// 获取歌词（骨架：歌词阶段接入插件 getLyric 与缓存）。
#[tauri::command]
pub async fn get_lyrics(
    song_id: String,
    state: State<'_, AppState>,
) -> Result<Option<String>, String> {
    // 优先从已存储的歌曲记录读取 lyric_text
    state
        .db
        .with_conn(|c| {
            c.query_row(
                "SELECT lyric_text FROM songs WHERE id = ?1",
                rusqlite::params![song_id],
                |r| r.get::<_, Option<String>>(0),
            )
            .or(Ok(None))
        })
        .map_err(|e| e.to_string())
}
