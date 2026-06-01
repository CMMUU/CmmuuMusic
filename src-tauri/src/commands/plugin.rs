use tauri::State;

use crate::types::plugin::PluginRecord;
use crate::AppState;

/// 列出已安装插件（骨架：插件系统阶段接入 PluginManager）。
#[tauri::command]
pub async fn list_plugins(_state: State<'_, AppState>) -> Result<Vec<PluginRecord>, String> {
    Ok(Vec::new())
}
