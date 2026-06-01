use tauri::State;

use crate::types::plugin::{MusicSource, PluginInfo, PluginRecord, PluginStatus, PluginType};
use crate::AppState;

#[tauri::command]
pub async fn list_plugins(state: State<'_, AppState>) -> Result<Vec<PluginRecord>, String> {
    let mut records = state
        .db
        .list_plugin_records()
        .map_err(|e| e.to_string())?;
    records.push(builtin_changqing_plugin());
    Ok(records)
}

fn builtin_changqing_plugin() -> PluginRecord {
    PluginRecord {
        info: PluginInfo {
            id: "builtin:changqing-svip".into(),
            name: "长青SVIP音源".into(),
            description: "内置 LX 兼容音源资源，当前仅登记元数据，不执行插件脚本。".into(),
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
        enabled: false,
        status: PluginStatus::Disabled,
        installed_at: "2026-06-02T00:00:00Z".into(),
        updated_at: "2026-06-02T00:00:00Z".into(),
    }
}
