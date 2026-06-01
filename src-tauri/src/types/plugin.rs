use serde::{Deserialize, Serialize};

/// 插件类型。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PluginType {
    /// Cmmuu 原生插件（方法导出）
    Cmmuu,
    /// 洛雪音乐兼容插件（事件驱动）
    Lx,
}

impl PluginType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Cmmuu => "cmmuu",
            Self::Lx => "lx",
        }
    }

    pub fn from_str(value: &str) -> Self {
        match value {
            "lx" => Self::Lx,
            _ => Self::Cmmuu,
        }
    }
}

/// 插件状态。对齐 SDD §5.2.1 生命周期。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PluginStatus {
    Ready,
    Disabled,
    Error,
}

/// 音源定义。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicSource {
    /// kw, wy, tx, kg, mg
    pub id: String,
    pub name: String,
    /// 支持的音质标签，如 ["128k", "320k", "flac"]
    pub qualities: Vec<String>,
}

/// 插件元信息（从文件头注释解析）。对齐 SDD §5.2.2。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginInfo {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub version: String,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub homepage: String,
    pub plugin_type: PluginType,
}

/// 插件注册表条目（持久化到数据库）。对齐 SDD §8.1 plugins 表。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginRecord {
    pub info: PluginInfo,
    pub sources: Vec<MusicSource>,
    pub file_path: String,
    pub enabled: bool,
    pub status: PluginStatus,
    pub installed_at: String,
    pub updated_at: String,
}
