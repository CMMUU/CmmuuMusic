//! 插件系统（骨架）。
//!
//! 对齐 SDD §5.2、§6。完整实现包含：
//! - manager: 插件生命周期管理（安装/校验/加载/启用/卸载）
//! - sandbox: 基于 QuickJS / Deno Core 的隔离执行环境（资源限制、最小权限）
//! - loader: 插件文件加载与元信息解析
//! - converter: LX 事件驱动插件 → Cmmuu 原生接口转换
//! - api: 注入沙箱的宿主 API（request / crypto / NoticeCenter）
//!
//! POC 阶段仅建立模块骨架与类型定义；沙箱运行时（QuickJS）将在插件系统阶段引入。

pub mod api;
pub mod converter;
pub mod loader;
pub mod manager;
pub mod sandbox;

// LX 事件名称常量（与 globalThis.lx.EVENT_NAMES 对齐，见 SDD §5.2.4）。
pub const LX_EVENT_INITED: &str = "inited";
pub const LX_EVENT_REQUEST: &str = "request";
pub const LX_EVENT_UPDATE_ALERT: &str = "updateAlert";
