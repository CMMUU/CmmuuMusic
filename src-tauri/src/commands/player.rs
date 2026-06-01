use std::path::PathBuf;

use tauri::State;

use crate::core::audio::PlaybackStatus;
use crate::AppState;

/// 播放本地文件。
#[tauri::command]
pub async fn play_file(path: String, state: State<'_, AppState>) -> Result<(), String> {
    let p = PathBuf::from(&path);
    if !p.exists() {
        return Err(format!("文件不存在: {path}"));
    }
    state.audio.play_file(&p).map_err(|e| e.to_string())
}

/// 播放远程 URL（音频管线阶段实现）。
#[tauri::command]
pub async fn play_url(url: String, state: State<'_, AppState>) -> Result<(), String> {
    state.audio.play_url(&url).map_err(|e| e.to_string())
}

/// 暂停 / 恢复。
#[tauri::command]
pub fn toggle_pause(state: State<'_, AppState>) {
    state.audio.toggle_pause();
}

/// 停止播放。
#[tauri::command]
pub fn stop(state: State<'_, AppState>) {
    state.audio.stop();
}

/// 跳转到指定位置（秒）。
#[tauri::command]
pub fn seek(position_secs: f64, state: State<'_, AppState>) -> Result<(), String> {
    state.audio.seek(position_secs).map_err(|e| e.to_string())
}

/// 设置音量 (0.0 - 1.0)。
#[tauri::command]
pub fn set_volume(volume: f32, state: State<'_, AppState>) {
    state.audio.set_volume(volume);
}

/// 获取当前播放状态。
#[tauri::command]
pub fn get_playback_status(state: State<'_, AppState>) -> PlaybackStatus {
    state.audio.status()
}
