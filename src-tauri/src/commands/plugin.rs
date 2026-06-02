use serde::Deserialize;
use tauri::State;

use crate::types::plugin::{MusicSource, PluginInfo, PluginRecord, PluginStatus, PluginType};
use crate::AppState;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterLocalPluginRequest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: Option<String>,
    pub plugin_type: PluginType,
    pub file_path: String,
}

const BUILTIN_CHANGQING_ID: &str = "builtin:changqing-svip";
const BUILTIN_CHANGQING_ENABLED_KEY: &str = "plugin.enabled.builtin:changqing-svip";

#[tauri::command]
pub async fn list_plugins(state: State<'_, AppState>) -> Result<Vec<PluginRecord>, String> {
    let mut records = state
        .db
        .list_plugin_records()
        .map_err(|e| e.to_string())?;
    let enabled = builtin_changqing_enabled(&state)?;
    records.push(builtin_changqing_plugin(enabled));
    Ok(records)
}

#[tauri::command]
pub async fn register_local_plugin(
    request: RegisterLocalPluginRequest,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let record = PluginRecord {
        info: PluginInfo {
            id: request.id,
            name: request.name,
            description: String::new(),
            version: request.version,
            author: request.author.unwrap_or_default(),
            homepage: String::new(),
            plugin_type: request.plugin_type,
        },
        sources: Vec::new(),
        file_path: request.file_path,
        enabled: false,
        status: PluginStatus::Disabled,
        installed_at: String::new(),
        updated_at: String::new(),
    };
    state
        .db
        .upsert_plugin_record(&record)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_plugin_enabled(
    plugin_id: String,
    enabled: bool,
    state: State<'_, AppState>,
) -> Result<bool, String> {
    if plugin_id == BUILTIN_CHANGQING_ID {
        state
            .db
            .set_setting(BUILTIN_CHANGQING_ENABLED_KEY, if enabled { "true" } else { "false" })
            .map_err(|e| e.to_string())?;
        return Ok(true);
    }

    if plugin_id.starts_with("builtin:") {
        return Err("未知内置音源".into());
    }

    state
        .db
        .set_plugin_enabled(&plugin_id, enabled)
        .map_err(|e| e.to_string())
}

pub fn is_builtin_changqing_enabled(state: &State<'_, AppState>) -> Result<bool, String> {
    builtin_changqing_enabled(state)
}

fn builtin_changqing_enabled(state: &State<'_, AppState>) -> Result<bool, String> {
    state
        .db
        .get_setting(BUILTIN_CHANGQING_ENABLED_KEY)
        .map(|value| value.as_deref() == Some("true"))
        .map_err(|e| e.to_string())
}

fn builtin_changqing_plugin(enabled: bool) -> PluginRecord {
    PluginRecord {
        info: PluginInfo {
            id: BUILTIN_CHANGQING_ID.into(),
            name: "长青SVIP音源".into(),
            description: "受控内置 LX 兼容音源，仅启用内置 provider，不执行第三方脚本。".into(),
            version: "1.2.0".into(),
            author: "SVIP".into(),
            homepage: "微信公众号: 元力菌".into(),
            plugin_type: PluginType::Lx,
        },
        sources: vec![
            MusicSource {
                id: "kg".into(),
                name: "酷狗".into(),
                qualities: vec!["128k".into(), "320k".into(), "flac".into()],
            },
            MusicSource {
                id: "tx".into(),
                name: "腾讯".into(),
                qualities: vec!["128k".into(), "320k".into(), "flac".into()],
            },
            MusicSource {
                id: "wy".into(),
                name: "网易".into(),
                qualities: vec!["128k".into(), "320k".into(), "flac".into()],
            },
            MusicSource {
                id: "kw".into(),
                name: "酷我".into(),
                qualities: vec!["128k".into(), "320k".into(), "flac".into()],
            },
            MusicSource {
                id: "mg".into(),
                name: "咪咕".into(),
                qualities: vec!["128k".into(), "320k".into(), "flac".into()],
            },
        ],
        file_path: "resources/sources/builtin/changqing-svip-v1.2.0.js".into(),
        enabled,
        status: if enabled {
            PluginStatus::Ready
        } else {
            PluginStatus::Disabled
        },
        installed_at: "2026-06-02T00:00:00Z".into(),
        updated_at: "2026-06-02T00:00:00Z".into(),
    }
}
