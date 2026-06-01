//! 缓存管理（占位）。
//!
//! 对齐 SDD §8.1 cache_index 表与 §7.1 缓存检查环节。
//! 负责音乐文件本地缓存的读写、过期清理与容量上限控制，将在缓存阶段实现。

pub mod metadata;
pub mod music;
