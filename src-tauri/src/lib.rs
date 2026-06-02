pub mod commands;
pub mod core;
pub mod types;
pub mod utils;

use std::sync::Arc;

use parking_lot::Mutex;
use tauri::Manager;

use crate::core::audio::AudioEngine;
use crate::core::db::Database;

/// 全局应用状态，注入到 Tauri 的托管状态中，供各命令访问。
pub struct AppState {
    pub db: Arc<Database>,
    pub audio: Arc<dyn AudioEngine>,
    pub current_song: Mutex<Option<types::music::Song>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("info"),
    )
    .init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_process::init())
        .setup(|app| {
            let state = build_app_state(app)?;
            app.manage(state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::greet,
            // 播放控制
            commands::player::play_file,
            commands::player::play_url,
            commands::player::toggle_pause,
            commands::player::stop,
            commands::player::seek,
            commands::player::set_volume,
            commands::player::get_playback_status,
            // 播放列表
            commands::playlist::create_playlist,
            commands::playlist::list_playlists,
            commands::playlist::delete_playlist,
            commands::playlist::add_song_to_playlist,
            commands::playlist::list_playlist_songs,
            // 历史
            commands::history::record_play_history,
            commands::history::list_play_history,
            commands::history::list_recent_songs,
            // 设置
            commands::settings::get_setting,
            commands::settings::set_setting,
            // 插件（骨架）
            commands::plugin::list_plugins,
            commands::plugin::register_local_plugin,
            commands::plugin::set_plugin_enabled,
            // 搜索（骨架）
            commands::search::search_music,
            commands::search::list_source_playlist_songs,
            // 歌词（骨架）
            commands::lyrics::get_lyrics,
            commands::lyrics::set_lyrics,
        ])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用时发生错误");
}

/// 构建应用状态：初始化数据库与音频引擎。
fn build_app_state(app: &tauri::App) -> anyhow::Result<AppState> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| anyhow::anyhow!("无法获取应用数据目录: {e}"))?;
    std::fs::create_dir_all(&app_data_dir)?;

    let db_path = app_data_dir.join("cmmuu_music.db");
    let db = Arc::new(Database::open(&db_path)?);
    db.run_migrations()?;
    log::info!("数据库已初始化: {}", db_path.display());

    let audio: Arc<dyn AudioEngine> = Arc::new(core::audio::CpalAudioEngine::new()?);
    log::info!("音频引擎已初始化");

    Ok(AppState {
        db,
        audio,
        current_song: Mutex::new(None),
    })
}
