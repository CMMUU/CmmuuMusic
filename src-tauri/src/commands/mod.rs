pub mod lyrics;
pub mod player;
pub mod playlist;
pub mod plugin;
pub mod search;
pub mod settings;

/// 连通性测试命令。
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("你好，{name}！欢迎使用 Cmmuu Music。")
}
