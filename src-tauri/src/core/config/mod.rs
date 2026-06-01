//! 配置管理（占位）。
//!
//! 对齐 SDD §8.2 AppSettings。负责应用配置的加载、持久化与默认值管理，
//! 当前设置读写直接走 settings 表（见 core::db::models），结构化配置将在配置阶段实现。

pub mod manager;
