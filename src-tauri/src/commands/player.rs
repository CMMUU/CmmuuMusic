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

/// 播放远程 URL（POC：异步下载完整音频后交给 Symphonia 解码）。
#[tauri::command]
pub async fn play_url(url: String, state: State<'_, AppState>) -> Result<(), String> {
    let response = reqwest::get(&url)
        .await
        .map_err(|e| format!("下载音频失败: {e}"))?;

    if !response.status().is_success() {
        return Err(format!("下载音频失败，HTTP 状态: {}", response.status()));
    }

    if response
        .content_length()
        .is_some_and(|len| len > 30 * 1024 * 1024)
    {
        return Err("音频文件超过 30MB，当前 POC 暂不支持".into());
    }

    let hint_ext = extension_from_url(&url).map(str::to_owned);
    let bytes = response
        .bytes()
        .await
        .map_err(|e| format!("读取音频数据失败: {e}"))?;

    state
        .audio
        .play_bytes(bytes.to_vec(), hint_ext.as_deref())
        .map_err(|e| e.to_string())
}

fn extension_from_url(url: &str) -> Option<&str> {
    let path = url.split('?').next().unwrap_or(url);
    let ext = path.rsplit_once('.')?.1;
    if ext.len() <= 5 {
        Some(ext)
    } else {
        None
    }
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
