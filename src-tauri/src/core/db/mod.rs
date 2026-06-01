pub mod migrations;
pub mod models;

use std::path::Path;

use parking_lot::Mutex;
use rusqlite::Connection;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DbError {
    #[error("数据库错误: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("迁移失败: {0}")]
    Migration(String),
}

/// SQLite 数据库句柄。
///
/// rusqlite 的 `Connection` 非 `Sync`，用 `Mutex` 包裹以便在 Tauri 托管状态中跨命令共享。
/// POC 阶段单连接足够；后续高并发场景可替换为 r2d2 连接池。
pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    /// 打开（或创建）数据库文件。
    pub fn open(path: &Path) -> Result<Self, DbError> {
        let conn = Connection::open(path)?;
        conn.pragma_update(None, "journal_mode", "WAL")?;
        conn.pragma_update(None, "foreign_keys", "ON")?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    /// 内存数据库（用于测试）。
    #[cfg(test)]
    pub fn open_in_memory() -> Result<Self, DbError> {
        let conn = Connection::open_in_memory()?;
        conn.pragma_update(None, "foreign_keys", "ON")?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    /// 执行数据库迁移。
    pub fn run_migrations(&self) -> Result<(), DbError> {
        let conn = self.conn.lock();
        migrations::run(&conn)
    }

    /// 获取底层连接的加锁访问。
    pub fn with_conn<T>(&self, f: impl FnOnce(&Connection) -> Result<T, rusqlite::Error>) -> Result<T, DbError> {
        let conn = self.conn.lock();
        f(&conn).map_err(DbError::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn migrations_create_all_tables() {
        let db = Database::open_in_memory().unwrap();
        db.run_migrations().unwrap();

        let tables = db
            .with_conn(|c| {
                let mut stmt =
                    c.prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")?;
                let rows = stmt
                    .query_map([], |r| r.get::<_, String>(0))?
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(rows)
            })
            .unwrap();

        for expected in [
            "cache_index",
            "play_history",
            "playlist_songs",
            "playlists",
            "plugins",
            "settings",
            "songs",
        ] {
            assert!(
                tables.iter().any(|t| t == expected),
                "缺少表: {expected}, 实际: {tables:?}"
            );
        }
    }

    #[test]
    fn migrations_are_idempotent() {
        let db = Database::open_in_memory().unwrap();
        db.run_migrations().unwrap();
        // 再次运行不应报错
        db.run_migrations().unwrap();
    }
}
